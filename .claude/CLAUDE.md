# Bnto - Agent & Developer Guide

**Last Updated:** March 3, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...        | Read this first                                                                                        |
| ------------------------------ | ------------------------------------------------------------------------------------------------------ |
| Any code                       | [code-standards.md](.claude/rules/code-standards.md)                                                   |
| Any UI / styling work          | [rules/theming.md](.claude/rules/theming.md)                                                           |
| Data fetching / hooks          | [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md)                                |
| Architecture decisions         | [rules/architecture.md](.claude/rules/architecture.md)                                                 |
| Repo structure                 | [monorepo-structure.md](.claude/strategy/monorepo-structure.md)                                        |
| Editor architecture            | [editor-architecture.md](.claude/strategy/editor-architecture.md)                                      |
| Conveyor belt system           | [visual-editor.md](.claude/strategy/visual-editor.md)                                                  |
| Input/output nodes             | [io-nodes.md](.claude/strategy/io-nodes.md)                                                            |
| Node architecture & execution  | [node-architecture.md](.claude/strategy/node-architecture.md)                                          |
| Editor user journey            | [editor-user-journey.md](.claude/strategy/editor-user-journey.md)                                      |
| Strategic direction            | [ROADMAP.md](.claude/ROADMAP.md)                                                                       |
| Implementation task            | [PLAN.md](.claude/PLAN.md)                                                                             |
| Free vs premium decisions      | [pricing-model.md](.claude/strategy/pricing-model.md)                                                  |
| Writing integration tests      | [journeys/](.claude/journeys/) — user journey test matrices                                            |
| Predefined recipes & SEO slugs | [strategy/bntos.md](.claude/strategy/bntos.md)                                                         |
| SEO & URL strategy             | [rules/seo.md](.claude/rules/seo.md)                                                                   |
| Code editor (Sprint 4B)        | [code-editor.md](.claude/strategy/code-editor.md)                                                      |
| Understanding the product      | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md)                                |
| Core principles (always)       | [core-principles.md](.claude/strategy/core-principles.md)                                              |
| `@bnto/core` internals         | [core-api.md](.claude/rules/core-api.md) + [core-unification.md](.claude/strategy/core-unification.md) |
| Environment variables          | [environment-variables.md](.claude/environment-variables.md)                                           |
| Go engine node migration       | [go-engine-migration.md](.claude/strategy/go-engine-migration.md)                                      |

---

## Quick Context

**Bnto** is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script. Simple by default, powerful when you need it.

Recipes are defined as `.bnto.json` files that compose nodes into pipelines. **M1 (Browser Execution) is delivered** — all 6 Tier 1 recipes run 100% client-side via Rust→WASM. Files never leave the user's machine. Cloud execution (Go API on Railway) is built and ready for M4 (premium server-side recipes). The dividing line: **browser nodes are free, server nodes are Pro.** See [ROADMAP.md](.claude/ROADMAP.md) and [pricing-model.md](.claude/strategy/pricing-model.md).

- **Rust WASM Engine (M1, delivered)**: `engine/` — Rust→WASM via `wasm-pack`, all browser execution
- **Go Engine (archived)**: `archive/engine-go/` — CLI + cloud execution engine (paused, ready for M4)
- **Go API (archived)**: `archive/api-go/` — HTTP API for cloud execution on Railway (paused, ready for M4)
- **Web**: Next.js on Vercel + Convex Cloud + `@convex-dev/auth`
- **Cloud (M4)**: Go API on Railway + Cloudflare R2 file transit (premium server-side bntos)
- **Desktop (M3)**: Tauri (Rust-native) — free local execution
- **Shared Packages**: `@bnto/core` (transport-agnostic API), `@bnto/auth` (auth), `@bnto/backend` (Convex), `@bnto/nodes` (engine-agnostic node definitions)
- **Open Source**: MIT licensed

---

## Critical Rules (Summary)

These are enforced in detail by the [rules/](.claude/rules/) files. This section is the quick reference.

