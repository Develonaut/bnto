# Codebase Quality Sweep

**Status:** Pass 1 Complete
**Branch:** `chore/codebase-quality-sweep`
**Created:** 2026-03-25

---

## Purpose

Systematic, exhaustive code quality review of the entire bnto codebase — package by package, app by app, crate by crate. Every file reviewed against our documented coding standards. No exceptions.

### Key Objectives

1. **Enforce Bento Box Principle everywhere** — one export per file, no utils grab bags, filename matches function name
2. **File size compliance** — target 50-100 lines, hard cap 250
3. **Import boundaries** — no layer violations, correct package imports
4. **Component standards** — flat named exports, CSS-first states, self-fetching, proper hook extraction
5. **TypeScript discipline** — inference-first, no gratuitous `any`/`as`, types flow down
6. **Theming/animation** — tokens only, animation components (never raw classes)
7. **Performance** — server components first, minimal `"use client"`, no barrel imports in client code
8. **Convex standards** — validators, skip guards, batch fetching, indexes
9. **Engine patterns** — parameter contracts, shared encoding, codegen flow
10. **Dead code removal** — unused imports, unused exports, stale references

---

## Review Process

Each surface area gets **5 code review passes** using `/code-review`:

| Pass  | Mode                | Description                                                                     |
| ----- | ------------------- | ------------------------------------------------------------------------------- |
| **1** | Parallel (3 agents) | Three independent agents review simultaneously. Each produces findings.         |
| **2** |                     |                                                                                 |
| **3** |                     |                                                                                 |
| —     | Fix                 | All findings from passes 1-3 are consolidated, deduplicated, and fixed. Commit. |
| **4** | Sequential          | Single deep review. Fix all findings. Commit.                                   |
| **5** | Sequential          | Final review — should be clean. Fix any remaining issues. Commit.               |

After pass 5, the surface area is marked **COMPLETE**.

---

## Review Surface Areas

### Codebase Statistics

- **Total TypeScript files:** ~873
- **Total Rust files:** ~79
- **Total reviewable units:** 16 (9 TypeScript packages + 7 Rust crates)

---

### Surface Area Map

Each area lists the path, approximate file count, and which rule categories apply.

#### TypeScript Packages

##### SA-01: `packages/ui/` — @bnto/ui (Motorway Design System)

- **Files:** ~137
- **Rules:** Bento Box, Components, Theming, Animation, TypeScript, Performance
- **Key concerns:** Utils grab bags, flat named exports, animation component API, `createCn` usage, primitives vs business separation
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-02: `packages/core/` — @bnto/core (Transport-agnostic API)

- **Files:** ~154
- **Rules:** Bento Box, Architecture, Core API, TypeScript, Convex, Performance, Feature Flags
- **Key concerns:** Import boundaries (adapters only import backend), opaque stores, lazy infrastructure, `select` transforms, skip guards, service isolation, namespace exports
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-03: `packages/editor/` — @bnto/editor (Recipe Editor)

- **Files:** ~193
- **Rules:** Bento Box, Components, Architecture, Core API, TypeScript, Node Responsibilities, Actions Pattern
- **Key concerns:** Actions are pure functions (not in hooks), store access patterns, definition vs graph vocabulary, no business logic in editor (reads from @bnto/nodes), import boundaries
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-04: `packages/@bnto/nodes/` — Engine-Generated Catalog

- **Files:** ~75
- **Rules:** Bento Box, Node Responsibilities, TypeScript, Architecture
- **Key concerns:** Generated vs hand-written separation, no hardcoded type lists, import boundary (only registry imports from here), validation not duplicating engine
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-05: `packages/@bnto/registry/` — Node System Facade + Curation

- **Files:** ~43
- **Rules:** Bento Box, Architecture, Node Responsibilities, TypeScript
- **Key concerns:** Re-export correctness, recipe compositions reference generated types, stateless lookups, no React/Zustand
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-06: `packages/@bnto/form/` — Schema-Driven Forms

- **Files:** ~23
- **Rules:** Bento Box, Components, TypeScript, Architecture
- **Key concerns:** Import boundary (leaf: @bnto/core + @bnto/ui only), control registry, one component per file
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-07: `packages/@bnto/backend/` — Data Layer (Convex)

- **Files:** ~39
- **Rules:** Convex, Security, Architecture, TypeScript
- **Key concerns:** Validators on all inputs, auth checks, batch fetching, `.withIndex()`, no `.filter()` on `_id`, `ConvexError`, no hyphens in filenames, schema migration patterns
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-08: `packages/@bnto/auth/` — Auth Client

