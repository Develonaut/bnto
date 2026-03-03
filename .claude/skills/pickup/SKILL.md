---
name: pickup
description: Pickup Work — Two-Phase Task Execution
args: "[--w | --worktree]"
---

# Pickup Work — Two-Phase Task Execution

This skill uses a **propose-then-execute** workflow. Phase 1 researches the next available task and presents a proposal. Phase 2 executes only after user approval.

## Arguments

| Flag | Description |
|------|-------------|
| `--w`, `--worktree` | Execute Phase 2 in an isolated git worktree. The main working tree stays clean while you work. Phase 1 (research) always runs on the main tree. |

**Usage:** `/pickup --w` or `/pickup --worktree`

---

## Phase 1: Research & Propose

**Do NOT write any code yet.** Your only job in Phase 1 is to understand the next task and present a clear proposal for the user to approve or reject.

### Step 1: Read the Plan

Read `.claude/PLAN.md`. Find the **current sprint** (the first sprint with unclaimed tasks).

### Step 2: Identify the Next Task

- Find unclaimed tasks (`- [ ]` without **CLAIMED**) in the earliest available wave
- **Do not pick tasks from a wave if the previous wave has unclaimed or claimed (in-progress) tasks** — waves are sequential
- If all tasks in the current wave are claimed or done, report that no tasks are available right now and stop
- If multiple unclaimed tasks exist in the same wave, list all of them so the user can choose

### Step 3: Research the Task

For the candidate task(s), do quick research to understand what's involved:

- Read the files that would need to change (use Glob/Grep/Read — do NOT modify anything)
- Identify the package scope (`[web]`, `[engine]`, `[core]`, `[backend]`, etc.)
- Identify which persona(s) would be activated
- Note any dependencies, blockers, or risks you see
- Estimate the rough scope (small/medium/large)

### Step 4: Present the Proposal

Present a clear summary to the user with:

1. **Task:** The task description from PLAN.md (verbatim)
2. **Sprint / Wave:** Which sprint and wave it belongs to
3. **Sprint branch:** The sprint's long-lived branch (e.g., `sprint/3-platform-features`). Found in PLAN.md under the sprint header. If not listed, flag it — the user needs to create one or confirm targeting `main`.
4. **Package scope:** Which packages/directories will be touched
5. **Persona(s):** Which domain expert persona(s) will be activated
6. **Approach:** 3-5 bullet points describing what you plan to do
7. **Files to modify:** List of files you expect to create or change
8. **Tests:** What tests you'll write (unit, integration, E2E, screenshots)
9. **Risks / Open questions:** Anything unclear or potentially tricky
10. **Scope estimate:** Small (< 1 hour), Medium (1-3 hours), Large (3+ hours)

If there are multiple available tasks in the wave, present all of them so the user can pick.

**Then STOP and wait for the user's response.** Do not proceed to Phase 2 until the user explicitly approves.

---

## Phase 2: Execute (after user approval)

