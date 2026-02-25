# Archive

This directory contains code from bnto's initial Go-based architecture. These are **not actively developed** but are preserved for reference. The codebase transitioned to Rust→WASM for browser execution in M1 (Feb 2026).

## What's here

### `engine-go/`

The original Go engine — CLI binary, execution orchestration, 10 node types, TUI, validation, storage, and menu/recipe system. ~33K lines of Go.

**Key packages:**
- `cmd/bnto/` — CLI entry point (cobra-based, with bubbletea TUI)
- `pkg/engine/` — DAG execution, progress tracking, context propagation
- `pkg/node/library/` — 10 node types (image, spreadsheet, filesystem, http, shellcommand, transform, parallel, loop, group, editfields)
- `pkg/menu/` — Recipe registry (JSON-embedded, feeds `menu.json` for the web app)
- `pkg/validator/` — Workflow schema validation
- `pkg/api/` — Shared service layer (`BntoService`, `DefaultRegistry`)
- `pkg/tui/` — Terminal UI (bubbletea + bubbles + huh)

**Why it was built:** The Go engine was the original execution backend — CLI, cloud API, and planned desktop (via Wails). It has >90% test coverage and 10 fully-implemented node types.

**Why it's archived:** M1 proved that Rust→WASM works well for browser execution. The unified engine vision means Rust powers all targets (browser, desktop via Tauri, CLI, cloud). Go engine development is paused. The CLI still works if built from this directory.

### `api-go/`

The Go HTTP API server that wrapped the engine for cloud execution on Railway. ~2.5K lines of Go.

**Key components:**
- `cmd/server/` — HTTP server entry point
- `internal/handler/` — Route handlers (run, validate, execution status)
- `internal/r2/` — Cloudflare R2 file transit (upload, download, cleanup)
- `internal/execution/` — Execution state management

**Why it's archived:** The API server depends on the Go engine. Cloud execution (M4) will eventually use Rust. The R2 file transit patterns are valuable reference for the Rust rewrite.

## Can I still build this?

Yes. The Go code is self-contained:

```bash
# Build the CLI
cd archive/engine-go && go build ./cmd/bnto

# Run tests
cd archive/engine-go && go test -race ./...

# Build the API server (requires engine-go)
# Update go.work to point to archive paths first
cd archive/api-go && go build ./cmd/server
```

You'll need `go.work` at the repo root to point to `archive/engine-go` and `archive/api-go` if you want cross-module imports to resolve.

## What moved where

| Before | After | Status |
|--------|-------|--------|
| `engine/` | `archive/engine-go/` | Archived |
| `apps/api/` | `archive/api-go/` | Archived |
| `engine-wasm/` | `engine/` | **Active** — Rust WASM engine |
