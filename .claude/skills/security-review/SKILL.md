---
name: security-review
description: Security posture review across codebase, cloud services, and attack surfaces
---

# Security Review

Comprehensive security audit of the bnto project across all surfaces: codebase, Rust engine (CLI + WASM), Convex functions, cloud infrastructure (Vercel, Convex), GitHub repo, and client-side attack vectors.

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
.claude/rules/code-standards.md             # Code standards
```

**Read ALL of these files now.** The audit sections below reference these documents. You need the full picture before scanning.

## Step 0b: Activate Your Personas

**Always invoke the security engineer persona first:** `/security-engineer` — your primary persona. Owns the entire attack surface, trust boundaries, and defense-in-depth strategy.

**Then invoke domain personas** for the specific packages you're auditing:

| Auditing files in...                              | Domain persona skill |
| ------------------------------------------------- | -------------------- |
| `engine/`                                         | `/rust-expert`       |
| `apps/web/`                                       | `/frontend-engineer` |
| `packages/core/`                                  | `/core-architect`    |
| `packages/@bnto/backend/`, `packages/@bnto/auth/` | `/backend-engineer`  |

**Invoke `/security-engineer` and all matching domain persona skills now.** The security persona gives you the adversarial mindset and cross-cutting awareness. The domain personas give you package-specific patterns, gotchas, and quality standards — e.g., Rust `unsafe` blocks, React XSS vectors, Convex auth enforcement patterns. A full security audit requires both perspectives.

---

## Section 1: Secret & Credential Scanning

Scan the **entire repo** for leaked secrets, credentials, and sensitive values. This is the highest-priority check.

### 1a: Hardcoded secrets in code

Search for patterns that indicate hardcoded secrets:

```
# API keys, tokens, passwords
!grep -rn "sk[-_]" --include="*.ts" --include="*.rs" --include="*.json" --exclude-dir=node_modules --exclude-dir=.next --exclude-dir=target
!grep -rn "secret.*=.*['\"]" --include="*.ts" --include="*.rs" --include="*.env*" --exclude-dir=node_modules --exclude-dir=target
!grep -rn "password.*=.*['\"]" --include="*.ts" --include="*.rs" --exclude-dir=node_modules --exclude-dir=target
!grep -rn "Bearer " --include="*.ts" --include="*.rs" --exclude-dir=node_modules --exclude-dir=target
!grep -rn "authorization.*:" --include="*.ts" --include="*.rs" --exclude-dir=node_modules --exclude-dir=_generated --exclude-dir=target
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
- No infrastructure credentials or tokens committed to the repo

---

## Section 2: CLI & Engine Security

The Rust engine powers both the CLI (`bnto-cli`) and browser execution (WASM). The CLI has full filesystem access — it's the highest-privilege execution target.

### 2a: Recipe definition trust

Read `engine/crates/bnto-engine/src/` for definition parsing:

- [ ] **Recipe definitions are validated before execution** — schema validation, known node types, valid connections
- [ ] **Malformed `.bnto.json` files produce clean errors** — no panics on untrusted input
- [ ] **No `unwrap()` on values derived from recipe definitions** — use `?` or `expect()` with descriptive messages

### 2b: CLI filesystem security

Read `engine/crates/bnto-cli/src/`:

- [ ] **File paths from CLI args are sanitized** — no path traversal via `../` in output directories
- [ ] **Output files don't escape the working directory** — verify output paths are resolved relative to CWD
- [ ] **Filenames from input files are sanitized before use in output** — prevent directory traversal via crafted filenames
- [ ] **No shell command injection** — if any node type executes external commands, arguments must be passed as arrays, not interpolated into shell strings

### 2c: Engine error handling

- [ ] **No `unsafe` blocks** without explicit justification and safety comments
- [ ] **Panic hook configured for WASM** — `console_error_panic_hook` for debuggable stack traces
- [ ] **Error types are domain-specific** — `thiserror` derive on enum variants, not string errors
- [ ] **No secret or path information leaked in error messages** — errors visible to users in both CLI output and browser console

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

