---
name: pickup
description: Pickup Work — Two-Phase Task Execution
args: "[--worktree | --w]"
---

# Pickup Work — Two-Phase Task Execution

This skill uses a **propose-then-execute** workflow. Phase 1 researches the next available task, checks the landscape for conflicts, and presents a proposal. Phase 2 executes only after user approval.

## Arguments

| Flag                | Description                                                                                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| _(no flag)_         | **Default.** Work on `main` with a feature branch. No worktree, no isolation — just branch and go.                                                 |
| `--worktree`, `--w` | **Force worktree.** Create an isolated worktree. Only use when the user explicitly asks for isolation or multiple agents need to work in parallel. |

**Usage:** `/pickup`, `/pickup --w`

---

## Phase 1: Research & Propose

**Do NOT write any code yet.** Your only job in Phase 1 is to understand the next task, assess the landscape, and present a clear proposal for the user to approve or reject.

### Step 0: Check the Landscape

Before looking at tasks, understand what's happening in the repo right now. Run these checks:

```bash
# Any uncommitted changes?
git status --short

# Recent activity on main?
git log --oneline -5 main
```

Then scan `PLAN.md` for **CLAIMED** tasks — note which packages they target. This tells you where other agents are working.

### Step 1: Read the Plan

Read `.claude/PLAN.md`. Find the **current sprint** (the first sprint with unclaimed tasks).

### Step 2: Identify the Next Task(s)

- Find unclaimed tasks (`- [ ]` without **CLAIMED**) in the earliest available wave
- **Do not pick tasks from a wave if the previous wave has unclaimed or claimed (in-progress) tasks** — waves are sequential
- If all tasks in the current wave are claimed or done, report that no tasks are available right now and stop
- If multiple unclaimed tasks exist in the same wave, list all of them

**Batch pickup:** Look for tasks that form a natural batch — same package scope, same domain, logically connected. An agent building `EditorToolbar`, `NodePalette`, and `NodeConfigPanel` in the same wave shouldn't PR after each one. Recommend batching when:

| Signal                                                      | Batch?                                                     |
| ----------------------------------------------------------- | ---------------------------------------------------------- |
| Same `[package]` tag, same wave, shared files/context       | Yes — recommend as a batch                                 |
| Same wave but different packages (e.g., `[core]` + `[web]`) | Maybe — only if one depends on the other and they're small |
| Different waves                                             | No — waves are sequential                                  |
| Batch would exceed ~1 day of agent work                     | No — too large, split into smaller batches                 |

When recommending a batch, present it as a single proposal with all tasks listed, a combined scope estimate, and a note on why batching makes sense.

### Step 3: Research the Task

For the candidate task(s), do quick research to understand what's involved:

- Read the files that would need to change (use Glob/Grep/Read — do NOT modify anything)
- Identify the package scope (`[web]`, `[engine]`, `[core]`, `[backend]`, etc.)
- Identify which persona(s) would be activated
- Note any dependencies, blockers, or risks you see
- Estimate the rough scope (small/medium/large)

### Step 4: Present the Proposal

**If the work spans 2+ PRs**, produce a structured multi-PR plan following [feature-planning.md](../../scopes/process/feature-planning.md). This is mandatory — not optional. The plan must include:

- Phase header with context, what changes, what doesn't change (counts, surfaces)
- Per-PR sections: branch, one-sentence summary, files (new/modified with counts), key function/API, RED tests, verification commands, count changes
- Dependency chain showing PR ordering

Present the full plan document to the user for approval. This IS the proposal.

**If the work fits in a single PR**, use the same structured format from [feature-planning.md](../../scopes/process/feature-planning.md). Single-PR work follows the same per-PR section structure — the only difference is there's no dependency chain.

Present the proposal using this exact structure:

```
## PR: One-sentence description

**Branch:** `<type>/<short-description>` from `main`
**One sentence:** Describe the PR in exactly one sentence without "and."
**Sprint / Wave:** Which sprint and wave
**Persona(s):** Which domain expert persona(s) will be activated

### What
2-3 sentences: what this PR delivers and why.

### Files (~N new, ~N modified)
**New:**
- exact file paths with brief description

**Modified:**
- exact file paths with brief description

### Key function / API
Code signature or data structure that defines the PR's contract.

### RED tests (write first)
Bullet list of failing tests to write BEFORE implementation.
These define acceptance criteria — when all pass, the feature is done.
- `test_name` — what it asserts (specific input → expected output)

### Verification
Exact shell commands. Copy-pasteable.

### Count changes
Which test count registries change, or "no count changes."

### Risks / Open questions
- Anything unclear or potentially tricky

### Scope estimate
Small (< 1 hour), Medium (1-3 hours), Large (3+ hours)
```

