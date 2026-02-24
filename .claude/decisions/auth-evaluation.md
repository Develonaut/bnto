# Decision: Auth Solution — `@convex-dev/auth` vs Better Auth

**Date:** 2026-02-23
**Status:** Decision Required
**Context:** Sprint 2A blocker — anonymous session → Convex mutation fails due to JWT propagation race condition between Better Auth and Convex's WebSocket protocol

---

## The Problem

The execution pipeline is blocked. Anonymous users can't run Bntos because:

1. `useAnonymousSession()` calls `authClient.signIn.anonymous()` via Better Auth
2. Better Auth creates a session and sets a cookie
3. `ConvexBetterAuthProvider` must detect the session, fetch a JWT from Better Auth's endpoint, and pass it to the Convex WebSocket client
4. During this async gap (steps 2→3), any Convex mutation fires before the JWT is propagated
5. Result: `getAppUserId()` returns `null` → "Not authenticated"

This is **not a bug in our code** — it's a fundamental architectural consequence of bridging an external auth server (Better Auth HTTP endpoint) with Convex's WebSocket protocol. The token propagation is inherently two-hop and asynchronous.

**Impact:** All downstream work is blocked — integration tests, execution event logging, quota tracking, and the entire monetization pipeline (anonymous → signup → paid).

---

## Options Evaluated

### Option A: Fix Better Auth Integration (Workarounds)

Keep Better Auth. Layer multiple fixes to close the race condition gap.

**Required fixes (all needed simultaneously):**

| Fix | What it does | Solves |
|---|---|---|
| Use `useConvexAuth()` as source of truth | Gate on Convex's auth state instead of Better Auth's `useSession()` | Initial load race |
| Pass `initialToken` from server layout | Eliminates cold-start token fetch on hydration | Initial load race |
| Set `expectAuth` on ConvexReactClient | Blocks all queries until auth resolves | Initial load race |
| Auth gate with `useConvexAuth()` | Wraps authenticated content | Initial load race |
| Increase `authRefreshTokenLeewaySeconds` | Refreshes token earlier to shrink the expiry gap | Token refresh flash (partial) |

**Pros:**
- Less migration work — keep existing code, add fixes on top
- Better Auth has broad plugin ecosystem (50+ OAuth providers, 2FA, magic links)
- No schema migration needed

