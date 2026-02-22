# Bnto Cloud + Desktop Strategy

**Date:** 2026-02-06
**Updated:** 2026-02-21
**Status:** Strategy Document — Active
**Supersedes:** Extends monorepo-structure.md
**Goal:** Evolve Bnto from a Go CLI into three products sharing a frontend

---

## Products

- **Bnto Web + Cloud** (Phase 1): Web app on Vercel + Go execution service on Railway — UI, auth, cloud execution for predefined Bntos (Convex Cloud for DB, Better Auth)
- **Bnto Desktop** (Phase 2, free): Wails v2 app — same React frontend, local Go engine execution, no account needed

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
| Vercel hosts Next.js | **Confirmed** | First-class Next.js platform, free tier, edge functions |
| Railway hosts Go services | **Confirmed** | Go is first-class on Railway, used for cloud execution (Phase 1) |
| Convex Cloud for DB | **Confirmed** | Managed service, free tier, real-time subscriptions |
| Better Auth with Convex | **Confirmed** | `@better-auth/convex` adapter, email/password + OAuth |
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
│  │  (Vercel)     │  │ (Local)      │  │   (Terminal) │  │
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
├── web/                  # @bnto/web — Next.js (Vercel, Better Auth)
└── desktop/              # @bnto/desktop — Vite/React (Wails v2)
packages/
├── core/                 # @bnto/core — THE API layer. All backend communication.
│                         #   Exposes hooks: useWorkflows(), useExecution(), etc.
│                         #   Zustand for client state, React Query for server state
│                         #   Runtime detection swaps transport (Convex vs Wails)
│                         #   Consumer code has NO idea how it talks to the backend
└── @bnto/                # Scoped internal packages
    ├── auth/             # @bnto/auth — Cloud auth (web only)
    └── backend/          # @bnto/backend — Convex schema, functions, business logic

apps/web/
├── components/           # UI components (shadcn wrappers, design system) — future @bnto/ui
└── app/                  # Pages, editor features — future @bnto/editor
```

> **Co-location note (Feb 2026):** UI components and editor features are co-located in `apps/web/` until the desktop app creates a real second consumer. Author with extraction in mind — when desktop arrives, shared UI extracts to `@bnto/ui` and editor to `@bnto/editor`.

### UI as Design System

The design system is the single source of truth for appearance, currently co-located in `apps/web/components/`:

- shadcn components are the primitives — wrappers are thin today
- Light and dark mode working out of the box from day one
- Theming is centralized — changing theme tokens updates everything
- The Next.js app composes components into pages
- Single responsibility: UI owns appearance, `@bnto/core` owns data, apps own composition/routing

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

**Decision:** Wails v2 (stable) as documented in monorepo-structure.md. **Target v2, not v3.**

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

### 3.3 Next.js on Vercel for the Web App

**Decision:** Next.js deployed to Vercel for the web frontend.

**Rationale:**
- First-class Next.js platform (built by the same team)
- Free tier sufficient for MVP
- Edge functions, SSR, ISR out of the box
- Simple deployment (git push to deploy)
- API routes for proxying to Go backend (Phase 3)

### 3.4 Convex Cloud as the Database

**Decision:** Convex Cloud (managed service) for state and real-time.

**Rationale:**
- Real-time subscriptions for execution progress (sub-50ms updates)
- Managed service — no infrastructure to maintain
- Free tier available
- Document model maps naturally to Bnto's JSON workflow definitions
- Workflow component (`@convex-dev/workflow`) for long-running executions
- File storage API for uploaded files

### 3.5 Better Auth for Authentication

**Decision:** Better Auth with Convex adapter instead of Convex Auth.

**Rationale:**
- More flexible auth framework (email/password, OAuth providers)
- `@better-auth/convex` adapter integrates cleanly with Convex Cloud
- Supports Google, Discord, GitHub OAuth out of the box
- Server-side session management
- Same pattern proven in darkmatter project
- `@bnto/auth` package wraps Better Auth (web only, desktop skips auth)

### 3.6 CLI as the Stable Public API

**Decision:** The CLI is the primary interface to the Go engine. All consumers trigger CLI operations.

**Rationale:**
- The CLI (`bnto run`, `bnto validate`, `bnto list`) is already tested and stable
- Every operation the web or desktop needs maps to a CLI command
- Keeps the API surface small, consistent, and well-documented
- The Go service on Railway wraps CLI operations (same logic, exposed via HTTP)
- Desktop (Wails) calls the same underlying Go functions that the CLI calls
- The TUI (Bubble Tea) is internal/beta — not public-facing

### 3.7 Go Backend on Railway for Cloud Execution (Phase 1)

**Decision:** Separate Go service on Railway that wraps CLI operations as HTTP endpoints. Part of Phase 1 — cloud execution ships alongside the web app.

**Rationale:**
- The Go execution engine is the compute-heavy part
- Convex handles state/real-time, Go handles execution
- Go service receives execution requests, streams progress back via Convex mutations
- Scales independently from the frontend
- Service endpoints mirror CLI commands: `/api/run`, `/api/validate`, `/api/list`
- Private networking between Go service and Vercel has **no timeout limit** (Railway feature)

### 3.8 Zustand for Client State

**Decision:** Zustand for all client-only state management.

**Rationale:**
- Lightweight (< 1KB), no boilerplate, no context providers needed
- Client state (editor content, selected workflow, UI preferences) is separate from server state
- Simple API: `create()` a store, use it as a hook
- No re-render cascades — components only re-render when their slice of state changes

### 3.9 React Query for Server State

**Decision:** React Query (TanStack Query) as the universal data fetching layer.

**Rationale:**
- Handles caching, background refetching, loading/error states, optimistic updates
- `@convex-dev/react-query` adapter preserves Convex real-time subscriptions through React Query's interface
- Desktop (Wails) path uses React Query with Wails adapter — same hook API, different transport
- Universal layer means `@bnto/core` hooks work identically on web and desktop
- Components never know if data comes from Convex subscriptions or Wails Go bindings

### 3.10 Desktop Shares the Web Frontend

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
Phase 0: Solidify Go Engine (CLI + tests) ✓ COMPLETE
    ↓ verified by: unit tests, integration tests, CLI smoke tests
Phase 1: Build Web App + Cloud Execution (Next.js + Convex + Better Auth + Railway)
    ↓ verified by: Playwright E2E tests, TypeScript compilation, API integration tests
Phase 2: Build Desktop App (Wails v2 + local execution)
    ↓ verified by: Wails integration tests, local execution tests
Phase 3: Monetization + Visual Editor
    ↓ verified by: Stripe integration tests, editor E2E tests
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
| Component tests | UI components render correctly, handle states | Vitest + React Testing Library |
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
pnpm --filter @bnto/web exec playwright test

# All layers
task test:all
```