**Rules (from feature-planning.md, apply to single-PR work too):**

1. **One sentence per PR.** If you can't describe it without "and," split it.
2. **RED tests are acceptance criteria.** They define what "done" looks like. When all RED tests turn green, the PR is complete.
3. **Files are enumerated.** List every new and modified file with paths. Approximate counts in the header.
4. **Verification is copy-pasteable.** Exact commands, not prose.
5. **Count changes are explicit.** "No count changes" is a valid and important statement.

If recommending a batch, list all tasks under "What" and explain why they form a natural unit (shared context, same files, logical sequence).

If there are multiple available tasks in the wave, present all of them so the user can pick.

**Then STOP and wait for the user's response.** Do not proceed to Phase 2 until the user explicitly approves.

---

## Phase 2: Execute (after user approval)

Only proceed here after the user says to go ahead. The user may:

- Approve as-is → proceed with the plan
- Approve with changes → adjust your approach, then proceed
- Reject → stop, or propose a different task
- Pick a different task from the ones presented → research that one instead

### Step 1: Read the Standards

Before doing ANY work, read and internalize the project's coding standards and architecture rules. These documents define how code must be written in this codebase:

```
.claude/CLAUDE.md                  # Master reference — architecture, layering, tech stack
.claude/rules/code-standards.md    # Single responsibility, file/function size limits (Bento Box Principle)
.claude/scopes/process/feature-planning.md  # Multi-PR feature plan format (required for 2+ PR work)
.claude/rules/                     # All rule files (if present)
.claude/scopes/web/pages.md             # SEO URL requirements and predefined Bnto page conventions
.claude/rules/architecture.md      # Run quota schema, R2 transit rules
.claude/strategy/core-principles.md # Trust commitments
```

**Read ALL of these files now.** Do not skim, do not skip. You will be held to every rule in them. The inlined summaries later in this prompt are reminders — the rule files and CLAUDE.md are the source of truth.

### Step 2: Claim the Task(s)

Edit `PLAN.md` to mark your task(s): change `- [ ]` to `- [ ] **CLAIMED**`

If you're picking up a batch, claim all tasks in the batch at once. This signals to other agents that the entire batch is spoken for.

### Step 2b: Set Up Branch

**Default: work on `main` with a feature branch.** Worktrees are only used when the user explicitly passes `--w`.

1. Ensure you start from a clean `main`: `git checkout main && git pull`
2. Create a feature branch: `git checkout -b <type>/<short-description>` (e.g., `feat/editor-toolbar`, `fix/skeleton-shift`)

#### If worktree (`--worktree` / `--w` flag only):

1. Ensure you start from a clean `main`: `git checkout main && git pull`
2. Use the `EnterWorktree` tool with a name based on your feature branch (e.g., `feat/editor-toolbar`)
3. The worktree creates an isolated copy at `.claude/worktrees/<name>` with a new branch based on HEAD
4. Your session's working directory switches to the worktree — all subsequent file reads, edits, and commands operate there

### Step 3: Activate Your Persona

Now that you know your task's `[package]` tag, activate the domain expert persona by invoking it as a skill:

| Package tag             | Persona skill                      |
| ----------------------- | ---------------------------------- |
| `[engine]`              | `/rust-expert`                     |
| `[web]`, `[ui]`         | `/frontend-engineer`               |
| `[core]`                | `/core-architect`                  |
| `[backend]`, `[auth]`   | `/backend-engineer`                |
| `[monorepo]`, `[infra]` | No persona — use general standards |

**Sprint-specific persona overrides:**

- **Sprint 4B (Code Editor):** ALL tasks invoke `/code-editor-expert` regardless of package tag. Wave 2+ tasks also invoke `/frontend-engineer`. Read [code-editor.md](.claude/strategy/code-editor.md) and the persona SKILL.md before starting.
- **Sprint 4 Wave 2+ (Visual Editor):** ALL tasks invoke `/reactflow-expert`. Wave 3+ also invoke `/frontend-engineer`.

