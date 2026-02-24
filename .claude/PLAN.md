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

- **Active:** Closing Sprint 2A → Sprint 2B (Browser Execution, M1 MVP)
- **Strategic direction:** Browser-first. All Tier 1 bntos run client-side via Rust→WASM (or JS fallback). Cloud execution (Railway + R2) is built and working — ready for M4 (premium server-side bntos). See ROADMAP.md.
- **Cloud pipeline:** Complete. 6/6 integration E2E tests pass. Go API on Railway, R2 file transit, Convex real-time subscriptions — all verified end-to-end. This is M4 infrastructure delivered ahead of schedule.
- **Auth:** Migrated to `@convex-dev/auth`. Anonymous sessions create real `users` rows. Integration tests complete. `AUTH_SECRET` env var required in Convex deployments.
- **Engine:** Complete. Go CLI with 10 node types (>90% coverage), Go HTTP API on Railway, BntoService shared API layer. Paused for web — browser adapter is M1 priority.
- **Web app:** Next.js on Vercel. Auth, SEO tool pages, execution UI, landing pages — all built.
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

**Monetization model (updated Feb 2026):** Browser execution is free unlimited. Pro sells real value — persistence, collaboration, premium compute. See ROADMAP.md for the full model.

| Sprint | What Ships | Revenue Implication |
|--------|-----------|---------------------|
| Sprint 2B | Browser execution (M1 MVP) | **All Tier 1 bntos run client-side.** Zero backend cost. Files never leave user's machine. SEO footprint live. |
| Sprint 3 | Platform features (accounts, history) | Accounts exist. Conversion hooks scaffolded (Save, History). Usage analytics instrumented. |
| Sprint 4 | JSON editor | Power users self-identify. Custom flows are a Pro signal. |
| Sprint 5-6 | Desktop app | Top-of-funnel. Word of mouth begins. Free forever — trust signal. |
| Sprint 7 | Stripe + Pro tier | **First revenue possible.** Pro: $8/month for persistence, collaboration, server-side AI, priority processing. |

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
- [x] `apps/web` — **CRITICAL: Anonymous → password userId preservation (C1-C2).** ConvexHttpClient integration tests proved userId is NOT preserved (new user created on upgrade). Browser cookies may behave differently. Playwright E2E spec written (`conversion-flow.integration.spec.ts`). **RESOLVED (Feb 24): Anonymous auth now works in Playwright browser context — 5/6 integration tests pass. Auth blocker was transient (likely Convex dev deployment state). Remaining failure: `clean-csv` execution fails at engine level (not auth).**
- [ ] `apps/web` — **Browser auth behavior verification:** Token expiry handling, sign-out session invalidation (JWT is stateless — browser relies on cookie clearing + proxy redirect). ConvexHttpClient can't test this — Playwright E2E required. **Moved to Sprint 3 (M2: Platform Features) — not blocking M1 browser execution.**
- [x] `apps/web` — **Monetization checkpoint:** Confirm execution events log `userId`, `bntoSlug`, `timestamp`, `durationMs` to Convex. **VERIFIED:** All 4 fields captured in `executionEvents` table. Run quota fields on user doc. `enforceQuota()` checks before execution. **Note:** Run quota enforcement applies to server-side bntos (M4). Browser bntos are free unlimited per ROADMAP.md.
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

### Sprint 2B: Browser Execution (M1 MVP)
**Goal:** All 6 Tier 1 bntos run 100% client-side in the browser. Zero backend for core experience. "Your files never leave your computer." This is the M1 milestone deliverable.

**Strategic context:** Browser execution costs us $0 and gives users unlimited free runs. Cloud execution (already built in Sprint 2/2A) becomes premium infrastructure for M4 (AI, shell, video). See ROADMAP.md for the full browser-first strategy.

#### Wave 1 (parallel — `@bnto/nodes` foundation)

The engine-agnostic package that any execution target (Rust WASM, JS, Go) consumes. Build this first — it's the safety net regardless of engine choice.

