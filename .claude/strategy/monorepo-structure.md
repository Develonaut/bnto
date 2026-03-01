# Bnto Monorepo Structure

**Date:** 2025-12-15
**Updated:** 2026-03-01
**Status:** Implemented
**See also:** [CLAUDE.md](../CLAUDE.md) (repo structure, commands)

> **March 2026 Trim:** Directory tree and build commands moved to CLAUDE.md. This file retains tooling decisions, dependency graph, Go workspace config, and API service layer details.

---

## Tooling Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Root orchestrator | **Taskfile.dev** | Polyglot, Go-native, cross-cutting tasks |
| Frontend orchestrator | **Turborepo** | Caching, dependency graph, standard layout |
| Package manager | **pnpm workspaces** | Fast, efficient, workspace linking |
| Package namespace | **`@bnto/`** directory (n8n pattern) | Visual grouping of internal packages. `core/` at packages root for public API |
| Go workspace | **`go.work` at repo root** | Connects `archive/engine-go/` and `archive/api-go/` modules locally |

---

## Package Dependency Graph

```
@bnto/web ──────┬──→ @bnto/auth ──→ @bnto/backend (cloud auth only)
                └──→ @bnto/core
@bnto/desktop ──────→ @bnto/core   (desktop skips @bnto/auth)
```

| Package | Dependencies | Purpose |
|---------|-------------|---------|
| `@bnto/core` | zustand, @tanstack/react-query, @convex-dev/react-query, convex | Hooks, types, Zustand stores, React Query + transport adapters |
| `@bnto/auth` | `@convex-dev/auth`, `@bnto/backend` | Cloud auth — provider, hooks (web only) |
| `@bnto/backend` | `convex` | Convex schema, functions, business logic |
| `@bnto/web` | `@bnto/auth`, `@bnto/core` | Next.js cloud app (UI + editor co-located) |
| `@bnto/desktop` | `@bnto/core` | Tauri local desktop app (M3 — no @bnto/auth) |

> **Co-location note:** UI components and editor features are co-located in `apps/web/`. When the desktop app creates a second consumer, extract `@bnto/ui` (design system) and `@bnto/editor` (workflow editor) as shared packages.

---

## Go Workspace (`go.work`)

The `go.work` file at repo root connects the two archived Go modules so they can import each other locally without published versions:

- `archive/engine-go/` — `module github.com/Develonaut/bnto`
- `archive/api-go/` — `module github.com/Develonaut/bnto-api`

`archive/api-go/` imports engine packages from `archive/engine-go/pkg/`. Uses `pkg/` (not `internal/`) so the API module can access them via `go.work`.

---

## Go API Service Layer (Archived)

`archive/engine-go/pkg/api/` provides a shared service layer consumed by CLI, HTTP server, and (future) desktop.

| CLI Command | BntoService Method | Input | Output |
|---|---|---|---|
| `bnto run <file>` | `RunWorkflow(ctx, def, opts)` | `*node.Definition`, `RunOptions` | `*RunResult` |
| `bnto run --dry-run` | `DryRunWorkflow(ctx, def)` | `*node.Definition` | `*DryRunResult` |
| `bnto validate <file>` | `ValidateWorkflow(ctx, def)` | `*node.Definition` | `*ValidationResult` |
| `bnto list` | `ListWorkflows(ctx)` | — | `[]WorkflowSummary` |

Key decisions:
- BntoService accepts `*node.Definition` — file path resolution stays in CLI
- `RunOptions` includes timeout, progress callback, and logger
- `DefaultRegistry()` consolidates all 10 node type registrations in one place

---

## References

- [Turborepo: Structuring a Repository](https://turborepo.dev/docs/crafting-your-repository/structuring-a-repository)
- [n8n Monorepo](https://github.com/n8n-io/n8n/tree/master/packages) — `@n8n/` namespace pattern
- [Go Workspaces](https://go.dev/doc/tutorial/workspaces)
