# Architecture Rules

## Layered Architecture

```
Apps (web/desktop) -> @bnto/editor -> @bnto/ui -> @bnto/core -> Go Engine
```

Each layer only depends on layers below it. Never skip layers.

**The key insight:** `@bnto/core` is the transport-agnostic API layer. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Wails bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**Package naming convention:** Internal packages are named by **role**, not by technology. This ensures any technology can be swapped by rewriting the package internals without changing consumers.

| Package | Role | Current Implementation |
|---|---|---|
| `@bnto/backend` | Data layer -- schema, functions, business logic | Convex |
| `@bnto/auth` | Auth client -- sign in, sign up, session | Better Auth |
| `@bnto/core` | Transport-agnostic API -- hooks, types, adapters | React Query + adapters |
| `@bnto/ui` | Design system -- shadcn wrappers, primitives | Tailwind + shadcn/ui |
| `@bnto/editor` | Workflow editor -- JSON editor (Phase 1), visual editor (Phase 4) | Monaco/CodeMirror |

**State management:** Zustand handles client-only state (editor content, UI preferences). React Query handles all server state (data fetching, caching, mutations). For the Convex path, `@convex-dev/react-query` preserves real-time subscriptions through React Query's interface.

**Desktop shares the web frontend:** Wails v2 renders the same React app in a system webview. `@bnto/core` detects the runtime (browser vs Wails) and swaps the transport adapter internally -- no separate frontend for desktop.

## API Abstraction

**UI code NEVER calls backend or storage APIs directly.** Always go through `@bnto/core` hooks.

```typescript
// CORRECT -- use @bnto/core hooks
import { useWorkflows, useExecution, useRunWorkflow } from "@bnto/core";
const workflows = useWorkflows();
const { mutate: run } = useRunWorkflow();

// WRONG -- direct Convex calls in components
const workflows = useQuery(api.workflows.list);

// WRONG -- direct Wails calls in components
const workflows = window.go.main.App.ListWorkflows();
```

## Cost-First Architecture

**The user's browser is a powerful computer. Use it.**

- Client-side processing where possible (file validation, preview generation)
- No always-on compute services. Backend and hosting are serverless/on-demand
- Every architectural decision should be tested against: "Does this cost $0? If not, can we make it cost $0?"

## Package Responsibilities

### `packages/core/` (`@bnto/core`) -- Transport-agnostic API layer
- React hooks for all data operations (workflows, executions, logs)
- TypeScript types and interfaces shared across the app
- Zustand stores for client-only state (editor content, UI preferences)
- React Query configuration + transport adapters (Convex for web, Wails for desktop)
- Runtime detection to swap adapters transparently
- NO backend or storage technology imports in public API -- only in internal adapters

### `packages/ui/` (`@bnto/ui`) -- Design system
- **`primitives/`** -- Raw shadcn/ui component drops. Never publicly exported. Internal only.
- **`components/`** -- Bnto wrapper components. These are the public API.
- Tailwind v4 with `@theme inline` for design tokens
- `motion` for React animation primitives
- Presentational ONLY -- no data fetching, no business logic

### `packages/editor/` (`@bnto/editor`) -- Workflow editor
- JSON editor for `.bnto.json` files (Phase 1)
- Visual node editor (Phase 4)
- Consumes `@bnto/ui` for primitives and `@bnto/core` for data

### `packages/@bnto/backend/` (`@bnto/backend`) -- Data layer
- Schema definition (tables, indexes, validators)
- Server functions (queries, mutations, actions)
- Business rules and validation logic
- **Currently:** Convex. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `packages/@bnto/auth/` (`@bnto/auth`) -- Auth client
- Sign in, sign up, sign out, session management
- OAuth provider configuration
- **Currently:** Better Auth. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `apps/web/` -- Next.js application (Vercel)
- Landing page (public, static/SSG routes)
- Authenticated app routes (dashboard, workflows, executions)
- Page composition -- imports from `@bnto/ui` and `@bnto/core`
- Minimal custom logic -- this is a thin composition layer

