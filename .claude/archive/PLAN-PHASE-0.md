# Bnto Master Plan

**The checklist for building Bnto Desktop + Bnto Cloud.**

Every task below feeds into the next. Work top to bottom. Don't skip ahead.

---

## Phase 0: Foundation

Solidify the engine, restructure the repo, establish the development environment.

### 0.1 Monorepo Setup
- [x] Rename Go packages from sushi names to simple names
  - itamae → engine, pantry → registry, hangiri → storage
  - miso → tui, kombu → paths, omakase → validator
  - shoyu → logger, wasabi → secrets
- [x] Move Go code to `engine/` directory
  - Moved `cmd/`, `pkg/`, `tests/`, `examples/`, `go.mod`, `go.sum`, docs
  - Module path unchanged (github.com/Develonaut/bnto)
  - No import path changes needed (Go resolves relative to go.mod)
- [x] Set up monorepo root (standard Turborepo layout)
  - `Taskfile.yml` at root (Go orchestration via `task build`, `task test`)
  - `turbo.json`, `package.json`, `pnpm-workspace.yaml` at root
  - `packages/@bnto/core/` (`@bnto/core` — API layer with BntoAPI interface)
  - `packages/@bnto/ui/` (`@bnto/ui` — design system)
  - `packages/@bnto/editor/` (`@bnto/editor` — workflow editor)
  - `apps/web/` (`@bnto/web` — Next.js stub)
  - `apps/desktop/` (`@bnto/desktop` — Wails stub)
  - `@bnto/` namespace directory (n8n pattern) for internal packages
- [x] Verify everything builds and tests pass from new structure
  - `task build` — Go engine builds
  - `task ui:build` — Turborepo builds all 5 packages in dependency order
  - `task build:all` — Full cross-cutting build verified
- [x] Update `.claude/` docs to reflect new paths
- [x] Fix integration tests (missed old sushi names in test code)

### 0.1b Codebase Cleanup
- [x] Remove interactive TUI package (`pkg/tui/` — 45+ files, ~3,500 lines)
  - Not in scope for current development phase
  - Deleted entire `pkg/tui/` directory and `cmd/bnto/tui.go`
  - Updated all consumers to use `pkg/paths` for config loading
  - Removed `configThemeCmd`, `configSlowMoCmd` (TUI-specific)
  - Removed bubbletea, bubbles, huh, teatest dependencies
  - Simplified CLI output (plain text errors, no lipgloss in CLI)
- [x] Fix shellcommand StallDetection race condition
  - Added `sync.Mutex` to test callback closure
  - Test now passes with `-race` flag

### 0.2 Engine Solidification (TDD)
- [x] Unit tests for every node type (target >90% coverage)
  - [x] edit-fields (90.0%)
  - [x] filesystem (90.2%)
  - [x] group (100.0%)
  - [x] http-request (91.5%)
  - [x] image (90.6%)
  - [x] loop (98.6%)
  - [x] parallel (90.7%)
  - [x] shell-command (93.3%)
  - [x] spreadsheet (90.5%)
  - [x] transform (93.9%)
- [x] Integration tests using fixture .bnto.json files
  - [x] Create "Resize Images" fixture (loop + image resize)
  - [x] Create "CSV Data Pipeline" fixture (spreadsheet + loop + transform)
  - [x] Create "Image Composite" fixture (filesystem list + loop + image composite)
  - [x] Create "HTTP + Transform" fixture (http-request + transform chain)
  - [x] Create "Edit Fields Pipeline" fixture (edit-fields + transform)
- [x] CLI smoke tests
  - [x] `bnto run` with each fixture (validates end-to-end)
  - [x] `bnto validate` with valid and invalid bntos
  - [x] `bnto list` returns expected results
  - [x] `--dry-run` flag works correctly
- [x] Document the public API surface
  - CLI → API mapping table in MONOREPO_STRUCTURE.md
  - Every CLI command mapped to BntoService method with inputs/outputs

### 0.3 Quality Gates
- [x] `go test ./engine/pkg/... -race` passes (all packages)
- [x] `go build ./engine/cmd/bnto` succeeds
- [x] `go vet ./engine/...` clean
- [x] All fixture bntos validate and execute correctly
- [x] Test coverage report generated and reviewed
  - Node types: 88–100%, api: 80.6%, registry: 100%, engine: 58.2%

