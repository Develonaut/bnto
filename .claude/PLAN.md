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

### 0.2 Engine Solidification (TDD)
- [ ] Unit tests for every neta type (target >90% coverage)
  - [ ] edit-fields
  - [ ] filesystem (including new `list` operation)
  - [ ] group
  - [ ] http-request
  - [ ] image (resize, export, composite)
  - [ ] loop (forEach, times, while)
  - [ ] parallel
  - [ ] shell-command (including retry/stall)
  - [ ] spreadsheet
  - [ ] transform
- [ ] Integration tests using fixture .bento.json files
  - [ ] Create "Resize Images" fixture (loop + image resize)
  - [ ] Create "CSV Data Pipeline" fixture (spreadsheet + loop + transform)
  - [ ] Create "Image Composite" fixture (filesystem list + loop + image composite)
  - [ ] Create "HTTP + Transform" fixture (http-request + transform chain)
  - [ ] Create "Edit Fields Pipeline" fixture (edit-fields + transform)
- [ ] CLI smoke tests
  - [ ] `bento run` with each fixture (validates end-to-end)
  - [ ] `bento validate` with valid and invalid bentos
  - [ ] `bento list` returns expected results
  - [ ] `--dry-run` flag works correctly
- [ ] Document the public API surface
  - What the CLI exposes = what the API layer will wrap
  - Every CLI command's inputs, outputs, and error cases

### 0.3 Quality Gates
- [ ] `go test ./engine/pkg/... -race` passes (all packages)
- [ ] `go build ./engine/cmd/bento` succeeds
- [ ] `go vet ./engine/...` clean
- [ ] All fixture bentos validate and execute correctly
- [ ] Test coverage report generated and reviewed

---

## Phase 1: Cloud MVP

Ship a working web app where users can upload, edit, and run workflows.

### 1.1 Go API Server
- [ ] Create `engine/cmd/bento-server/`
  - HTTP handlers wrapping engine operations
  - `/api/run` — start async execution, return execution ID
  - `/api/validate` — validate a workflow definition
  - `/api/workflows` — list/get/save workflows
  - `/api/executions/:id` — get execution status
- [ ] Create `engine/internal/api/` service layer
  - `BentoService` interface shared by CLI and HTTP server
  - Decouple from CLI-specific code
- [ ] API integration tests
  - Every endpoint tested (happy path + error cases)
  - Use `net/http/httptest` for in-process testing
- [ ] Contract tests
  - API responses match TypeScript types in `@bento/core`

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
- [ ] Cloud filesystem neta (operates on uploaded files)
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
- [ ] Auto-complete for neta type names
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
- [ ] Full local execution (all neta types including shell-command)
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
- [ ] Node palette with all neta types
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
