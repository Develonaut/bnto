---
name: code-review
description: Cold review — hand finished work to a fresh agent on the best model, which reviews it against bnto's docs and reports findings for the author to fix
---

# Code Review (Cold Review)

A **cold review** hands a finished body of work to a **fresh agent** on **Opus or a stronger model**. That agent has none of the author's context, reviews the work against bnto's docs, and **reports** what doesn't meet them. The author fixes or justifies every finding before the PR. It's required for every PR. See [cold-review.md](../../decisions/cold-review.md) for why.

The skill has two parts:

- **Part 1 is for the author:** launch the reviewer, then act on its report.
- **Part 2 is for the reviewer:** its procedure.

An author never runs Part 2 on its own work. That would be a self-review, which this replaces.

---

## Part 1: The Author

### 1. When to run it

- **When:** it's the last gate before you push. Before launching it, make the work complete:
  - code, tests and cleanup;
  - the PLAN.md and ISSUES.md updates;
  - `task check` passing;
  - the work committed on its branch (not pushed);
  - the PR description drafted.
- **Again:** If your fixes changed more than wording, run a second cold review; if that one still finds blocking or should-fix problems, fix them and ask the user before a third.
- **Once per PR, before its first push, docs-only included.** The reviewer scales its review to the change. **After the PR opens,** a commit that changes more than wording (a CI fix, a requested change) gets its own cold review before it's pushed.

### 2. Launch the reviewer

Use the Agent tool:

- **`subagent_type: "general-purpose"`, a fresh agent.** Never `fork`: a fork inherits your conversation, so it isn't cold.
- **`model: "opus"`:** Ryan's standard for cold reviews. Use a stronger model if the user names one or you're running on one, never a weaker one.
- **It runs in the background.** Wait for its completion notice before you go further. Don't predict or act on its findings early.
- **The prompt below, filled in.** Give the reviewer the work and its brief. Don't give it your reasoning, what you think is fine, or which findings to expect.

```
You are doing a cold review. You have none of the author's context, on purpose.

Repo: <absolute repo path>. Branch: <branch>; base: main.
The work: `git diff main...HEAD` and its commits (`git log main..HEAD`), plus anything
`git status --short` shows uncommitted.

What the work claims to do (its brief):
<quote the ISSUES.md entry, the plan's PR section, the PLAN.md task, or the user's request>

Draft PR description:
<quote it>

Follow Part 2 of .claude/skills/code-review/SKILL.md exactly, read from the working tree
(the work may change it). Do not edit, create or delete any file, and do not commit or
push. Report your findings in Part 2's format.
```

### 3. Act on the report

- **Every finding gets an answer:** fix it, or justify it in the PR description. A **blocking** finding is fixed, unless the user agrees otherwise.
- **Re-check:** commit the fixes, update the draft description, re-run `task check`, and follow the second-review rule in §1.
- **Record the outcome** in the PR description, e.g. `Cold review: 3 findings, 2 fixed, 1 justified`, with the justification under it.
- **Work isn't done, and no PR opens, before this.**

---

## Part 2: The Reviewer

You are reviewing someone else's work against bnto's docs. **Report; never edit.** Your report is the deliverable.

