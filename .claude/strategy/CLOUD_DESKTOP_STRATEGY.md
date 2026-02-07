# Bnto Cloud + Desktop Strategy

**Date:** 2026-02-06
**Status:** Strategy Document — Ready for Review
**Supersedes:** Extends MONOREPO_STRUCTURE.md, informs WAILS_DESKTOP_STRATEGY.md
**Goal:** Evolve Bnto from a Go CLI/TUI into two products sharing a frontend

---

## Products

- **Bnto Desktop** (free): Download and run locally, no account needed
- **Bnto Cloud** (paid): Web app on Railway + Convex, runs workflows in the cloud

---

## Table of Contents

1. [Feasibility Assessment](#1-feasibility-assessment)
2. [Architecture Design](#2-architecture-design)
3. [Technology Decisions](#3-technology-decisions)
4. [Convex Data Model](#4-convex-data-model)
5. [Cloud Execution Model](#5-cloud-execution-model)
6. [Development Philosophy: TDD Bottom-Up](#6-development-philosophy-tdd-bottom-up)
7. [MVP Phases](#7-mvp-phases)
8. [Monetization & Freemium Model](#8-monetization--freemium-model)
9. [Open Source Philosophy](#9-open-source-philosophy)
10. [Cost Control](#10-cost-control)
11. [Risk Assessment](#11-risk-assessment)

---

## 1. Feasibility Assessment

### Infrastructure Viability (Verified)

| Claim | Status | Evidence |
|-------|--------|----------|
| Railway hosts Next.js + Go simultaneously | **Confirmed** | Railway monorepo deployment supports separate root directories per service with different languages |
| Convex self-hostable on Railway | **Confirmed** | Multiple Railway templates available: basic, +Postgres, +MySQL, +S3 backups (194+ deployments, 100% success rate) |
| Railway 15-min HTTP timeout | **Confirmed** | Increased from 5 min in June 2025. No timeout on private networking (service-to-service) |
| Convex Workflow component | **Confirmed** | `@convex-dev/workflow` — supports async workflows, retry, cancellation, reactive status subscriptions |
| Wails stable for desktop | **Confirmed** | Wails v2.11.0 (Nov 2025) is production-ready. v3 is alpha — target v2 |

### Architecture Feasibility

The layered abstraction (shared `@bnto/core` → multiple clients → single Go engine) is well-established pattern. pnpm workspaces handle sharing React packages between Vite (Wails) and Next.js cleanly. Go compiles for Railway (first-class citizen) and cross-compiles for all desktop platforms trivially.

**Verdict: Feasible.** No blockers identified. All infrastructure claims verified.

---

## 2. Architecture Design

### Layered Abstraction — Every Layer Only Knows the Layer Below

```
┌─────────────────────────────────────────────────────────┐
│                    User-Facing Apps                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Next.js Web  │  │ Wails Desktop│  │   CLI        │  │
│  │  (Railway)    │  │ (Local)      │  │   (Terminal) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
└─────────┼──────────────────┼──────────────────┼─────────┘
          │                  │                  │
┌─────────▼──────────────────▼──────────────────▼─────────┐
│  @bnto/core — THE abstraction layer                     │
│  Exposes: useWorkflows(), useExecution(), useRunWorkflow()│
│  Consumer code knows NOTHING about what's below          │
│  ┌─────────────┐  ┌──────────────┐                      │
│  │   Zustand    │  │  React Query  │                     │
│  │(client state)│  │(server state) │                     │
│  └─────────────┘  └──────┬───────┘                      │
│      ┌───────────────────┤                               │
│      ▼                   ▼                               │
│  ┌────────────┐  ┌────────────┐                         │
│  │  Convex    │  │   Wails    │  ← runtime detection    │
│  │  adapter   │  │   adapter  │    swaps transport       │
│  └─────┬──────┘  └─────┬──────┘                         │
└────────┼───────────────┼────────────────────────────────┘
         │               │               │
┌────────▼───────────────▼───────────────▼────────────────┐
│  Go Engine (CLI commands = the stable public API)        │
│  bnto run, bnto validate, bnto list                   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │
│  │ engine   │ │ registry │ │ storage  │ │ paths    │   │
│  │(orchestr)│ │(node reg)│ │(persist) │ │(resolve) │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │
│  TUI — internal/beta, NOT public                         │
└─────────────────────────────────────────────────────────┘
```

### Frontend Package Structure

Standard Turborepo layout at repo root with `@bnto/` namespace (n8n pattern):

```
apps/
├── web/                  # @bnto/web — Next.js (Railway, cloud auth)
└── desktop/              # @bnto/desktop — Vite/React (Wails v2)
packages/
└── @bnto/               # Scoped internal packages
    ├── core/             # @bnto/core — THE API layer. All backend communication.
    │                     #   Exposes hooks: useWorkflows(), useExecution(), etc.
    │                     #   Zustand for client state, React Query for server state
    │                     #   Runtime detection swaps transport (Convex vs Wails)
    │                     #   Consumer code has NO idea how it talks to the backend
    ├── ui/               # @bnto/ui — shadcn thin wrappers (Button, Card, Dialog)
    │                     #   Pure presentational. Uses shadcn as primitives.
    │                     #   Ready for future customization without changing consumers.
    └── editor/           # @bnto/editor — Workflow editor (JSON editor for MVP)
                          #   Consumes @bnto/ui for primitives, @bnto/core for data
```

### Why Thin shadcn Wrappers

`@bnto/ui` components are thin wrappers around shadcn today. Tomorrow you can customize internals (animations, theming, behavior) without touching any consumer code. The abstraction pays off when you want a consistent Bnto look-and-feel across web and desktop.

### UI as Design System

`@bnto/ui` IS the design system — not just components, but the single source of truth for appearance:

- shadcn components are the primitives — wrappers are thin today
- Light and dark mode working out of the box from day one
- Theming is centralized — changing theme tokens updates everything
- The Next.js app is a thin composition layer — import components, compose into pages
- Single responsibility: `@bnto/ui` owns appearance, `@bnto/core` owns data, apps own composition/routing

---

## 3. Technology Decisions

### 3.1 Go Stays as the Core Language

**Decision:** Keep Go. Do not rewrite.

**Rationale:**
- 5-10ms startup, 5MB memory baseline, single 20MB binary
- Native Wails integration (direct function calls, no IPC)
- First-class Railway backend (Go is a first-class citizen on Railway)
- Cross-compiles trivially for all desktop platforms
- Can compile to WebAssembly if browser execution ever needed
- The only faster alternative is Rust, at 10x development cost
- TypeScript would sacrifice the performance advantage that differentiates Bnto

### 3.2 Wails v2 for Desktop (Not Tauri or Electron)

**Decision:** Wails v2 (stable) as documented in MONOREPO_STRUCTURE.md. **Target v2, not v3.**

**Rationale:**
- Direct Go bindings — execution engine runs in-process, zero serialization overhead
- Lighter than Electron (no bundled Chromium, uses system webview like Tauri)
- Auto-generates TypeScript bindings from Go structs
- Wails frontend uses Vite/React — we share components via packages, not app shells

**Why v2, not v3:**
- v2.11.0 (Nov 2025) is production-ready and stable
- v3 is still alpha (v3.0.0-alpha.57, Jan 2026) with no committed release date
- v3 adds multi-window and system tray support we don't need for MVP
- Desktop is Phase 3 — if v3 stabilizes by then, we can evaluate migration
- Starting on alpha introduces risk of breaking changes and incomplete docs

### 3.3 Next.js for the Web App (Cloud)

**Decision:** Next.js deployed to Railway for the cloud product.

**Rationale:**
- Server-side rendering for auth pages, marketing, dashboard
- API routes for proxying to Go backend
- First-class Convex SDK integration
- Railway has native Next.js support and auto-detection

### 3.4 Convex as the Cloud Backend

**Decision:** Convex (self-hosted on Railway via template) for state, auth, and real-time.

**Rationale:**
- Real-time subscriptions for execution progress (sub-50ms updates)
- Built-in auth (Convex Auth for magic links, OAuth, email/password)
- Document model maps naturally to Bnto's JSON workflow definitions
- Workflow component (`@convex-dev/workflow`) for long-running executions
- File storage API for uploaded files
- Self-hosted on Railway keeps all infrastructure in one place

**License note:** Convex self-hosted uses the Functional Source License (FSL) Apache 2.0 — source-available with a non-compete restriction that converts to Apache 2.0 after two years. This is fine for Bnto's use case (we're not reselling Convex), but it's not truly "open source" in the OSI sense. Worth noting for documentation accuracy.

### 3.5 CLI as the Stable Public API

**Decision:** The CLI is the primary interface to the Go engine. All consumers trigger CLI operations.

**Rationale:**
- The CLI (`bnto run`, `bnto validate`, `bnto list`) is already tested and stable
- Every operation the web or desktop needs maps to a CLI command
- Keeps the API surface small, consistent, and well-documented
- The Go service on Railway wraps CLI operations (same logic, exposed via HTTP)
- Desktop (Wails) calls the same underlying Go functions that the CLI calls
- The TUI (Bubble Tea) is internal/beta — not public-facing

### 3.6 Go Backend on Railway for Execution

**Decision:** Separate Go service on Railway that wraps CLI operations as HTTP endpoints.

**Rationale:**
- The Go execution engine is the compute-heavy part
- Convex handles state/real-time, Go handles execution
- Go service receives execution requests, streams progress back via Convex mutations
- Scales independently from the frontend
- Service endpoints mirror CLI commands: `/api/run`, `/api/validate`, `/api/list`
- Private networking between Go service and Next.js has **no timeout limit** (Railway feature)

### 3.7 Zustand for Client State

**Decision:** Zustand for all client-only state management.

**Rationale:**
- Lightweight (< 1KB), no boilerplate, no context providers needed
- Client state (editor content, selected workflow, UI preferences) is separate from server state
- Simple API: `create()` a store, use it as a hook
- No re-render cascades — components only re-render when their slice of state changes

### 3.8 React Query for Server State

**Decision:** React Query (TanStack Query) as the universal data fetching layer.

**Rationale:**
- Handles caching, background refetching, loading/error states, optimistic updates
- `@convex-dev/react-query` adapter preserves Convex real-time subscriptions through React Query's interface
- Desktop (Wails) path uses React Query with Wails adapter — same hook API, different transport
- Universal layer means `@bnto/core` hooks work identically on web and desktop
- Components never know if data comes from Convex subscriptions or Wails Go bindings

### 3.9 Desktop Shares the Web Frontend

**Decision:** Wails v2 renders the same React frontend in a system webview. No separate desktop frontend.

**Rationale:**
- `@bnto/core` detects the runtime environment (browser vs Wails webview) and swaps the transport adapter internally
- Components, hooks, and UI are 100% shared between web and desktop
- Only the transport layer differs: Convex adapter (web) vs Wails adapter (desktop calling Go bindings)
- Reduces maintenance burden — one frontend codebase, two deployment targets

---

## 4. Convex Data Model

### Schema

```
users          → { email, name, plan, runsUsed, runLimit, runsResetAt, createdAt }
workflows      → { userId, name, definition (JSON), version, isPublic, createdAt, updatedAt }
executions     → { userId, workflowId, status, progress, currentNode, startedAt, completedAt, error }
executionLogs  → { executionId, nodeId, level, message, timestamp }
files          → { userId, executionId?, name, storageId, size, mimeType, createdAt }
secrets        → { userId, name, encryptedValue }
subscriptions  → { userId, plan, stripeId, status, currentPeriodEnd }
```

### Key Relationships

- `users` 1:N `workflows` — user owns workflows
- `workflows` 1:N `executions` — workflow has execution history
- `executions` 1:N `executionLogs` — execution streams log entries
- `users` 1:N `files` — user uploads files for processing
- `users` 1:1 `subscriptions` — user has a billing subscription

### Real-Time Patterns

- Execution progress: client subscribes to `executions` doc by ID, Go service mutates `progress`/`currentNode`/`status`
- Run counter: client subscribes to `users` doc, sees `runsUsed`/`runLimit` update in real-time
- Execution logs: client subscribes to `executionLogs` filtered by `executionId`, new entries appear as Go service inserts them

---

## 5. Cloud Execution Model

### Node Type Support in Cloud

| Node Type | Cloud Support | Notes |
|-----------|--------------|-------|
| edit-fields | Full | Pure data transformation |
| http-request | Full | Works identically |
| transform | Full | Expression evaluation, no I/O |
| group | Full | Orchestration only |
| loop | Full | Orchestration only |
| parallel | Full | Orchestration only |
| image | Full | Go image libraries work server-side |
| spreadsheet | Full | Upload CSV/Excel, process in memory |
| file-system | Adapted | Cloud file storage (Convex) instead of local paths |
| shell-command | Limited | Pre-approved commands only OR disabled in cloud |

### Cloud File Handling

- Users upload files through the web UI → stored in Convex file storage
- Workflow nodes reference files by Convex file ID instead of local paths
- Cloud `file-system` node operates on a virtual workspace (temp directory per execution)
- Results (output files) downloadable from Convex after execution

### Timeout Strategy

- Railway public HTTP timeout: 15 minutes (hard limit)
- Railway private networking: **no timeout** (service-to-service)
- Solution: Async execution model
  1. Client sends "start execution" → gets execution ID immediately
  2. Go backend runs workflow asynchronously
  3. Progress streamed to Convex via mutations (real-time to client)
  4. Client subscribes for completion via Convex real-time
  5. Individual node timeout: configurable (default 10 min, max per plan tier)
- Go service communicates with Convex over private networking (no 15-min limit)

### shell-command in Cloud

- **MVP:** Disabled in cloud
- **Future:** Pre-approved command allowlist (ffmpeg, imagemagick, etc.) installed in Railway container
- **Long-term:** Sandboxed execution environments (containers per workflow)

---

## 6. Development Philosophy: TDD Bottom-Up

### The Principle

**Build from the engine outward, not from the UI inward.** Each layer is solidified through tests before the next layer is built on top of it. This is critical for an agentic development workflow — agents need automated verification at every step.

```
Phase 0: Solidify Go Engine (CLI + tests)
    ↓ verified by: unit tests, integration tests, CLI smoke tests
Phase 1: Build API Layer (Go HTTP service)
    ↓ verified by: API integration tests, contract tests
Phase 2: Build Frontend (Next.js + React)
    ↓ verified by: Playwright E2E tests against real API
```

### Why Bottom-Up

1. **The engine IS the product.** If `bnto run` doesn't work correctly, nothing above it matters.
2. **Tests as specification.** Writing tests for the CLI first means we define exactly what the API needs to expose — the tests ARE the spec.
3. **Agentic development.** AI agents can verify their own work by running tests. Without automated verification at each layer, agents can't confirm correctness.
4. **Confidence compounds.** When the API layer is built on a tested engine, you know bugs are in the API layer, not the engine. When the UI is built on a tested API, you know bugs are in the UI, not the API.

### Layer-by-Layer Test Strategy

#### Layer 1: Go Engine (CLI)

**Goal:** Every CLI operation is tested. Every node type has unit + integration tests.

**Approach:** Simulate what a user wants from the UI by writing bntos and running them against the CLI.

| Test Type | What It Covers | Runner |
|-----------|---------------|--------|
| Unit tests | Individual node execution, template resolution, validation | `go test ./pkg/...` |
| Integration tests | Multi-node workflows, loops, parallel execution, edge cases | `go test ./tests/integration/...` |
| CLI smoke tests | `bnto run`, `bnto validate`, `bnto list` with real .bnto.json files | Shell scripts or `go test` with `os/exec` |
| Fixture bntos | Real-world workflows in `tests/fixtures/` that exercise common patterns | Executed by integration tests |

**What "solidified" means:** All existing node types have >90% test coverage. All CLI commands have smoke tests. A comprehensive fixture suite of .bnto.json files covers the patterns users will build in the UI.

**Key insight:** The fixture bntos ARE the user stories. "Compress PNGs" as a bnto file, "Fetch API and transform", "Batch resize images" — these test the engine AND define the templates we'll ship in the cloud product.

#### Layer 2: Go API Service (HTTP)

**Goal:** Every API endpoint is tested. Contract tests ensure the API matches what `@bnto/core` expects.

| Test Type | What It Covers | Runner |
|-----------|---------------|--------|
| API integration tests | Each endpoint returns correct responses for valid/invalid input | `go test` with `net/http/httptest` |
| Contract tests | API responses match TypeScript type definitions in `@bnto/core` | JSON schema validation or generated contract tests |
| Load tests | Concurrent execution requests, progress streaming under load | `go test -bench` or k6 |
| Railway integration tests | Deployment works, services communicate, Convex mutations land | CI pipeline on Railway (staging environment) |

**What "solidified" means:** Every endpoint has happy-path and error-path tests. Contract tests prevent API drift from the frontend's expectations.

#### Layer 3: Frontend (Next.js + Wails)

**Goal:** Core user flows work end-to-end. The UI is a thin composition layer — most logic lives in `@bnto/core`.

| Test Type | What It Covers | Runner |
|-----------|---------------|--------|
| Component tests | `@bnto/ui` components render correctly, handle states | Vitest + React Testing Library |
| Core package tests | `@bnto/core` methods work with mock and real backends | Vitest |
| E2E tests | User flows: upload workflow → edit → run → see results | Playwright against staging |
| Visual regression | UI doesn't break across changes | Playwright screenshots |

**What "solidified" means:** Playwright tests cover the 5-10 core user flows. A user can upload, edit, run, and see results without manual QA.

### Agentic Development Workflow

Agents (Claude, CI bots) follow this loop at every layer:

```
1. Write/modify code
2. Run tests for that layer
3. If tests fail → fix and retry
4. If tests pass → move to next task
5. Never claim "done" without passing tests
```

**Test commands agents can run:**

```bash
# Layer 1: Engine
go test ./pkg/... -race -count=1
go test ./tests/integration/... -race -count=1
bnto validate tests/fixtures/*.bnto.json
bnto run tests/fixtures/smoke-test.bnto.json --dry-run

# Layer 2: API
go test ./cmd/bnto-server/... -race -count=1
# Contract tests TBD

# Layer 3: Frontend
pnpm --filter @bnto/core test
pnpm --filter @bnto/ui test
pnpm --filter @bnto/web exec playwright test

# All layers
task test:all
```

### CI Pipeline (Railway)

```
Push to main →
  ├── Go tests (all packages + integration)
  ├── Frontend tests (vitest + playwright)
  ├── Build verification (go build, pnpm build)
  └── Deploy to staging →
        └── Railway integration tests (API + Convex communication)
```

---

## 7. MVP Phases

### Phase 0: Engine Solidification (TDD Foundation)

**Goal:** The Go engine is bulletproof. Every node type, every CLI command, every edge case is tested.

**Scope:**
- Comprehensive unit tests for all 10 node types (>90% coverage target)
- Integration tests using fixture .bnto.json files that represent real user workflows
- CLI smoke tests for `bnto run`, `bnto validate`, `bnto list`
- Fixture bntos that double as cloud templates:
  - "Compress PNGs" — image node with resize/export
  - "Batch resize images" — loop + image node
  - "Fetch API data" — http-request + transform node
  - "CSV to folders" — spreadsheet + filesystem node
  - "Edit fields pipeline" — edit-fields + transform chain
- Fix any bugs discovered during test writing
- Document the public API surface (what the CLI exposes = what the API layer will wrap)

**Verification:** `go test ./... -race` passes. All fixture bntos validate and run correctly.

**This is the most important phase.** Everything above the engine depends on it being correct.

### Phase 1: Cloud MVP — Upload, Edit, Run

**Goal:** One developer can ship a working cloud product that hooks users.

**Scope:**
- Go API service wrapping CLI operations as HTTP endpoints
  - API integration tests for every endpoint
  - Contract tests matching `@bnto/core` types
- Next.js app with basic auth (Convex Auth, email/password)
- JSON code editor as the primary interface:
  - Upload an existing .bnto.json file (populates the editor)
  - OR start from a template (the fixture bntos from Phase 0)
  - Edit the JSON directly in-browser (Monaco editor or CodeMirror)
  - Hit "Run" to execute
- Execute on the Go backend (Railway)
- Real-time progress via Convex subscriptions
- Support: edit-fields, http-request, transform, image, spreadsheet, group, loop, parallel
- Workflow list showing saved bntos with last run status
- Run counter: "X runs remaining this month"
- Playwright E2E tests for core flows: sign up → upload → edit → run → see results

**Infrastructure:** Railway (Next.js + Go service + Convex via template)

**Target user:** Solo operators, small business owners needing quick automation without writing code.

### Phase 2: Polish + File Support + Execution History

**Goal:** Make the product feel complete for individual users.

**Scope:**
- Cloud file upload/download (Convex file storage)
- Cloud file-system node (operates on uploaded files, not local paths)
- Execution history with detailed logs (re-run previous executions)
- Workflow versioning and duplication
- Better template library (more pre-built bntos)
- Improved JSON editor (syntax validation, auto-complete for node types)
- Extended test suite: file upload/download flows, history pagination, versioning

### Phase 3: Desktop App (Free Product)

**Goal:** Free desktop app using Wails v2 rendering the same React frontend in a system webview.

**Scope:**
- Bootstrap Wails v2 desktop app (from MONOREPO_STRUCTURE.md plan)
- Same React frontend as web — Wails webview renders the shared React code
- `@bnto/core` runtime detection routes requests to Wails Go bindings instead of Convex
- React Query + Wails adapter replaces React Query + Convex adapter (same hook API)
- Full local execution (all 10+ node types including shell-command)
- Purely local — no account required, no cloud connectivity, no sync
- Desktop and cloud are separate products sharing one frontend codebase
- Component tests for Wails-specific integration

### Phase 4: Monetization + Visual Editor

**Goal:** Revenue and the next-gen editing experience.

**Scope:**
- Stripe integration for paid tiers
- Usage limits per plan tier (runs/month, file storage, timeout limits)
- Visual workflow editor (drag-and-drop nodes, connect edges) — the big Phase 4 investment
- Pre-approved shell commands in cloud (ffmpeg, imagemagick, etc.)
- Note: Team/org features NOT in scope until demand justifies it

---

## 8. Monetization & Freemium Model

### The Hook

The web app is paid, but new users get a **perpetual free tier with monthly run refresh**:

1. **Sign up free** — create an account, explore the UI, upload workflows
2. **Build workflows** — full access to upload and configure (building = free, running = metered)
3. **Monthly runs** — ~5 free executions per month, refreshing monthly
4. **Invest and convert** — users accumulate workflows they depend on, 5/month limit drives upgrades
5. **Pay to scale** — when 5 runs/month isn't enough, upgrade

### Why This Works

- Monthly refresh keeps free users coming back (not a one-and-done trial)
- Users accumulate workflows over time, increasing switching cost
- 5 runs/month is enough to stay engaged but not enough for production use
- No feature gating — all node types available on free tier, limit is execution count only
- Workflows persist forever — constant reminder of value

### Tier Structure (Sketch)

| Tier | Price | Runs/Month | Storage | Timeout/Node |
|------|-------|------------|---------|--------------|
| Free | $0 | 5/month (refreshing) | 100MB | 5 min |
| Starter | $X/mo | 100 | 1GB | 10 min |
| Pro | $Y/mo | 1,000 | 10GB | 15 min |

Team/org tiers come later, only if demand justifies it.

### Implementation Notes

- Run counter tracked in Convex (`users.runsUsed`, `users.runLimit`, `users.runsResetAt`)
- Monthly cron job in Convex resets `runsUsed` for free-tier users
- Execution endpoint checks remaining runs before starting
- Convex real-time subscription shows "X runs remaining" in the UI
- Stripe webhook updates plan tier and run limits on payment
- Workflows are never deleted — only execution is gated

---

## 9. Open Source Philosophy

Bnto is and stays **open source**. The paid cloud service is not about locking people out — it's about convenience. Anyone can pull the repo, build the binary, and run Bnto themselves for free.

What the cloud service sells:
- **Hosting and running the code for you** — no setup, no local machine dependency
- **Managed infrastructure** — we handle the servers, you click Run

Every node type, every capability — it's all in the open source repo. The cloud's value proposition is pure convenience, not proprietary features.

---

## 10. Cost Control

**Critical concern:** Infrastructure costs could spiral if the free tier is too generous.

**Approach:**
- Hard limits on free-tier execution — 5 runs/month + timeout caps (5 min/node)
- Monitor infrastructure costs closely before setting paid tier prices
- Phase 1 is small scale — limited users, limited capabilities, evaluate real costs before scaling
- Paid pricing TBD — set after understanding actual per-execution cost on Railway
- Railway private networking is free (service-to-service), reducing egress costs

This is a "figure out costs before committing to prices" approach. Launch lean, measure, then price accordingly.

---

## 11. Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Railway 15-min public timeout kills long workflows | High | Medium | Async execution + private networking (no timeout) between services |
| shell-command unusable in cloud | Medium | Certain | Disable in MVP, add allowlist later, position desktop for power users |
| Convex self-hosting complexity | Medium | Low | Railway template simplifies; fallback to Convex Cloud if issues arise |
| Convex FSL license restrictions | Low | Low | We're not competing with Convex — FSL allows our use case completely |
| Sharing React components across Vite and Next.js | Low | Low | pnpm workspaces + @bnto/ui package pattern handles this cleanly |
| Wails v2 lacks features v3 would provide | Low | Low | v2 is stable and sufficient for MVP. v3 features (multi-window, system tray) aren't needed |
| Solo developer scope creep | High | High | Ruthless MVP scoping. JSON editor + templates, not visual editor. No team features. Phase 0 TDD prevents building on shaky foundations. |
| Cloud file handling complexity | Medium | Medium | Start with in-memory processing only (upload → process → download). Persistent cloud storage is Phase 2. |
| Engine bugs discovered late in cloud development | High | Medium | **Mitigated by Phase 0.** TDD bottom-up approach means engine is tested before API or UI is built. |
| AI agents can't verify their own work | High | Medium | **Mitigated by TDD.** Every layer has runnable test commands. Agents verify by running tests, not by claiming completion. |

---

## Files to Create/Modify

| Action | File | Description |
|--------|------|-------------|
| **Created** | `.claude/strategy/CLOUD_DESKTOP_STRATEGY.md` | This document (main deliverable) |
| **Update** | `.claude/strategy/MONOREPO_STRUCTURE.md` | Add `apps/web/`, add ConvexClient, rename `api` to `core` |
| **Review** | `.claude/strategy/WAILS_DESKTOP_STRATEGY.md` | Annotate: target v2, desktop is Phase 3 |
| **Review** | `.claude/strategy/DESKTOP_UI_ARCHITECTURE.md` | No conflicts — evaluation still valid |
| **Review** | `.claude/strategy/BENTOBOX_UI_POC.md` | Annotate: visual editor is Phase 4 |

---

## Verification Checklist

- [x] Strategy document internally consistent
- [x] BntoAPI interface design supports ConvexClient, WailsClient, and RestClient
- [x] Railway can host all three services (Next.js, Go, Convex) — confirmed via templates and docs
- [x] pnpm workspace structure supports sharing packages between Vite and Next.js apps
- [x] Convex self-hosting feasibility confirmed (FSL license, Railway template)
- [x] Wails v2 stability confirmed (v2.11.0, production-ready)
- [x] Railway 15-min HTTP timeout confirmed (no limit on private networking)
- [x] Convex Workflow component confirmed (`@convex-dev/workflow`)
- [x] TDD bottom-up development philosophy captured with concrete test strategy per layer
- [x] Agentic development workflow documented with runnable test commands

---

## Related Documents

- [Monorepo Structure](MONOREPO_STRUCTURE.md) — Go + React monorepo architecture
- [Wails Desktop Strategy](WAILS_DESKTOP_STRATEGY.md) — Desktop app implementation plan
- [Desktop UI Architecture](DESKTOP_UI_ARCHITECTURE.md) — UI framework evaluation
- [Bento Box UI PoC](BENTOBOX_UI_POC.md) — Visual editor concept (Phase 4)
- [Bento Box Principle](../BENTO_BOX_PRINCIPLE.md) — Core philosophy
