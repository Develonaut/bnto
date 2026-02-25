---
name: security-review
description: Security posture review across codebase, cloud services, and attack surfaces
---

# Security Review

Comprehensive security audit of the bnto project across all surfaces: codebase, Go API, Convex functions, cloud infrastructure (Cloudflare, Railway, Vercel), GitHub repo, and client-side attack vectors.

**The repo is PUBLIC.** Every file, every commit, every `.claude/` document is visible to anyone. All checks below must be evaluated with that in mind — infrastructure identifiers, business strategy details, internal notes, and git history are all exposed.

This is **read-only** — you are auditing, not fixing. Present findings to the user.

## Step 0: Read the Context

Before reviewing anything, read these files to understand the architecture and known security posture:

```
.claude/CLAUDE.md                          # Architecture, tech stack, data flow
.claude/rules/security.md                  # Existing security checklist (deferred items)
.claude/rules/auth-routing.md              # Auth routing model (proxy, middleware, cookies)
.claude/environment-variables.md           # All env vars, where they're configured
.claude/rules/convex.md                    # Convex function standards
.claude/rules/code-standards.md             # Code standards (includes Go conventions)
```

**Read ALL of these files now.** The audit sections below reference these documents. You need the full picture before scanning.

---

## Section 1: Secret & Credential Scanning

Scan the **entire repo** for leaked secrets, credentials, and sensitive values. This is the highest-priority check.

### 1a: Hardcoded secrets in code

Search for patterns that indicate hardcoded secrets:

```
# API keys, tokens, passwords
!grep -rn "sk[-_]" --include="*.ts" --include="*.go" --include="*.json" --exclude-dir=node_modules --exclude-dir=.next
!grep -rn "secret.*=.*['\"]" --include="*.ts" --include="*.go" --include="*.env*" --exclude-dir=node_modules
!grep -rn "password.*=.*['\"]" --include="*.ts" --include="*.go" --exclude-dir=node_modules
!grep -rn "Bearer " --include="*.ts" --include="*.go" --exclude-dir=node_modules
!grep -rn "authorization.*:" --include="*.ts" --include="*.go" --exclude-dir=node_modules --exclude-dir=_generated
```

### 1b: Environment files committed to git

```
!git ls-files | grep -E "\.env" | grep -v "\.example"
!git ls-files | grep -E "credentials|secret" -i
```

Any `.env` file (except `.env.example`) in git is a **CRITICAL** finding. Check `.gitignore` covers:
- `.env`, `.env.local`, `.env.development`, `.env.production`, `.env.staging`
- `.env*.local`
- Any file containing "secret" or "credential"

### 1c: Git history secrets

```
!git log --all --diff-filter=D --name-only -- "*.env*" 2>/dev/null | head -20
!git log --all -p --follow -S "SECRET" -- "*.env*" 2>/dev/null | head -50
```

If secrets were ever committed and later deleted, they're still in git history. The repo is public — this is an active exposure. Flag for BFG Repo-Cleaner or `git filter-repo` immediately.

### 1d: Sensitive values in documentation

Check `.claude/environment-variables.md` and other docs for actual secret values (not just variable names):

- Variable names, deployment names, project IDs = **OK** (public identifiers)
- Actual secret values, access keys, tokens = **CRITICAL**
- Cloudflare tunnel credentials should be in `~/.cloudflared/` only, never in repo

---

## Section 2: Go API Security

The Go API (`archive/api-go/`) is the cloud execution endpoint on Railway. It receives workflow definitions and executes them.

### 2a: Authentication & authorization

Read `archive/api-go/internal/server/server.go` and all handler files in `archive/api-go/internal/handler/`.

Check:
- [ ] **Do endpoints require authentication?** Currently, the Go API has no auth layer — Convex actions call it server-to-server. Is this acceptable? Document the trust model
- [ ] **Is the Go API publicly accessible?** If Railway exposes it to the internet, anyone can POST workflow definitions. Check if Railway has network restrictions or if auth middleware is needed
- [ ] **Rate limiting** — is there any? Without it, the public endpoint is vulnerable to DoS via expensive workflow executions

### 2b: CORS configuration

Read the `cors()` middleware in `server.go`:

- [ ] **`Access-Control-Allow-Origin: *`** — is this intentional? Document whether the Go API is only called server-to-server (Convex actions) or also from browsers
- [ ] If server-to-server only, CORS headers are unnecessary (they only affect browsers). Consider removing them or restricting to specific origins
- [ ] If browser access is needed, restrict to `https://bnto.io`, `https://*.vercel.app`, `http://localhost:3000`

