# PRE-COMMIT Checklist (MANDATORY)

**CRITICAL:** Before committing ANY code, you MUST run through this entire checklist. If any item fails, STOP, fix the issue, and restart from the beginning.

**IMPORTANT -- No Ignoring Failures:** You are NOT allowed to deem any issues as "pre-existing" or ignore them on your own. If automated checks fail for ANY reason (even in packages you didn't modify), you MUST report ALL failures to the user and let them decide whether to proceed. Only the user can determine if an issue is ignorable.

## Step 1: Automated Checks

```bash
# Rust checks (if engine/ files changed)
task wasm:lint          # clippy (Rust linter) -- must pass clean
task wasm:test          # Rust unit tests + WASM integration tests -- must pass
task cli:test           # CLI integration + golden tests -- must pass

# Frontend checks
task ui:build           # TypeScript compilation -- must pass
task ui:test            # Frontend tests -- must pass
```

Or run `task check` to execute all of the above in one command.

If any check fails: fix the errors, re-run from the top.

### Lighthouse CI audit (if `apps/web/` files changed)

```bash
task seo:audit         # Build + run Lighthouse against all public routes -- must pass
```

Lighthouse CI also runs as a GitHub Actions workflow (`lighthouse.yml`) on every PR. Error-level assertions (accessibility, best practices, SEO >= 90) block merge. Performance warnings are advisory. If audits fail, run `/lighthouse-audit --local` to triage and fix.

## Step 2: Architecture & Bento Box Compliance

For EACH file you modified, verify against the Bento Box Principle (`code-standards.md`):

- [ ] **Layered Architecture**: CLI links engine directly. Web: Apps -> `@bnto/core` -> Engine (Rust WASM). No layer skipping.
- [ ] **API Abstraction**: No direct Convex queries/mutations in components. All data access via `@bnto/core`.
- [ ] **Component complexity**: Logic inline is fine. Extract a hook only when the component earns it (~20+ lines of logic, reuse needed, or testability). No mandatory hooks for simple components.
- [ ] **Pure Functions -> Logic Hooks -> Components**: Business rules in pure functions (no React). Logic hooks compose them reactively -- extract when complex or shared, not for every component.
- [ ] **One Export Per File**: Every exported component, hook, or function in its own file. No `hooks.ts` grab bags, no `utils.ts` grab bags, no multi-component files. Folder + barrel export for related pieces. Only exception: shadcn primitives.
- [ ] **Single Responsibility**: TS files target 50-100 lines, hard cap 250. TS functions < 20 lines. No utility grab bags, no god objects. More than 2-3 sub-components in one file = break into folder + barrel.
- [ ] **Composition**: Small pieces that compose together. Compound components, not mega-prop components.
- [ ] **Flat Named Exports**: ALL multi-part components use flat prefixed exports (`DialogTitle`, `CardHeader`), NOT `Object.assign` dot-notation (`Dialog.Title`, `Card.Header`). Dot-notation breaks React Server Components. If you see `Object.assign` compound patterns, convert to flat exports. Report PASS or FAIL with specific files.
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

| What changed                                  | Verification                                                                        | Screenshot regeneration needed? |
| --------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------- |
| Page layout, routing, chrome, auth forms      | Screenshots (`toHaveScreenshot()`) in `pages/` and `auth/` specs                    | Yes — two-pass regeneration     |
| Execution flows, WASM output, file processing | Programmatic assertions (magic bytes, data attributes) in `journeys/browser/` specs | No                              |
| Components used in both                       | Run all E2E tests, regenerate page-level screenshots only                           | Only if page layout shifted     |

**When to regenerate page-level screenshots:**

If you modified routing, page layout, navbar, footer, or auth forms — regenerate:

```bash
lsof -ti:4000  # check if dev server is running — start `task dev` if not

cd apps/web && pnpm exec playwright test --update-snapshots && pnpm exec playwright test
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
- Agents: check `lsof -ti:4000` first. If a dev server is running, reuse it. If not, start `task dev` yourself

### E2E Verification After Tests

1. **Check test output for `[e2e errors]`** -- the shared fixture logs captured console/page errors. Review each error.
2. **E2E environment** -- ensure `task dev` is running on port 4000 (`lsof -ti:4000` to check). Run `task e2e` to execute both stages (browser parallel, then editor serial).

### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes:

- **Screenshots** -- If you changed page-level layout, regenerate with `--update-snapshots`. Execution flow specs have no screenshots to manage.
- **Test assertions** -- If you changed behavior, update any unit tests that assert on the old behavior.
- **Code references** -- If you renamed, removed, or changed exports, find and update all consumers.
- **Documentation** -- If you changed behavior that's documented, update the docs to match.
- **Engine crate READMEs** -- If you added or changed processors, verify the crate's Processors table is up to date

## Step 7: PLAN.md Task Update (MANDATORY)

**If you picked up this work via `/pickup`, you MUST update PLAN.md before committing.**

1. Run `grep -n "CLAIMED" .claude/PLAN.md` to find any CLAIMED tasks
2. If any CLAIMED tasks are yours, change `- [ ] **CLAIMED**` to `- [x]` in PLAN.md now
3. If a task is partially done, leave it as `- [ ] **CLAIMED**` and add a note about what remains

**This is a hard gate.** Do NOT proceed to the commit step without updating PLAN.md. Skipping this causes other agents to pick up finished work or miss unblocked waves.

## Step 8: Proof of Work Summary

Present a summary to the user before committing:

1. **Branch** -- name of the feature branch this work is on (e.g., `feat/execution-history`)
2. **PR target** -- `main` (always). Confirm you are creating a PR targeting `main`.
3. **PLAN.md updated?** -- Yes (list tasks marked done) or N/A (not a `/pickup` task)
4. **Did you touch UI?** -- Yes or No.
5. **If yes:** What e2e tests did you write or update? List spec files and screenshot assertions.
6. **If no UI touched:** What unit/integration tests were written?
7. **Flat named exports** -- PASS or FAIL. If FAIL, list files with `Object.assign` dot-notation patterns.
8. **TS checks result** -- confirm `task ui:build`, `task ui:test`, `task ui:lint` passed clean
9. **Rust checks result** -- confirm `task wasm:lint`, `task wasm:test` passed clean (skip if no Rust files touched)
10. **Lighthouse audit result** -- confirm `task seo:audit` passed clean, or SKIPPED (no `apps/web/` changes)
11. **Files changed** -- brief description of each

## Step 9: Commit & Branch Workflow

**Branch-based development is mandatory.** `main` is protected — all changes go through PRs with CI gate.

### Branching

1. **Create a feature branch** before committing: `git checkout -b <type>/<short-description>` (e.g., `feat/execution-history`, `fix/skeleton-layout-shift`, `chore/eslint-config`)
2. **Branch naming:** `feat/`, `fix/`, `chore/`, `refactor/`, `test/` prefixes. Lowercase, hyphen-separated.
3. **Never commit directly to `main`.** All changes go through PRs.
4. **Feature branches start from `main`.** Create task branches from `main`: `git checkout -b <type>/<short-description> main`.

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
- **Push to your feature branch**, then create a PR targeting `main`.
- **CI must pass** before merging. The `CI Gate` check (Rust + TypeScript) is required.
- **NEVER force-push to `main`** or merge without CI passing.
- **Always squash merge PRs.** No merge commits, no rebase merges.
- **Ask the user before pushing** if you're unsure. A request to "commit" does not imply "push." A request to "commit and push" authorizes both.

### PR Sizing — Single-Concern PRs (MANDATORY)

**Every PR must represent a single, coherent concern.** A PR that bundles unrelated changes is harder to review, harder to revert, and harder to bisect when something breaks. This is not a suggestion — it's a hard gate.

**Before creating a PR, ask:** "Can I describe this PR in one sentence without using 'and'?" If not, it's too big.

#### Guidelines

| Metric            | Guideline                              | Action if exceeded                                         |
| ----------------- | -------------------------------------- | ---------------------------------------------------------- |
| **Files changed** | ~15 files                              | Split into separate PRs unless all files serve one concern |
| **Lines changed** | ~400 lines (excluding generated files) | Split by concern — extraction, feature, test, config       |
| **Concerns**      | Exactly 1                              | Separate PRs for separate concerns — always                |

These are guidelines, not hard limits. A single-concern refactor touching 25 files is fine. A 10-file PR that mixes a feature, a config change, and an unrelated cleanup is not.

#### What counts as separate concerns

- **Feature + infrastructure** — TUI app shell vs shared palette codegen = 2 PRs
- **Code + unrelated assets** — OG image swap vs theme generation = 2 PRs
- **Refactor + new feature** — extracting a module vs adding behavior to it = 2 PRs
- **Bug fix + cleanup** — fixing the bug vs reformatting nearby code = 2 PRs

#### What can stay together

- **Feature + its tests** — always ship together
- **Codegen source + generated output** — palette.toml + generated CSS/Rust = 1 PR
- **Tightly coupled cross-package changes** — engine processor + TypeScript codegen + recipe definition = 1 PR (they're one concern: "add a node type")

#### Planning for PR sizing

**Think about PR boundaries before writing code, not after.** When picking up a task or plan:

1. Identify the distinct concerns in the work
2. Plan the branch/PR split upfront — one branch per concern
3. Note dependencies between PRs (PR 2 depends on PR 1)
4. If the user explicitly says "ship it all together," that overrides this rule

**When in doubt, split.** Two small PRs that each take 5 minutes to review are better than one large PR that takes 30 minutes and gets rubber-stamped.

### Convex Production Deploy (release-gated)

**Convex production deploys happen during the release pipeline, not on merge to `main`.** The `convex-deploy` job in `.github/workflows/release.yml` runs `npx convex deploy --yes` against the production deployment (`gregarious-donkey-712`) after the release gate passes (stable tags only — skipped for beta/rc pre-releases).

- **No deploy on merge.** Merging to `main` runs CI checks but does NOT deploy Convex to production. Production deploys only happen when a stable release tag (`v*.*.*`) passes the full release gate.
- **Schema changes are safe.** `convex deploy` validates schema changes against existing production data before applying. If validation fails, the deploy job fails and the release author is notified via GitHub Actions.
- **If you changed Convex schema or functions** (`packages/@bnto/backend/convex/`), your changes will go live on production when the next stable release tag is pushed and passes all checks. Make sure schema migrations follow the pattern in [gotchas.md](gotchas.md#convex-schema-migration-production) if you're renaming or changing field types.
