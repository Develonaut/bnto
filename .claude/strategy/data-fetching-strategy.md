# Data Fetching Strategy

## Decision: Hybrid Convex Native + React Query

**Date:** 2026-02-21
**Status:** Active

Bnto uses a hybrid data fetching strategy that leverages each tool where it's strongest. This is a deliberate architectural choice, not an accident of incremental development.

---

## The Two Systems

### Convex Native (internal data)

Convex maintains live WebSocket subscriptions. Data updates push to the client automatically -- no polling, no manual refetching. This is the foundation of the real-time experience.

**Used for:**
- Paginated lists (`usePaginatedQuery`) -- workflows, executions, node types
- Any query where real-time reactivity matters (execution status, workflow runs, live logs)

**Why:** `usePaginatedQuery` maintains per-page subscriptions. When a workflow execution completes, the status updates instantly across all connected clients. This is critical for a workflow automation tool where users need to know execution state in real time.

### React Query (server state caching layer)

React Query provides caching, deduplication, stale-while-revalidate, DevTools, and a familiar API. `@convex-dev/react-query` bridges Convex subscriptions into React Query's cache for regular (non-paginated) queries.

**Used for:**
- Single-entity queries (workflow detail, execution detail, node configuration) -- via `convexQuery()` bridge
- Mutations with optimistic updates and cache invalidation
- External API data (future: webhook endpoints, third-party service status)

**Why:** React Query's caching and deduplication is valuable for single-entity fetches (multiple components fetching the same workflow hit the cache). The bridge preserves Convex's real-time subscriptions while adding React Query's API surface.

---

## Why Hybrid, Not One-or-the-Other

### Why not all React Query?

React Query's `useInfiniteQuery` would lose Convex's real-time reactivity on paginated data. Pages would be fetched once and cached -- new content requires manual refetching or polling. For a workflow tool where execution status changes constantly, this feels dead.

### Why not all Convex native?

1. Convex native hooks require `ConvexProvider` context. The bridge layer (`@convex-dev/react-query`) works through `QueryClientProvider` which is simpler to manage with SSR/SSG.
2. React Query's caching, deduplication, and DevTools add genuine value for single-entity queries.
3. Future integrations will fetch from external APIs (webhook status, third-party service health checks). These aren't Convex data -- they need React Query's HTTP caching.

### The bridge library agrees

`@convex-dev/react-query` re-exports `usePaginatedQuery` directly from `convex/react`. The library authors designed for exactly this hybrid: regular queries through React Query, pagination through Convex native.

---

## How It Maps to Product Areas

| Area | Data Source | Fetching Strategy |
|---|---|---|
| **Workflow List** | Convex (internal) | Paginated lists via Convex native, single entities via React Query bridge |
| **Execution Monitor** | Convex (internal) | Real-time execution status via Convex native subscriptions |
| **Workflow Editor** | Convex (internal) | Single workflow via React Query bridge, real-time validation feedback |
| **External Integrations** | External APIs | React Query for HTTP caching + deduplication |

React Query becomes *more* important as external integrations grow.

---

## Implementation Pattern

### Paginated hooks (Convex native)

```typescript
// packages/@bnto/core/src/hooks/workflow/use-workflows.ts
export function useWorkflows(options?: { pageSize?: number }) {
  const ready = core.session.useReady();
  const { funcRef, args, transform } = core.workflows.listRef();
  const { results, status, loadMore } = usePaginatedQuery(
    funcRef,
    ready ? args : "skip",   // guard: skip until provider is ready
    { initialNumItems: pageSize },
  );
  const workflows = useMemo(() => transform(results), [results, transform]);
  return { workflows, isLoading: !ready || status === "LoadingFirstPage", status, loadMore };
}
```

The `useReady()` guard is essential: the auth provider (which provides `ConvexProvider` context) mounts after hydration via `useEffect`. On the first render, there's no provider -- `usePaginatedQuery` would crash. Passing `"skip"` defers the subscription until the provider is available.

### Single-entity hooks (React Query bridge)

```typescript
// packages/@bnto/core/src/hooks/workflow/use-workflow-by-id.ts
export function useWorkflowById(workflowId: string) {
  return useQuery({
    ...core.workflows.byIdQueryOptions(workflowId),
    select: (raw) => toWorkflow(raw),
  });
}
```

### Abstraction boundary

`packages/@bnto/core/src/adapters/convex/paginated-query.ts` re-exports `usePaginatedQuery` from `convex/react`. If the backend ever changes, this one file gets rewritten. Consumers (hooks) never import from `convex/react` directly.

This is particularly important for Bnto because `@bnto/core` already implements the transport-agnostic adapter pattern. The Convex adapter and the future Wails adapter both expose the same interface. Hooks in `@bnto/core` work identically whether the app is running as a Next.js web app or a Wails desktop app.

---

## Desktop (Wails) Compatibility

`@bnto/core` detects the runtime environment and swaps the transport adapter:

- **Web:** React Query + `@convex-dev/react-query` bridge -> Convex Cloud
- **Desktop:** React Query + Wails adapter -> Go engine bindings

The same hooks work across web and desktop. The data fetching strategy ports cleanly because React Query is the universal caching layer in both paths. Only the transport adapter changes.

---

## Summary

| Query Type | Tool | Why |
|---|---|---|
| Paginated lists | Convex native `usePaginatedQuery` | Real-time per-page subscriptions |
| Single entities | React Query via `convexQuery()` bridge | Caching, deduplication, familiar API |
| Mutations | Convex client (through @bnto/core adapters) | Direct, with React Query cache invalidation |
| External APIs | React Query `useQuery` / `useInfiniteQuery` | HTTP caching for non-Convex sources |
