# Bnto - Agent & Developer Guide

**Last Updated:** April 9, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...        | Read this first                                                                                |
| ------------------------------ | ---------------------------------------------------------------------------------------------- |
| Any code                       | [code-standards.md](.claude/rules/code-standards.md)                                           |
| Any UI / styling work          | [rules/theming.md](.claude/rules/theming.md)                                                   |
| Landing page / homepage        | [landing-page-inspiration.md](.claude/strategy/landing-page-inspiration.md)                    |
| Homepage sprint plan           | [homepage-sprint-plan.md](.claude/strategy/homepage-sprint-plan.md)                            |
| Brand, messaging & mascots     | [brand-messaging-audit.md](.claude/strategy/brand-messaging-audit.md)                          |
| Data fetching / hooks          | [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md)                        |
| Architecture decisions         | [rules/architecture.md](.claude/rules/architecture.md)                                         |
| Editor architecture (frozen)   | [editor-architecture.md](.claude/strategy/editor-architecture.md)                              |
| Editor API layer (frozen)      | [editor-api.md](.claude/strategy/editor-api.md)                                                |
| Visual editor (frozen)         | [visual-editor.md](.claude/strategy/visual-editor.md)                                          |
| Input/output nodes (frozen)    | [io-nodes.md](.claude/strategy/io-nodes.md)                                                    |
| Node system responsibilities   | [node-responsibilities.md](.claude/rules/node-responsibilities.md)                             |
| Adding a new engine node       | [engine-node-patterns.md](.claude/rules/engine-node-patterns.md)                               |
| Adding a new recipe            | [engine-node-patterns.md](.claude/rules/engine-node-patterns.md#checklist-adding-a-new-recipe) |
| Planning multi-PR features     | [feature-planning.md](.claude/rules/feature-planning.md)                                       |
| Engine execution / pipeline    | [engine-execution.md](.claude/strategy/engine-execution.md)                                    |
| Engine expansion strategy      | [engine-expansion.md](.claude/strategy/engine-expansion.md)                                    |
| TUI strategy & design          | [tui-strategy.md](.claude/strategy/tui-strategy.md)                                            |
| Smart iteration / auto-looping | [smart-iteration.md](.claude/strategy/smart-iteration.md)                                      |
| Editor user journey (frozen)   | [editor-user-journey.md](.claude/strategy/editor-user-journey.md)                              |
| Strategic direction            | [ROADMAP.md](.claude/ROADMAP.md)                                                               |
| Implementation task            | [PLAN.md](.claude/PLAN.md)                                                                     |
| Free vs premium decisions      | [pricing-model.md](.claude/strategy/pricing-model.md)                                          |
| Writing integration tests      | [journeys/](.claude/journeys/) — user journey test matrices                                    |
| Predefined recipes & SEO slugs | [strategy/bntos.md](.claude/strategy/bntos.md)                                                 |
| SEO & URL strategy             | [rules/seo.md](.claude/rules/seo.md)                                                           |
| Code editor (frozen)           | [code-editor.md](.claude/strategy/code-editor.md)                                              |
| Understanding the product      | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md)                        |
| Core principles (always)       | [core-principles.md](.claude/strategy/core-principles.md)                                      |
| `@bnto/core` internals         | [core-api.md](.claude/rules/core-api.md)                                                       |
| Environment variables          | [environment-variables.md](.claude/environment-variables.md)                                   |
| Expression input UX (frozen)   | [expression-input-ux.md](.claude/strategy/expression-input-ux.md)                              |
| Config panel controls (frozen) | [config-controls.md](.claude/strategy/config-controls.md)                                      |
| Feature flags & A/B testing    | [feature-flags.md](.claude/rules/feature-flags.md)                                             |
| Releases & versioning          | [releases.md](.claude/rules/releases.md)                                                       |

---

## Quick Context

**Bnto** is workflow automation through composable parts. Each node encapsulates a single capability — compress an image, call an API, run a shell command, download a video. Chain nodes into recipes that automate your workflow. One Rust engine compiles to every target: CLI, browser (WASM), desktop (Tauri), server. Write a recipe once, run it anywhere.

Recipes are defined as `.bnto.json` files that compose nodes into pipelines. **15 recipes ship today** — running via CLI (native Rust) and browser (Rust→WASM). The CLI is the primary development surface. The dividing line: **nodes that run locally are free, nodes that need a managed server cost money** (monetization tabled). See [ROADMAP.md](.claude/ROADMAP.md).