**Invoke the persona skill now.** Each persona is a domain expert with specialized knowledge, vocabulary, gotchas, and quality standards that go beyond the general rules. The persona will shape your approach for the duration of this task.

**Cross-package work:** If your task requires touching files outside your primary package (e.g., an `[engine]` task that also updates the WASM worker in `apps/web/`), invoke all relevant persona skills. Multiple personas sharpen your awareness of each domain's standards.

**Security-sensitive work:** If your task touches auth, middleware, input validation, file uploads, Convex mutations, or API endpoints, also invoke `/security-engineer`. The security persona owns trust boundaries across all packages and will help you think adversarially about the code you're writing.

**Testing work:** If your task involves writing E2E tests, updating screenshot baselines, or modifying test infrastructure, also invoke `/quality-engineer`. The quality persona owns E2E strategy, journey-based test design, screenshot regression workflows, and the correct way to run tests (port isolation, two-run verification, selector patterns).

### Step 4: Scope Check

Before writing any code, confirm your boundaries:

- **Read the `[package]` tag** on your task — that's your workspace
- **Do not modify files outside your tagged package** unless the task explicitly requires it
- **Check git status first** — if you see uncommitted changes in your package's files, STOP and report to the user. Another agent may have been working here
- **Read existing code** in the files you plan to modify before making changes. Understand patterns, naming conventions, and structure already in place

**Pricing model scope check** — ask these before writing a single line (see [pricing-model.md](../../strategy/pricing-model.md)):

- **Adding a new predefined recipe?** — It needs a dedicated URL slug, server-side metadata, and node classification (browser vs server). See `.claude/scopes/web/pages.md` and `.claude/strategy/bntos.md`.
- **Adding execution logic?** — Browser-node executions are free, unlimited, no tracking needed. Server-node executions must be tracked (they count against Pro usage quota).
- **Building a user-facing flow?** — Conversion hooks should trigger on value moments (save, history, server nodes, team) — never on browser execution limits.
- **Touching the recipe editor?** — The editor is free. Create, run, export = free. Save, share, server nodes = Pro. Don't gate editor access.

### Step 5: Write Tests First (TDD)

**Tests define acceptance criteria.** Before writing any implementation code, write the tests that prove the feature works. This is the most important step — it forces you to think about the API, edge cases, and expected behavior before getting lost in implementation details.

#### The TDD Cycle

```
1. Write a failing test  →  defines WHAT the code should do
2. Write minimal code    →  makes the test pass (and nothing more)
3. Refactor              →  clean up while tests stay green
4. Repeat                →  next test case, next behavior
```

#### What to Test First (by layer)

| Layer                        | Write these tests FIRST                                                                  | Tool                               |
| ---------------------------- | ---------------------------------------------------------------------------------------- | ---------------------------------- |
| **Pure functions / actions** | Input → output assertions. Edge cases. Guard conditions. Error paths.                    | Vitest unit tests                  |
| **Rust engine logic**        | Native Rust unit tests in `#[cfg(test)]` blocks. WASM integration tests for JS boundary. | `cargo test` + `wasm-bindgen-test` |
| **Core hooks / services**    | Service method behavior. Query option construction. Cache invalidation.                  | Vitest unit tests                  |
| **Backend (Convex)**         | Mutation validation. Auth guards. Query correctness.                                     | `convex-test`                      |
| **UI components**            | Render with expected props. User interactions trigger correct callbacks.                 | Vitest + testing-library (light)   |
| **User flows**               | Full journeys through the UI.                                                            | Playwright E2E                     |

#### How This Changes Your Workflow

**Instead of:** Write code → hope it works → write tests to verify → discover edge cases → go back and fix

**Do this:** Think about behavior → write test that asserts it → write code to pass the test → move to next behavior

