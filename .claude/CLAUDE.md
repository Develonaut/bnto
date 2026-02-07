# Bento - Agent & Developer Guide

**Last Updated:** February 6, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...      | Read this first                                                    |
| ---------------------------- | ------------------------------------------------------------------ |
| Any code                     | [BENTO_BOX_PRINCIPLE.md](.claude/BENTO_BOX_PRINCIPLE.md)           |
| Architecture decisions       | [CLOUD_DESKTOP_STRATEGY.md](.claude/strategy/CLOUD_DESKTOP_STRATEGY.md) |
| Repo structure               | [MONOREPO_STRUCTURE.md](.claude/strategy/MONOREPO_STRUCTURE.md)    |
| Build tooling                | [MONOREPO_TOOLING.md](.claude/decisions/MONOREPO_TOOLING.md)       |
| Implementation task          | [PLAN.md](.claude/PLAN.md)                                        |
| Understanding the product    | [CLOUD_DESKTOP_STRATEGY.md](.claude/strategy/CLOUD_DESKTOP_STRATEGY.md) |

---

## Quick Context

**Bento** is a workflow automation engine. Users define workflows as `.bento.json` files that orchestrate tasks like image processing, file operations, data transformation, and HTTP requests.

- **Engine**: Go (CLI + execution engine in `engine/`)
- **Desktop**: Wails v2 (Phase 3 — free local app)
- **Web**: Next.js + Convex (Phase 1 — paid cloud app)
- **Shared Packages**: TypeScript monorepo with `@bento/core`, `@bento/ui`, `@bento/editor`

---

## Critical Architecture Rules

### 1. Layered Architecture

```
Apps (web/desktop) → @bento/editor → @bento/ui → @bento/core → Go Engine
```

Each layer only depends on layers below it. Never skip layers.

**The key insight:** `@bento/core` is the transport-agnostic API layer. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Wails bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**State management:** Zustand handles client-only state (editor content, UI preferences). React Query handles all server state (data fetching, caching, mutations). For the Convex path, `@convex-dev/react-query` preserves real-time subscriptions through React Query's interface.

**Desktop shares the web frontend:** Wails v2 renders the same React app in a system webview. `@bento/core` detects the runtime (browser vs Wails) and swaps the transport adapter internally — no separate frontend for desktop.

```typescript
// @bento/core — components use these hooks (any platform)
import { useWorkflows, useExecution, useRunWorkflow } from "@bento/core";

const workflows = useWorkflows();
const execution = useExecution(id);
const { mutate: run } = useRunWorkflow();

// Under the hood, @bento/core detects the environment:
// Web:     React Query + @convex-dev/react-query adapter → Convex
// Desktop: React Query + Wails adapter → Go engine bindings
```

### 2. API Abstraction

**UI code NEVER calls Convex, Wails, or Go directly.** Always go through `@bento/core` hooks.

```typescript
// CORRECT — use @bento/core hooks
const workflows = useWorkflows();
const { mutate: save } = useSaveWorkflow();

// WRONG — direct Convex calls in components
const workflows = useQuery(api.workflows.list);
const save = useMutation(api.workflows.save);

// WRONG — direct Wails calls in components
const workflows = window.go.main.App.ListWorkflows();
```

### 3. Bento Box Principle

Every file, function, and package does ONE thing well. See [BENTO_BOX_PRINCIPLE.md](.claude/BENTO_BOX_PRINCIPLE.md).

- **Files**: < 250 lines (max 500)
- **Functions**: < 20 lines (max 30)
- **No utility grab bags**: No `utils.go` or `helpers.ts` dumping grounds
- **Clear boundaries**: Each package owns its domain, no circular deps
- **YAGNI**: Don't add features, exports, or complexity "just in case"

### 4. Go Package Guidelines

```go
// Each package has one responsibility
engine/pkg/
├── api/          # Shared service layer (BentoService, DefaultRegistry)
├── engine/       # Orchestration ONLY
├── registry/     # Node type registration ONLY
├── storage/      # Persistent storage ONLY
├── node/         # Node type definitions and implementations
├── validator/    # Workflow validation ONLY
├── paths/        # Path resolution + config loading ONLY
├── logger/       # Logging ONLY
└── secrets/      # Secrets management ONLY
```

### 5. TypeScript Package Guidelines

```
packages/@bento/
├── core/         # API layer ONLY — hooks, types, transport adapters (Convex/Wails)
│                 #   Zustand: client state. React Query: server state.
│                 #   Runtime detection swaps transport — components never know.
├── auth/         # Cloud auth ONLY — wraps @convex-dev/auth (web only, desktop skips)
│                 #   Provider (server), hooks (client), middleware (server)
├── ui/           # Presentational ONLY — shadcn wrappers, design system
└── editor/       # Editor ONLY — JSON editor (Phase 1), visual editor (Phase 4)
```

