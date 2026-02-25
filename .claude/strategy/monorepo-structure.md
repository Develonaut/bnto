# Bnto Monorepo Structure
**Go Backend + React Frontend with Transport-Agnostic API Layer**

**Date:** 2025-12-15
**Updated:** 2026-02-22
**Status:** Implemented
**See also:** [Cloud + Desktop Strategy](cloud-desktop-strategy.md), [Monorepo Tooling Decision](../decisions/monorepo-tooling.md)

---

## Structure

Standard Turborepo layout with Go engine alongside TypeScript packages:

```
bnto/
├── package.json                     # Turborepo root workspace
├── pnpm-workspace.yaml              # pnpm workspace config
├── turbo.json                       # Turborepo task config
├── pnpm-lock.yaml
├── Taskfile.yml                     # Go + cross-cutting orchestration
├── go.work                          # Go workspace (archive/engine-go + archive/api-go)
│
├── apps/
│   ├── web/                         # @bnto/web — Next.js on Vercel
│   └── desktop/                     # @bnto/desktop — Tauri frontend (M3)
│
├── packages/
│   ├── core/                        # @bnto/core — Transport-agnostic API layer
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── src/
│   │       └── index.ts             # BntoAPI interface + types
│   └── @bnto/                       # Scoped internal packages (n8n pattern)
│       ├── auth/                    # @bnto/auth — Cloud auth (wraps Better Auth)
│       │   ├── package.json
│       │   ├── tsconfig.json
│       │   └── src/
│       │       ├── index.ts        # Provider + hooks (client)
│       │       └── middleware.ts   # Next.js middleware helpers (server)
│       └── backend/                # @bnto/backend — Convex functions (cloud backend)
│           ├── package.json
│           ├── tsconfig.json
│           ├── .env.local           # Symlink → root .env.local
│           └── convex/
│               ├── schema.ts        # Tables + authTables + indexes
│               ├── auth.ts          # Password provider config
│               ├── auth.config.ts   # Auth provider config
│               ├── http.ts          # Auth HTTP routes
│               ├── users.ts         # User queries + mutations
│               ├── workflows.ts     # Workflow CRUD
│               ├── executions.ts    # Execution tracking + Go API proxy
│               ├── executionLogs.ts # Log queries + mutations
│               └── crons.ts         # Monthly run counter reset
│
├── engine/                          # Rust WASM engine (primary — browser execution)
│   ├── Cargo.toml                   # Cargo workspace
│   └── crates/
│       ├── bnto-core/               # Core types, traits, progress
│       ├── bnto-image/              # Image compress/resize/convert
│       ├── bnto-csv/                # CSV clean/rename columns
│       ├── bnto-file/               # File rename
│       └── bnto-wasm/               # cdylib entry point (single WASM binary)
│
├── archive/                         # Preserved reference code (not actively developed)
│   ├── engine-go/                   # Go CLI + engine (~33K LOC)
│   │   ├── go.mod                   # module github.com/Develonaut/bnto
│   │   ├── cmd/bnto/               # CLI binary
│   │   ├── pkg/                     # Engine packages (api, engine, node, validator, etc.)
│   │   └── tests/                   # Integration tests + fixtures
│   └── api-go/                      # Go HTTP API server (~2.5K LOC)
│       ├── go.mod                   # module github.com/Develonaut/bnto-api
│       └── cmd/server/              # Server binary
│
└── .claude/                         # Project strategy and decisions
    ├── PLAN.md
    ├── rules/                           # Coding standards (Bento Box Principle, etc.)
    ├── strategy/
    └── decisions/
```

