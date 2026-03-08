# Core API Architecture

The `@bnto/core` package follows a layered singleton pattern:

```
clients (public API)  ->  queries (read-path)  +  services (write-path)  ->  adapters (backend-specific)
```

- **Clients** -- Domain-namespaced public API. `core.recipes`, `core.executions`, `core.auth`. Compose queries, services, and each other. Handle cross-domain side effects. Receive services via constructor injection
- **Queries** -- Pure read-path functions. Query option construction with `select` transforms. One file per domain (e.g., `queries/executionQueries.ts`). No side effects, no state mutation
- **Services** -- Single-domain write-path logic. Mutations, cache invalidation, infrastructure lifecycle (e.g., lazy engine init). **Services do NOT call other services.** Cross-domain orchestration lives in clients only
- **Adapters** -- Backend-specific bridge. Currently Convex (web) + browser (WASM engine, Web Worker), Tauri adapter planned (desktop). The only layer that imports from `@bnto/backend`. **Every adapter function that accepts an ID must use `"skip"` when the ID is falsy** -- see [convex.md](convex.md#convexquery-skip-guard-critical)

### Dependency Rules

```
Clients -> Queries + Services -> Adapters -> @bnto/backend
   |
 (compose multiple services, never service->service)
```

- Services NEVER import other services or clients
- Clients compose services via constructor injection (see `core.ts`)
- Hooks import from `core.*` (the client layer), never from services directly
- **Namespace exports, not top-level exports.** New core functionality is exposed via its domain namespace on the `core` singleton (e.g. `core.auth.onAuthError`), NOT as a top-level export from `@bnto/core`. Top-level exports are reserved for the `core` object itself, `BntoProvider`, constants, and types

**Core is imperative-first.** Plain TypeScript classes with async methods -- no React dependency. A separate React binding layer (hooks) makes it reactive for React consumers.

```typescript
// Imperative (framework-agnostic) -- clients
await core.recipes.save(input);

// React binding (thin reactive layer) -- hooks
const { data: recipes, isLoading } = useQuery(core.recipes.listQueryOptions());
```

### File Structure

```
packages/core/src/
|-- core.ts                    # Singleton -- wires services into clients
|-- clients/                   # Public API layer (cross-domain orchestration)
|-- queries/                   # Read-path: query option construction + select transforms
|-- services/                  # Write-path: mutations, invalidation, infrastructure lifecycle
|-- adapters/
|   |-- convex/                # Convex-specific bridge (web)
|   |-- browser/               # Browser execution: WASM engine, Web Worker, downloads
|   +-- tauri/                 # Tauri-specific bridge (desktop, planned)
|-- stores/                    # Zustand stores (internal -- opaque to consumers)
|-- transforms/                # Doc -> API type mappers
|-- hooks/                     # React binding layer (useExecutionState, useAuth, etc.)
+-- types/                     # Shared TypeScript types
```

---

## API Design Rules

These rules govern how `@bnto/core` exposes functionality to consumers. See [core-unification.md](../strategy/core-unification.md) for the full rationale.

### No Implementation Detail Leakage

**No technology names in the public API.** Consumers see domain concepts (`execution`, `history`, `flow`), never infrastructure (`wasm`, `convex`, `indexeddb`, `zustand`). Method names describe WHAT they do, not HOW they do it.

```typescript
// GOOD -- domain concept
core.executions.createExecution();
core.executions.clearHistory();

// BAD -- technology leaked
core.wasm.createExecution();
core.executions.clearLocalHistory();
```

**Test:** Read every method name on the `core` singleton. If you can identify the underlying technology from the name alone, it's a leak.

### Opaque Stores

Zustand stores created by core are **opaque to consumers**. Consumers access state via `core.<domain>.use*State()` hooks, never raw `.store.getState()` or `useStore(instance.store, ...)`.

```typescript
// GOOD -- opaque, core controls the API
const state = core.executions.useExecutionState(instance);

// BAD -- consumer reaches into internals
const state = useStore(instance.store, useShallow(s => ({ ... })));
```

Page-level orchestration stores (recipe flow, editor state) belong in the **app layer** (`apps/web`), not core. Core provides execution primitives; the app orchestrates the user journey.

### Lazy Infrastructure

Heavy infrastructure (Web Workers, WASM engines) initializes **lazily on first use**. No explicit setup hooks (`registerEngine`, `useBrowserEngine`) for consumers. Core owns its lifecycle.

```typescript
// GOOD -- just works, engine initializes on first call
const instance = core.executions.createExecution();
await instance.run("compress-images", files);

// BAD -- consumer must set up infrastructure
useBrowserEngine(); // manually registers engine
core.wasm.registerEngine(engine); // manual DI in consumer code
```

Services accept optional `engineOverride` for testing -- but the default path requires zero setup.

### Auth Boundary

Core hooks import from `@bnto/auth`, **never from `@convex-dev/auth/*` directly**. `@bnto/auth` is the abstraction boundary for authentication.

```typescript
// GOOD -- respects auth boundary
import { useSignOut as useAuthSignOut } from "@bnto/auth";

// BAD -- bypasses abstraction
import { useAuthActions } from "@convex-dev/auth/react";
```

### Separate Concerns

- **Recipes** -- what definition to run (registry/lookup). App or dedicated namespace concern
- **Executions** -- run a definition, track progress, get results. Core concern
- **Flow orchestration** -- coordinate user actions on a page (file drop → configure → run → results). App concern

The execution API accepts a **self-describing definition**. It doesn't know or care where the definition came from (predefined, custom, marketplace). The definition contains its own metadata (slug, name, nodes).

**Engine owns pipeline execution.** `core.executions.runPipeline()` converts browser types (File to bytes, Definition to WASM struct) and delegates to a single WASM call (`run_pipeline`). The Rust engine handles graph walking, file iteration, container semantics, and progress events internally. The JS-side `executePipeline.ts` orchestrator (which loops over files and calls per-file WASM functions) is **deprecated** in favor of the Rust executor. See [engine-execution.md](../strategy/engine-execution.md).

```typescript
// GOOD -- execution is definition-agnostic
core.executions.start({ definition, sessionId });

// BAD -- execution knows about recipe registry
core.executions.startPredefined({ slug: "compress-images", definition, sessionId });
```

---

## 5-Domain Public API

The `core` singleton exposes exactly 5 top-level domains:

| Domain            | Responsibility                                                | Key methods                                                                                    |
| ----------------- | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `core.recipes`    | Recipe definitions (predefined or user-created)               | `listQueryOptions()`, `save()`, `remove()`, `run()`                                            |
| `core.executions` | Unified execution -- delegates to engine via single WASM call | `createExecution()`, `useExecutionState()`, `isCapable()`, `downloadResult()`, `runPipeline()` |
| `core.user`       | Profile + usage stats (absorbed analytics)                    | `meQueryOptions()`, `usageQueryOptions()`, `useCurrentUser()`                                  |
| `core.auth`       | Session state + auth actions (absorbed session)               | `useReady()`, `useIsAuthenticated()`, `useAuth()`, `useSignOut()`                              |
| `core.telemetry`  | Product event tracking (PostHog)                              | `capture()`, `identify()`, `reset()`                                                           |

**Removed from public API:** `core.wasm` (→ executions), `core.recipe` (→ app layer), `core.analytics` (→ user), `core.session` (→ auth)

**Hidden (internal only):** `core.uploads`, `core.downloads` -- cloud execution infrastructure for M4

---

## State Management

| Type                             | Tool                                   | Example                              |
| -------------------------------- | -------------------------------------- | ------------------------------------ |
| **Server state (single entity)** | React Query (via `convexQuery` bridge) | Recipe detail, execution detail      |
| **Server state (paginated)**     | Convex native `usePaginatedQuery`      | Recipe lists, execution history      |
| **Client app state**             | Zustand (via core store layer)         | Editor content, UI preferences       |
| **Local UI state**               | `useState`                             | Modal open, active tab, form inputs  |
| **URL state**                    | Router params / search params          | Active tab, filters, selected recipe |

**Rules:**

- Select specific state slices, not entire stores (`useStore(s => s.field)`, not `useStore()`)
- Server data goes through `@bnto/core`, never fetched directly in components
- If only one component needs it, `useState` is fine -- don't over-engineer

### React Query: Always Use `select` for Data Transforms

**NEVER transform query data outside of `useQuery`.** Calling `.map()` or any transform on `data` in the hook body creates new references every render -> render thrashing.

```typescript
// BAD -- new array every render
return { recipes: data ? data.map(toRecipe) : [], isLoading };

// BAD -- useMemo is a band-aid
const recipes = useMemo(() => data?.map(toRecipe) ?? [], [data]);

// GOOD -- select is memoized by React Query
const { data, isLoading } = useQuery({
  ...queryOptions,
  select: (raw) => raw.map(toRecipe),
});
```

**Checklist for every `useQuery` call:**

- [ ] No transforms outside `select` -- `data ? toFoo(data) : null` must move into `select`
- [ ] No `.map()` / `.filter()` outside `select`
- [ ] No spread of query data -- `{ ...data, isLoading }` creates new objects
- [ ] Return primitives directly -- `data ?? false`, `data ?? []` are fine (referentially stable)

### Convex Native Pagination: `usePaginatedQuery`

Paginated lists use Convex's native `usePaginatedQuery` for real-time per-page subscriptions. See [data-fetching-strategy.md](../strategy/data-fetching-strategy.md) for the full decision record.

**All paginated hooks MUST guard on `useReady()`** -- the `ConvexProvider` mounts after hydration. Without the guard, `usePaginatedQuery` crashes on the first render.

```typescript
export function usePaginatedHook(id: string, options?: { pageSize?: number }) {
  const ready = core.auth.useReady();
  const { pageSize = 24 } = options ?? {};
  const { funcRef, args, transform } = core.domain.refMethod(id);
  const { results, status, loadMore } = usePaginatedQuery(
    funcRef,
    ready && id ? args : "skip", // skip until provider + valid ID
    { initialNumItems: pageSize },
  );
  const items = useMemo(() => transform(results), [results, transform]);
  return { items, isLoading: !ready || status === "LoadingFirstPage", status, loadMore };
}
```

**Rules:**

- [ ] Always guard: `ready && condition ? args : "skip"`
- [ ] Include `!ready` in `isLoading` so consumers see loading state before provider mounts
- [ ] Transform via `useMemo` -- `usePaginatedQuery` returns raw Convex docs
- [ ] Adapter file (`adapters/convex/paginated-query.ts`) is the abstraction boundary -- hooks never import `convex/react` directly