### 2c: Input validation

Read all handler files (`run.go`, `validate.go`, `workflow.go`, `execution.go`):

- [ ] **Request body size limits** — is `http.MaxBytesReader` used? Without it, an attacker can send a multi-GB JSON body to exhaust memory
- [ ] **JSON decoding** — is `DisallowUnknownFields()` set? (Check `decodeBody` in `response.go`)
- [ ] **Timeout enforcement** — can a caller set an arbitrarily long timeout? What's the max?
- [ ] **Workflow definition validation** — are definitions validated before execution? Does the engine reject unknown node types, malformed configs?

### 2d: Execution sandboxing

Read `run.go` for the execution flow:

- [ ] **Temp directory cleanup** — are temp dirs always cleaned up (even on panic)?
- [ ] **Environment variable isolation** — the `envMu` mutex serializes executions using env vars. Is this safe? Could a race condition leak one execution's file paths to another?
- [ ] **File path traversal** — when downloading from R2, could a malicious session ID or filename cause path traversal? (e.g., `../../etc/passwd`)
- [ ] **Resource limits** — is there a max execution time? Max memory? Max output file size?
- [ ] **Shell command injection** — if the engine has a shell/command node type, are commands sanitized?

### 2e: Error information disclosure

- [ ] **Do error responses leak internal paths, stack traces, or implementation details?** Check all `writeError` calls
- [ ] **Are Go panics caught?** An unrecovered panic returns a bare 500 with no body, which is fine. But does the Railway log include the full stack trace?

---

## Section 3: Convex Function Security

Convex is the data layer. Every query and mutation is a public API endpoint callable by any authenticated (or anonymous) client.

### 3a: Auth enforcement

Read every file in `packages/@bnto/backend/convex/` (excluding `_generated/` and `_helpers/`):

For EACH exported query/mutation, verify:
- [ ] **Mutations check `getAppUserId(ctx)`** and reject if null
- [ ] **Queries that return user data filter by `userId`** — no query returns another user's data
- [ ] **Resource ownership verified** — mutations that modify a resource check `resource.userId === userId` before modifying
- [ ] **No mutation accepts a `userId` parameter** — always derive from session via `getAppUserId(ctx)`

### 3b: Input validation

For EACH exported query/mutation:
- [ ] **All args use Convex validators** (`v.string()`, `v.id()`, etc.)
- [ ] **No `v.any()` on external-facing functions** — `v.any()` is acceptable on internal mutations, but public mutations should validate structure
- [ ] **String inputs are bounded** — very long strings could be used for storage abuse. Check if Convex has built-in limits or if explicit length checks are needed

### 3c: Quota enforcement

