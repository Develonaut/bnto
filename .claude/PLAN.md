# Bnto — Build Plan

**Last Updated:** February 24, 2026
**This is the single source of truth for what's been built, what's in progress, and what's next.**

Skills and commands that reference the plan read this file. Update it after every sprint.

---

## How This Works

Tasks are organized into **sprints** (features) and **waves** (dependency groups within a sprint). All tasks in a wave can be picked up in parallel by agents. Waves must complete in order before the next wave starts.

```
- [ ]              → available, grab it
- [ ] **CLAIMED**  → an agent is working on this, pick something else
- [x]              → done
```

**Scope rule:** Each task targets ONE package. Don't touch files outside the tagged package unless the task explicitly says so.

**Co-location decision (Feb 2026):** UI components and editor features live in `apps/web` for now. No separate `@bnto/ui` or `@bnto/editor` packages until there's a real second consumer (desktop app). Engine, core API, and data layer logic stays in `@bnto/core`.

---

## Current State

- **Active:** Sprint 2A Wave 5 IN PROGRESS — E2E test specs written, pre-execution screenshots captured, progress-aware helpers added. **BLOCKER: Anonymous auth fails silently in Playwright browser (`signIn("anonymous")` from `@convex-dev/auth` never establishes session). Full pipeline verification blocked.**
- **Next:** Fix anonymous auth in dev environment → complete Wave 5 pipeline verification → **Convex dev cleanup (Better Auth remnants)** → Sprint 2.5 (resume polish) → Sprint 3 (dashboard + quota)
- **Auth:** Migrated to `@convex-dev/auth`. Anonymous sessions create real `users` rows. Integration tests (Wave 3) complete. `AUTH_SECRET` env var required in Convex deployments.
- **Engine:** Complete. Go CLI with 10 node types (>90% coverage), Go HTTP API on Railway, BntoService shared API layer.
- **Web app:** Next.js on Vercel. Auth, SEO tool pages, execution UI, landing pages — all built. Needs pipeline verification.
- **Packages:** `@bnto/core` (layered singleton), `@bnto/auth` (`@convex-dev/auth` wrappers), `@bnto/backend` (Convex schema + functions). UI co-located in `apps/web/components/`.

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine: 10 node types, orchestration, validation, storage, secrets, path resolution
- [x] Go API server: HTTP handlers wrapping BntoService (apps/api/), deployed to Railway
- [x] Contract tests: Go JSON responses match @bnto/core TypeScript types
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38 hooks
- [x] @bnto/auth: `@convex-dev/auth` integration (migrated from Better Auth — see decisions/auth-evaluation.md)
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, run counter fields
- [x] Web app: Landing pages (Mainline template — hero, features, pricing, about, FAQ, contact, privacy, footer)
- [x] Web app: Auth flow (sign-in, sign-up, sign-out with signal cookie, proxy route protection)
- [x] Web app: SEO infrastructure (bnto-registry.ts, [bnto]/page.tsx, generateStaticParams, generateMetadata, BntoJsonLd, sitemap.ts, llms.txt, robots.txt)
- [x] Web app: Middleware (canonical URL normalization, auth routing, bnto slug pass-through)
- [x] All 6 Tier 1 fixtures in engine/tests/fixtures/workflows/
- [x] Playwright E2E: user journeys, visual parity tests, execution flow tests (9 tests, 8 screenshots)
- [x] Execution UI: RunButton, ExecutionProgress (real-time), ExecutionResults (R2 download)
- [x] Predefined execution path: BntoPageShell → useRunPredefined → Convex startPredefined → Railway Go API
- [x] Download infrastructure: @bnto/core download client/service/adapter/hook chain
- [x] R2 infrastructure: buckets (dev + prod), presigned URLs, CORS, env vars across Convex/Vercel/Railway
- [x] Auth integration tests: anonymous execution (A1-A7), conversion flow (C1-C3), auth lifecycle (S1-S3) — see journeys/auth.md
- [x] Codebase polish: Node.js subpath imports, PascalCase components, camelCase hooks, dot-notation primitives, layout/typography primitives, CSS animation system

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in Notion ("SEO & Monetization Strategy").

