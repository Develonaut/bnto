# Bnto — Parallel Execution Plan

**How this works:** Tasks are broken into **sprints** (features) and **waves** (dependency groups). All tasks within a wave can run in parallel. Waves within a sprint must run in order.

## Agent Protocol

1. Read this file before starting work
2. Find an unclaimed task in the current sprint/wave
3. Mark it **CLAIMED** before you start (edit the line)
4. When done, mark it `[x]` and remove the CLAIMED tag
5. If all tasks in a wave are claimed/done, move to the next wave

```
- [ ] Available — grab this
- [ ] **CLAIMED** — agent is working on this, pick something else
- [x] Done
```

**Scope rule:** Each task targets ONE package. Don't touch files outside your tagged package unless the task says otherwise.

---

## Current State

**Status:** Phase 0 complete. Phase 1 in progress — web app UI with auth.

**Engine (complete):**
- Go CLI with 10 node types (all >90% test coverage)
- Integration tests with fixture .bnto.json files
- CLI smoke tests (run, validate, list, dry-run)
- Go HTTP API server with 20+ integration tests
- BntoService shared API layer

**Web app (partially built):**
- Next.js app shell with splash page + passphrase gate
- Convex Auth (email/password) — to be replaced with Better Auth
- Whitelist gate for approved users
- Global nav with theme toggle
- Deployed to Railway — migrating to Vercel

**Packages (partially built):**
- @bnto/core — React Query + Convex adapter, hooks, runtime detection
- @bnto/ui — shadcn primitives (Button, Card, Input, Label, Dialog, Select, Tabs, Toaster)
- @bnto/editor — stub only
- @bnto/auth — Convex Auth wrapper — to be replaced with Better Auth
- @bnto/backend — Convex schema, functions, auth config

**TUI (paused):**
- Bubble Tea TUI preserved in engine/pkg/tui/ — not actively maintained

---

## What's Built (reference, don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine: 10 node types, orchestration, validation, storage, secrets, path resolution
- [x] Go API server: HTTP handlers wrapping BntoService (apps/api/)
- [x] Contract tests: Go JSON responses match @bnto/core TypeScript types
- [x] @bnto/core: React Query + Convex adapter, hooks (useWorkflows, useExecution, useRunWorkflow, etc.)
- [x] @bnto/ui: shadcn design system (Button, Card, Input, Label, Dialog, Select, Tabs, Toaster)
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons
- [x] Web app shell: splash page, passphrase gate, sign-in/up, whitelist gate, nav + theme toggle
- [x] Playwright E2E infrastructure: 6 tests (splash gate, auth, navigation)
- [x] TUI: Bubble Tea interactive terminal UI (paused, preserved in engine/pkg/tui/)

---

## Completed Phases (archived)

### Phase 0: Foundation — COMPLETE

Full details at [`.claude/archive/PLAN-PHASE-0.md`](archive/PLAN-PHASE-0.md) (to be created from current PLAN.md).

**Summary:**
- Monorepo restructuring (Go packages renamed, moved to engine/)
- Engine solidification with TDD (>90% coverage on all node types)
- Integration tests using fixture .bnto.json files
- CLI smoke tests for run, validate, list, dry-run
- Go API server with HTTP handlers + integration tests
- Convex setup (schema, auth, run counter)
- Web app shell (splash, auth, whitelist gate, nav)
- @bnto/core hooks and adapter layer
- @bnto/ui shadcn design system

---

## Phase 1: Web App

**Goal:** Ship a web app on Vercel where users can create, edit, and manage workflows. Auth gates access. Better Auth + Convex Cloud for backend.

### Sprint 1: Infrastructure Migration

**Goal:** Move from Railway/Convex Auth to Vercel/Convex Cloud/Better Auth.

#### Wave 1 (parallel — setup)

- [x] `@bnto/auth` — Replace Convex Auth with Better Auth + @better-auth/convex adapter (see [AUTH_PATTERN.md](strategy/AUTH_PATTERN.md))
  - `client.ts` — createAuthClient with convexClient plugin
  - `server.ts` — convexBetterAuthNextJs wrapper (handler, getToken, isAuthenticated, preloadAuthQuery, fetchAuth*)
  - `middleware.ts` — re-export getSessionCookie from better-auth/cookies
  - `hooks/` — useSession, useSignIn, useSignUp, useSignOut (each in own file)
