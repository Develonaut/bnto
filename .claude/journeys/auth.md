# Auth — User Journey Test Matrix

**Domain:** Authentication, sessions, identity, and authorization gates
**Status:** Sprint 3A — anonymous system removed
**Last Updated:** March 2, 2026

---

## Why This Matters

The auth system gates access to Convex functions and protected routes. Users either sign up with email/password or use bnto without an account (browser execution is free, unlimited, no session needed). Auth exists for: (1) saving workflows, (2) execution history, (3) server-node access control (Pro).

---

## Auth Gate Map

Every point in the execution flow where auth is checked.

| Step | What Happens | Auth Gate | Function | Unauthenticated Behavior |
|------|-------------|-----------|----------|--------------------------|
| Page load | `/compress-images` renders | None (public route) | N/A | Renders tool page |
| Browser execution | WASM processes files locally | None (client-side) | `BntoWorker` | Works — no server interaction |
| Upload URLs | Generate R2 presigned upload URLs | `getAppUserId()` + plan check | `convex/uploads.ts:generateUploadUrls` | Rejected — requires auth |
| Upload to R2 | PUT file to presigned URL | None (URL is the credential) | R2 presigned URL | Works (if URL obtained) |
| Start execution | Create execution record | `getAppUserId()` + plan check | `convex/executions.ts:startPredefined` | Rejected — requires auth |
| Progress stream | Subscribe to execution status | `getAppUserId()` + ownership check | `convex/executions.ts:get` | Returns null |
| Download URLs | Generate R2 presigned download URLs | `getAppUserId()` + ownership check | `convex/downloads.ts:generateDownloadUrls` | Rejected — requires auth |
| Download from R2 | GET file from presigned URL | None (URL is the credential) | R2 presigned URL | Works (if URL obtained) |

**Key insight:** Browser execution bypasses all auth gates — it's 100% client-side. Auth gates only apply to cloud execution (R2 upload/download, Convex execution records).

---

## Journey Matrix

Each row is one test. "Pass" means auth didn't block the user from completing the action.

### Standard Auth Lifecycle

Email sign-up, sign-in, sign-out.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **S1** | Email sign-up works | Sign up with name/email/password | Session created. `users` table row exists. `getAppUserId()` returns valid userId. |
| **S2** | Email sign-in works | Sign in with existing email/password | Session created. Same userId as sign-up. Convex mutations succeed. |
| **S3** | Sign-out clears session | S1 → sign out | Session invalidated. `getAppUserId()` returns null. Protected queries return null. |
| **S4** | Auth API surface | All auth endpoints respond correctly | sign-in, sign-up, sign-out, token refresh return correct shapes. Invalid credentials return errors, not crashes. |

### Conversion (Anonymous → Authenticated)

Data migration when a user signs up after using bnto without an account.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **C1** | Local history migrates on signup | Run recipe as unauth → verify local history → sign up | Execution history from IndexedDB appears in Convex-backed `/my-recipes` history. Same slug, timestamp, status preserved. |
| **C2** | Local history cleared after migration | C1 complete → check IndexedDB | IndexedDB execution history is empty after successful migration to Convex. |
| **C3** | Migration is idempotent | C1 → sign out → sign in again | No duplicate executions in Convex history. Migration only runs once (on first auth transition, not every sign-in). |
| **C4** | Migration handles empty local history | Sign up with no prior runs | No errors. Convex history starts empty. Migration is a no-op. |

### Access Control

Execution visibility and ownership.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **V1** | Execution visible to owner | S1 → start execution → query | Owner can see their execution. |
| **V2** | Execution NOT visible to others | S1 → start execution → different user queries | Returns null. |
| **V3** | Unauthenticated cannot start execution | No auth → `startPredefined` | Throws "Not authenticated". |
| **V4** | Unauthenticated cannot download | No auth → `generateDownloadUrls` | Throws error. |

---

## Cross-Domain Dependencies

| Journey | Also Touches | Notes |
|---------|-------------|-------|
| C1-C4 | Core (IndexedDB adapter), Backend (Convex executions), Web (history UI) | Migration spans three layers — browser storage, transport, server storage |
| V1-V2 | Web (real-time UI) | Auth gates the Convex subscription, web renders progress |

---

## Implementation Notes

- Tests run against real Convex dev backend (`task dev:all`), not mocks
- Integration tests (Vitest) + E2E tests (Playwright) — test the full auth pipeline
- Group tests by file:
  - `auth-lifecycle.test.ts` (S1-S4) — ConvexHttpClient integration tests
  - `execution.test.ts` (V1-V3) — execution lifecycle + access control
  - `upload.test.ts` (V4 variant) — upload access control
  - `auth-lifecycle.spec.ts` — Playwright E2E for browser auth flows