| Sprint | What Ships | Revenue Implication |
|--------|-----------|---------------------|
| Sprint 3 | Dashboard + run quota tracking | Accounts exist. Run counter visible. Upgrade prompt scaffolded. |
| Sprint 4 | JSON editor | Power users self-identify. Custom flows are a Pro signal. |
| Sprint 5-6 | Desktop app | Top-of-funnel. Word of mouth begins. |
| Sprint 7 | Stripe + quota enforcement | **First revenue possible.** Free capped at 25/month. Pro at $8/month. |

---

## Completed Sprints (collapsed)

### Phase 0: Foundation — COMPLETE
Monorepo restructuring, engine solidification with TDD (>90% coverage on all 10 node types), integration test fixtures, CLI smoke tests, Go API server, Convex setup, web app shell, @bnto/core hooks.

### Sprint 1: Infrastructure Migration — COMPLETE
Moved from Railway/Convex Auth to Vercel/Better Auth. Auth provider, Convex schema, Vercel deployment, proxy middleware, sign-in/sign-up/sign-out pages, route protection. Wave 4 (auth verification) was skipped — gaps caught and resolved in Sprint 2A.

### Sprint 2: Predefined Bntos + Cloud Execution — Waves 1-4 COMPLETE
6 Tier 1 fixtures, SEO URL routing, bnto registry, tool page UI (file drop, per-bnto config), R2 file transit, Railway deployment, env config (R2/Convex/Vercel/Railway), execution UI (RunButton, ExecutionProgress, ExecutionResults), predefined execution path. Wave 5 (pipeline verification) blocked by auth — deferred to Sprint 2A Wave 5.

### Sprint 2A: Auth Fix — Waves 1-3 COMPLETE
Evaluated `@convex-dev/auth` vs fixing Better Auth. Chose `@convex-dev/auth` (eliminates JWT race condition). Full migration: auth provider, anonymous sessions with real `users` rows, proxy middleware, AppGate removal. Integration tests: anonymous execution (A1-A7), conversion flow (C1-C3), auth lifecycle (S1-S3). See `decisions/auth-evaluation.md`.

### Sprint 2.5: Codebase Polish — Waves 1-2 COMPLETE
Node.js subpath imports (`#components/*`, `#lib/*`), camelCase file rename (hooks, utils, lib), PascalCase component rename, dot-notation primitive wrappers, Button audit/migration. Font review (DM Sans → Geist evaluation) deferred.

---

## Phase 1: Web App + Cloud Execution (continued)

### Sprint 2A: Auth Fix — Waves 4-5 (ACTIVE)

#### Wave 4 (sequential — core integration tests against real Convex dev)

**Why this layer exists:** `convex-test` (Wave 3) validates logic in-memory. Playwright E2E (Wave 5) validates user journeys through a browser. This layer sits between — it calls real Convex dev functions through `@bnto/core`'s imperative API without browser overhead. It catches: wrong env vars, missing indexes, auth provider misconfiguration, schema migration issues, R2 connectivity.

**Auth approach:** `ConvexHttpClient` calls `@convex-dev/auth`'s `signIn` action directly — returns a JWT token. Set it with `client.setAuth(token)` and all subsequent calls are authenticated. No React, no browser. See `decisions/core-integration-testing.md`.

**Test infrastructure:** Requires `task dev:all` (Convex dev + Go API via tunnel + R2 dev bucket).

