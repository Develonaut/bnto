# Bnto - Agent & Developer Guide

**Last Updated:** February 24, 2026

---

## Before You Write Any Code

**STOP.** Read the relevant documentation first.

| If you're working on...      | Read this first                                                    |
| ---------------------------- | ------------------------------------------------------------------ |
| Any code                     | [code-standards.md](.claude/rules/code-standards.md)              |
| Any UI / styling work        | [rules/theming.md](.claude/rules/theming.md)                       |
| Architecture decisions       | [rules/architecture.md](.claude/rules/architecture.md)             |
| Repo structure               | [monorepo-structure.md](.claude/strategy/monorepo-structure.md)    |
| Build tooling                | [monorepo-tooling.md](.claude/decisions/monorepo-tooling.md)       |
| Strategic direction          | [ROADMAP.md](.claude/ROADMAP.md)                                  |
| Implementation task          | [PLAN.md](.claude/PLAN.md)                                        |
| Writing integration tests    | [journeys/](.claude/journeys/) — user journey test matrices        |
| Predefined Bntos & SEO slugs | [strategy/bntos.md](.claude/strategy/bntos.md)                    |
| SEO & URL strategy           | [rules/seo.md](.claude/rules/seo.md)                              |
| Understanding the product    | [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) |
| Core principles (always)     | [core-principles.md](.claude/strategy/core-principles.md)          |
| Environment variables        | [environment-variables.md](.claude/environment-variables.md)       |

---

## Quick Context

**Bnto** is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script. Simple by default, powerful when you need it.

Workflows are defined as `.bnto.json` files that orchestrate tasks. **Browser execution is the M1 priority** — Tier 1 bntos run 100% client-side via Rust→WASM (or JS fallback). Files never leave the user's machine. Cloud execution (Go API on Railway) is built and ready for M4 (premium server-side bntos). See [ROADMAP.md](.claude/ROADMAP.md).

- **Browser Engine (M1)**: Rust→WASM via `wasm-pack` (or JS library adapters as fallback)
- **Go Engine**: CLI + cloud execution engine in `engine/` (CLI stable, cloud ready for M4)
- **Web**: Next.js on Vercel + Convex Cloud + `@convex-dev/auth`
- **Cloud (M4)**: Go API on Railway + Cloudflare R2 file transit (premium server-side bntos)
- **Desktop (M3)**: Tauri or Wails (depends on M1 Rust outcome) — free local execution
- **Shared Packages**: `@bnto/core` (transport-agnostic API), `@bnto/auth` (auth), `@bnto/backend` (Convex), `@bnto/nodes` (engine-agnostic node definitions — M1)
- **Open Source**: MIT licensed

---

## Critical Rules (Summary)

These are enforced in detail by the [rules/](.claude/rules/) files. This section is the quick reference.

1. **Layered Architecture:** `Apps → @bnto/core → Go Engine`. Never skip layers. See [architecture.md](.claude/rules/architecture.md).
2. **API Abstraction:** UI code NEVER calls Convex, Wails, or Go directly. Always through `@bnto/core` hooks.
3. **Bento Box Principle:** One thing per file/function/package. Files < 250 lines, functions < 20 lines. No `utils.ts` or `helpers.go` grab bags. See [code-standards.md](.claude/rules/code-standards.md).
4. **Co-location:** UI components live in `apps/web` until a second consumer (desktop) exists. When extracted, UI becomes `@bnto/ui` (officially named **Motorway** — the Mini Motorways design system).
5. **Transport-agnostic:** `@bnto/core` detects runtime (browser vs Wails) and swaps adapters. Components never know which backend they're talking to.

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

This applies to all code in `engine-wasm/` and any other `.rs` files in the repo.

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
task wasm:clean         # Clean Rust build artifacts

```bash
# Go engine (via Taskfile)
task build              # Build bnto CLI binary
task test               # Run engine tests with race detector
task vet                # Run go vet on all Go packages

# API server
task api:build          # Build API server binary
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

# E2E tests (requires task dev running)
task e2e                # Run all Playwright E2E tests (expects dev stack on port 4000)

# Everything
task build:all          # Build engine + API + frontend
task test:all           # Test engine + API + frontend
task check              # Full quality gate (vet + test + build)
```

