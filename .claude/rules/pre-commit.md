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

- [ ] **Layered Architecture**: Apps -> `@bnto/editor` -> `@bnto/ui` -> `@bnto/core` -> Go Engine. No layer skipping. `@bnto/ui` is presentational only.
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
- [ ] Imports from correct packages (`@bnto/ui` for components, `@bnto/core` for data/actions)

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
- Always set `test.use({ reducedMotion: "reduce" })` to disable animations
- Use `data-testid` markers for reliable state detection
- Use semantic selectors (`getByRole`, `getByText`) over CSS classes

### Stale Artifact Cleanup (MANDATORY)

**After making changes, you MUST clean up anything that your changes have invalidated.** This includes:

- **Screenshots** -- If you changed visual output, delete stale `.png` files. They regenerate on the next e2e run with `--update-snapshots`.
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

When all checks pass, see [commits.md](commits.md) for format rules.