- [x] `@bnto/core` — **Test harness setup:** `ConvexHttpClient` factory with anonymous + password auth, test lifecycle helpers (cleanup), Vitest integration config (separate from unit tests, longer timeouts). File: `packages/core/src/__tests__/integration/setup.ts`
- [x] `@bnto/core` — **Auth integration tests:** Anonymous sign-in, authenticated client calls, unauthenticated rejection, password sign-up/sign-in, anonymous → password upgrade (userId NOT preserved via ConvexHttpClient — browser E2E needed). Files: `auth-lifecycle.test.ts` (S1-S3, 16 tests), `conversion-flow.test.ts` (C1-C3, 14 tests). Also fixed: `@convex-dev/auth` v0.0.90 response parsing in `setup.ts`, broken `convex.config.ts` component import, `auth.config.ts` provider config, deprecated Vitest `poolOptions`
- [x] `@bnto/core` — **Execution integration tests:** `core.executions.startPredefined()` against real Convex dev — creates execution record, increments runsUsed, enforces quota. Verify status transitions (pending → running → completed/failed) via polling. Also fixed: unified run limit system (env-driven via `_helpers/run_limits.ts`, single source of truth for anonymous=3, free=25). File: `execution.test.ts` (17 tests)
- [x] `@bnto/core` — **Upload/download integration tests:** `core.uploads.generateUrls()` returns valid R2 presigned URLs, upload succeeds, after execution `core.downloads.getDownloadUrls()` returns valid download URLs. Full transit: upload → execute → download against R2 dev bucket. **Coverage gap:** The full file pipeline (browser → R2 → Go engine → R2 → browser) is untested end-to-end at the integration layer. This is the highest-priority gap in our test coverage.

#### Wave 5 (sequential — verify full pipeline in browser)

**Note on Sprint 2 stash:** `git stash@{0}` contains Sprint 2 Wave 5 work (integration test spec, playwright config, data-session attributes). This was written against the old Better Auth system — **review and adapt, don't blindly pop.** The test structure may be reusable but auth wiring is obsolete.

- [x] `apps/web` — Playwright E2E integration tests: full pipeline (upload → R2 → Go engine → R2 → download) using shared engine test fixtures. Separate `playwright.integration.config.ts` targets `task dev:all` on port 4000. Progress-aware test helpers added (`waitForExecutionStatus`, `waitForPhase`, `captureTransientPhase`, `captureUploadProgress`). Data attributes added to UploadProgress, ExecutionProgress, ExecutionResults for observability.
- [x] `apps/web` — **CRITICAL: Anonymous → password userId preservation (C1-C2).** ConvexHttpClient integration tests proved userId is NOT preserved (new user created on upgrade). Browser cookies may behave differently. Playwright E2E spec written (`conversion-flow.integration.spec.ts`). **BLOCKER: Anonymous auth (`signIn("anonymous")`) fails silently in Playwright browser context — `@convex-dev/auth` session never establishes. Execution tests fail with "Not authenticated". Pre-execution UI states captured via screenshots (6 bnto pages verified). Full pipeline verification blocked until anonymous auth is resolved in dev environment.**
- [ ] `apps/web` — **Browser auth behavior verification:** Token expiry handling, sign-out session invalidation (JWT is stateless — browser relies on cookie clearing + proxy redirect). ConvexHttpClient can't test this — Playwright E2E required. **Blocked by same anonymous auth issue as above.**
- [ ] `apps/web` — **Monetization checkpoint:** Confirm execution events log `userId`, `bntoSlug`, `timestamp`, `durationMs` to Convex. Sprint 3 builds the dashboard on this data.
- [ ] `apps/web` — Verify auth flow end-to-end on Vercel preview deployment

> **SEO checkpoint:** Before closing Sprint 2, verify in browser devtools that each `/[bnto]` URL returns correct `<title>` and `<meta description>` in the page source (not client-rendered).

> **Monetization checkpoint:** Confirm execution events are being written to Convex with `userId`, `bntoSlug`, `timestamp`, and `durationMs`. Sprint 3 builds the usage dashboard on top of this data.

---

### Sprint 2.5: Codebase Polish — Waves 3-4 (PAUSED — resume after Sprint 2A)

#### Wave 3 (parallel — button polish)