```typescript
// EXAMPLE: Building an addNode action

// Step 1: Write the test FIRST — this IS your acceptance criteria
describe("addNode", () => {
  it("returns null if IO node already exists", () => {
    const state = makeState({ nodes: [inputNode] });
    expect(addNode(state, "input")).toBeNull();
  });

  it("creates node with unique ID", () => {
    const state = makeState({ nodes: [] });
    const result = addNode(state, "image");
    expect(result?.nextState.nodes).toHaveLength(1);
    expect(result?.nodeId).toBeDefined();
  });

  it("captures undo snapshot before mutation", () => {
    const state = makeState({ nodes: [] });
    const result = addNode(state, "image");
    expect(result?.nextState.undoStack).toHaveLength(1);
  });
});

// Step 2: NOW write the addNode function to make these pass
// Step 3: Refactor (extract helpers, simplify) while tests stay green
```

#### Rules

1. **Tests are not optional and not an afterthought.** They come BEFORE implementation. If you find yourself writing code without a failing test, stop and write the test first.
2. **Test the contract, not the implementation.** Assert on inputs/outputs and observable behavior. Don't test private methods or internal state unless there's no other way to verify correctness.
3. **Each test case is an acceptance criterion.** When you're done, the test suite IS the specification. Someone reading your tests should understand exactly what the feature does without reading the implementation.
4. **Start with the happy path, then edge cases.** First test: "it does the main thing correctly." Then: "it handles empty input," "it rejects invalid args," "it doesn't duplicate," etc.
5. **Run tests frequently.** After writing each test, run the suite to confirm it fails for the right reason. After writing implementation, run again to confirm it passes. Green → move on. Red → fix before adding more.

### Step 6: Implement

Write the code to make your tests pass. Follow the rules in `CLAUDE.md` and `.claude/rules/`:

#### Component Philosophy (CRITICAL)

- **Components are dumb** — they receive data and render UI. That's it. No API calls, no business logic, no domain state in render. All data flows through `@bnto/core` hooks
- **One component/hook per file** — every exported component or hook gets its own file. No multi-component files. Use folder + barrel export (`index.ts`). Only exception: shadcn primitives (thin `forwardRef` wrappers with no logic)
- **Folder organization** — components (PascalCase `.tsx`) at folder root, hooks in `hooks/` subdirectory (`use-kebab-case.ts`), pure functions in `utils/` subdirectory (`kebab-case.ts`). Only create subdirectories when needed. Test files co-locate next to implementation
- **Props are domain objects, not destructured primitives** — pass `workflow` not `name, description, status, nodeCount, ...`
- **Compound composition** — compose complex UI from small parts (Radix pattern), not by adding props. `<Card><CardHeader>...</Card>` not `<Card header={...} />`
- **Primitives vs business components** — generic reusable components (Button, Card, Badge) go in `primitives/`. Domain-specific components (WorkflowCard, ExecutionTimeline, NodeEditor) go in `components/`

#### Layered Code Organization

- **Pure functions -> hooks -> components** — extract business logic into pure testable functions (< 20 lines), hooks are thin reactive wrappers (< 30 lines), components just render
- **Hook decomposition** — if a hook does fetching + transformation + subscription + side effects, split it into focused sub-hooks. Signs it's too big: >30 lines, multiple unrelated state, hard to name without "and"
- **Bento Box Principle** — every file < 250 lines, every function < 20 lines. No utility grab bags, no god objects. See `.claude/rules/code-standards.md` for the full checklist

#### Other Standards

- **TypeScript:** infer types, no `any`, no gratuitous `as` assertions, types flow down from core
- **Import discipline:** UI from local `@/components/`, data from `@bnto/core`, never skip layers. Third-party UI deps should be wrapped locally
- **Transport-agnostic API:** Components NEVER call Convex or backend APIs directly. All data access via `@bnto/core` hooks
- Match existing patterns — look at sibling files for naming, structure, and style

#### UI Reference: shadcn-blocks

**Before building any UI**, check shadcn-blocks for patterns and inspiration:

**shadcn-blocks** (`/Users/ryan/Code/shadcn-blocks/blocks/`) — A library of well-composed, production-quality component examples. Browse the relevant block category for your task and pick the best variant to adapt. Key categories for Bnto:

- `data-table/` — sortable tables, pagination, row selection (workflow lists, execution history)
- `sidebar/`, `application-shell/` — navigation, rail layouts (main app shell)
- `cards/`, `stats-card/` — dashboard cards, stat displays (workflow status, execution metrics)
- `settings-profile/` — settings pages, edit forms
- `onboarding/` — split-screen layouts, upload zones (workflow import)
- `project/`, `projects/` — project cards, article layouts (workflow detail pages)

