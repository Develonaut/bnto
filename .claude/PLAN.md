# Bnto — Build Plan

**Last Updated:** February 22, 2026
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

**Co-location decision (Feb 2026):** UI components and editor features live in `apps/web` for now. No separate `@bnto/ui` or `@bnto/editor` packages until there's a real second consumer (desktop app). Author with the intent to extract later. Engine, core API, and data layer logic stays in `@bnto/core` with the client/service abstraction pattern.

---

## Current State

**Status:** Sprint 1 complete (Waves 1-3). Sprint 2 partially complete — all 6 Tier 1 fixtures exist, SEO routing is live, landing pages rebuilt with shadcn Mainline template. Cloud execution infrastructure (R2, Railway deployment, execution UI) not started.

**Engine (complete):** Go CLI with 10 node types (all >90% test coverage), integration test fixtures, CLI smoke tests, Go HTTP API server with 20+ integration tests, BntoService shared API layer.

**Web app (in progress):** Next.js on Vercel. Better Auth integrated. Proxy middleware for route protection. Landing pages (hero, features, pricing, about, FAQ, contact, privacy). SEO tool pages (`/[bnto]/page.tsx`) with static generation, per-slug metadata, JSON-LD, sitemap, llms.txt. Auth flow (sign-in, sign-up, sign-out with signal cookie). No execution UI yet.

**Packages:** `@bnto/core` (layered singleton — clients, services, adapters, hooks), `@bnto/auth` (Better Auth integration), `@bnto/backend` (Convex schema + functions). UI components co-located in `apps/web/components/`.

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine: 10 node types, orchestration, validation, storage, secrets, path resolution
- [x] Go API server: HTTP handlers wrapping BntoService (apps/api/)
- [x] Contract tests: Go JSON responses match @bnto/core TypeScript types
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38 hooks
- [x] @bnto/auth: Better Auth + @better-auth/convex (server + client + middleware)
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, run counter fields
- [x] Web app: Landing pages (Mainline template — hero, features, pricing, about, FAQ, contact, privacy, footer)
- [x] Web app: Auth flow (sign-in, sign-up, sign-out with signal cookie, proxy route protection)
- [x] Web app: SEO infrastructure (bnto-registry.ts, [bnto]/page.tsx, generateStaticParams, generateMetadata, BntoJsonLd, sitemap.ts, llms.txt, robots.txt)
- [x] Web app: Middleware (canonical URL normalization, auth routing, bnto slug pass-through)
- [x] All 6 Tier 1 fixtures in engine/tests/fixtures/workflows/
- [x] Playwright E2E: user journeys + visual parity tests with screenshots

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in Notion.

> **Notion:** Search the bnto workspace for "SEO & Monetization Strategy" using the Notion MCP.

**Quick reference (from Notion):**

| Sprint | What Ships | Revenue Implication |
|--------|-----------|---------------------|
| Sprint 1 | Auth + Vercel deployment | Foundation only |
| Sprint 2 | Predefined Bntos + cloud execution | **First real users.** SEO footprint live. Audience building starts. |
| Sprint 3 | Dashboard + run quota tracking | Accounts exist. Run counter visible. Upgrade prompt scaffolded. |
| Sprint 4 | JSON editor | Power users self-identify. Custom flows are a Pro signal. |
| Sprint 5-6 | Desktop app | Top-of-funnel. Word of mouth begins. |
| Sprint 7 | Stripe + quota enforcement | **First revenue possible.** Free capped at 25/month. Pro at $8/month. |

---

## Revised Phase Order

The original plan deferred execution to Phase 2 (desktop) and Phase 3 (cloud). That's been reordered. **Cloud execution moves up.** A casual user who can't run anything isn't a user at all.

```
Old order: Web UI → Desktop execution → Cloud execution
New order: Web UI + Cloud execution (predefined Bntos) → JSON editor → Desktop app
```

---

## Phase 0: Foundation — COMPLETE