- [ ] `apps/web` — Fix Button pseudo-state bug: after active/click, button returns to hover state instead of default resting state. Investigate CSS `:active` → `:hover` transition and the depth/pressable system
- [ ] `apps/web` — Experiment with Button animations per Mini Motorways motion language (see `animation.md`): entrance spring for button appearance, smooth ease-out for press/release transitions, `motion-safe:` guards

#### Wave 4 (sequential — verify)

- [ ] `apps/web` — `task ui:build` + `task ui:lint` pass clean
- [ ] `apps/web` — E2E screenshots updated and visually verified

**Deferred from Wave 2:**
- [ ] `apps/web` — **Font family review: evaluate replacing DM Sans with Geist for display/headings.** Current: DM Sans (display) + Inter (body) + Geist Mono (code). DM Sans is bubbly/rounded-geometric — may not match Mini Motorways' precise-but-warm feel. Geist has Swiss-style technical precision. Reference: `/Users/ryan/Desktop/Mini Motorways Reference/`. Proposed: **Geist (display) + Inter (body) + Geist Mono (code)**. Prototype the swap and compare visually.

---

### Sprint 3: Dashboard + Run Quota
**Goal:** Authenticated users see their history, run count, and get a meaningful account experience. Monetization infrastructure in place before any paywall.

#### Wave 1 (parallel — quota schema + dashboard components)

- [ ] `@bnto/backend` — Add `runsUsedThisMonth`, `runResetDate`, `planTier` to user schema (if not already present)
- [ ] `@bnto/backend` — Monthly reset cron (reset `runsUsedThisMonth` on the 1st of each month)
- [ ] `@bnto/backend` — Increment run counter on each execution (check before allowing, reject if over limit)
- [ ] `@bnto/core` — `useRunsRemaining()` hook (returns `{ used, limit, resetDate }`)
- [ ] `apps/web` — WorkflowCard component (name, description, node count, last run status)
- [ ] `apps/web` — StatusBadge component (pending, running, completed, failed)
- [ ] `apps/web` — EmptyState component (no workflows yet)

#### Wave 2 (parallel — dashboard UI)

- [ ] `apps/web` — Dashboard page: list of saved workflows, recent executions, run counter widget
- [ ] `apps/web` — Run counter widget ("X of 25 runs used this month, resets [date]")
- [ ] `apps/web` — Upgrade prompt component (copy from Notion: "You've used all 25 free runs this month. Upgrade to Pro for 500 runs/month — or download the desktop app for unlimited local processing. Pro is $8/month. That's it.")
- [ ] `apps/web` — Execution history page (list of past runs with status and output links)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: run counter increments after execution
- [ ] `apps/web` — Playwright E2E: upgrade prompt appears when limit reached
- [ ] `@bnto/backend` — Unit tests for run counter logic and monthly reset cron

---

### Sprint 4: JSON Editor
**Goal:** Users who want to go deeper can write or customize `.bnto.json` files in-browser. Power users self-identify — tag them for targeted upgrade messaging.

#### Wave 1 (parallel — editor core)

- [ ] `apps/web` — Monaco/CodeMirror editor component with `.bnto.json` schema validation and syntax highlighting
- [ ] `apps/web` — Template selector (start from a predefined Bnto, customize from there)
- [ ] `apps/web` — Zustand editor state store (editor content, dirty flag, validation errors)
- [ ] `@bnto/backend` — Tag users who open the editor (`hasUsedEditor: true` — highest-intent upgrade candidates)

#### Wave 2 (parallel — integration)

- [ ] `apps/web` — New workflow page (blank editor or start from template)
- [ ] `apps/web` — Edit workflow page (load existing workflow into editor)
- [ ] `apps/web` — Save workflow flow (editor content → Convex via @bnto/core)
- [ ] `apps/web` — Run from editor (execute the JSON currently in the editor)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: open editor, select template, modify, save, run
- [ ] `apps/web` — Playwright E2E: edit existing workflow, re-run

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app using Wails v2. Same React frontend, local Go engine. Free forever, unlimited runs. No account needed. Trust signal and top-of-funnel growth driver.