---

## Repository Structure

```
bnto/
├── apps/
│   ├── api/                     # Go HTTP API server (Railway)
│   ├── web/                     # Next.js on Vercel
│   │   └── components/          # UI components (co-located, future @bnto/ui)
│   └── desktop/                 # Wails frontend (Phase 2)
├── packages/
│   ├── core/                    # @bnto/core — Transport-agnostic API
│   └── @bnto/
│       ├── auth/                # @bnto/auth — Cloud auth (web only)
│       └── backend/             # @bnto/backend — Convex schema + functions
├── engine/                      # All Go code (CLI, engine, node types)
├── engine-wasm/                 # Rust WASM engine (browser execution)
│   └── crates/
│       └── bnto-core/           # Core WASM library (types, traits, progress)
└── .claude/                     # Strategy docs, decisions, plan, rules
```

---

## Agent Workflow

1. **Read context** — Review this file, rules/, and relevant docs
2. **Check the plan** — See [PLAN.md](.claude/PLAN.md) for current sprint
3. **Claim a task** — Mark it CLAIMED before starting
4. **Follow patterns** — Match existing code style (see rules/)
5. **Test boundaries** — Write tests for engine logic and API contracts
6. **E2E test** — If you touched UI, run `task e2e` (requires `task dev` running — Next.js + Convex on port 4000). Start it yourself if needed — never skip because "the stack isn't running"
7. **Mark done** — Update the plan when complete
8. **Pre-commit** — Follow [pre-commit.md](.claude/rules/pre-commit.md) before every commit

---

## Key Principles

1. **TDD is the core of our success** — If you can't test it, you can't ship it
2. **Go with the grain** — Work with tools the way they want to be used
3. **Modularity is our bread and butter** — Think small, build small, compose big
4. **Abstraction is the goal** — "Did we make this easier?" If no, go back
5. **CLI is the stable API** — Go engine is the source of truth
6. **Open source core** — Cloud sells convenience, not proprietary features

See [core-principles.md](.claude/strategy/core-principles.md) for the full treatment.

---

## Documentation Index

### Rules (auto-loaded, always active)

| Document | Purpose |
|----------|---------|
| [code-standards.md](.claude/rules/code-standards.md) | Bento Box Principle, size limits, file organization |
| [architecture.md](.claude/rules/architecture.md) | Layered architecture, data flow, deployment topology |
| [components.md](.claude/rules/components.md) | Component patterns, hooks, dot-notation, CSS-first states |
| [theming.md](.claude/rules/theming.md) | Color tokens, fonts, radius, shadows |
| [animation.md](.claude/rules/animation.md) | Motion language, CSS animation system, Animate.* API |
| [seo.md](.claude/rules/seo.md) | URL strategy, slug registry, metadata, shipping checklist |
| [pre-commit.md](.claude/rules/pre-commit.md) | Mandatory checklist before every commit |
| [core-api.md](.claude/rules/core-api.md) | @bnto/core client/service/adapter pattern |
| [auth-routing.md](.claude/rules/auth-routing.md) | Proxy route protection, auth flow |
| [convex.md](.claude/rules/convex.md) | Query patterns, validators, N+1 prevention |
| [gotchas.md](.claude/rules/gotchas.md) | Known pitfalls and fixes |

### Strategy & Reference (read on demand)

| Document | Purpose |
|----------|---------|
| [ROADMAP.md](.claude/ROADMAP.md) | Strategic roadmap — milestones, direction, big decisions |
| [PLAN.md](.claude/PLAN.md) | Build plan — sprints, waves, what's next |
| [cloud-desktop-strategy.md](.claude/strategy/cloud-desktop-strategy.md) | Full architecture, tech decisions, phases |
| [monorepo-structure.md](.claude/strategy/monorepo-structure.md) | Repo structure, packages, CLI-to-API mapping |
| [core-principles.md](.claude/strategy/core-principles.md) | TDD, Grain, Modularity, Abstraction |
| [design-language.md](.claude/strategy/design-language.md) | Visual identity, brand personality |
| [skills/](.claude/skills/) | Agent skills (pre-commit, pickup, code-review) |
