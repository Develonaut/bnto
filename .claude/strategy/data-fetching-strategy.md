# Data Fetching Strategy

## Decision: Hybrid Convex Native + React Query

**Date:** 2026-03-02
**Status:** Active

Bnto uses a hybrid data fetching strategy that leverages each tool where it's strongest. This is a deliberate architectural choice, not an accident of incremental development.

---

## The Two Systems

### Convex Native (real-time lists)

Convex maintains live WebSocket subscriptions. Data updates push to the client automatically -- no polling, no manual refetching.

**Used for:**
- Paginated lists (`usePaginatedQuery`) -- workflows, executions, execution logs
- Any query where real-time reactivity matters (execution status updates, live progress)

**Why:** `usePaginatedQuery` maintains per-page subscriptions. When an execution completes or a new workflow is created, updates appear instantly across all connected clients without manual cache invalidation.

### React Query (server state caching layer)

React Query provides caching, deduplication, stale-while-revalidate, DevTools, and a familiar API. `@convex-dev/react-query` bridges Convex subscriptions into React Query's cache for regular (non-paginated) queries.

**Used for:**
- Single-entity queries (workflow detail, execution detail) -- via `convexQuery()` bridge
- Mutations with optimistic updates and cache invalidation
- External API data (future: marketplace integrations, community recipes)

**Why:** React Query's caching and deduplication is valuable for self-fetching components. Multiple components on the same page fetching the same workflow by ID all hit the cache -- zero redundant network calls. The bridge preserves Convex's real-time subscriptions while adding React Query's API surface.

---

## Why Hybrid, Not One-or-the-Other

### Why not all React Query?

React Query's `useInfiniteQuery` would lose Convex's real-time reactivity on paginated data. Pages would be fetched once and cached -- new content requires manual refetching or polling. For a tool that shows live execution progress, this feels broken.

### Why not all Convex native?

1. Convex native hooks require `ConvexProvider` context. The bridge layer (`@convex-dev/react-query`) works through `QueryClientProvider` which is simpler to manage with SSR/SSG.
2. React Query's caching and deduplication add genuine value for single-entity queries -- the self-fetching component pattern depends on it.
3. Future external APIs (community recipe marketplace, external tool integrations) aren't Convex data -- they need React Query's HTTP caching.

### The bridge library agrees

`@convex-dev/react-query` re-exports `usePaginatedQuery` directly from `convex/react`. The library authors designed for exactly this hybrid: regular queries through React Query, pagination through Convex native.

---

## How It Maps to the Product

| Area | Data Source | Fetching Strategy |
|---|---|---|
| **Recipe catalog** | Static (registry) | Build-time SSG, no runtime fetch |
| **Workflow CRUD** | Convex (internal) | Single entities via React Query bridge, lists via Convex native pagination |
| **Execution tracking** | Convex (internal) | Real-time status via Convex native, detail via React Query bridge |
| **WASM processing** | Local (browser) | No server fetch -- files stay client-side |
| **Community recipes (M4+)** | External APIs | React Query for HTTP caching + deduplication |

React Query becomes *more* important as the platform grows beyond Convex-only data.

---

## Implementation Patterns

### Self-fetching components (React Query bridge)

The foundational pattern. Each component fetches its own data by ID. React Query deduplicates and caches.

```typescript
// The component
export function WorkflowTitle({ workflowId }: { workflowId: string }) {
  const { data: workflow, isLoading } = core.workflows.useWorkflowById(workflowId);
  if (isLoading || !workflow) return <Skeleton className="h-7 w-48" />;
  return <Heading level={1}>{workflow.name}</Heading>;
}

// The hook (in @bnto/core)
export function useWorkflowById(workflowId: string) {
  return useQuery({
    ...core.workflows.byIdQueryOptions(workflowId),
    select: (raw) => toWorkflow(raw),
  });
}
```

**Why this works:** Five components on the same page all calling `useWorkflowById("abc123")` result in ONE Convex subscription. React Query deduplicates by query key. The skeleton shows while the single subscription resolves, then all five components render simultaneously.

### Paginated hooks (Convex native)

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

The `useReady()` guard is essential: `ConvexProvider` mounts after hydration via `useEffect`. On the first render, there's no provider -- `usePaginatedQuery` would crash. Passing `"skip"` defers the subscription until the provider is available.

### Abstraction boundary

`packages/core/src/adapters/convex/paginated-query.ts` re-exports `usePaginatedQuery` from `convex/react`. If the backend ever changes, this one file gets rewritten. Consumers (hooks) never import from `convex/react` directly.

---

## Co-located Queries: The Self-Fetching Component Pattern

This is the most important data fetching pattern in the codebase. It connects the fetching strategy to the component architecture.

**The rule:** Each leaf component fetches its own data. The query is co-located with the component that renders the result. The skeleton is co-located in the same file.

```tsx
// WorkflowTitle.tsx -- query, skeleton, and render all in one file
"use client";

export function WorkflowTitle({ workflowId }: { workflowId: string }) {
  const { data: workflow, isLoading } = core.workflows.useWorkflowById(workflowId);
  if (isLoading || !workflow) return <Skeleton className="h-7 w-48" />;
  return <Heading level={1}>{workflow.name}</Heading>;
}
```

**Three things co-located:**
1. **The query** -- `core.workflows.useWorkflowById(workflowId)`
2. **The skeleton** -- `<Skeleton className="h-7 w-48" />`
3. **The render** -- `<Heading level={1}>{workflow.name}</Heading>`

**Why co-location matters:**
- **Skeleton accuracy.** The skeleton is right next to the loaded render. When the render changes (different heading size, new badge), the skeleton is staring you in the face -- you'll update it
- **No prop drilling.** Parent doesn't fetch, doesn't pass data. Each component is self-contained
- **Cache deduplication.** Multiple components fetching the same entity converge on one cache entry. No redundant subscriptions
- **Independent loading.** Each component shows its own skeleton at its own pace. Fast queries resolve first. No waterfall waiting for the slowest sibling

### Anti-patterns

```tsx
// BAD -- parent fetches, passes data props (prop drilling)
const { data: workflow } = core.workflows.useWorkflowById(workflowId);
<WorkflowTitle title={workflow?.name} />
<WorkflowDescription description={workflow?.description} />

// BAD -- skeleton in a different file from the render
// WorkflowTitle.tsx renders the title
// WorkflowTitleSkeleton.tsx renders the skeleton (for simple cases)
// Now they can drift apart

// BAD -- "loading wrapper" that hides the skeleton
<LoadingGuard query={workflowQuery}>
  {(workflow) => <WorkflowTitle workflow={workflow} />}
</LoadingGuard>
```

---

## Desktop Compatibility

`convex/react` works in React Native and Tauri webviews. `usePaginatedQuery` with live subscriptions ports cleanly. The same hooks from `@bnto/core` work across web and desktop -- we swap the transport adapter, not the data layer.

---

## Summary

| Query Type | Tool | Why |
|---|---|---|
| Paginated lists | Convex native `usePaginatedQuery` | Real-time per-page subscriptions |
| Single entities | React Query via `convexQuery()` bridge | Caching, deduplication for self-fetching components |
| Mutations | Convex client (through core adapters) | Direct, with React Query cache invalidation |
| External APIs (future) | React Query `useQuery` / `useInfiniteQuery` | HTTP caching for non-Convex sources |
| Static catalog | Build-time SSG | Zero runtime cost |