1. **Layered Architecture:** `Apps → @bnto/core → Go Engine`. Never skip layers. See [architecture.md](.claude/rules/architecture.md).
2. **API Abstraction:** UI code NEVER calls Convex, Tauri, or Go directly. Always through `@bnto/core` hooks.
3. **Bento Box Principle:** One thing per file/function/package. Files < 250 lines, functions < 20 lines. No `utils.ts` or `helpers.go` grab bags. See [code-standards.md](.claude/rules/code-standards.md).
4. **Co-location:** UI components live in `apps/web` until a second consumer (desktop) exists. When extracted, UI becomes `@bnto/ui` (officially named **Motorway** — the Mini Motorways design system).
5. **Transport-agnostic:** `@bnto/core` detects runtime (browser vs Tauri) and swaps adapters. Components never know which backend they're talking to.

---

## Rust Code Standards

**All Rust code must be heavily commented for learning purposes.** Ryan is learning Rust — every `.rs` file should be written as if the comments will be read by a five-year-old. This means:

- Explain what every function does in plain English before the function
- Explain WHY each line exists, not just WHAT it does
- Explain Rust-specific concepts inline (ownership, borrowing, lifetimes, traits, etc.)
- Use analogies and simple language in comments
- Don't assume the reader knows Rust idioms — explain `unwrap()`, `?` operator, `impl`, `match`, etc.
- Comment density should be high — aim for a comment every 2-3 lines of code minimum
- Group related logic with section comments (e.g., `// --- Step 1: Read the input file ---`)

This applies to all code in `engine/` (Rust WASM) and any other `.rs` files in the repo.

**Rust/WASM is TDD-first.** Since we can't visually inspect WASM output the way we can with UI components, tests are our primary verification tool. Every Rust function, trait implementation, and WASM export must have corresponding tests BEFORE being used in production code. The testing layers are:

1. **Unit tests** (in `#[cfg(test)]` blocks) — test pure Rust logic natively. Fast, no JS runtime needed.
2. **WASM integration tests** (in `tests/` directory via `wasm-bindgen-test`) — test the Rust ↔ JS boundary. Run in Node.js or a real browser.
3. **E2E tests** (Playwright) — test the full pipeline: Web Worker loads WASM, processes a file, returns results to the UI.

Write tests at every layer. If a function can be tested as pure Rust (no WASM boundary), write a unit test. If it crosses the JS boundary, add a WASM integration test. If it's visible in the UI, add a Playwright E2E test.

---

## Commands

````bash
# Rust WASM engine (via Taskfile)
task wasm:build         # Build WASM crates (release, web target)
task wasm:build:dev     # Build WASM in dev mode (faster, better errors)
task wasm:test          # Run Rust unit tests + WASM integration tests
task wasm:test:unit     # Run Rust unit tests only (fast, native)
task wasm:lint          # Run clippy (Rust linter)
task wasm:fmt           # Auto-format Rust code
task wasm:fmt:check     # Check Rust formatting (CI)
task wasm:clean         # Clean Rust build artifacts