---

## Phase 1: Cloud MVP

Ship a working web app where users can upload, edit, and run workflows.

### 1.1 Go API Server
- [x] Set up `apps/api/` as separate Go module with `go.work` workspace
- [x] Create `engine/pkg/api/` service layer
  - `BntoService` struct shared by CLI, HTTP server, and Wails
  - `DefaultRegistry()` consolidates node type registration
  - CLI refactored to use `api.DefaultRegistry()`
- [x] Create `apps/api/` HTTP server
  - HTTP handlers wrapping BntoService (stdlib `net/http` with Go 1.25 routing)
  - `POST /api/run` — async execution with in-memory tracking, returns 202 + ID
  - `POST /api/validate` — validate a workflow definition
  - `GET/POST/DELETE /api/workflows` — list/get/save/delete workflows
  - `GET /api/executions/{id}` — poll execution status + progress
  - CORS middleware, graceful shutdown, background cleanup of expired executions
- [x] API integration tests
  - 12 handler tests + 8 execution manager tests (20 total, all pass with -race)
  - `net/http/httptest` for in-process testing
  - Tests cover: validate, CRUD workflows, async run+poll, CORS, error cases
- [x] Contract tests
  - 6 contract tests verify Go JSON responses match `@bnto/core` TypeScript types
  - TS types updated to match actual Go API shapes (WorkflowDefinition, Execution, etc.)

### 1.2 Convex Setup
- [x] Deploy Convex on Railway via template
- [x] Define Convex schema (users, workflows, executions, executionLogs)
- [x] Implement Convex functions
  - Queries: list workflows, get execution status, get logs
  - Mutations: save workflow, update execution progress, insert logs
  - Actions: proxy execution requests to Go backend (poll-based)
- [x] Convex Auth (email/password via @convex-dev/auth Password provider)
- [x] Run counter logic (runsUsed, runLimit, monthly reset cron)

### 1.2b Deploy Guarded Web App Shell
- [x] Set up `apps/web/` as a real Next.js app with Convex client provider
- [x] Coming soon splash page (public — shown to unauthenticated visitors)
- [x] Sign-in page (Convex Auth email/password)
- [x] Whitelist gate — only approved users see the app behind the splash
- [x] App shell layout (authenticated + whitelisted users only)
- [x] Passphrase gate on splash page
  - CLI-style input, server-side validation via `BNTO_PASSPHRASE` env var
  - Cookie remembers access (30 days), reveals Sign In / Sign Up buttons
  - `POST /api/verify-passphrase` API route
- [x] Global nav bar with theme toggle
  - Fixed top bar: "Bnto" left, ThemeToggle right (in root layout)
  - Removed per-page theme toggles (dashboard, etc.)
- [x] Deploy to Railway — splash screen live at public URL
  - `Dockerfile.web`: multi-stage pnpm monorepo build, Next.js standalone output
  - `railway.toml`: Dockerfile path + health check config
  - Service: `bnto-web` on Railway (`bnto-web-production.up.railway.app`)
  - Convex `_generated/` types checked into git (needed for CI/CD builds)
- [x] Verify: unauthenticated → splash, authenticated + not whitelisted → waitlist message, whitelisted → app shell
  - Playwright E2E tests cover splash gate, auth redirect, sign-in form
  - Screenshots committed for visual regression tracking

### 1.3 Frontend — @bnto/core
- [N/A] ~~Set up Zustand for client state~~ (deferred — no client state needs until @bnto/editor Phase 1.5)
- [x] Set up React Query with `@convex-dev/react-query` adapter
  - `BntoCoreProvider` creates `ConvexReactClient` + `ConvexQueryClient` + `QueryClientProvider`
  - Also wraps `ConvexAuthNextjsProvider` so `useConvex()`/`useConvexAuth()` work app-wide
  - Source-based exports (no `dist/` build step — Next.js `transpilePackages` compiles from source)
- [x] Runtime detection (browser vs Wails webview) for transport switching
  - `runtime.ts`: `isWailsEnvironment()` checks for `window.go` (Wails v2 injection)
  - `adapters/index.ts` barrel selects Convex or Wails adapter at module load time
  - Wails adapter stubs throw "not implemented (Phase 3)"
