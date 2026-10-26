# Open Issues: ISS-7 to ISS-18 — 15 PRs

**Created:** October 24, 2026
**Status:** Proposed, waiting for Ryan's approval. ISS-12 needs a product decision before PRs 7–8. Part B's decisions were made on October 24, 2026: rebuild the handoff work here, remove `precision`, keep codegen in TS.
**Related:** [ISSUES.md](../ISSUES.md), the dogfood-sessions handoff (October 21 sessions, summarized in Part B), [feature-planning.md](../scopes/process/feature-planning.md), [function-length.md](../decisions/function-length.md) (the enforce-in-tooling precedent)

## Context

Seven defects are open after the ISS-1 to ISS-6 run (#734–#742). Five were found while fixing those, and two more (ISS-12, ISS-13) turned up while researching this plan. They fall into three groups:

- **Gates that miss things:**
  - ISS-9: Turbo's lint cache doesn't hash the root ESLint config.
  - ISS-10: `task check` skips the crate-publish checks that CI runs.
  - ISS-13: CI never builds, lints or tests `@bnto/i18n`.
- **CLI and tests:**
  - ISS-8: about 28 test files run `bnto` against the developer's real `~/.bnto`.
  - ISS-11: `bnto dry-run <path>` refuses paths, even though its own help text and the consent prompt both offer it.
- **Telemetry:**
  - ISS-12: released CLIs send nothing, because the PostHog key is read only from a runtime env var that nothing sets.
  - ISS-7: the notice links `bnto.io/telemetry`, which is a 404.

Each gate fix also adds a check, so the gap can't quietly come back (Ryan: tooling should block it, not discipline).

**What changes:** CI workflows, `turbo.json`, `Taskfile.yml`, two scripts, CLI test files, `bnto dry-run`, one new web page (`/telemetry`), and possibly how release builds get the telemetry key.

**What doesn't change:** no node, processor, recipe or node-type counts, and no engine crates other than `bnto`. The sitemap, Lighthouse and URL-collection lists each grow by one page (PR 8); nothing asserts their exact counts.

Split into 8 PRs. PRs 1–5 are independent, and PR 6 depends on PR 5. PRs 7–8 wait on the ISS-12 decision.

---

## PR 1: CI builds, lints and tests every workspace package (ISS-13)

**Branch:** `fix/ci-all-packages` from `main`
**One sentence:** CI picks up packages from the workspace instead of a hand-written list.

### What

`ci.yml` lists packages one per step for build (l.270–294), lint (l.302–326) and test (l.333–354). `@bnto/i18n` is in none of them, though it has `build`, `test` and `lint` scripts. Replace each list with one `pnpm turbo run <task>` over the workspace. CI has no Turbo cache, so every task runs, and a new package is included automatically. Do the same in `release.yml` if it repeats the lists.

### Files (~0 new, 4 modified)

- `.github/workflows/ci.yml` and `.github/workflows/release.yml` (both repeat the list), plus `.github/workflows/lighthouse.yml`, which had a third copy
- the `web` change filters in `ci.yml` and `release.yml` gain `turbo.json` and `pnpm-workspace.yaml`, since they now decide what the job runs
- `.claude/ISSUES.md`: delete ISS-13

### Key change

```yaml
- name: Build packages
  run: pnpm turbo run build
```

The same goes for lint and test. No `--filter`: one would skip any new app under `apps/`, and CI should run exactly what `task check` runs. The only cost is `@bnto/desktop`'s three `echo` placeholders. On Actions, Turbo groups each task's output under its name (`@bnto/i18n:test`), so the logs stay readable.

### RED tests

- `grep -c i18n .github/workflows/ci.yml` is 0 today, and CI's logs never show `@bnto/i18n:test`.
- After the change, the PR's TypeScript job log shows `@bnto/i18n:build`, `:lint` and `:test`.

### Verification

```bash
pnpm turbo run build lint test --dry=json | jq -r '.tasks[].taskId' | sort | grep i18n
gh pr checks <n>   # TypeScript job log lists @bnto/i18n tasks
```

### No count changes

---

## PR 2: Turbo hashes the root files its tasks read (ISS-9)

**Branch:** `fix/turbo-root-inputs` from `main`
**One sentence:** Each Turbo task's inputs include the root files it reads.
**Status:** Built (October 24, 2026). It differs from the first draft of this section in four ways, listed under "As built".

### What

Six root files are read by tasks but hashed by none:

| File                                 | Task                                                                       |
| ------------------------------------ | -------------------------------------------------------------------------- |
| `eslint.config.base.mjs`             | every package's `lint`                                                     |
| `engine/catalog.snapshot.json`       | `@bnto/nodes#test`                                                         |
| `engine/crates/bnto-core/src/lib.rs` | `@bnto/nodes#test` (the Rust format-version sync test)                     |
| `README.md`                          | `@bnto/registry#test`                                                      |
| `.prettierrc`                        | `@bnto/registry#test` (`renderReadme` formats with the config it resolves) |
| `engine/Cargo.toml`                  | `@bnto/web#build` and `@bnto/web#test` (`getBntoCliVersion` and its test)  |

Add them as per-task `inputs` (`$TURBO_DEFAULT$` plus `$TURBO_ROOT$/…`), not `globalDependencies`, which would invalidate every task's cache on a lint-rule change. A check fails if a package's ESLint config imports a file outside the package that its lint inputs don't list.

### As built

- **Package configs for the one-package reads.** Only the root `lint` gets root inputs, since every package's lint reads the base config. The other reads go in `turbo.json` files in `packages/@bnto/nodes`, `packages/@bnto/registry` and `apps/web` (`"extends": ["//"]`), so a README edit re-runs one test task, not every package's. A package config inherits the root task's `dependsOn` and `outputs`; `--dry=json` shows the same resolved definitions and inputs for all 33 tasks as root `<pkg>#<task>` entries did, which would have had to copy them.
- **Two reads the draft missed:** `.prettierrc` (registry test) and `engine/Cargo.toml` in `@bnto/web#test`.
- **The check is TypeScript, not bash:** `scripts/turbo/missingLintInputs.ts` (pure, tested with `node:test`) and `scripts/turbo/checkLintInputs.ts` (reads `pnpm turbo run lint --dry=json`). Path resolution in bash needs `realpath -m`, which macOS lacks. It finds imports with TypeScript's `ts.preProcessFile` (static, re-exported and dynamic; comments and strings skipped) and follows a root file's own relative imports.
- **Only ESLint imports are checked.** A test's `readFileSync` of a root file builds its path at run time, which no static check reads reliably; [gotchas.md](../rules/gotchas.md#turbo-hashes-only-a-packages-own-files) says to add the input in the same PR.

### Files (6 new including 1 test, 6 modified)

**New:**

- `scripts/turbo/missingLintInputs.ts`, `scripts/turbo/missingLintInputs.test.ts`, `scripts/turbo/checkLintInputs.ts`
- `packages/@bnto/nodes/turbo.json`, `packages/@bnto/registry/turbo.json`, `apps/web/turbo.json`

**Modified:**

- `turbo.json`: `lint` inputs
- `Taskfile.yml`: `check:turbo-inputs` (the tests, then the check), added to `check:`
- `.github/workflows/ci.yml`: one step in the TypeScript job's hygiene section
- `.claude/rules/gotchas.md`: "Turbo Hashes Only a Package's Own Files"
- `.claude/ISSUES.md`: ISS-9 deleted
- `.claude/strategy/open-issues-plan.md`: this section

### Key change

```json
"lint": { "inputs": ["$TURBO_DEFAULT$", "$TURBO_ROOT$/eslint.config.base.mjs"] }
```

### RED tests

- With a rule that fails every file added to `eslint.config.base.mjs`, `pnpm turbo run lint --filter=@bnto/auth` replays the cache (same hash, exit 0) while `eslint .` in the package exits 1. After the fix it's a cache miss and exits 1.
- `task check:turbo-inputs` against `main`'s `turbo.json` exits 1, naming all ten `*#lint` tasks that have an ESLint config.
- Appending a line to any of the six files changes only the hashes of the tasks that read it (dry run).
- The check's unit tests find an import after a `"**/*.tsx"` glob string, an `export … from` and a dynamic `import()`. A regex version missed the first and last.

### Verification

```bash
task check:turbo-inputs
task check
```

### No count changes

---

## PR 3: `task check` runs the crate publish checks (ISS-10)

**Branch:** `fix/check-crates` from `main`
**One sentence:** `task check` runs CI's crate publish-list and own-version checks.
**Status:** Built (October 26, 2026). Offline, the own-version check also fails when `CI` is set, so a crates.io outage can't pass CI's job silently. The `((errors++))` exit only happens on bash 4.1+ (CI's Ubuntu); macOS's bash 3.2 survives it, so the fixture test turns red only in CI. As built, it adds two test files (`scripts/check-crate-publish-list.test.ts`, `scripts/check-own-version-crates.test.ts`).

### What

Add a `check:crates` task that runs `scripts/check-crate-publish-list.sh` and `scripts/check-own-version-crates.sh`, and add it to `check:`. Fix two script bugs found while researching this plan:

- The own-version script treats a failed crates.io request as "isn't on crates.io yet: it will publish" and exits 0, so offline it passes silently. It should instead print a loud "skipped, CI checks it" warning.
- `((errors++))` under `set -e` exits on the first mismatch, before the summary prints.

### Files (~0 new, ~5 modified)

- `Taskfile.yml`: `check:crates`, added to `check:` with an `=== crate publish checks ===` header
- `scripts/check-own-version-crates.sh`: tell offline apart from unpublished
- `scripts/check-crate-publish-list.sh`: `errors=$((errors + 1))`
- `.claude/rules/pre-commit.md`: the "what `task check` runs" list, which is already stale (it lacks `tui:test`, `theme:check`, `docs:validate` and the dogfood checks)
- `.claude/scopes/process/releases.md`: note that `task check` now runs the own-version check

### RED tests

In a temp worktree (never the main checkout):

- Set tonkotsu back to 0.1.2 in `crates/tonkotsu/Cargo.toml` and `engine/Cargo.toml`. `task check:crates` exits 1 with `tonkotsu changed since 0.1.2 was published`. This depends on 0.1.3 having shipped in v0.18.2; until then, any un-bumped edit works.
- With the network blocked (`HTTPS_PROXY=http://127.0.0.1:9`), it prints the skip warning. It doesn't print "will publish".
- A publish-list fixture with two mismatches reports both.

### Verification

```bash
task check:crates
task check
```

### No count changes

---

## PR 4: `bnto dry-run` accepts a recipe file path (ISS-11)

**Branch:** `fix/dry-run-path` from `main`
**One sentence:** `bnto dry-run` reads a slug or a `.bnto.json` path, as `bnto run` does.

### What

Three places read "slug, else file" on their own: `run.rs:145` reads both, `install.rs:311` is a copy, and `bnto dry-run` reads only the catalog. That means `bnto dry-run <path>` fails with "Unknown recipe", although the CLI help (`cli.rs:149`) and the consent prompt (`consent.rs:42`) both offer it. Move the lookup into one function, and have `dry_run_recipe` take the definition JSON instead of a slug.

### Files (~2 new, ~6 modified)

**New:**

- `engine/crates/bnto/src/recipe_source.rs`
- `engine/crates/bnto/tests/dry_run_path.rs`

**Modified:**

- `src/run.rs` and `src/install.rs`: use `recipe_source`
- `src/inspect.rs`: read the definition once, then pass it to the host, unfilled-field, trust and preview steps
- `src/dry_run/mod.rs`: `dry_run_recipe(recipe_arg, definition_json, inputs, params)`
- `engine/crates/bnto/README.md`: dry-run examples
- `.claude/journeys/cli.md`: the run → dry-run suggestion journey

### Key functions

```rust
/// A slug in the catalog, else a .bnto.json path.
pub fn read_recipe(arg: &str, catalog: &RecipeCatalog) -> std::io::Result<String>
pub fn dry_run_recipe(recipe_arg: &str, definition_json: &str, inputs: &[String], params: &[String]) -> Result<DryRunResult, String>
```

### RED tests

**Unit:**

- A slug wins over a file with the same name, a path is read, and neither gives an error.
- The preview's name and description come from the JSON, and the argument is used when the recipe has no name.

**Sandboxed CLI** (`helpers::Sandbox`):

- An absolute path and a relative path to `fixtures/shell-file-input.bnto.json` both preview.
- **The journey:** declining `bnto run <path> -p key=value <input>` prints `bnto dry-run <path> -p key=value` (each word quoted when it needs it), and that exact command then exits 0, for a recipe with a required field too.
- An unfilled `{{fields.*}}` in a path recipe exits 1.
- A missing file exits 1, and so does invalid JSON.

### Verification

```bash
cd engine && cargo test -p bnto --bin bnto recipe_source && cargo test -p bnto --bin bnto dry_run && cargo test -p bnto --no-fail-fast --test dry_run_path --test cli_commands --test unfilled_fields
task check
```

### No count changes

---

## PR 5: Move the unsandboxed CLI tests onto `Sandbox` (ISS-8, part 1)

**Branch:** `test/sandbox-remaining` from `main`
**One sentence:** The 18 CLI test files with no sandbox run `bnto` through `helpers::Sandbox`.

### What

These files start `bnto` with no HOME or BNTO_HOME, so every run writes a session log into the developer's `~/.bnto/state/logs`:

- `assert_files`, `check_edges`, `check_folders`, `check_run_time`, `edges`, `edit_fields`, `file_delete`, `file_inspect`, `file_join`
- `progressive_output`, `report_file_names`, `run_args`, `run_json`, `run_temp_files`, `shell_each`, `shell_options`
- `dry_run_temp_files` and `run_timeout`

All their paths are absolute. Where a test sets its own working directory (`shell_each`, `shell_options`) or temp vars (`file_join`, `run_temp_files`), it sets them after `sandbox.command()`, so they still win. The library workarounds for `bnto dry-run` (`dry_run_temp_files`, `run_timeout`) write into `sandbox.bnto_home()`, or pass the fixture path, which `bnto dry-run` takes since PR 4.

### Files (~0 new, ~19 modified)

- The 18 test files above
- `tests/helpers/sandbox.rs`: add `home()`. PR 4 already added `work_dir()` (the working directory) and `sh()`; use those, don't add a second accessor

### RED tests

A full run of the CLI tests adds no session logs to the real `~/.bnto`:

```bash
before=$(ls ~/.bnto/state/logs | wc -l); (cd engine && cargo test -p bnto --tests --no-fail-fast); test "$before" -eq "$(ls ~/.bnto/state/logs | wc -l)"
```

It fails today (`--test edges --test run_json` alone added 3).

### Verification

```bash
cd engine && cargo test -p bnto --tests --no-fail-fast && cargo clippy -p bnto --all-targets -- -D warnings
task tui:test
```

### No count changes

---

## PR 6: Only `Sandbox` may start `bnto` in tests (ISS-8, part 2)

**Branch:** `test/sandbox-enforced` from `main`, after PR 5
**One sentence:** A guard test fails any CLI test that starts the binary outside `Sandbox`.

### What

- **Move the remaining 10 files onto `Sandbox`.** They build partial sandboxes today: `when_guard`, `scoped_requires`, `dogfood_recipes`, three `telemetry_*`, three `keep_awake*` and `shell_security/support.rs`. Some never set HOME, or leave telemetry able to touch the real config.
- **Enforce it, two ways:**
  - `bnto_bin()` becomes private to `helpers/sandbox.rs`, which is a compile-time block.
  - `tests/sandboxed_spawns.rs` walks `tests/**/*.rs` and fails, naming each file, when `CARGO_BIN_EXE_bnto` appears outside `helpers/sandbox.rs` and the three tests that need the binary path itself (`tui/harness.rs`, `update_notice.rs`, `upgrade.rs`), each of which already sandboxes in full.

### Files (~1 new, ~13 modified)

- The 10 files above
- `tests/helpers/mod.rs`, `tests/helpers/sandbox.rs`
- `.claude/rules/gotchas.md`: "CLI tests start `bnto` only through `Sandbox`"
- `.claude/ISSUES.md`: delete ISS-8

### RED tests

- The guard test fails on today's tree, naming the 10 files.
- A unit test for the guard's matcher, using a fixture string, keeps the guard from matching itself, and normalizes `\` to `/` for Windows.

### Verification

```bash
cd engine && cargo test -p bnto --tests --no-fail-fast && cargo clippy -p bnto --all-targets -- -D warnings
task tui:test
```

The keep-awake files only run on their own OS, so CI's macOS, Linux and Windows jobs are the real check.

### No count changes

---

## PR 7: Ship the telemetry key in release builds (ISS-12) — needs a decision

**Branch:** `fix/telemetry-key` from `main`
**One sentence:** Release builds embed the PostHog key, so CLI telemetry actually reaches PostHog.

### What

`telemetry/mod.rs:57` reads `BNTO_POSTHOG_KEY` from the run's environment, and with no key, telemetry is off (`:60`). Nothing sets it: no `option_env!`, nothing in `release.yml`. So shipped CLIs send nothing, even though the notice says bnto "collects anonymous usage data" and the code comment calls the key "safe to embed". `bnto telemetry status` also reports "Effective: true" without checking for a key (`:134`).

**Decision for Ryan:**

- **(a) Turn it on:** embed the key at build time with `option_env!("BNTO_POSTHOG_KEY")` and the release secret. Keep the runtime variable as an override, and make `status` say when there's no key.
- **(b) Turn it off honestly:** drop the first-run notice and say telemetry is off until a key ships.

Option (a) matches the notice and the opt-out design. The web app already uses PostHog. Note that PostHog will see each request's IP address. There's no geoip opt-out today, and the page in PR 8 must say so.

### Files for option (a) (~0 new, ~4 modified)

- `src/telemetry/mod.rs`: compile-time key with a runtime override
- `src/telemetry/mod.rs` `print_status`: report "no key: nothing is sent"
- `.github/workflows/release.yml`: pass the `BNTO_POSTHOG_KEY` secret to the CLI build and crates publish steps, if the published crate should carry it (crates.io builds from source; see the open question below)
- `.claude/environment-variables.md`

**Open question:** `cargo install bnto` builds from crates.io source on the user's machine, so a build-time env var on our side never reaches those builds. Embedding the key there would mean putting it in the published source, as a constant in the code. PostHog project keys are write-only and public by design; the web bundle already ships one.

### RED tests

- With `BNTO_POSTHOG_KEY` unset, `status` names the missing key (unit test on a pure `effective()` function).
- A compile-time key is used when no runtime value is set.

### No count changes

---

## PR 8: A `/telemetry` page that says what the CLI sends (ISS-7)

**Branch:** `docs/telemetry-page` from `main`, after the PR 7 decision
**One sentence:** bnto.io/telemetry documents the CLI's four events, what's never sent, and how to opt out.

### What

The notice's link (`consent.rs:27`) renders "Page Not Found", with HTTP 200 and the `not-found-heading`. Add a static page, mirroring `/privacy` (`page.tsx`, `layout.tsx` metadata, `telemetry.mdx`), every claim tied to the code. The page covers:

- The four events and their properties.
- **Recipe names:** bundled slugs only, anything else sent as `"custom"`.
- **Buckets:** duration, size and count, never exact values.
- **CI detection.**
- **The anonymous ID:** what it is and where it's stored.
- **Never sent:** file names, paths, `-p` values, error text, OS or version.
- **Opt-outs:** the three ways to turn it off.
- **See it yourself:** `BNTO_TELEMETRY_DEBUG=1`.

The wording depends on PR 7: "when telemetry is on" versus "currently off".

### Files (~4 new, ~7 modified)

**New:**

- `apps/web/app/(app)/telemetry/{page.tsx, layout.tsx, telemetry.mdx}`
- `apps/web/e2e/pages/telemetry.spec.ts`

**Modified:**

- `apps/web/app/buildStaticSitemapEntries.ts`
- `scripts/generate-lighthouserc.ts`
- `scripts/seo/collectUrls.ts` and its test
- `apps/web/e2e/pages/site-navigation.spec.ts`
- `FooterLinks.tsx`: a Telemetry link
- `privacy.mdx`: a "bnto CLI" paragraph linking the page

### RED tests (e2e, `@browser`)

- `/telemetry` shows a level-1 heading, and `not-found-heading` doesn't appear (the live page is a soft 404, so status alone proves nothing).
- All four event names, `bnto telemetry disable`, `BNTO_TELEMETRY_DISABLED` and `DO_NOT_TRACK` appear.
- `footer-link-telemetry` navigates there.
- `collectUrls` includes `/telemetry`.

### Verification

```bash
lsof -ti:4000   # else: task dev
cd apps/web && pnpm exec playwright test e2e/pages/telemetry.spec.ts e2e/pages/site-navigation.spec.ts
task e2e && task seo:audit && task check
```

### Count changes

- Sitemap static entries: 5 → 6
- Lighthouse static pages: 4 → 5
- `collectUrls` static pages: 5 → 6
- Site-navigation routes: 11 → 12

No test asserts an exact count. Goes live with the next release.

---

## Part B: the dogfood-sessions handoff (ISS-14 to ISS-17, plus one backlog task)

Seven sessions started on October 21. One merged (#707, lossy WebP). Five stopped with uncommitted work in worktrees on the Windows machine, and one produced nothing. None of their branches exist here or on GitHub. **Decided:** rebuild from the handoff's approved designs, here, RED tests first, rather than rebasing about 70 commits of drift. Every claim below was checked against `main` on October 24, 2026.

### PR 9: `image-resize` can leave smaller images alone (ISS-14)

**Branch:** `fix/resize-never-enlarge` from `main`
**One sentence:** `image-resize` gets an `enlarge` switch, and the web and thumbnail recipes turn it off.

**What:** `optimize-images-for-web` resizes to width 800, so a 400×400 image comes out 800×800. That was reproduced today with `test-fixtures/images/medium.jpg`. The approved design is a generic boolean `enlarge`:

- default `true`, label "Enlarge Smaller Images", group `dimensions`
- set to `false` in `optimize-images-for-web` and `generate-thumbnails`

It's a yes/no question about the node's behavior, so the name fits `node-vocabulary.md`'s boolean rule (re-check on the day).

**Files (~0 new, ~12 modified):**

- `bnto-image/src/resize/`: `param_defs.rs`, `params.rs`, `dimensions.rs`, `processor.rs`, `wasm_bridge.rs`
- the two bundled recipes and their CLI fixtures
- the two recipes' goldens (`task cli:golden:bless`, after reviewing the diff)
- codegen output (`task wasm:codegen`)
- the node doc and the crate README

**RED tests:**

- With `enlarge: false`, width 800 on a 400×400 image keeps 400×400; with `true` it gives 800×800; a 1600×1600 image shrinks either way; and a height-only resize follows the same rule.
- `optimize-images-for-web` on `medium.jpg` comes out 400×400.

**Verification:** `task wasm:codegen && task wasm:test && task cli:test && task check`

**Count changes:** none (a param, not a node).

### PR 10: Remove `vector-optimize`'s unused `precision` (ISS-15)

**Branch:** `fix/remove-svg-precision` from `main`
**One sentence:** `vector-optimize` drops the `precision` param it never implemented, and `bnto migrate` drops it from recipes.

**What:** `precision` (default 3, range 0–8) is declared and shown in every UI, but it does nothing; `processor.rs:78` says "reserved for Tier 2". Ryan's call: remove it rather than finish it, which needs no new dependency and matches dogfooding.md's finding that SVG optimization isn't worth it here.

Pre-flight refuses undeclared params, so removal needs `bnto migrate` to learn a new kind of change. Today `renamed.rs` only maps node types and params to new names. Add a `REMOVED_PARAMS` table (node type, param, why), and point the undeclared-param message at `bnto migrate`.

The bundled `optimize-svg` recipe and `tests/fixtures/explicit/optimize-svg.bnto.json` set `precision`, so both drop it.

**Files (~1 new, ~8 modified):**

- `bnto-vector/src/optimize/processor.rs`
- `bnto-core/src/renamed.rs`
- `bnto/src/migrate/mod.rs`, and a new `bnto/src/migrate/removed_params.rs`
- the bundled `optimize-svg` recipe and its explicit fixture
- codegen output
- `.claude/decisions/svg-optimizer.md`

**RED tests:**

- `bnto migrate` removes `precision` from a `vector-optimize` node and leaves other nodes' `precision`-named params alone.
- `bnto check` on an old recipe names `bnto migrate`.
- The catalog no longer lists the param.
- Goldens are unchanged, because the param never did anything.

**Verification:** `task wasm:codegen && task wasm:test && task cli:test && task check`

**Count changes:** none.

### PR 11: `assert`'s `op` value `not_equals` becomes `not-equals` (PLAN.md backlog task)

**Branch:** `fix/assert-not-equals` from `main`
**One sentence:** `bnto migrate` renames `assert`'s snake_case `not_equals` to the kebab-case `not-equals`.

**What:** this is already an audit row in `node-vocabulary.md:232` ("Yes, migrate") and part of PLAN.md's open backlog task **Node Vocabulary Alignment → Shared param meanings**. It isn't logged as an issue, so it isn't tracked twice. This PR does only the `assert` part of that task. Enum values are kebab-case (`:209`), and `when` already uses `not-equals`. It lands before any new comparison node, so none copies the snake_case value.

**Files (~1 new, ~6 modified):**

- `bnto-core/src/control/assert/{compare,params,tests}.rs`
- `bnto-core/src/renamed.rs`: an enum-value table
- `bnto/src/migrate/mod.rs`, and a new `bnto/src/migrate/enum_values.rs`
- `node-vocabulary.md`: close the audit row
- codegen output

**RED tests:**

- `not-equals` passes and fails as `not_equals` did.
- `bnto migrate` rewrites `not_equals` in an `assert` node only.
- `bnto check` on an old recipe names `bnto migrate`.
- Recipes in `recipes/` that use the old value are migrated in the same PR.

**Verification:** `task wasm:codegen && task wasm:test && task cli:test && task check`

**Count changes:** none.

### PR 12: Lint `@bnto/nodes` scripts, with a baseline (ISS-16, part 1)

**Branch:** `fix/lint-nodes-scripts` from `main`
**One sentence:** `@bnto/nodes`'s ESLint config stops ignoring `scripts/`.
**Status:** Built (October 25, 2026), together with PR 13 in one PR on `refactor/split-node-codegen`. Splitting in the same PR left one baseline entry: the entry script's kebab-case file name (`generate-from-catalog.ts`), kept because the generated `nodes.json` names it in `_generatedBy`.

**What:** `packages/@bnto/nodes/eslint.config.mjs:6` ignores `scripts/**`, so `scripts/generate-from-catalog.ts` grew to 1,381 lines (limit 250) unseen. Lint the folder, and record today's `max-lines` and `max-lines-per-function` findings in `eslint-suppressions.json`, the same ratchet as #739. New violations fail at once.

**Files (~0 new, ~2 modified):**

- `packages/@bnto/nodes/eslint.config.mjs`
- `packages/@bnto/nodes/eslint-suppressions.json`

**RED tests:**

- A scratch 23-line function in `scripts/` passes lint today and fails after.
- A stale suppression fails, which keeps the baseline shrink-only.

**Verification:** `cd packages/@bnto/nodes && pnpm exec eslint . && task check`

**Count changes:** none.

### PR 13: Split `generate-from-catalog.ts` under the size limit (ISS-16, part 2)

**Branch:** `refactor/split-node-codegen` from `main`, after PR 12
**One sentence:** The 1,381-line codegen script becomes one module per generated artifact, and its output doesn't change.
**Status:** Built (October 25, 2026), with PR 12, as one PR. The modules are grouped by output under `scripts/codegen/` (`catalog/`, `schemas/`, `fields/`, `files/`, `docs/`), every file under 250 lines and every function under 20, with no function on the baseline. Old and new scripts produced byte-identical output on the real snapshot and on 30 mutated snapshots, and `task nodes:generate` changes no generated file.

**What:** codegen stays in TypeScript (Ryan's call: dogfooding.md's "Not dogfooded, on purpose" stands). Split the script by output: catalog, schemas, `paramFieldInfo`, definition schema, docs, Convex labels, i18n and format version. Each file stays under 250 lines, and its suppressions are pruned. Probably two PRs if a single diff passes about 400 lines; decide on the day.

**Files:** about 10 new under `packages/@bnto/nodes/scripts/codegen/`, plus the trimmed entry script and the suppressions file.

**RED test (acceptance):** after `task nodes:generate`, `git diff --exit-code packages/ engine/catalog.snapshot.json` is clean, so every generated file is byte-identical.

**Verification:** `task wasm:codegen && git diff --exit-code && task check`

**Count changes:** none.

### PR 14: The workflow-expert skill reads the live node list (ISS-17)

**Branch:** `docs/workflow-expert-inventory` from `main`
**One sentence:** The `/workflow-expert` skill points at the generated node docs instead of a hand-kept inventory.

**What:** `.claude/skills/workflow-expert/SKILL.md:30` still says "the 6 implemented browser operations", and its operations table is stale in general. Replace the table with a pointer to the generated catalog (`packages/@bnto/nodes/docs/`, `bnto list`, `bnto info`), so it can't go stale again.

**Files (~0 new, ~1 modified):** `.claude/skills/workflow-expert/SKILL.md`

**RED test:** none (docs only). Check that every node in `engine/catalog.snapshot.json` is reachable from the pointer.

**Count changes:** none.

---

### PR 15: The Windows grandchild test waits for a slow runner (ISS-18)

**Branch:** `test/windows-grandchild-timeout` from `main`
**One sentence:** `terminate_all_kills_orphaned_grandchild_on_windows` stops failing when PowerShell starts slowly.

**What:** the test gives PowerShell one 20 s `recv_timeout` to print its PID (`context.rs:758-759`). It failed the v0.18.2 release's Windows job (run 40096930376), and the same commit had passed on PR CI. Pick one fix:

- Launch the grandchild with a lighter command that prints its PID at once (`cmd`'s own `start /b` of a `ping -n 30 127.0.0.1` loop, read through `tasklist`).
- Keep PowerShell with `-NoLogo -NonInteractive` and a 60 s bound.

What the test proves (job-object cleanup of an orphaned grandchild) mustn't change.

**Files (~0 new, ~1 modified):** `engine/crates/bnto/src/context.rs` (tests module)

**RED test:** run the test 20 times in a row on the Windows CI job (a temporary matrix or a `for` loop step, on the PR's branch only). It should never time out, and the grandchild must still be killed.

**Verification:** the PR's `rust-windows` job, and `cargo test -p bnto --bin bnto terminate_all` on Windows.

**Count changes:** none.

---

## Part C: features and ideas from the handoff (to PLAN.md's backlog, not issues)

These aren't defects, so they go to [PLAN.md's backlog](../PLAN.md#backlog). Each new node or param gets a `/workflow-expert` review against `node-vocabulary.md` before code, per Ryan's standing rule. The handoff's October 21 reviews stand as a starting point.

| Item                                                                                                                                  | State                    | Note                                                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `file-replace` node ("Replace Text in Files", new verb `replace`, `find`/`replace`/`patternMode`/`caseSensitive`, `-replaced` suffix) | Design approved, rebuild | High user demand; independent of dogfooding. Must fit the post-#727 `node_types/` layout and the 400-line and 20-line limits |
| `file-filter` content criterion (`contentPattern`, read but pass `FileData::Path`)                                                    | Design approved, rebuild | Fold into [dogfooding.md](dogfooding.md) row 9 (`feat/content-search`, Phase 3, waits on Ryan's go-ahead)                    |
| `spreadsheet-read` `field` (dotted key to the records), then object entries                                                           | Approved, not started    | Generic; worth doing without the codegen recipe                                                                              |
| Item-level `filter` node (`value`/`op`/`expected` + `keep`) sharing `assert`'s comparison module                                      | Approved, not started    | After PR 11, so it inherits `not-equals`                                                                                     |
| `file-render` `escape: json`                                                                                                          | Approved, not started    | Check against template filters' `tojson` first                                                                               |
| Template filters (pipe filters in the shared interpolator, Jinja-compatible syntax)                                                   | Research done            | Needs a plan per feature-planning.md and a `/workflow-expert` review before code                                             |
| "Cold review" gate before work counts as done                                                                                         | **Done (#745)**          | `/code-review` is now the cold review ([cold-review.md](../decisions/cold-review.md))                                        |
| Node codegen as a recipe (handoff #4 PR 4)                                                                                            | **Declined October 24**  | Codegen stays in TS; PR 13 handles the size problem                                                                          |
| Finishing `precision`                                                                                                                 | **Declined October 24**  | Removed instead (PR 10)                                                                                                      |

## Dependency Chain

```
Part A
PR 1 (CI covers every package, ISS-13)        independent
PR 2 (Turbo root inputs, ISS-9)               independent
PR 3 (task check runs crate checks, ISS-10)   independent
PR 4 (dry-run takes a path, ISS-11)           independent
PR 5 (unsandboxed tests onto Sandbox, ISS-8)  independent
└── PR 6 (Sandbox enforced, ISS-8)            depends on PR 5
Ryan's ISS-12 decision
├── PR 7 (telemetry key, ISS-12)
└── PR 8 (/telemetry page, ISS-7)             wording depends on the decision

Part B
PR 9  (resize never enlarges, ISS-14)         independent
PR 10 (remove precision, ISS-15)              independent
PR 11 (assert not-equals, backlog task)       independent
PR 12 (lint nodes scripts, ISS-16)            independent
└── PR 13 (split codegen script, ISS-16)      depends on PR 12
PR 14 (workflow-expert inventory, ISS-17)     independent
PR 15 (Windows test timeout, ISS-18)          independent
```

**Suggested order:**

1. PRs 15, 1, 2, 3 and 12: get the gates right (and stop the flaky release blocker) before more code lands.
2. PRs 9, 4, 10 and 11: user-facing fixes, with PR 9 (the upscaling) first.
3. PRs 5, 6, 13 and 14.
4. PRs 7 and 8 once ISS-12 is decided, then release, so the page and the key ship together.