**What shipped:** Monorepo restructuring, engine solidification with TDD (>90% coverage on all 10 node types), integration test fixtures, CLI smoke tests, Go API server, Convex setup, web app shell, @bnto/core hooks.

---

## Phase 1: Web App + Cloud Execution

**Goal:** Ship a web app on Vercel where anyone can open a browser, pick a predefined Bnto, drop files, and run it. Execution backed by the Go HTTP API on Railway. Free. No account needed for core experience. SEO URL routing live from day one.

---

### Sprint 1: Infrastructure Migration — COMPLETE
**Goal:** Move from Railway/Convex Auth to Vercel/Better Auth. Clean auth foundation before building features on top.

#### Wave 1 (parallel — setup) — COMPLETE

- [x] `@bnto/auth` — Replace Convex Auth with Better Auth + @better-auth/convex adapter
- [x] `@bnto/backend` — Update Convex schema and functions for Better Auth
- [x] `apps/web` — Set up Vercel deployment (vercel.json, env vars, preview deployments)

#### Wave 2 (parallel — integration) — COMPLETE

- [x] `@bnto/core` — BntoProvider + ConvexClientProvider + SessionProvider (hydration-safe)
- [x] `apps/web` — Proxy middleware (cookie-presence route protection)
- [x] `apps/web` — AppGate component (splash until auth resolves)
- [x] `apps/web` — Sign-in / sign-up pages using Better Auth client
- [x] `apps/web` — Route definitions (lib/routes.ts — public/private/auth paths)

#### Wave 3 (parallel — cleanup) — COMPLETE

- [x] `@bnto/core` — Sign-out flow (signal cookie + background cleanup pattern)
- [x] `apps/web` — Remove passphrase gate and whitelist logic (auth is the gate now)
- [x] `apps/web` — Remove old Convex Auth integration

#### Wave 4 (sequential — verify)

- [ ] `apps/web` — Verify auth flow end-to-end on Vercel preview deployment
- [ ] `apps/web` — Playwright E2E: sign-in, sign-out, route protection (requires staging Convex backend)

---

### Sprint 2: Predefined Bntos + Cloud Execution
**Goal:** Users land on a Bnto URL, drop files, run, and get output. This is the MVP moment — the first time a real user gets value from bnto. SEO footprint goes live.

**Reference:** `.claude/strategy/bntos.md` — Tier 1 list, slugs, target queries, fixture status.

#### Wave 1 (parallel — fixtures) — COMPLETE

All 6 Tier 1 fixtures exist in `engine/tests/fixtures/workflows/` and are wired into the integration test suite.

- [x] `engine` — Fixture: `compress-images.bnto.json`
- [x] `engine` — Fixture: `resize-images.bnto.json`
- [x] `engine` — Fixture: `convert-image-format.bnto.json` (png → webp via image node)
- [x] `engine` — Fixture: `rename-files.bnto.json` (filesystem + edit-fields nodes)
- [x] `engine` — Fixture: `clean-csv.bnto.json` (spreadsheet node — strip empty rows/cols, normalize headers)
- [x] `engine` — Fixture: `rename-csv-columns.bnto.json` (spreadsheet + edit-fields nodes)

#### Wave 2 (parallel — SEO routing + tool page UI)

SEO infrastructure is done. Tool page UI (the actual interactive experience) is next.

- [x] `apps/web` — SEO URL routing: `app/[bnto]/page.tsx` with generateStaticParams + generateMetadata + notFound()
- [x] `apps/web` — Bnto registry (`lib/bnto-registry.ts`) with all 6 Tier 1 entries, isValidBntoSlug, getBntoBySlug
- [x] `apps/web` — Per-slug metadata (title, description, og tags), JSON-LD structured data, sitemap, llms.txt
- [x] `apps/web` — Bnto gallery on home page (warm, card-based grid of all Tier 1 tools — browsable, no account required)
- [ ] `apps/web` — Per-Bnto configuration UI (context-specific controls: quality slider for images, format selector, column mapping for CSV)
- [ ] `apps/web` — File drop interface (drag & drop zone, batch file selection, shows selected files with size/type)
- [ ] `@bnto/backend` — Execution event logging (every run logged — userId or browser fingerprint, bnto slug, timestamp, durationMs)

#### Wave 3 (parallel — R2 file transit layer)

**Architecture:** Browser → R2 → Railway → R2 → Browser. Files are never stored permanently. Upload → process → download → delete (1-hour TTL).

- [ ] `apps/web` — Cloudflare R2 bucket setup (temp storage, TTL-keyed paths: `/executions/{id}/input/`, `/executions/{id}/output/`)
- [ ] `@bnto/backend` — Convex action to generate R2 presigned upload URLs (validate file type + enforce 25MB free / 500MB Pro size limits)
- [ ] `apps/web` — Browser → R2 direct upload (presigned URLs, progress indicator)
- [ ] `apps/api` — Deploy Go API server to Railway (private networking to Convex)
- [ ] `apps/api` — Railway endpoint: pull input files from R2, execute `.bnto.json`, push output files to R2
- [ ] `@bnto/backend` — Convex actions to trigger Railway execution and update status via mutations (pending → running → complete/failed)
- [ ] `@bnto/core` — Execution hooks wired to Convex adapter (start execution, subscribe to progress, get results)

#### Wave 4 (parallel — execution UI)

- [ ] `apps/web` — Execution progress component (real-time via Convex subscription — node-by-node progress, not just a spinner)
- [ ] `apps/web` — Results/download component (signed R2 URL → zip download → R2 cleanup)
- [ ] `apps/web` — RunButton component (run with loading state, disabled when no files selected)

#### Wave 5 (sequential — test + verify)

- [ ] `apps/web` — Playwright E2E: land on `/compress-images`, upload file, run, download result
- [ ] `apps/web` — Playwright E2E: verify SEO metadata renders correctly on each Tier 1 slug
- [ ] `engine` — Verify all 6 fixtures run clean via `bnto run` integration tests

> **SEO checkpoint:** Before this sprint closes, verify in browser devtools that each `/[bnto]` URL returns correct `<title>` and `<meta description>` in the page source (not client-rendered). If they're missing from the HTML source, the metadata is being rendered client-side and won't be indexed.

> **Monetization checkpoint:** Confirm execution events are being written to Convex with `userId` (or fingerprint), `bntoSlug`, `timestamp`, and `durationMs`. Sprint 3 builds the usage dashboard on top of this data — it needs to exist first.

---

### Sprint 3: Dashboard + Run Quota
**Goal:** Authenticated users see their history, run count, and get a meaningful account experience. The monetization infrastructure is in place before any paywall is needed.

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
**Goal:** Users who want to go deeper can write or customize `.bnto.json` files in-browser. Power users self-identify here — tag them for targeted upgrade messaging.

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

**Goal:** Free desktop app using Wails v2. Same React frontend, local Go engine. Free forever, unlimited runs. No account needed. This is the trust signal and top-of-funnel growth driver.

---

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

**Goal:** Wire up payments, enforce quotas, and make the product feel complete. By this point we have real users, real signal, and a working product worth paying for.

**"Ready to charge" gate:** Before Sprint 7, confirm: real users are running Bntos, run counter data is accurate in Convex, upgrade prompt is built and tested, product people return to voluntarily.

---

### Sprint 7: Stripe + Quota Enforcement

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — enforcement)

- [ ] `apps/api` — Reject execution if `runsUsedThisMonth >= limit` (server-side, not client-side)
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

Real-world dogfooding. Runs alongside any phase. Adds general-purpose node types that benefit all users.

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

The `loop` node currently collects original items, not sub-node outputs. This means a workflow like "loop + edit-fields → collect transformed rows → write" doesn't work — the loop passes through the original rows, discarding the edit-fields transformation.

**Impact:** The `rename-csv-columns` fixture is a read → write pass-through. True column remapping requires this fix.

**Options:**
- [ ] `engine` — Loop node collects sub-node outputs instead of (or in addition to) original items
- [ ] `engine` — New array-level transform node that operates on all rows at once (alternative to loop-based approach)

Pick one approach when this is prioritized. The loop output collection route is more general-purpose; the array transform node is simpler for bulk column operations.

### Engine: `pdf` Node Type

Required for the PDF to Images Bnto (Tier 2, 50K+ monthly searches). High-priority engine work.

- [ ] `engine` — Implement `pdf` node type (wrap `pdfcpu` Go library, or shell-command + ghostscript as interim)
- [ ] `engine` — Unit tests for PDF → image conversion
- [ ] `engine` — Integration fixture: `pdf-to-images.bnto.json`

### Engine: Browser Fingerprint for Anonymous Run Tracking

Anonymous users get 25 runs/month tracked by browser fingerprint. No fingerprint implementation exists yet.

- [ ] `apps/web` — Implement browser fingerprint generation (FingerprintJS or similar)
- [ ] `@bnto/backend` — Store fingerprint on execution records for anonymous users
- [ ] `@bnto/backend` — Query runs-by-fingerprint for anonymous quota enforcement

### Tooling: Use package.json `imports` Instead of TSConfig Path Aliases

Replace `@/components`, `@/lib`, etc. TSConfig path aliases with Node.js-native `package.json` `imports` field (`#components/*`, `#lib/*`). Standards-track approach.

- [ ] `apps/web` — Add `imports` field to `package.json` with `#*` mappings
- [ ] `apps/web` — Update all `@/` imports to `#` imports
- [ ] `apps/web` — Remove `paths` from `tsconfig.json`

### Auth: Enable OAuth Social Providers

Google and Discord OAuth social providers are configured in `@bnto/backend` (`convex/auth.ts`) but commented out — they require OAuth credentials.

- [ ] `@bnto/backend` — Uncomment `socialProviders` block in `convex/auth.ts`
- [ ] `@bnto/backend` — Set Google and Discord OAuth credentials in Convex env vars (dev + prod)
- [ ] `apps/web` — Add Google and Discord sign-in buttons to `SignInForm`

### Docs: Update Strategy Docs for Cloud-First Phase Order

Strategy docs still reference the old phase order (Desktop = Phase 2, Cloud = Phase 3). Reality: cloud execution is Phase 1, desktop is Phase 2.

- [x] `.claude/strategy/cloud-desktop-strategy.md` — Updated MVP Phases (Section 7), package structure, testing refs, risk table
- [x] `.claude/strategy/monorepo-structure.md` — Updated phase references, removed `@bnto/ui`/`@bnto/editor`, added co-location note
- [x] `CLAUDE.md` — Updated phase order, package structure, repo tree, tech stack, architecture compliance checklist
- [x] `.claude/rules/architecture.md` — Updated layer diagram, removed `@bnto/ui`/`@bnto/editor` package sections, fixed Sprint ref

---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/strategy/bntos.md` | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers |
| `.claude/rules/pages.md` | SEO URL implementation rules |
| `.claude/rules/architecture.md` | Run quota schema, R2 transit rules |
| `.claude/strategy/core-principles.md` | Trust commitments |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, technology decisions, execution model |
| `.claude/strategy/monorepo-structure.md` | Repo structure, API abstractions, package design |
| `.claude/decisions/monorepo-tooling.md` | Taskfile + Turborepo decision |
| `.claude/rules/code-standards.md` | Code philosophy, Bento Box Principle |
| `.claude/rules/` | All coding standards and conventions |
| `.claude/skills/` | Agent skills (pickup, groom, code-review, pre-commit) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits, conversion triggers |
| Notion: "MVP Scope & Feature Roadmap" | Phase order rationale, R2 architecture, per-Bnto UX spec |
| Notion: "Bnto Directory & Launch Plan" | Strategic layer — why each Bnto was prioritized, search volume data |