**Cons:**
- Fixes are layered workarounds for a design gap that can't be fully closed
- Token refresh still causes ~500ms unauthenticated flash (Manifestation 2 — `useConvexAuth()` drops to `false` while refreshing)
- Known to get permanently stuck in unauthenticated state requiring page reload (Manifestation 3)
- `@convex-dev/better-auth` is self-described as "early alpha development"
- Anonymous plugin has open bugs: #5824 (anonymous user deleted during sign-in — workaround exists), #180 (ArgumentValidationError during anonymous sign-in — still open)
- Better Auth must be pinned to exact version (1.4.9) for Convex compatibility
- Two-tier user table: Better Auth component user → app `users` table (linked by `userId` string), requires `getAppUserId()` helper to resolve
- Anonymous → authenticated upgrade creates a **new user ID** — requires data migration via `onLinkAccount` callback and the existing `migrate_anonymous.ts` mutation
- JWK race condition (Issue #5663) — concurrent requests can create duplicate keys

### Option B: Migrate to `@convex-dev/auth` (Native Convex Auth)

Replace Better Auth with Convex's own auth library. Sessions are Convex mutations on the same WebSocket.

**How it eliminates the race condition:**

```
Better Auth (two-hop, async):
  Client → Better Auth HTTP endpoint → JWT → Cookie → Convex WebSocket reads JWT → validates
  ↑ gap here where mutations fail

@convex-dev/auth (single-hop, atomic):
  Client → Convex WebSocket → mutation creates session → JWT returned directly
  ↑ no gap — signIn() resolves after session commits
```

**Pros:**
- **Eliminates the race condition architecturally** — no external HTTP hop in the auth path
- Anonymous sessions are native via `Anonymous` provider — session creation is a Convex mutation, immediately usable
- **Same user ID preserved on upgrade** — `createOrUpdateUser` callback patches the existing user document instead of creating a new one. No data migration needed. Delete `migrate_anonymous.ts` entirely
- **Single-tier user table** — the `users` table IS the auth user table. `getAuthUserId(ctx)` returns `users._id` directly. No `userId` string link, no `getAppUserId()` helper
- First-class Next.js middleware support — `convexAuthNextjsMiddleware` with real token validation (not just cookie-presence check)
- Built and maintained by the Convex team, designed specifically for Convex
- Refresh token reuse detection built in (prevents replay attacks)
- Server-side data fetching via `convexAuthNextjsToken()`

**Cons:**
- Migration effort: ~15 files to touch (well-contained in `@bnto/auth`, `@bnto/backend`, `@bnto/core`, `apps/web`)
- Schema migration: Better Auth component tables → Convex auth tables (different structure)
- All active sessions invalidated on migration (users must re-authenticate)
- Library is in beta (but actively maintained, Convex team's own product)
- Anonymous auth marked as "advanced feature" with recommendation for CAPTCHA to prevent abuse
- 80+ OAuth providers via Auth.js (slightly different ecosystem than Better Auth's 50+)

---

## Comparison Matrix

| Dimension | Better Auth (Option A) | `@convex-dev/auth` (Option B) |
|---|---|---|
| **Race condition** | Mitigated by 5 layered workarounds, not eliminated | Eliminated architecturally |
| **Token refresh gap** | ~500ms flash, can get permanently stuck | No external hop, refresh is a Convex mutation |
| **Anonymous sessions** | Plugin with open bugs, early alpha Convex integration | Native provider, designed for Convex |
| **User ID on upgrade** | New ID created → data migration required | Same ID preserved → no migration |
| **User table** | Two-tier (component user → app user) | Single-tier (`users._id` = auth user ID) |
| **Auth helper** | `getAppUserId()` — two hops via `authComponent` | `getAuthUserId(ctx)` — direct, one line |
| **Middleware** | Cookie-presence check (no validation) | Real token validation via `convexAuth.isAuthenticated()` |
| **Next.js support** | Via `@convex-dev/better-auth/nextjs` (experimental JWT cache) | First-class `@convex-dev/auth/nextjs/server` |
| **Migration effort** | Low (add workaround code) | Medium (~15 files, well-contained) |
| **Schema risk** | None | Low (pre-launch, no real users yet) |
| **Maturity** | Better Auth is mature; Convex integration is early alpha | Beta, actively maintained by Convex team |
| **Version pinning** | Must pin to 1.4.9 exactly | Standard semver |

---

## Recommendation: Option B — Migrate to `@convex-dev/auth`

### Why

The race condition is not a bug — it's an architectural consequence of bridging two systems with different transport mechanisms (HTTP vs WebSocket). Option A layers workarounds on top of this gap. Option B eliminates the gap entirely by making auth native to Convex.

The secondary benefits compound:
- **Simpler user model** — one table, one ID, no resolution helpers
- **No data migration on upgrade** — same user ID, delete `migrate_anonymous.ts`
- **Stronger middleware** — real token validation instead of cookie-presence check
- **Less code overall** — remove two-tier user resolution, migration mutation, version pinning workarounds

The migration effort is medium but well-contained. Every file that needs to change is in the auth surface area (`@bnto/auth`, `@bnto/backend` auth files, `@bnto/core` provider, `apps/web` middleware/layout). Pre-launch status means schema migration has zero user impact.

### What We Lose

Better Auth's broader plugin ecosystem (2FA, magic links, organization management). These are future backlog items, not current requirements. `@convex-dev/auth` supports email/password and 80+ OAuth providers via Auth.js — everything we need for Sprint 2A and beyond.

### What We Gain

A working anonymous session flow. That's the blocker. Everything else is bonus.

---

## Migration Plan

### Phase 1: Dependencies & Schema

| Action | Files |
|---|---|
| Install `@convex-dev/auth`, `@auth/core` | Root `package.json` |
| Remove `better-auth`, `@convex-dev/better-auth` | Root `package.json` |
| Add `authTables` to Convex schema | `@bnto/backend/convex/schema.ts` |
| Restructure `users` table (remove `userId` string link, keep app fields) | `@bnto/backend/convex/schema.ts` |
| Configure `convexAuth()` with `Anonymous` + `Password` providers | `@bnto/backend/convex/auth.ts` |
| Implement `createOrUpdateUser` callback (anonymous flag, app field defaults) | `@bnto/backend/convex/auth.ts` |
| Update auth config | `@bnto/backend/convex/auth.config.ts` |
| Remove Better Auth component from `convex.config.ts` | `@bnto/backend/convex/convex.config.ts` |
| Update HTTP router | `@bnto/backend/convex/http.ts` |

### Phase 2: Auth Package & Core

| Action | Files |
|---|---|
| Replace `@bnto/auth` client with `@convex-dev/auth/react` imports | `@bnto/auth/src/client.ts` |
| Replace `@bnto/auth` server with `@convex-dev/auth/nextjs/server` imports | `@bnto/auth/src/server.ts` |
| Update middleware exports | `@bnto/auth/src/middleware.ts` |
| Update auth hooks (`useSession`, `useSignIn`, `useSignUp`, `useSignOut`) | `@bnto/auth/src/hooks/*.ts` |
| Replace `ConvexBetterAuthProvider` with `ConvexAuthProvider` | `@bnto/core/src/provider.tsx` |
| Simplify `getAppUserId()` → `getAuthUserId(ctx)` | `@bnto/backend/convex/_helpers/auth.ts` |
| Simplify or remove `ensureUser` (auth creates users via callback) | `@bnto/backend/convex/users.ts` |
| Delete `migrate_anonymous.ts` (same user ID preserved) | `@bnto/backend/convex/migrate_anonymous.ts` |

### Phase 3: Web App

| Action | Files |
|---|---|
| Add `ConvexAuthNextjsServerProvider` to root layout | `apps/web/app/layout.tsx` |
| Replace middleware with `convexAuthNextjsMiddleware` | `apps/web/middleware.ts` |
| Update Providers component | `apps/web/app/providers/index.tsx` |
| Verify sign-out signal cookie pattern still works | `apps/web` sign-out flow |

### Phase 4: Verify

| Action | How |
|---|---|
| Anonymous session → Convex mutation succeeds | Manual test + integration test |
| `proxy.ts` / middleware route protection works | E2E test |
| Sign-in, sign-up, sign-out flows work | E2E test |
| Anonymous → authenticated preserves data | Integration test |
| All existing tests pass | `task test`, `task api:test`, `task ui:build`, `task ui:test` |

### Risk: CAPTCHA for Anonymous Abuse

`@convex-dev/auth` docs recommend CAPTCHA for anonymous auth to prevent automated user creation. Options:
- **Cloudflare Turnstile** (free, privacy-friendly) — already using Cloudflare for R2 and tunnels
- **Defer to Sprint 3** — pre-launch, abuse risk is near zero. Add Turnstile when quota enforcement goes live

**Recommendation:** Defer CAPTCHA to Sprint 3. Note it as a prerequisite for quota enforcement.

---

## Decision Summary

| Aspect | Choice |
|---|---|
| Auth library | **`@convex-dev/auth`** (native Convex auth) |
| Anonymous sessions | **`Anonymous` provider** (native, atomic, same WebSocket) |
| Email/password | **`Password` provider** (built-in, with future email verification via Resend) |
| User table | **Single-tier** — `users._id` IS the auth user ID |
| Anonymous upgrade | **Same user ID** — `createOrUpdateUser` patches existing doc |
| Middleware | **`convexAuthNextjsMiddleware`** — real token validation |
| CAPTCHA | **Deferred to Sprint 3** — Cloudflare Turnstile when quota enforcement ships |
| Migration timing | **Now** — pre-launch, zero user impact, unblocks all downstream work |

---

## References

- [Convex Auth Documentation](https://labs.convex.dev/auth)
- [Convex Auth Anonymous Users](https://labs.convex.dev/auth/config/anonymous)
- [Convex Auth Next.js Server](https://labs.convex.dev/auth/authz/nextjs)
- [Race condition: convex-backend #242](https://github.com/get-convex/convex-backend/issues/242)
- [Stuck unauthenticated: convex-backend #259](https://github.com/get-convex/convex-backend/issues/259)
- [Better Auth anonymous plugin broken: #5824](https://github.com/better-auth/better-auth/issues/5824)
- [Upgrading anonymous accounts (Convex Discord)](https://discord-questions.convex.dev/m/1366162915035447416)
- Prior research: `.claude/research/anonymous-auth-better-auth-convex.md`
- Prior research: `.claude/research/nextjs-convex-better-auth-setup.md`
- Auth pattern: `.claude/strategy/auth-pattern.md`
- Auth journey tests: `.claude/journeys/auth.md`
