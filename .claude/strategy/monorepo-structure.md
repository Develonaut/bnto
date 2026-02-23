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
├── go.work                          # Go workspace (engine + apps/api)
│
├── apps/
│   ├── api/                         # Go HTTP API server (Phase 1 — cloud execution)
│   │   ├── go.mod                   # module github.com/Develonaut/bnto-api
│   │   └── cmd/server/              # Server binary (thin consumer of engine)
│   ├── web/                         # @bnto/web — Next.js on Vercel (Phase 1)
│   └── desktop/                     # @bnto/desktop — Wails frontend (Phase 2)
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
├── engine/                          # Pure Go engine (core logic only)
│   ├── go.mod                       # module github.com/Develonaut/bnto
│   ├── go.sum
│   ├── cmd/
│   │   └── bnto/                   # CLI binary
│   ├── pkg/
│   │   ├── api/                     # Shared service layer (BntoService)
│   │   ├── engine/                  # Orchestration (executor)
│   │   ├── registry/                # Node type registry
│   │   ├── storage/                 # Persistent storage
│   │   ├── paths/                   # Path resolution
│   │   ├── validator/               # Workflow validation
│   │   ├── logger/                  # Logging
│   │   ├── secrets/                 # Secrets management
│   │   ├── logs/                    # Log management
│   │   └── node/                    # Node types (10+ node types)
│   ├── tests/
│   │   ├── integration/             # End-to-end workflow tests
│   │   ├── fixtures/                # Test fixture files
│   │   └── mocks/                   # Mock implementations
│   └── examples/                    # Example .bnto.json files
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
| Go workspace | **`go.work` at repo root** | Connects engine/ and apps/api/ modules locally |
| API server location | **`apps/api/`** | Follows Turborepo convention; engine stays pure ([decision](../decisions/API_SERVER_LOCATION.md)) |
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

`engine/pkg/api/` provides a shared service layer consumed by CLI, HTTP server, and Wails.
Uses `pkg/` (not `internal/`) so `apps/api/` (a separate Go module via `go.work`) can import it.

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

The HTTP transport layer lives in `apps/api/` (separate Go module, linked via `go.work`):

```go
// apps/api/ — HTTP handlers wrapping BntoService
// Thin layer: routing, request parsing, response serialization
```

---

## References

- [Turborepo: Structuring a Repository](https://turborepo.dev/docs/crafting-your-repository/structuring-a-repository)
- [n8n Monorepo](https://github.com/n8n-io/n8n/tree/master/packages) — `@n8n/` namespace pattern
- [Wails v2 Documentation](https://wails.io/docs/introduction)
- [Go Workspaces](https://go.dev/doc/tutorial/workspaces)
