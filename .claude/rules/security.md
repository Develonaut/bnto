# Security Audit Checklist

**Status:** Active — repo is public, all checks apply now
**Last Updated:** February 2026
**Tracks:** Security audit requirements (previously in open-source-strategy.md, now consolidated here)
**Full audit skill:** `.claude/skills/security-review/SKILL.md`

---

## Auth Flow Security

Each item maps to a testable assertion — unit tests now, e2e when auth flows are wired.

### Proxy Route Protection

- [ ] Unauthenticated requests to private routes redirect to `/signin`
- [ ] Authenticated requests to `/signin` redirect to `/`
- [ ] Sign-out signal cookie bypasses the auth-on-signin redirect
- [ ] Public paths (`/`, `/signin`, `/waitlist`) are accessible without auth

**Test approach:** Unit tests on `proxy.ts` logic (route matching, cookie presence checks). E2e tests for the full redirect flow once auth is live.

### OAuth & Session

- [ ] OAuth redirect URIs are validated server-side (no open redirect)
- [ ] Session tokens are `httpOnly`, `secure`, `sameSite`
- [ ] Session expiry triggers `onSessionLost` callback (client-side redirect)
- [ ] Sign-out clears server session and invalidates cookies

**Test approach:** Integration tests against `@convex-dev/auth` config (see `convex-test` tests in `@bnto/backend`). E2e tests for sign-in → session → sign-out flow.

### Invite / Beta Gating

- [ ] If a beta code system exists, it can't be bypassed by navigating directly to a protected URL
- [ ] Waitlist page functions correctly for unauthenticated users

**Test approach:** E2e test — navigate to gated route without invite code, verify redirect.

---

## API Route Authorization

### Convex Functions

- [ ] All mutations that modify user data verify the caller owns the resource
- [ ] Public queries (workflow listings) work without auth where intended
- [ ] No mutation accepts a `userId` parameter (derive from session, never trust client)

**Test approach:** Unit tests with `convex-test` — call mutations without auth, call with wrong user, verify rejection.

### Go API Endpoints

- [ ] All endpoints validate authentication tokens
- [ ] No endpoint is accessible to unauthenticated users unless explicitly public
- [ ] Token validation rejects expired, malformed, and missing tokens

**Test approach:** Integration tests with `httptest` — request without token, with bad token, with valid token.

---

## Input Validation & Injection

### Convex Validators

- [ ] Every mutation and action validates all inputs with Convex validators (`v.string()`, `v.id()`, etc.)
- [ ] No mutation accepts unvalidated `Record<string, unknown>` or `any`

**Test approach:** Unit tests with `convex-test` — pass invalid input shapes, verify rejection.

### Go Engine Validation

- [ ] Workflow definitions are validated before execution (schema, node types, connections)
- [ ] Malformed `.bnto.json` files produce clear error messages, not panics
- [ ] Template expressions (`{{...}}`) are sandboxed — no access to OS env, filesystem, or network

**Test approach:** Unit tests in `engine/pkg/validator/` — malformed workflows, unknown node types, invalid connections. Already partially covered by existing fixture tests.

### XSS Prevention

- [ ] No raw user input rendered without sanitization in React components
- [ ] Workflow names, descriptions, and node labels are escaped in all views
- [ ] Log output is rendered in a safe context (pre/code blocks, not dangerouslySetInnerHTML)

**Test approach:** E2e test — create workflow with `<script>alert(1)</script>` in name, verify it renders as text.

### File Upload Security

- [ ] Presigned URL generation validates file types against an allowlist
- [ ] Presigned URL generation enforces file size limits
- [ ] Uploaded files are validated server-side, not just client-side

**Test approach:** Integration tests — request presigned URL with disallowed type, oversized file.

---

## Sensitive Content Review

### Code Comments

- [ ] No comments reference internal business plans, competitive analysis, or personal notes
- [ ] No TODO comments contain sensitive context

**Test approach:** Grep scan — search for patterns like `TODO`, `HACK`, `FIXME`, `competitor`, `pricing`, `revenue`. Review results manually.

### Test Fixtures

- [ ] No test fixtures contain real user data (names, emails, addresses)
- [ ] No test fixtures contain real API responses with PII
- [ ] All fixture data uses obviously fake values (`test@example.com`, `Jane Doe`)

**Test approach:** Grep scan of `engine/tests/fixtures/`, `engine/examples/`, and any `__tests__/` directories for email patterns, phone patterns, real-looking names.

### Dependency Surface

- [ ] `package.json` has no unnecessary dependencies
- [ ] `go.mod` has no unnecessary dependencies
- [ ] No vendored code with its own license obligations

**Test approach:** Manual review. Compare dependency list against actual imports.

---

## When to Run Full Audit

Run the full security-review skill (`/security-review`) when:

1. **Auth implementation changes** — Sprint 2A completes, auth solution switches
2. **New API surface exposed** — new Convex functions, Go API endpoints, or web routes
3. **Dependency updates** — major version bumps, new dependencies added
4. **Before any release milestone** — Sprint 3 (monetization), Sprint 7 (Stripe)