### Sprint 5: Wails Bootstrap

#### Wave 1 (parallel — setup)

- [ ] `apps/desktop` — Bootstrap Wails v2 project with Vite + React
- [ ] `@bnto/core` — Implement Wails adapter (replace stubs with real Go bindings)
- [ ] `engine` — Expose engine functions for Wails bindings (RunWorkflow, ValidateWorkflow, etc.)

#### Wave 2 (parallel — integration)

- [ ] `apps/desktop` — Wire up Go ↔ React bindings (auto-generated TypeScript from Go structs)
- [ ] `@bnto/core` — Runtime detection routes to Wails adapter in Wails webview
- [ ] `apps/desktop` — Local file browser for selecting .bnto.json files

#### Wave 3 (sequential — verify)

- [ ] `apps/desktop` — Verify workflow list, edit, and save work via Wails bindings
- [ ] `apps/desktop` — Verify runtime detection correctly identifies Wails environment

---

### Sprint 6: Local Execution

#### Wave 1 (parallel — execution)

- [ ] `apps/desktop` — Execute workflows via Wails Go bindings (all 10 node types)
- [ ] `@bnto/core` — Execution progress streaming via Wails adapter
- [ ] `apps/web` — Execution progress component (reusable — node status, duration, logs)

#### Wave 2 (parallel — features)

- [ ] `apps/desktop` — Execution results view (output data, logs, duration)
- [ ] `apps/desktop` — shell-command node support (full local execution, no restrictions)
- [ ] `apps/desktop` — Error handling and cancellation support

#### Wave 3 (sequential — build + distribute)

- [ ] `apps/desktop` — Integration tests for local execution
- [ ] `apps/desktop` — macOS build (.app bundle, code signing)
- [ ] `apps/desktop` — Windows build (.exe)
- [ ] `apps/desktop` — Linux build (AppImage)

---

## Phase 3: Monetization + Polish

**Goal:** Wire up payments, enforce quotas, make the product feel complete.

**"Ready to charge" gate:** Before Sprint 7, confirm: real users running Bntos, run counter data accurate, upgrade prompt built and tested, people return voluntarily.

### Sprint 7: Stripe + Quota Enforcement

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — enforcement)

- [ ] `apps/api` — Reject execution if `runsUsedThisMonth >= limit` (server-side)
- [ ] `apps/web` — File size enforcement at R2 presigned URL generation (free: 25MB, Pro: 500MB)
- [ ] `@bnto/backend` — Pro feature gates: 30-day history retention, team sharing (up to 5 members), priority queue

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: free user hits limit, sees upgrade prompt, upgrades via Stripe
- [ ] `apps/web` — Playwright E2E: Pro user runs >25 flows without hitting limit

---

### Sprint 8: Visual Editor + History

- [ ] `apps/web` — Drag-and-drop node canvas (React Flow or custom)
- [ ] `apps/web` — Node palette with all 10 node types
- [ ] `apps/web` — Property editor per node
- [ ] `apps/web` — JSON ↔ visual round-trip (edit in either mode)
- [ ] `apps/web` — Execution history with full per-node logs and re-run support
- [ ] `apps/web` — Workflow versioning and duplication

---

## Parallel Track: ADO Dashboard

Real-world dogfooding. Runs alongside any phase.

### Phase A: `ado` Node Type
- [ ] `engine` — Implement `ado` node (WIQL queries, work items, test runs, build status)
- [ ] `engine` — Unit tests with mock HTTP server
- [ ] `engine` — Integration fixture `.bnto.json`
- [ ] `engine` — Register in DefaultRegistry()

### Phase B: `aggregate` Node Type
- [ ] `engine` — Implement `aggregate` node (groupBy, count, sum, average, percentage, sortBy)
- [ ] `engine` — Unit tests, integration test chaining with other node types

### Phase C: `report` Node Type
- [ ] `engine` — Implement `report` node (terminal, markdown, json output)
- [ ] `engine` — Unit tests for each format

