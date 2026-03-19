# @bnto/core

Transport-agnostic API layer for bnto — the single interface between UI and all backends.

## Overview

`@bnto/core` is the abstraction boundary. UI components import hooks and methods from `core`, never from Convex, Zustand, WASM, or any other infrastructure directly. Core detects the runtime (browser vs desktop) and routes requests to the correct backend adapter transparently.

Consumed by `apps/web/` (and eventually `apps/desktop/`). No UI code — just data, state, and infrastructure.

## Directory Structure

```
src/
├── core.ts                    # Singleton — wires services into clients
├── reactCore.ts               # React binding — merges hooks onto imperative clients
├── clients/                   # Public API (6 domain clients)
│   ├── authClient.ts          # Session state + auth actions
│   ├── executionClient.ts     # Unified execution (browser WASM + cloud)
│   ├── recipeClient.ts        # Recipe definitions (list, save, run)
│   ├── registryClient.ts      # Predefined recipes + node type metadata
│   ├── telemetryClient.ts     # Product event tracking (PostHog)
│   └── userClient.ts          # Profile + usage stats
├── queries/                   # Read-path — query option construction + select transforms
├── services/                  # Write-path — mutations, cache invalidation, lifecycle
├── adapters/
│   ├── convex/                # Convex bridge (web data layer)
│   ├── browser/               # WASM engine, Web Worker, file downloads
│   ├── local/                 # IndexedDB (execution history)
│   └── posthog/               # PostHog telemetry
├── stores/                    # Zustand stores (opaque to consumers)
├── transforms/                # Convex doc → API type mappers
├── hooks/                     # React binding layer
├── providers/                 # React providers (Session, Telemetry)
├── types/                     # Shared TypeScript types
└── utils/                     # Pure utility functions
```

## 6-Domain Public API

All access goes through the `core` singleton:

| Domain            | Responsibility                                             | Example                                |
| ----------------- | ---------------------------------------------------------- | -------------------------------------- |
| `core.recipes`    | Recipe definitions — list, get, save, remove, run          | `core.recipes.useRecipes()`            |
| `core.executions` | Execution lifecycle — create, run pipeline, track progress | `core.executions.createExecution()`    |
| `core.user`       | Profile + usage stats                                      | `core.user.useCurrentUser()`           |
| `core.auth`       | Session state + auth actions                               | `core.auth.useIsAuthenticated()`       |
| `core.telemetry`  | Product event tracking                                     | `core.telemetry.capture("recipe_run")` |
| `core.registry`   | Predefined recipes + node type metadata                    | `core.registry.useRecipes()`           |

## Key Concepts

- **Clients** — public API, one per domain. Compose queries + services. Handle cross-domain side effects
- **Queries** — pure read-path. Query option construction with `select` transforms. No side effects
- **Services** — single-domain write-path. Mutations, cache invalidation, infrastructure lifecycle. Services never call other services
- **Adapters** — backend-specific bridge. The only layer that imports `@bnto/backend`. Swappable per runtime
- **Stores** — Zustand, opaque to consumers. Accessed via `core.<domain>.use*State()` hooks, never raw `useStore()`

## Development

```bash
task ui:build       # TypeScript compilation (all packages)
task ui:test        # Run all TS tests (Vitest)
task ui:lint        # Lint all TS packages
```

Tests are co-located with source files. Integration tests live in `src/__tests__/integration/`.

## Usage

### Provider Setup

Wrap your app with `BntoCoreProvider` (handles Convex, React Query, auth, session):

```tsx
import { BntoCoreProvider } from "@bnto/core";

<BntoCoreProvider onSessionLost={() => router.replace("/signin")}>{children}</BntoCoreProvider>;
```

### Consuming Data

```tsx
import { core } from "@bnto/core";

// React hooks (reactive)
const { data, isLoading } = core.recipes.useRecipes();
const user = core.user.useCurrentUser();

// Imperative (framework-agnostic)
await core.recipes.save(recipeInput);
core.telemetry.capture("recipe_saved");
```

### Execution

```tsx
const instance = core.executions.createExecution();
await instance.run(definition, files);
const state = core.executions.useExecutionState(instance);
```

## Layered Architecture

```
UI Components
    ↓
core.{domain} (clients — public API)
    ↓
queries (read) + services (write)
    ↓
adapters (Convex / browser WASM / Tauri)
    ↓
@bnto/backend (Convex) | Rust engine (WASM) | Tauri (planned)
```

Each layer only depends on the layer below it. Adapters are the only code that touches infrastructure.
