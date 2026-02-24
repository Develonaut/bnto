# Auth — User Journey Test Matrix

**Domain:** Authentication, sessions, identity, and authorization gates
**Status:** Sprint 2A — active priority
**Last Updated:** February 23, 2026

---

## Why This Matters

The monetization strategy depends on: (1) anonymous users running bntos freely to build habit, (2) converting anonymous → signed-up when they hit the free limit or want to save work, (3) signed-up → Pro when they need more runs. Every step requires auth sessions that Convex recognizes. These tests prove auth has UNBLOCKED users from using the environment — not that every feature is polished, but that auth doesn't stand in the way.

---

## Auth Gate Map

Every point in the execution flow where auth is checked. This is the system-level view — what functions get called, what they check, and what happens for each user type.

| Step | What Happens | Auth Gate | Function | Anonymous Behavior |
|------|-------------|-----------|----------|-------------------|
| Page load | `/compress-images` renders | None (public route) | N/A | Renders tool page |
| Session init | `useAnonymousSession()` fires on mount | `authClient.signIn.anonymous()` | `useAnonymousSession.ts` | Creates anonymous session + `users` row |
| Upload URLs | Generate R2 presigned upload URLs | `api.users.getMe` → plan check | `convex/uploads.ts:generateUploadUrls` | Defaults to `"free"` plan limits |
| Upload to R2 | PUT file to presigned URL | None (URL is the credential) | R2 presigned URL | Works |
| Start execution | Create execution record | `getAppUserId()` + `enforceQuota()` | `convex/executions.ts:startPredefined` | **THE GATE THAT'S BROKEN** — needs valid userId |
| Progress stream | Subscribe to execution status | `getAppUserId()` + ownership check | `convex/executions.ts:get` | Returns own execution only |
| Download URLs | Generate R2 presigned download URLs | `getAppUserId()` + ownership check | `convex/downloads.ts:generateDownloadUrls` | Can download own results only |
| Download from R2 | GET file from presigned URL | None (URL is the credential) | R2 presigned URL | Works |

**Key insight:** R2 operations (upload, download) don't check auth — the presigned URL IS the credential. Auth gates are all in Convex functions that generate those URLs or create/read execution records.

---

## Journey Matrix

Each row is one test. "Pass" means auth didn't block the user from completing the action.

### Anonymous Execution Flow

The happy path. Anonymous user goes from landing to download.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **A1** | Anonymous session bootstrap | Land on `/compress-images` → `useAnonymousSession` fires | Session created. `users` table row exists with `isAnonymous: true`. `getAppUserId()` returns non-null for this session. |
| **A2** | Anonymous can generate upload URLs | A1 → call `api.uploads.generateUploadUrls` | Returns presigned URLs. Plan defaults to `"free"`. File size limits enforced (not rejected outright). |
| **A3** | Anonymous can start execution | A1 → call `api.executions.startPredefined` with valid args | Execution record created in Convex with status `"pending"`, linked to anonymous userId. No "Not authenticated" error. |
| **A4** | Anonymous can subscribe to progress | A3 → subscribe to `api.executions.get(executionId)` | Returns the execution record (not null). Ownership check passes — anonymous user sees their own execution. |
| **A5** | Anonymous can generate download URLs | A3 (completed execution) → call `api.downloads.generateDownloadUrls` | Returns presigned download URLs. Ownership check passes. |

### Anonymous Edge Cases

Quota enforcement and session durability.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **A6** | Anonymous quota enforcement | A1 → exhaust anonymous run limit → attempt another execution | `startPredefined` throws quota error (not auth error). Error is `"ANONYMOUS_QUOTA_EXCEEDED"`, not `"Not authenticated"`. |
| **A7** | Anonymous session persists across refresh | A1 → page refresh → call any Convex mutation | Same userId. No re-authentication. Session cookie survives refresh. |

### Conversion Flow

Anonymous → signup. The revenue funnel.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **C1** | Anonymous → signup conversion | A1 (has runs) → sign up with email/password | Same userId preserved. Existing runs and executions still belong to this user. `isAnonymous` flips to `false`. |
| **C2** | Converted user retains access | C1 → call `api.executions.get` for pre-conversion execution | Returns the execution. Ownership still valid — userId didn't change. |
| **C3** | Converted user gets full quota | C1 → check `runsUsedThisMonth` and `runLimit` | Quota upgraded from anonymous limit to free-tier limit (25/month). Existing run count preserved (not reset). |

### Standard Auth Lifecycle

Email sign-in/sign-out and API surface.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **S1** | Email sign-in works | Sign in with email/password | Session created. `getAppUserId()` returns valid userId. Convex mutations succeed. |
| **S2** | Sign-out clears session | S1 → sign out | Session invalidated. `getAppUserId()` returns null. Convex mutations correctly reject. |
| **S3** | Auth API surface | All auth endpoints respond correctly | sign-in (anonymous + email), sign-up, sign-out, get-session return correct shapes. Invalid credentials return errors, not crashes. |

---

## Cross-Domain Dependencies

| Journey | Also Touches | Notes |
|---------|-------------|-------|
| A2, A5 | API (R2 presigned URLs) | Auth validates ownership, API generates URLs |
| A3 | API (Railway execution) | Auth creates execution record, API runs the workflow |
| A4 | Web (real-time UI) | Auth gates the Convex subscription, web renders progress |
| C1-C3 | Web (signup form) | Conversion triggered through web app UI |

---

## Implementation Notes

- Tests run against real Convex dev backend (`task dev:all`), not mocks
- Integration tests (Vitest or Playwright), not unit tests — these test the full auth pipeline
- Group tests by file matching the matrix sections:
  - `anonymous-execution.test.ts` (A1-A5)
  - `anonymous-edge-cases.test.ts` (A6-A7)
  - `conversion-flow.test.ts` (C1-C3)
  - `auth-lifecycle.test.ts` (S1-S3)