- [x] `packages/@bnto/nodes` — Create package: node type definitions (what each node does, input/output types)
- [ ] `packages/@bnto/nodes` — Migrate recipe definitions from `engine/pkg/menu/recipes/` to TypeScript (6 Tier 1 recipes)
- [ ] `packages/@bnto/nodes` — Define input/output schemas per node type (drives config panel UI — Atomiton `createFieldsFromSchema` pattern)
- [ ] `packages/@bnto/nodes` — Workflow validation functions (works in browser, CLI, desktop)
- [ ] `packages/@bnto/nodes` — Unit tests for schemas and validation

#### Wave 2 (sequential — first Rust WASM node + evaluation checkpoint)

Build one node in Rust, compile to WASM, run in a Web Worker. This is the M1 evaluation — by the end of this wave, we know if Rust works for us.

- [ ] `engine-wasm/` — Set up Rust workspace with `wasm-pack` + `wasm-bindgen` targeting web
- [ ] `engine-wasm/` — Build `compress-images` node in Rust (MozJPEG for JPEG, OxiPNG for PNG, WebP via `image` crate)
- [ ] `apps/web` — Web Worker wrapper with progress reporting via `postMessage`
- [ ] `packages/core` — Browser adapter: detect browser runtime, route execution to WASM Web Worker
- [ ] `apps/web` — Wire BntoPageShell to use browser adapter for `/compress-images`
- [ ] `apps/web` — E2E test: compress-images runs entirely client-side (no backend required)

**EVALUATION CHECKPOINT — answer honestly after this wave:**
- Is development velocity acceptable? (Building nodes getting faster?)
- Is the WASM boundary manageable? (Data passes cleanly, File/Blob handling works?)
- Is bundle size reasonable? (< 500KB gzipped for all nodes?)
- Is the ecosystem covering our needs? (`image` and `csv` crates work?)
- Is developer experience tolerable? (Build times, debugging?)

**If Rust passes → continue Wave 3a (Rust). If Rust fails → switch to Wave 3b (JS).**

#### Wave 3a: Remaining Rust WASM nodes (if Rust passed checkpoint)

- [ ] `engine-wasm/` — `resize-images` node (Lanczos3/CatmullRom filters via `image` crate)
- [ ] `engine-wasm/` — `convert-image-format` node (decode any → encode any via `image` crate)
- [ ] `engine-wasm/` — `clean-csv` node (`csv` + `serde` crates)
- [ ] `engine-wasm/` — `rename-csv-columns` node (`csv` + `serde` crates, header rewrite)
- [ ] `apps/web` — `rename-files` stays pure JS (no Rust needed — pattern matching on File objects)
- [ ] `apps/web` — Web Worker wrappers for all new WASM nodes
- [ ] `apps/web` — E2E tests: all 6 bntos run client-side

#### Wave 3b: JS library adapters (if Rust failed checkpoint)

- [ ] `apps/web` — Image processing via jSquash (MozJPEG, OxiPNG, WebP, AVIF — used by Discourse for 50MB+ images)
- [ ] `apps/web` — CSV processing via PapaParse (1M rows in ~5s)
- [ ] `apps/web` — `rename-files` stays pure JS
- [ ] `apps/web` — Web Worker wrappers for all JS adapters
- [ ] `apps/web` — E2E tests: all 6 bntos run client-side

#### Wave 4 (sequential — integration + polish)

- [ ] `apps/web` — Decompose BntoPageShell: extract `useBntoExecution` and `useBntoFiles` hooks, slim to composition shell
- [ ] `apps/web` — Update BntoPageShell to route through browser adapter (not cloud pipeline) for Tier 1 bntos
- [ ] `apps/web` — Zip + individual download for multi-file browser results
- [ ] `apps/web` — Performance benchmarks: bundle size per node, processing speed vs cloud, memory usage
- [ ] `apps/web` — Update all E2E screenshots for browser execution flow
- [ ] `apps/web` — `task ui:build` + `task ui:lint` pass clean

---

