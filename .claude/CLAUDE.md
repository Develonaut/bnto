# Bnto - Agent & Developer Guide

**Last Updated:** February 22, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...      | Read this first                                                    |
| ---------------------------- | ------------------------------------------------------------------ |
| Any code                     | [code-standards.md](.claude/rules/code-standards.md)              |
| Any UI / styling work        | [rules/theming.md](.claude/rules/theming.md)                       |
| Architecture decisions       | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) |
| Repo structure               | [monorepo-structure.md](.claude/strategy/monorepo-structure.md)    |
| Build tooling                | [monorepo-tooling.md](.claude/decisions/monorepo-tooling.md)       |
| Implementation task          | [PLAN.md](.claude/PLAN.md)                                        |
| Predefined Bntos & SEO slugs | [strategy/bntos.md](.claude/strategy/bntos.md)                    |
| SEO & URL strategy           | [rules/seo.md](.claude/rules/seo.md)                              |
| Coding standards             | [rules/](.claude/rules/) directory                                 |
| Understanding the product    | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) |
| Core principles (always)     | [core-principles.md](.claude/strategy/core-principles.md)          |

---

## Quick Context

**Bnto** is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script. Simple by default, powerful when you need it.

Workflows are defined as `.bnto.json` files that orchestrate tasks like image processing, file operations, data transformation, and HTTP requests. The Go CLI engine is the stable core. Everything else — web app, desktop, cloud execution — is a UI on top of it.

- **Engine**: Go (CLI + execution engine in `engine/`)
- **Web + Cloud**: Next.js on Vercel + Convex Cloud + Better Auth + Go API on Railway (Phase 1 — UI + auth + cloud execution)
- **Desktop**: Wails v2 (Phase 2 — free local execution)
- **Shared Packages**: `@bnto/core` (transport-agnostic API), `@bnto/auth` (auth), `@bnto/backend` (Convex). UI and editor components co-located in `apps/web` until a second consumer (desktop) exists
- **Open Source**: MIT licensed

---

## Critical Architecture Rules

### 1. Layered Architecture

```
Apps (web/desktop) → @bnto/core → Go Engine
```

Each layer only depends on layers below it. Never skip layers.

> **Co-location note:** UI components (`@bnto/ui`) and editor features (`@bnto/editor`) are currently co-located in `apps/web/`. They will be extracted into separate packages when the desktop app creates a real second consumer. Engine, core API, and data layer logic stays in `@bnto/core`.

**The key insight:** `@bnto/core` is the transport-agnostic API layer. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Wails bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**State management:** Zustand handles client-only state (editor content, UI preferences). React Query handles all server state (data fetching, caching, mutations). For the Convex path, `@convex-dev/react-query` preserves real-time subscriptions through React Query's interface.

**Desktop shares the web frontend:** Wails v2 renders the same React app in a system webview. `@bnto/core` detects the runtime (browser vs Wails) and swaps the transport adapter internally — no separate frontend for desktop.

```typescript
// @bnto/core — components use these hooks (any platform)
import { useWorkflows, useExecution, useRunWorkflow } from "@bnto/core";

const workflows = useWorkflows();
const execution = useExecution(id);
const { mutate: run } = useRunWorkflow();

// Under the hood, @bnto/core detects the environment:
// Web:     React Query + @convex-dev/react-query adapter → Convex
// Desktop: React Query + Wails adapter → Go engine bindings
```

### 2. API Abstraction

**UI code NEVER calls Convex, Wails, or Go directly.** Always go through `@bnto/core` hooks.

```typescript
// CORRECT — use @bnto/core hooks
const workflows = useWorkflows();
const { mutate: save } = useSaveWorkflow();

// WRONG — direct Convex calls in components
const workflows = useQuery(api.workflows.list);
const save = useMutation(api.workflows.save);

// WRONG — direct Wails calls in components
const workflows = window.go.main.App.ListWorkflows();
```

### 3. Bento Box Principle

Every file, function, and package does ONE thing well. See [code-standards.md](.claude/rules/code-standards.md).