---

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Root orchestrator | **Taskfile.dev** | Polyglot, Go-native, Wails-aligned |
| Frontend orchestrator | **Turborepo** | Caching, dependency graph, standard layout |
| Package manager | **pnpm workspaces** | Fast, efficient, workspace linking |
| Package namespace | **`@bnto/`** directory (n8n pattern) | Visual grouping of internal packages. `core/` at packages root for public API |
| UI co-location | **`apps/web/`** | UI + editor co-located until desktop app creates a second consumer |
| Go module path | **`github.com/Develonaut/bnto`** | Unchanged — Go resolves relative to go.mod |
| Go workspace | **`go.work` at repo root** | Connects archive/engine-go/ and archive/api-go/ modules locally |
| API server location | **`archive/api-go/`** | Archived — ready for M4 premium server-side bntos |
| API abstraction | **`@bnto/core` with provider pattern** | Same UI code, different backends |

---

## Package Dependency Graph

```
@bnto/web ──────┬──→ @bnto/auth ──→ @bnto/backend (cloud auth only)
                  └──→ @bnto/core
@bnto/desktop ──────→ @bnto/core   (desktop skips @bnto/auth)
```

> **Co-location note:** UI components and editor features are currently co-located in `apps/web/`. When the desktop app creates a second consumer, extract `@bnto/ui` (design system) and `@bnto/editor` (workflow editor) as shared packages.

| Package | Dependencies | Purpose |
|---------|-------------|---------|
| `@bnto/core` | zustand, @tanstack/react-query, @convex-dev/react-query, convex | Hooks, types, Zustand stores, React Query + transport adapters |
| `@bnto/auth` | `better-auth`, `@better-auth/convex`, `@bnto/backend` | Cloud auth — Better Auth provider, hooks, middleware (web only) |
| `@bnto/backend` | `convex` | Convex schema, functions, business logic |
| `@bnto/web` | `@bnto/auth`, `@bnto/core` | Next.js cloud app (UI + editor co-located here) |
| `@bnto/desktop` | `@bnto/core` | Wails v2 local desktop app (Phase 2 — no @bnto/auth) |

---

## Build Commands

```bash
# Go engine
task build           # Build CLI binary
task test            # Run Go tests with race detector
task vet             # Run go vet

# Frontend (Turborepo)
task ui:build        # Build all TS packages (cached)
task ui:test         # Test all TS packages
task ui:dev          # Start web dev server

# Everything
task build:all       # Build engine + frontend
task test:all        # Test engine + frontend
```

---

## API Abstraction Layer

The core architectural pattern — `@bnto/core` provides transport-agnostic React hooks backed by React Query and Zustand:

**State separation:**
- **Zustand** — client-only state (editor content, selected workflow, UI preferences)
- **React Query** — server state (data fetching, caching, mutations, real-time)

**Transport adapters:**
- **Convex adapter** — web: React Query + `@convex-dev/react-query` (preserves real-time subscriptions). File transit via R2 (cloud-only)
- **Wails adapter** — desktop: React Query + Go engine (in-process bindings for MVP, subprocess to `bnto` CLI long-term). Files accessed directly from local filesystem — no R2

`@bnto/core` detects the runtime environment and swaps adapters internally. Desktop (Wails v2) renders the same React frontend in a system webview — there is no separate desktop frontend.

```typescript
// @bnto/core — components use these hooks (any platform)
import { useWorkflows, useExecution, useRunWorkflow } from "@bnto/core";

const workflows = useWorkflows();           // React Query under the hood
const execution = useExecution(id);         // real-time via Convex or polling via Wails
const { mutate: run } = useRunWorkflow();   // mutation via appropriate adapter

// Under the hood, @bnto/core detects the environment:
// Web:     React Query + @convex-dev/react-query adapter → Convex → R2 → Railway
// Desktop: React Query + Wails adapter → Go engine (local) → filesystem (no R2)
```

```typescript
// Zustand stores for client-only state (not server data)
import { useEditorStore, useUIStore } from "@bnto/core";

const { content, setContent } = useEditorStore();
const { theme, toggleTheme } = useUIStore();
```

---

## Go API Service Layer

`archive/engine-go/pkg/api/` provides a shared service layer consumed by CLI, HTTP server, and Wails.
Uses `pkg/` (not `internal/`) so `archive/api-go/` (a separate Go module via `go.work`) can import it.

### CLI → API Mapping