- [x] Implement hooks: `useWorkflows`, `useWorkflow`, `useExecution`, `useRunsRemaining`
  - Plus: `useExecutions`, `useExecutionLogs`, `useCurrentUser`
  - Each hook is a thin wrapper around `useQuery` + adapter query options
- [x] Implement mutations: `useRunWorkflow`, `useSaveWorkflow`, `useRemoveWorkflow`
  - Each mutation wraps `useMutation` + adapter mutation factory (`useConvexMutation`)
- [N/A] ~~Unit tests for core package with mock adapters~~ (hooks are thin wrappers — `tsc --noEmit` validates; tests come when logic is added)

### 1.4 Frontend — @bnto/ui
- [x] Initialize shadcn with Bnto theme tokens
  - `components.json`, `globals.css` with CSS variables for light/dark themes
  - Primitives layer (shadcn raw) + shared layer (thin wrappers)
- [x] Light/dark mode from day one
  - ThemeProvider (next-themes), ThemeToggle component, global nav integration
- [x] Core primitives: Button, Card, Input, Label
- [x] Additional primitives: Dialog, Select, Tabs, Toaster
- [ ] Bnto-specific: WorkflowCard, RunButton, StatusBadge

### 1.5 Frontend — @bnto/editor
- [ ] JSON editor component (Monaco or CodeMirror)
- [ ] Schema validation for .bnto.json format
- [ ] Syntax highlighting for bnto-specific fields

### 1.6 Next.js Web App
- [ ] Create `ui/apps/web/` with Next.js + Convex integration
- [ ] Pages:
  - [ ] Landing / sign-up / sign-in (Convex Auth)
  - [ ] Dashboard — list saved workflows, "X runs remaining"
  - [ ] Editor — upload .bnto.json or start from template, edit, run
  - [ ] Execution — real-time progress via Convex subscription
  - [ ] Results — output download, execution logs
- [ ] Pre-built templates (the fixture bntos from Phase 0)
  - "Resize Images", "CSV Data Pipeline", "Image Composite"
- [x] Playwright E2E test infrastructure
  - Chromium-only, port 3100, `task e2e` command, screenshots committed
  - 6 tests: splash gate (4), navigation/auth (2)
- [ ] Playwright E2E tests for app features
  - [ ] Sign up flow
  - [ ] Upload workflow → edit → run → see results
  - [ ] Template selection → run → download results
  - [ ] Run counter decrements correctly

### 1.7 Deploy to Railway
- [ ] Next.js service deployed
- [ ] Go API server deployed
- [ ] Convex instance running
- [ ] Services communicating over private networking
- [ ] Staging environment functional
- [ ] Railway integration tests passing in CI

---

## Phase 2: Polish + Files + History

Make the product feel complete for individual users.

### 2.1 Cloud File Handling
- [ ] File upload UI (drag & drop)
- [ ] Convex file storage integration
- [ ] Cloud filesystem node (operates on uploaded files)
- [ ] File download after execution
- [ ] Storage limit enforcement per tier

### 2.2 Execution History
- [ ] Execution history page with filtering
- [ ] Detailed execution logs per node
- [ ] Re-run previous executions
- [ ] Execution duration and performance stats

### 2.3 Workflow Management
- [ ] Workflow versioning
- [ ] Duplicate workflow
- [ ] Delete workflow (soft delete)
- [ ] Expanded template library

### 2.4 Editor Improvements
- [ ] Auto-complete for node type names
- [ ] Inline validation with error highlighting
- [ ] Parameter documentation on hover

---

## Phase 3: Desktop App

Free local desktop app using Wails v2 + shared React components.

### 3.1 Wails Setup
- [ ] Create `engine/cmd/bnto-desktop/`
- [ ] Wails v2 project with Vite + React
- [ ] Go ↔ React bindings via Wails

### 3.2 Desktop Integration
- [ ] `WailsClient` implements `BntoAPI` in `@bnto/core`
- [ ] Reuse `@bnto/ui` and `@bnto/editor` from web
- [ ] Full local execution (all node types including shell-command)
- [ ] Local file browser for selecting .bnto.json files

