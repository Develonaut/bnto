# Bento Monorepo Structure
**Go Backend + React Frontend with Transport-Agnostic API Layer**

**Date:** 2025-12-15
**Updated:** 2026-02-06
**Status:** Implemented
**See also:** [Cloud + Desktop Strategy](CLOUD_DESKTOP_STRATEGY.md), [Monorepo Tooling Decision](../decisions/MONOREPO_TOOLING.md)

---

## Structure

Standard Turborepo layout with Go engine alongside TypeScript packages:

```
bento/
├── package.json                     # Turborepo root workspace
├── pnpm-workspace.yaml              # pnpm workspace config
├── turbo.json                       # Turborepo task config
├── pnpm-lock.yaml
├── Taskfile.yml                     # Go + cross-cutting orchestration
├── go.work                          # Go workspace (engine + apps/api)
│
├── apps/
│   ├── api/                         # Go HTTP API server (Phase 1)
│   │   ├── go.mod                   # module github.com/Develonaut/bento-api
│   │   └── cmd/server/              # Server binary (thin consumer of engine)
│   ├── web/                         # @bento/web — Next.js cloud app (Phase 1)
│   └── desktop/                     # @bento/desktop — Wails frontend (Phase 3)
│
├── packages/
│   └── @bento/                      # Scoped internal packages (n8n pattern)
│       ├── core/                    # @bento/core — Transport-agnostic API layer
│       │   ├── package.json
│       │   ├── tsconfig.json
│       │   └── src/
│       │       └── index.ts         # BentoAPI interface + types
│       ├── ui/                      # @bento/ui — Design system (shadcn wrappers)
│       │   ├── package.json
│       │   ├── tsconfig.json
│       │   └── src/
│       │       └── index.ts
│       ├── editor/                  # @bento/editor — Workflow editor components
│       │   ├── package.json
│       │   ├── tsconfig.json
│       │   └── src/
│       │       └── index.ts
│       └── backend/                # @bento/backend — Convex functions (cloud backend)
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
│   ├── go.mod                       # module github.com/Develonaut/bento
│   ├── go.sum
│   ├── cmd/
│   │   └── bento/                   # CLI binary
│   ├── pkg/
│   │   ├── api/                     # Shared service layer (BentoService)
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
│   └── examples/                    # Example .bento.json files
│
└── .claude/                         # Project strategy and decisions
    ├── PLAN.md
    ├── BENTO_BOX_PRINCIPLE.md
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
| Package namespace | **`@bento/`** directory (n8n pattern) | Visual grouping of internal packages |
| Go module path | **`github.com/Develonaut/bento`** | Unchanged — Go resolves relative to go.mod |
| Go workspace | **`go.work` at repo root** | Connects engine/ and apps/api/ modules locally |
| API server location | **`apps/api/`** | Follows Turborepo convention; engine stays pure ([decision](../decisions/API_SERVER_LOCATION.md)) |
| API abstraction | **`@bento/core` with provider pattern** | Same UI code, different backends |

---

## Package Dependency Graph

```
@bento/web ──────┐
                  ├──→ @bento/editor ──→ @bento/ui ──→ @bento/core
@bento/desktop ──┘
```

| Package | Dependencies | Purpose |
|---------|-------------|---------|
| `@bento/core` | none | BentoAPI interface, ConvexClient, WailsClient, RestClient |
| `@bento/ui` | `@bento/core` | shadcn thin wrappers — design system |
| `@bento/editor` | `@bento/core`, `@bento/ui` | JSON editor (Phase 1), visual editor (Phase 4) |
| `@bento/web` | all packages | Next.js cloud app |
| `@bento/desktop` | all packages | Wails v2 local desktop app |

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

The core architectural pattern — `@bento/core` provides a transport-agnostic interface:

```typescript
// @bento/core — consumers call these methods
export interface BentoAPI {
  workflows: {
    run(id: string): Promise<Execution>;
    validate(definition: WorkflowDefinition): Promise<ValidationResult>;
    list(): Promise<Workflow[]>;
    get(id: string): Promise<Workflow>;
    save(workflow: Workflow): Promise<Workflow>;
  };
  executions: {
    get(id: string): Promise<Execution>;
    list(workflowId?: string): Promise<Execution[]>;
  };
}

// Each environment provides its own client:
// - ConvexClient  → web app talks to Convex cloud
// - WailsClient   → desktop app calls Go directly
// - RestClient    → future REST API consumer
```

Apps compose via React context:
```typescript
// Desktop: <BentoAPIProvider client={createWailsClient()}>
// Web:     <BentoAPIProvider client={createConvexClient(convex)}>
```

---

## Go API Service Layer

`engine/pkg/api/` provides a shared service layer consumed by CLI, HTTP server, and Wails.
Uses `pkg/` (not `internal/`) so `apps/api/` (a separate Go module via `go.work`) can import it.

### CLI → API Mapping

| CLI Command | BentoService Method | Input | Output |
|---|---|---|---|
| `bento run <file>` | `RunWorkflow(ctx, def, opts)` | `*node.Definition`, `RunOptions` | `*RunResult` |
| `bento run --dry-run` | `DryRunWorkflow(ctx, def)` | `*node.Definition` | `*DryRunResult` |
| `bento validate <file>` | `ValidateWorkflow(ctx, def)` | `*node.Definition` | `*ValidationResult` |
| `bento list` | `ListWorkflows(ctx)` | — | `[]WorkflowSummary` |
| load from storage | `GetWorkflow(ctx, name)` | name string | `*node.Definition` |
| save to storage | `SaveWorkflow(ctx, name, def)` | name, `*node.Definition` | error |
| delete from storage | `DeleteWorkflow(ctx, name)` | name string | error |

**Key decisions:**
- BentoService accepts `*node.Definition` — file path resolution stays in CLI
- `RunOptions` includes timeout, progress callback, and logger
- `DefaultRegistry()` consolidates all 10 node type registrations in one place
- Directory listing (`bento list <dir>`) stays CLI-only — the API uses storage

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
// apps/api/ — HTTP handlers wrapping BentoService
// Thin layer: routing, request parsing, response serialization
```

---

## References

- [Turborepo: Structuring a Repository](https://turborepo.dev/docs/crafting-your-repository/structuring-a-repository)
- [n8n Monorepo](https://github.com/n8n-io/n8n/tree/master/packages) — `@n8n/` namespace pattern
- [Wails v2 Documentation](https://wails.io/docs/introduction)
- [Go Workspaces](https://go.dev/doc/tutorial/workspaces)