### Phase D: Dashboard Templates
- [ ] `engine` — 3-5 example dashboard `.bnto.json` fixtures in `engine/examples/`

---

## Backlog

### Engine: Loop Node Output Collection

The `loop` node currently collects original items, not sub-node outputs. A workflow like "loop + edit-fields → collect transformed rows → write" doesn't work — the loop passes through the original rows, discarding the edit-fields transformation.

**Impact:** The `rename-csv-columns` fixture is a read → write pass-through. True column remapping requires this fix.

- [ ] `engine` — Loop node collects sub-node outputs instead of (or in addition to) original items
- [ ] `engine` — Alternative: new array-level transform node that operates on all rows at once

### Engine: `pdf` Node Type

Required for the PDF to Images Bnto (Tier 2, 50K+ monthly searches).

- [ ] `engine` — Implement `pdf` node type (wrap `pdfcpu` Go library, or shell-command + ghostscript as interim)
- [ ] `engine` — Unit tests for PDF → image conversion
- [ ] `engine` — Integration fixture: `pdf-to-images.bnto.json`

### Infra: Clean Up Convex Dev Environment (Better Auth Remnants)

The Convex dev deployment (`zealous-canary-422`) still contains stale data from the old Better Auth system and accumulated integration test artifacts. The `auth*` tables (`authAccounts`, `authRateLimits`, `authRefreshTokens`, `authSessions`, `authVerificationCodes`, `authVerifiers`) are now managed by `@convex-dev/auth`, but old Better Auth records and orphaned test users are still present.

**What to clean:**
- Old user records from Better Auth era (pre-migration)
- Orphaned auth sessions/tokens/verifiers that reference non-existent users
- Accumulated integration test users (`test-*@test.bnto.dev`)
- Stale execution records and execution events from test runs
- Any workflows created by test accounts

**Approach:** Write a one-off Convex mutation that identifies and deletes stale records. Run against dev first, then production if needed. Back up data before deletion.

- [ ] `@bnto/backend` — Audit Convex dev tables: identify Better Auth remnants vs current `@convex-dev/auth` records
- [ ] `@bnto/backend` — Write cleanup mutation: delete orphaned auth records, test users, and stale test data
- [ ] `@bnto/backend` — Run cleanup against dev deployment, verify table health
- [ ] `@bnto/backend` — (If needed) Run cleanup against production deployment

### Infra: Configure R2 Lifecycle Rules

R2 object cleanup layers 1-2 are implemented in code. Layer 3 (Cloudflare lifecycle rules) needs manual dashboard configuration.

| Bucket | Prefix | Auto-delete after |
|---|---|---|
| `bnto-transit` + `bnto-transit-dev` | `uploads/` | 1 hour |
| `bnto-transit` + `bnto-transit-dev` | `executions/` | 24 hours |

- [ ] `infra` — Configure R2 lifecycle rules in Cloudflare dashboard (prod + dev buckets)

### Infra: Domain Setup (bnto.io Custom Domains)

Railway first (API) to validate DNS + TLS, then Vercel (web app).

- [ ] `infra` — Add `api.bnto.io` CNAME in Cloudflare DNS → Railway
- [ ] `infra` — Configure custom domain in Railway dashboard
- [ ] `infra` — Update `GO_API_URL` in Convex prod to `https://api.bnto.io`
- [ ] `infra` — Verify API health check at `https://api.bnto.io/health`
- [ ] `infra` — Connect `bnto.io` to Vercel, update Cloudflare DNS
- [ ] `infra` — Verify auth redirects work on `bnto.io`

### Testing: Split user-journeys.spec.ts

`user-journeys.spec.ts` is currently `.disabled`. Several per-persona spec files already exist (execution-flow, file-drop, seo-metadata, bnto-config). Remaining: split the disabled file's coverage into `anonymous-visitor.spec.ts`, `authenticated-user.spec.ts`, `auth-flow.spec.ts`, `navigation.spec.ts`, `dark-mode.spec.ts`.