### CI Pipeline

```
Push to main →
  ├── Go tests (all packages + integration)
  ├── Frontend tests (vitest + playwright)
  ├── Build verification (go build, pnpm build)
  └── Deploy to staging →
        ├── Vercel preview deployment (web app)
        └── Railway staging (Go API, Phase 3)
```

---

## 7. MVP Phases

> **Revised phase order (Feb 2026):** Cloud execution moved up to Phase 1. Desktop deferred to Phase 2. Rationale: a casual user who can't run anything isn't a user at all. Lean into the monetization model early.
>
> ```
> Old order: Web UI → Desktop execution → Cloud execution
> New order: Web UI + Cloud execution (predefined Bntos) → JSON editor → Desktop app
> ```

### Phase 0: Engine Solidification (TDD Foundation) — COMPLETE

**Goal:** The Go engine is bulletproof. Every node type, every CLI command, every edge case is tested.

**Status:** Complete. All 10 node types >90% coverage, integration tests passing, CLI smoke tests passing.

### Phase 1: Web App + Cloud Execution (Predefined Bntos)

**Goal:** Ship a web app where anyone can open a browser, pick a predefined Bnto, drop files, and run it. Execution backed by the Go HTTP API on Railway. Free. No account needed for core experience.

**Scope:**
- Next.js app on **Vercel** with Better Auth (email/password + OAuth)
- **Convex Cloud** for database (workflows, executions, users)
- No account required for first use — anonymous runs tracked by browser fingerprint
- Predefined Bntos with SEO URL routing (Sprint 2)
- Cloud execution via Railway Go API (Sprint 2)
- File transit via Cloudflare R2: Browser → R2 → Railway → R2 → Browser (temp, 1-hour TTL)
- Dashboard with run quota tracking (Sprint 3)
- JSON code editor for custom workflows (Sprint 4)
- `@bnto/core` hooks for all data access (transport-agnostic)
- UI components co-located in `apps/web` (no separate `@bnto/ui` or `@bnto/editor` packages yet)
- Playwright E2E tests for core flows

**Infrastructure:** Vercel (Next.js) + Convex Cloud (DB + real-time) + Railway (Go execution) + Cloudflare R2 (temp file storage)

**Target user:** Anyone who Googles "compress images online free" or "clean csv online" — no download, no setup, just works.

### Phase 2: Desktop App (Local Execution — Free)

**Goal:** Free desktop app using Wails v2. Same React frontend, local Go engine execution. Free forever, unlimited runs. No account needed.

**Scope:**
- Bootstrap Wails v2 desktop app
- Same React frontend as web — Wails webview renders the shared React code
- `@bnto/core` runtime detection routes requests to Wails Go bindings instead of Convex
- React Query + Wails adapter replaces React Query + Convex adapter (same hook API)
- Full local execution (all 10+ node types including shell-command)
- Real-time execution progress in the UI
- Purely local — no account required, no cloud connectivity
- Desktop and web share one frontend codebase
- Component tests for Wails-specific integration