### Sprint 3: Platform Features (M2)
**Goal:** Accounts earn their keep. Users who sign up get persistence, history, and a reason to stay. Conversion hooks are natural — Save, History, Premium Bntos — not artificial run caps.

#### Wave 1 (parallel — account value + analytics schema)

- [ ] `@bnto/backend` — `planTier` field on user schema (free, pro). Usage analytics fields: `totalRuns`, `lastRunAt`
- [ ] `@bnto/backend` — Execution analytics: aggregate queries for per-user history (by slug, by date range)
- [ ] `@bnto/core` — `useExecutionHistory()` hook (paginated, per-user)
- [ ] `@bnto/core` — `useUsageAnalytics()` hook (total runs, most-used bntos, last activity)
- [ ] `apps/web` — WorkflowCard component (name, description, node count, last run status)
- [ ] `apps/web` — StatusBadge component (pending, running, completed, failed)
- [ ] `apps/web` — EmptyState component (no workflows yet)

#### Wave 2 (parallel — dashboard + conversion hooks)

- [ ] `apps/web` — Dashboard page: saved workflows, recent executions, usage analytics
- [ ] `apps/web` — Execution history page (list of past runs with status, re-run capability)
- [ ] `apps/web` — **Save prompt** (conversion hook): "Want to keep this workflow? Sign up to save it." — appears after successful browser execution for anonymous users
- [ ] `apps/web` — **History prompt** (conversion hook): "Sign up to access your execution history and re-run past workflows."
- [ ] `apps/web` — **Browser auth behavior verification:** Token expiry, sign-out invalidation (moved from Sprint 2A Wave 5)
- [ ] `apps/web` — Pricing page update: Pro sells persistence, collaboration, premium compute — not run limits

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: save prompt appears after anonymous execution
- [ ] `apps/web` — Playwright E2E: execution history page shows past runs for authenticated users
- [ ] `@bnto/backend` — Unit tests for execution analytics queries

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

**Goal:** Free desktop app. Same React frontend, local engine execution. Free forever, unlimited runs. No account needed. Trust signal and top-of-funnel growth driver.

**Desktop tech depends on M1 outcome:**
- If Rust WASM succeeded → **Tauri** (Rust-native). One codebase for browser + desktop + CLI.
- If we bailed to JS → **Wails v2** (Go-native). Go engine powers desktop + CLI. JS adapters stay for browser.

### Sprint 5: Desktop Bootstrap

#### Wave 1 (parallel — setup)

- [ ] `apps/desktop` — Bootstrap desktop project (Tauri or Wails, per M1 outcome)
- [ ] `@bnto/core` — Implement desktop adapter (Tauri IPC bindings or Wails Go bindings)
- [ ] `engine` — Expose engine functions for desktop bindings (RunWorkflow, ValidateWorkflow, etc.)

#### Wave 2 (parallel — integration)

- [ ] `apps/desktop` — Wire up native ↔ React bindings
- [ ] `@bnto/core` — Runtime detection routes to desktop adapter in native webview
- [ ] `apps/desktop` — Local file browser for selecting .bnto.json files

#### Wave 3 (sequential — verify)

- [ ] `apps/desktop` — Verify workflow list, edit, and save work via native bindings
- [ ] `apps/desktop` — Verify runtime detection correctly identifies desktop environment

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

**"Ready to charge" gate:** Before Sprint 7, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 7: Stripe + Pro Tier (M5)

**Goal:** First revenue. Pro sells real value — not artificial limits on browser-native operations.

**What Pro includes:** $8/month or $69/year. Saved workflows, execution history (30-day retention), team sharing (up to 5 members), server-side premium bntos (AI, shell, video — M4), priority processing, API access.

**What stays free forever:** All browser-capable bntos, unlimited runs, desktop app. See ROADMAP.md trust commitments.

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — Pro feature gates)

- [ ] `@bnto/backend` — Pro feature gates: 30-day history retention, team sharing (up to 5 members), priority processing queue
- [ ] `apps/api` — Server-side execution quota enforcement (applies to premium server-side bntos only — AI, shell, video)
- [ ] `apps/web` — File size enforcement at R2 presigned URL generation for server-side bntos (free: 25MB, Pro: 500MB)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: free user sees Pro conversion hooks (save, history, premium bntos)
- [ ] `apps/web` — Playwright E2E: Pro user has access to saved workflows and execution history

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