**Don't copy blindly** — adapt the layout, structure, and interaction patterns to fit our design system. The value is in the _composition patterns_, not the exact styling.

### Step 7: Verify — Code Review + Automated Checks

#### 7a: Code Review

Run `/code-review` to audit all your changes against the project's coding standards, architecture rules, and known gotchas. Fix any violations before proceeding. This is a critical quality gate — do not skip it.

#### 7b: Automated Checks

Run ALL checks. Do not skip any even if you think your changes are safe:

```bash
# Rust checks (only if you touched engine/ files)
task wasm:lint          # clippy — must pass clean
task wasm:test:unit     # Rust unit tests — must pass

# TypeScript checks (always run)
task ui:build          # TypeScript compilation — must pass
task ui:test           # Frontend tests — must pass
task ui:lint           # Lint all TS packages — must pass
```

Or run `task check` to execute all of the above in one command.

**If any check fails:**

1. Fix the issue
2. Re-run ALL checks from the top (not just the one that failed)
3. Repeat until all pass clean

**Critical rule:** You are NOT allowed to ignore failures as "pre-existing." If a check fails, report ALL failures to the user and let them decide. Only the user can determine if an issue predates your work.

### Step 8: Verify — Test Coverage

**If your task involves E2E tests, screenshot updates, or test infrastructure changes**, invoke `/quality-engineer` now. The quality persona owns the correct way to run tests, write selectors, capture screenshots, and handle known issues like "01 Issue" hydration mismatches.

**By this point, your tests from Step 5 should already be passing.** This step verifies completeness — did you cover all the layers? Are there gaps?

- **Rust engine logic** (node crates, WASM bindings) -> **Unit tests** in `#[cfg(test)]` blocks + WASM integration tests via `wasm-bindgen-test`
- **Core hooks/adapters** (`@bnto/core`) -> **Unit tests** using Vitest in `packages/core/`
- **Backend functions** (`@bnto/backend`) -> **Unit/integration tests** in `packages/@bnto/backend/__tests__/`
- **Pure utils/functions** (any `utils/` directory) -> **Unit tests** co-located next to the source file
- **Configuration or type-only changes** -> Tests not required

**No exceptions.** If your task adds a function and you didn't write a test, you're not done. Go back and write the tests before proceeding.

#### Did you touch UI?

Ask yourself: **did you create, modify, or wire up ANY component, dialog, form, page, or layout that a user will see or interact with?** This includes:

- Components in `apps/web/components/` (even "presentational only" — they render on screen)
- Wiring in `apps/web/` (routes, dialogs, pages)
- Changes to props, layout, styling, or behavior of existing UI

**If yes — you MUST write or update e2e tests with screenshot assertions.** This is non-negotiable. Unit tests alone are not proof that UI works. The user needs to see tangible visual evidence that the feature renders correctly.

**Required e2e coverage:**

- Add to or create spec files in `apps/web/e2e/`. Use existing helpers and patterns from sibling spec files.
- Test the actual user flow, not just that a page renders.
- Include `await expect(page).toHaveScreenshot()` assertions — at minimum:
  - One screenshot of the primary UI state the change introduces or modifies
  - One screenshot of any new dialog, modal, or form in its populated state
- Run the e2e tests and confirm screenshots are generated
- **VISUALLY VERIFY screenshots** — After e2e tests generate screenshots, you MUST use the Read tool to open each new or updated `.png` file and confirm the visual output matches expectations. Do not report "screenshots generated" without actually looking at them. If a screenshot looks wrong (broken layout, missing elements, wrong colors), fix the issue before proceeding.

**"It's just a UI component" is not an excuse to skip e2e tests.** If it renders on screen, it gets tested on screen. A `[ui]` task that creates a form component used by a `[web]` dialog still needs an e2e test proving the dialog works end-to-end.

**If you genuinely believe no e2e test is needed** (e.g., pure internal refactor with zero visual change), you MUST ask the user for explicit approval before skipping. Do not decide this on your own.

If screenshots already exist and the change modifies visual output, run with `--update-snapshots` after confirming the new appearance is correct.

**E2e test conventions:**