| CLI Command | BntoService Method | Input | Output |
|---|---|---|---|
| `bnto run <file>` | `RunWorkflow(ctx, def, opts)` | `*node.Definition`, `RunOptions` | `*RunResult` |
| `bnto run --dry-run` | `DryRunWorkflow(ctx, def)` | `*node.Definition` | `*DryRunResult` |
| `bnto validate <file>` | `ValidateWorkflow(ctx, def)` | `*node.Definition` | `*ValidationResult` |
| `bnto list` | `ListWorkflows(ctx)` | — | `[]WorkflowSummary` |
| load from storage | `GetWorkflow(ctx, name)` | name string | `*node.Definition` |
| save to storage | `SaveWorkflow(ctx, name, def)` | name, `*node.Definition` | error |
| delete from storage | `DeleteWorkflow(ctx, name)` | name string | error |

**Key decisions:**
- BntoService accepts `*node.Definition` — file path resolution stays in CLI
- `RunOptions` includes timeout, progress callback, and logger
- `DefaultRegistry()` consolidates all 10 node type registrations in one place
- Directory listing (`bnto list <dir>`) stays CLI-only — the API uses storage

```go
// engine/pkg/api/ — shared logic, no transport concerns
svc := api.New(api.DefaultRegistry(), store)

result, err := svc.RunWorkflow(ctx, def, api.RunOptions{
    Timeout:    30 * time.Second,
    OnProgress: func(nodeID, status string) { /* ... */ },
})
```

The HTTP transport layer lives in `archive/api-go/` (separate Go module, linked via `go.work`):

```go
// archive/api-go/ — HTTP handlers wrapping BntoService
// Thin layer: routing, request parsing, response serialization
```

---

## Infrastructure & Deployment

### What Runs Where

| Component | Code Location | Production | Development |
|---|---|---|---|
| Web app (Next.js) | `apps/web/` | Vercel | localhost:4000 |
| Database + real-time | `packages/@bnto/backend/` | Convex Cloud (gregarious-donkey-712) | Convex Cloud (zealous-canary-422) |
| Go API server | `archive/api-go/` | Railway (bnto-production.up.railway.app) | localhost:8080 |
| Go engine | `archive/engine-go/` | Consumed by `archive/api-go/` via `go.work` | Same |
| File transit | -- | Cloudflare R2 (bnto-transit) | Cloudflare R2 (bnto-transit-dev) |
| Auth | `packages/@bnto/auth/` | Better Auth via Convex HTTP routes | Same (Convex dev deployment) |

### Local Development: `task dev:all`

One command starts everything needed for local development:

```bash
task dev:all
# Starts in parallel:
#   task dev         -> Next.js (port 4000) + Convex dev watcher
#   task api:dev     -> Go API server (port 8080)
#   task api:tunnel  -> Cloudflare Named Tunnel (bnto-dev)
```

### Why the Cloudflare Tunnel?

Convex dev is cloud-hosted -- even the "dev" deployment runs on Convex's infrastructure, not on your machine. When a Convex action needs to call the Go API (to trigger workflow execution), it can't reach `localhost:8080` because the Convex action is running remotely.

The Cloudflare Named Tunnel (`bnto-dev`) solves this by exposing `localhost:8080` at `https://api-dev.bnto.io`. Convex dev functions use `GO_API_URL=https://api-dev.bnto.io` to reach the local Go API through the tunnel. In production, the same code uses `GO_API_URL=https://bnto-production.up.railway.app` to reach Railway.

This means dev mirrors prod exactly -- same Convex functions, same R2 upload/download flow, same Go API contract. The only difference is environment variables.

---

## References

- [Turborepo: Structuring a Repository](https://turborepo.dev/docs/crafting-your-repository/structuring-a-repository)
- [n8n Monorepo](https://github.com/n8n-io/n8n/tree/master/packages) — `@n8n/` namespace pattern
- [Wails v2 Documentation](https://wails.io/docs/introduction)
- [Go Workspaces](https://go.dev/doc/tutorial/workspaces)