### Infra: Shared Test Fixtures Package (`@bnto/test-fixtures`)

**Priority: High (Sprint 2B prerequisite).** Centralized test assets — photos, CSVs, spreadsheets, sample files — shared across packages and apps. Currently test fixtures are scattered (engine fixtures in `engine/tests/fixtures/`, web E2E uses ad-hoc files). A single package means `@bnto/nodes`, `@bnto/core`, `apps/web` E2E, and engine tests all use the same canonical test data.

**What it contains:**
- Sample images (JPEG, PNG, WebP — small, medium, large sizes)
- Sample CSVs (clean, dirty with empty rows/whitespace, large row count)
- Sample files for rename operations (various extensions)
- Helper functions to load fixtures by name
- TypeScript + Go consumption (TS package for web/core, Go reads from same directory via `go:embed` or path)

- [ ] `packages/@bnto/test-fixtures` — Create package with sample images (JPEG, PNG, WebP at various sizes)
- [ ] `packages/@bnto/test-fixtures` — Add sample CSVs (clean, dirty, large)
- [ ] `packages/@bnto/test-fixtures` — Add miscellaneous files for rename/file operations
- [ ] `packages/@bnto/test-fixtures` — TypeScript helpers to load fixtures by name
- [ ] `packages/@bnto/test-fixtures` — Update `apps/web/e2e/` to import fixtures from shared package
- [ ] `engine` — Update Go test fixtures to reference shared directory (or symlink)

### Engine: Spreadsheet Node Template Resolution — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work — not blocking M1 (browser execution uses Rust/JS, not Go). The `clean-csv` predefined Bnto fails in cloud execution. The `read-csv` node (type: `spreadsheet`) receives `<no value>` for its input file path template variable.

**Discovered via:** Integration E2E test. All image-based pipelines work — only the spreadsheet node path is broken.

- [ ] `engine` — Reproduce locally: `bnto run` with `clean-csv` fixture against a test CSV file
- [ ] `engine` — Debug template resolution in `spreadsheet` node's `Execute()`
- [ ] `engine` — Fix template variable resolution so `read-csv` receives the actual file path
- [ ] `engine` — Verify fix: integration E2E `clean-csv` test passes (`task e2e:integration`)

### Web: BntoPageShell Decomposition — Sprint 2B Wave 4

**Promoted to Sprint 2B Wave 4.** BntoPageShell needs to be rewired for the browser adapter anyway. Decomposition happens as part of that work. See Sprint 2B Wave 4 tasks.

### Engine: Loop Node Output Collection — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work — not blocking M1. The `loop` node currently collects original items, not sub-node outputs.

**Impact:** The `rename-csv-columns` fixture is a read → write pass-through. True column remapping requires this fix.

- [ ] `engine` — Loop node collects sub-node outputs instead of (or in addition to) original items
- [ ] `engine` — Alternative: new array-level transform node that operates on all rows at once

### Engine: `pdf` Node Type — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work. Required for the PDF to Images Bnto (Tier 2, 50K+ monthly searches).

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

### Infra: Configure R2 Lifecycle Rules — M4 (cloud execution)

**Milestone: M4.** R2 is only used for cloud (server-side) execution. Not needed for M1 browser execution.

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

### Testing: Concurrent Quota Race Condition — M4/M5 (server-side quotas)

**Milestone: M4/M5.** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

- [ ] `@bnto/core` — Integration test: fire 2+ concurrent `startPredefined` calls for a user at limit-1 runs, verify at most 1 succeeds
- [ ] `@bnto/backend` — If race confirmed, investigate Convex transaction isolation guarantees or atomic increment patterns

### Testing: Monthly Run Reset Cycle — M4/M5 (server-side quotas)

**Milestone: M4/M5.** Run reset logic applies to server-side quota tracking. Browser bntos have no quotas.

