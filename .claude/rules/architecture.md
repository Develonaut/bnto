# Architecture Rules

## Layered Architecture

```
Consumers (CLI / web / desktop) -> Engine (Rust)
Web-specific: Apps (web) -> @bnto/core -> Engine (Rust→WASM)
```

Each layer only depends on layers below it. Never skip layers.

**The CLI is the primary consumer.** It links the engine directly as a native Rust binary — no adapters, no TypeScript, no WASM boundary. The web app is a secondary consumer that accesses the engine through the `@bnto/core` adapter layer (Rust→WASM). Desktop (Tauri, future) will link the engine natively like the CLI.

> **Package extraction (March 2026):** UI components and editor features were extracted from `apps/web/` into `@bnto/ui` and `@bnto/editor` packages. Page-level components remain in `apps/web/`. Engine, core API, and data layer logic stays in `@bnto/core`.

**The web insight:** `@bnto/core` is the transport-agnostic API layer for web consumers. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Tauri bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**Package naming convention:** Internal packages are named by **role**, not by technology. This ensures any technology can be swapped by rewriting the package internals without changing consumers.

| Package          | Role                                                                          | Current Implementation |
| ---------------- | ----------------------------------------------------------------------------- | ---------------------- |
| `@bnto/backend`  | Data layer -- schema, functions, business logic                               | Convex                 |
| `@bnto/auth`     | Auth client -- sign in, sign up, session                                      | `@convex-dev/auth`     |
| `@bnto/core`     | Transport-agnostic API -- hooks, types, adapters                              | React Query + adapters |
| `@bnto/registry` | Node system facade -- re-exports all of @bnto/nodes + curation functions      | Stateless lookups      |
| `@bnto/form`     | Schema-driven forms -- auto-generates UI controls from node schemas           | React + Zod            |
| `@bnto/nodes`    | Engine-generated catalog -- types, schemas, validation (INTERNAL to registry) | Codegen + Zod          |

**State management:** Zustand handles client-only state (editor content, UI preferences). Server state uses a hybrid strategy -- see [data-fetching-strategy.md](../strategy/data-fetching-strategy.md) for the full decision record:

- **Paginated lists** -> Convex native `usePaginatedQuery` (real-time per-page subscriptions)
- **Single-entity queries** -> React Query via `@convex-dev/react-query` bridge (caching, deduplication for self-fetching components)
- **External APIs** (future community recipes, marketplace) -> React Query for HTTP caching

**Desktop (future):** Tauri renders the same React app in a system webview. Links the engine natively (like CLI). `@bnto/core` detects the runtime and swaps the transport adapter.

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

See [core-api.md](../scopes/backend/core-api.md) for the full API design rules.

## Import Boundary Rules

The dependency chain is strictly linear:

```
@bnto/editor   → @bnto/form, @bnto/core, @bnto/ui
@bnto/form     → @bnto/core, @bnto/ui    (leaf — schema-driven forms)
@bnto/core     → @bnto/registry, @bnto/auth, @bnto/backend
@bnto/registry → @bnto/nodes
@bnto/ui       → (leaf — no @bnto/* imports)
@bnto/nodes    → (leaf — engine-generated catalog)

Apps (apps/web, apps/desktop)
  → @bnto/core (runtime — hooks, state, API)
  → @bnto/form (schema-driven forms — standalone, no editor dependency)
  → @bnto/registry (build-time SSG only — where Zustand doesn't exist)
```

**`@bnto/nodes` is internal.** Only `@bnto/registry` imports from it. The registry re-exports everything consumers need (types, constants, helpers, validation). `@bnto/core` re-exports from `@bnto/registry` for convenience so the editor and apps import from `@bnto/core`.

**One rule:** "Import from `@bnto/core`." Editor, apps, and all runtime code import node system types, constants, and functions from `@bnto/core`. Build-time SSG code can import from `@bnto/registry` directly (no React context at build time).

**NEVER import from `@bnto/nodes` directly** — not in core, not in editor, not in apps. If you need something from the node system, import from `@bnto/core` (runtime) or `@bnto/registry` (SSG).

## Cost-First Architecture

**Local execution is free execution.** The CLI runs on the user's machine at zero cost to us. The browser runs WASM at zero cost to us. Only managed server execution costs money.

- Local processing first (CLI, browser WASM, desktop native)
- No always-on compute services. Backend and hosting are serverless/on-demand
- Every architectural decision should be tested against: "Does this cost $0? If not, can we make it cost $0?"

## Package Responsibilities

### `packages/core/` (`@bnto/core`) -- Transport-agnostic API layer

- 7 domains: `core.recipes`, `core.executions`, `core.user`, `core.auth`, `core.telemetry`, `core.registry`, `core.flags`
- React hooks for all data operations (recipes, executions, user, auth, registry)
- TypeScript types and interfaces shared across the app
- Zustand stores for domain state (opaque to consumers -- accessed via `use*State()` hooks)
- Registry domain (`core.registry`) -- runtime source of truth for predefined recipes + node type metadata
- Query layer (`queries/`) for read-path option construction with select transforms
- Service layer (`services/`) for mutations, cache invalidation, infrastructure lifecycle
- Transport adapters: Convex (web data), browser (WASM engine + Web Worker), Tauri (desktop, planned)
- Browser execution infrastructure (Web Worker, WASM engine) with lazy initialization
- Runtime detection to swap adapters transparently
- NO backend, storage, or state management technology imports in public API -- only in internal adapters/services

