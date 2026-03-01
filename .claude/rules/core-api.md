# Core API Architecture

The `@bnto/core` package follows a layered singleton pattern:

```
clients (public API)  ->  services (single-domain logic)  ->  adapters (backend-specific)
```

- **Clients** -- Domain-namespaced public API. `core.workflows`, `core.executions`, `core.auth`. Compose one or more services. Handle cross-domain side effects. Receive services via constructor injection
- **Services** -- Single-domain business logic. Query options with transforms, mutations, cache invalidation for their own domain. **Services do NOT call other services.** Cross-domain orchestration lives in clients only
- **Adapters** -- Backend-specific bridge. Currently Convex (web), Tauri adapter planned (desktop). The only layer that imports from `@bnto/backend`. **Every adapter function that accepts an ID must use `"skip"` when the ID is falsy** -- see [convex.md](convex.md#convexquery-skip-guard-critical)

### Dependency Rules

```
Clients -> Services -> Adapters -> @bnto/backend
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
await core.workflows.save(input);

// React binding (thin reactive layer) -- hooks
const { data: workflows, isLoading } = useQuery(core.workflows.listQueryOptions());
```

### File Structure

```
packages/core/src/
|-- core.ts                    # Singleton -- wires services into clients
|-- clients/                   # Public API layer (cross-domain orchestration)
|-- services/                  # Internal single-domain logic
|-- adapters/
|   |-- convex/                # Convex-specific bridge (web)
|   +-- tauri/                 # Tauri-specific bridge (desktop, planned)
|-- transforms/                # Doc -> API type mappers
|-- hooks/                     # React binding layer
+-- types/                     # Shared TypeScript types
```

---

## State Management

| Type | Tool | Example |
|---|---|---|
| **Server state (single entity)** | React Query (via `convexQuery` bridge) | Workflow detail, execution detail |
| **Server state (paginated)** | Convex native `usePaginatedQuery` | Workflow lists, execution history |
| **Client app state** | Zustand (via core store layer) | Editor content, UI preferences |
| **Local UI state** | `useState` | Modal open, active tab, form inputs |
| **URL state** | Router params / search params | Active tab, filters, selected workflow |

**Rules:**
- Select specific state slices, not entire stores (`useStore(s => s.field)`, not `useStore()`)
- Server data goes through `@bnto/core`, never fetched directly in components
- If only one component needs it, `useState` is fine -- don't over-engineer

### React Query: Always Use `select` for Data Transforms

**NEVER transform query data outside of `useQuery`.** Calling `.map()` or any transform on `data` in the hook body creates new references every render -> render thrashing.

```typescript
// BAD -- new array every render
return { workflows: data ? data.map(toWorkflow) : [], isLoading };

// BAD -- useMemo is a band-aid
const workflows = useMemo(() => data?.map(toWorkflow) ?? [], [data]);

// GOOD -- select is memoized by React Query
const { data, isLoading } = useQuery({
  ...queryOptions,
  select: (raw) => raw.map(toWorkflow),
});
```

**Checklist for every `useQuery` call:**
- [ ] No transforms outside `select` -- `data ? toFoo(data) : null` must move into `select`
- [ ] No `.map()` / `.filter()` outside `select`
- [ ] No spread of query data -- `{ ...data, isLoading }` creates new objects
- [ ] Return primitives directly -- `data ?? false`, `data ?? []` are fine (referentially stable)

### Convex Native Pagination: `usePaginatedQuery`

Paginated lists use Convex's native `usePaginatedQuery` for real-time per-page subscriptions.

**All paginated hooks MUST guard on `useReady()`** -- the `ConvexProvider` mounts after hydration. Without the guard, `usePaginatedQuery` crashes on the first render.

```typescript
export function usePaginatedHook(id: string, options?: { pageSize?: number }) {
  const ready = core.session.useReady();
  const { pageSize = 24 } = options ?? {};
  const { funcRef, args, transform } = core.domain.refMethod(id);
  const { results, status, loadMore } = usePaginatedQuery(
    funcRef,
    ready && id ? args : "skip",        // skip until provider + valid ID
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
