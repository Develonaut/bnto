# Bento Monorepo Structure Strategy
**Go Backend + React Frontend with Transport-Agnostic API Layer**

**Date:** 2025-12-15
**Updated:** 2026-02-06
**Status:** Research Complete, Ready for Implementation
**Goal:** Establish a monorepo structure that supports desktop (Wails), web (Next.js cloud), and future mobile interfaces with a shared React frontend
**See also:** [Cloud + Desktop Strategy](CLOUD_DESKTOP_STRATEGY.md) — extends this document with cloud dimension

---

## Executive Summary

This document outlines the recommended monorepo structure for Bento's multi-interface architecture. The key insight is creating a **transport-agnostic API layer** in the React frontend, so the same UI code works across Wails desktop, REST server, and future mobile apps.

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Monorepo Tool | **Task (taskfile.dev)** | Wails v3 default, polyglot support, no Node.js required |
| Go Module Management | **go.work** (local dev only) | Native Go 1.18+ workspace support |
| Frontend Package Manager | **pnpm** | Fast, efficient disk usage, good monorepo support |
| Frontend Structure | **pnpm workspaces** | Shared packages without Turborepo overhead |
| API Abstraction | **TypeScript interface + providers** | Same UI code, different backends |

---

## Research Findings

### Wails Project Patterns

