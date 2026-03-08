# Architecture Rules

## Layered Architecture

```
Apps (web/desktop) -> @bnto/core -> Engine (Rust WASM / Go)
```

Each layer only depends on layers below it. Never skip layers.

> **Co-location note:** UI components and editor features are currently co-located in `apps/web/`. They will be extracted into `@bnto/ui` and `@bnto/editor` packages when the desktop app creates a real second consumer. Engine, core API, and data layer logic stays in `@bnto/core`.

**The key insight:** `@bnto/core` is the transport-agnostic API layer. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Tauri bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**Package naming convention:** Internal packages are named by **role**, not by technology. This ensures any technology can be swapped by rewriting the package internals without changing consumers.

| Package         | Role                                             | Current Implementation |
| --------------- | ------------------------------------------------ | ---------------------- |
| `@bnto/backend` | Data layer -- schema, functions, business logic  | Convex                 |
| `@bnto/auth`    | Auth client -- sign in, sign up, session         | `@convex-dev/auth`     |
| `@bnto/core`    | Transport-agnostic API -- hooks, types, adapters | React Query + adapters |

**State management:** Zustand handles client-only state (editor content, UI preferences). Server state uses a hybrid strategy -- see [data-fetching-strategy.md](../strategy/data-fetching-strategy.md) for the full decision record:

- **Paginated lists** -> Convex native `usePaginatedQuery` (real-time per-page subscriptions)
- **Single-entity queries** -> React Query via `@convex-dev/react-query` bridge (caching, deduplication for self-fetching components)
- **External APIs** (future community recipes, marketplace) -> React Query for HTTP caching

**Desktop shares the web frontend:** Tauri renders the same React app in a system webview. `@bnto/core` detects the runtime and swaps the transport adapter internally. Desktop uses local filesystem directly (no R2). Engine runs as native Rust (same codebase as WASM, compiled for desktop).

## API Abstraction

**UI code NEVER calls backend, storage, or state management APIs directly.** Always go through `@bnto/core` hooks and methods.

This abstraction covers three boundaries:

1. **Data layer** -- no direct Convex queries/mutations in components
2. **State stores** -- no raw Zustand `.store.getState()` in consumer code. Use `core.<domain>.use*State()` hooks
3. **Infrastructure** -- no manual WASM engine registration or Web Worker setup. Core initializes lazily

```typescript
// CORRECT -- use @bnto/core hooks
const recipes = core.recipes.useRecipes();
const execState = core.executions.useExecutionState(instance);

// WRONG -- direct Convex calls
const recipes = useQuery(api.recipes.list);

// WRONG -- raw store access
const state = useStore(instance.store, useShallow(s => ({ ... })));
```

See [core-api.md](core-api.md) for the full API design rules and [core-unification.md](../strategy/core-unification.md) for the rationale.

## Cost-First Architecture

**The user's browser is a powerful computer. Use it.**

- Client-side processing where possible (file validation, preview generation)
- No always-on compute services. Backend and hosting are serverless/on-demand
- Every architectural decision should be tested against: "Does this cost $0? If not, can we make it cost $0?"

## Package Responsibilities

### `packages/core/` (`@bnto/core`) -- Transport-agnostic API layer

- React hooks for all data operations (recipes, executions, user, auth)
- TypeScript types and interfaces shared across the app
- Zustand stores for domain state (opaque to consumers -- accessed via `use*State()` hooks)
- Query layer (`queries/`) for read-path option construction with select transforms
- Service layer (`services/`) for mutations, cache invalidation, infrastructure lifecycle
- Transport adapters: Convex (web data), browser (WASM engine + Web Worker), Tauri (desktop, planned)
- Browser execution infrastructure (Web Worker, WASM engine) with lazy initialization
- Runtime detection to swap adapters transparently
- NO backend, storage, or state management technology imports in public API -- only in internal adapters/services

### `packages/@bnto/backend/` (`@bnto/backend`) -- Data layer

- Schema definition (tables, indexes, validators)
- Server functions (queries, mutations, actions)
- Business rules and validation logic
- **Currently:** Convex. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `packages/@bnto/auth/` (`@bnto/auth`) -- Auth client

- Sign in, sign up, sign out, session management
- OAuth provider configuration
- **Currently:** `@convex-dev/auth`. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `apps/web/` -- Next.js application (Vercel)

