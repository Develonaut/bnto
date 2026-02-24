---
name: code-review
description: Review code changes against project standards
---

# Code Review

Review all changed files against the project's coding standards, architecture rules, and known gotchas. This is a **read-only review** — you are auditing code quality, not committing or implementing.

## Step 0: Read the Standards

Before reviewing ANY code, read and internalize these files. They are the source of truth:

```
.claude/CLAUDE.md                  # Master reference — architecture, layering, tech stack
.claude/rules/code-standards.md    # Single responsibility, file/function size limits (Bento Box Principle)
.claude/rules/                     # All rule files (if present)
```

**Read ALL of these files now.** The checklist items below are reminders — the rule files and CLAUDE.md are the complete reference.

## Step 1: Identify Changed Files

Determine what needs review:

### Uncommitted changes
!`git diff --name-only HEAD 2>/dev/null; git diff --cached --name-only 2>/dev/null; git ls-files --others --exclude-standard`

### Current branch
!`git branch --show-current`

Read every changed file in full. You cannot review code you haven't read.

## Step 2: Architecture & Layer Compliance

For EACH changed file, verify:

- [ ] **Layer discipline**: `apps/web` or `apps/desktop` -> `@bnto/core` -> Go Engine. No layer skipping. UI and editor co-located in `apps/web/`
- [ ] **API abstraction**: No direct Convex queries/mutations in components. No direct Wails bindings in components. All data access via `@bnto/core` hooks
- [ ] **Package boundaries**: Internal packages (`@bnto/backend`, `@bnto/auth`) consumed only by `@bnto/core` internals. `apps/web` and `apps/desktop` NEVER import from them directly
- [ ] **Import discipline**: UI from local `@/components/`, data/actions from `@bnto/core`. Types flow down from core
- [ ] **Go package boundaries**: Each Go package stays in its lane — `engine/` orchestrates, `registry/` registers, `node/` types execute, `storage/` persists, `validator/` validates. No circular dependencies

## Step 3: Bento Box Compliance

For EACH changed file, verify:

- [ ] **Single responsibility**: Go files < 250 lines, Go functions < 20 lines. TS files < 250 lines, TS functions < 20 lines, hooks < 30 lines
- [ ] **One export per file**: Every exported component, hook, or function gets its own file. No `hooks.ts` grab bags, no `utils.ts` grab bags, no multi-component files. Only exception: shadcn primitives (thin `forwardRef` wrappers)
- [ ] **Folder organization**: Components (PascalCase `.tsx`) at folder root, hooks in `hooks/` subdirectory (`use-kebab-case.ts`), pure functions in `utils/` subdirectory (`kebab-case.ts`). Test files co-located next to implementation
- [ ] **Components are render shells**: No `useState`, `useEffect`, handlers, or business logic in the component body. All logic lives in a `use<Component>Props` hook or pure functions. Components call one props hook and spread onto JSX
- [ ] **Compose, don't configure**: Consumers compose which parts appear via JSX children, not boolean/config props. Domain components with 3+ sub-parts use dot-notation namespace via `Object.assign`, NOT flat barrel exports
- [ ] **Pure functions -> logic hooks -> props hooks -> components**: Business rules in pure testable functions, logic hooks compose them reactively, props hooks build the complete props object, components are render shells
- [ ] **Hook decomposition**: Hooks doing multiple things split into focused sub-hooks. Signs it's too big: >30 lines, multiple unrelated state, hard to name without "and"
- [ ] **Primitives vs business components**: Generic in `primitives/` (no domain knowledge), domain-specific in `components/`

## Step 4: Go Code Compliance

Skip if no Go files changed. Otherwise:

