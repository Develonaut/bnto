# Bnto - Agent & Developer Guide

**Last Updated:** March 14, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...        | Read this first                                                         |
| ------------------------------ | ----------------------------------------------------------------------- |
| Any code                       | [code-standards.md](.claude/rules/code-standards.md)                    |
| Any UI / styling work          | [rules/theming.md](.claude/rules/theming.md)                            |
| Data fetching / hooks          | [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md) |
| Architecture decisions         | [rules/architecture.md](.claude/rules/architecture.md)                  |
| Editor architecture            | [editor-architecture.md](.claude/strategy/editor-architecture.md)       |
| Editor API layer (Sprint 5D)   | [editor-api.md](.claude/strategy/editor-api.md)                         |
| Visual editor                  | [visual-editor.md](.claude/strategy/visual-editor.md)                   |
| Input/output nodes             | [io-nodes.md](.claude/strategy/io-nodes.md)                             |
| Node system responsibilities   | [node-responsibilities.md](.claude/rules/node-responsibilities.md)      |
| Engine execution / pipeline    | [engine-execution.md](.claude/strategy/engine-execution.md)             |
| Editor user journey            | [editor-user-journey.md](.claude/strategy/editor-user-journey.md)       |
| Strategic direction            | [ROADMAP.md](.claude/ROADMAP.md)                                        |
| Implementation task            | [PLAN.md](.claude/PLAN.md)                                              |
| Free vs premium decisions      | [pricing-model.md](.claude/strategy/pricing-model.md)                   |
| Writing integration tests      | [journeys/](.claude/journeys/) — user journey test matrices             |
| Predefined recipes & SEO slugs | [strategy/bntos.md](.claude/strategy/bntos.md)                          |
| SEO & URL strategy             | [rules/seo.md](.claude/rules/seo.md)                                    |
| Code editor (Sprint 4B)        | [code-editor.md](.claude/strategy/code-editor.md)                       |
| Understanding the product      | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) |
| Core principles (always)       | [core-principles.md](.claude/strategy/core-principles.md)               |
| `@bnto/core` internals         | [core-api.md](.claude/rules/core-api.md)                                |
| Environment variables          | [environment-variables.md](.claude/environment-variables.md)            |
| Expression input UX            | [expression-input-ux.md](.claude/strategy/expression-input-ux.md)       |
| Config panel controls          | [config-controls.md](.claude/strategy/config-controls.md)               |
| Releases & versioning          | [releases.md](.claude/rules/releases.md)                                |

---

## Quick Context

**Bnto** is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script. Simple by default, powerful when you need it.

Recipes are defined as `.bnto.json` files that compose nodes into pipelines. **M1 (Browser Execution) is delivered** — all 6 Tier 1 recipes run 100% client-side via Rust→WASM. Files never leave the user's machine. The dividing line: **browser nodes are free, server nodes are Pro.** See [ROADMAP.md](.claude/ROADMAP.md) and [pricing-model.md](.claude/strategy/pricing-model.md).

- **Rust WASM Engine (M1, delivered)**: `engine/` — Rust→WASM via `wasm-pack`, all browser execution
- **Web**: Next.js on Vercel + Convex Cloud + `@convex-dev/auth`
- **Cloud (M4, planned)**: Server-side execution for premium recipes (technology TBD)
- **Desktop (M3)**: Tauri (Rust-native) — free local execution
- **Shared Packages**: `@bnto/core` (transport-agnostic API), `@bnto/auth` (auth), `@bnto/backend` (Convex), `@bnto/nodes` (engine-agnostic node definitions)
- **Open Source**: MIT licensed

---

## Critical Rules (Summary)

These are enforced in detail by the [rules/](.claude/rules/) files. This section is the quick reference.

1. **Layered Architecture:** `Apps → @bnto/core → Engine (Rust WASM)`. Never skip layers. See [architecture.md](.claude/rules/architecture.md).
2. **API Abstraction:** UI code NEVER calls Convex, Tauri, or Go directly. Always through `@bnto/core` hooks.
3. **Bento Box Principle:** One thing per file/function/package. Files < 250 lines, functions < 20 lines. No `utils.ts` or `helpers.go` grab bags. See [code-standards.md](.claude/rules/code-standards.md).
4. **Co-location:** UI components live in `apps/web` until a second consumer (desktop) exists. When extracted, UI becomes `@bnto/ui` (officially named **Motorway** — the Mini Motorways design system).
5. **Transport-agnostic:** `@bnto/core` detects runtime (browser vs Tauri) and swaps adapters. Components never know which backend they're talking to.

---

## Rust Code Standards

**Comment what's non-obvious, not what's routine.** Rust code should be well-commented but not tutorial-style. The reader is assumed to know basic Rust syntax — don't explain `match`, `unwrap()`, `Vec`, `Option`, `Result`, `impl`, or standard library patterns.