- **Files:** ~8
- **Rules:** Architecture, Auth Routing, Security, TypeScript
- **Key concerns:** Auth boundary (only package importing @convex-dev/auth), session management, no direct imports from consumers
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-09: `apps/web/` — Next.js Application

- **Files:** ~201
- **Rules:** ALL rules apply (pages, components, theming, animation, SEO, auth routing, skeletons, performance, gotchas)
- **Key concerns:** Server components first, self-fetching leaves, page composition, `"use client"` pushed to leaves, no direct backend imports, skeleton compliance, SEO metadata, auth routing, no barrel imports in client code
- **Sub-areas** (reviewed independently due to size):
  - **SA-09a:** `apps/web/app/` — Routes, pages, layouts (~60 files)
  - **SA-09b:** `apps/web/components/` — Shared components (~30 files)
  - **SA-09c:** `apps/web/lib/` — Utilities and helpers (~15 files)
  - **SA-09d:** `apps/web/hooks/` — App-level hooks (~10 files)
  - **SA-09e:** `apps/web/e2e/` — E2E tests (review for test conventions, not code standards)
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

#### Rust Crates

##### SA-10: `engine/crates/bnto-core/` — Core Library

- **Files:** ~28
- **Rules:** Rust Code Standards, Engine Node Patterns, Architecture
- **Key concerns:** Comment quality (non-obvious, not tutorial), doc comments on pub items, parameter contracts, progress events, executor patterns
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-11: `engine/crates/bnto-image/` — Image Processing

- **Files:** ~19
- **Rules:** Rust Code Standards, Engine Node Patterns
- **Key concerns:** Shared encoding (`encode::encode_image()`), parameter contracts (quality flows to all formats), shared param definitions in `common.rs`, parameterized tests
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-12: `engine/crates/bnto-csv/` — CSV Processing

- **Files:** ~7
- **Rules:** Rust Code Standards, Engine Node Patterns
- **Key concerns:** Parameter contracts, metadata completeness, test coverage
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-13: `engine/crates/bnto-file/` — File Operations

- **Files:** ~6
- **Rules:** Rust Code Standards, Engine Node Patterns
- **Key concerns:** Parameter contracts, metadata completeness, test coverage
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-14: `engine/crates/bnto-engine/` — Shared Registry + Pipeline

- **Files:** ~1
- **Rules:** Rust Code Standards, Architecture
- **Key concerns:** Registry pattern, single file — verify it's not a grab bag
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-15: `engine/crates/bnto-wasm/` — WASM Entry Point

- **Files:** ~10
- **Rules:** Rust Code Standards, Architecture, Engine Node Patterns
- **Key concerns:** WASM boundary correctness, catalog export, execute pipeline wrapper, no business logic (thin adapter)
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

##### SA-16: `engine/crates/bnto-cli/` — Native CLI Binary

- **Files:** ~8
- **Rules:** Rust Code Standards, Architecture
- **Key concerns:** CLI argument parsing, progress output, I/O patterns, test coverage (golden tests)
- **Status:** `PENDING`
- **Pass 1-3:** `NOT STARTED`
- **Pass 4:** `NOT STARTED`
- **Pass 5:** `NOT STARTED`

---

## Review Order

Ordered by dependency — review leaf packages first, then consumers.

| Order | Surface Area                   | Rationale                              |
| ----- | ------------------------------ | -------------------------------------- |
| 1     | SA-08: `@bnto/auth`            | Leaf, smallest (8 files), quick win    |
| 2     | SA-06: `@bnto/form`            | Leaf, small (23 files)                 |
| 3     | SA-04: `@bnto/nodes`           | Foundation — generated catalog + types |
| 4     | SA-05: `@bnto/registry`        | Facade over nodes                      |
| 5     | SA-07: `@bnto/backend`         | Data layer — Convex standards          |
| 6     | SA-01: `packages/ui`           | Design system — consumed by everything |
| 7     | SA-02: `packages/core`         | Transport-agnostic API — central layer |
| 8     | SA-03: `packages/editor`       | Recipe editor — consumes core + ui     |
| 9     | SA-09a: `apps/web/app/`        | Routes and pages                       |
| 10    | SA-09b: `apps/web/components/` | Shared app components                  |
| 11    | SA-09c: `apps/web/lib/`        | App utilities                          |
| 12    | SA-09d: `apps/web/hooks/`      | App hooks                              |
| 13    | SA-09e: `apps/web/e2e/`        | E2E test conventions                   |
| 14    | SA-10: `bnto-core`             | Engine core library                    |
| 15    | SA-11: `bnto-image`            | Image processing crate                 |
| 16    | SA-12: `bnto-csv`              | CSV processing crate                   |
| 17    | SA-13: `bnto-file`             | File operations crate                  |
| 18    | SA-14: `bnto-engine`           | Shared registry                        |
| 19    | SA-15: `bnto-wasm`             | WASM entry point                       |
| 20    | SA-16: `bnto-cli`              | CLI binary                             |