- [ ] **Bento Box Principle**: One concept per file, one purpose per function
- [ ] **Error handling**: Errors properly wrapped with context — `return fmt.Errorf("loading workflow %s: %w", path, err)` not bare `return err`. No swallowed errors
- [ ] **Context propagation**: `context.Context` passed through the chain. All long-running operations accept and respect context. Cancellation checked in loops and before expensive operations
- [ ] **Interface design**: Small, consumer-defined interfaces. Accept interfaces, return structs. No mega-interfaces with 10+ methods
- [ ] **Package boundaries**: `engine/` orchestrates (doesn't do I/O), `registry/` registers (doesn't execute), `node/` types execute (don't know about other types), `storage/` persists, `validator/` validates
- [ ] **No circular dependencies** between Go packages

## Step 5: TypeScript Compliance

Skip if no TypeScript files changed. Otherwise:

- [ ] Types inferred where possible (no redundant annotations like `const x: Foo = getFoo()`)
- [ ] No `any` without eslint-disable + justification comment
- [ ] No `Record<string, unknown>` for domain data — use typed interfaces
- [ ] No gratuitous `as` type assertions — only at trust boundaries (JSON.parse, external API)
- [ ] `as const` + `satisfies` used where appropriate (literal preservation, shape validation without widening)
- [ ] Return types inferred on internal functions, only annotated at public API boundaries

## Step 6: React Query & State Management

Skip if no state management files changed. Otherwise:

- [ ] **`select` for transforms**: Every `useQuery` that transforms data (`.map()`, `toFoo()`, spread) does it inside `select`, NOT in the hook body
- [ ] **No transforms outside `select`**: No `.map()` / `.filter()` on query data outside `select`
- [ ] **No spread of query data**: `{ ...data, isLoading }` creates a new object. Destructure explicitly
- [ ] **Zustand selectors**: `useStore(s => s.field)`, not `useStore()` (selecting entire store)
- [ ] **Right tool for the job**: Server state -> React Query, client app state -> Zustand, local UI state -> `useState`, URL state -> router params

## Step 7: Performance

Skip if no frontend files changed. Otherwise:

- [ ] **Minimal `"use client"`** — only on leaf components that need interactivity, not pages or layouts
- [ ] **Server Components first** — data fetched on server where possible, not everything client-side
- [ ] **No barrel imports in client components** — import from specific files, not `index.ts`
- [ ] **Heavy components lazy loaded** — modals, dialogs, below-fold content use `next/dynamic`
- [ ] **Images use `next/image`** — proper lazy loading, sizing, format optimization

## Step 8: Known Gotchas Check

Scan changed files for these specific pitfalls:

- [ ] **Tailwind dynamic classes**: No template literals or string concatenation for class names. All tokens must use static string literals
- [ ] **Tailwind monorepo**: Classes in shared packages need `@source` directive in `globals.css`
- [ ] **Stale symlinks**: After moving packages, `node_modules` cleaned up
- [ ] **Transport-agnostic API**: Components never call Convex or Wails directly — all data flows through `@bnto/core` hooks

## Step 9: Code Quality

- [ ] No secrets, API keys, or passwords in code
- [ ] No magic numbers/strings — use constants and theme tokens
- [ ] No dead code or unused imports
- [ ] Consistent style with existing patterns in sibling files
- [ ] No unnecessary complexity — YAGNI applies

## Step 10: Test Coverage Check

Verify tests exist for the changes. For detailed test quality evaluation (are tests testing behavior or implementation? are they at the right level?), use `/test-review`.

Quick coverage check — flag if missing:

- **Go engine logic** (node execution, validation, path resolution) -> Unit tests in `engine/pkg/*/`
- **Go API endpoints** (`apps/api/`) -> Integration tests with `httptest`
- **Core hooks/adapters** (`@bnto/core`) -> Unit tests in `packages/@bnto/core/`
- **Backend functions** (`@bnto/backend`) -> Tests in `packages/@bnto/backend/convex/`
- **Pure utils/functions** -> Co-located `.test.ts` or `_test.go` files next to the source
- **Headless hooks** with non-trivial logic -> Co-located tests

Flag any missing test coverage.

## Step 11: Stale Artifact Check

Verify the changes didn't leave stale artifacts behind:

- [ ] **Test assertions** updated for changed behavior, props, APIs, DOM structure
- [ ] **Code references** updated for renamed/removed/changed exports, props, interfaces
- [ ] **Documentation** updated for changed behavior documented in comments, JSDoc, or markdown
- [ ] **Imports** — no broken imports from renames or moves

Search the codebase for references to anything that was changed (class names, prop names, component names, function signatures, selectors, text strings). Flag anything that still references the old version.

## Step 12: Review Summary

Present findings organized by severity:

### Violations (must fix)
Issues that break architecture rules, introduce bugs, or violate hard constraints. List each with:
- **File**: path and line number
- **Rule**: which standard is violated (reference the rule file)
- **Issue**: what's wrong
- **Fix**: specific recommendation

### Warnings (should fix)
Issues that don't break rules but degrade code quality. Style inconsistencies, missing optimizations, suboptimal patterns.

### Notes
Observations, questions, or suggestions that aren't violations.

### Checklist Summary
```
Architecture & Layers:  PASS / FAIL (count)
Bento Box:              PASS / FAIL (count)
Go Code:                PASS / FAIL (count) / SKIPPED
TypeScript:             PASS / FAIL (count) / SKIPPED
React Query / State:    PASS / FAIL (count) / SKIPPED
Performance:            PASS / FAIL (count) / SKIPPED
Gotchas:                PASS / FAIL (count)
Code Quality:           PASS / FAIL (count)
Test Coverage:          PASS / FAIL (count)
Stale Artifacts:        PASS / FAIL (count)
```

**Overall verdict:** PASS / NEEDS FIXES (with count of violations)
