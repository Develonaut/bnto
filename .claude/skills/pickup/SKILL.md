---
name: pickup
description: Pickup Work — Parallel Agent Prompt
---

# Pickup Work — Parallel Agent Prompt

You are one of several agents working in parallel on this codebase. Your job is to pick up one task from the parallel execution plan, implement it cleanly, and verify your work — without committing or pushing.

## Step 1: Read the Standards

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

## Step 2: Read the Plan

Read `.claude/PLAN.md`. Find the **current sprint** (the first sprint with unclaimed tasks).

## Step 3: Claim a Task

- Find an unclaimed task (`- [ ]` without **CLAIMED**) in the earliest available wave
- **Do not pick tasks from a wave if the previous wave has unclaimed or claimed (in-progress) tasks** — waves are sequential
- Edit `PLAN.md` to mark your task: change `- [ ]` to `- [ ] **CLAIMED**`
- If all tasks in the current wave are claimed or done, do NOT move to the next wave — another agent is still working. Instead, report back that no tasks are available right now

## Step 4: Scope Check

Before writing any code, confirm your boundaries:

- **Read the `[package]` tag** on your task — that's your workspace
- **Do not modify files outside your tagged package** unless the task explicitly requires it
- **Check git status first** — if you see uncommitted changes in your package's files, STOP and report to the user. Another agent may have been working here
- **Read existing code** in the files you plan to modify before making changes. Understand patterns, naming conventions, and structure already in place

**SEO & Monetization scope check** — ask these before writing a single line:

- **Adding a new predefined Bnto?** — It needs a dedicated URL slug and server-side metadata. This is not optional. If the task doesn't mention it, add it. See `.claude/rules/pages.md`.
- **Adding execution logic?** — Does it increment `runsUsedThisMonth` in Convex? If not, why not? Every execution counts against the user's quota.
- **Building a user-facing flow?** — If the user could be near their run limit during this flow, does the UI surface that? Does hitting the limit show the honest upgrade prompt?
- **Sprint 2 or earlier?** — Execution events must be logged from the first real run. Even anonymously. The data must exist before Sprint 3 builds the dashboard on top of it.

## Step 5: Implement

Write the code for your task. Follow the rules in `CLAUDE.md` and `.claude/rules/`:

### Component Philosophy (CRITICAL)

- **Components are dumb** — they receive data and render UI. That's it. No API calls, no business logic, no domain state in render. All data flows through `@bnto/core` hooks
- **One component/hook per file** — every exported component or hook gets its own file. No multi-component files. Use folder + barrel export (`index.ts`). Only exception: shadcn primitives (thin `forwardRef` wrappers with no logic)
- **Folder organization** — components (PascalCase `.tsx`) at folder root, hooks in `hooks/` subdirectory (`use-kebab-case.ts`), pure functions in `utils/` subdirectory (`kebab-case.ts`). Only create subdirectories when needed. Test files co-locate next to implementation
- **Props are domain objects, not destructured primitives** — pass `workflow` not `name, description, status, nodeCount, ...`
- **Compound composition** — compose complex UI from small parts (Radix pattern), not by adding props. `<Card><CardHeader>...</Card>` not `<Card header={...} />`
- **Primitives vs business components** — generic reusable components (Button, Card, Badge) go in `primitives/`. Domain-specific components (WorkflowCard, ExecutionTimeline, NodeEditor) go in `components/`

### Layered Code Organization

- **Pure functions -> hooks -> components** — extract business logic into pure testable functions (< 20 lines), hooks are thin reactive wrappers (< 30 lines), components just render
- **Hook decomposition** — if a hook does fetching + transformation + subscription + side effects, split it into focused sub-hooks. Signs it's too big: >30 lines, multiple unrelated state, hard to name without "and"
- **Bento Box Principle** — every file < 250 lines, every function < 20 lines. No utility grab bags, no god objects. See `.claude/rules/code-standards.md` for the full checklist

### Go Code Standards (CRITICAL)

