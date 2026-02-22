# Open Source Strategy

**Last Updated:** February 2026
**Status:** Planned -- execute after security audit passes

---

## Decision

**Full open source.** Make the entire monorepo public under the MIT license. No mirror, no partial visibility, no complexity. Just flip the repo to public after a security audit confirms it's safe to do so.

The competitive advantage is the community, the product execution, and the hosted service -- not the code. Being open is itself a differentiator in the workflow automation space.

---

## Why Full Open Source

### The Moat Is Execution, Not Code

Bnto is a workflow automation engine with a cloud-hosted convenience layer. The value proposition is being a better workflow tool -- simpler than n8n, more developer-friendly than Zapier, with a Go engine that runs locally or in the cloud. That execution quality and community can't be replicated by forking a GitHub repo. Someone could clone every line of code and still lack the hosted infrastructure, the community, and the product taste.

The codebase is well-engineered orchestration -- Go engine, Convex, Next.js, React Query, Wails -- but none of it is a proprietary algorithm or trade secret. It's standard workflow automation patterns done with care.

### Being Open IS the Differentiator

The workflow automation space has a spectrum of openness:

| Platform | Open Source? |
|---|---|
| n8n | Fair-code (source-available, restrictive license) |
| Temporal | Open source (MIT) |
| Zapier | No |
| Make | No |
| GitHub Actions | No (runner is open) |
| **Bnto** | **Yes (MIT)** |

Being fully MIT-licensed is a stronger position than n8n's fair-code model. It signals: *"We're building this for you, not extracting from you."*

### n8n Precedent

n8n proved that open source workflow automation builds massive community:
- 50K+ GitHub stars
- Vibrant community of contributors and integration builders
- Sustainable business with hosted service
- Their fair-code license still generates debate. MIT is simpler and more trusted.

### Bluesky Precedent

Bluesky open sourced **everything** -- the AT Protocol (MIT + Apache 2.0) and the full social app (MIT). Despite the entire codebase being public:
- No meaningful fork has competed with them
- They raised $97M Series B at $700M valuation (Jan 2025)
- Open source actually *accelerated* ecosystem growth

### Maximum Portfolio Impact

The full stack visible -- from the Go engine to the transport-agnostic core API to the Next.js composition layer -- tells a complete architectural story:

- **The layered architecture** -- transport-agnostic API, adapter pattern, runtime detection
- **The Bento Box Principle** -- file organization, single responsibility, compound composition
- **Go engine design** -- node types, registry, orchestration, validation
- **Desktop via Wails** -- same frontend, different transport
- **The full composition** -- how a thin app layer snaps core + UI together

A private repo is a bullet point on a resume. A public repo is proof.

### Simplicity

No mirror repos, no GitHub Actions to maintain, no "which parts are public?" decisions on every commit. One repo, one visibility setting. Ship code, it's public. Done.

---

## Monetization Compatibility

Fully open source platforms monetize successfully through service-layer revenue:

| Company | Open Source | Monetized |
|---|---|---|
| **n8n** | Workflow engine (fair-code) | Hosted service, enterprise features |
| **Temporal** | Orchestration engine (MIT) | Temporal Cloud (hosted service) |
| **Supabase** | Full platform | Hosted service, enterprise features |
| **Cal.com** | Full scheduling platform | Hosted Pro service |
| **GitLab** | Core platform (MIT) | Premium/Ultimate tiers, hosted service |

Bnto's planned revenue streams (hosted cloud execution, premium workflow features, team features) are **service-layer monetization** -- they don't depend on the code being private. The code is the engine; the hosted service is the product.

---

## Pre-Launch Security Audit

**Before flipping the repo to public, this audit must pass.** The code isn't secret, but we need to ensure nothing unsafe is exposed.

### 1. Git History Scan

- [x] Scan entire git history for accidentally committed secrets (API keys, tokens, passwords) — *Done Feb 2026: gitleaks found 2 false positives (test fixture `figd_secret123` in manager_test.go), no real secrets*
- [x] Use a tool like `gitleaks`, `trufflehog`, or `git-secrets` to automate the scan — *Done: gitleaks installed and run, 119 commits scanned*
- [ ] If secrets are found in history: rotate the credentials immediately, then either rewrite history (`git filter-repo`) or start a fresh repo with a squashed initial commit — *N/A: no real secrets found*

### 2. Environment Variable Audit

- [x] Verify every secret is in `.env` files (which are `.gitignore`d), not hardcoded — *Done Feb 2026: all secrets use env vars, Go secrets via OS keychain*
- [x] Confirm `.env.example` files contain only placeholder values, never real credentials — *Done: root and apps/web .env.example checked, only placeholders*
- [x] Review `next.config.ts`, `convex.config.ts`, `Taskfile.yml`, and any config files for leaked values — *Done: all clean*
- [x] Review Go code for hardcoded URLs, keys, or credentials — *Done: no hardcoded secrets, tests use httptest mock servers*

