---
name: code-review
description: Review code changes against project standards
---

# Code Review

Review all changed files against the project's coding standards, architecture rules, and known gotchas. **Fix violations and warnings immediately** — don't just report them, apply the fixes yourself.

## Step 0: Read the Standards

Before reviewing ANY code, read and internalize these files. They are the source of truth:

```
.claude/CLAUDE.md                  # Master reference — architecture, layering, tech stack
.claude/rules/code-standards.md    # Single responsibility, file/function size limits (Bento Box Principle)
.claude/rules/                     # All rule files (if present)
```

**Read ALL of these files now.** The checklist items below are reminders — the rule files and CLAUDE.md are the complete reference.

## Step 0b: Activate Domain Personas

Identify which packages the changed files belong to and invoke the relevant persona skill(s):

| Changed files in...                               | Persona skill                           |
| ------------------------------------------------- | --------------------------------------- |
| `engine/`                                         | `/rust-expert`                          |
| `apps/web/`                                       | `/frontend-engineer` + `/nextjs-expert` |
| `packages/core/`                                  | `/core-architect`                       |
| `packages/@bnto/backend/`, `packages/@bnto/auth/` | `/backend-engineer`                     |

**If changes touch auth, security headers, input validation, file uploads, or Convex mutations**, also invoke `/security-engineer` — the security persona owns trust boundaries across all packages.

**If changes touch `apps/web/`**, always invoke `/nextjs-expert` alongside `/frontend-engineer` — the Next.js expert catches framework-level performance issues, server/client boundary violations, and rendering strategy mistakes that the frontend engineer's component-level review won't cover.

**If changes affect package structure, public API, directory layout, exports, commands, or README files**, also invoke `/technical-writer` — the technical writer owns all human-facing `README.md` files and will check whether any READMEs need updating to reflect the changes.

**Invoke the matching persona skill(s) now.** Each persona is a domain expert with specialized quality standards, gotchas, and performance patterns that inform a deeper review. If changes span multiple packages, invoke all relevant personas.

## Step 1: Identify Changed Files

Determine what needs review:

### Uncommitted changes

!`git diff --name-only HEAD 2>/dev/null; git diff --cached --name-only 2>/dev/null; git ls-files --others --exclude-standard`

### Current branch

!`git branch --show-current`

**Read every changed file in full.** You cannot review code you haven't read. If there are more than 15 changed files, batch your reads but you MUST read ALL of them before proceeding to Step 2.

## Step 1b: Per-File Audit Table (MANDATORY)

**Before ANY analysis, produce this table for EVERY changed file.** This is not optional. Skip no files. Measure, don't estimate.

For each changed file, read it and record:

| File | Lines | Exported Components | Exported Functions/Hooks | Largest Function (lines) | Status |
| ---- | ----- | ------------------- | ------------------------ | ------------------------ | ------ |

Fill in every row. Use actual line counts from the file, not guesses.

**Status column values:**

- `OK` — all metrics within limits
- `VIOLATION` — any metric exceeds limits (file > 250 lines, function > 20 lines, multiple exported components, etc.)
- `WARNING` — near limits or has code quality concerns
- `DELETED` — file was removed (verify no stale references)

**Hard limits to check per file:**

- File length: **> 250 lines = VIOLATION** (hard cap, no exceptions)
- Function/component length: **> 20 lines = VIOLATION** (hooks get 30 lines)
- Exported components per file: **> 1 = VIOLATION** (exception: shadcn primitives)
- Exported hooks per file: **> 1 = VIOLATION**

**This table is the foundation of the review.** Every violation found in later steps should trace back to a row in this table. If a file has `VIOLATION` status, it MUST be fixed in Step 13.

## Step 2: Standards Audit — Per-File Checklist

For EACH changed file, run it through every applicable standard below. Produce a per-file findings list noting PASS, FAIL, or N/A for each standard category.

### 2a. Architecture & Layers ([architecture.md](../../rules/architecture.md))

- [ ] **Layer discipline**: CLI links engine directly. Web: `apps/web` -> `@bnto/core` -> Rust Engine (WASM). No layer skipping
- [ ] **API abstraction**: No direct Convex queries/mutations in components. All data via `@bnto/core` hooks
- [ ] **Package boundaries**: `@bnto/backend`, `@bnto/auth` consumed only by `@bnto/core` internals
- [ ] **Import discipline**: UI from local `@/components/`, data from `@bnto/core`. Types flow down

### 2b. Bento Box Principle ([code-standards.md](../../rules/code-standards.md))

Use the audit table from Step 1b. Every file with `VIOLATION` status must be addressed.