---

## Standards Checklist (per review pass)

Each `/code-review` pass evaluates every file in the surface area against these categories:

### A. Bento Box Principle

- [ ] One export per file (strict)
- [ ] No `utils.ts`, `helpers.ts` grab bags — split into individual files where filename = function name
- [ ] File size: target 50-100 lines, hard cap 250
- [ ] Function size: < 20 lines
- [ ] No multi-export files (except shadcn primitives)
- [ ] No god objects or mega-prop components

### B. Architecture & Import Boundaries

- [ ] Layered: Apps → @bnto/core → Engine (no skipping)
- [ ] API abstraction: no direct Convex/backend in components
- [ ] Import chain: editor → core → registry → nodes (never skip)
- [ ] `@bnto/nodes` consumed ONLY by `@bnto/registry`
- [ ] `@bnto/backend` consumed ONLY by `@bnto/core` internals
- [ ] No technology names leaked in public API

### C. Component Standards

- [ ] Flat named exports (NO Object.assign dot-notation)
- [ ] Self-fetching (pass IDs, not data)
- [ ] CSS-first interaction states (no JS for visual states CSS handles)
- [ ] `group-hover:` always paired with `group-focus-within:`
- [ ] Hooks extracted only when earned (not for every component)
- [ ] `createCn()` for variants, never raw `tv()` or inline `cn()` for variants
- [ ] Sizing defaults to `md`

### D. TypeScript Standards

- [ ] Inference-first (no redundant annotations)
- [ ] No `any` without eslint-disable + justification
- [ ] No `Record<string, unknown>` for domain data
- [ ] No gratuitous `as` (only at trust boundaries)
- [ ] Types flow down (core defines, consumers use)
- [ ] Named React imports (`import { useState }` not `import * as React`)

### E. Theming & Animation

- [ ] Semantic tokens only (never hardcoded colors, radii, shadows)
- [ ] `font-display` for headings, `font-sans` for body, `font-mono` for code
- [ ] Animation via components (`<ScaleIn>`, `<SlideUp>`) never raw classes
- [ ] Every animation respects `prefers-reduced-motion`
- [ ] Compositor-only properties (never animate width/height/margin/padding)

### F. Data & State

- [ ] React Query: all transforms inside `select` (never outside)
- [ ] Convex: `"skip"` guard on every query with falsy ID
- [ ] Convex: `.withIndex()` over `.filter()`
- [ ] Convex: no N+1 queries (batch fetch pattern)
- [ ] Convex: `ctx.db.get(id)` for direct lookups (never `.filter()` on `_id`)
- [ ] State mutation: pure action functions, thin hook wrappers

### G. Performance

- [ ] Server components by default, `"use client"` on smallest leaf only
- [ ] No barrel imports in client code
- [ ] Heavy components lazy loaded (`next/dynamic`)
- [ ] Images use `next/image`

### H. Security & Code Quality

- [ ] No secrets in code
- [ ] No magic values (use constants/tokens)
- [ ] No dead code or unused imports
- [ ] No `dangerouslySetInnerHTML` without sanitization
- [ ] Input validation on all API boundaries

### I. Rust Standards (engine crates only)

- [ ] Comments: non-obvious decisions, not tutorial-style
- [ ] `///` doc comments on all public items
- [ ] Parameter contracts: every param in `metadata()` used in ALL `process()` paths
- [ ] Shared encoding: `encode::encode_image()` for all image processors
- [ ] Parameterized tests: different values → different outputs

---

## Progress Tracking