### 3. Auth Flow Security Review

- [ ] Auth is secured by proper validation, not by obscurity (hidden routes, unlisted endpoints)
- [ ] OAuth redirect URIs are validated server-side
- [ ] Session tokens are httpOnly, secure, sameSite
- [ ] Any invite/beta code system can't be bypassed by knowing the URL

### 4. API Route Authorization

- [ ] API routes are gated by appropriate auth checks
- [ ] Convex functions verify caller authorization
- [ ] Go API endpoints validate authentication tokens
- [ ] No functionality is accessible to unauthenticated users that shouldn't be

### 5. Input Validation & Injection

- [ ] All Convex mutations validate inputs with Convex validators
- [ ] Go engine validates workflow definitions before execution
- [ ] No raw user input rendered without sanitization (XSS vectors)
- [ ] Presigned URL generation validates file types and sizes

### 6. Dependency Audit

- [x] Run `pnpm audit` -- no critical or high vulnerabilities — *Done Feb 2026: "No known vulnerabilities found"*
- [x] Run `go vet` and review for security concerns — *Done: passes clean (TUI package excluded via build tag since it's on hold)*
- [ ] Review `package.json` and `go.mod` for unnecessary dependencies that expand attack surface

### 7. Sensitive File Review (BLOCKER)

- [x] **Review `.claude/strategy/` before going public.** — *Done Feb 2026: pricing/tier details removed from cloud-desktop-strategy.md and PLAN.md, moved to Notion. Architecture and engineering docs kept (showcase value). seo-monetization.md deleted and content distributed to appropriate docs.*
- [ ] Review any comments in code that reference internal plans, personal notes, or competitive analysis
- [ ] Ensure no test fixtures contain real user data or API responses with PII
- [x] Review `.claude/` broadly -- architecture docs are fine (they showcase engineering standards), but anything business-strategic must be reviewed — *Done: archive/ nuked, all strategy docs reviewed, SCREAMING_SNAKE filenames standardized to kebab-case, stale cross-references fixed*

---

## Execution Plan

### Phase 1: Security Audit

- [ ] Run all 7 audit checks above
- [ ] Fix any issues found
- [ ] Re-run to confirm clean

### Phase 2: Prepare for Public

- [x] **Review `.claude/strategy/` docs** -- migrate business-sensitive content to private location if needed — *Done Feb 2026: pricing moved to Notion, archive cleaned, stale refs fixed*
- [x] Add MIT LICENSE file to repo root — *Done: LICENSE exists (MIT, 2024-2026 Develonaut)*
- [x] Write a showcase README -- architecture diagram, quick start, design philosophy — *Done: README.md exists with architecture, quick start, CLI usage, project structure*
- [ ] Add CONTRIBUTING.md if accepting community contributions

### Phase 3: Go Public

- [ ] Flip GitHub repo visibility to public
- [ ] Write a blog post / social media announcement
- [ ] Share on relevant communities (r/selfhosted, r/golang, r/webdev, r/reactjs, dev.to, Hacker News, workflow automation forums)

---

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Someone forks and builds a competitor | The moat is hosted service + community + execution quality. A fork with zero infrastructure isn't a threat |
| Accidental secret exposure | Pre-launch security audit catches this. Rotate any credentials found before going public |
| Strategy docs reveal business plans | Review `.claude/strategy/` and migrate sensitive content before going public |
| Code quality not yet at showcase level | Only go public after ship-ready state. The code must be exemplary -- it's a portfolio piece |
| Maintenance burden of community contributions | Start with issues only, accept PRs cautiously. No obligation to merge anything |

---

## Research & References

- [n8n -- GitHub](https://github.com/n8n-io/n8n) -- Fair-code workflow automation, 50K+ stars
- [Temporal -- GitHub](https://github.com/temporalio/temporal) -- MIT-licensed workflow orchestration
- [Bluesky AT Protocol -- GitHub](https://github.com/bluesky-social/atproto) -- Full protocol, MIT + Apache 2.0
- [Bluesky Social App -- GitHub](https://github.com/bluesky-social/social-app) -- Full app, MIT license
- [GitLab Open Core Model](https://about.gitlab.com/company/) -- Reference for open core in practice
- [Supabase Open Source](https://supabase.com/open-source) -- Client libs open, hosted service monetized
- [Cal.com Open Source](https://cal.com/open) -- Scheduling engine open, Pro service monetized