### `packages/@bnto/registry/` (`@bnto/registry`) -- Node system facade + curation

- **Public facade for the entire node system.** Re-exports all types, constants, helpers, validation, and schema introspection from `@bnto/nodes`. `@bnto/nodes` is internal -- only registry imports from it
- Curation functions: `getAllRecipes()`, `getRecipeBySlug()`, `getRecipesByCategory()`
- Node type lookups: `getAllNodeTypes()`, `getBrowserNodeTypes()`
- Category + processor lookups: `getAllCategories()`, `getAllProcessors()`
- No React, no Zustand -- purely stateless
- Depends on `@bnto/nodes` only
- Consumed by `@bnto/core` (re-exports for runtime consumers) and `apps/web` (SSG build-time)
- Future: community recipes (DB-backed), search/exploration API

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

### `engine/crates/bnto/` -- Native CLI binary (`bnto`)

- **Primary consumer.** Links `bnto-engine` directly as a Rust dependency
- Commands: `bnto run <recipe> [files...]`, `bnto list`, `bnto info <recipe>`, `bnto doctor`
- Full system access via `ProcessContext` (run commands, temp files, env vars)
- Golden tests as the primary determinism check for output correctness
- Published to crates.io (`cargo install bnto`)

### `apps/web/` -- Next.js application (Vercel)

- Landing page, docs, and predefined recipe pages for SEO
- Browser-based recipe execution (Rust→WASM, files never leave the machine)
- Recipe editor (open + export, sessionStorage only)
- Page composition -- imports from `@bnto/core` for data, `@bnto/ui` for components, `@bnto/editor` for recipe editing

### `apps/desktop/` -- Tauri application (M4, backlog)

- Same React frontend rendered in system webview
- Links engine natively (like CLI) for full system access
- `@bnto/core` detects Tauri runtime and swaps transport adapter

## Node System Layers

The node system spans three layers: Engine (Rust), `@bnto/nodes` (TypeScript), and Editor. Each layer has distinct responsibilities. See [node-responsibilities.md](../scopes/rust/node-responsibilities.md) for the full decision matrix, golden rule, and common violations to watch for.

**Key principle:** The engine defines what nodes CAN do. `@bnto/nodes` makes that knowledge available in TypeScript (mostly generated). The editor manages the visual experience.

## Execution Model: Engine Owns the Pipeline

**The Rust engine owns pipeline execution.** The `bnto-core` crate contains the `PipelineExecutor` -- it handles graph walking, topological ordering, container node semantics (loop/group), per-file iteration, `NodeProcessor` dispatch, and structured progress events. JS / `@bnto/core` is a thin adapter: convert browser types (File to bytes, Definition to WASM struct), make a single WASM call (`run_pipeline`), and relay progress events to the UI.

This design ensures identical execution across all consumers -- CLI (native), browser (WASM), desktop (Tauri), server. See [engine-execution.md](../strategy/engine-execution.md) for the full architecture.

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

### Engine Consumers

The Rust engine is the core. Consumers access it through different paths depending on the target:

```
                          +---------------------+
                          |    Rust Engine       |
                          |  (bnto-engine crate) |
                          +-----+-------+-------+
                                |       |
               +----------------+       +----------------+
               v                                         v
    +---------------------+                   +---------------------+
    |    CLI (primary)    |                   |    WASM (browser)   |
    |   bnto crate        |                   |   bnto-wasm crate   |
    |   Links engine      |                   |   wasm-pack build   |
    |   directly as Rust  |                   +----------+----------+
    +---------------------+                              |
                                                         v
                                              +---------------------+
                                              |    @bnto/core       |
                                              |  (TypeScript API)   |
                                              |  Web Worker + WASM  |
                                              +----------+----------+
                                                         |
                                              +----------+----------+
                                              |    Next.js Web      |
                                              |    (apps/web)       |
                                              +---------------------+
```

**CLI** links `bnto-engine` as a native Rust dependency — zero overhead, full system access, no WASM boundary. This is the primary development and execution path.

**Browser** compiles `bnto-engine` to WASM via `bnto-wasm` (single cdylib). `@bnto/core` provides the TypeScript adapter layer (Web Worker, progress relay, file conversion). The web app is a secondary consumer.

**Desktop** (future, Tauri) will link `bnto-engine` natively like the CLI, running inside a system webview with the same React frontend.

### Web Abstraction Layer

For the web consumer, `@bnto/core` provides transport-agnostic adapters so UI components never know which backend they're talking to:

```
+--------------------------------------------------+
|               Next.js Web (apps/web)              |
|                       |                           |
|                       v                           |
|              @bnto/core                           |
|    +-------------+ +-------------+                |
|    |   Zustand    | | React Query |                |
|    |(client state)| |(server state)|               |
|    +------+------+ +------+------+                |
|           v               v                       |
|    +------------+  +------------+                 |
|    |  Convex    |  |  Browser   |                 |
|    |  adapter   |  |  adapter   |                 |
|    +-----+------+  +-----+------+                 |
|          v               v                        |
|    +----------+    +----------+                   |
|    | Convex   |    | WASM     |                   |
|    | (cloud)  |    | (engine) |                   |
|    +----------+    +----------+                   |
+--------------------------------------------------+
```

### Cloud Execution (M4)

Cloud execution topology TBD for M4. See `ROADMAP.md` for details.

### Development

`task dev` starts Next.js (localhost:4000) + Convex dev.

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