**Infrastructure:** None — runs entirely on user's machine.

**BYOK AI (desktop advantage):** Desktop is the natural home for AI-powered nodes. Users already have API keys for Claude, OpenAI, etc. BYOK (Bring Your Own Key) means zero inference costs for Bnto, zero data privacy concerns (files never leave the machine), and zero rate limit headaches. The existing secrets system (`engine/pkg/secrets/`) handles API keys — `ANTHROPIC_API_KEY` is just another secret. Cloud users would need to supply keys via settings UI or Bnto would need to proxy calls (cost/privacy decision deferred). See [bntos.md Tier 4](../strategy/bntos.md#tier-4-ai-powered-nodes-backlog--requires-async-execution) for planned AI node types.

**Why Phase 2 (not Phase 1):** Desktop is the trust signal and top-of-funnel growth driver, but cloud execution reaches users faster — no download required. Ship cloud first, desktop second.

### Phase 3: Monetization + Polish + Visual Editor

**Goal:** Revenue, polish, and the next-gen editing experience. By this point we have real users, real signal, and a working product worth paying for.

**Scope:**
- Stripe integration for paid tiers (Free: 25 runs/month, Pro: $8/month or $69/year)
- Usage limits per plan tier (runs/month, file size, timeout limits)
- Run quota enforcement server-side
- Execution history with detailed logs (Pro: 30-day retention)
- Workflow versioning and duplication
- Visual workflow editor (drag-and-drop nodes, connect edges)
- Priority processing queue (Pro users skip the line)
- Team sharing up to 5 members (no per-seat pricing)
- Pre-approved shell commands in cloud (ffmpeg, imagemagick, etc.)
- Note: Team/org features beyond 5-member sharing NOT in scope until demand justifies it

---

## 8. Monetization & Freemium Model

Pricing details, tier structure, run limits, and file size limits live in Notion — not in this public repo. Search the bnto workspace for "SEO & Monetization Strategy" for the current pricing model.

**Core model:** Perpetual free tier with monthly run refresh. Building workflows is free, running is metered. No feature gating — all node types available on free tier, limit is execution count only.

**Implementation:** Run counter tracked in Convex, monthly cron reset, Stripe webhook for plan upgrades. See `PLAN.md` Sprint 3 (quota tracking) and Sprint 7 (Stripe) for implementation details.

---

## 9. Open Source Philosophy

Bnto is and stays **open source**. The paid cloud service is not about locking people out — it's about convenience. Anyone can pull the repo, build the binary, and run Bnto themselves for free.

What the cloud service sells:
- **Hosting and running the code for you** — no setup, no local machine dependency
- **Managed infrastructure** — we handle the servers, you click Run

Every node type, every capability — it's all in the open source repo. The cloud's value proposition is pure convenience, not proprietary features.

---

## 10. Cost Control

**Target: $0/month until revenue.** Leverage free tiers aggressively.

**Approach:**
- **Vercel** free tier for web app (sufficient for MVP traffic)
- **Convex Cloud** free tier for database + real-time
- **Desktop execution is free** — no infrastructure cost, runs on user's machine
- **Railway** needed from Phase 1 for cloud execution ($5/mo hobby tier)
- Hard limits on cloud free-tier execution (specific limits in Notion: "SEO & Monetization Strategy")
- Monitor infrastructure costs closely before setting paid tier prices

**Key insight:** Cloud execution ships in Phase 1 to lean into the monetization model. Users can run predefined Bntos immediately — no download, no setup. Railway's $5/mo hobby tier covers MVP scale. Desktop (Phase 2) adds free local execution for power users.

---

## 11. Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Railway 15-min public timeout kills long workflows | High | Medium | Async execution + private networking (no timeout) between services. Phase 3 concern — desktop (Phase 2) has no timeouts. |
| shell-command unusable in cloud | Medium | Certain | Disable in cloud, full support on desktop. Position desktop for power users. |
| Sharing React components across Vite and Next.js | Low | Low | UI co-located in apps/web for now; pnpm workspaces + @bnto/ui extraction when desktop arrives |
| Wails v2 lacks features v3 would provide | Low | Low | v2 is stable and sufficient for MVP. v3 features (multi-window, system tray) aren't needed |
| Solo developer scope creep | High | High | Ruthless MVP scoping. JSON editor + templates, not visual editor. No team features. Cloud before desktop. |
| Cloud file handling complexity | Medium | Medium | Start with in-memory processing only (upload → process → download). Phase 3+ concern. |
| Engine bugs discovered late | High | Medium | **Mitigated by Phase 0.** TDD bottom-up approach means engine is tested before any frontend is built. |
| AI agents can't verify their own work | High | Medium | **Mitigated by TDD.** Every layer has runnable test commands. Agents verify by running tests, not by claiming completion. |

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

- [Monorepo Structure](monorepo-structure.md) — Go + React monorepo architecture
- [Code Standards](../rules/code-standards.md) — Bento Box Principle, code philosophy
