# Decision: API Server Lives in apps/api/

**Date:** 2026-02-06
**Status:** Accepted

## Context

Phase 1.1 requires a Go HTTP server wrapping the engine for cloud execution. The question was where it should live: `engine/cmd/bento-server/` or `apps/api/`.

## Decision

The API server lives in `apps/api/` as a separate Go module, connected to `engine/` via a Go workspace (`go.work` at repo root).

## Rationale

- **`engine/` stays pure.** It's the core Go logic — orchestrator, node types, registry, validator. No transport layers.
- **The API server is a consumer**, not engine internals. It's the same relationship as the CLI (`engine/cmd/bento/`) — a thin wrapper. But unlike the CLI, it's an *app* that serves a frontend, not a developer tool.
- **Follows Turborepo convention.** Apps live in `apps/`. The web frontend is in `apps/web/`, the desktop app in `apps/desktop/`, and now the API server in `apps/api/`.
- **Go workspaces handle the cross-module imports.** `apps/api/` imports `engine/pkg/engine`, `engine/pkg/registry`, etc. The `go.work` file resolves them locally without publishing.

## Structure

```
bento/
├── go.work                  # Go workspace: engine/ + apps/api/
├── engine/                  # Pure Go engine (module: github.com/Develonaut/bento)
│   ├── pkg/                 # Core packages (engine, registry, node, etc.)
│   ├── internal/api/        # Shared BentoService interface (used by CLI, API server, Wails)
│   └── cmd/bento/           # CLI binary (thin consumer of engine)
├── apps/
│   ├── api/                 # HTTP API server (module: github.com/Develonaut/bento-api)
│   │   └── cmd/server/      # Server binary (thin consumer of engine)
│   ├── web/                 # Next.js frontend
│   └── desktop/             # Wails desktop app
```

## What stays in engine/

- `engine/internal/api/` — the shared `BentoService` interface that the CLI, API server, and Wails bindings all consume. This is engine-level logic (run, validate, list) abstracted from transport.

## What goes in apps/api/

- HTTP handlers, routing, middleware, request/response serialization — the transport layer that wraps `BentoService` for the web frontend.