- **CLI** (primary): `engine/crates/bnto/` — native Rust binary, full system access, published to crates.io
- **Rust Engine**: `engine/` — shared engine crate powering CLI (native) and browser (WASM)
- **Web**: Next.js on Vercel + Convex Cloud — landing page, docs, predefined recipe pages for SEO, browser execution
- **Desktop** (M4, backlog): Tauri (Rust-native) — links engine natively like CLI
- **Cloud** (M4, backlog): Server-side execution for premium recipes (technology TBD)
- **Shared Packages**: `@bnto/core` (transport-agnostic web API), `@bnto/registry` (node system facade + curation), `@bnto/auth` (auth), `@bnto/backend` (Convex), `@bnto/nodes` (engine-generated catalog, internal to registry)
- **Open Source**: MIT licensed

---

## Critical Rules (Summary)

These are enforced in detail by the [rules/](.claude/rules/) files. This section is the quick reference.

1. **Layered Architecture:** CLI links engine directly. Web: `Apps → @bnto/core → Engine (Rust→WASM)`. Never skip layers. See [architecture.md](.claude/rules/architecture.md).
2. **API Abstraction:** UI code NEVER calls Convex, Tauri, or Go directly. Always through `@bnto/core` hooks.
3. **Bento Box Principle:** One thing per file/function/package. Files < 250 lines, functions < 20 lines. No `utils.ts` or `helpers.go` grab bags. See [code-standards.md](.claude/rules/code-standards.md).
4. **Co-location:** `@bnto/ui` (Motorway design system) and `@bnto/editor` extracted as packages (March 2026). Page-level components remain in `apps/web`.
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

**TDD Red — tests are the design phase.** Every feature starts with failing (Red) tests that define what the code should do. Tests are not verification after the fact — they are the design tool. Write Red tests first, then implement the minimum code to make them Green, then Refactor. This applies to all code: Rust, TypeScript, UI components.

**Why Red first:** Failing tests force you to think about the API, contracts, edge cases, and error paths before getting lost in implementation. The test suite becomes the executable specification — when all Red tests turn Green, the feature is done.

```
1. RED    — Write a failing test that defines one behavior
2. GREEN  — Write the minimum code to make it pass
3. REFACTOR — Clean up while tests stay green
4. REPEAT — Next behavior, next Red test
```

**Rust/WASM is especially TDD-first.** Since we can't visually inspect WASM output the way we can with UI components, tests are our primary verification tool. Every Rust function, trait implementation, and WASM export must have corresponding tests BEFORE being used in production code. The testing layers are:

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