- Always set `test.use({ reducedMotion: "reduce" })` to disable animations
- Use `data-testid` markers for reliable state detection
- Use semantic selectors (`getByRole`, `getByText`) over CSS classes
- Reference existing spec files for patterns

#### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes but is not limited to:

- **Screenshots** — If you changed visual output, delete stale `.png` files. They regenerate on the next e2e run with `--update-snapshots`.
- **Test assertions** — If you changed behavior, props, APIs, or DOM structure, update any tests that assert on the old behavior.
- **Code references** — If you renamed, removed, or changed exports, props, or interfaces, find and update all consumers.
- **Documentation** — If you changed behavior that's documented in comments, JSDoc, or markdown, update the docs to match.

**How to find stale references:** Search the codebase (`Grep`) for the specific things you changed — class names, prop names, component names, function signatures, selectors, text strings. If something references the old version, fix it.

**Do not skip this.** Leaving stale artifacts behind breaks CI, confuses other developers, and wastes everyone's time debugging phantom failures.

### Step 9: Update PLAN.md (MANDATORY — do this BEFORE the PR)

**You MUST update PLAN.md now.** This is the #1 thing agents forget, and it causes real problems — other agents pick up work that's already done, or miss unblocked waves.

1. Edit `.claude/PLAN.md`
2. Change each completed task from `- [ ] **CLAIMED**` to `- [x]`
3. If a task is partially done, leave it as `- [ ] **CLAIMED**` and add a note about what remains
4. If your completion unblocks the next wave (all tasks in current wave are now `[x]`), note this in your proof of work summary

**Quick check:** Run `grep -n "CLAIMED" .claude/PLAN.md` — if any CLAIMED tasks are yours, update them now.

**Do NOT skip this step.** PLAN.md changes MUST be included in your commit and PR. The `/pre-commit` and `/merge-pr` skills will verify this.

### Step 9b: Proof of Work

After all checks pass, provide a summary:

1. **Branch** — name of the feature branch (e.g., `feat/editor-toolbar`)
2. **PR target** — `main` (always)
3. **PLAN.md updated?** — Yes (list tasks marked done) or N/A (not a `/pickup` task)
4. **Did you touch UI?** — Yes or No. If you created, modified, or wired up any component, dialog, form, page, or layout — the answer is Yes.
5. **If yes:** What e2e tests did you write or update? List spec files and the flows they cover. List screenshot assertions. **Confirm you visually inspected each screenshot using the Read tool** and describe what you see. If no e2e tests, explain why and confirm user approved the skip.
6. **If no UI touched:** What unit/integration tests did you write? List test files and what they cover.
7. **Checks result** — confirm `task check` (or individual checks) passed clean. List which checks ran.
8. **Files changed** — files created/modified, with brief description of each

### Step 9c: Create the PR

**PRs always target `main`.** Use `--base main` when creating the PR.

When creating the PR with `gh pr create`, use this format for the body:

```
## Summary
<1-3 bullet points describing what changed and why>

## Verification
<What you actually did to verify the change works. Be specific:>
- What checks you ran and their results (e.g., "task ui:build — passed clean")
- What tests you wrote or ran (e.g., "Added 3 unit tests in historyService.test.ts — all pass")
- What you manually verified (e.g., "Read the generated output file and confirmed correct CSV headers")
- For UI changes: what screenshots you captured and visually inspected
- For docs/config-only changes: what you reviewed to confirm correctness
```

**The Verification section documents what YOU did, not what someone else should do.** It's proof of work — past tense, specific, with results. Not a forward-looking checklist of TODOs.

---

## E2E Testing

All E2E tests run against the full dev stack (Next.js + Convex). There is no "UI-only" mode — the backend must always be running.

**How to run E2E tests — decision tree:**

```
Step 1: Is a dev server already running on port 4000?
  $ lsof -ti:4000

  YES (output shows a PID) → The user has `task dev` running. Reuse it:
    $ task e2e
    This runs both stages (browser parallel, then editor serial).
    Playwright's reuseExistingServer: true connects to the already-running server.

  NO (no output) → Start the dev server first, then run tests:
    $ cd /Users/ryan/Code/bnto && task dev &
    $ sleep 15  # wait for Next.js + Convex to start
    $ task e2e
```

