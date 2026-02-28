# PRE-COMMIT Checklist (MANDATORY)

**CRITICAL:** Before committing ANY code, you MUST run through this entire checklist. If any item fails, STOP, fix the issue, and restart from the beginning.

**IMPORTANT -- No Ignoring Failures:** You are NOT allowed to deem any issues as "pre-existing" or ignore them on your own. If automated checks fail for ANY reason (even in packages you didn't modify), you MUST report ALL failures to the user and let them decide whether to proceed. Only the user can determine if an issue is ignorable.

## Step 1: Automated Checks

```bash
# Go checks
task vet                # go vet -- must pass clean
task test               # engine go test -race -- must pass
task api:test           # API server go test -race -- must pass

# Frontend checks
task ui:build           # TypeScript compilation -- must pass
task ui:test            # Frontend tests -- must pass
```

If any check fails: fix the errors, re-run from the top.

## Step 2: Architecture & Bento Box Compliance

For EACH file you modified, verify against the Bento Box Principle (`code-standards.md`):

- [ ] **Layered Architecture**: Apps -> `@bnto/core` -> Go Engine. No layer skipping. UI and editor co-located in `apps/web/`.
- [ ] **API Abstraction**: No direct Convex queries/mutations in components. No direct Wails bindings in components. All data access via `@bnto/core`.
- [ ] **Component complexity**: Logic inline is fine. Extract a hook only when the component earns it (~20+ lines of logic, reuse needed, or testability). No mandatory hooks for simple components.
- [ ] **Pure Functions -> Logic Hooks -> Components**: Business rules in pure functions (no React). Logic hooks compose them reactively -- extract when complex or shared, not for every component.
- [ ] **One Export Per File**: Every exported component, hook, or function in its own file. No `hooks.ts` grab bags, no `utils.ts` grab bags, no multi-component files. Folder + barrel export for related pieces. Only exception: shadcn primitives.
- [ ] **Single Responsibility**: TS files < 250 lines, TS functions < 20 lines. Go files < 250 lines, Go functions < 20 lines. No utility grab bags, no god objects.
- [ ] **Composition**: Small pieces that compose together. Compound components, not mega-prop components.
- [ ] **Dot-Notation Compliance**: ALL multi-part components (including primitives like Dialog, Card, DropdownMenu) use dot-notation (`Dialog.Title`, not `DialogTitle`). If you touched a file with flat primitive imports, migrate them to dot-notation. Report PASS or FAIL with specific files.
- [ ] **Primitives vs Business Components**: Generic in `primitives/`, domain-specific in `components/`.
- [ ] **React Query `select` Rule**: Every `useQuery` that transforms data (`.map()`, `toFoo()`, spread) MUST do it inside `select`. Returning `data ? toFoo(data) : null` or `{ ...data, isLoading }` from the hook body creates new references every render -> infinite loops.
- [ ] **Cost Check**: No new paid services without explicit discussion.

## Step 3: Go Code Compliance

For EACH Go file you created or modified:

- [ ] **Bento Box Principle**: One concept per file, one purpose per function
- [ ] **Error Handling**: Errors wrapped with context (`fmt.Errorf("loading workflow %s: %w", path, err)`), no bare `return err`, no swallowed errors
- [ ] **Context Propagation**: `context.Context` passed through the chain, cancellation checked in loops and before expensive operations
- [ ] **Interface Design**: Accept interfaces, return structs. No mega-interfaces with 10+ methods
- [ ] **Package Boundaries**: Each Go package stays in its lane (engine orchestrates, registry registers, node executes, validator validates)

## Step 4: TypeScript Compliance

- [ ] Types inferred where possible (no redundant annotations)
- [ ] No `any` without eslint-disable + justification comment
- [ ] No `Record<string, unknown>` for domain data -- use typed doc interfaces
- [ ] No `as` type assertions unless crossing a trust boundary (e.g. JSON.parse, external API, `Id<T>` -> `string`)
- [ ] Types flow down: core defines types, UI and web consume them. UI never defines its own data types
- [ ] Imports from correct packages (`@bnto/core` for data/actions, local `@/components/` for UI)

## Step 5: Convex Compliance

- [ ] All inputs validated with Convex validators (`v.string()`, `v.id()`, etc.)
- [ ] Auth checks on mutations that modify user data
- [ ] Descriptive errors via `ConvexError`
- [ ] **No `.filter()` on `_id`** -- use `ctx.db.get(id)` for direct document lookups
- [ ] **No N+1 queries** -- batch fetch: deduplicate IDs -> `Promise.all` unique -> `Map` -> join
- [ ] **`.withIndex()` over `.filter()`** -- if an index exists for the field, use it. Check `schema.ts`
- [ ] **`.collect()` awareness** -- avoid `.collect()` on unbounded tables without `.take(n)` or pagination

## Step 6: Code Quality

- [ ] No secrets, API keys, or passwords in code
- [ ] No magic numbers/strings -- use constants and theme tokens
- [ ] No dead code or unused imports
- [ ] Consistent style with existing patterns

## Step 7: Test Coverage Verification

Tests are **mandatory** for most changes. Determine which type:

- **Go engine logic** (node execution, validation, path resolution) -> **Unit tests** with `go test -race`
- **Go API endpoints** -> **Integration tests** with `httptest`
- **Backend functions** (`@bnto/backend`) -> **Unit/integration tests** using Vitest + convex-test
- **Core hooks/adapters** (`@bnto/core`) -> **Unit tests** using Vitest
- **Pure utils/functions** (any `utils/` directory) -> **Unit tests** co-located next to the source file. No exceptions for utils.
- **Configuration or type-only changes** -> Tests not required.

### E2E Screenshot Regression Gate (MANDATORY)

**Every commit that touches visual output MUST include up-to-date screenshots.** Stale screenshots are broken tests -- they cause false failures for every subsequent agent and developer. There is no excuse for skipping this; the scripts exist.

**When to regenerate:**

Ask yourself: **did you modify ANY file that affects what renders on screen?** This includes components, layouts, pages, CSS/theme tokens, fonts, primitives, animation classes, or anything in `apps/web/` that changes visual output. If yes, you MUST regenerate screenshots.

**How to regenerate:**

```bash
# Step 1: Check if dev server is already running
lsof -ti:4000

# If port 4000 is active (preferred — fast, reuses running server):
cd apps/web && pnpm exec playwright test --update-snapshots   # regenerate
cd apps/web && pnpm exec playwright test                      # verify stable

# If port 4000 is NOT active (use isolated port — starts its own server):
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test --update-snapshots
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test

# Stage the updated screenshots with the rest of your changes
git add e2e/**/__screenshots__/*.png
```

**Both runs are required.** The first regenerates baselines. The second proves they're stable. If the second run has screenshot mismatches, the baselines are flaky -- investigate and fix before proceeding.

**Intermittent "01 Issue" hydration failures** are known (PopoverTrigger `asChild` SSR mismatch). These are NOT screenshot failures. If the only failures in the second run are "01 Issue" overlay detections with zero screenshot mismatches, that's acceptable. Report them to the user but do not block the commit.

### Did you touch UI?

Ask yourself: **did you create, modify, or wire up ANY component, dialog, form, page, or layout that a user will see or interact with?**

**If yes -- you MUST write or update e2e tests with screenshot assertions.** This is non-negotiable. Unit tests alone are not proof that UI works.

**Required e2e coverage:**
- Add to or create spec files in `apps/web/e2e/`
- Test the actual user flow, not just that a page renders
- Include `await expect(page).toHaveScreenshot()` assertions
- Run the e2e tests and confirm screenshots are generated
- **VISUALLY VERIFY screenshots** -- use the Read tool to open each new or updated `.png` file and confirm the visual output matches expectations

**If you genuinely believe no e2e test is needed** (e.g., pure internal refactor with zero visual change), you MUST ask the user for explicit approval before skipping.

**E2e test conventions:**
- Always import `{ test, expect }` from `./fixtures` (NOT from `@playwright/test`) -- the shared fixture captures console and page errors automatically
- Always set `test.use({ reducedMotion: "reduce" })` to disable animations
- Use `data-testid` markers for reliable state detection (see `integrationHelpers.ts` for available attributes)
- Use semantic selectors (`getByRole`, `getByText`) over CSS classes
- Add `await page.evaluate(() => window.scrollTo(0, 0))` before `toHaveScreenshot()` in tests where user actions may shift the viewport (e.g., clicking Run triggers errors that scroll to footer)
- Agents: check `lsof -ti:4000` first. If a dev server is running, reuse it (`cd apps/web && pnpm exec playwright test`). If not, use `task e2e:isolated` (port 4001) or start `task dev` yourself. Never kill the user's dev server on port 4000.
- Use progress-aware helpers from `integrationHelpers.ts` (`waitForPhase`, `waitForExecutionStatus`, `captureTransientPhase`, etc.) to observe and snapshot execution progress

### E2E Screenshot Verification (MANDATORY)

After running E2E tests, agents MUST verify screenshot health:

1. **Check test output for `[e2e errors]`** -- the shared fixture logs captured console/page errors with this prefix. Review each error. If an error indicates a real bug (not an expected "no backend" failure), investigate and fix before committing.
2. **Visually inspect each new or updated screenshot** -- use the Read tool to open `.png` files. Check for:
   - **Next.js error overlay** (red "1 Issue" badge in bottom-left) -- if present, it means an unhandled error occurred. Investigate the root cause via the `[e2e errors]` output.
   - **Wrong viewport position** -- screenshot shows footer/header instead of the expected tool UI. Add `scrollTo(0, 0)` before the screenshot call.
   - **Missing or garbled content** -- indicates a rendering issue or missing data.
3. **E2E environment** -- agents should check `lsof -ti:4000` first. If a dev server is running on port 4000, reuse it (fastest path). If not, use `task e2e:isolated` (port 4001, starts its own Next.js) or start `task dev` yourself. Never kill the user's dev server.

### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes:

- **Screenshots** -- If you changed visual output, regenerate with `--update-snapshots` (see Screenshot Regression Gate above). Never delete screenshots without regenerating -- stale baselines break every subsequent test run.
- **Test assertions** -- If you changed behavior, update any unit tests that assert on the old behavior.
- **Code references** -- If you renamed, removed, or changed exports, find and update all consumers.
- **Documentation** -- If you changed behavior that's documented, update the docs to match.

## Step 8: Proof of Work Summary

Present a summary to the user before committing:

1. **Did you touch UI?** -- Yes or No.
2. **If yes:** What e2e tests did you write or update? List spec files and screenshot assertions.
3. **If no UI touched:** What unit/integration tests were written?
4. **Dot-notation compliance** -- PASS or FAIL. If FAIL, list files with flat multi-part imports.
5. **Lint/typecheck/test results** -- confirm all pass (Go + TS), note test counts
6. **Files changed** -- brief description of each

## Step 9: Commit

When all checks pass:

1. Stage only relevant files (no accidental additions)
2. Write a clear commit message:
   - Summarize the "why", not the "what"
   - Keep under 72 characters for the subject line
   - Use imperative mood ("Add feature" not "Added feature")
3. Do NOT include:
   - `Generated with Claude Code` or `Co-Authored-By` lines
   - "Test Plan" sections
   - Unrelated changes bundled together

**CRITICAL -- Scope and push rules:**
- **Only commit YOUR OWN work.** If `git status` shows changes from other agents or unrelated work, DO NOT stage or commit those files. Only stage files you personally created or modified as part of your current task.
- **NEVER push without explicit user confirmation.** After committing, ask the user if they want you to push. A request to "commit" does not imply "push." A request to "commit and push" authorizes both. When in doubt, ask.
