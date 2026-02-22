# Bnto — Build Plan

**Last Updated:** February 2026
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

---

## Current State

**Status:** Phase 0 complete. Phase 1 Sprint 1 in progress — auth migration + Vercel deployment.

**Engine (complete):** Go CLI with 10 node types (all >90% test coverage), integration test fixtures, CLI smoke tests, Go HTTP API server with 20+ integration tests, BntoService shared API layer.

**Web app (partially built):** Next.js app shell, Convex Auth being replaced with Better Auth, passphrase gate being removed, deployed to Railway migrating to Vercel.

**Packages (partially built):** @bnto/core hooks + Convex adapter, @bnto/ui shadcn primitives, @bnto/editor stub, @bnto/auth (migrating to Better Auth), @bnto/backend Convex schema + functions.

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine: 10 node types, orchestration, validation, storage, secrets, path resolution
- [x] Go API server: HTTP handlers wrapping BntoService (apps/api/)
- [x] Contract tests: Go JSON responses match @bnto/core TypeScript types
- [x] @bnto/core: React Query + Convex adapter, hooks (useWorkflows, useExecution, useRunWorkflow, etc.)
- [x] @bnto/ui: shadcn design system (Button, Card, Input, Label, Dialog, Select, Tabs, Toaster)
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, run counter
- [x] Web app shell: splash page, passphrase gate, sign-in/up, whitelist gate, nav + theme toggle
- [x] Playwright E2E infrastructure: 6 tests (splash gate, auth, navigation)

---

## Revenue & Monetization Context

Pricing, revenue projections, sprint-to-revenue milestone mapping, and "ready to charge" criteria live in Notion — not in this public repo.

> **Notion:** Search the bnto workspace for "SEO & Monetization Strategy" using the Notion MCP.
> Agents: fetch that page when you need pricing details, quota limits, or to understand what monetization gates apply to the sprint you're working on.



---

## Phase 0: Foundation — COMPLETE

Phase 0 archive removed from repo (nuked with archive folder cleanup).

**What shipped:** Monorepo restructuring, engine solidification with TDD (>90% coverage on all 10 node types), integration test fixtures, CLI smoke tests, Go API server, Convex setup, web app shell, @bnto/core hooks, @bnto/ui shadcn system.

---

## Phase 1: Web App

**Goal:** Ship a web app on Vercel where users can run predefined Bntos, create workflows, and manage their account. Auth gates access. SEO URL routing live from day one.

---

### Sprint 1: Infrastructure Migration
**Goal:** Move from Railway/Convex Auth to Vercel/Better Auth. Clean auth foundation before building features on top.

#### Wave 1 (parallel — setup)

- [x] `@bnto/auth` — Replace Convex Auth with Better Auth + @better-auth/convex adapter
- [x] `@bnto/backend` — Update Convex schema and functions for Better Auth
- [x] `apps/web` — Set up Vercel deployment (vercel.json, env vars, preview deployments)

#### Wave 2 (parallel — integration)

- [x] `@bnto/core` — BntoProvider + ConvexClientProvider + SessionProvider (hydration-safe)
- [x] `apps/web` — Proxy middleware (cookie-presence route protection)
- [x] `apps/web` — AppGate component (splash until auth resolves)
- [x] `apps/web` — Sign-in / sign-up pages using Better Auth client
- [x] `apps/web` — Route definitions (lib/routes.ts — public/private/auth paths)

#### Wave 3 (parallel — cleanup)

- [ ] `@bnto/core` — Sign-out flow (signal cookie + background cleanup pattern)
- [ ] `apps/web` — Remove passphrase gate and whitelist logic (auth is the gate now)
- [ ] `apps/web` — Remove old Convex Auth integration

#### Wave 4 (sequential — verify)

- [ ] `apps/web` — Verify auth flow end-to-end on Vercel preview deployment
- [ ] `apps/web` — Playwright E2E: sign-in, sign-out, route protection

---

### Sprint 2: Predefined Bntos + SEO
**Goal:** Users can land on a Bnto URL, run it immediately, and get output. This is the first moment bnto is useful to a stranger. SEO footprint goes live.

**Reference:** `.claude/strategy/bntos.md` — the full Bnto directory with Tier 1 list, slugs, target queries, and fixture status.

#### Wave 1 (parallel — fixtures)

- [ ] `engine` — Fixture: `compress-images.bnto.json` (verify existing, add to test suite if not already)
- [ ] `engine` — Fixture: `resize-images.bnto.json` (verify existing, add to test suite)
- [ ] `engine` — Fixture: `convert-image-format.bnto.json` (png → webp via image node)
- [ ] `engine` — Fixture: `rename-files.bnto.json` (filesystem + edit-fields nodes)
- [ ] `engine` — Fixture: `clean-csv.bnto.json` (spreadsheet node — strip empty rows/cols, normalize headers)
- [ ] `engine` — Fixture: `rename-csv-columns.bnto.json` (spreadsheet + edit-fields nodes)

#### Wave 2 (parallel — SEO routing + UI)