**CRITICAL: Never kill the user's dev server on port 4000.** If it's running, reuse it. If it's not running, start one.

**Updating screenshots (two runs required):**

```bash
cd apps/web && pnpm exec playwright test --update-snapshots   # Run 1: regenerate
cd apps/web && pnpm exec playwright test                      # Run 2: verify stable
```

**Common mistakes agents make:**

1. Running from the repo root instead of `apps/web/` — Playwright config is in `apps/web/`, so you must `cd apps/web` first (or use `pnpm --filter @bnto/web exec playwright test`).
2. Skipping the `lsof -ti:4000` check — always check first. If port 4000 is active, just use it.
3. Forgetting to start `task dev` — E2E requires a running dev server on port 4000.

**Key details:**

- `task e2e` runs two stages: browser tests parallel, then editor tests serial (`--workers=1`)
- `task e2e:browser` and `task e2e:editor` run individual stages
- `reuseExistingServer: true` in playwright.config.ts — Playwright reuses whatever server is already on port 4000
- Test fixtures are shared with the engine (`test-fixtures/`)

**Shared test helpers** (in `e2e/helpers.ts`):

- `navigateToRecipe(page, slug, h1)` — navigate to recipe page, wait for heading visible
- `assertBrowserExecution(page)` — verify `data-execution-mode="browser"` on shell
- `uploadFiles(page, filePaths[])` — set file input, wait for count text, return run button
- `runAndComplete(page, options?)` — click Run, wait for terminal phase, return run button
- `downloadAndVerify(page, options?)` — download output, verify magic bytes/size, return buffer
- `downloadAllAsZip(page)` — click Download All, verify ZIP magic bytes, return buffer
- `assertWebPBytes(buffer)` — verify WebP RIFF + WEBP magic bytes
- Constants: `IMAGE_FIXTURES_DIR`, `CSV_FIXTURES_DIR`, `MAGIC` (JPEG, PNG, WEBP_RIFF, WEBP_TAG, ZIP)

**Screenshot strategy:** Page-level screenshots only (site navigation, auth forms). Execution flows verified programmatically (magic bytes, data attributes, file sizes).

**Data attributes for E2E observability:**

- `data-testid="run-button"` + `data-phase` — RunButton lifecycle (idle, uploading, running, completed, failed)
- `data-testid="execution-progress"` + `data-status` — ExecutionProgress status
- `data-testid="node-progress"` + `data-node-id` + `data-node-status` — per-node progress
- `data-testid="upload-file"` + `data-file-status` — per-file upload progress
- `data-testid="execution-results"` — results panel container
- `data-testid="output-file"` — individual output file items
- `data-testid="bnto-shell"` + `data-session` + `data-user-id` — session and identity state

---

## DO NOT

- **Branch-based workflow is mandatory.** Create a feature branch (`git checkout -b <type>/<short-description>`) before committing. Never commit directly to `main` — PRs are required. If the user asks you to commit, create a branch first, commit YOUR OWN work from this task (never bundle other agents' changes), then ask if they want you to push and create a PR. Before pushing, ALWAYS ask the user for explicit confirmation — never push autonomously
- **PRs always target `main`.** Feature branches are created from `main` and PR'd into `main`. Always squash merge.
- **Do not modify files outside your package scope** — other agents may be working there
- **Do not modify `CLAUDE.md`, `.claude/rules/`, or config files** unless your task explicitly requires it
- **Do not install new dependencies** without noting it in your summary. If a dependency is needed, prefer one already in the monorepo
- **Do not delete or rename existing exports** — other agents or existing code may depend on them
- **Do not run `pnpm dev` or standalone dev servers** — use `task dev` for E2E tests, and run it in the background

## Multi-Agent Awareness

- **Feature branches on `main` are the default.** Worktrees are only used when explicitly requested via `--w`.
- **File conflicts:** If you need to modify a file and see it has been recently changed (check `git diff`), read the current state carefully before editing. Work with what's there, not what you expected
- **Schema changes:** If your task adds to any schema, append — don't reorganize existing structures. Other agents may depend on the current structure
- **Shared indexes/exports:** If you add to a barrel export (`index.ts`), add your entries at the end to minimize merge conflicts
- **Port conflicts:** Only start `task dev` when running E2E tests (and check if it's already running first). For non-E2E verification, use `task check`