- [ ] **File size**: > 250 lines = FAIL. Flag exact count
- [ ] **Function size**: > 20 lines = FAIL (hooks: 30). List every oversized function by name and line count
- [ ] **One export per file**: > 1 exported component = FAIL. > 1 exported hook = FAIL
- [ ] **No multi-component files**: Multiple `function` returning JSX in same file = FAIL. Extract to own files
- [ ] **Folder organization**: Components at folder root (PascalCase), hooks in `hooks/`, utils in `utils/`
- [ ] **Code duplication**: Near-identical logic across files = WARNING. Extract shared functions/components
- [ ] **No grab bags**: No `utils.ts`, `helpers.ts`, `hooks.ts` files with multiple unrelated exports

### 2c. Component Standards ([components.md](../../rules/components.md))

Skip if no `.tsx` component files changed.

- [ ] **Start inline, extract when earned**: Logic inline is fine. Hooks only when > 80 lines or reuse needed
- [ ] **Self-fetching**: Components fetch own data by ID. Pass IDs, not data
- [ ] **CSS-first states**: Hover/focus/active via pseudo-classes, not `useState`. Data attributes over ternary classes
- [ ] **Flat named exports**: No `Object.assign` dot-notation. All compound components use prefixed flat exports
- [ ] **Size `md` default**: T-shirt-sized props default to `md`. Consumers rarely specify
- [ ] **Hover/focus parity**: Every `group-hover:` has `group-focus-within:` for keyboard users

### 2d. TypeScript Standards ([typescript.md](../../rules/typescript.md))

Skip if no TypeScript files changed.

- [ ] **Inference preferred**: No redundant annotations (`const x: Foo = getFoo()` → `const x = getFoo()`)
- [ ] **No `any`**: Must have eslint-disable + justification. Use `unknown` with type guards
- [ ] **No `Record<string, unknown>`**: Use typed interfaces for domain data
- [ ] **No gratuitous `as`**: Only at trust boundaries (JSON.parse, external API, `Id<T>`)
- [ ] **`as const` + `satisfies`**: Used for literal preservation and shape validation
- [ ] **Return types inferred**: Only annotate at public API boundaries

### 2e. Data Fetching & State ([core-api.md](../../rules/core-api.md), [data-fetching-strategy.md](../../strategy/data-fetching-strategy.md))

Skip if no data fetching or state management files changed.

- [ ] **`select` for transforms**: Every `useQuery` transform inside `select`, not hook body
- [ ] **No prop drilling server data**: Children self-fetch by ID
- [ ] **Zustand selectors**: `useStore(s => s.field)`, not `useStore()` (whole store)
- [ ] **Right tool**: Server state → React Query, client state → Zustand, UI state → `useState`
- [ ] **`convexQuery` skip guard**: Every `convexQuery()` with ID param uses `"skip"` when falsy

### 2f. Performance ([performance.md](../../rules/performance.md))

Skip if no frontend files changed.

- [ ] **Minimal `"use client"`**: Only on leaf components needing interactivity
- [ ] **Server Components first**: Data fetched on server where possible
- [ ] **No barrel imports in client**: Import specific files, not `index.ts`
- [ ] **Heavy components lazy loaded**: Modals/dialogs use `next/dynamic`

### 2g. Theming & Animation ([theming.md](../../rules/theming.md), [animation.md](../../rules/animation.md))

Skip if no styling/animation changes.

- [ ] **Semantic tokens only**: No hardcoded colors, radii, shadows, or fonts
- [ ] **Animation components**: Use `<ScaleIn>`, `<SlideUp>`, etc. — never raw `animate-*` classes
- [ ] **`motion-safe:` guard**: Every animation respects `prefers-reduced-motion`
- [ ] **Compositor-only**: Animate `opacity`, `scale`, `translate`, `rotate` only

### 2h. Known Gotchas ([gotchas.md](../../rules/gotchas.md))

- [ ] **Tailwind dynamic classes**: No template literals or string concatenation for class names
- [ ] **Tailwind monorepo**: Classes in shared packages need `@source` in `globals.css`
- [ ] **Transport-agnostic**: Components never call Convex or backend APIs directly

### 2i. Rust Code ([engine-node-patterns.md](../../rules/engine-node-patterns.md))

Skip if no Rust files changed.