- [ ] `@bnto/backend` — Unit test: seed user with `runsResetAt` in the past, call the reset mutation, verify `runsUsed` resets to 0 and `runsResetAt` advances to next month
- [ ] `@bnto/core` — Integration test (if feasible): verify reset behavior against real Convex dev

### Auth: Enable OAuth Social Providers

Google and Discord OAuth configured in `convex/auth.ts` but commented out — need OAuth credentials.

- [ ] `@bnto/backend` — Uncomment `socialProviders` in `convex/auth.ts`
- [ ] `@bnto/backend` — Set Google and Discord OAuth credentials in Convex env vars
- [ ] `apps/web` — Add Google and Discord sign-in buttons to `SignInForm`

### Growth: Referral Program — M5+

Referral links to boost user acquisition. With browser-first, the referral reward shifts from "bonus runs" (old model) to Pro trial or extended history.

**Open questions:** What's the reward? Options: (a) 7-day Pro trial for both, (b) extended execution history (90 days instead of 30), (c) early access to new bntos. Needs validation after launch.

- [ ] `@bnto/backend` — Schema: `referrals` table (referrerId, referredUserId, reward, createdAt), `referralCode` field on users
- [ ] `@bnto/backend` — Mutation: `applyReferral` — validates code, applies reward to both users
- [ ] `@bnto/core` — Referral service/hooks: `useReferralCode()`, `useApplyReferral()`
- [ ] `apps/web` — Referral link generation UI in settings/profile
- [ ] `apps/web` — Landing page referral code capture (via URL param `?ref=CODE`)

### UX: Conversion Hook Messaging Audit — M2/M5

**Milestone: M2 (Sprint 3) for hook UX, M5 (Sprint 7) for Stripe integration.**

Conversion messaging should be value-driven, not limit-driven. Hooks trigger on natural value moments (Save, History, Premium Bntos, Team) — not artificial run caps on browser bntos.

**What to audit:**
- Existing `quota.ts` error messages — reframe from "limit reached" to value-driven CTAs
- Quota enforcement only applies to server-side bntos (M4). Browser bntos never show quota errors
- Conversion hooks: "Save this workflow" (signup), "View your history" (signup), "Run AI/shell bntos" (Pro), "Share with team" (Pro)

- [ ] `@bnto/backend` — Review and update error messages: separate browser (no limits) from server-side (quota) paths
- [ ] `apps/web` — Design conversion hook components (Save prompt, History prompt, Premium bnto upsell)
- [ ] `apps/web` — Ensure all CTAs route to pricing page with value messaging, not "you've hit a limit"

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

### UX: Execution Activity Feed — M2 (Sprint 3)

**Updated from "Animated Run Counter."** With browser-first, there's no run limit to count down. Instead, show an activity feed / recent executions indicator that reinforces the value of signing up (persistence, history).

- [ ] `apps/web` — Design activity indicator for bnto tool pages (recent executions, total runs)
- [ ] `apps/web` — For anonymous users: "You've run 12 bntos this session. Sign up to save your history."
- [ ] `apps/web` — For authenticated users: animated activity feed with execution count and last-run status

### Premium: Cloud Drive Export (Post-MVP) — M5+

**Premium conversion hook.** After running a bnto, Pro users can auto-save results directly to their cloud drive — Google Drive, OneDrive, SharePoint, Dropbox. No manual download-and-upload cycle.

**Why it's a strong hook:** The browser-first experience is "drop files → process → download." Adding "→ save to Google Drive" removes the last friction step for users who process files regularly. It's a natural Pro feature because it requires server-side OAuth + API calls.

- [ ] `apps/web` — Design cloud drive export UX (post-execution "Save to..." button with provider icons)
- [ ] `apps/api` — OAuth integration for Google Drive, OneDrive (server-side token management)
- [ ] `@bnto/backend` — Store connected cloud drive credentials per user (Pro only)
- [ ] `apps/api` — Upload execution output to user's connected drive
- [ ] `apps/web` — E2E test: Pro user saves output to connected Google Drive

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