- `@bento/ui` components are thin wrappers around shadcn — customize internals without touching consumers
- `@bento/editor` consumes `@bento/ui` for primitives and `@bento/core` for data
- Apps (`web`, `desktop`) are thin composition layers — import components, compose pages, minimal custom styling

---

## Repository Structure

```
bento/
├── CLAUDE.md                    # You are here
├── package.json                 # Turborepo root workspace
├── pnpm-workspace.yaml          # pnpm workspace config
├── turbo.json                   # Turborepo task config
├── Taskfile.yml                 # Go + cross-cutting orchestration
├── go.work                      # Go workspace (engine + apps/api)
├── apps/
│   ├── api/                     # Go HTTP API server (imports engine)
│   ├── web/                     # @bento/web — Next.js cloud app
│   └── desktop/                 # @bento/desktop — Wails frontend
├── packages/
│   └── @bento/                  # Scoped internal packages (n8n pattern)
│       ├── core/                # @bento/core — Transport-agnostic API
│       ├── auth/                # @bento/auth — Cloud auth (web only)
│       ├── ui/                  # @bento/ui — Design system
│       └── editor/              # @bento/editor — Workflow editor
├── engine/                      # All Go code
│   ├── go.mod                   # module github.com/Develonaut/bento
│   ├── cmd/bento/               # CLI binary
│   ├── pkg/                     # Go packages
│   ├── tests/                   # Integration tests + fixtures
│   └── examples/                # Example .bento.json files
└── .claude/                     # Strategy docs, decisions, plan
    ├── PLAN.md
    ├── BENTO_BOX_PRINCIPLE.md
    ├── strategy/
    └── decisions/
```

---

## Tech Stack

| Layer              | Technology                                    |
| ------------------ | --------------------------------------------- |
| **Engine**         | Go (CLI, execution, all node types)           |
| **Desktop**        | Wails v2 (Go + system webview)                |
| **Web Frontend**   | Next.js (React, server-side rendering)        |
| **Cloud Backend**  | Convex (self-hosted on Railway via template)   |
| **Cloud Execution**| Go HTTP service on Railway                    |
| **Auth**           | Convex Auth                                   |
| **Shared UI**      | shadcn/ui + Tailwind CSS                      |
| **Client State**   | Zustand (editor content, UI preferences)       |
| **Server State**   | React Query (universal data layer) + Convex real-time subscriptions |
| **Build (Go)**     | Taskfile.dev                                  |
| **Build (TS)**     | Turborepo + pnpm workspaces                   |

---

## Data Flow Architecture

All execution flows through `@bento/core` hooks → React Query → transport adapter → backend → Go engine.

Desktop (Wails v2) renders the **same React frontend** in a system webview. `@bento/core` detects the runtime and swaps adapters — components are identical across web and desktop.

```
┌──────────────────────────────────────────────────────────────┐
│                    Apps (same React code)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Next.js Web  │  │ Wails Desktop│  │   CLI        │       │
│  │  (Railway)    │  │ (webview)    │  │   (Terminal) │       │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │
│         │                  │                  │               │
│         └────────┬─────────┘                  │               │
│                  ▼                            │               │
│  ┌───────────────────────────────────┐        │               │
│  │         @bento/core               │        │               │
│  │  ┌─────────────┐ ┌─────────────┐ │        │               │
│  │  │   Zustand    │ │ React Query │ │        │               │
│  │  │(client state)│ │(server state)│ │        │               │
│  │  └─────────────┘ └──────┬──────┘ │        │               │
│  │          ┌───────────────┤         │        │               │
│  │          ▼               ▼         │        │               │
│  │  ┌────────────┐  ┌────────────┐   │        │               │
│  │  │  Convex    │  │   Wails    │   │        │               │
│  │  │  adapter   │  │   adapter  │   │        │               │
│  │  └─────┬──────┘  └─────┬──────┘   │        │               │
│  └────────┼───────────────┼──────────┘        │               │
│           ▼               ▼                   ▼               │
│    ┌──────────┐    ┌──────────┐        ┌──────────┐          │
│    │ Convex   │    │ Go Engine│        │ Go Engine│          │
│    │ (cloud)  │    │ (local)  │        │  (CLI)   │          │
│    └──────────┘    └──────────┘        └──────────┘          │
└──────────────────────────────────────────────────────────────┘
```