# Native CLI (via Taskfile)
task cli:build          # Build native CLI binary (release)
task cli:test           # Run CLI unit + integration + golden tests
task cli:golden         # Run golden tests only (byte-exact output verification)
task cli:golden:bless   # Regenerate golden files from current CLI output

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
│       ├── nodes/               # @bnto/nodes — Engine-agnostic node definitions
│       └── registry/            # @bnto/registry — Curation + discovery layer
├── engine/                      # Rust engine (CLI + WASM + native)
│   ├── recipes/                 # Authoritative .bnto.json recipe definitions (15 files)
│   ├── crates/
│   │   ├── bnto-core/           # Core library (types, traits, progress)
│   │   ├── bnto-image/          # Image compression/resize/convert
│   │   ├── bnto-csv/            # CSV clean/rename columns
│   │   ├── bnto-file/           # File rename
│   │   ├── bnto-video/          # Video download (yt-dlp, native-only)
│   │   ├── bnto-engine/         # Shared registry + pipeline runner + recipe catalog
│   │   ├── bnto-wasm/           # cdylib entry point (WASM binary)
│   │   └── bnto/                # Native CLI binary (`bnto`) — primary consumer
├── test-fixtures/               # Shared test assets (images, CSVs)
└── .claude/                     # Strategy docs, decisions, plan, rules
```

---

## Agent Workflow

1. **Read context** — Review this file, rules/, and relevant docs
2. **Check the plan** — See [PLAN.md](.claude/PLAN.md) for current sprint
3. **Claim a task** — Mark it CLAIMED before starting
4. **Plan multi-PR work** — If the task spans 2+ PRs, produce a structured plan per [feature-planning.md](.claude/rules/feature-planning.md) before writing code. Present the plan for approval
5. **Create a branch** — `git checkout -b <type>/<short-description>`. Never commit directly to `main`
6. **Follow patterns** — Match existing code style (see rules/)
7. **Test boundaries** — Write tests for engine logic and API contracts
8. **E2E test** — If you touched UI, run `task e2e` (requires `task dev` running — Next.js + Convex on port 4000). Start it yourself if needed — never skip because "the stack isn't running"
9. **Mark done** — Update the plan when complete
10. **Pre-commit** — Follow [pre-commit.md](.claude/rules/pre-commit.md) before every commit
11. **Push & PR** — Push your branch, create a PR targeting `main`. CI Gate must pass before merge

**Branch protection:** `main` requires the CI Gate check (Rust + TypeScript) to pass via PR. Direct pushes to `main` are blocked.

---

## Key Principles

1. **TDD Red — tests are the design phase** — Write failing tests first to define what code should do, then implement to make them pass. Tests are not verification — they are the design tool
2. **Go with the grain** — Work with tools the way they want to be used
3. **Modularity is our bread and butter** — Think small, build small, compose big
4. **Abstraction is the goal** — "Did we make this easier?" If no, go back
5. **Config as code** — The repo is the source of truth. Dashboards override, never gatekeep
6. **Engine is the stable API** — CLI (native Rust), browser (WASM), desktop (Tauri native)
7. **Open source core** — Cloud sells convenience, not proprietary features

See [core-principles.md](.claude/strategy/core-principles.md) for the full treatment.

---

## Documentation Index

### Rules (auto-loaded, always active)

| Document                                                           | Purpose                                                                                  |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
| [code-standards.md](.claude/rules/code-standards.md)               | Bento Box Principle, size limits, file organization                                      |
| [architecture.md](.claude/rules/architecture.md)                   | Layered architecture, data flow, deployment topology                                     |
| [components.md](.claude/rules/components.md)                       | Component patterns, hooks, flat exports, CSS-first states                                |
| [theming.md](.claude/rules/theming.md)                             | Color tokens, fonts, radius, shadows                                                     |
| [animation.md](.claude/rules/animation.md)                         | Motion language, CSS animation system, animation components                              |
| [seo.md](.claude/rules/seo.md)                                     | URL strategy, slug registry, metadata, shipping checklist                                |
| [pre-commit.md](.claude/rules/pre-commit.md)                       | Mandatory checklist before every commit                                                  |
| [core-api.md](.claude/rules/core-api.md)                           | @bnto/core client/service/adapter pattern                                                |
| [auth-routing.md](.claude/rules/auth-routing.md)                   | Proxy route protection, auth flow                                                        |
| [convex.md](.claude/rules/convex.md)                               | Query patterns, validators, N+1 prevention                                               |
| [node-responsibilities.md](.claude/rules/node-responsibilities.md) | Engine / @bnto/nodes / Editor responsibility matrix                                      |
| [engine-node-patterns.md](.claude/rules/engine-node-patterns.md)   | Adding new nodes & recipes — full checklists, test counts, codegen, surface verification |
| [feature-planning.md](.claude/rules/feature-planning.md)           | Multi-PR feature plans — structure, RED tests, dependency chains, verification           |
| [feature-flags.md](.claude/rules/feature-flags.md)                 | Feature flags & A/B testing via PostHog + `core.flags` API                               |
| [gotchas.md](.claude/rules/gotchas.md)                             | Known pitfalls and fixes                                                                 |

### Strategy & Reference (read on demand)

| Document                                                                    | Purpose                                                                                                           |
| --------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| [ROADMAP.md](.claude/ROADMAP.md)                                            | Strategic roadmap — milestones, direction, big decisions                                                          |
| [PLAN.md](.claude/PLAN.md)                                                  | Build plan — sprints, waves, what's next                                                                          |
| [pricing-model.md](.claude/strategy/pricing-model.md)                       | Free vs premium — nodes, recipes, features, terminology                                                           |
| [data-fetching-strategy.md](.claude/strategy/data-fetching-strategy.md)     | Hybrid Convex native + React Query, co-located queries, self-fetching components                                  |
| [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md)     | Full architecture, tech decisions, phases                                                                         |
| [core-principles.md](.claude/strategy/core-principles.md)                   | TDD, Grain, Modularity, Abstraction                                                                               |
| [design-language.md](.claude/strategy/design-language.md)                   | Visual identity, brand personality                                                                                |
| [landing-page-inspiration.md](.claude/strategy/landing-page-inspiration.md) | Landing page creative direction — reference sites, hero concept, page sections                                    |
| [editor-architecture.md](.claude/strategy/editor-architecture.md)           | Shared editor layer — store, hooks, package strategy, switchable editors                                          |
| [editor-api.md](.claude/strategy/editor-api.md)                             | Editor API layer — client → service → store abstraction, Sprint 5D                                                |
| [visual-editor.md](.claude/strategy/visual-editor.md)                       | Bento box visual editor — compartment design, grid layout, execution state                                        |
| [io-nodes.md](.claude/strategy/io-nodes.md)                                 | Input & output nodes — self-describing recipe I/O, generic renderers, migration                                   |
| [editor-user-journey.md](.claude/strategy/editor-user-journey.md)           | Editor user journey — stages, flows, success criteria, phased delivery                                            |
| [code-editor.md](.claude/strategy/code-editor.md)                           | Code editor design — CM6, slash commands, JSON Schema                                                             |
| [engine-execution.md](.claude/strategy/engine-execution.md)                 | Engine execution architecture — pipeline executor, progress events, multi-consumer                                |
| [tui-strategy.md](.claude/strategy/tui-strategy.md)                         | TUI design — TEA architecture, Motorway design language, 5-system breakdown, TDD approach                         |
| [expression-input-ux.md](.claude/strategy/expression-input-ux.md)           | Expression input UX — pill tokens, variable picker, competitor analysis, phased rollout                           |
| Private business docs (see `BNTO_PRIVATE_DOCS_PATH` in `.env.local`)        | Pricing strategy, revenue projections, SEO monetization, feature funnel, brand, personas, competitive positioning |
| [skills/](.claude/skills/)                                                  | Agent skills (pre-commit, pickup, code-review, merge-pr, lighthouse-audit)                                        |

### Domain Expert Personas (invoke with `/persona-name`)

Persona skills are domain experts that can be activated to adopt specialized knowledge for a specific area of the codebase. Invoke them directly when working in their domain, or let workflow skills (`/pickup`, `/pre-commit`, `/code-review`) activate them automatically.

| Persona            | Domain                                                                                                            | Invoke                |
| ------------------ | ----------------------------------------------------------------------------------------------------------------- | --------------------- |
| Frontend Engineer  | `apps/web/` — React, Next.js, components, theming, animation, E2E                                                 | `/frontend-engineer`  |
| Next.js Expert     | `apps/web/` — App Router optimization, server/client boundaries, caching, streaming, bundle size, Core Web Vitals | `/nextjs-expert`      |
| ReactFlow Expert   | Visual editor canvas — `@xyflow/react`, graph state, custom nodes/edges, headless-first                           | `/reactflow-expert`   |
| Code Editor Expert | JSON code editor — CodeMirror 6, slash commands, schema-aware editing, headless-first                             | `/code-editor-expert` |
| Rust Expert        | `engine/` — CLI, WASM, node crates, execution engine                                                              | `/rust-expert`        |
| Core Architect     | `packages/core/` — transport-agnostic API, clients, services, adapters                                            | `/core-architect`     |
| Backend Engineer   | `packages/@bnto/backend/`, `packages/@bnto/auth/` — Convex, schema, auth                                          | `/backend-engineer`   |
| Security Engineer  | Cross-cutting — trust boundaries, attack surfaces, defense-in-depth                                               | `/security-engineer`  |
| Quality Engineer   | `apps/web/e2e/`, `.claude/journeys/` — E2E testing, journey design, screenshot regression, test infrastructure    | `/quality-engineer`   |
| Workflow Expert    | Recipe design, competitive analysis, multi-node compositions, custom recipe journey tests                         | `/workflow-expert`    |
| Technical Writer   | Package READMEs — accuracy audits, structural documentation, staleness prevention                                 | `/technical-writer`   |

| Project Manager | `.claude/PLAN.md`, `.claude/ROADMAP.md` — roadmap alignment, sprint planning | `/project-manager` |

The `/groom` workflow skill invokes `/project-manager` automatically to run a full plan review. The `/code-review` and `/pre-commit` skills invoke `/technical-writer` when changes affect package structure or public API.