**What to comment:**

- File-level purpose header (1-3 lines explaining what this module does and why it exists)
- Section separators for logical groupings (`// --- Progress Events ---`)
- Non-obvious design decisions and trade-offs ("why this approach, not that one")
- Domain-specific knowledge (business rules, format specs, algorithm choices)
- Genuinely tricky Rust patterns (lifetime tricks, unsafe blocks, complex trait bounds, macro internals)
- `///` doc comments on all public items (structs, enums, traits, functions)

**What NOT to comment:**

- Standard Rust patterns (`match`, `?` operator, `Option`/`Result` handling, iterators)
- What a line of code does when the code is self-evident
- "RUST CONCEPT:" tutorial blocks explaining language fundamentals
- Verbose "WHAT IS THIS FILE?" / "WHY IS THIS A SEPARATE FILE?" headers — use a concise 1-3 line module doc instead
- Serde attributes (`#[serde(rename_all)]`, `#[derive(Deserialize)]`) — these are standard and self-documenting

This applies to all code in `engine/` (Rust WASM) and any other `.rs` files in the repo.

**Rust/WASM is TDD-first.** Since we can't visually inspect WASM output the way we can with UI components, tests are our primary verification tool. Every Rust function, trait implementation, and WASM export must have corresponding tests BEFORE being used in production code. The testing layers are:

1. **Unit tests** (in `#[cfg(test)]` blocks) — test pure Rust logic natively. Fast, no JS runtime needed.
2. **WASM integration tests** (in `tests/` directory via `wasm-bindgen-test`) — test the Rust ↔ JS boundary. Run in Node.js or a real browser.
3. **E2E tests** (Playwright) — test the full pipeline: Web Worker loads WASM, processes a file, returns results to the UI.

Write tests at every layer. If a function can be tested as pure Rust (no WASM boundary), write a unit test. If it crosses the JS boundary, add a WASM integration test. If it's visible in the UI, add a Playwright E2E test.

---

## Commands

```bash
# Rust WASM engine (via Taskfile)
task wasm:build         # Build WASM crates (release, web target)
task wasm:build:dev     # Build WASM in dev mode (faster, better errors)
task wasm:test          # Run Rust unit tests + WASM integration tests
task wasm:test:unit     # Run Rust unit tests only (fast, native)
task wasm:lint          # Run clippy (Rust linter)
task wasm:fmt           # Auto-format Rust code
task wasm:fmt:check     # Check Rust formatting (CI)
task wasm:bench         # Run Criterion benchmarks (results in engine/target/criterion/)
task wasm:clean         # Clean Rust build artifacts

# Frontend (via Turborepo)
task ui:build           # Build all TS packages (with Turbo caching)
task ui:test            # Run all TS tests
task ui:dev             # Start web app dev server
task ui:lint            # Lint all TS packages

# Development (starts everything)
task dev                # Start web + Convex dev servers (Next.js on port 4000 + Convex)

# E2E tests
# IMPORTANT: E2E tests need a running dev server on port 4000.
#   lsof -ti:4000  (if output, dev server is running — use task e2e directly)
#   If nothing running, start one: task dev (background it, wait ~10s for startup)
task e2e                # Run all E2E tests (browser parallel, then editor serial)
task e2e:browser        # Run non-editor tests in parallel
task e2e:editor         # Run editor tests serially (avoids ReactFlow flakiness)

# Updating screenshots (run from apps/web/):
#   cd apps/web && pnpm exec playwright test --update-snapshots   # regenerate
#   cd apps/web && pnpm exec playwright test                      # verify stable
# Both runs required.

# Everything
task build:all          # Build Rust + TypeScript
task test:all           # Test Rust + TypeScript
task check              # Full quality gate (lint + test + build)
```

---

## Repository Structure

