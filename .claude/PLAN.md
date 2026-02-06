# Bento Master Plan

**The checklist for building Bento Desktop + Bento Cloud.**

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
  - Module path unchanged (github.com/Develonaut/bento)
  - No import path changes needed (Go resolves relative to go.mod)
- [x] Set up monorepo root (standard Turborepo layout)
  - `Taskfile.yml` at root (Go orchestration via `task build`, `task test`)
  - `turbo.json`, `package.json`, `pnpm-workspace.yaml` at root
  - `packages/@bento/core/` (`@bento/core` — API layer with BentoAPI interface)
  - `packages/@bento/ui/` (`@bento/ui` — design system)
  - `packages/@bento/editor/` (`@bento/editor` — workflow editor)
  - `apps/web/` (`@bento/web` — Next.js stub)
  - `apps/desktop/` (`@bento/desktop` — Wails stub)
  - `@bento/` namespace directory (n8n pattern) for internal packages
- [x] Verify everything builds and tests pass from new structure
  - `task build` — Go engine builds
  - `task ui:build` — Turborepo builds all 5 packages in dependency order
  - `task build:all` — Full cross-cutting build verified
- [x] Update `.claude/` docs to reflect new paths
- [x] Fix integration tests (missed old sushi names in test code)

### 0.1b Codebase Cleanup
- [x] Remove interactive TUI package (`pkg/tui/` — 45+ files, ~3,500 lines)
  - Not in scope for current development phase
  - Deleted entire `pkg/tui/` directory and `cmd/bento/tui.go`
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
- [x] Integration tests using fixture .bento.json files
  - [x] Create "Resize Images" fixture (loop + image resize)
  - [x] Create "CSV Data Pipeline" fixture (spreadsheet + loop + transform)
  - [x] Create "Image Composite" fixture (filesystem list + loop + image composite)
  - [x] Create "HTTP + Transform" fixture (http-request + transform chain)
  - [x] Create "Edit Fields Pipeline" fixture (edit-fields + transform)
- [x] CLI smoke tests
  - [x] `bento run` with each fixture (validates end-to-end)
  - [x] `bento validate` with valid and invalid bentos
  - [x] `bento list` returns expected results
  - [x] `--dry-run` flag works correctly
- [x] Document the public API surface
  - CLI → API mapping table in MONOREPO_STRUCTURE.md
  - Every CLI command mapped to BentoService method with inputs/outputs

### 0.3 Quality Gates
- [x] `go test ./engine/pkg/... -race` passes (all packages)
- [x] `go build ./engine/cmd/bento` succeeds
- [x] `go vet ./engine/...` clean
- [x] All fixture bentos validate and execute correctly
- [x] Test coverage report generated and reviewed
  - Node types: 88–100%, api: 80.6%, registry: 100%, engine: 58.2%

---

## Phase 1: Cloud MVP

Ship a working web app where users can upload, edit, and run workflows.

### 1.1 Go API Server
- [x] Set up `apps/api/` as separate Go module with `go.work` workspace
- [x] Create `engine/pkg/api/` service layer
  - `BentoService` struct shared by CLI, HTTP server, and Wails
  - `DefaultRegistry()` consolidates node type registration
  - CLI refactored to use `api.DefaultRegistry()`
- [x] Create `apps/api/` HTTP server
  - HTTP handlers wrapping BentoService (stdlib `net/http` with Go 1.25 routing)
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
  - 6 contract tests verify Go JSON responses match `@bento/core` TypeScript types
  - TS types updated to match actual Go API shapes (WorkflowDefinition, Execution, etc.)

### 1.2 Convex Setup
- [ ] Deploy Convex on Railway via template
- [ ] Define Convex schema (users, workflows, executions, executionLogs, files)
- [ ] Implement Convex functions
  - Queries: list workflows, get execution status, get logs
  - Mutations: save workflow, update execution progress, insert logs
  - Actions: proxy execution requests to Go backend
- [ ] Convex Auth (email/password for MVP)
- [ ] Run counter logic (runsUsed, runLimit, monthly reset cron)

### 1.3 Frontend — @bento/core
- [ ] Implement `BentoAPI` TypeScript interface
- [ ] Implement `ConvexClient` (web → Convex)
- [ ] React context provider (`BentoAPIProvider`)
- [ ] Unit tests for core package with mock backends

### 1.4 Frontend — @bento/ui
- [ ] Initialize shadcn with Bento theme tokens
- [ ] Light/dark mode from day one
- [ ] Core components: Button, Card, Input, Dialog, Select
- [ ] Bento-specific: WorkflowCard, RunButton, StatusBadge

### 1.5 Frontend — @bento/editor
- [ ] JSON editor component (Monaco or CodeMirror)
- [ ] Schema validation for .bento.json format
- [ ] Syntax highlighting for bento-specific fields

### 1.6 Next.js Web App
- [ ] Create `ui/apps/web/` with Next.js + Convex integration
- [ ] Pages:
  - [ ] Landing / sign-up / sign-in (Convex Auth)
  - [ ] Dashboard — list saved workflows, "X runs remaining"
  - [ ] Editor — upload .bento.json or start from template, edit, run
  - [ ] Execution — real-time progress via Convex subscription
  - [ ] Results — output download, execution logs
- [ ] Pre-built templates (the fixture bentos from Phase 0)
  - "Resize Images", "CSV Data Pipeline", "Image Composite"
- [ ] Playwright E2E tests
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
- [ ] Create `engine/cmd/bento-desktop/`
- [ ] Wails v2 project with Vite + React
- [ ] Go ↔ React bindings via Wails

### 3.2 Desktop Integration
- [ ] `WailsClient` implements `BentoAPI` in `@bento/core`
- [ ] Reuse `@bento/ui` and `@bento/editor` from web
- [ ] Full local execution (all node types including shell-command)
- [ ] Local file browser for selecting .bento.json files

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

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/strategy/CLOUD_DESKTOP_STRATEGY.md` | Architecture, technology decisions, execution model |
| `.claude/strategy/MONOREPO_STRUCTURE.md` | Repo structure, API abstractions, package design |
| `.claude/decisions/MONOREPO_TOOLING.md` | Taskfile + Turborepo decision |
| `.claude/BENTO_BOX_PRINCIPLE.md` | Core code philosophy |