- **Files**: < 250 lines (max 500)
- **Functions**: < 20 lines (max 30)
- **No utility grab bags**: No `utils.go` or `helpers.ts` dumping grounds
- **Clear boundaries**: Each package owns its domain, no circular deps
- **YAGNI**: Don't add features, exports, or complexity "just in case"

### 4. Go Package Guidelines

```go
// Each package has one responsibility
engine/pkg/
├── api/          # Shared service layer (BntoService, DefaultRegistry)
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

Shared packages live under `packages/`. UI and editor components are co-located in `apps/web/` until a second consumer exists.

```
packages/
├── core/         # API layer ONLY — hooks, types, transport adapters (Convex/Wails)
│                 #   Zustand: client state. React Query: server state.
│                 #   Runtime detection swaps transport — components never know.
└── @bnto/
    ├── auth/     # Cloud auth ONLY — wraps Better Auth + @better-auth/convex (web only, desktop skips)
    │             #   Provider (server), hooks (client), middleware (server)
    └── backend/  # Data layer ONLY — Convex schema, functions, business logic

apps/web/
├── components/   # UI components (shadcn wrappers, design system) — future @bnto/ui
└── app/          # Pages, editor features — future @bnto/editor
```

- UI components and editor features live in `apps/web/` for now — author with extraction in mind
- When desktop app arrives, extract shared UI into `@bnto/ui` and editor into `@bnto/editor`
- `@bnto/core` remains the transport-agnostic API layer consumed by all apps

---

## Repository Structure

```
bnto/
├── CLAUDE.md                    # You are here
├── package.json                 # Turborepo root workspace
├── pnpm-workspace.yaml          # pnpm workspace config
├── turbo.json                   # Turborepo task config
├── Taskfile.yml                 # Go + cross-cutting orchestration
├── go.work                      # Go workspace (engine + apps/api)
├── apps/
│   ├── api/                     # Go HTTP API server (Phase 1 — cloud execution)
│   ├── web/                     # @bnto/web — Next.js on Vercel (Phase 1)
│   │   └── components/          # UI components + editor (co-located, future @bnto/ui + @bnto/editor)
│   └── desktop/                 # @bnto/desktop — Wails frontend (Phase 2)
├── packages/
│   ├── core/                    # @bnto/core — Transport-agnostic API (public)
│   └── @bnto/                   # Private internal packages
│       ├── auth/                # @bnto/auth — Cloud auth (web only)
│       └── backend/             # @bnto/backend — Convex schema + functions
├── engine/                      # All Go code
│   ├── go.mod                   # module github.com/Develonaut/bnto
│   ├── cmd/bnto/               # CLI binary
│   ├── pkg/                     # Go packages
│   │   └── tui/                 # Bubble Tea TUI (paused — preserved for reference)
│   ├── tests/                   # Integration tests + fixtures
│   └── examples/                # Example .bnto.json files
└── .claude/                     # Strategy docs, decisions, plan
    ├── PLAN.md                  # Build plan (sprints, waves, tasks)
    ├── rules/                   # Coding standards and conventions
    ├── skills/                  # Agent skills (pre-commit, pickup, code-review)
    ├── strategy/
    ├── decisions/
    └── archive/                 # Completed phases and old docs
```

---

## Tech Stack

| Layer              | Technology                                    |
| ------------------ | --------------------------------------------- |
| **Engine**         | Go (CLI, execution, all node types)           |
| **Web Frontend**   | Next.js on Vercel                             |
| **Desktop**        | Wails v2 (Go + system webview)                |
| **Database**       | Convex Cloud (managed)                        |
| **Cloud Execution**| Go HTTP service on Railway (Phase 1)          |
| **Auth**           | Better Auth + @better-auth/convex             |
| **Shared UI**      | shadcn/ui + Tailwind CSS                      |
| **Client State**   | Zustand (editor content, UI preferences)       |
| **Server State**   | React Query (universal data layer) + Convex real-time subscriptions |
| **Build (Go)**     | Taskfile.dev                                  |
| **Build (TS)**     | Turborepo + pnpm workspaces                   |

---

## Data Flow Architecture

All execution flows through `@bnto/core` hooks → React Query → transport adapter → backend → Go engine.

Desktop (Wails v2) renders the **same React frontend** in a system webview. `@bnto/core` detects the runtime and swaps adapters — components are identical across web and desktop.

```
┌──────────────────────────────────────────────────────────────┐
│                    Apps (same React code)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Next.js Web  │  │ Wails Desktop│  │   CLI        │       │
│  │  (Vercel)     │  │ (webview)    │  │   (Terminal) │       │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │
│         │                  │                  │               │
│         └────────┬─────────┘                  │               │
│                  ▼                            │               │
│  ┌───────────────────────────────────┐        │               │
│  │         @bnto/core               │        │               │
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
| Transport detection    | @bnto/core runtime check (browser vs Wails webview) |
| Workflow execution     | Go engine (local or Railway)              |
| Auth                   | Better Auth (cloud only)                  |