- Landing page (public, static/SSG routes)
- Authenticated app routes (dashboard, workflows, executions)
- UI components and editor features co-located here (future `@bnto/ui` + `@bnto/editor`)
- Page composition -- imports from `@bnto/core` for data, local components for UI

### `apps/desktop/` -- Tauri application (M3, planned)

- Same React frontend rendered in system webview
- `@bnto/core` detects Tauri runtime and swaps transport adapter
- Engine runs as native Rust binary (no network, no cloud)

### `archive/` -- Go engine + API (archived)

- Archived Go engine for cloud/server-side execution (M4). See `archive/engine-go/` for internals.
- Archived Go HTTP API server for Railway. See `archive/api-go/` for internals.

## Execution Model: Engine Owns the Pipeline

**The Rust engine owns pipeline execution.** The `bnto-core` crate contains the `PipelineExecutor` -- it handles graph walking, topological ordering, container node semantics (loop/group), per-file iteration, `NodeProcessor` dispatch, and structured progress events. JS / `@bnto/core` is a thin adapter: convert browser types (File to bytes, Definition to WASM struct), make a single WASM call (`run_pipeline`), and relay progress events to the UI.

This design ensures identical execution across all consumers -- browser (WASM), CLI (native), desktop (Tauri), server. See [engine-execution.md](../strategy/engine-execution.md) for the full architecture.

**Async & long-running node support:** The engine must support nodes that take 2-30+ seconds (AI API calls, large HTTP requests, complex transforms). This is a prerequisite for the `ai` node type (see [bntos.md Tier 4](../strategy/bntos.md#tier-4-ai-powered-nodes-backlog--requires-async-execution)) but also benefits `http-request` and any future external API integration.

**Requirements agents must preserve when working on execution infrastructure:**

- **Progress reporting** -- structured events (PipelineStarted, NodeStarted, FileProgress, NodeCompleted, PipelineCompleted, etc.)
- **Per-node timeouts** -- configurable per node type, overridable in node config
- **Cancellation** -- check cancellation before expensive operations, respect mid-operation abort
- **Retry/fallback** -- per-node retry config (max attempts, backoff) without per-type boilerplate
- **Streaming output** -- append intermediate output incrementally for long-running nodes

**What this does NOT mean:** Don't build any of this speculatively. These are constraints to keep in mind when designing execution infrastructure.

---

## Data Flow

### Abstraction Layer (how code sees it)

Components never know which backend they're talking to. `@bnto/core` detects the runtime and swaps adapters transparently.

```
+--------------------------------------------------------------+
|                    Apps (same React code)                      |
|  +--------------+  +--------------+  +--------------+         |
|  |  Next.js Web |  | Tauri Desktop|  |   CLI        |         |
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
|  |  |  Convex    |  |   Tauri    |       |    |                |
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

### Cloud Execution (M4)

Cloud execution topology: Browser -> R2 (upload) -> Railway Go API -> R2 (output) -> Browser (download). See `ROADMAP.md` for M4 details.

### Development

`task dev` starts Next.js (localhost:4000) + Convex dev. `task dev:all` adds the Go API + Cloudflare tunnel for cloud execution testing.

## R2 Storage: Cloud-Only Transit Layer

R2 is a **cloud-only** transit layer, not a storage product. Files exist for minutes. Desktop execution does NOT use R2 -- files stay on the user's local filesystem. Never repurpose R2 as long-term storage without an explicit product decision. `@bnto/core` handles the cloud vs local path transparently -- components never know which they're on.

See `ROADMAP.md` for R2 cleanup architecture (M4).

---

## Content Model: Recipes and Executions

**Recipes are the atomic unit of content.** Users define recipes as `.bnto.json` files that orchestrate tasks.

```
Recipe (atomic unit)
  |-- name, description, version
  |-- nodes[] (task definitions)
  |     +-- Node
  |           |-- type, id, config
  |           +-- connections (inputs/outputs)
  +-- executions (queried via by_recipeId index)
        +-- Execution
              |-- status, startedAt, completedAt
              +-- ExecutionLog[] (per-node results)
```

### Key Principles

- **Recipe-first:** A recipe defines what to do. Executions track runs of that recipe.
- **Nodes are typed:** Each node has a type (image, file, http, transform, etc.) registered in the engine.
- **Execution logs are per-node:** Each node in an execution produces its own log entry with status, output, and timing.