| Concern                | Technology                                |
| ---------------------- | ----------------------------------------- |
| Client state           | Zustand (editor content, UI preferences)  |
| Server state           | React Query (caching, fetching, mutations)|
| Real-time (web)        | @convex-dev/react-query adapter           |
| Real-time (desktop)    | React Query + Wails bindings              |
| Transport detection    | @bento/core runtime check (browser vs Wails webview) |
| Workflow execution     | Go engine (local or Railway)              |
| Auth                   | Convex Auth (cloud only)                  |

---

## Commands

```bash
# Go engine (via Taskfile)
task build              # Build bento CLI binary
task test               # Run engine tests with race detector
task vet                # Run go vet on all Go packages

# API server
task api:build          # Build API server binary
task api:test           # Run API server tests with race detector

# Frontend (via Turborepo)
task ui:build           # Build all TS packages (with Turbo caching)
task ui:test            # Run all TS tests
task ui:dev             # Start web app dev server
task ui:lint            # Lint all TS packages

# Everything
task build:all          # Build engine + API + frontend
task test:all           # Test engine + API + frontend
task check              # Full quality gate (vet + test + build)
```

---

## Documentation Index

### Must Read

| Document                                                           | Purpose                                        |
| ------------------------------------------------------------------ | ---------------------------------------------- |
| **[BENTO_BOX_PRINCIPLE.md](.claude/BENTO_BOX_PRINCIPLE.md)**       | Code organization, file/function size limits    |
| **[PLAN.md](.claude/PLAN.md)**                                     | Master checklist — what's done, what's next     |

### Reference

| Document                                                                        | Purpose                                    |
| ------------------------------------------------------------------------------- | ------------------------------------------ |
| [CLOUD_DESKTOP_STRATEGY.md](.claude/strategy/CLOUD_DESKTOP_STRATEGY.md)         | Full architecture, tech decisions, phases   |
| [MONOREPO_STRUCTURE.md](.claude/strategy/MONOREPO_STRUCTURE.md)                 | Repo structure, API abstractions, packages  |
| [MONOREPO_TOOLING.md](.claude/decisions/MONOREPO_TOOLING.md)                    | Taskfile + Turborepo decision rationale     |

---

## Agent Workflow

When starting work:

1. **Read context** - Review this file and relevant docs in `.claude/`
2. **Check the plan** - See where we are in [PLAN.md](.claude/PLAN.md)
3. **Plan first** - Break down the task before writing code
4. **Follow patterns** - Match existing code style and architecture
5. **Test boundaries** - Write tests for engine logic and API contracts
6. **Document decisions** - Add notes to `.claude/decisions/` for significant choices

### Quality Checklist

Before completing any task, verify:

- [ ] Follows layered architecture (no layer violations)
- [ ] Bento Box Principle respected (single responsibility, no grab bags)
- [ ] Go: files < 250 lines, functions < 20 lines
- [ ] API calls go through `@bento/core` (never direct Convex/Wails calls)
- [ ] No `any` types without justification
- [ ] Matches existing code patterns

---

## PRE-COMMIT Checklist (MANDATORY)

**CRITICAL:** Before committing ANY code, you MUST run through this entire checklist. If any item fails, STOP, fix the issue, and restart from the beginning.