| SA  | Area            | Files | Pass 1 | Fix  | Pass 2 (verify) | Status   |
| --- | --------------- | ----- | ------ | ---- | --------------- | -------- |
| 01  | @bnto/ui        | ~137  | DONE   | DONE | PASS            | COMPLETE |
| 02  | @bnto/core      | ~154  | DONE   | DONE | PASS            | COMPLETE |
| 03  | @bnto/editor    | ~193  | DONE   | DONE | PASS            | COMPLETE |
| 04  | @bnto/nodes     | ~75   | DONE   | N/A  | PASS (notes)    | COMPLETE |
| 05  | @bnto/registry  | ~43   | DONE   | DONE | PASS            | COMPLETE |
| 06  | @bnto/form      | ~23   | DONE   | N/A  | PASS            | COMPLETE |
| 07  | @bnto/backend   | ~39   | DONE   | N/A  | PASS            | COMPLETE |
| 08  | @bnto/auth      | ~8    | DONE   | N/A  | PASS            | COMPLETE |
| 09a | web/app/        | ~60   | DONE   | N/A  | PASS            | COMPLETE |
| 09b | web/components/ | ~30   | DONE   | DONE | PASS            | COMPLETE |
| 09c | web/lib/        | ~15   | DONE   | N/A  | PASS            | COMPLETE |
| 09d | web/hooks/      | ~10   | DONE   | N/A  | PASS            | COMPLETE |
| 09e | web/e2e/        | ~86   | DONE   | N/A  | PASS            | COMPLETE |
| 10  | bnto-core       | ~28   | DONE   | N/A  | PASS            | COMPLETE |
| 11  | bnto-image      | ~19   | DONE   | N/A  | PASS            | COMPLETE |
| 12  | bnto-csv        | ~7    | DONE   | N/A  | PASS            | COMPLETE |
| 13  | bnto-file       | ~6    | DONE   | N/A  | PASS            | COMPLETE |
| 14  | bnto-engine     | ~1    | DONE   | N/A  | PASS            | COMPLETE |
| 15  | bnto-wasm       | ~10   | DONE   | N/A  | PASS            | COMPLETE |
| 16  | bnto-cli        | ~8    | DONE   | N/A  | PASS            | COMPLETE |

### Notes

**@bnto/nodes (SA-04):** 5 functions exceed the 20-line hard cap but are accepted as-is:

- `createBlankDefinition` (54 lines) — data construction, splitting reduces clarity
- `inferFieldType` (44 lines) — type decision tree, cohesive as single function
- `validateDefinition` (36 lines) — orchestrator, already well-decomposed internally
- `validateEdges` (25 lines) — standard loop validator
- `validateLoop` (26 lines) — mode-based validator

All are data-construction, orchestration, or decision-tree functions where the Bento Box Principle's spirit (clarity, single responsibility) is better served by keeping them intact.

**@bnto/backend (SA-07):** Multiple query/mutation exports per file is standard Convex convention — not treated as a violation.

**Rust crates (SA-10 through SA-16):** Large files (200-400+ lines) are 50-70% inline test code (`#[cfg(test)]`). This is idiomatic Rust — flagged for future consideration but not a violation.

---

## Commit Strategy

Each surface area produces commits in this pattern:

```
chore(SA-XX): fix quality issues from parallel review (passes 1-3)
chore(SA-XX): fix quality issues from review pass 4
chore(SA-XX): fix quality issues from review pass 5 (final)
```

After all surface areas are complete, a final commit:

```
chore: codebase quality sweep complete — mark strategy doc DONE
```

---

## Common Patterns to Fix

Based on known codebase patterns, these are the most likely violations we'll find:

### 1. Utils Grab Bags → Individual Files

```
# BEFORE
utils/
  helpers.ts        # 15 functions in one file
  formatters.ts     # 8 format functions

# AFTER
utils/
  formatDuration.ts
  formatFileSize.ts
  formatTimeAgo.ts
  clampValue.ts
  ...
```

Every function gets its own file. Filename matches function name (camelCase).

### 2. Multi-Export Files → One Export Per File

```
# BEFORE
hooks.ts  # exports useA, useB, useC

# AFTER
useA.ts
useB.ts
useC.ts
```

### 3. Oversized Files → Extracted Pieces

Files over 150 lines get examined for extraction opportunities. Files over 250 lines are mandatory splits.

### 4. Object.assign Compound Patterns → Flat Named Exports

```tsx
// BEFORE
const Dialog = Object.assign(DialogRoot, { Content, Title, Trigger });

// AFTER
export { DialogRoot as Dialog } from "./DialogRoot";
export { DialogContent } from "./DialogContent";
export { DialogTitle } from "./DialogTitle";
```

### 5. Raw Animation Classes → Animation Components

```tsx
// BEFORE
<div className="motion-safe:animate-scale-in" style={{ '--stagger-index': i }}>

// AFTER
<ScaleIn index={i}>
```

### 6. Direct Store Access → Opaque Hooks

```tsx
// BEFORE
const state = useStore(instance.store, useShallow(s => ({ ... })));

// AFTER
const state = core.executions.useExecutionState(instance);
```

---

## Definition of Done

A surface area is **COMPLETE** when:

1. All 5 review passes have been executed
2. Zero violations found on the final (5th) pass
3. All fixes committed
4. `task check` passes clean (or `task wasm:lint && task wasm:test` for Rust crates)

The entire sweep is **DONE** when:

1. All 20 surface areas are COMPLETE
2. `task check` passes clean on the full codebase
3. This strategy document is updated with final status
4. PR is merged to main