- [ ] `apps/web` — Split remaining coverage from disabled `user-journeys.spec.ts` into per-persona spec files
- [ ] `apps/web` — Delete the `.disabled` file after all coverage is migrated

### Testing: Standardize E2E Selectors on data-testid

Current E2E tests mix CSS classes, `getByRole`, `getByText`, and `data-testid`. Standardize on `data-testid` for state detection and element targeting. Keep semantic selectors for accessibility assertions.

- [ ] `apps/web` — Audit E2E specs, add `data-testid` attributes, update selectors

### Testing: Concurrent Quota Race Condition

Two simultaneous `startPredefined` calls from the same user could both pass the `enforceQuota` check before either increments `runsUsed` — allowing a user to exceed their limit. The current flow is: read user → check `runsUsed >= runLimit` → patch `runsUsed + 1`. If two requests interleave between the read and the patch, both pass.

- [ ] `@bnto/core` — Integration test: fire 2+ concurrent `startPredefined` calls for a user at limit-1 runs, verify at most 1 succeeds
- [ ] `@bnto/backend` — If race confirmed, investigate Convex transaction isolation guarantees or atomic increment patterns

### Testing: Monthly Run Reset Cycle

The `runsResetAt` cron resets `runsUsed` to 0 when the month rolls over. This logic exists but is untested against real Convex scheduler — unit tests seed past timestamps but don't trigger the actual cron.

- [ ] `@bnto/backend` — Unit test: seed user with `runsResetAt` in the past, call the reset mutation, verify `runsUsed` resets to 0 and `runsResetAt` advances to next month
- [ ] `@bnto/core` — Integration test (if feasible): verify reset behavior against real Convex dev

### Auth: Enable OAuth Social Providers

Google and Discord OAuth configured in `convex/auth.ts` but commented out — need OAuth credentials.

- [ ] `@bnto/backend` — Uncomment `socialProviders` in `convex/auth.ts`
- [ ] `@bnto/backend` — Set Google and Discord OAuth credentials in Convex env vars
- [ ] `apps/web` — Add Google and Discord sign-in buttons to `SignInForm`

### Growth: Referral Link Program

Referral links to boost user acquisition. A user shares a referral link → new user visits and creates an account → both the referrer and the new user receive N bonus free runs.

**Open questions:** How many bonus runs? What's the cap per user? Does it stack? How do we prevent abuse (throwaway accounts)?

- [ ] `@bnto/backend` — Schema: `referrals` table (referrerId, referredUserId, bonusRuns, createdAt), `referralCode` field on users
- [ ] `@bnto/backend` — Mutation: `applyReferral` — validates code, credits both users with bonus runs
- [ ] `@bnto/core` — Referral service/hooks: `useReferralCode()`, `useApplyReferral()`
- [ ] `apps/web` — Referral link generation UI in settings/profile
- [ ] `apps/web` — Landing page referral code capture (via URL param `?ref=CODE`)

### UX: Upgrade/Upsell Messaging Audit

Quota error messages and upgrade CTAs need to be tier-aware. We may have multiple paid tiers (Pro, Team, etc.) — the messaging in error codes, API responses, and UI prompts should not hardcode a single upgrade destination. Review and make tier-neutral or dynamically driven.

**What to audit:**
- `quota.ts` error messages (`ANONYMOUS_QUOTA_EXCEEDED` → "Sign up for a free account", `RUN_LIMIT_REACHED` → "Run limit reached") — these are what the UI renders. Wording should guide users toward the right next step without assuming which tier they'd upgrade to
- UI components that display quota errors — do they link to a specific plan page or a generic upgrade/pricing page?
- Future: error payloads could include `suggestedAction` (e.g. `"signup"`, `"upgrade"`, `"contact-sales"`) so the UI doesn't need to infer the CTA from the error code alone

- [ ] `@bnto/backend` — Review and update quota error messages/codes to be tier-neutral
- [ ] `apps/web` — Review upgrade prompts and CTAs — ensure they route to pricing page, not a hardcoded tier
- [ ] `@bnto/backend` — Consider adding `suggestedAction` and `upgradeUrl` fields to quota error payloads