```
bnto/
├── apps/
│   ├── web/                     # Next.js on Vercel
│   └── desktop/                 # Tauri frontend (M3, planned)
├── packages/
│   ├── core/                    # @bnto/core — Transport-agnostic API
│   ├── ui/                      # @bnto/ui — Motorway design system
│   ├── editor/                  # @bnto/editor — Recipe editor
│   └── @bnto/
│       ├── auth/                # @bnto/auth — Cloud auth (web only)
│       ├── backend/             # @bnto/backend — Convex schema + functions
│       └── nodes/               # @bnto/nodes — Engine-agnostic node definitions
├── engine/                      # Rust WASM engine (browser execution)
│   └── crates/
│       ├── bnto-core/           # Core WASM library (types, traits, progress)
│       ├── bnto-image/          # Image compression/resize/convert
│       ├── bnto-csv/            # CSV clean/rename columns
│       ├── bnto-file/           # File rename
│       └── bnto-wasm/           # cdylib entry point (single WASM binary)
├── test-fixtures/               # Shared test assets (images, CSVs)
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

| Document                                                           | Purpose                                                     |
| ------------------------------------------------------------------ | ----------------------------------------------------------- |
| [code-standards.md](.claude/rules/code-standards.md)               | Bento Box Principle, size limits, file organization         |
| [architecture.md](.claude/rules/architecture.md)                   | Layered architecture, data flow, deployment topology        |
| [components.md](.claude/rules/components.md)                       | Component patterns, hooks, flat exports, CSS-first states   |
| [theming.md](.claude/rules/theming.md)                             | Color tokens, fonts, radius, shadows                        |
| [animation.md](.claude/rules/animation.md)                         | Motion language, CSS animation system, animation components |
| [seo.md](.claude/rules/seo.md)                                     | URL strategy, slug registry, metadata, shipping checklist   |
| [pre-commit.md](.claude/rules/pre-commit.md)                       | Mandatory checklist before every commit                     |
| [core-api.md](.claude/rules/core-api.md)                           | @bnto/core client/service/adapter pattern                   |
| [auth-routing.md](.claude/rules/auth-routing.md)                   | Proxy route protection, auth flow                           |
| [convex.md](.claude/rules/convex.md)                               | Query patterns, validators, N+1 prevention                  |
| [node-responsibilities.md](.claude/rules/node-responsibilities.md) | Engine / @bnto/nodes / Editor responsibility matrix         |
| [gotchas.md](.claude/rules/gotchas.md)                             | Known pitfalls and fixes                                    |

### Strategy & Reference (read on demand)

| Document                                                                | Purpose                                                                                                           |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| [ROADMAP.md](.claude/ROADMAP.md)                                        | Strategic roadmap — milestones, direction, big decisions                                                          |
| [PLAN.md](.claude/PLAN.md)                                              | Build plan — sprints, waves, what's next                                                                          |
| [pricing-model.md](.claude/strategy/pricing-model.md)                   | Free vs premium — nodes, recipes, features, terminology                                                           |
| [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md) | Hybrid Convex native + React Query, co-located queries, self-fetching components                                  |
| [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) | Full architecture, tech decisions, phases                                                                         |
| [core-principles.md](.claude/strategy/core-principles.md)               | TDD, Grain, Modularity, Abstraction                                                                               |
| [design-language.md](.claude/strategy/design-language.md)               | Visual identity, brand personality                                                                                |
| [editor-architecture.md](.claude/strategy/editor-architecture.md)       | Shared editor layer — store, hooks, package strategy, switchable editors                                          |
| [editor-api.md](.claude/strategy/editor-api.md)                         | Editor API layer — client → service → store abstraction, Sprint 5D                                                |
| [visual-editor.md](.claude/strategy/visual-editor.md)                   | Bento box visual editor — compartment design, grid layout, execution state                                        |
| [io-nodes.md](.claude/strategy/io-nodes.md)                             | Input & output nodes — self-describing recipe I/O, generic renderers, migration                                   |
| [editor-user-journey.md](.claude/strategy/editor-user-journey.md)       | Editor user journey — stages, flows, success criteria, phased delivery                                            |
| [code-editor.md](.claude/strategy/code-editor.md)                       | Code editor design — CM6, slash commands, JSON Schema                                                             |
| [engine-execution.md](.claude/strategy/engine-execution.md)             | Engine execution architecture — pipeline executor, progress events, multi-consumer                                |
| [expression-input-ux.md](.claude/strategy/expression-input-ux.md)       | Expression input UX — pill tokens, variable picker, competitor analysis, phased rollout                           |
| Private business docs (see `BNTO_PRIVATE_DOCS_PATH` in `.env.local`)    | Pricing strategy, revenue projections, SEO monetization, feature funnel, brand, personas, competitive positioning |
| [skills/](.claude/skills/)                                              | Agent skills (pre-commit, pickup, code-review, merge-pr, lighthouse-audit)                                        |

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
| Security Engineer  | Cross-cutting — trust boundaries, attack surfaces, defense-in-depth                                               | `/security-engineer`  |
| Quality Engineer   | `apps/web/e2e/`, `.claude/journeys/` — E2E testing, journey design, screenshot regression, test infrastructure    | `/quality-engineer`   |
| Workflow Expert    | Recipe design, competitive analysis, multi-node compositions, custom recipe journey tests                         | `/workflow-expert`    |
| Technical Writer   | Package READMEs — accuracy audits, structural documentation, staleness prevention                                 | `/technical-writer`   |

| Project Manager | `.claude/PLAN.md`, `.claude/ROADMAP.md` — roadmap alignment, sprint planning | `/project-manager` |

The `/groom` workflow skill invokes `/project-manager` automatically to run a full plan review. The `/code-review` and `/pre-commit` skills invoke `/technical-writer` when changes affect package structure or public API.