- [ ] **Bento Box**: One concept per file/module, one purpose per function
- [ ] **Error handling**: `Result<T, E>` used. No bare `.unwrap()` — use `?` or descriptive `.expect()`
- [ ] **Parameter contract**: Every param in `metadata()` read and used in ALL `process()` code paths
- [ ] **Shared encoding**: Image processors use `encode::encode_image()`, not custom encode functions
- [ ] **Parameterized tests**: Different param values produce measurably different outputs
- [ ] **Golden tests**: If output changed, golden files updated and diff reviewed
- [ ] **Processor naming**: `fn name()` returns the same string as the registry key (category-first: `"image-compress"`)
- [ ] **Crate README**: Processors table is up to date and section order follows the standard template

## Step 3: Code Quality

- [ ] No secrets, API keys, or passwords in code
- [ ] No magic numbers/strings — use constants and theme tokens
- [ ] No dead code or unused imports
- [ ] Consistent style with existing patterns in sibling files
- [ ] No unnecessary complexity — YAGNI applies

## Step 4: Test Coverage Check

Verify tests exist for the changes. For detailed quality evaluation, use `/test-review`.

- **Rust engine logic** → Unit tests in `#[cfg(test)]` + WASM integration tests
- **Core hooks/adapters** → Unit tests in `packages/core/`
- **Backend functions** → Tests in `packages/@bnto/backend/convex/`
- **Pure utils/functions** → Co-located `.test.ts` files
- **Headless hooks** with non-trivial logic → Co-located tests

Flag any missing test coverage.

## Step 5: Stale Artifact & Dead Code Check

- [ ] **Test assertions** updated for changed behavior, props, APIs, DOM structure
- [ ] **Code references** updated for renamed/removed/changed exports
- [ ] **Imports** — no broken imports from renames or moves
- [ ] **Dead exports** — no barrel exports re-exporting symbols with zero external consumers. For each exported symbol, grep across the monorepo excluding its own package. If only referenced in its own barrel + source = dead
- [ ] **Orphaned files** — no source files left behind after deletes or renames

## Step 6: Documentation Check

If `/technical-writer` was activated, it handles this. Otherwise:

- [ ] **README accuracy** — exports, directories, commands, types still match reality
- [ ] **New packages/crates** must have a `README.md`
- [ ] **No `.claude/` links in READMEs**

## Step 7: Fix Violations & Warnings

**Do not just report issues — fix them.** For every file marked `VIOLATION` in the Step 1b audit table, and for every FAIL in the Step 2 checklist, apply the fix immediately.

### Fixing workflow

1. For each violation or warning found, apply the fix directly using Edit/Write tools
2. After all fixes are applied, re-run `task ui:build` and `task ui:test` to verify nothing broke
3. If a fix introduces new issues, fix those too — repeat until clean

### What NOT to fix automatically

- **Notes** — observations or suggestions that aren't violations. Present these to the user
- **Architectural questions** — significant restructuring or design decisions need user approval

## Step 8: Review Summary

After fixing all issues, present a summary with these sections:

### Per-File Audit Table (Final)

Reproduce the completed audit table from Step 1b showing final state of every file (with updated line counts after fixes).

### Standards Matrix

Produce a summary matrix showing PASS/FAIL/N-A for each standard category per file:

| File | Arch | Bento | Component | TS  | Data | Perf | Theme | Gotchas | Quality | Tests | Stale |
| ---- | ---- | ----- | --------- | --- | ---- | ---- | ----- | ------- | ------- | ----- | ----- |

Each cell: `PASS`, `FAIL` (with count), or `-` (not applicable).

### Fixes Applied

List each fix with:

- **File**: path and line number
- **Rule**: which standard was violated (link to rule document section)
- **What was wrong**: brief description
- **What was fixed**: what you changed

### Notes

Observations, questions, or suggestions that aren't violations — presented for the user's consideration.

### Overall Verdict

```
Architecture & Layers:    PASS / FAIL (count)
Bento Box:                PASS / FAIL (count)
Component Standards:      PASS / FAIL (count) / SKIPPED
TypeScript:               PASS / FAIL (count)
Data Fetching / State:    PASS / FAIL (count) / SKIPPED
Performance:              PASS / FAIL (count) / SKIPPED
Theming & Animation:      PASS / FAIL (count) / SKIPPED
Gotchas:                  PASS / FAIL (count)
Rust Code:                PASS / FAIL (count) / SKIPPED (no Rust changes)
Code Quality:             PASS / FAIL (count)
Test Coverage:            PASS / FAIL (count)
Stale Artifacts:          PASS / FAIL (count)
Documentation:            PASS / FAIL / SKIPPED (no structural changes)
Unit/Integration Tests:   PASS / FAIL — NEVER skip without explicit user permission
E2E Tests:                PASS / FAIL — NEVER skip without explicit user permission
```

**Overall verdict:** PASS / FIXED (count of fixes applied)