- [x] `@bnto/backend` — Update Convex for Better Auth
  - `auth.config.ts` — getAuthConfigProvider
  - `auth.ts` — betterAuth with email/password + OAuth (Google, Discord)
  - `http.ts` — register auth routes
  - Update schema for Better Auth tables
- [x] `apps/web` — Set up Vercel deployment (vercel.json, env vars, preview deployments)

#### Wave 2 (parallel — integration)

- [x] `@bnto/core` — BntoProvider + ConvexClientProvider + SessionProvider (hydration-safe pattern)
- [x] `apps/web` — Proxy middleware (cookie-presence route protection, see AUTH_PATTERN.md)
- [x] `apps/web` — AppGate component (splash screen until auth resolves)
- [x] `apps/web` — Sign-in/sign-up pages using Better Auth client
- [x] `apps/web` — Route definitions (lib/routes.ts — public/private/auth paths)

#### Wave 3 (parallel — sign-out + cleanup)

- [ ] `@bnto/core` — Sign-out flow (signal cookie + background cleanup pattern)
- [ ] `apps/web` — Remove passphrase gate and whitelist logic (auth is the gate now)
- [ ] `apps/web` — Remove old Convex Auth integration

#### Wave 4 (sequential — verify)

- [ ] `apps/web` — Verify auth flow end-to-end on Vercel preview deployment
- [ ] `apps/web` — Playwright E2E: sign-in, sign-out, route protection

---

### Sprint 2: Dashboard & Workflow Management

**Goal:** Users can see their workflows, create new ones, and manage them.

#### Wave 1 (parallel — components)

- [ ] `@bnto/ui` — WorkflowCard component (name, last run status, node count)
- [ ] `@bnto/ui` — StatusBadge component (pending, running, completed, failed)
- [ ] `@bnto/ui` — RunButton component (run with loading state)
- [ ] `@bnto/ui` — EmptyState component (no workflows yet)

#### Wave 2 (parallel — pages)

- [ ] `apps/web` — Dashboard page (list saved workflows via @bnto/core hooks)
- [ ] `apps/web` — New workflow page (upload .bnto.json or start from template)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: dashboard loads, shows workflows
- [ ] `apps/web` — Playwright E2E: create workflow from template

---

### Sprint 3: JSON Editor

**Goal:** Users can edit .bnto.json files in-browser with validation and syntax highlighting.

#### Wave 1 (parallel — editor core)

- [ ] `@bnto/editor` — JSON editor component (Monaco or CodeMirror)
- [ ] `@bnto/editor` — Schema validation for .bnto.json format
- [ ] `@bnto/editor` — Syntax highlighting for bnto-specific fields

#### Wave 2 (parallel — integration)

- [ ] `apps/web` — Editor page (load workflow → edit → save)
- [ ] `apps/web` — Template selector (fixture bntos from Phase 0 as starting points)
- [ ] `@bnto/core` — Zustand store for editor state (content, dirty flag, selected workflow)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: open editor, modify workflow, save
- [ ] `@bnto/editor` — Unit tests for schema validation

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app using Wails v2. Same React frontend, local Go engine execution. Fastest path to a working product.

### Sprint 4: Wails Bootstrap

**Goal:** Get the React frontend running inside Wails with Go bindings.

#### Wave 1 (parallel — setup)

- [ ] `apps/desktop` — Bootstrap Wails v2 project with Vite + React
- [ ] `@bnto/core` — Implement Wails adapter (replace stubs with real Go bindings)
- [ ] `engine` — Expose engine functions for Wails bindings (RunWorkflow, ValidateWorkflow, etc.)

#### Wave 2 (parallel — integration)

- [ ] `apps/desktop` — Wire up Go ↔ React bindings (auto-generated TypeScript from Go structs)
- [ ] `@bnto/core` — Runtime detection routes to Wails adapter when in Wails webview
- [ ] `apps/desktop` — Local file browser for selecting .bnto.json files

#### Wave 3 (sequential — verify)

- [ ] `apps/desktop` — Verify workflow list, edit, and save work via Wails bindings
- [ ] `apps/desktop` — Verify runtime detection correctly identifies Wails environment

