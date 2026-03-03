# Core API Unification

**Status:** Complete
**Last Updated:** March 2026

---

## What Changed

`@bnto/core` had implementation details leaking through its public API. Consumers had to juggle multiple namespaces, touch raw Zustand stores, and the package itself imported auth providers directly. This refactor eliminates all of it.

The public API surface went from 10 domain namespaces to 5. "Workflow" terminology was scrubbed — recipes are the atomic unit.

### Before (10 namespaces)

```
core.wasm.hasImplementation(slug)          // "wasm" in public API
core.wasm.createExecution()                // separate namespace for browser execution
core.wasm.registerEngine(engine)           // manual engine setup
core.wasm.downloadResult(result)

core.executions.startPredefined(input)     // "predefined" couples recipe registry
core.executions.clearLocalHistory()        // "local" leaks storage detail

core.recipe.createStore(config)            // page orchestration in core
core.analytics.useUsageAnalytics()         // confusing next to telemetry
core.session.useReady()                    // separate from auth

browserInstance.store.getState()           // raw Zustand in consumer
useStore(browserInstance.store, ...)        // consumer touches internals

// In core's useSignOut hook:
import { useAuthActions } from "@convex-dev/auth/react"  // bypasses @bnto/auth
```

Consumer (`useRecipeFlow.ts`) had to:
1. Check `core.wasm.hasImplementation()` for browser capability
2. Create execution via `core.wasm.createExecution()`
3. Subscribe to progress via `useStore(instance.store, useShallow(...))`
4. Download results via `core.wasm.downloadResult()`
5. Track page flow via `core.recipe.createStore()`
6. Check session via `core.session.useReady()`
7. Check usage via `core.analytics.useUsageAnalytics()`

Seven API calls across five namespaces. The consumer had to understand WASM, Zustand, predefined vs custom, local vs server.

### After (5 namespaces)

```
core.recipes.*                           // recipe definitions
core.executions.hasImplementation(slug)  // unified execution
core.executions.createExecution()
core.executions.useExecutionState(instance)
core.executions.downloadResult(result)
core.user.useUsageAnalytics()            // usage stats (was analytics)
core.auth.useReady()                     // session merged into auth
core.telemetry.*                         // product tracking
```

Consumer:
1. Check `core.executions.hasImplementation()` for capability
2. Create execution via `core.executions.createExecution()`
3. Subscribe to progress via `core.executions.useExecutionState(instance)`
4. Download results via `core.executions.downloadResult()`
5. Recipe flow state via app-local `createRecipeFlowStore()` (imported from `@bnto/core`)

One execution namespace. No "wasm", "server", "local", "predefined", "convex", or "zustand" in the API. Recipe flow state is an app-level hook, not a core concern. Engine initializes lazily. Stores are opaque.

---

## Domain Audit — 10 → 5 Namespaces

| Domain | Verdict | Rationale |
|---|---|---|
| `core.workflows` | **RENAMED** → `core.recipes` | Recipes are the atomic unit. No workflow/recipe distinction |
| `core.executions` | **KEPT** (unified) | Absorbed `wasm`, browser execution, history. Auto-routes browser/server |
| `core.user` | **KEPT** (absorbed analytics) | Profile + usage stats. "Analytics" was confusing next to "telemetry" |
| `core.session` | **MERGED** → `auth` | Session state and auth actions are the same domain |
| `core.auth` | **KEPT** (absorbed session) | One place for "who am I and am I signed in" |
| `core.telemetry` | **KEPT** | Product event tracking (PostHog). Not user-facing |
| `core.wasm` | **MERGED** → `executions` | Implementation detail (browser execution) |
| `core.recipe` | **MOVED** → `apps/web` | App concern (page-level flow orchestration) |
| `core.uploads` | **HIDDEN** | Cloud execution infrastructure (M4, not active) |
| `core.downloads` | **HIDDEN** | Cloud execution infrastructure (M4, not active) |
| `core.analytics` | **MERGED** → `user` | Usage stats are user-scoped data |

---

## Why

### 1. Implementation details leaked to consumers

"WASM" is how browser execution works internally. The consumer doesn't care — they want to run a recipe. Exposing `core.wasm` forces consumers to understand the execution infrastructure.

### 2. Two namespaces for one concept

`core.wasm.*` and `core.executions.*` both deal with execution. The consumer had to know which one to use for which operation. This is a layering failure — the client layer should compose services, not expose them individually.

### 3. Raw stores in consumer code

`browserInstance.store.getState()` and `useStore(instance.store, useShallow(...))` leak Zustand into consumer code. If core changes its state management (unlikely but possible), every consumer breaks.

### 4. Auth provider bypass

`useSignOut` in core imported `@convex-dev/auth/react` directly instead of going through `@bnto/auth`. This violates the layered architecture — `@bnto/auth` exists specifically as the auth abstraction boundary.

### 5. Confusing analytics vs telemetry split

`core.analytics` (user-facing usage stats) and `core.telemetry` (PostHog tracking) sounded like the same thing. Merging analytics into `core.user` clarifies: user domain = user-facing data, telemetry = product-team tracking.