- [ ] `apps/web` — SEO URL routing: `app/[bnto]/page.tsx` with dynamic slug → Bnto mapping
- [ ] `apps/web` — Per-slug server-side metadata (title, description, og tags) for all 6 Tier 1 Bntos
- [ ] `@bnto/ui` — WorkflowCard component (name, description, node count, last run status)
- [ ] `@bnto/ui` — StatusBadge component (pending, running, completed, failed)
- [ ] `@bnto/ui` — RunButton component (run with loading state)
- [ ] `@bnto/ui` — EmptyState component (no workflows yet)
- [ ] `@bnto/backend` — Execution event logging (every run logged, even anonymous, with timestamp + bnto slug)

#### Wave 3 (parallel — cloud execution)

- [ ] `apps/api` — Deploy Go API server to Railway (private networking to Convex)
- [ ] `@bnto/backend` — Convex actions to proxy execution to Go API and poll for results
- [ ] `@bnto/core` — Execution hooks wired to Convex adapter (start, poll, get results)
- [ ] `apps/web` — File upload UI for Bnto inputs (drag & drop, R2 presigned URL)
- [ ] `apps/web` — Execution progress page (real-time via Convex subscription)
- [ ] `apps/web` — Results page (output download, execution summary)

#### Wave 4 (sequential — test + verify)

- [ ] `apps/web` — Playwright E2E: land on `/compress-images`, upload file, run, download result
- [ ] `apps/web` — Playwright E2E: verify SEO metadata renders correctly on each Tier 1 slug
- [ ] `engine` — Verify all 6 fixtures run clean via `bnto run` integration tests

> **SEO checkpoint:** Before this sprint closes, verify in browser devtools that each `/[bnto]` URL returns correct `<title>` and `<meta description>` in the page source (not client-rendered). If they're missing from the HTML source, the metadata is being rendered client-side and won't be indexed.

> **Monetization checkpoint:** Confirm execution events are being written to Convex with `userId` (or fingerprint), `bntoslug`, `timestamp`, and `durationMs`. Sprint 3 builds the usage dashboard on top of this data — it needs to exist first.

---

### Sprint 3: Dashboard + Run Quota
**Goal:** Authenticated users see their history, run count, and get a meaningful account experience. The monetization infrastructure is in place before any paywall is needed.

#### Wave 1 (parallel — quota schema)

- [ ] `@bnto/backend` — Add `runsUsedThisMonth`, `runResetDate`, `planTier` to user schema
- [ ] `@bnto/backend` — Monthly reset cron (reset `runsUsedThisMonth` on the 1st of each month)
- [ ] `@bnto/backend` — Increment run counter on each execution (check before allowing, reject if over limit)
- [ ] `@bnto/core` — `useRunsRemaining()` hook (returns `{ used, limit, resetDate }`)

#### Wave 2 (parallel — dashboard UI)

- [ ] `apps/web` — Dashboard page: list of saved workflows, recent executions, run counter widget
- [ ] `apps/web` — Run counter widget (shows "X of [limit] runs used this month, resets [date]" — limit from Convex user record)
- [ ] `apps/web` — Upgrade prompt component (shown when at or near limit — see copy in Notion: search bnto workspace for "SEO & Monetization Strategy")
- [ ] `apps/web` — Execution history page (list of past runs with status and output links)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: run counter increments after execution
- [ ] `apps/web` — Playwright E2E: upgrade prompt appears when limit reached
- [ ] `@bnto/backend` — Unit tests for run counter logic and monthly reset cron

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app using Wails v2. Same React frontend, local Go engine. Free forever, unlimited runs.

---

### Sprint 4: Wails Bootstrap

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

### Sprint 5: Local Execution

#### Wave 1 (parallel — execution)

- [ ] `apps/desktop` — Execute workflows via Wails Go bindings (all 10 node types)
- [ ] `@bnto/core` — Execution progress streaming via Wails adapter
- [ ] `@bnto/ui` — Execution progress component (node status, duration, logs)

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

## Phase 3: Polish + Monetization

**Goal:** Wire up payments, enforce quotas, and make the product feel complete.

---

### Sprint 6: Stripe + Quota Enforcement

**"Ready to charge" gate:** Before this sprint starts, confirm: real users are running Bntos, run counter data is accurate in Convex, upgrade prompt is built and tested.

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — enforcement)

- [ ] `apps/api` — Reject execution if `runsUsedThisMonth >= limit` (server-side, not client-side)
- [ ] `apps/web` — File size enforcement at R2 presigned URL generation (limits in Notion: "SEO & Monetization Strategy")
- [ ] `@bnto/backend` — Pro feature gates: 30-day history retention, team sharing (up to 5 members)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: free user hits limit, sees upgrade prompt, upgrades via Stripe
- [ ] `apps/web` — Playwright E2E: Pro user runs >25 flows without hitting limit

---

### Sprint 7: Visual Editor + History

- [ ] `@bnto/editor` — Drag-and-drop node canvas (React Flow or custom)
- [ ] `@bnto/editor` — Node palette with all 10 node types
- [ ] `@bnto/editor` — Property editor per node
- [ ] `@bnto/editor` — JSON ↔ visual round-trip (edit in either mode)
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