---

### Sprint 5: Local Execution

**Goal:** Run workflows locally through the desktop app with real-time progress.

#### Wave 1 (parallel — execution)

- [ ] `apps/desktop` — Execute workflows via Wails Go bindings (all 10 node types)
- [ ] `@bnto/core` — Execution progress streaming via Wails adapter
- [ ] `@bnto/ui` — Execution progress component (node status, duration, logs)

#### Wave 2 (parallel — features)

- [ ] `apps/desktop` — Execution results view (output data, logs, duration)
- [ ] `apps/desktop` — shell-command node support (full local execution, no restrictions)
- [ ] `apps/desktop` — Error handling and cancellation support

#### Wave 3 (sequential — test + distribute)

- [ ] `apps/desktop` — Integration tests for local execution
- [ ] `apps/desktop` — macOS build (.app bundle)
- [ ] `apps/desktop` — Windows build (.exe)

---

## Phase 3: Cloud Processing (Paid)

**Goal:** Add cloud execution for web app users via Go HTTP API on Railway.

### Sprint 6: Cloud Execution

#### Wave 1 (parallel — connect)

- [ ] `apps/api` — Deploy Go API server to Railway
- [ ] `@bnto/backend` — Convex actions to proxy execution requests to Go API
- [ ] `@bnto/core` — Convex adapter execution methods (start, poll progress, get results)

#### Wave 2 (parallel — features)

- [ ] `apps/web` — Execution page (real-time progress via Convex subscriptions)
- [ ] `apps/web` — Results page (output download, execution logs)
- [ ] `@bnto/backend` — Run counter enforcement (check remaining runs before execution)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: run workflow → see progress → view results
- [ ] `apps/api` — Railway integration tests (services communicating)

---

## Phase 4: Polish + Monetization + Visual Editor

### Sprint 7: Payments & File Handling

- [ ] Stripe integration (Free, Starter, Pro tiers)
- [ ] Cloud file upload/download (Convex file storage)
- [ ] Execution history with detailed logs
- [ ] Workflow versioning and duplication
- [ ] Waitlist/signup flow for public access

### Sprint 8: Visual Workflow Editor

- [ ] Drag-and-drop node canvas (React Flow or custom)
- [ ] Node palette with all node types
- [ ] Property editor per node
- [ ] Edge connections between nodes
- [ ] JSON ↔ visual round-trip (edit in either mode)

---

## Parallel Track: ADO Dashboard Use Case

Real-world dogfooding effort. Runs parallel to any phase — no blocking dependencies. See [ADO_DASHBOARD_USE_CASE.md](strategy/ADO_DASHBOARD_USE_CASE.md).

### Phase A: `ado` Node Type

- [ ] Implement `ado` node (WIQL queries, work items, test runs, build status)
- [ ] Unit tests with mock HTTP server
- [ ] Integration test with fixture .bnto.json
- [ ] Register in DefaultRegistry()

### Phase B: `aggregate` Node Type

- [ ] Implement `aggregate` node (groupBy, count, sum, average, percentage, sortBy)
- [ ] Unit tests with diverse sample data
- [ ] Integration test chaining with other node types

### Phase C: `report` Node Type

- [ ] Implement `report` node (terminal, markdown, json output formats)
- [ ] Unit tests for each output format
- [ ] Verify nested template resolution

### Phase D: Dashboard Templates

- [ ] Create 3-5 example dashboard .bnto.json workflows
- [ ] Add to engine/examples/ as showcase templates
- [ ] Documentation for customization

---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/strategy/CLOUD_DESKTOP_STRATEGY.md` | Architecture, technology decisions, execution model |
| `.claude/strategy/MONOREPO_STRUCTURE.md` | Repo structure, API abstractions, package design |
| `.claude/strategy/ADO_DASHBOARD_USE_CASE.md` | ADO dashboard use case, new node types |
| `.claude/decisions/MONOREPO_TOOLING.md` | Taskfile + Turborepo decision |
| `.claude/BENTO_BOX_PRINCIPLE.md` | Core code philosophy |
| `.claude/rules/` | Coding standards and conventions |
| `.claude/skills/` | Agent skills (pre-commit, pickup, code-review) |