---

## Commands

```bash
# Go engine (via Taskfile)
task build              # Build bnto CLI binary
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
| **[core-principles.md](.claude/strategy/core-principles.md)**      | TDD, Grain, Modularity, Abstraction — the DNA  |
| **[code-standards.md](.claude/rules/code-standards.md)**           | Code organization, file/function size limits    |
| **[PLAN.md](.claude/PLAN.md)**                                     | Build plan — sprints, waves, what's next        |
| **[rules/](.claude/rules/)**                                       | Coding standards and conventions                |

### Reference

| Document                                                                        | Purpose                                    |
| ------------------------------------------------------------------------------- | ------------------------------------------ |
| [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md)         | Full architecture, tech decisions, phases   |
| [monorepo-structure.md](.claude/strategy/monorepo-structure.md)                 | Repo structure, API abstractions, packages  |
| [monorepo-tooling.md](.claude/decisions/monorepo-tooling.md)                    | Taskfile + Turborepo decision rationale     |
| [skills/](.claude/skills/)                                                      | Agent skills (pre-commit, pickup, code-review) |

---

## Agent Workflow

When starting work:

1. **Read context** - Review this file, rules/, and relevant docs in `.claude/`
2. **Check the plan** - See where we are in [PLAN.md](.claude/PLAN.md)
3. **Claim a task** - Mark it CLAIMED in PLAN.md before starting
4. **Follow patterns** - Match existing code style and architecture (see rules/)
5. **Test boundaries** - Write tests for engine logic and API contracts
6. **Mark done** - Update the plan when task is complete
7. **Document decisions** - Add notes to `.claude/decisions/` for significant choices

### Quality Checklist

Before completing any task, verify:

- [ ] Follows layered architecture (no layer violations)
- [ ] Bento Box Principle respected (single responsibility, no grab bags)
- [ ] Go: files < 250 lines, functions < 20 lines
- [ ] API calls go through `@bnto/core` (never direct Convex/Wails calls)
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
  - Apps → @bnto/core → Go Engine
  - No layer skipping (components calling Convex/Wails directly)
  - UI and editor components co-located in `apps/web/` (not in separate packages yet)

- [ ] **API Abstraction**: Are all data operations going through `@bnto/core`?
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
  - See [code-standards.md](.claude/rules/code-standards.md)

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
  - Import types from `@bnto/core`, not re-exports
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
  - **Go integration tests**: End-to-end workflow execution with fixture .bnto.json files
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

1. **TDD is the core of our success** - Predefined Bnto fixtures run against the CLI. Every node ships with tests. If you can't test it, you can't ship it.
2. **Go with the grain** - Don't fight your environment. Intuitive software comes from working with tools the way they want to be used.
3. **Modularity is our bread and butter** - Think small, build small, compose big. One node, one job. Layers own their domain.
4. **Abstraction is the goal** - "Did we make this easier?" If no, go back to the drawing board. Both UX and DX.
5. **Transport-agnostic API** - All data flows through `@bnto/core`, never direct backend calls
6. **CLI is the stable API** - Every operation maps to a CLI command; the Go engine is the source of truth
7. **Open source core** - Cloud sells convenience (hosting, managed infrastructure), not proprietary features

See [core-principles.md](.claude/strategy/core-principles.md) for the full treatment of each.