### `apps/desktop/` -- Wails v2 application
- Same React frontend rendered in system webview
- `@bnto/core` detects Wails runtime and swaps transport adapter
- No separate frontend code -- shares everything with web

### `apps/api/` -- Go HTTP API server (Railway)
- HTTP API that wraps the Go engine for cloud execution
- Receives workflow execution requests from Convex actions
- Returns execution results

### `engine/` -- Go engine (CLI + execution)
- CLI binary (`cmd/bnto/`)
- Workflow execution engine (`pkg/engine/`)
- Node type implementations (`pkg/node/`)
- Workflow validation (`pkg/validator/`)
- Node registry (`pkg/registry/`)
- Path resolution and config (`pkg/paths/`)

## Data Flow

```
+--------------------------------------------------------------+
|                    Apps (same React code)                      |
|  +--------------+  +--------------+  +--------------+         |
|  |  Next.js Web |  | Wails Desktop|  |   CLI        |         |
|  |  (Vercel)    |  | (webview)    |  |   (Terminal) |         |
|  +------+-------+  +------+-------+  +------+-------+         |
|         |                  |                  |                |
|         +--------+---------+                  |                |
|                  v                            |                |
|  +---------------------------------------+    |                |
|  |         @bnto/core                    |    |                |
|  |  +-------------+ +-------------+     |    |                |
|  |  |   Zustand    | | React Query |     |    |                |
|  |  |(client state)| |(server state)|    |    |                |
|  |  +-------------+ +------+------+     |    |                |
|  |          +---------------+            |    |                |
|  |          v               v            |    |                |
|  |  +------------+  +------------+       |    |                |
|  |  |  Convex    |  |   Wails    |       |    |                |
|  |  |  adapter   |  |   adapter  |       |    |                |
|  |  +-----+------+  +-----+------+       |    |                |
|  +--------+---------------+----------+    |                |
|           v               v               v                |
|    +----------+    +----------+    +----------+             |
|    | Convex   |    | Go Engine|    | Go Engine|             |
|    | (cloud)  |    | (local)  |    |  (CLI)   |             |
|    +----------+    +----------+    +----------+             |
+--------------------------------------------------------------+
```

## Run Quota Infrastructure

Every execution must:
- Increment `runsUsedThisMonth` in the user's Convex record
- Record `slug`, `timestamp`, and `durationMs`
- Be associated with an authenticated user OR an anonymous browser fingerprint

| Sprint | What gets built |
|--------|----------------|
| Sprint 2 | Execution events logged (userId or fingerprint, slug, timestamp, durationMs) |
| Sprint 3 | `runsUsedThisMonth`, `runResetDate`, `totalRunsAllTime` per user. Dashboard shows usage. |
| Sprint 6 | Stripe integration. Quota enforced server-side. |

File size limits are enforced at the **R2 presigned URL generation step in Convex -- not client-side**. For tier limits, see Notion (`SEO & Monetization Strategy`).

## R2 Storage: Transit Layer Only

R2 is a transit layer, not a storage product. Files exist for minutes.

- Upload → process → download → delete
- Objects deleted immediately after download, or 1-hour TTL
- Storage stays near zero at all times
- Never repurpose R2 as long-term storage without an explicit product decision

---

## Content Model: Workflows and Executions

**Workflows are the atomic unit of content.** Users define workflows as `.bnto.json` files that orchestrate tasks.

```
Workflow (atomic unit)
  |-- name, description, version
  |-- nodes[] (task definitions)
  |     +-- Node
  |           |-- type, id, config
  |           +-- connections (inputs/outputs)
  +-- executions (queried via by_workflowId index)
        +-- Execution
              |-- status, startedAt, completedAt
              +-- ExecutionLog[] (per-node results)
```

### Key Principles

- **Workflow-first:** A workflow defines what to do. Executions track runs of that workflow.
- **Nodes are typed:** Each node has a type (image, file, http, transform, etc.) registered in the Go engine's registry.
- **Execution logs are per-node:** Each node in an execution produces its own log entry with status, output, and timing.
- **CLI is the stable API:** Every operation maps to a CLI command. The Go engine is the source of truth for execution behavior.