### 3c: Upload security

Read `uploads.ts` and `_helpers/upload_validation.ts`:

- [ ] **File type allowlist enforced server-side** — not just client-side
- [ ] **File size limits enforced server-side per plan**
- [ ] **Presigned URL expiry is reasonable** (not hours/days)
- [ ] **Filename sanitization** — path traversal characters stripped
- [ ] **Session ID generation** — uses `randomUUID()` (cryptographically random)

### 3d: Download security

Read `downloads.ts`:

- [ ] **Download URLs are scoped to the user's execution** — can't download another user's output files
- [ ] **R2 keys are validated** — can a client request a download for an arbitrary R2 key?
- [ ] **Cleanup** — are R2 objects deleted after download or on a TTL?

### 3e: Internal vs public functions

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

- [ ] **Session cookies are `httpOnly`** — JavaScript can't read them (`@convex-dev/auth` default)
- [ ] **Session cookies are `secure`** — only sent over HTTPS
- [ ] **Session cookies have `sameSite`** — prevents CSRF. Check `@convex-dev/auth` config
- [ ] **Sign-out clears server session** — not just client-side cookie deletion
- [ ] **The signout signal cookie** (`bnto-signout`) — is it `httpOnly: false` intentionally? (Yes — JS needs to set it. But verify the TTL is short, ~10s)

### 4d: CSRF protection

- [ ] **Convex mutations use session tokens** — the `@convex-dev/auth` session token acts as a CSRF token since it's `httpOnly` and verified server-side
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

### 5c: Convex deployment

- [ ] **Dev vs prod deployments** — separate, with separate env vars
- [ ] **Convex dashboard access** — who has access? Is 2FA enabled?
- [ ] **No sensitive data in Convex function logs** — check if error messages or execution results leak PII

---

## Section 6: Dependency Security

### 6a: Known vulnerabilities

```
!cd apps/web && pnpm audit --audit-level=high 2>/dev/null | head -30
!cd /Users/ryan/Code/bnto/engine && cargo audit 2>/dev/null || echo "cargo-audit not installed — run: cargo install cargo-audit"
```

### 6b: Dependency review

- [ ] **No unnecessary dependencies** — compare `package.json` deps against actual imports
- [ ] **Rust dependencies minimal** — check `engine/Cargo.toml` workspace deps
- [ ] **Lock files committed** — `pnpm-lock.yaml` and `Cargo.lock` should be in git

### 6c: Supply chain

- [ ] **GitHub Actions** — are action versions pinned to SHA (not `@v3` tags that can be overwritten)?
- [ ] **pnpm `onlyBuiltDependencies`** — explicit allowlist for native deps that run install scripts

---

## Section 7: Open Source Readiness

The repo IS public. Every check below applies NOW, not as a future consideration:

### 7a: Sensitive content in code

```
!grep -rn "TODO.*secret\|TODO.*credential\|TODO.*password\|TODO.*token\|TODO.*key" --include="*.ts" --include="*.rs" --exclude-dir=node_modules --exclude-dir=_generated --exclude-dir=target
!grep -rn "HACK\|FIXME\|XXX" --include="*.ts" --include="*.rs" --exclude-dir=node_modules --exclude-dir=_generated --exclude-dir=target | head -20
!grep -rn "competitor\|pricing\|revenue\|valuation" --include="*.ts" --include="*.rs" --include="*.md" --exclude-dir=node_modules --exclude-dir=_generated --exclude-dir=target | head -20
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
| CLI Security          | ...        | ...                                           |
| Engine Error Handling  | ...        | ...                                           |
| Convex Auth           | ...        | ...                                           |
| Convex Input          | ...        | ...                                           |
| Convex Uploads        | ...        | ...                                           |
| Web App Headers       | ...        | ...                                           |
| Web App XSS           | ...        | ...                                           |
| Web App Auth          | ...        | ...                                           |
| GitHub Repo           | ...        | ...                                           |
| Vercel                | ...        | ...                                           |
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