### 3.3 Desktop Distribution
- [ ] macOS build (.app bundle, code signing)
- [ ] Windows build (.exe installer)
- [ ] Linux build (AppImage or .deb)
- [ ] Auto-update mechanism

---

## Phase 4: Monetization + Visual Editor

Revenue and the next-gen editing experience.

### 4.1 Payments
- [ ] Stripe integration
- [ ] Tier management (Free, Starter, Pro)
- [ ] Webhook handling for payment events
- [ ] Upgrade/downgrade flow in UI

### 4.2 Visual Workflow Editor
- [ ] Drag-and-drop node canvas (React Flow or custom)
- [ ] Node palette with all node types
- [ ] Property editor per node
- [ ] Edge connections between nodes
- [ ] JSON ↔ visual round-trip (edit in either mode)

### 4.3 Cloud Shell Commands
- [ ] Pre-approved command allowlist (ffmpeg, imagemagick)
- [ ] Install approved tools in Railway container
- [ ] Sandboxing strategy for user safety

---

## Parallel Track: ADO Dashboard Use Case

Real-world dogfooding effort that adds general-purpose node types and showcase templates. Runs parallel to Phase 1-4 — no blocking dependencies. See [ADO_DASHBOARD_USE_CASE.md](.claude/strategy/ADO_DASHBOARD_USE_CASE.md) for full context.

### Phase A: `ado` Node Type
- [ ] Implement `ado` node in `engine/pkg/node/library/ado/`
  - Operations: `wiql` (WIQL query → work item IDs), `getWorkItems` (batch fetch by IDs), `getTestRuns`, `getBuildStatus`
  - Authentication via Bnto secrets (`{{SECRETS.ADO_PAT}}`, `{{SECRETS.ADO_ORG}}`)
  - ADO REST API via Go `net/http` (no SDK dependency)
  - Handle batch pagination (200 items per request max)
- [ ] Unit tests with mock HTTP server (`httptest`)
- [ ] Integration test with fixture `.bnto.json`
- [ ] Register in `DefaultRegistry()` in `engine/pkg/api/service.go`

### Phase B: `aggregate` Node Type
- [ ] Implement `aggregate` node in `engine/pkg/node/library/aggregate/`
  - Operations: `groupBy`, `count`, `sum`, `average`, `percentage`, `sortBy`
  - General-purpose — works on any collection, not ADO-specific
  - Handle JSON number type (`float64`) correctly
- [ ] Unit tests with diverse sample data sets
- [ ] Integration test chaining `aggregate` with other node types

### Phase C: `report` Node Type
- [ ] Implement `report` node in `engine/pkg/node/library/report/`
  - Output formats: `terminal` (ANSI + Unicode charts), `markdown`, `json`
  - Chart types: bar chart (Unicode blocks), progress bar, table
  - General-purpose — works with any structured data
- [ ] Unit tests for each output format and chart type
- [ ] Verify nested template resolution in `sections` parameter

### Phase D: Dashboard Templates & Documentation
- [ ] Create 3-5 example `.bnto.json` dashboard workflows
  - Sprint progress, triage queue, test completion, build status, effort burndown
- [ ] Add to `engine/examples/` as showcase templates
- [ ] Documentation for customization and extension

### Prerequisites / Engine Gaps to Address
- [ ] **Workflow-level parameters** — Currently no mechanism for `bnto run file.bnto.json --param key=value`. Either add an `edit-fields` preamble or enhance the engine. Needed for configurable dashboard workflows.
- [ ] **YAML config loading** (optional, Phase D) — Dashboard config in YAML is a nice-to-have. Can defer — hardcode values in `.bnto.json` for now.

---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/strategy/CLOUD_DESKTOP_STRATEGY.md` | Architecture, technology decisions, execution model |
| `.claude/strategy/MONOREPO_STRUCTURE.md` | Repo structure, API abstractions, package design |
| `.claude/strategy/ADO_DASHBOARD_USE_CASE.md` | ADO dashboard use case, new node types, phasing |
| `.claude/decisions/MONOREPO_TOOLING.md` | Taskfile + Turborepo decision |
| `.claude/BENTO_BOX_PRINCIPLE.md` | Core code philosophy |