Read `executions.ts`:
- [ ] **`enforceQuota()` runs before execution starts** — not after
- [ ] **Quota increment is atomic with execution start** — no TOCTOU race between checking quota and incrementing
- [ ] **Anonymous user limits work** — `ANONYMOUS_RUN_LIMIT` env var is read and enforced
- [ ] **Can quota be bypassed?** Can a client call `executeWorkflow` directly (it's `internalAction`, so no)? Can they start multiple executions concurrently to race past the quota check?

### 3d: Upload security

Read `uploads.ts` and `_helpers/upload_validation.ts`:
- [ ] **File type allowlist enforced server-side** — not just client-side
- [ ] **File size limits enforced server-side per plan**
- [ ] **Presigned URL expiry is reasonable** (not hours/days)
- [ ] **Filename sanitization** — path traversal characters stripped
- [ ] **Session ID generation** — uses `randomUUID()` (cryptographically random)

### 3e: Download security

Read `downloads.ts`:
- [ ] **Download URLs are scoped to the user's execution** — can't download another user's output files
- [ ] **R2 keys are validated** — can a client request a download for an arbitrary R2 key?
- [ ] **Cleanup** — are R2 objects deleted after download or on a TTL?

### 3f: Internal vs public functions

- [ ] **Are `internalMutation` and `internalAction` used correctly?** These should NOT be callable from the client
- [ ] **Public queries/mutations are intentionally public** — no accidental exposure of internal functions

---

## Section 4: Web Application Security

The Next.js web app on Vercel is the primary client.

### 4a: Security headers

Read `apps/web/next.config.ts` for response headers:

- [ ] **Content-Security-Policy (CSP)** — is it configured? Should restrict script sources, frame ancestors
- [ ] **X-Frame-Options** — prevents clickjacking. Should be `DENY` or `SAMEORIGIN`
- [ ] **X-Content-Type-Options** — should be `nosniff`
- [ ] **Referrer-Policy** — should be `strict-origin-when-cross-origin` or stricter
- [ ] **Permissions-Policy** — restrict unnecessary browser APIs (camera, microphone, geolocation)

If not in `next.config.ts`, check if they're configured in Vercel dashboard or `vercel.json`.

### 4b: XSS vectors

Search for dangerous patterns in `apps/web/`:

```
!grep -rn "dangerouslySetInnerHTML" apps/web/ --include="*.tsx" --include="*.ts"
!grep -rn "innerHTML" apps/web/ --include="*.tsx" --include="*.ts"
```

For each finding, verify the content is sanitized or comes from a trusted source (not user input).

Also check:
- [ ] **Workflow names, descriptions, and node labels** — are they rendered as text content or injected as HTML?
- [ ] **Execution log output** — rendered in `<pre>` / `<code>` blocks (safe) or interpolated?
- [ ] **URL parameters** — any search params rendered directly in the page?

### 4c: Auth cookie security

Read `middleware.ts` and `apps/web/app/providers/`:

- [ ] **Session cookies are `httpOnly`** — JavaScript can't read them (Better Auth default)
- [ ] **Session cookies are `secure`** — only sent over HTTPS (Better Auth uses `__Secure-` prefix in prod)
- [ ] **Session cookies have `sameSite`** — prevents CSRF. Check Better Auth config
- [ ] **Sign-out clears server session** — not just client-side cookie deletion
- [ ] **The signout signal cookie** (`bnto-signout`) — is it `httpOnly: false` intentionally? (Yes — JS needs to set it. But verify the TTL is short, ~10s)

### 4d: CSRF protection

- [ ] **Convex mutations use session tokens** — the Better Auth session token acts as a CSRF token since it's `httpOnly` and verified server-side
- [ ] **No custom API routes that accept form submissions without CSRF tokens** — check `apps/web/app/api/` routes

### 4e: Client-side data exposure

- [ ] **`NEXT_PUBLIC_*` env vars** — only Convex URL and site URL should be public. No secrets with `NEXT_PUBLIC_` prefix
- [ ] **Client bundle** — does the build include any server-only code? Check for `"use server"` boundary discipline
- [ ] **React Query cache** — is sensitive data in the cache cleared on sign-out?

---

## Section 5: Infrastructure & Cloud Services

### 5a: GitHub repository

```
!gh repo view --json isPrivate,defaultBranchRef,hasIssuesEnabled,hasWikiEnabled
!gh api repos/{owner}/{repo}/branches/main/protection 2>/dev/null || echo "No branch protection"
```

Check:
- [ ] **Repo visibility** — is it public or private? If public, all code and git history is visible
- [ ] **Branch protection on `main`** — require PR reviews, status checks
- [ ] **GitHub Actions secrets** — are they scoped correctly? Check `.github/workflows/`
- [ ] **Dependabot/security alerts** — enabled?
- [ ] **No webhook URLs with tokens** in repo settings

### 5b: Vercel

- [ ] **Environment variables** — `NEXT_PUBLIC_*` vars only contain public identifiers (Convex URLs)
- [ ] **No preview deployment leaks** — preview deployments on PRs could expose the app to unreviewed code. Are preview deployments restricted?
- [ ] **Build logs** — Vercel build logs could contain env var values if they're echoed. Are logs public?

### 5c: Railway

- [ ] **Network access** — is the Go API endpoint publicly accessible or restricted to Convex's IP range?
- [ ] **Environment variables** — R2 credentials are set as Railway variables (not in code). Verified
- [ ] **Health check** — `/health` endpoint exists and is configured in `railway.toml`
- [ ] **Restart policy** — `ON_FAILURE` with max retries prevents infinite crash loops

### 5d: Cloudflare R2

- [ ] **R2 API tokens** — scoped to bnto buckets only (Object Read & Write), not account-wide
- [ ] **Bucket access** — are buckets public or private? They should be private (presigned URLs only)
- [ ] **Separate dev/prod buckets** — `bnto-transit-dev` vs `bnto-transit`. Verified in env docs
- [ ] **Object lifecycle** — 1-hour TTL on transit objects. Is this enforced via R2 lifecycle rules or just application-level cleanup?
- [ ] **CORS on R2 bucket** — browser uploads via presigned URLs need CORS configured on the bucket. Is it restricted to `bnto.io` and `localhost`?

### 5e: Convex deployment

- [ ] **Dev vs prod deployments** — separate, with separate env vars
- [ ] **Convex dashboard access** — who has access? Is 2FA enabled?
- [ ] **No sensitive data in Convex function logs** — check if error messages or execution results leak PII

### 5f: Cloudflare tunnel (dev only)

- [ ] **Tunnel credentials** — in `~/.cloudflared/` only, never in repo
- [ ] **Tunnel URL** — `api-dev.bnto.io` points to localhost. Is this URL accessible when the tunnel is down? (Should 502)
- [ ] **Not used in production** — prod uses Railway directly

---

## Section 6: Dependency Security

### 6a: Known vulnerabilities

```
!cd apps/web && pnpm audit --audit-level=high 2>/dev/null | head -30
!cd /Users/ryan/Code/bnto && govulncheck ./engine/... 2>/dev/null || echo "govulncheck not installed — run: go install golang.org/x/vuln/cmd/govulncheck@latest"
!cd /Users/ryan/Code/bnto && govulncheck ./archive/api-go/... 2>/dev/null || echo "govulncheck not installed"
```

### 6b: Dependency review

- [ ] **No unnecessary dependencies** — compare `package.json` deps against actual imports
- [ ] **Go dependencies minimal** — check `archive/engine-go/go.mod` and `archive/api-go/go.mod`
- [ ] **Lock files committed** — `pnpm-lock.yaml` and `go.sum` should be in git

### 6c: Supply chain

- [ ] **GitHub Actions** — are action versions pinned to SHA (not `@v3` tags that can be overwritten)?
- [ ] **pnpm `onlyBuiltDependencies`** — explicit allowlist for native deps that run install scripts

---

## Section 7: Open Source Readiness

The repo IS public. Every check below applies NOW, not as a future consideration:

### 7a: Sensitive content in code

```
!grep -rn "TODO.*secret\|TODO.*credential\|TODO.*password\|TODO.*token\|TODO.*key" --include="*.ts" --include="*.go" --exclude-dir=node_modules --exclude-dir=_generated
!grep -rn "HACK\|FIXME\|XXX" --include="*.ts" --include="*.go" --exclude-dir=node_modules --exclude-dir=_generated | head -20
!grep -rn "competitor\|pricing\|revenue\|valuation" --include="*.ts" --include="*.go" --include="*.md" --exclude-dir=node_modules --exclude-dir=_generated | head -20
```

### 7b: Test fixtures

```
!grep -rn "@[a-zA-Z0-9.]+\.[a-z]{2,}" engine/tests/ engine/examples/ --include="*.json" | grep -v "example.com\|test.com\|bnto" | head -10
```

Check for real email addresses, phone numbers, or PII in test data.

### 7c: License compliance

- [ ] **MIT license file exists** at repo root
- [ ] **No vendored code with incompatible licenses** — check `vendor/` or embedded third-party code
- [ ] **Dependency licenses compatible** — no GPL deps in an MIT project (for linked/bundled deps)

---

## Section 8: Attack Surface Summary

After completing all checks, produce a summary table:

```
| Surface               | Risk Level | Key Findings                                  |
|-----------------------|------------|-----------------------------------------------|
| Secret Management     | ...        | ...                                           |
| Go API Auth           | ...        | ...                                           |
| Go API Input          | ...        | ...                                           |
| Go API Execution      | ...        | ...                                           |
| Convex Auth           | ...        | ...                                           |
| Convex Input          | ...        | ...                                           |
| Convex Uploads        | ...        | ...                                           |
| Web App Headers       | ...        | ...                                           |
| Web App XSS           | ...        | ...                                           |
| Web App Auth          | ...        | ...                                           |
| GitHub Repo           | ...        | ...                                           |
| Vercel                | ...        | ...                                           |
| Railway               | ...        | ...                                           |
| Cloudflare R2         | ...        | ...                                           |
| Convex Deployment     | ...        | ...                                           |
| Dependencies          | ...        | ...                                           |
| Open Source Readiness  | ...        | ...                                           |
```

Risk levels: `LOW` (best practice met), `MEDIUM` (improvement possible), `HIGH` (should fix before production traffic), `CRITICAL` (fix immediately — active vulnerability).

---

## Section 9: Prioritized Action Items

List specific actions ordered by priority:

### Critical (fix now)
Items that represent active vulnerabilities or data exposure.

### High (fix before production)
Items that would be exploitable under real traffic.

### Medium (fix soon)
Best practices not yet followed, defense-in-depth gaps.

### Low (when convenient)
Hardening measures, nice-to-haves, future considerations.

For each item, include:
- **What**: The specific issue
- **Where**: File path and line number (or dashboard/service)
- **Why**: What could go wrong
- **How**: Specific fix recommendation
