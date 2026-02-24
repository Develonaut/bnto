# Core Integration Testing Against Real Convex Dev

**Date:** February 23, 2026
**Status:** Planned (Sprint 2A Wave 4)

---

## Problem

We have two testing layers:
- **`convex-test`** (in-memory) — validates function logic but never touches the real deployment
- **Playwright E2E** (browser) — validates user journeys but is slow and couples to UI

There's a gap: nothing tests the deployed Convex infrastructure (auth provider config, env vars, indexes, R2 connectivity) without a browser. If a schema migration breaks in dev, we won't know until Playwright runs.

## Decision

Add an integration test layer at `@bnto/core` that calls real Convex dev functions via `ConvexHttpClient`. No React, no browser — just the imperative API against the deployed stack.

## Testing Pyramid

```
convex-test       →  logic correct        (in-memory, fast, CI-safe)
@bnto/core        →  transport correct    (real Convex dev, no browser)
Playwright E2E    →  user journey correct (real browser, full stack)
```

## Auth Approach: `ConvexHttpClient` + `api.auth.signIn`

`@convex-dev/auth`'s `signIn` is a public Convex action callable without React:

```typescript
import { ConvexHttpClient } from "convex/browser";
import { api } from "@bnto/backend/convex/_generated/api";

const client = new ConvexHttpClient(CONVEX_DEV_URL);

// Anonymous auth — no React, no browser
const result = await client.action(api.auth.signIn, {
  provider: "anonymous",
});
// result.tokens = { token: "jwt...", refreshToken: "..." }

client.setAuth(result.tokens.token);

// Now all calls are authenticated against real Convex dev
const user = await client.query(api.users.getMe);
```

### How it works under the hood

1. `ConvexHttpClient.action()` sends an HTTP POST to the Convex deployment
2. `@convex-dev/auth`'s `signIn` action creates a `users` row + `authSessions` row + `authAccounts` row
3. Returns a JWT signed with RS256 (`sub: "userId|sessionId"`)
4. `client.setAuth(token)` includes `Authorization: Bearer <jwt>` on all subsequent requests
5. Convex validates the JWT against its JWKS endpoint — `ctx.auth.getUserIdentity()` works

### Password auth (for conversion tests)

```typescript
// Sign up
const result = await client.action(api.auth.signIn, {
  provider: "password",
  params: { email: "test@test.bnto.dev", password: "test-pass-123", flow: "signUp" },
});

// Sign in to existing account
const result = await client.action(api.auth.signIn, {
  provider: "password",
  params: { email: "test@test.bnto.dev", password: "test-pass-123", flow: "signIn" },
});
```

### Token refresh

JWT expires in 1 hour. For long-running suites:

```typescript
const refreshResult = await client.action(api.auth.signIn, {
  refreshToken: result.tokens.refreshToken,
});
client.setAuth(refreshResult.tokens.token);
```

## Gotchas

1. **Type generation:** `api.auth.signIn` may need `as any` cast — the generated types don't expose `@convex-dev/auth`'s internal action signatures fully. Fallback: `import { anyApi } from "convex/server"` and use `anyApi.auth.signIn`.

2. **Test data cleanup:** Each test creates real users/sessions in dev. Options:
   - Accept accumulation (Convex dev is cheap)
   - Clean up via admin mutation after each suite
   - Use naming conventions for test users

3. **Rate limiting:** `@convex-dev/auth` may rate-limit sign-in attempts. Create one authenticated client per suite, not per test.

4. **`task dev:all` required:** Tests need the full dev stack running (Convex dev + Go API via tunnel + R2 dev bucket). CI would need to either skip these or have access to the dev stack.

5. **No backend changes needed.** The `signIn` action is already public. This is purely a test-side concern.

## Alternatives Considered

### `setAdminAuth` with deploy key

`ConvexHttpClient` has an internal `setAdminAuth(deployKey, actingAsIdentity)` method that can impersonate any identity. Rejected because:
- Marked `@internal` — could change without notice
- Doesn't test the actual auth flow
- Deploy key in tests is a security concern

### Calling Convex functions directly via HTTP

Convex functions are HTTP endpoints under the hood. We could POST directly. Rejected because:
- `ConvexHttpClient` already wraps this cleanly
- No benefit over using the official client