From the [Wails GitHub discussions](https://github.com/wailsapp/wails/discussions/909) and [awesome-wails](https://github.com/wailsapp/awesome-wails):

1. **No enforced structure** - Wails is flexible, two philosophies exist:
   - Frontend-heavy: React handles state, Go is persistence layer
   - Backend-heavy: Go handles state, React just renders

2. **Wails v3 uses Task** - The [Wails v3 alpha](https://v3alpha.wails.io/) uses taskfile.dev as its default build system

3. **Real-world example**: [my-app](https://github.com/jinyaoMa/my-app/tree/starter-code) demonstrates:
   - pnpm workspaces for frontend monorepo
   - Separate `backend/` and `frontend/` directories
   - `frontend/packages/` for shared UI components

### Go Monorepo Patterns

From [Earthly's Go monorepo guide](https://earthly.dev/blog/golang-monorepo/) and [Go workspaces tutorial](https://go.dev/doc/tutorial/workspaces):

1. **go.work for local development** - Don't commit `go.work`, use it locally
2. **replace directives** - Alternative for CI/CD when go.work isn't committed
3. **Structure**: `services/` for apps, `libs/` for shared code

### Polyglot Monorepo Patterns

From [Turborepo docs](https://turborepo.com/docs/crafting-your-repository/structuring-a-repository) and [Taskfile monorepo example](https://github.com/bensivo/monorepo-example-taskfile):

1. **apps/ + packages/** - Standard convention for Turborepo
2. **Task is lighter** - No Node.js dependency, works for Go + JS
3. **Root Taskfile includes sub-packages** - `task package:command` pattern

---

## Recommended Structure

```
bento/
├── Taskfile.yml                    # Root task orchestration
├── go.work                         # Go workspace (gitignored, local dev only)
├── go.mod                          # Main Go module
├── go.sum
│
├── cmd/
│   ├── bento/                      # CLI (existing, unchanged)
│   │   └── main.go
│   ├── bento-desktop/              # Wails v2 desktop app (Phase 3)
│   │   ├── main.go
│   │   ├── app.go                  # Wails-bound methods
│   │   ├── Taskfile.yml            # Desktop-specific tasks
│   │   └── wails.json
│   └── bento-server/               # Go API server (Phase 1 — cloud execution backend)
│       ├── main.go
│       ├── handlers.go             # HTTP handlers wrapping BentoService
│       └── Taskfile.yml
│
├── pkg/                            # Core Go packages (existing, unchanged)
│   ├── neta/
│   ├── itamae/
│   ├── pantry/
│   └── ...
│
├── internal/
│   └── api/                        # Go API service layer (shared by all interfaces)
│       ├── service.go              # BentoService interface
│       ├── workflows.go            # Workflow operations
│       ├── nodes.go                # Node operations
│       └── execution.go            # Execution operations
│
├── tests/                          # Integration tests and fixtures
│   ├── integration/                # Multi-node workflow tests
│   └── fixtures/                   # Real-world .bento.json files for testing
│
└── ui/                             # Shared React frontend
    ├── package.json                # Root package.json (private: true)
    ├── pnpm-workspace.yaml         # pnpm workspace config
    ├── Taskfile.yml                # Frontend tasks
    │
    ├── apps/
    │   ├── web/                    # Next.js cloud app (Phase 1 — Railway)
    │   │   ├── package.json
    │   │   ├── next.config.ts
    │   │   ├── convex/             # Convex schema and functions
    │   │   └── src/
    │   │       ├── app/            # Next.js App Router pages
    │   │       └── ...
    │   └── desktop/                # Wails v2 frontend entry (Phase 3)
    │       ├── package.json
    │       ├── index.html
    │       ├── vite.config.ts
    │       └── src/
    │           ├── main.tsx
    │           └── App.tsx
    │
    └── packages/
        ├── core/                   # @bento/core — THE API abstraction layer
        │   ├── package.json        #   (renamed from @bento/api for clarity)
        │   └── src/
        │       ├── index.ts
        │       ├── types.ts        # BentoAPI interface
        │       ├── convex.ts       # ConvexClient (web → Convex cloud)
        │       ├── wails.ts        # WailsClient (desktop → Go in-process)
        │       ├── rest.ts         # RestClient (future)
        │       └── provider.tsx    # React context provider
        │
        ├── ui/                     # @bento/ui — shadcn thin wrappers (design system)
        │   ├── package.json
        │   └── src/
        │       ├── components/
        │       │   ├── button.tsx
        │       │   ├── card.tsx
        │       │   └── ...
        │       └── index.ts
        │
        └── editor/                 # @bento/editor — Workflow editor components
            ├── package.json        #   JSON editor (Phase 1), visual editor (Phase 4)
            └── src/
                ├── JsonEditor.tsx  # Monaco/CodeMirror JSON editor
                ├── BentoBox.tsx    # Visual editor (Phase 4)
                └── ...
```

---

## API Abstraction Layer

The key to transport-agnostic UI is a well-defined API interface:

### TypeScript Interface (`ui/packages/core/src/types.ts`)

```typescript
// Core types (will be auto-generated from Go structs in Wails)
export interface Workflow {
  id: string
  name: string
  description?: string
  nodes: WorkflowNode[]
}

export interface WorkflowNode {
  id: string
  type: string
  name: string
  config: Record<string, unknown>
}

export interface ExecutionStatus {
  id: string
  state: 'pending' | 'running' | 'completed' | 'failed'
  progress: number
  currentNode?: string
  error?: string
}

// API Interface - implemented by each transport
export interface BentoAPI {
  workflows: {
    list(): Promise<string[]>
    load(path: string): Promise<Workflow>
    save(path: string, workflow: Workflow): Promise<void>
    validate(path: string): Promise<string[]>
    run(path: string, input?: Record<string, unknown>): Promise<string>
  }

  nodes: {
    listTypes(): Promise<string[]>
    getSchema(type: string): Promise<Record<string, unknown>>
  }

  execution: {
    getStatus(id: string): Promise<ExecutionStatus>
    cancel(id: string): Promise<void>
    onProgress(id: string, callback: (status: ExecutionStatus) => void): () => void
  }
}
```

### Wails Implementation (`ui/packages/core/src/wails.ts`)

```typescript
import type { BentoAPI } from './types'
// These are auto-generated by Wails from Go struct methods
import * as App from '../../wailsjs/go/main/App'
import { EventsOn, EventsOff } from '../../wailsjs/runtime/runtime'

export function createWailsClient(): BentoAPI {
  return {
    workflows: {
      list: () => App.ListWorkflows(),
      load: (path) => App.LoadWorkflow(path),
      save: (path, workflow) => App.SaveWorkflow(path, workflow),
      validate: (path) => App.ValidateWorkflow(path),
      run: (path, input) => App.RunWorkflow(path, input || {}),
    },

    nodes: {
      listTypes: () => App.ListNodeTypes(),
      getSchema: (type) => App.GetNodeSchema(type),
    },

    execution: {
      getStatus: (id) => App.GetExecutionStatus(id),
      cancel: (id) => App.CancelExecution(id),
      onProgress: (id, callback) => {
        const eventName = `execution:${id}:progress`
        EventsOn(eventName, callback)
        return () => EventsOff(eventName)
      },
    },
  }
}
```

### REST Implementation (`ui/packages/core/src/rest.ts`)

```typescript
import type { BentoAPI } from './types'

export function createRestClient(baseUrl: string): BentoAPI {
  const fetchJSON = async (path: string, options?: RequestInit) => {
    const res = await fetch(`${baseUrl}${path}`, {
      ...options,
      headers: { 'Content-Type': 'application/json', ...options?.headers },
    })
    if (!res.ok) throw new Error(await res.text())
    return res.json()
  }

  return {
    workflows: {
      list: () => fetchJSON('/api/workflows'),
      load: (path) => fetchJSON(`/api/workflows?path=${encodeURIComponent(path)}`),
      save: (path, workflow) => fetchJSON('/api/workflows', {
        method: 'PUT',
        body: JSON.stringify({ path, workflow }),
      }),
      validate: (path) => fetchJSON(`/api/workflows/validate?path=${encodeURIComponent(path)}`),
      run: (path, input) => fetchJSON('/api/workflows/run', {
        method: 'POST',
        body: JSON.stringify({ path, input }),
      }),
    },

    nodes: {
      listTypes: () => fetchJSON('/api/nodes'),
      getSchema: (type) => fetchJSON(`/api/nodes/${type}/schema`),
    },

    execution: {
      getStatus: (id) => fetchJSON(`/api/executions/${id}`),
      cancel: (id) => fetchJSON(`/api/executions/${id}/cancel`, { method: 'POST' }),
      onProgress: (id, callback) => {
        // SSE or WebSocket implementation
        const eventSource = new EventSource(`${baseUrl}/api/executions/${id}/stream`)
        eventSource.onmessage = (e) => callback(JSON.parse(e.data))
        return () => eventSource.close()
      },
    },
  }
}
```

### Convex Implementation (`ui/packages/core/src/convex.ts`)

```typescript
import type { BentoAPI } from './types'
import { ConvexClient } from 'convex/browser'
import { api } from '../convex/_generated/api'

export function createConvexClient(convex: ConvexClient): BentoAPI {
  return {
    workflows: {
      list: () => convex.query(api.workflows.list),
      load: (id) => convex.query(api.workflows.get, { id }),
      save: (id, workflow) => convex.mutation(api.workflows.save, { id, workflow }),
      validate: (id) => convex.action(api.workflows.validate, { id }),
      run: (id, input) => convex.action(api.executions.start, { workflowId: id, input: input || {} }),
    },

    nodes: {
      listTypes: () => convex.query(api.nodes.listTypes),
      getSchema: (type) => convex.query(api.nodes.getSchema, { type }),
    },

    execution: {
      getStatus: (id) => convex.query(api.executions.getStatus, { id }),
      cancel: (id) => convex.mutation(api.executions.cancel, { id }),
      onProgress: (id, callback) => {
        // Convex real-time subscription — automatic reactivity
        const unsubscribe = convex.onUpdate(api.executions.getStatus, { id }, callback)
        return unsubscribe
      },
    },
  }
}
```

### React Provider (`ui/packages/core/src/provider.tsx`)

```typescript
import { createContext, useContext, type ReactNode } from 'react'
import type { BentoAPI } from './types'

const BentoAPIContext = createContext<BentoAPI | null>(null)

export function BentoAPIProvider({
  client,
  children
}: {
  client: BentoAPI
  children: ReactNode
}) {
  return (
    <BentoAPIContext.Provider value={client}>
      {children}
    </BentoAPIContext.Provider>
  )
}

export function useBentoAPI(): BentoAPI {
  const api = useContext(BentoAPIContext)
  if (!api) {
    throw new Error('useBentoAPI must be used within BentoAPIProvider')
  }
  return api
}

// Convenience hooks
export function useWorkflows() {
  return useBentoAPI().workflows
}

export function useNodes() {
  return useBentoAPI().nodes
}

export function useExecution() {
  return useBentoAPI().execution
}
```

### Usage in Desktop App (`ui/apps/desktop/src/main.tsx`)

```typescript
import { createRoot } from 'react-dom/client'
import { BentoAPIProvider, createWailsClient } from '@bento/core'
import App from './App'

const client = createWailsClient()

createRoot(document.getElementById('root')!).render(
  <BentoAPIProvider client={client}>
    <App />
  </BentoAPIProvider>
)
```

---

## Task Configuration

### Root Taskfile (`Taskfile.yml`)

```yaml
version: '3'

includes:
  desktop: ./cmd/bento-desktop/Taskfile.yml
  ui: ./ui/Taskfile.yml

tasks:
  default:
    desc: Show available tasks
    cmds:
      - task --list

  # Development
  dev:
    desc: Run desktop app in development mode
    cmds:
      - task: desktop:dev

  # Build
  build:
    desc: Build CLI
    cmds:
      - go build -o bin/bento ./cmd/bento

  build:desktop:
    desc: Build desktop app
    cmds:
      - task: desktop:build

  build:all:
    desc: Build all targets
    cmds:
      - task: build
      - task: build:desktop

  # Test
  test:
    desc: Run Go tests
    cmds:
      - go test ./...

  test:all:
    desc: Run all tests (Go + frontend)
    cmds:
      - task: test
      - task: ui:test

  # Lint
  lint:
    desc: Run linters
    cmds:
      - golangci-lint run
      - task: ui:lint
```

### Desktop Taskfile (`cmd/bento-desktop/Taskfile.yml`)

```yaml
version: '3'

tasks:
  dev:
    desc: Run in development mode with hot reload
    dir: "{{.ROOT_DIR}}/cmd/bento-desktop"
    cmds:
      - wails dev

  build:
    desc: Build for current platform
    dir: "{{.ROOT_DIR}}/cmd/bento-desktop"
    cmds:
      - wails build

  build:all:
    desc: Build for all platforms
    dir: "{{.ROOT_DIR}}/cmd/bento-desktop"
    cmds:
      - wails build -platform darwin/universal
      - wails build -platform windows/amd64
      - wails build -platform linux/amd64
```

### UI Taskfile (`ui/Taskfile.yml`)

```yaml
version: '3'

tasks:
  install:
    desc: Install frontend dependencies
    dir: "{{.ROOT_DIR}}/ui"
    cmds:
      - pnpm install

  dev:
    desc: Run frontend dev server (standalone)
    dir: "{{.ROOT_DIR}}/ui"
    cmds:
      - pnpm --filter @bento/desktop dev

  build:
    desc: Build all frontend packages
    dir: "{{.ROOT_DIR}}/ui"
    cmds:
      - pnpm build

  test:
    desc: Run frontend tests
    dir: "{{.ROOT_DIR}}/ui"
    cmds:
      - pnpm test

  lint:
    desc: Run frontend linter
    dir: "{{.ROOT_DIR}}/ui"
    cmds:
      - pnpm lint

  add:ui:
    desc: Add a shadcn component (usage: task ui:add:ui -- button)
    dir: "{{.ROOT_DIR}}/ui/packages/ui"
    cmds:
      - pnpm dlx shadcn@latest add {{.CLI_ARGS}}
```

---

## pnpm Workspace Configuration

### `ui/pnpm-workspace.yaml`

```yaml
packages:
  - 'apps/*'
  - 'packages/*'
```

### `ui/package.json`

```json
{
  "name": "@bento/ui-root",
  "private": true,
  "scripts": {
    "build": "pnpm -r build",
    "dev": "pnpm --filter @bento/desktop dev",
    "test": "pnpm -r test",
    "lint": "pnpm -r lint"
  },
  "devDependencies": {
    "typescript": "^5.3.0"
  }
}
```

### Package Naming Convention

Following [Turborepo's recommendation](https://turborepo.com/docs/crafting-your-repository/structuring-a-repository), use a namespace prefix:

| Package | Name | Description |
|---------|------|-------------|
| Core API layer | `@bento/core` | Transport-agnostic API (ConvexClient, WailsClient, RestClient) |
| UI components | `@bento/ui` | shadcn thin wrappers — the design system |
| Editor components | `@bento/editor` | Workflow editor (JSON for MVP, visual for Phase 4) |
| Web app | `@bento/web` | Next.js cloud app (Railway) |
| Desktop app | `@bento/desktop` | Wails v2 local app |

---

## Go API Service Layer

The `internal/api/` package provides a clean interface that all UI backends (Wails, REST) can use:

### `internal/api/service.go`

```go
package api

import (
    "context"

    "github.com/yourusername/bento/pkg/itamae"
    "github.com/yourusername/bento/pkg/hangiri"
    "github.com/yourusername/bento/pkg/pantry"
)

// BentoService provides the API for all UI interfaces
type BentoService struct {
    storage  *hangiri.Hangiri
    executor *itamae.Executor
    registry *pantry.Registry
}

func NewBentoService() *BentoService {
    return &BentoService{
        storage:  hangiri.New(),
        executor: itamae.NewExecutor(),
        registry: pantry.DefaultRegistry(),
    }
}

// Workflows
func (s *BentoService) ListWorkflows(ctx context.Context) ([]string, error)
func (s *BentoService) LoadWorkflow(ctx context.Context, path string) (*neta.Definition, error)
func (s *BentoService) SaveWorkflow(ctx context.Context, path string, def *neta.Definition) error
func (s *BentoService) ValidateWorkflow(ctx context.Context, path string) ([]string, error)
func (s *BentoService) RunWorkflow(ctx context.Context, path string, input map[string]any) (string, error)

// Nodes
func (s *BentoService) ListNodeTypes(ctx context.Context) ([]string, error)
func (s *BentoService) GetNodeSchema(ctx context.Context, nodeType string) (any, error)

// Execution
func (s *BentoService) GetExecutionStatus(ctx context.Context, id string) (*ExecutionStatus, error)
func (s *BentoService) CancelExecution(ctx context.Context, id string) error
```

This service is then used by:
- **Wails**: `cmd/bento-desktop/app.go` wraps `BentoService` methods
- **REST**: `cmd/bento-server/handlers.go` calls `BentoService` from HTTP handlers

---

## Bootstrap Steps

### 1. Install Prerequisites

```bash
# Task runner (Wails v3 default)
go install github.com/go-task/task/v3/cmd/task@latest

# Wails CLI
go install github.com/wailsapp/wails/v3/cmd/wails@latest

# pnpm
npm install -g pnpm

# Verify installations
task --version
wails version
pnpm --version
```

### 2. Create Directory Structure

```bash
cd /Users/Ryan/Code/bento

# Create UI structure
mkdir -p ui/apps/desktop/src
mkdir -p ui/packages/api/src
mkdir -p ui/packages/ui/src
mkdir -p ui/packages/editor/src

# Create Go API layer
mkdir -p internal/api

# Create desktop command
mkdir -p cmd/bento-desktop
```

### 3. Initialize pnpm Workspace

```bash
cd ui

# Create workspace config
cat > pnpm-workspace.yaml << 'EOF'
packages:
  - 'apps/*'
  - 'packages/*'
EOF

# Create root package.json
cat > package.json << 'EOF'
{
  "name": "@bento/ui-root",
  "private": true,
  "scripts": {
    "build": "pnpm -r build",
    "dev": "pnpm --filter @bento/desktop dev",
    "test": "pnpm -r test",
    "lint": "pnpm -r lint"
  }
}
EOF

# Install dependencies
pnpm install
```

### 4. Initialize Wails Project

```bash
cd cmd/bento-desktop

# Initialize Wails (will create frontend structure)
wails init -n bento-desktop -t react-ts

# Move frontend to ui/apps/desktop (manual step - restructure as needed)
```

### 5. Set Up shadcn/ui

```bash
cd ui/packages/ui

# Initialize with Vite + React + TypeScript
pnpm create vite . --template react-ts

# Initialize shadcn
pnpm dlx shadcn@latest init

# Add components as needed
pnpm dlx shadcn@latest add button card dialog
```

### 6. Create Root Taskfile

```bash
cd /Users/Ryan/Code/bento

# Create root Taskfile.yml (copy from configuration above)
```

### 7. Verify Setup

```bash
# From repo root
task --list           # Should show all available tasks
task ui:install       # Install frontend deps
task desktop:dev      # Should launch Wails dev mode
```

---

## Migration Path

Since Bento already has an existing structure, here's the migration approach. See [CLOUD_DESKTOP_STRATEGY.md](CLOUD_DESKTOP_STRATEGY.md) for full phasing with TDD bottom-up philosophy.

### Phase 0: Engine Solidification (TDD Foundation)
1. Comprehensive tests for all neta types and CLI commands
2. Integration tests using real-world .bento.json fixture files
3. Existing `cmd/bento/` and `pkg/` unchanged, just heavily tested

### Phase 1: Cloud MVP (Next.js + Go API + Convex)
1. Create `ui/` directory with pnpm workspace
2. Create `internal/api/` service layer + `cmd/bento-server/`
3. Create `ui/apps/web/` (Next.js) + `ui/packages/core/` (with ConvexClient)
4. API integration tests before any UI work

### Phase 2: Polish + File Support
1. Extend cloud capabilities (file handling, execution history)
2. More fixture bentos, more integration tests

### Phase 3: Desktop App (Wails v2)
1. Create `cmd/bento-desktop/` with Wails v2 project
2. Wire up to `internal/api/` service via WailsClient in `@bento/core`
3. Reuse `@bento/ui` and `@bento/editor` from web

### Phase 4: Monetization + Visual Editor
1. Stripe integration, usage tiers
2. Visual workflow editor using `@bento/editor`

---

## Why Not Turborepo?

While Turborepo is excellent for JS-only monorepos, it adds complexity for a polyglot repo:

| Aspect | Turborepo | Task |
|--------|-----------|------|
| Node.js required | Yes | No |
| Go support | Via scripts only | Native |
| Wails integration | Manual | Default in Wails v3 |
| Learning curve | Medium | Low |
| Caching | Excellent | Basic (sources/generates) |

**Recommendation**: Start with Task. If frontend caching becomes a bottleneck, consider adding Turborepo for the `ui/` directory only.

---

## References

### Research Sources
- [Wails v3 Alpha Documentation](https://v3alpha.wails.io/)
- [Wails GitHub Discussion: Complex App Structure](https://github.com/wailsapp/wails/discussions/909)
- [Awesome Wails](https://github.com/wailsapp/awesome-wails)
- [Go Workspaces Tutorial](https://go.dev/doc/tutorial/workspaces)
- [Building a Monorepo in Golang - Earthly](https://earthly.dev/blog/golang-monorepo/)
- [Turborepo: Structuring a Repository](https://turborepo.com/docs/crafting-your-repository/structuring-a-repository)
- [Minimalist Monorepos with Taskfile.dev](https://github.com/bensivo/monorepo-example-taskfile)
- [my-app Wails Starter with pnpm Monorepo](https://github.com/jinyaoMa/my-app/tree/starter-code)

### Related Bento Documents
- [Wails Desktop Strategy](.claude/strategy/WAILS_DESKTOP_STRATEGY.md)
- [Desktop UI Architecture](.claude/strategy/DESKTOP_UI_ARCHITECTURE.md)
- [Bento Box UI POC](.claude/strategy/BENTOBOX_UI_POC.md)

---

**Status:** Research complete, updated for cloud strategy (2026-02-06)
**Next Action:** Phase 0 — Engine solidification via TDD (see CLOUD_DESKTOP_STRATEGY.md)