- **One concept per file, one purpose per function** — same Bento Box Principle applies to Go
- **Error handling**: Always wrap errors with context — `return fmt.Errorf("loading workflow %s: %w", path, err)` not bare `return err`. Never swallow errors
- **Context propagation**: Pass `context.Context` through the chain. Check cancellation in loops and before expensive operations
- **Interface design**: Small, consumer-defined. Accept interfaces, return structs. No mega-interfaces
- **Package boundaries**: `engine/` orchestrates (doesn't do I/O), `registry/` registers (doesn't execute), `node/` types execute (don't know about other types), `storage/` persists, `validator/` validates. No circular deps

### Other Standards

- **TypeScript:** infer types, no `any`, no gratuitous `as` assertions, types flow down from core
- **Import discipline:** UI from local `@/components/`, data from `@bnto/core`, never skip layers. Third-party UI deps should be wrapped locally
- **Transport-agnostic API:** Components NEVER call Convex or Wails directly. All data access via `@bnto/core` hooks
- Match existing patterns — look at sibling files for naming, structure, and style

### UI Reference: shadcn-blocks

**Before building any UI**, check shadcn-blocks for patterns and inspiration:

**shadcn-blocks** (`/Users/ryan/Code/shadcn-blocks/blocks/`) — A library of well-composed, production-quality component examples. Browse the relevant block category for your task and pick the best variant to adapt. Key categories for Bnto:

- `data-table/` — sortable tables, pagination, row selection (workflow lists, execution history)
- `sidebar/`, `application-shell/` — navigation, rail layouts (main app shell)
- `cards/`, `stats-card/` — dashboard cards, stat displays (workflow status, execution metrics)
- `settings-profile/` — settings pages, edit forms
- `onboarding/` — split-screen layouts, upload zones (workflow import)
- `project/`, `projects/` — project cards, article layouts (workflow detail pages)

**Don't copy blindly** — adapt the layout, structure, and interaction patterns to fit our design system. The value is in the *composition patterns*, not the exact styling.

## Step 6: Verify — Code Review + Automated Checks

### 6a: Code Review

Run `/code-review` to audit all your changes against the project's coding standards, architecture rules, and known gotchas. Fix any violations before proceeding. This is a critical quality gate — do not skip it.

### 6b: Automated Checks

Run ALL checks. Do not skip any even if you think your changes are safe:

### Go checks
```bash
task vet               # go vet — must pass clean
task test              # engine tests with race detector — must pass
task api:test          # API server tests with race detector — must pass
```

### TypeScript checks
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

## Step 7: Verify — Test Coverage

Your work MUST include tests. Determine which type based on what you built:

- **Go engine logic** (node execution, validation, path resolution) -> **Unit tests** in `archive/engine-go/pkg/*/` next to the source. Cover happy path, error cases, and edge cases.
- **Go API endpoints** (`archive/api-go/`) -> **Integration tests** using `httptest`. Cover request/response contracts.
- **Core hooks/adapters** (`@bnto/core`) -> **Unit tests** using Vitest in `packages/@bnto/core/`.
- **Backend functions** (`@bnto/backend`) -> **Unit/integration tests** in `packages/@bnto/backend/__tests__/`.
- **Configuration or type-only changes** -> Tests not required.

**No exceptions.** If your task adds a function and you didn't write a test, you're not done. Go back and write the tests before proceeding.

### Did you touch UI?

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

### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes but is not limited to:

- **Screenshots** — If you changed visual output, delete stale `.png` files. They regenerate on the next e2e run with `--update-snapshots`.
- **Test assertions** — If you changed behavior, props, APIs, or DOM structure, update any tests that assert on the old behavior.
- **Code references** — If you renamed, removed, or changed exports, props, or interfaces, find and update all consumers.
- **Documentation** — If you changed behavior that's documented in comments, JSDoc, or markdown, update the docs to match.

**How to find stale references:** Search the codebase (`Grep`) for the specific things you changed — class names, prop names, component names, function signatures, selectors, text strings. If something references the old version, fix it.

**Do not skip this.** Leaving stale artifacts behind breaks CI, confuses other developers, and wastes everyone's time debugging phantom failures.

## Step 8: Verify — Proof of Work

After all checks pass, provide a summary:

1. **Did you touch UI?** — Yes or No. If you created, modified, or wired up any component, dialog, form, page, or layout — the answer is Yes.
2. **If yes:** What e2e tests did you write or update? List spec files and the flows they cover. List screenshot assertions. **Confirm you visually inspected each screenshot using the Read tool** and describe what you see. If no e2e tests, explain why and confirm user approved the skip.
3. **If no UI touched:** What unit/integration tests did you write? List test files and what they cover.
4. **Go checks result** — confirm `task vet`, `task test`, `task api:test` passed clean
5. **TS checks result** — confirm `task ui:build`, `task ui:test`, `task ui:lint` passed clean
6. **Files changed** — files created/modified, with brief description of each

## Step 9: Update the Plan

Edit `.claude/PLAN.md`:
- Change your task from `- [ ] **CLAIMED**` to `- [x]` (mark done)
- If your completion unblocks the next wave (all tasks in current wave are now `[x]`), note this in your summary so the user knows to start new agents on the next wave

## E2E Testing

All E2E tests run against the full dev stack (Next.js + Convex) on port 4000. There is no "UI-only" mode — the backend must always be running.

**IMPORTANT: You CAN and SHOULD start the dev stack yourself.** Do NOT skip E2E testing because "the stack isn't running." You have the Bash tool — use it. Start `task dev` in the background and wait for port 4000 to respond before running tests.

**Running E2E tests:**

```bash
# Step 1: Check if dev stack is already running
curl -s -o /dev/null -w "%{http_code}" http://localhost:4000 || echo "not running"

# Step 2: If not running, start it in the background
task dev &

# Step 3: Wait for the dev server to be ready (port 4000)
# Poll until you get a 200 response:
for i in $(seq 1 30); do
  curl -s -o /dev/null -w "%{http_code}" http://localhost:4000 2>/dev/null | grep -q "200" && break
  sleep 2
done

# Step 4: Run E2E tests
task e2e
```

**Key details:**
- All E2E tests use `playwright.config.ts` (targets port 4000, Next.js + Convex)
- `reuseExistingServer: true` — if `task dev` is already running, Playwright uses it
- Test fixtures are shared with the Go engine (`engine/tests/fixtures/`)

**Progress-aware test helpers** (in `e2e/integrationHelpers.ts`):
- `waitForSession(page)` — wait for anonymous session to establish
- `waitForUserId(page)` — wait for `data-user-id` attribute to populate
- `waitForPhase(page, phase, screenshot?)` — wait for RunButton `data-phase` and optionally snapshot
- `waitForExecutionStatus(page, status, screenshot?)` — wait for ExecutionProgress `data-status` and optionally snapshot
- `captureTransientPhase(page, phases, screenshot)` — try to capture a fleeting phase without failing
- `captureUploadProgress(page, screenshot)` — snapshot upload file items if visible
- `runPipeline(page, { files, debugLabel })` — upload files, click Run, wait for terminal phase
- `assertCompletedWithScreenshot(page, phase, screenshot)` — assert completed + capture results

**Data attributes for E2E observability:**
- `data-testid="run-button"` + `data-phase` — RunButton lifecycle (idle, uploading, running, completed, failed)
- `data-testid="execution-progress"` + `data-status` — ExecutionProgress status
- `data-testid="node-progress"` + `data-node-id` + `data-node-status` — per-node progress
- `data-testid="upload-file"` + `data-file-status` — per-file upload progress
- `data-testid="execution-results"` — results panel container
- `data-testid="output-file"` — individual output file items
- `data-testid="bnto-shell"` + `data-session` + `data-user-id` — session and identity state

## DO NOT

- **Do not commit or push.** Leave changes staged or unstaged. The user will review and commit
- **Do not modify files outside your package scope** — other agents may be working there
- **Do not modify `CLAUDE.md`, `.claude/rules/`, or config files** unless your task explicitly requires it
- **Do not install new dependencies** without noting it in your summary. If a dependency is needed, prefer one already in the monorepo
- **Do not delete or rename existing exports** — other agents or existing code may depend on them
- **Do not run `pnpm dev` or standalone dev servers** — use `task dev` for E2E tests, and run it in the background

## Multi-Agent Awareness

- **File conflicts:** If you need to modify a file and see it has been recently changed (check `git diff`), read the current state carefully before editing. Work with what's there, not what you expected
- **Schema changes:** If your task adds to any schema, append — don't reorganize existing structures. Other agents may depend on the current structure
- **Shared indexes/exports:** If you add to a barrel export (`index.ts`), add your entries at the end to minimize merge conflicts
- **Port conflicts:** Only start `task dev` when running E2E tests (and check if it's already running first). For non-E2E verification, use the checks (`task vet`, `task test`, `task ui:build`, `task ui:test`, `task ui:lint`)
