# PRE-COMMIT Checklist (MANDATORY)

**CRITICAL:** Before committing ANY code, you MUST run through this entire checklist. If any item fails, STOP, fix the issue, and restart from the beginning.

**IMPORTANT -- No Ignoring Failures:** You are NOT allowed to deem any issues as "pre-existing" or ignore them on your own. If automated checks fail for ANY reason (even in packages you didn't modify), you MUST report ALL failures to the user and let them decide whether to proceed. Only the user can determine if an issue is ignorable.

## Step 1: Automated Checks

```bash
# Rust checks
task wasm:lint          # clippy (Rust linter) -- must pass clean
task wasm:test:unit     # Rust unit tests (native) -- must pass

# Frontend checks
task ui:build           # TypeScript compilation -- must pass
task ui:test            # Frontend tests -- must pass
```

Or run `task check` to execute all of the above in one command.

Note: Go checks (`task vet`, `task test`, `task api:test`) are no longer in CI. The Go engine is archived and not actively developed.

If any check fails: fix the errors, re-run from the top.

### Lighthouse CI audit (if `apps/web/` files changed)

```bash
task seo:audit         # Build + run Lighthouse against all public routes -- must pass
```

Lighthouse CI also runs as a GitHub Actions workflow (`lighthouse.yml`) on every PR. Error-level assertions (accessibility, best practices, SEO >= 90) block merge. Performance warnings are advisory. If audits fail, run `/lighthouse-audit --local` to triage and fix.

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

## Step 3: TypeScript Compliance

- [ ] Types inferred where possible (no redundant annotations)
- [ ] No `any` without eslint-disable + justification comment
- [ ] No `Record<string, unknown>` for domain data -- use typed doc interfaces
- [ ] No `as` type assertions unless crossing a trust boundary (e.g. JSON.parse, external API, `Id<T>` -> `string`)
- [ ] Types flow down: core defines types, UI and web consume them. UI never defines its own data types
- [ ] Imports from correct packages (`@bnto/core` for data/actions, local `@/components/` for UI)

## Step 4: Convex Compliance

- [ ] All inputs validated with Convex validators (`v.string()`, `v.id()`, etc.)
- [ ] Auth checks on mutations that modify user data
- [ ] Descriptive errors via `ConvexError`
- [ ] **No `.filter()` on `_id`** -- use `ctx.db.get(id)` for direct document lookups
- [ ] **No N+1 queries** -- batch fetch: deduplicate IDs -> `Promise.all` unique -> `Map` -> join
- [ ] **`.withIndex()` over `.filter()`** -- if an index exists for the field, use it. Check `schema.ts`
- [ ] **`.collect()` awareness** -- avoid `.collect()` on unbounded tables without `.take(n)` or pagination

## Step 5: Code Quality

- [ ] No secrets, API keys, or passwords in code
- [ ] No magic numbers/strings -- use constants and theme tokens
- [ ] No dead code or unused imports
- [ ] Consistent style with existing patterns

## Step 6: Test Coverage Verification

Tests are **mandatory** for most changes. Determine which type:

- **Backend functions** (`@bnto/backend`) -> **Unit/integration tests** using Vitest + convex-test
- **Core hooks/adapters** (`@bnto/core`) -> **Unit tests** using Vitest
- **Pure utils/functions** (any `utils/` directory) -> **Unit tests** co-located next to the source file. No exceptions for utils.
- **Configuration or type-only changes** -> Tests not required.

### E2E Testing: Two Verification Strategies

**Screenshots are for page-level layout** (site navigation, auth forms). **Execution flows are verified programmatically** (magic bytes, data attributes, file sizes, download events).

| What changed | Verification | Screenshot regeneration needed? |
|---|---|---|
| Page layout, routing, chrome, auth forms | Screenshots (`toHaveScreenshot()`) in `pages/` and `auth/` specs | Yes — two-pass regeneration |
| Execution flows, WASM output, file processing | Programmatic assertions (magic bytes, data attributes) in `journeys/browser/` specs | No |
| Components used in both | Run all E2E tests, regenerate page-level screenshots only | Only if page layout shifted |

**When to regenerate page-level screenshots:**

If you modified routing, page layout, navbar, footer, or auth forms — regenerate:

```bash
lsof -ti:4000  # check if dev server is running

# If port 4000 is active:
cd apps/web && pnpm exec playwright test --update-snapshots && pnpm exec playwright test

# If port 4000 is NOT active:
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test --update-snapshots
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test
```

**Intermittent "01 Issue" hydration failures** are known (PopoverTrigger `asChild` SSR mismatch). If the only failures are "01 Issue" overlay detections with zero screenshot mismatches, that's acceptable.

### Did you touch UI?

**If yes -- you MUST write or update e2e tests.** Use programmatic assertions for execution flows (magic bytes, file sizes, data attributes). Use screenshots only for page-level layout verification.

**E2e test conventions:**
- Always import `{ test, expect }` from `./fixtures` (NOT from `@playwright/test`)
- Always set `test.use({ reducedMotion: "reduce" })` to disable animations
- Use shared helpers from `helpers.ts` (`uploadFiles`, `runAndComplete`, `downloadAndVerify`, `navigateToRecipe`, `assertBrowserExecution`)
- Use `data-testid` markers for reliable state detection
- Use semantic selectors (`getByRole`, `getByText`) over CSS classes
- Tag describe blocks with `@browser` (no Convex needed) or `@auth` (needs Convex) for selective test runs
- Agents: check `lsof -ti:4000` first. If a dev server is running, reuse it. If not, use `task e2e:isolated` (port 4001) or start `task dev` yourself

### E2E Verification After Tests

1. **Check test output for `[e2e errors]`** -- the shared fixture logs captured console/page errors. Review each error.
2. **E2E environment** -- agents should check `lsof -ti:4000` first. Reuse running dev server when possible.

### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes:

- **Screenshots** -- If you changed page-level layout, regenerate with `--update-snapshots`. Execution flow specs have no screenshots to manage.
- **Test assertions** -- If you changed behavior, update any unit tests that assert on the old behavior.
- **Code references** -- If you renamed, removed, or changed exports, find and update all consumers.
- **Documentation** -- If you changed behavior that's documented, update the docs to match.

## Step 7: Proof of Work Summary

Present a summary to the user before committing:

1. **Branch** -- name of the branch this work is on (e.g., `feat/execution-history`)
2. **PR** -- confirm you are creating a PR and state which branch it targets (e.g., "Creating PR targeting `sprint/3-platform-features`"). PRs target the sprint branch by default, not `main`.
3. **Did you touch UI?** -- Yes or No.
4. **If yes:** What e2e tests did you write or update? List spec files and screenshot assertions.
5. **If no UI touched:** What unit/integration tests were written?
6. **Dot-notation compliance** -- PASS or FAIL. If FAIL, list files with flat multi-part imports.
7. **TS checks result** -- confirm `task ui:build`, `task ui:test`, `task ui:lint` passed clean
8. **Rust checks result** -- confirm `task wasm:lint`, `task wasm:test:unit` passed clean (skip if no Rust files touched)
9. **Lighthouse audit result** -- confirm `task seo:audit` passed clean, or SKIPPED (no `apps/web/` changes)
10. **Files changed** -- brief description of each

## Step 8: Commit & Branch Workflow

**Branch-based development is mandatory.** `main` is protected — all changes go through PRs with CI gate.

### Branching

1. **Create a feature branch** before committing: `git checkout -b <type>/<short-description>` (e.g., `feat/execution-history`, `fix/skeleton-layout-shift`, `chore/eslint-config`)
2. **Branch naming:** `feat/`, `fix/`, `chore/`, `refactor/`, `test/` prefixes. Lowercase, hyphen-separated.
3. **Never commit directly to `main` or the sprint branch.** All changes go through PRs.
4. **Feature branches start from the sprint branch.** Each active sprint has a long-lived branch (`sprint/<id>-<short-name>`). Create task branches from the sprint branch, not `main`. If no sprint branch exists, ask the user. If the user says to target `main`, use `main` as the base instead.

### Committing

1. Stage only relevant files (no accidental additions)
2. Write a clear commit message:
   - Summarize the "why", not the "what"
   - Keep under 72 characters for the subject line
   - Use imperative mood ("Add feature" not "Added feature")
3. Do NOT include:
   - `Generated with Claude Code` or `Co-Authored-By` lines
   - "Test Plan" sections
   - Unrelated changes bundled together

### Pushing & PRs

- **Only commit YOUR OWN work.** If `git status` shows changes from other agents or unrelated work, DO NOT stage or commit those files. Only stage files you personally created or modified as part of your current task.
- **Push to your feature branch**, then create a PR targeting the sprint branch (or `main` if instructed).
- **CI must pass** before merging. The `CI Gate` check (Rust + TypeScript) is required.
- **NEVER force-push to `main`** or the sprint branch, or merge without CI passing.
- **Ask the user before pushing** if you're unsure. A request to "commit" does not imply "push." A request to "commit and push" authorizes both.

### Convex Production Deploy (automated)

**Merging to `main` triggers an automatic Convex production deploy.** The `convex-deploy` job in `.github/workflows/ci.yml` runs `npx convex deploy --yes` against the production deployment (`gregarious-donkey-712`) after CI Gate passes.

- **No manual deploy needed.** Every merge to `main` deploys Convex functions to production automatically.
- **Schema changes are safe.** `convex deploy` validates schema changes against existing production data before applying. If validation fails, the deploy job fails and the PR author is notified via GitHub Actions.
- **If you changed Convex schema or functions** (`packages/@bnto/backend/convex/`), your changes will go live on production as soon as the PR merges. Make sure schema migrations follow the pattern in [gotchas.md](gotchas.md#convex-schema-migration-production) if you're renaming or changing field types.