**Scale to the change.** Read every changed file in full whatever its kind. For docs-only work, mark the code checklists (2a–2i, Step 1b's size metrics) SKIPPED, and spend the review on the brief, the claims, consistency between docs, and links.

### Step 0: Read the Standards

Before reviewing ANY code, read and internalize these files. They are the source of truth:

```
.claude/CLAUDE.md                  # Master reference — architecture, layering, tech stack
.claude/rules/code-standards.md    # Single responsibility, file/function size limits (Bento Box Principle)
.claude/rules/                     # All rule files (if present)
```

**Read ALL of these files now.** The checklist items below are reminders — the rule files and CLAUDE.md are the complete reference.

### Step 0b: Activate Domain Personas

Identify which packages the changed files belong to and invoke the relevant persona skill(s):

| Changed files in...                               | Persona skill                                                           |
| ------------------------------------------------- | ----------------------------------------------------------------------- |
| `engine/`                                         | `/rust-expert`                                                          |
| `apps/web/`                                       | `/frontend-engineer` + `/nextjs-expert`                                 |
| `packages/core/`                                  | `/core-architect`                                                       |
| `packages/@bnto/backend/`, `packages/@bnto/auth/` | `/backend-engineer`                                                     |
| `.claude/` only (docs, rules, skills)             | none — read `scopes/process/` and every doc the change touches or links |

**If changes touch auth, security headers, input validation, file uploads, or Convex mutations**, also invoke `/security-engineer` — the security persona owns trust boundaries across all packages.

**If changes touch `apps/web/`**, always invoke `/nextjs-expert` alongside `/frontend-engineer` — the Next.js expert catches framework-level performance issues, server/client boundary violations, and rendering strategy mistakes that the frontend engineer's component-level review won't cover.

**If changes affect package structure, public API, directory layout, exports, commands, or README files**, also invoke `/technical-writer` — the technical writer owns all human-facing `README.md` files and will check whether any READMEs need updating to reflect the changes.

**Invoke the matching persona skill(s) now.** Each persona is a domain expert with specialized quality standards, gotchas, and performance patterns that inform a deeper review. If changes span multiple packages, invoke all relevant personas.

### Step 1: Identify the Work

The work is everything on the branch since `main`, plus any uncommitted changes:

```bash
git diff --name-only main...HEAD
git status --short
git log --oneline main..HEAD
```

**Read the brief** (what the work claims to do) **and every changed file in full.** You cannot review code you haven't read. If there are more than 15 changed files, batch your reads but you MUST read ALL of them before proceeding to Step 2.

### Step 1b: Per-File Audit Table (MANDATORY)

**Before ANY analysis, produce this table for EVERY changed file.** This is not optional. Skip no files. Measure, don't estimate.

For each changed file, read it and record:

| File | Lines | Exported Components | Exported Functions/Hooks | Largest Function (lines) | Status |
| ---- | ----- | ------------------- | ------------------------ | ------------------------ | ------ |

Fill in every row. Use actual line counts from the file, not guesses.

**Status column values:**

- `OK` — all metrics within limits
- `VIOLATION` — any metric exceeds limits (TS file > 250 lines, Rust file > 400 prod lines, function > 20 lines, multiple exported components, etc.)
- `WARNING` — near limits or has code quality concerns
- `DELETED` — file was removed (verify no stale references)

**Hard limits to check per file:**

- File length (TypeScript): **> 250 lines = VIOLATION** (hard cap, no exceptions)
- File length (Rust): **> 400 production lines = VIOLATION** (exclude `#[cfg(test)]` blocks)
- Function length: **> 20 lines = VIOLATION**, not counting blank lines and comments. A `.tsx` function gets 60, and tests are exempt ([code-standards.md](../../rules/code-standards.md)). A function already on a baseline (`eslint-suppressions.json`, `#[expect(clippy::too_many_lines)]`) that the change didn't lengthen is `OK (baselined)`, not a violation. If the change made it longer, that's should-fix. A new long function is blocking
- Exported components per file: **> 1 = VIOLATION** (exception: shadcn primitives)
- Exported hooks per file: **> 1 = VIOLATION**

**This table is the foundation of the review.** Every violation found in later steps should trace back to a row in this table. Every file with `VIOLATION` status becomes a finding in Step 8.

### Step 2: Standards Audit — Per-File Checklist

For EACH changed file, run it through every applicable standard below. Produce a per-file findings list noting PASS, FAIL, or N/A for each standard category.

#### 2a. Architecture & Layers ([architecture.md](../../rules/architecture.md))

- [ ] **Layer discipline**: CLI links engine directly. Web: `apps/web` -> `@bnto/core` -> Rust Engine (WASM). No layer skipping
- [ ] **API abstraction**: No direct Convex queries/mutations in components. All data via `@bnto/core` hooks
- [ ] **Package boundaries**: `@bnto/backend`, `@bnto/auth` consumed only by `@bnto/core` internals
- [ ] **Import discipline**: UI from local `@/components/`, data from `@bnto/core`. Types flow down

#### 2b. Bento Box Principle ([code-standards.md](../../rules/code-standards.md))

Use the audit table from Step 1b. Every file with `VIOLATION` status must be addressed.

- [ ] **File size**: TS > 250 lines = FAIL, Rust > 400 prod lines (excl. `#[cfg(test)]`) = FAIL. Flag exact count
- [ ] **Function size**: > 20 lines = FAIL (`.tsx`: 60; tests exempt). List every oversized function by name and line count, and say which are new and which are baselined
- [ ] **One export per file**: > 1 exported component = FAIL. > 1 exported hook = FAIL
- [ ] **No multi-component files**: Multiple `function` returning JSX in same file = FAIL. Extract to own files
- [ ] **Folder organization**: Components at folder root (PascalCase), hooks in `hooks/`, utils in `utils/`
- [ ] **Code duplication**: Near-identical logic across files = WARNING. Extract shared functions/components
- [ ] **No grab bags**: No `utils.ts`, `helpers.ts`, `hooks.ts` files with multiple unrelated exports

#### 2c. Component Standards ([components.md](../../scopes/web/components.md))

Skip if no `.tsx` component files changed.

- [ ] **Start inline, extract when earned**: Logic inline is fine. A hook once the component holds ~20+ lines of logic, or for reuse or testability ([pre-commit.md](../../rules/pre-commit.md) Step 2)
- [ ] **Self-fetching**: Components fetch own data by ID. Pass IDs, not data
- [ ] **CSS-first states**: Hover/focus/active via pseudo-classes, not `useState`. Data attributes over ternary classes
- [ ] **Flat named exports**: No `Object.assign` dot-notation. All compound components use prefixed flat exports
- [ ] **Size `md` default**: T-shirt-sized props default to `md`. Consumers rarely specify
- [ ] **Hover/focus parity**: Every `group-hover:` has `group-focus-within:` for keyboard users

#### 2d. TypeScript Standards ([typescript.md](../../scopes/web/typescript.md))

Skip if no TypeScript files changed.

- [ ] **Inference preferred**: No redundant annotations (`const x: Foo = getFoo()` → `const x = getFoo()`)
- [ ] **No `any`**: Must have eslint-disable + justification. Use `unknown` with type guards
- [ ] **No `Record<string, unknown>`**: Use typed interfaces for domain data
- [ ] **No gratuitous `as`**: Only at trust boundaries (JSON.parse, external API, `Id<T>`)
- [ ] **`as const` + `satisfies`**: Used for literal preservation and shape validation
- [ ] **Return types inferred**: Only annotate at public API boundaries

#### 2e. Data Fetching & State ([core-api.md](../../scopes/backend/core-api.md), [data-fetching-strategy.md](../../strategy/data-fetching-strategy.md))

Skip if no data fetching or state management files changed.

- [ ] **`select` for transforms**: Every `useQuery` transform inside `select`, not hook body
- [ ] **No prop drilling server data**: Children self-fetch by ID
- [ ] **Zustand selectors**: `useStore(s => s.field)`, not `useStore()` (whole store)
- [ ] **Right tool**: Server state → React Query, client state → Zustand, UI state → `useState`
- [ ] **`convexQuery` skip guard**: Every `convexQuery()` with ID param uses `"skip"` when falsy

#### 2f. Performance ([performance.md](../../scopes/web/performance.md))

Skip if no frontend files changed.

- [ ] **Minimal `"use client"`**: Only on leaf components needing interactivity
- [ ] **Server Components first**: Data fetched on server where possible
- [ ] **No barrel imports in client**: Import specific files, not `index.ts`
- [ ] **Heavy components lazy loaded**: Modals/dialogs use `next/dynamic`

#### 2g. Theming & Animation ([theming.md](../../scopes/web/theming.md), [animation.md](../../scopes/web/animation.md))

Skip if no styling/animation changes.

- [ ] **Semantic tokens only**: No hardcoded colors, radii, shadows, or fonts
- [ ] **Animation components**: Use `<ScaleIn>`, `<SlideUp>`, etc. — never raw `animate-*` classes
- [ ] **`motion-safe:` guard**: Every animation respects `prefers-reduced-motion`
- [ ] **Compositor-only**: Animate `opacity`, `scale`, `translate`, `rotate` only

#### 2h. Known Gotchas ([gotchas.md](../../rules/gotchas.md))

- [ ] **Tailwind dynamic classes**: No template literals or string concatenation for class names
- [ ] **Tailwind monorepo**: Classes in shared packages need `@source` in `globals.css`
- [ ] **Transport-agnostic**: Components never call Convex or backend APIs directly

#### 2i. Rust Code ([engine-node-patterns.md](../../scopes/rust/engine-node-patterns.md))

Skip if no Rust files changed.

- [ ] **Bento Box**: One concept per file/module, one purpose per function
- [ ] **Generic nodes**: A new node or param is a single-purpose, generic building block, not use-case logic added to an existing node. A param that reshapes the node "when X is set" is a second node. See [generic-building-block-nodes.md](../../decisions/generic-building-block-nodes.md)
- [ ] **Error handling**: `Result<T, E>` used. No bare `.unwrap()` — use `?` or descriptive `.expect()`
- [ ] **Parameter contract**: Every param in `metadata()` read and used in ALL `process()` code paths
- [ ] **Shared encoding**: Image processors use `encode::encode_image()`, not custom encode functions
- [ ] **Parameterized tests**: Different param values produce measurably different outputs
- [ ] **Golden tests**: If output changed, golden files updated and diff reviewed
- [ ] **Processor naming**: `fn name()` returns the same string as the registry key (category-first: `"image-compress"`)
- [ ] **Node vocabulary**: A new or renamed node is `<domain>-<verb>` with a verb from [node-vocabulary.md](../../scopes/rust/node-vocabulary.md), and behaves as that verb's class promises (content, side effects, output name). Label is "Verb Objects". Params are camelCase and reuse the Shared Params names. Renaming a shipped type or param comes with a `bnto migrate` step
- [ ] **Crate README**: Processors table is up to date and section order follows the standard template

### Step 3: Code Quality

- [ ] No secrets, API keys, or passwords in code
- [ ] No magic numbers/strings — use constants and theme tokens
- [ ] No dead code or unused imports
- [ ] Consistent style with existing patterns in sibling files
- [ ] No unnecessary complexity — YAGNI applies

### Step 4: Test Coverage Check

Verify tests exist for the changes. For detailed quality evaluation, use `/test-review`.

- **Rust engine logic** → Unit tests in `#[cfg(test)]` + WASM integration tests
- **Core hooks/adapters** → Unit tests in `packages/core/`
- **Backend functions** → Tests in `packages/@bnto/backend/convex/`
- **Pure utils/functions** → Co-located `.test.ts` files
- **Headless hooks** with non-trivial logic → Co-located tests

Flag any missing test coverage.

### Step 5: Stale Artifact & Dead Code Check

- [ ] **Test assertions** updated for changed behavior, props, APIs, DOM structure
- [ ] **Code references** updated for renamed/removed/changed exports
- [ ] **Imports** — no broken imports from renames or moves
- [ ] **Dead exports** — no barrel exports re-exporting symbols with zero external consumers. For each exported symbol, grep across the monorepo excluding its own package. If only referenced in its own barrel + source = dead
- [ ] **Orphaned files** — no source files left behind after deletes or renames

### Step 6: Documentation Check

If `/technical-writer` was activated, it handles this. Otherwise:

- [ ] **README accuracy** — exports, directories, commands, types still match reality
- [ ] **New packages/crates** must have a `README.md`
- [ ] **No `.claude/` links in READMEs**

### Step 7: Check the Brief and the Claims

- [ ] **The brief is met:** the work does what its issue, plan section or task says, no less, and no unrelated extras (single-concern rule, [pre-commit.md](../../rules/pre-commit.md)).
- [ ] **Tests prove the behavior:** each new behavior has a test that asserts it and would fail without the change. Check this by reading what the test asserts against what changed. A test that only exercises the new code's shape doesn't count. Note that this checks RED-first by its result; a squashed branch can't show the order
- [ ] **Every claim is true:** check each statement in the PR description and commit messages against the code and tests (counts, "no behavior change", "byte-identical", file and line references).
- [ ] **Bookkeeping:** the issue's `ISSUES.md` entry is deleted when the PR fixes it. Plan and tracker rows are updated. New defects found are logged, not fixed in passing.

### Step 8: Verify, Then Report

**Verify every finding before you report it.** Re-read the line, confirm the rule applies, and run a command where one settles it (a line count, a grep, a test). Drop anything you can't confirm, and say what you dropped.

**Don't edit files, commit or push.** Report in this format:

#### Findings

| #   | Severity | File:line | Rule (doc link) | What's wrong | Evidence |
| --- | -------- | --------- | --------------- | ------------ | -------- |

**Severity levels:**

- **blocking:** breaks a rule in the docs, is incorrect or insecure, lacks a RED test for new behavior, or makes a false claim.
- **should-fix:** a deviation from established patterns, stale docs, or weak tests.
- **note:** a suggestion, not a violation.

#### Per-File Audit Table

The Step 1b table, completed.

#### Standards Matrix

| File | Arch | Bento | Component | TS  | Data | Perf | Theme | Gotchas | Quality | Tests | Stale | Brief |
| ---- | ---- | ----- | --------- | --- | ---- | ---- | ----- | ------- | ------- | ----- | ----- | ----- |

Each cell: `PASS`, `FAIL` (with count), or `-` (not applicable).

#### Verdict

```
Architecture & Layers:    PASS / FAIL (count)
Bento Box:                PASS / FAIL (count)
Component Standards:      PASS / FAIL (count) / SKIPPED
TypeScript:               PASS / FAIL (count) / SKIPPED
Data Fetching / State:    PASS / FAIL (count) / SKIPPED
Performance:              PASS / FAIL (count) / SKIPPED
Theming & Animation:      PASS / FAIL (count) / SKIPPED
Gotchas:                  PASS / FAIL (count)
Rust Code:                PASS / FAIL (count) / SKIPPED (no Rust changes)
Code Quality:             PASS / FAIL (count)
Test Coverage:            PASS / FAIL (count)
Stale Artifacts:          PASS / FAIL (count)
Documentation:            PASS / FAIL / SKIPPED (no structural changes)
Brief & Claims:           PASS / FAIL (count)
```

**Overall:**

- **PASS:** no blocking or should-fix findings.
- **CHANGES NEEDED:** give the count of blocking and should-fix findings.