**CRITICAL: Do NOT enter a worktree unless the user explicitly passed `--w` or `--worktree`.** If the user invoked `/pickup` without the flag, you work on the current tree. Never use `EnterWorktree` on your own initiative. If you are already inside a worktree from a previous session, that does NOT authorize worktree usage for new tasks — ask the user.

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
.claude/rules/                     # All rule files (if present)
.claude/rules/pages.md             # SEO URL requirements and predefined Bnto page conventions
.claude/rules/architecture.md      # Run quota schema, R2 transit rules
.claude/strategy/core-principles.md # Trust commitments
```

**Read ALL of these files now.** Do not skim, do not skip. You will be held to every rule in them. The inlined summaries later in this prompt are reminders — the rule files and CLAUDE.md are the source of truth.

### Step 2: Claim the Task

Edit `PLAN.md` to mark your task: change `- [ ]` to `- [ ] **CLAIMED**`

### Step 2b: Determine the Sprint Branch

**Sprint branches are the default merge target.** Each active sprint has a long-lived branch (`sprint/<id>-<short-name>`, e.g. `sprint/3-platform-features`). Task branches are created from and PR'd into the sprint branch — not `main`. This prevents Vercel deployment rate-limiting.

1. **Identify the sprint branch** for your task's sprint. Check if it exists: `git ls-remote --heads origin sprint/<id>-*`
2. **If the sprint branch doesn't exist**, ask the user to create it or confirm you should create one (e.g., `git checkout -b sprint/3-platform-features origin/main && git push -u origin sprint/3-platform-features`).
3. **If the user explicitly says to target `main`**, skip the sprint branch and use `main` as the base instead.

### Step 2c: Enter Worktree (ONLY if `--w` / `--worktree` flag was explicitly passed)

**NEVER enter a worktree unless the user explicitly passed `--w` or `--worktree` in their `/pickup` invocation.** This is strictly opt-in. If the flag was not passed, skip this entire step — no exceptions, no judgment calls, no "it would be better in a worktree." Work directly on the main tree (create a branch in Step 5).

If the user invoked `/pickup --w` or `/pickup --worktree`, create an isolated git worktree **now**, before writing any code:

1. **CRITICAL: Ensure you start from the sprint branch.** Before creating the worktree, check out the sprint branch and pull: `git checkout sprint/<id>-<name> && git pull`. Worktrees branch from HEAD — if HEAD is wrong, the worktree inherits that divergence. If no sprint branch exists (or the user told you to target `main`), use `main` instead.
2. Use the `EnterWorktree` tool with a name based on the branch you'd create (e.g., `feat/execution-history`)
3. The worktree creates an isolated copy of the repo at `.claude/worktrees/<name>` with a new branch based on HEAD (which is now the sprint branch)
4. Your session's working directory switches to the worktree — all subsequent file reads, edits, and commands operate there
5. The main working tree stays completely untouched

**Why worktrees?** They let the user (or other agents) keep working on the main tree while you work in isolation. No stashing, no branch switching conflicts, no accidental interference with in-progress work.

### Step 3: Activate Your Persona

Now that you know your task's `[package]` tag, activate the domain expert persona by invoking it as a skill:

| Package tag | Persona skill |
|---|---|
| `[engine]` | `/rust-expert` |
| `[engine-go]`, `[api-go]` | `/go-engineer` |
| `[web]`, `[ui]` | `/frontend-engineer` |
| `[core]` | `/core-architect` |
| `[backend]`, `[auth]` | `/backend-engineer` |
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

- **Adding a new predefined recipe?** — It needs a dedicated URL slug, server-side metadata, and node classification (browser vs server). See `.claude/rules/pages.md` and `.claude/strategy/bntos.md`.
- **Adding execution logic?** — Browser-node executions are free, unlimited, no tracking needed. Server-node executions must be tracked (they count against Pro usage quota).
- **Building a user-facing flow?** — Conversion hooks should trigger on value moments (save, history, server nodes, team) — never on browser execution limits.
- **Touching the recipe editor?** — The editor is free. Create, run, export = free. Save, share, server nodes = Pro. Don't gate editor access.

### Step 5: Implement

Write the code for your task. Follow the rules in `CLAUDE.md` and `.claude/rules/`:

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

#### Go Code Standards (CRITICAL)

- **One concept per file, one purpose per function** — same Bento Box Principle applies to Go
- **Error handling**: Always wrap errors with context — `return fmt.Errorf("loading workflow %s: %w", path, err)` not bare `return err`. Never swallow errors
- **Context propagation**: Pass `context.Context` through the chain. Check cancellation in loops and before expensive operations
- **Interface design**: Small, consumer-defined. Accept interfaces, return structs. No mega-interfaces
- **Package boundaries**: `engine/` orchestrates (doesn't do I/O), `registry/` registers (doesn't execute), `node/` types execute (don't know about other types), `storage/` persists, `validator/` validates. No circular deps

#### Other Standards

- **TypeScript:** infer types, no `any`, no gratuitous `as` assertions, types flow down from core
- **Import discipline:** UI from local `@/components/`, data from `@bnto/core`, never skip layers. Third-party UI deps should be wrapped locally
- **Transport-agnostic API:** Components NEVER call Convex or Wails directly. All data access via `@bnto/core` hooks
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

**Don't copy blindly** — adapt the layout, structure, and interaction patterns to fit our design system. The value is in the *composition patterns*, not the exact styling.

### Step 6: Verify — Code Review + Automated Checks

#### 6a: Code Review

Run `/code-review` to audit all your changes against the project's coding standards, architecture rules, and known gotchas. Fix any violations before proceeding. This is a critical quality gate — do not skip it.

#### 6b: Automated Checks

Run ALL checks. Do not skip any even if you think your changes are safe:

#### Go checks
```bash
task vet               # go vet — must pass clean
task test              # engine tests with race detector — must pass
task api:test          # API server tests with race detector — must pass
```

#### TypeScript checks
```bash
task ui:build          # TypeScript compilation — must pass
task ui:test           # Frontend tests — must pass
task ui:lint           # Lint all TS packages — must pass
```

**If any check fails:**
1. Fix the issue
2. Re-run ALL checks from the top (not just the one that failed)
3. Repeat until all pass clean

**Critical rule:** You are NOT allowed to ignore failures as "pre-existing." If a check fails, report ALL failures to the user and let them decide. Only the user can determine if an issue predates your work.

### Step 7: Verify — Test Coverage

**If your task involves E2E tests, screenshot updates, or test infrastructure changes**, invoke `/quality-engineer` now. The quality persona owns the correct way to run tests, write selectors, capture screenshots, and handle known issues like "01 Issue" hydration mismatches.

Your work MUST include tests. Determine which type based on what you built:

- **Go engine logic** (node execution, validation, path resolution) -> **Unit tests** in `archive/engine-go/pkg/*/` next to the source. Cover happy path, error cases, and edge cases.
- **Go API endpoints** (`archive/api-go/`) -> **Integration tests** using `httptest`. Cover request/response contracts.
- **Core hooks/adapters** (`@bnto/core`) -> **Unit tests** using Vitest in `packages/@bnto/core/`.
- **Backend functions** (`@bnto/backend`) -> **Unit/integration tests** in `packages/@bnto/backend/__tests__/`.
- **Configuration or type-only changes** -> Tests not required.

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

### Step 8: Verify — Proof of Work

After all checks pass, provide a summary:

1. **Sprint branch** — name of the sprint branch this work targets (e.g., `sprint/3-platform-features`). Found in PLAN.md under the sprint header. If no sprint branch is listed, ask the user.
2. **Branch** — name of the feature branch this work is on (e.g., `feat/execution-history`)
3. **PR** — confirm you are creating a PR and state which branch it targets (e.g., "Creating PR targeting `sprint/3-platform-features`"). PRs target the sprint branch by default, not `main`.
4. **Did you touch UI?** — Yes or No. If you created, modified, or wired up any component, dialog, form, page, or layout — the answer is Yes.
5. **If yes:** What e2e tests did you write or update? List spec files and the flows they cover. List screenshot assertions. **Confirm you visually inspected each screenshot using the Read tool** and describe what you see. If no e2e tests, explain why and confirm user approved the skip.
6. **If no UI touched:** What unit/integration tests did you write? List test files and what they cover.
7. **Go checks result** — confirm `task vet`, `task test`, `task api:test` passed clean
8. **TS checks result** — confirm `task ui:build`, `task ui:test`, `task ui:lint` passed clean
9. **Files changed** — files created/modified, with brief description of each

### Step 8b: Create the PR

**PRs target the sprint branch by default**, not `main`. Use `--base sprint/<id>-<name>` when creating the PR. If the user explicitly told you to target `main`, use `--base main` instead.

When creating the PR with `gh pr create`, use this format for the body:

```
## Summary
<1-3 bullet points describing what changed and why>

## Verification
<What you actually did to verify the change works. Be specific:>
- What checks you ran and their results (e.g., "task ui:build — passed clean")
- What tests you wrote or ran (e.g., "Added 3 unit tests in wasmExecutionService.test.ts — all pass")
- What you manually verified (e.g., "Read the generated output file and confirmed correct CSV headers")
- For UI changes: what screenshots you captured and visually inspected
- For docs/config-only changes: what you reviewed to confirm correctness
```

**The Verification section documents what YOU did, not what someone else should do.** It's proof of work — past tense, specific, with results. Not a forward-looking checklist of TODOs.

### Step 9: Update the Plan

Edit `.claude/PLAN.md`:
- Change your task from `- [ ] **CLAIMED**` to `- [x]` (mark done)
- If your completion unblocks the next wave (all tasks in current wave are now `[x]`), note this in your summary so the user knows to start new agents on the next wave

---

## E2E Testing

All E2E tests run against the full dev stack (Next.js + Convex). There is no "UI-only" mode — the backend must always be running.

**How to run E2E tests — decision tree:**

```
Step 1: Is a dev server already running on port 4000?
  $ lsof -ti:4000

  YES (output shows a PID) → The user has `task dev` running. Reuse it:
    $ cd apps/web && pnpm exec playwright test
    This is the fastest path (~30-60s). Playwright's reuseExistingServer: true
    connects to the already-running server. No startup delay.

  NO (no output) → You need a server. Two options:

    Option A (recommended): Start the dev server yourself, then run tests:
      $ cd /Users/ryan/Code/bnto && task dev &
      $ sleep 15  # wait for Next.js + Convex to start
      $ cd apps/web && pnpm exec playwright test

    Option B (fallback): Use the isolated task (starts its own Next.js):
      $ task e2e:isolated
      This is slower (builds a fresh .next-e2e cache) but self-contained.
```

**CRITICAL: Never kill the user's dev server on port 4000.** If it's running, reuse it. If it's not running, start one or use isolated mode.

**Updating screenshots (two runs required):**

```bash
# If port 4000 is active (preferred — fast):
cd apps/web && pnpm exec playwright test --update-snapshots   # Run 1: regenerate
cd apps/web && pnpm exec playwright test                      # Run 2: verify stable

# If port 4000 is NOT active (use isolated port):
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test --update-snapshots
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test
```

**Common mistakes agents make:**
1. Running `E2E_PORT=4001 pnpm ... playwright test` WITHOUT `task e2e:isolated` first — this fails because no server is listening on port 4001. Either use `task e2e:isolated` (which starts a server) or run against port 4000.
2. Running from the repo root instead of `apps/web/` — Playwright config is in `apps/web/`, so you must `cd apps/web` first (or use `pnpm --filter @bnto/web exec playwright test`).
3. Skipping the `lsof -ti:4000` check — always check first. If port 4000 is active, just use it.
4. Using `task e2e:isolated` when `task dev` is already running — unnecessarily slow. Just run `cd apps/web && pnpm exec playwright test`.

**Key details:**
- `task e2e:isolated` uses port 4001 + `NEXT_DIST_DIR=.next-e2e` (separate build cache)
- `task e2e` uses port 4000 and reuses a running `task dev`
- Both share the same Convex dev deployment (cloud-hosted, no conflict)
- `reuseExistingServer: true` in playwright.config.ts — Playwright reuses whatever server is already on the target port
- Test fixtures are shared with the Go engine (`engine/tests/fixtures/`)

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

- **Branch-based workflow is mandatory.** If you're in a worktree (from `--w` flag), the worktree already created a branch — use it. Otherwise, create a feature branch (`git checkout -b <type>/<short-description>`) before committing. Never commit directly to `main` or to the sprint branch — PRs are required. If the user asks you to commit, create a branch first (or use the worktree branch), commit YOUR OWN work from this task (never bundle other agents' changes), then ask if they want you to push and create a PR. Before pushing, ALWAYS ask the user for explicit confirmation — never push autonomously
- **PRs target the sprint branch by default.** Task feature branches are created from and PR'd into the sprint branch (e.g., `sprint/3-platform-features`), not `main`. Only target `main` if the user explicitly says so. When the sprint completes, the user merges the sprint branch into `main` (single deploy)
- **Worktrees are strictly opt-in.** NEVER use `EnterWorktree` or create a worktree unless the user explicitly passed `--w` or `--worktree` in their `/pickup` invocation. Being inside an existing worktree from a previous session does NOT authorize creating or using worktrees for new tasks. If unsure, work on the main tree
- **Worktrees start from the sprint branch.** When using `--w` / `--worktree`, ALWAYS check out the sprint branch and pull before creating the worktree. If no sprint branch exists (or the user told you to target `main`), use `main`. The worktree branches from HEAD — if HEAD is wrong, the worktree inherits that divergence
- **Do not modify files outside your package scope** — other agents may be working there
- **Do not modify `CLAUDE.md`, `.claude/rules/`, or config files** unless your task explicitly requires it
- **Do not install new dependencies** without noting it in your summary. If a dependency is needed, prefer one already in the monorepo
- **Do not delete or rename existing exports** — other agents or existing code may depend on them
- **Do not run `pnpm dev` or standalone dev servers** — use `task dev` for E2E tests, and run it in the background

## Multi-Agent Awareness

- **Worktrees eliminate conflicts.** If you're running in a worktree (`--w` flag), you have a fully isolated copy of the repo. No file conflicts with the main tree or other agents. This is the recommended mode for parallel agent work
- **File conflicts (non-worktree):** If you need to modify a file and see it has been recently changed (check `git diff`), read the current state carefully before editing. Work with what's there, not what you expected
- **Schema changes:** If your task adds to any schema, append — don't reorganize existing structures. Other agents may depend on the current structure
- **Shared indexes/exports:** If you add to a barrel export (`index.ts`), add your entries at the end to minimize merge conflicts
- **Port conflicts:** Only start `task dev` when running E2E tests (and check if it's already running first). For non-E2E verification, use the checks (`task vet`, `task test`, `task ui:build`, `task ui:test`, `task ui:lint`)