### 6. Recipe flow orchestration in core

`core.recipe.createStore()` managed page-level state (which step the user is on, what files are staged). This is app-layer orchestration, not a core primitive. Moved to `apps/web`.

---

## Patterns Established

These patterns are now codified in [core-api.md](../rules/core-api.md) and apply to all `@bnto/core` development going forward.

### No Implementation Detail Leakage

No technology names in the public API. Consumers see domain concepts (`execution`, `history`), never infrastructure (`wasm`, `convex`, `indexeddb`, `zustand`).

**Test:** Read every method name on the `core` singleton. If you can identify the underlying technology from the name alone, it's a leak.

### Opaque Stores

Zustand stores created by core are opaque. Consumers access state via `core.<domain>.use*State()` hooks, never raw `.store.getState()`. This gives core freedom to change state management without breaking consumers.

Page-level orchestration stores (like recipe flow) belong in the app layer, not core.

### Lazy Infrastructure

Heavy infrastructure (Web Workers, WASM engines) initializes lazily on first use. No explicit setup hooks (`registerEngine`, `useBrowserEngine`) for consumers. Core owns its lifecycle.

### Auth Boundary

Core hooks import from `@bnto/auth`, never from `@convex-dev/auth/*` directly. `@bnto/auth` is the abstraction boundary. If we swap auth providers, only `@bnto/auth` internals change — core and apps are unaffected.

### Separate Concerns

- **Recipes** — what definition to run (registry/lookup, app concern)
- **Executions** — run a definition, track progress, get results (core concern)
- **Flow orchestration** — coordinate user actions on a page (app concern)

The execution API accepts a definition. It doesn't know or care where it came from.

### Query Layer Separation

Pure read-path functions (query option construction with transforms) live in `queries/`. Services handle mutations and cache invalidation only. Clients compose both.

```
Clients → Queries + Services → Adapters → @bnto/backend
```

---

## How It Applies Going Forward

When adding new functionality to `@bnto/core`:

1. **Name methods by what they do, not how they do it.** `createExecution()`, not `createWasmExecution()`.
2. **Return opaque types.** If consumers need reactive state, provide a `use*State()` hook. Don't expose raw stores.
3. **Lazy-initialize infrastructure.** If it needs a Worker, WebSocket, or engine — create it on first use. Don't make the consumer set it up.
4. **Keep orchestration in the app.** Multi-step user flows (file drop → configure → run → results) are app-level hooks. Core provides the primitives.
5. **One namespace per domain.** Don't split a concept across multiple namespaces (`core.wasm` + `core.executions`). If it's all execution, it's all `core.executions`.
6. **Definitions are self-describing.** Pass complete objects, not IDs that need server-side lookup. The definition contains its own metadata (slug, name, nodes).

---

## Files Changed

**Renamed (wasm → browser):** `types/wasm.ts` → `browser.ts`, `adapters/wasm/` → `browser/`, `stores/wasmExecutionStore` → `executionInstanceStore`, `services/wasmExecutionService` → `browserExecutionService`

**Renamed (workflow → recipe):** `clients/workflowClient.ts` → `recipeClient.ts`, `services/workflowService.ts` → `recipeService.ts`, `queries/workflowQueries.ts` → `recipeQueries.ts`, adapters

**Merged:** `wasmClient.ts` → `executionClient.ts`, `analyticsClient.ts` → `userClient.ts`, session hooks → `core.auth`, analytics hooks → `core.user`

**Deleted:** `wasmClient.ts`, `analyticsClient.ts`, `recipeFlowClient.ts`, `uploadClient.ts`, `downloadClient.ts`, `useWasmExecution.ts`, `useWasmExecutionStore.ts`

**New:** `hooks/useExecutionState.ts` (opaque store access via Symbol)

**Consumer updates:** `useRecipeFlow.ts`, `UsageStats.tsx`, `SavedRecipeCard.tsx`, `RecipeGrid.tsx`, `RecipeCardShowcase.tsx`

---

## Copyable Prompt

Use this prompt when starting a Claude Code session that works on `@bnto/core` internals:

```
Read these documents before making any changes to @bnto/core:

1. .claude/rules/core-api.md — canonical architecture (clients → queries + services → adapters)
2. .claude/strategy/core-unification.md — patterns from the core unification refactor

Key rules:
- No implementation details in the public API (no "wasm", "convex", "zustand", "local" in method names)
- Stores are opaque — consumers use core.<domain>.use*State() hooks, never raw .store
- Infrastructure initializes lazily (no manual setup hooks for consumers)
- Auth hooks import from @bnto/auth, never from @convex-dev/auth/* directly
- Execution accepts definitions, not slugs — recipe lookup is the caller's concern
- Page-level orchestration (multi-step flows) belongs in apps/web, not core
- Query options live in queries/, mutations in services/, backend specifics in adapters/

5 domains: core.recipes, core.executions, core.user, core.auth, core.telemetry
```