### Schema-Driven Config Panel (Single Source of Truth)

**Prior art:** Atomiton project (`~/Code/atomiton`) — `createFieldsFromSchema` auto-derives UI field configs from schemas. ~70-80% of fields need zero UI code. See `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts`.

**Problem:** Frontend hardcodes per-node config shapes. Two sources of truth (Go engine + frontend).

**Solution:** Define node parameter schemas in Go, expose as structured metadata, auto-derive config panel UI. Five implementation layers:

1. **Go engine schema declarations** — `ParameterSchema` struct per node type, schema registry
2. **API exposure** — `GET /nodes/{type}/schema` and `GET /nodes/schemas` endpoints
3. **TypeScript consumption** — `createFieldsFromSchema()` utility, `useNodeSchema()` hook
4. **Dynamic config panel** — Generic `ConfigPanel` rendering from `FieldConfig[]`, per-bnto overrides (~20-30%)
5. **Pipeline integrity tests** — E2E + contract tests verifying schema-to-UI pipeline

See detailed task breakdown in `.claude/archive/schema-driven-config-panel.md` (archived from original plan).

### UX: Animated Run Counter

**User feedback:** Users need a persistent, visible indicator of how many runs they have left while working with bntos. When they execute a workflow, the counter animates down. If they upgrade, it animates up. This serves as a feedback hook — users see the consequence of each run and the benefit of upgrading in real time.

**Open questions:** Where does this live? Options: (a) in the bnto tool page header, (b) in the nav bar, (c) as a floating indicator. Needs design exploration.

- [ ] `apps/web` — Design and place the run counter component (visible on bnto tool pages)
- [ ] `apps/web` — Animated number transition: count down on execution, count up on upgrade/reset
- [ ] `apps/web` — Wire to `useRunsRemaining()` from `@bnto/core` (Sprint 3 prerequisite)
- [ ] `apps/web` — E2E test: counter decrements after execution, displays correct remaining count

### UX: Two-Column Bnto Tool Page Layout

**User feedback:** The current single-column bnto tool page layout forces users to scroll below the fold to configure settings and then run. On wider viewports, the Settings panel, drop zone, and Run button should be visible without scrolling.

**Proposed layout:** Bento box grid varios panels needed like dropzone, config, and progress. Settings panel + Run button, drop zone + file list + execution progress/results. (current behavior preserved).

- [ ] `apps/web` — Responsive two-column layout for `[bnto]/page.tsx` (Settings + Run on left, files + progress on right)
- [ ] `apps/web` — Ensure all 6 Tier 1 bnto pages work correctly in two-column mode
- [ ] `apps/web` — Update E2E screenshots for new layout (all bnto-config, execution-flow, file-drop specs)
- [ ] `apps/web` — Mobile breakpoint preserves current single-column stack

### Recursive Workflow Composability (Web App)

The Go engine supports recursive `Definition.Nodes`. The web app must preserve this composability. Guard rails (not new tasks — apply when building related features):

- Config panels must work at any nesting depth
- Execution progress must be recursive (group nodes show children's progress)
- JSON editor must represent recursive structure faithfully
- Visual editor (Sprint 8) must support drill-down into group nodes

---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/journeys/` | User journey test matrices — auth, engine, API, web app |
| `.claude/strategy/bntos.md` | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers |
| `.claude/rules/pages.md` | SEO URL implementation rules |
| `.claude/rules/architecture.md` | Run quota schema, R2 transit rules |
| `.claude/strategy/core-principles.md` | Trust commitments |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, technology decisions, execution model |
| `.claude/decisions/auth-evaluation.md` | Auth migration decision (Better Auth → @convex-dev/auth) |
| `.claude/rules/code-standards.md` | Code philosophy, Bento Box Principle |
| `.claude/skills/` | Agent skills (pickup, groom, code-review, pre-commit) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits |
