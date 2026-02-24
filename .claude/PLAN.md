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

**Status:** Sprint 1 complete (Waves 1-3). Sprint 2 Waves 1-4 complete. All 6 Tier 1 fixtures exist, SEO routing is live, landing pages rebuilt with shadcn Mainline template. Environment infrastructure complete: R2 buckets + credentials configured (dev + prod), Convex env vars set for both deployments, Vercel env vars split per environment. Go API server deployed to Railway (`https://bnto-production.up.railway.app`) with R2 file transit enabled. Execution UI complete: RunButton, ExecutionProgress (real-time), ExecutionResults (download), wired into BntoPageShell with full predefined execution path (slug + definition → Convex → Railway → R2). E2E flow tests with 8 screenshots cover the execution lifecycle. Wave 5 (E2E integration tests) is next.

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
- [x] Execution UI: RunButton, ExecutionProgress (real-time Convex subscription), ExecutionResults (R2 download)
- [x] Predefined execution path: BntoPageShell → useRunPredefined → Convex startPredefined → Railway Go API
- [x] Download infrastructure: @bnto/core download client/service/adapter/hook chain
- [x] Playwright E2E: execution flow tests (9 tests, 8 screenshots — compress, resize, csv, rename)

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
- [x] `apps/web` — Per-Bnto configuration UI (context-specific controls: quality slider for images, format selector, column mapping for CSV)
- [x] `apps/web` — File drop interface (drag & drop zone, batch file selection, shows selected files with size/type)
- [x] `@bnto/backend` — Execution event logging (every run logged — userId or browser fingerprint, bnto slug, timestamp, durationMs)

#### Wave 3 (parallel — R2 file transit + Railway deployment + env config)

**Architecture:** Browser → R2 → Railway → R2 → Browser. Files are never stored permanently. Upload → process → download → delete (1-hour TTL).

**Environment setup (prerequisite for everything in this wave):**

- [x] `infra` — Create R2 API token in Cloudflare (Object Read & Write, scoped to bnto buckets)
- [x] `infra` — Create separate R2 buckets per environment (`bnto-transit-dev`, `bnto-transit`)
- [x] `infra` — Set R2 env vars in Convex dev deployment (`R2_ACCOUNT_ID`, `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `R2_BUCKET_NAME=bnto-transit-dev`)
- [x] `infra` — Set R2 env vars in Convex prod deployment (same keys, prod bucket name)
- [x] `infra` — Set prod Convex env vars (`BETTER_AUTH_SECRET` — generate new, `SITE_URL=https://bnto.io`)
- [x] `infra` — Link Railway project to repo (`railway link`), create API service, link service
- [x] `infra` — Set `GO_API_URL` in Convex dev + prod deployments (Railway service URL)
- [x] `infra` — Set Vercel env vars for production (`NEXT_PUBLIC_CONVEX_URL`, `NEXT_PUBLIC_CONVEX_SITE_URL` pointing to prod Convex)
- [x] `infra` — Verify env vars doc (`.claude/environment-variables.md`) matches reality after setup

**R2 file transit:**

- [x] `apps/web` — Cloudflare R2 bucket setup (temp storage, TTL-keyed paths: `/executions/{id}/input/`, `/executions/{id}/output/`)
- [x] `@bnto/backend` — Convex action to generate R2 presigned upload URLs (validate file type + enforce 25MB free / 500MB Pro size limits)
- [x] `apps/web` — Browser → R2 direct upload (presigned URLs, progress indicator)

**Railway deployment:**

- [x] `apps/api` — Deploy Go API server to Railway (private networking to Convex)
- [x] `apps/api` — Railway endpoint: pull input files from R2, execute `.bnto.json`, push output files to R2

**Wiring:**

- [x] `@bnto/backend` — Convex actions to trigger Railway execution and update status via mutations (pending → running → complete/failed)
- [x] `@bnto/core` — Execution hooks wired to Convex adapter (start execution, subscribe to progress, get results)

#### Wave 4 (parallel — execution UI) — COMPLETE

- [x] `apps/web` — Execution progress component (real-time via Convex subscription — node-by-node progress, not just a spinner)
- [x] `apps/web` — Results/download component (signed R2 URL → zip download → R2 cleanup)
- [x] `apps/web` — RunButton component (run with loading state, disabled when no files selected)

#### Wave 5 (sequential — test + verify)

- [ ] **CLAIMED** `apps/web` — Playwright E2E: land on `/compress-images`, upload file, run, download result
- [x] `apps/web` — Playwright E2E: verify SEO metadata renders correctly on each Tier 1 slug
- [x] `engine` — Verify all 6 fixtures run clean via `bnto run` integration tests

> **SEO checkpoint:** Before this sprint closes, verify in browser devtools that each `/[bnto]` URL returns correct `<title>` and `<meta description>` in the page source (not client-rendered). If they're missing from the HTML source, the metadata is being rendered client-side and won't be indexed.

> **Monetization checkpoint:** Confirm execution events are being written to Convex with `userId` (or fingerprint), `bntoSlug`, `timestamp`, and `durationMs`. Sprint 3 builds the usage dashboard on top of this data — it needs to exist first.

---

### Sprint 2.5: Codebase Polish & Consistency
**Goal:** Clean up naming, imports, component consistency, and animation language before the codebase grows further. Knock out tech debt while the surface area is small.

#### Wave 1 (parallel — naming & imports)

- [ ] `apps/web` — Audit and convert monorepo to Node.js `package.json` imports (`#components/*`, `#lib/*`) replacing TSConfig `@/` path aliases. Scope: `apps/web` first, then shared packages if applicable
- [x] `apps/web` — Rename JS/TS files to camelCase where they aren't already (hooks, utils, lib files)
- [ ] `apps/web` — Rename component files and component names to PascalCase where they aren't already

#### Wave 2 (parallel — component wrappers & audit)

- [ ] `apps/web` — Create dot-notation wrappers for remaining primitives (Accordion, Form, Select, Collapsible, Carousel) and migrate all consumers
- [ ] `apps/web` — Button audit: find every `<button>` and third-party button in the web app that isn't using our `Button` component, migrate to `Button`
- [ ] `apps/web` — **Font family review: evaluate replacing DM Sans with Geist for display/headings.** Current stack: DM Sans (display) + Inter (body) + Geist Mono (code). DM Sans is bubbly/rounded-geometric — doesn't match Mini Motorways' precise-but-warm cartographic feel. Geist (Vercel's sans) has the Swiss-style technical precision that better matches the game's clean geometric typography. Reference: `/Users/ryan/Desktop/Mini Motorways Reference/` — title screen font is clean geometric with slightly rounded terminals, map labels are lightweight spaced uppercase. Proposed stack: **Geist (display/headings) + Inter (body) + Geist Mono (code)**. Do web research on Geist vs DM Sans characteristics, then prototype the swap and compare visually. Also verify `font-display` vs `font-sans` usage is consistent across all components per theming rules

#### Wave 3 (parallel — button polish)

- [ ] `apps/web` — Fix Button pseudo-state bug: after active/click, button returns to hover state instead of default resting state. Investigate CSS `:active` → `:hover` transition and the depth/pressable system
- [ ] `apps/web` — Experiment with Button animations per Mini Motorways motion language (see `animation.md`): entrance spring for button appearance, smooth ease-out for press/release transitions, ensure `motion-safe:` guards are in place

#### Wave 4 (sequential — verify)

- [ ] `apps/web` — `task ui:build` + `task ui:lint` pass clean
- [ ] `apps/web` — E2E screenshots updated and visually verified

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

### Infra: Configure R2 Lifecycle Rules

R2 object cleanup has three layers (Go API immediate, Convex scheduled, R2 lifecycle). Layers 1-2 are implemented in code. Layer 3 needs manual configuration in the Cloudflare dashboard.

**Action:** In Cloudflare dashboard > R2 > each bucket > Settings > Object lifecycle rules, add:

| Bucket | Prefix | Auto-delete after |
|---|---|---|
| `bnto-transit` (prod) | `uploads/` | 1 hour |
| `bnto-transit` (prod) | `executions/` | 24 hours |
| `bnto-transit-dev` (dev) | `uploads/` | 1 hour |
| `bnto-transit-dev` (dev) | `executions/` | 24 hours |

This is the final safety net — catches any objects that layers 1-2 missed. See [architecture.md](rules/architecture.md#r2-object-cleanup-defense-in-depth) for the full cleanup strategy.

- [ ] `infra` — Configure R2 lifecycle rules in Cloudflare dashboard (prod + dev buckets)

### Engine: Browser Fingerprint for Anonymous Run Tracking

Anonymous users get 25 runs/month tracked by browser fingerprint. No fingerprint implementation exists yet.

- [ ] `apps/web` — Implement browser fingerprint generation (FingerprintJS or similar)
- [ ] `@bnto/backend` — Store fingerprint on execution records for anonymous users
- [ ] `@bnto/backend` — Query runs-by-fingerprint for anonymous quota enforcement

### Schema-Driven Config Panel (Single Source of Truth)

**Prior art:** The `atomiton` project (`~/Code/atomiton`) implemented this pattern fully. Key files to study:
- `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts` — the utility that auto-derives UI field configs from Zod schemas
- `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.test.ts` — thorough test suite showing all derivation cases
- `packages/@atomiton/nodes/src/core/types/parameters.ts` — `NodeFieldConfig` and `NodeFieldControlType` type definitions
- `packages/@atomiton/nodes/src/schemas/image/index.ts` — example Zod schema with `.describe()`, `.min()`, `.max()`, `.default()`, `.enum()`, `.optional()`
- `packages/@atomiton/nodes/src/definitions/image/fields.ts` — example of `createFieldsFromSchema(imageSchema, overrides)` with ~30% selective overrides

**Problem:** The frontend currently hardcodes per-node configuration shapes (quality sliders, format selectors, column mappings) in `app/[bnto]/_components/configs/`. This creates two sources of truth — the Go engine knows what each node expects, and the frontend independently guesses. Every new node type requires new hardcoded UI code. Constraints can drift (engine says max quality is 100, frontend slider goes to 95).

**Solution:** Define node parameter schemas once (in Go), expose them as structured metadata, and auto-derive the config panel UI from the schema. The frontend renders controls dynamically based on schema introspection. Only UI-specific concerns that can't be inferred from the schema (e.g., "this string field should render as a code editor") require explicit overrides.

**The pattern (proven in atomiton):**

```
Schema (single source of truth)
  │
  ├── Used in Go engine for validation at execution time
  │
  └── Exposed as structured metadata to frontend
        │
        └── createFieldsFromSchema(schema, overrides)
              │
              ├── Auto-derived (~70-80% of fields need zero configuration):
              │   - controlType: string→text, number→number, enum→select, boolean→checkbox, url→url, email→email
              │   - label: camelCase→"Title Case" (e.g., maxRetries→"Max Retries")
              │   - required: derived from optional/required in schema
              │   - min/max: derived from .min()/.max() constraints
              │   - options: derived from enum values
              │   - helpText: derived from .describe() on the schema field
              │   - placeholder: derived from .default() value (shows "Default: X")
              │
              └── Selective overrides (~20-30% of fields):
                  - controlType override (string→"code", string→"textarea")
                  - custom option labels (enum value "GET" → label "GET - Retrieve data")
                  - rows for textarea
                  - step for number inputs
                  - placeholder text
```

**Control type taxonomy** (from atomiton's `NodeFieldControlType`, adapt for bnto):

| Schema type | Auto-derived control | When to override |
|---|---|---|
| `string` | `text` | Override to `textarea`, `code`, `markdown`, `password` |
| `string` with url validation | `url` | Rarely |
| `string` with email validation | `email` | Rarely |
| `number` | `number` | Override to `range` (slider) for bounded values |
| `boolean` | `checkbox` / `switch` | Rarely |
| `enum` | `select` (dropdown) | Override option labels for clarity |
| `object` | `json` | Override to `code` for complex objects |
| `array` | `json` | Override to `textarea` for simple string arrays |
| `date` | `date` | Rarely |

**Implementation scope:**

#### Layer 1: Go engine schema declarations
- [ ] `engine` — Define a `ParameterSchema` struct that each node type can declare: field name, Go type (string/number/bool/enum/object/array), default value, constraints (min/max, allowed values), description text, and whether it's required
- [ ] `engine` — Each node type in `pkg/node/library/` registers its `ParameterSchema` alongside its `Executable` (co-located, same pattern as atomiton's schema + definition + executable per node type)
- [ ] `engine` — Schema registry: `pkg/registry/` exposes `GetParameterSchema(nodeType string) *ParameterSchema`
- [ ] `engine` — Unit tests: verify every registered node type has a parameter schema, and schema constraints match execution validation

#### Layer 2: API exposure
- [ ] `apps/api` — `GET /nodes/{type}/schema` endpoint returns the parameter schema as JSON
- [ ] `apps/api` — `GET /nodes/schemas` endpoint returns all node type schemas in one request (for frontend caching)
- [ ] `apps/api` — Integration tests: verify schema responses match expected shapes

#### Layer 3: TypeScript schema consumption
- [ ] `@bnto/core` — TypeScript types for `ParameterSchema` and `FieldConfig` (mirror the Go structs)
- [ ] `@bnto/core` — `createFieldsFromSchema(schema, overrides?)` utility — introspects the parameter schema and returns UI field configs. Port the proven logic from atomiton's implementation (see `~/Code/atomiton/packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts`)
- [ ] `@bnto/core` — Unit tests for `createFieldsFromSchema` — test every auto-derivation case: type inference, label formatting, constraint extraction, default value placeholder, optional detection, enum option generation, and override merging (port test cases from atomiton's test suite)
- [ ] `@bnto/core` — Hook: `useNodeSchema(nodeType)` fetches and caches the parameter schema via React Query

#### Layer 4: Dynamic config panel
- [ ] `apps/web` — Generic `ConfigPanel` component that renders a form from `FieldConfig[]` — maps each `controlType` to the appropriate shadcn input component (Input, Select, Slider, Switch, Textarea, etc.)
- [ ] `apps/web` — Per-bnto override files (only for UI hints that can't be inferred): custom option labels, control type overrides, field grouping/ordering. These are thin — ~20-30% of fields at most
- [ ] `apps/web` — Integration tests: verify the schema-to-UI pipeline (every schema field produces a UI field, every UI field traces back to a schema field — no orphans in either direction, following atomiton's integration test pattern)
- [ ] `apps/web` — Remove hardcoded per-bnto config components once the generic renderer covers all Tier 1 cases

#### Layer 5: Pipeline integrity tests
- [ ] `apps/web` — E2E test: load a bnto tool page, verify the config panel renders the correct controls for that node type's schema (e.g., `/compress-images` shows a quality slider with min=1 max=100)
- [ ] `engine` + `apps/web` — Contract test: when a new node type is added to the Go engine with a parameter schema, the frontend can render a config panel for it with zero new UI code (the test adds a mock node type and verifies the pipeline end-to-end)

**Design decisions (resolved by studying atomiton):**
- **Schema format:** Use a lean custom format (like atomiton's approach), not JSON Schema. JSON Schema is verbose and most of its features (allOf, oneOf, $ref) aren't needed. A simple struct with `{ field, type, default, min, max, enum, required, description }` is sufficient. The Go engine can declare schemas with a builder pattern similar to how Zod works.
- **UI hints in schema vs. registry:** The schema declares constraints and descriptions. The bnto registry (or per-bnto override files) declares purely visual concerns (grouping, display order, custom labels). This keeps the engine UI-agnostic.
- **Fixtures and defaults:** The `.bnto.json` fixture embeds the *current* parameter values. The schema provides the *defaults* and *constraints*. When the config panel loads, it reads parameter values from the fixture and uses the schema for validation and control rendering.

### Recursive Workflow Composability (Web App)

**Why this matters:** The name "bnto" (bento box) is the product metaphor — compartments that contain compartments. A bento box where each compartment can itself be a bento box. The Go engine already supports this: `Definition.Nodes` is recursive (a group node contains child nodes, which can themselves be group nodes). The web app must preserve this composability from day one, even if the initial UI is simple.

**Prior art:** Atomiton's core insight — "everything is a node." A flow is just a saved `NodeDefinition` with child nodes and edges. There's no separate `Flow` type. This means any workflow can be embedded inside another workflow as a sub-node. Bnto's Go engine already has this pattern (`Definition.Nodes []Definition`).

**The risk:** If the web app's config panel, execution UI, or editor treats workflows as flat (single level of nodes), it becomes architecturally expensive to add nesting later. Design for recursion now, even if the MVP only uses one level.

**Principles to preserve:**

1. **A workflow IS a group node.** The top-level `.bnto.json` is itself a `Definition` with `type: "group"`. Don't introduce a separate `Workflow` type that wraps nodes differently than a group node wraps child nodes. One type, recursive.

2. **Config panels work at any depth.** When a user configures a node inside a loop (which is inside a group), the config panel renders the same way as a top-level node. The `ConfigPanel` component receives a node definition — it doesn't care about nesting depth.

3. **Execution progress is recursive.** The execution UI must show per-node progress at every level. A loop node shows its child nodes' progress. A group node shows its child nodes' progress. The progress component is recursive — it renders itself for child nodes.

4. **The editor (Sprint 8) must be depth-aware.** When the visual node editor ships, it needs to support drilling into group nodes to see and edit their children. Don't design the node canvas as a flat 2D space — design it as a zoomable hierarchy.

**Scope (guard rails, not new tasks — apply these principles when building the items below):**

- [ ] `apps/web` — When building the `ConfigPanel` (schema-driven config), ensure it works with node definitions at any nesting depth. Accept a `Definition` (not a workflow-specific type). Test with a node inside a loop inside a group.
- [ ] `apps/web` — When building execution progress UI (Sprint 2 Wave 4), make the progress component recursive. A group node's progress view contains its children's progress views. Test with the `compress-images` fixture (group → loop → image node — 3 levels deep).
- [ ] `apps/web` — When building the JSON editor (Sprint 4), don't flatten the node tree for editing. The editor should represent the recursive structure faithfully. Collapsible sections for group/loop child nodes.
- [ ] `apps/web` — When building the visual editor (Sprint 8), support drill-down into group nodes. Each group is a sub-canvas. Breadcrumb navigation for depth. Study atomiton's `@atomiton/editor` package for the `nodeToReactFlow` / `reactFlowToNode` conversion pattern that handles nested nodes.


### ~~Migrate Animation Primitives from motion/react to Custom CSS~~ — DONE

**Resolved:** CSS animation system built in `globals.mini-motorways-depth.css` with Mini Motorways motion language. Spring easing curves extracted from `tailwindcss-motion` source as CSS `linear()` custom properties. Six keyframes (fade-in, scale-in, slide-up, slide-down, pulse-soft, breathe) registered in `@theme inline` as Tailwind animation utilities. Stagger cascade utility with reduced-motion safety. `contact-form.tsx` migrated from `motion/react` to CSS classes. `motion` package retained for future AnimatePresence needs. Animation component wrappers (PopIn, SlideIn, etc.) were already deleted in a prior cleanup — only `contact-form.tsx` needed migration. See `.claude/rules/animation.md` for the full animation standards.

### Domain Setup: bnto.io Custom Domains

Custom domains for production services. Do Railway first (API) to validate, then Vercel (web app).

**Order:** Railway → Vercel (Railway first to validate DNS + TLS before going live on the main domain)

- [ ] `infra` — Add `api.bnto.io` CNAME in Cloudflare DNS pointing to Railway (`bnto-production.up.railway.app`)
- [ ] `infra` — Configure custom domain in Railway dashboard for the bnto service
- [ ] `infra` — Update `GO_API_URL` in Convex prod to `https://api.bnto.io`
- [ ] `infra` — Verify API health check at `https://api.bnto.io/health`
- [ ] `infra` — Connect `bnto.io` to Vercel (add domain in Vercel dashboard, update Cloudflare DNS)
- [ ] `infra` — Update `SITE_URL` in Convex prod to `https://bnto.io` (already set — verify after DNS propagation)
- [ ] `infra` — Verify auth redirects work on `bnto.io`

### Testing: Split user-journeys.spec.ts Into Per-Persona E2E Files

`user-journeys.spec.ts` is a single 320-line file covering every page and persona (anonymous visitor, authenticated user, dark mode, 404, etc.). It should be split so each file represents one user journey or persona testing the application.

**Proposed structure:**

```
apps/web/e2e/
├── fixtures.ts                    # Shared test fixtures (unchanged)
├── anonymous-visitor.spec.ts      # Home page sections, about, pricing, FAQ, contact, privacy, 404
├── authenticated-user.spec.ts     # Protected pages (workflows, executions, settings)
├── auth-flow.spec.ts              # Signup form, sign-in, sign-out
├── navigation.spec.ts             # Navbar links, page transitions
├── dark-mode.spec.ts              # Theme toggle, dark mode rendering
├── bnto-tool-page.spec.ts         # Tool pages (/compress-images, etc.)
├── theme-demo.spec.ts             # Showcase page (already separate)
├── execution-flow.spec.ts         # Execution lifecycle (already separate)
├── file-drop.spec.ts              # File drop interactions (already separate)
├── seo-metadata.spec.ts           # SEO metadata (already separate)
└── bnto-config.spec.ts            # Bnto config (already separate)
```

Each file is a self-contained persona: "What does an anonymous visitor see?", "What can an authenticated user do?", "Does dark mode work?" Screenshots stay co-located with their spec file.

- [ ] `apps/web` — Split `user-journeys.spec.ts` into per-persona spec files
- [ ] `apps/web` — Move screenshots into per-spec `__screenshots__/` directories
- [ ] `apps/web` — Verify all tests pass after restructure

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