**IMPORTANT - No Ignoring Failures:** You are NOT allowed to deem any issues as "pre-existing" or ignore them on your own. If automated checks fail for ANY reason (even in packages you didn't modify), you MUST report ALL failures to the user and let them decide whether to proceed. Only the user can determine if an issue is ignorable.

### Step 1: Automated Checks

Run all automated checks:

```bash
# Go checks
task vet                # go vet — must pass clean
task test               # engine go test -race — must pass
task api:test           # API server go test -race — must pass

# Frontend checks
task ui:build           # TypeScript compilation — must pass
task ui:test            # Frontend tests — must pass
```

**If any check fails:**

1. Fix the errors
2. Re-run from the top

### Step 2: Architecture Compliance

For EACH file you modified, verify:

- [ ] **Layered Architecture**: Does the code respect layer boundaries?
  - Apps → @bento/editor → @bento/ui → @bento/core → Go Engine
  - No layer skipping (components calling Convex/Wails directly)

- [ ] **API Abstraction**: Are all data operations going through `@bento/core`?
  - NO direct Convex queries in components or hooks
  - NO direct Wails bindings in components
  - ALL data access via `api.workflows.run()`, `api.executions.get()`, etc.

- [ ] **Go Package Boundaries**: Does each Go package stay in its lane?
  - `engine/` orchestrates, doesn't do I/O
  - `registry/` registers node types, doesn't execute them
  - `node/` types execute, don't know about other node types
  - No circular dependencies between packages

- [ ] **Single Responsibility**: Is each file/function small and focused?
  - Go: Files < 250 lines, functions < 20 lines
  - TS: Components render, hooks manage state, utils compute
  - No "god objects" or utility grab bags

### Step 3: Go Code Compliance

For EACH Go file you created or modified:

- [ ] **Bento Box Principle**: One concept per file, one purpose per function
  - See [BENTO_BOX_PRINCIPLE.md](.claude/BENTO_BOX_PRINCIPLE.md)

- [ ] **Error Handling**: Are errors properly wrapped with context?
  - `return fmt.Errorf("loading workflow %s: %w", path, err)` not bare `return err`
  - No swallowed errors (ignoring returned errors without comment)

- [ ] **Context Propagation**: Is `context.Context` passed through the chain?
  - All long-running operations accept and respect context
  - Cancellation is checked in loops and before expensive operations

- [ ] **Interface Design**: Are interfaces small and consumer-defined?
  - Accept interfaces, return structs
  - No mega-interfaces with 10+ methods

### Step 4: TypeScript Code Compliance

For EACH TypeScript file you created or modified:

- [ ] **Type Inference**: Are types inferred where possible?
  - NO redundant type annotations: `const x: Type = getValue<Type>()`
  - Let return types be inferred unless documenting a public API

- [ ] **No `any` Without Justification**: Search for `any` in your changes
  - Each `any` must have a comment explaining why
  - Consider `unknown` with type guards instead

- [ ] **Component Patterns**: Are components dumb?
  - Components receive data via props or hooks
  - Components render UI based on that data
  - NO business logic in render functions

- [ ] **Import Discipline**: Are imports from the right packages?
  - Import types from `@bento/core`, not re-exports
  - Each package only exports what it owns

### Step 5: Code Quality

- [ ] **No Secrets**: Are there any secrets or credentials in the code?
  - NO API keys, tokens, or passwords
  - Environment variables for all sensitive config

- [ ] **No Hardcoded Values**: Are magic numbers/strings extracted?
  - Use constants for repeated values
  - Use theme tokens for colors/spacing (no raw hex in components)

- [ ] **No Unused Code**: Did you remove dead code and unused imports?
  - No commented-out code blocks
  - No imports that are no longer used

- [ ] **Consistent Style**: Does the code match existing patterns?
  - Same naming conventions as similar files
  - Same file structure as similar components/packages

### Step 6: Test Evaluation

For EACH significant change, evaluate if tests are needed:

- [ ] **Does this code need tests?** Consider:
  - Go engine logic (node execution, validation, path resolution) → YES, unit tests
  - Go API endpoints → YES, integration tests with httptest
  - TypeScript API client logic → YES, unit tests with mocks
  - Pure utility functions → YES, unit tests
  - Simple UI components with no logic → NO, unless complex interactions
  - Configuration or type-only changes → NO

- [ ] **What type of tests?**
  - **Go unit tests**: Node types, engine execution, validators, path resolution
  - **Go integration tests**: End-to-end workflow execution with fixture .bento.json files
  - **TS unit tests**: API client logic, utility functions
  - **E2E tests**: Critical user flows (upload workflow → run → see results)

- [ ] **Test the boundaries**:
  - API contracts (what goes in, what comes out)
  - Error handling (what happens when things fail)
  - Edge cases (empty workflows, missing parameters, cancelled executions)

**If tests are needed but missing:**

1. Write the tests before committing
2. Ensure tests pass with `task test` and `task ui:test`

### Step 7: Commit Guidelines

When all checks pass:

1. Stage only the relevant files (no accidental additions)
2. Write a clear commit message:
   - Summarize the "why", not the "what"
   - Keep under 72 characters for the subject
   - Use imperative mood ("Add feature" not "Added feature")
3. Do NOT include:
   - `Generated with Claude Code` or Co-Authored-By lines
   - "Test Plan" sections
   - Unrelated changes bundled together

---

## Key Principles

1. **Transport-agnostic API** - All data flows through `@bento/core`, never direct backend calls
2. **Bento Box Principle** - One responsibility per file/function/package, no grab bags
3. **CLI is the stable API** - Every operation maps to a CLI command; the Go engine is the source of truth
4. **TDD bottom-up** - Solidify engine tests → API tests → E2E tests. Each layer tested before the next
5. **Test at boundaries** - Focus on API contracts and engine behavior, not implementation details
6. **Open source core** - Cloud sells convenience (hosting, managed infrastructure), not proprietary features