```bash
# Go engine (archived — via Taskfile)
task build              # Build bnto CLI binary (archive/engine-go)
task test               # Run engine tests with race detector
task vet                # Run go vet on all Go packages

# API server (archived)
task api:build          # Build API server binary (archive/api-go)
task api:test           # Run API server tests with race detector
task api:dev            # Run Go API server locally (port 8080)
task api:tunnel         # Start Cloudflare tunnel (exposes local API at api-dev.bnto.io)

# Frontend (via Turborepo)
task ui:build           # Build all TS packages (with Turbo caching)
task ui:test            # Run all TS tests
task ui:dev             # Start web app dev server
task ui:lint            # Lint all TS packages

# Development (starts everything)
task dev                # Start web + Convex dev servers (Next.js on port 4000 + Convex)
task dev:all            # Start web + Convex + Go API + Cloudflare tunnel in parallel

# E2E tests
# IMPORTANT: E2E tests need a running dev server. Check if one is running first:
#   lsof -ti:4000  (if output, dev server is running — use task e2e directly)
#   If nothing running, start one: task dev (background it, wait ~10s for startup)
task e2e                # Run Playwright E2E tests against port 4000 (reuses running dev server)
task e2e:isolated       # Starts own Next.js on port 4001 (slower — only if port 4000 is unavailable)

# Updating screenshots (run from apps/web/):
#   cd apps/web && pnpm exec playwright test --update-snapshots   # regenerate
#   cd apps/web && pnpm exec playwright test                      # verify stable
# Both runs required. If using isolated port:
#   E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test --update-snapshots
#   E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test

# Everything
task build:all          # Build engine + API + frontend
task test:all           # Test engine + API + frontend
task check              # Full quality gate (vet + test + build)
````

---

## Repository Structure

```
bnto/
├── apps/
│   ├── web/                     # Next.js on Vercel
│   │   └── components/          # UI components (co-located, future @bnto/ui)
│   └── desktop/                 # Tauri frontend (M3)
├── packages/
│   ├── core/                    # @bnto/core — Transport-agnostic API
│   └── @bnto/
│       ├── auth/                # @bnto/auth — Cloud auth (web only)
│       └── backend/             # @bnto/backend — Convex schema + functions
├── engine/                      # Rust WASM engine (browser execution — primary)
│   └── crates/
│       ├── bnto-core/           # Core WASM library (types, traits, progress)
│       ├── bnto-image/          # Image compression/resize/convert
│       ├── bnto-csv/            # CSV clean/rename columns
│       ├── bnto-file/           # File rename
│       └── bnto-wasm/           # cdylib entry point (single WASM binary)
├── archive/                     # Preserved reference code (not actively developed)
│   ├── engine-go/               # Go CLI + engine (~33K LOC)
│   └── api-go/                  # Go HTTP API server (~2.5K LOC)
└── .claude/                     # Strategy docs, decisions, plan, rules
```

---

## Agent Workflow

1. **Read context** — Review this file, rules/, and relevant docs
2. **Check the plan** — See [PLAN.md](.claude/PLAN.md) for current sprint
3. **Claim a task** — Mark it CLAIMED before starting
4. **Create a branch** — `git checkout -b <type>/<short-description>`. Never commit directly to `main`
5. **Follow patterns** — Match existing code style (see rules/)
6. **Test boundaries** — Write tests for engine logic and API contracts
7. **E2E test** — If you touched UI, run `task e2e` (requires `task dev` running — Next.js + Convex on port 4000). Start it yourself if needed — never skip because "the stack isn't running"
8. **Mark done** — Update the plan when complete
9. **Pre-commit** — Follow [pre-commit.md](.claude/rules/pre-commit.md) before every commit
10. **Push & PR** — Push your branch, create a PR targeting `main`. CI Gate must pass before merge

**Branch protection:** `main` requires the CI Gate check (Rust + TypeScript) to pass via PR. Direct pushes to `main` are blocked.

---

## Key Principles

1. **TDD is the core of our success** — If you can't test it, you can't ship it
2. **Go with the grain** — Work with tools the way they want to be used
3. **Modularity is our bread and butter** — Think small, build small, compose big
4. **Abstraction is the goal** — "Did we make this easier?" If no, go back
5. **Engine is the stable API** — Rust WASM for browser, Tauri native for desktop
6. **Open source core** — Cloud sells convenience, not proprietary features

See [core-principles.md](.claude/strategy/core-principles.md) for the full treatment.

---

## Documentation Index

### Rules (auto-loaded, always active)

| Document                                             | Purpose                                                     |
| ---------------------------------------------------- | ----------------------------------------------------------- |
| [code-standards.md](.claude/rules/code-standards.md) | Bento Box Principle, size limits, file organization         |
| [architecture.md](.claude/rules/architecture.md)     | Layered architecture, data flow, deployment topology        |
| [components.md](.claude/rules/components.md)         | Component patterns, hooks, flat exports, CSS-first states   |
| [theming.md](.claude/rules/theming.md)               | Color tokens, fonts, radius, shadows                        |
| [animation.md](.claude/rules/animation.md)           | Motion language, CSS animation system, animation components |
| [seo.md](.claude/rules/seo.md)                       | URL strategy, slug registry, metadata, shipping checklist   |
| [pre-commit.md](.claude/rules/pre-commit.md)         | Mandatory checklist before every commit                     |
| [core-api.md](.claude/rules/core-api.md)             | @bnto/core client/service/adapter pattern                   |
| [auth-routing.md](.claude/rules/auth-routing.md)     | Proxy route protection, auth flow                           |
| [convex.md](.claude/rules/convex.md)                 | Query patterns, validators, N+1 prevention                  |
| [gotchas.md](.claude/rules/gotchas.md)               | Known pitfalls and fixes                                    |

### Strategy & Reference (read on demand)

| Document                                                                | Purpose                                                                          |
| ----------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| [ROADMAP.md](.claude/ROADMAP.md)                                        | Strategic roadmap — milestones, direction, big decisions                         |
| [PLAN.md](.claude/PLAN.md)                                              | Build plan — sprints, waves, what's next                                         |
| [pricing-model.md](.claude/strategy/pricing-model.md)                   | Free vs premium — nodes, recipes, features, terminology                          |
| [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md) | Hybrid Convex native + React Query, co-located queries, self-fetching components |
| [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) | Full architecture, tech decisions, phases                                        |
| [monorepo-structure.md](.claude/strategy/monorepo-structure.md)         | Repo structure, packages, CLI-to-API mapping                                     |
| [core-principles.md](.claude/strategy/core-principles.md)               | TDD, Grain, Modularity, Abstraction                                              |
| [design-language.md](.claude/strategy/design-language.md)               | Visual identity, brand personality                                               |
| [editor-architecture.md](.claude/strategy/editor-architecture.md)       | Shared editor layer — store, hooks, package strategy, switchable editors         |
| [visual-editor.md](.claude/strategy/visual-editor.md)                   | Bento box visual editor — compartment design, grid layout, execution state       |
| [io-nodes.md](.claude/strategy/io-nodes.md)                             | Input & output nodes — self-describing recipe I/O, generic renderers, migration  |
| [editor-user-journey.md](.claude/strategy/editor-user-journey.md)       | Editor user journey — stages, flows, success criteria, phased delivery           |
| [code-editor.md](.claude/strategy/code-editor.md)                       | Code editor design — CM6, slash commands, JSON Schema                            |
| [core-unification.md](.claude/strategy/core-unification.md)             | Core API unification — patterns, rationale, copyable prompt                      |
| [go-engine-migration.md](.claude/strategy/go-engine-migration.md)       | Go engine node inventory — migration reference before archive deletion           |
| [skills/](.claude/skills/)                                              | Agent skills (pre-commit, pickup, code-review, merge-pr, lighthouse-audit)       |

### Domain Expert Personas (invoke with `/persona-name`)

Persona skills are domain experts that can be activated to adopt specialized knowledge for a specific area of the codebase. Invoke them directly when working in their domain, or let workflow skills (`/pickup`, `/pre-commit`, `/code-review`) activate them automatically.

| Persona            | Domain                                                                                                            | Invoke                |
| ------------------ | ----------------------------------------------------------------------------------------------------------------- | --------------------- |
| Frontend Engineer  | `apps/web/` — React, Next.js, components, theming, animation, E2E                                                 | `/frontend-engineer`  |
| Next.js Expert     | `apps/web/` — App Router optimization, server/client boundaries, caching, streaming, bundle size, Core Web Vitals | `/nextjs-expert`      |
| ReactFlow Expert   | Visual editor canvas — `@xyflow/react`, graph state, custom nodes/edges, headless-first                           | `/reactflow-expert`   |
| Code Editor Expert | JSON code editor — CodeMirror 6, slash commands, schema-aware editing, headless-first                             | `/code-editor-expert` |
| Rust Expert        | `engine/` — WASM, node crates, execution engine                                                                   | `/rust-expert`        |
| Core Architect     | `packages/core/` — transport-agnostic API, clients, services, adapters                                            | `/core-architect`     |
| Backend Engineer   | `packages/@bnto/backend/`, `packages/@bnto/auth/` — Convex, schema, auth                                          | `/backend-engineer`   |
| Go Engineer        | `archive/engine-go/`, `archive/api-go/` — Go engine, API server                                                   | `/go-engineer`        |
| Security Engineer  | Cross-cutting — trust boundaries, attack surfaces, defense-in-depth                                               | `/security-engineer`  |
| Quality Engineer   | `apps/web/e2e/`, `.claude/journeys/` — E2E testing, journey design, screenshot regression, test infrastructure    | `/quality-engineer`   |

| Project Manager | `.claude/PLAN.md`, `.claude/ROADMAP.md` — roadmap alignment, sprint planning | `/project-manager` |

The `/groom` workflow skill invokes `/project-manager` automatically to run a full plan review.
