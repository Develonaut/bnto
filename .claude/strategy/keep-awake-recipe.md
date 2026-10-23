# Conditional Steps + Cross-Platform Keep Awake

**Created:** October 20, 2026
**Status:** Done (October 22, 2026). All 11 PRs and review fixes A-F merged; keep-awake runs on macOS, Linux and Windows through `when`-guarded branches. Kept as the record of what was built and why; open items are under [Follow-ups](#follow-ups).
**Related:** [engine-execution.md](engine-execution.md), [generic-building-block-nodes.md](../decisions/generic-building-block-nodes.md), [shell-run.md](../scopes/security/shell-run.md#script-mode-the-safe-script-contract), [engine-node-patterns.md](../scopes/rust/engine-node-patterns.md#checklist-adding-a-new-recipe)

## Tracker

Order of work: PRs 1-2 shipped, then a code review of both (October 21, 2026) added fixes A-F, which land before PR 3. After F comes the [CLI upgrade plan](cli-upgrade.md) (3 PRs), then resume at PR 3.

Shipped alongside: the README's recipe count is generated and checked (#675); every bnto crate shares the release version (#677); the home page's install demo shows it (#678). Releases: v0.16.0 (site: keep-awake page + fixes A-E), v0.16.1 (crates.io).

| #   | PR                                                           | Branch                             | Status        |
| --- | ------------------------------------------------------------ | ---------------------------------- | ------------- |
| 1   | Publish keep-awake as a built-in macOS recipe                | `feat/keep-awake-macos`            | Merged (#662) |
| 2   | Lint bundled recipe scripts with ShellCheck in CI            | `chore/shellcheck-bundled-scripts` | Merged (#665) |
| A   | Make keep-awake's Stop and Start only touch its own session  | `fix/keep-awake-session-safety`    | Merged (#671) |
| B   | Stop `bnto doctor` failing off macOS over keep-awake's deps  | `fix/keep-awake-doctor-deps`       | Merged (#673) |
| C   | Make the System category's web copy and filter accurate      | `fix/system-category-web`          | Merged (#676) |
| D   | Stop CLI-only recipe pages describing a browser tool         | `fix/cli-only-recipe-pages`        | Merged (#674) |
| E   | Limit ShellCheck disables in bundled scripts                 | `chore/shellcheck-hardening`       | Merged (#672) |
| F   | Refresh the new-recipe checklist and stale recipe counts     | `docs/new-recipe-checklist`        | Merged (#679) |
| 3   | Skip a step when its `when` guard doesn't hold               | `feat/when-guard`                  | Merged (#697) |
| 4   | Show skipped steps in the CLI, TUI and previews              | `feat/when-surfaces`               | Merged (#703) |
| 5   | Let a container's fields reach the steps inside it           | `feat/scoped-fields`               | Merged (#708) |
| 6   | Work out the OSes a recipe runs on, refuse unsupported hosts | `feat/recipe-os-support`           | Merged (#710) |
| 7   | Show which operating systems a recipe runs on, on bnto.io    | `feat/web-os-badge`                | Merged (#711) |
| 8   | Scope dependencies to the steps that use them                | `feat/scoped-requires`             | Merged (#712) |
| 9   | PowerShell script mode                                       | `feat/powershell-script-mode`      | Merged (#713) |
| 10  | Keep awake on Linux                                          | `feat/keep-awake-linux`            | Merged (#714) |
| 11  | Keep awake on Windows                                        | `feat/keep-awake-windows`          | Merged (#715) |

Workflow per PR: branch from `main`, RED tests first, `/pre-commit`, push, open a PR, enable squash auto-merge, fix CI. No attribution lines or "Test Plan" sections. Tests never touch the real `~/.bnto` (temp `BNTO_HOME` + `HOME`), and never start or stop a session outside a sandbox.

## Context

The personal `keep-awake` recipe (caffeinate session manager) is being published as a built-in recipe, then made cross-platform. Planning it showed that the engine has **no conditional execution**. The only node-level control key is `always`. Without a conditional, per-OS behavior would need a one-off `platforms` override bolted onto shell-run, which `generic-building-block-nodes.md` flags as a red flag ("build the one generic piece").

Research (n8n If/Switch, Node-RED switch, NiFi RouteOnAttribute, Make Router vs GitHub Actions `if:`, Ansible `when:`, Argo/Tekton `when`):

- Dataflow tools route **per item** through multi-output nodes.
- Step pipelines **guard steps**. Per-OS steps are the textbook guard case (Ansible `when: os_family`, GitHub Actions `if: runner.os`).
- bnto recipes are step pipelines authored in JSON/CLI/TUI (the editor is frozen), so guards fit.
- **Decided:** a node-level `when` guard now. A per-item router is a separate future node next to `file-filter`. if/else = two guards; switch = several guards on one value.

The workflow-expert review confirmed real demand beyond OS: optional steps toggled by a field, "only continue if something was found", and "process only JPGs, pass the rest through untouched". It also found the **fields-scope gap** (see PR 5) and the missing CLI journey matrix.

**What changes:** recipes 21 → 22 (`keep-awake`, first `system` recipe). Node types stay at 39 and processors at 32: `when` is a node key, not a node. New: the `when` key, a `NodeSkipped` progress event, scoped container fields, recipe OS support worked out from guards (`os`), and `requires` on any node, scoped to the OSes it runs on.
**What does NOT change:** node-type `platforms` (browser/cli/server runtime targets) keeps its meaning, with OS as a separate axis. The editor stays frozen; it only has to preserve `when` on round-trip. WASM gets no keep-awake. "System" recipes may run on a server later, not only the CLI, so copy stays neutral about where they run.

Naming follows [node-vocabulary.md](../scopes/rust/node-vocabulary.md). Enum values are kebab-case. `when` reuses `assert`'s comparison words. PR 3 adds a "Node keys" section (`always`, `when`) there. The `shell-command` → `shell-run` rename shipped in #663.

## Decisions (record in `.claude/decisions/conditional-steps.md`, PR 3)

| Decision              | Choice                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Conditional primitive | Node-level `when: { value, op, expected }`, a sibling of `always` (not in `parameters`, which #666 now refuses when undeclared). Branches stay ordinary `nodes` children, so consent (`consent_lines.rs`), trust, raw shell checks (`raw_params.rs`), deps and migrate keep seeing them.                                                                                                                                                          |
| Comparison            | Extract `Op` and `compare` from `bnto-core/src/control/assert/compare.rs` into a shared `bnto-core/src/compare.rs`, used by `assert` and `when`. `Op::parse` accepts `not-equals` (vocabulary) and `not_equals` (until assert's audit rename + migration).                                                                                                                                                                                        |
| Evaluation            | In `executor/mod.rs` `execute_node`, before `emit_node_started`, with a `TemplateContext` built from `PipelineContext`: scoped fields, ctx, env, `node.*` records, loop item. Per call, so per iteration inside loops.                                                                                                                                                                                                                            |
| Skip                  | Emits `NodeSkipped { node_id, node_index, total_nodes, node_type, parent_node_id, reason }`. Returns the input files unchanged, with no records. A false `when` beats `always`.                                                                                                                                                                                                                                                                   |
| Fields scope          | A container's `fields` are visible to its descendants (nearest declaration wins). `--param key=value` targets the node that declares `key`; if two nodes declare it, the run errors and asks for `nodeId:key`. The TUI form lists each declared field once.                                                                                                                                                                                       |
| OS support            | Worked out, never hand-written. A guard counts as a platform guard only when `value` is exactly `{{ctx.platform}}`, `op` is equals/not-equals, and `expected` is a literal. `runs_on(node, os)` = the guard allows `os`, and the node is a primitive or has a processing descendant that runs on `os`. A recipe's `os` = the OSes on which some root processing node runs. Exposed as `os` (not `platforms`, which already means runtime target). |
| OS-scoped deps        | `requires` allowed on any node (planned for containers only; a step's own `requires` is the tightest scope and `runs_on` treats both alike). Pre-flight, doctor and install skip deps of nodes that don't run on the host (same `runs_on`). No new `platforms` field on `Dependency`.                                                                                                                                                             |
| Unsupported host      | `bnto run` stops before any step runs, and before consent for library recipes: "keep-awake runs on macOS. This machine runs Linux." (Built-in recipes are trusted and have no consent step.)                                                                                                                                                                                                                                                      |
| Doctor until PR 8     | **Closed by PR 8 (#712).** (Review, Oct 21.) Fix B dropped keep-awake's recipe-level `requires`: `bnto doctor` checked every recipe's deps on every OS, so `caffeinate`/`osascript` failed it on Linux and Windows with no way to fix it. PR 8 put them back on the `macos` group; doctor now lists them there as `skip … (macOS only)`.                                                                                                          |

---

## Done

### PR 1: Publish keep-awake as a built-in macOS recipe (#662)

Built-in recipe #22, the first `system` recipe. One script-mode `shell-run` step (script source is the recipe's `session.parameters.script`), args `[{{ctx.paths.home_dir}}, action, duration, display, jiggle]`; script mode clears the environment, so the bnto home comes from the engine, never `$BNTO_HOME`. `display` options are `keep-on`/`allow-sleep`. Web wiring: icon, `system` category content and breadcrumb, keep-awake FAQs, registry/nav tests. 9 macOS-gated CLI tests in `bnto/tests/keep_awake.rs`, each in a temp `BNTO_HOME` + `HOME`.

### PR 2: Lint bundled recipe scripts with ShellCheck in CI (#665)

`bnto-engine/tests/bundled_scripts.rs` runs ShellCheck over every bundled script-mode step; CI's Linux unit-test step sets `BNTO_REQUIRE_SHELLCHECK=1` so a missing ShellCheck fails instead of skipping. SEC-31 in `shell-run.md`.

---

## Review fixes (A-F)

A code review of PRs 1-2 (October 21, 2026; security/Rust and web/docs reviewers, every finding reproduced) found the following. Each fix PR lists the findings it owns.

### A: Make keep-awake's Stop and Start only touch its own session

- **Orphaned nudger.** Stop and Start find a session only through the caffeinate pid. If caffeinate died (e.g. `killall caffeinate`) while the cursor nudger lived, Stop deleted the state file, the nudger's only record, and left it running; with `duration=inf`, forever. Fix: end the session and the nudger independently; status reports an orphaned nudger.
- **Pid reuse.** `alive()` matched only the process name (`caffeinate`, and `osascript|[Pp]ython` for the nudger), so a stale pid reused by another program counted, and Stop would kill it. Reproduced with a `caffeinate` another tool started. Fix: match the full command line this script starts (`/usr/bin/caffeinate [-du] -i -s -t …`, `/usr/bin/osascript -l JavaScript … loop …`), and treat a timed session past its `until` as gone. Sessions from the old personal python recipe are no longer recognized (it shadows the built-in anyway).
- **Labels.** `label_for` printed "1 hours"/"0 hours" for durations that aren't menu options; build labels from hours and minutes, refuse leading-zero durations (`sh` reads them as octal), and test every option's label.
- Remaining time rounded down ("29m" on a fresh 30-minute session): round up.
- "STOPPED (check Accessibility permission)" blamed the wrong thing: losing the permission makes nudges silently do nothing; the loop doesn't exit.
- **Tests:** the replace test now checks the first caffeinate died; the sandbox's Drop kills recorded pids directly instead of relying on the Stop under test; new tests for pid reuse, expired state, an orphaned nudger, `duration=inf`, activity simulation on, and every duration label.
- Known gap: these tests are macOS-gated and CI has no macOS job, so they run only locally.
- Unverified, not fixed: two Starts at the same instant can race (no lock); caffeinate's `-t` may not count time asleep.

### B: Stop `bnto doctor` failing off macOS over keep-awake's deps

`collect_catalog_dependencies` gathers every recipe's `requires` with no OS filter, and doctor exits 1 on any missing binary, so every Linux/Windows machine failed with "MISSING caffeinate, built into macOS". Decision (Ryan, Oct 21): drop keep-awake's recipe-level `requires` now; PR 8 restores them, OS-scoped. See the "Doctor until PR 8" decision.

### C: Make the System category's web copy and filter accurate

- "Every command shown first" (`categoryContent.ts`) is false for built-in recipes: they're trusted and never prompt (`trust.rs`, `bundled_recipes_run_without_asking`). A security claim the product doesn't keep.
- "bnto doctor confirms everything is in place" overclaims: doctor checks binaries are on PATH, not Accessibility permission.
- The category FAQ names one recipe and promises Windows/Linux; "each recipe page lists what it supports" isn't true until PR 7.
- `/explore` excludes `system` from its category filter (`ExploreFilters.tsx`), so `/explore?category=system`, linked from breadcrumb JSON-LD, shows a blank filter.
- Recipe metadata: "Optionally nudges" (it's on by default), "so the screen never locks" (it stops display sleep, not locks), the "No install" tag contradicts the page's "Install the CLI" card.
- Not done: no `system` mascot (falls back to `file`), and three recipe descriptions still say "CLI/desktop only" though there's no desktop app yet.

### D: Stop CLI-only recipe pages describing a browser tool

Affects `keep-awake`, `download-video`, `flatten-folders`. Branch on `isRecipeBrowserCapable`:

- `getHowItWorks` ("Drop your files → Download results") and the shared FAQ ("files never leave your browser", "works on mobile") in `recipePageContent.ts`.
- `CliPromo.tsx` prints `bnto run {slug} <url>`, which fails for no-input recipes; build the hint from the input mode like `bnto info`'s `run_hint`.
- Title/h1 "… Online Free" (`bntoRegistry.ts`), `WhyBntoSection` ("runs entirely in your browser tab"), `CategoryFeaturesSection` ("Drop your files and go"), JSON-LD (`operatingSystem: "Any"`, "Requires a modern web browser"), and `llms-full.txt` ("Accepts: URL input" for keep-awake).
- README "Try it" links CLI-only rows to a page that can't run them.

### E: Limit ShellCheck disables in bundled scripts

- A bundled script can switch off any check with `# shellcheck disable=…`, including `all`; allow only listed codes (today SC2016) and assert it.
- Run ShellCheck with `--norc` so a developer's `~/.shellcheckrc` can't change local results.
- Pinned since (follow-up): CI installs ShellCheck v0.11.0 by checksum instead of using the runner image's copy.

### F: Refresh the new-recipe checklist and stale recipe counts

- `engine-node-patterns.md` "Adding a New Recipe" points at the removed `RECIPE_IDS`/`WEB_*` overlay and the old `engine/recipes/` path, and never mentions `categoryContent.ts`, `CATEGORY_BREADCRUMB_LABELS`, `categoryMascot.ts` or the `ExploreFilters` exclusion (the gaps C fixes).
- Stale counts: `ROADMAP.md` "The 20 predefined recipes", `packages/@bnto/nodes/README.md` "18 recipes" (and its `engine/recipes/` path).
- No `system` mascot (falls back to `file`); `bntos.md` doesn't list keep-awake (or download-video, flatten-folders).

---

## Planned

### PR 3: Skip a step when its `when` guard doesn't hold

**Branch:** `feat/when-guard`
**One sentence:** Let any node carry a `when` comparison that skips it, passing its files through, when false.

**What:**

- `When { value, op, expected }` on `Definition` (`definition.rs`, next to `always` L82) and `PipelineNode` (`pipeline.rs` L68-115).
- The shared `compare.rs` (extracted from assert).
- Evaluation and skip in `execute_node`.
- The `NodeSkipped` event (`events.rs`), counted in `run_summary.rs`.
- `check/`: refuse an unknown op, a missing `value`, and `when` on input/output (like the `always` finding, `check/mod.rs` L89-102). `placeholder_findings` covers `when` strings.
- The JSON schema (`definition_schema.rs`).
- Regenerated TS definition types.
- Docs:
  - decision record
  - `engine-execution.md` "Conditional steps (`when`)" section
  - new `.claude/journeys/cli.md` (CJ1–CJ8 below)

**RED tests (engine + CLI):**

- true runs; false skips with input passed through unchanged
- `not-equals` and `not_equals` both parse
- the guard reads its own fields, `{{ctx.platform}}`, `{{node.x.count}}`, and `{{item.*}}` inside a loop (per iteration)
- a skipped node leaves no `{{node.<id>.*}}` record
- a false `when` beats `always`
- a skipped group never starts its children
- a graph-level skip routes its input to consumers
- `check` refusals
- consent still lists guarded shell steps (SEC-16), and raw checks still refuse a bad command inside a guarded group

**Verification:** `task wasm:lint && task wasm:test && task cli:test && task nodes:codegen && task ui:build && task ui:test`
**Count changes:** none (node types 39, processors 32).

### PR 4: Show skipped steps in the CLI, TUI and previews

**Branch:** `feat/when-surfaces`
**One sentence:** Report skipped steps, and the guards behind them, everywhere a run is shown or previewed.

**What:**

- `bnto/src/progress.rs` prints `skipped (ctx.platform is linux, not macos)`.
- TUI `NodeStatus::Skipped` (`tui/screens/execution.rs`, `execution/update.rs`, `message.rs`, `tui/bridge/events.rs`).
- Consent lines annotate guarded steps ("runs when ctx.platform equals macos").
- `--dry-run` marks steps skipped on this host.
- TS: `packages/core/src/types/pipelineEvents.ts` + editor `executionState.ts` / `logEventToConsole.ts` handle `nodeSkipped`.
- Editor round-trip preserves `when` (follow how `always` survives in `packages/editor/src/actions/loadDefinition.ts`).

**RED tests:**

- CLI progress line
- TUI status
- consent and dry-run annotations
- TS event reducer
- editor load→save keeps `when`

**Verification:** `task cli:test && task ui:build && task ui:test && task ui:lint` (+ `task e2e` if editor UI is touched).
**Count changes:** none.

### PR 5: Let a container's fields reach the steps inside it

**Branch:** `feat/scoped-fields`
**One sentence:** Make a container's declared fields visible to its descendants' templates and guards.

**What:**

- `PipelineContext` carries the inherited field values, extended by each container (`executor/mod.rs` `layer()` L104, `container.rs`). The primitive merges them in `primitive.rs` `node_field_values` L105; own fields win.
- `bnto/src/input.rs` `apply_param_overrides`: `key=value` targets the node that declares `key`, and errors on ambiguity. This replaces `first_processing_node_id` for field keys; raw-param fallback is unchanged.
- TUI `detail_loader.rs` `collect_field_params` is unchanged (each field is declared once).
- SEC-13 (`field_values.rs`) still checks every declaration.

**RED tests:**

- a child reads a parent group's field
- own field shadows parent
- `--param action=stop` reaches a field declared on a group
- two declarations → an error naming both nodes
- a `when` on a group reads its own field

**Verification:** `task wasm:test && task cli:test`
**Count changes:** none.
**As built:** `executor/field_scope.rs` (`declare`, `along`) is the one rule; `PipelineContext.fields` carries it down, and `bnto check` and the dry run (`host_skip`, shell previews) walk the same path. `--param` routing lives in `bnto/src/field_owner.rs`, shared by `bnto run` and `bnto dry-run`. A declaration with no default still shadows. CJ2 is `cj2_*` in `bnto/tests/when_guard.rs`.

### PR 6: Work out the OSes a recipe runs on, and refuse unsupported hosts

**Branch:** `feat/recipe-os-support`
**One sentence:** Compute each recipe's supported operating systems from its platform guards.

**What:**

- A pure `runs_on` / `recipe_os` in `bnto-core` (new `os_support.rs`, reusing `HostPlatform` from `metadata/install_hints.rs`).
- Catalog `RecipeEntry.os` (`bnto-wasm/src/catalog.rs` L36-44) → snapshot → `generate-from-catalog.ts` → `Recipe.os` (`@bnto/registry`).
- `bnto info` "Runs on:" (`info.rs`); `bnto list` marker (`list.rs`); TUI detail header (`render_detail.rs`).
- `bnto run` stops before any step runs (and before consent) on an unsupported host.
- Wrap keep-awake's step in a `when ctx.platform equals macos` group, and drop "macOS only" from its description.
- Fix `info.rs` L92 / `dry_run/shell.rs` L53 skip lists to use `is_container_node` (they missed `parallel`).

**RED tests:**

- `recipe_os`: no guards → all; macOS-only group → `[macos]`; unguarded step + macOS group → all; a guard on a field → all; `not-equals windows` → `[macos, linux]`
- keep-awake → `[macos]`
- `bnto run keep-awake` on Linux/Windows CI exits before any step runs, with the message (runs in the `rust`/`rust-windows` jobs)
- `bnto info` line

**Verification:** `task wasm:codegen && task wasm:test && task cli:test && task ui:build && task ui:test`
**Count changes:** none. Split the TS codegen into its own PR if the diff passes ~400 lines.
**As built:** `bnto_core::os_support` (`runs_on`, `recipe_os`, `describe_os`); `HostPlatform` gained `ALL`, `as_str` (also what `{{ctx.platform}}` now reads) and `label`. `bnto/src/host_support.rs` holds the refusal and the `[macOS]` badge shared by `bnto list` and the TUI header (shown only when a recipe misses an OS; `bnto info` always prints `Runs on:`). The refusal runs first in `run_recipe`, so before vars and consent. `Recipe.os` is optional in `@bnto/registry` (user recipes aren't analysed in TS). The TS side was small (~40 hand-written lines), so no split. TUI journey TJ6 moved off keep-awake to a message-mode fixture: on Linux CI keep-awake's step is now skipped. `isRecipeBrowserCapable` (`@bnto/registry`) skipped containers without looking inside, so keep-awake's step in a group made it browser-capable; it now walks children (keep-awake is the first built-in with a step in a container). Not done: the TUI and `bnto dry-run` don't refuse an unsupported host (a TUI run there skips every step).

### PR 7: Show which operating systems a recipe runs on, on bnto.io

**Branch:** `feat/web-os-badge`
**One sentence:** Render a Mac/Windows/Linux badge on recipe pages and cards from `Recipe.os`.
**RED tests:** the badge renders for `keep-awake` (Mac only); nothing renders when `os` is all three or the recipe is browser-capable. Read `scopes/web/components.md` + `theming.md` first.
**Verification:** `task ui:build && task ui:test && task ui:lint && task e2e && task seo:audit` (regenerate page screenshots if the layout shifts).
**Count changes:** none.
**As built:** `@bnto/registry` `recipeOsLabels` and `getRecipeOsBadge` ("macOS only", `null` for a browser recipe, one that runs everywhere, or one without `os`). `apps/web/components/blocks/RecipeOsBadge.tsx` (outline `Badge`, `data-testid="recipe-os-badge-<slug>"`) sits beside "CLI Only" in `CliPromo` and beside the category on `ExploreRecipeCard` (explore grid, related recipes). It reads `@bnto/registry`, as `CliPromo` already does, because the recipe page is a server component and `@bnto/core` is client-only. CLI recipes' JSON-LD now claims `operatingSystem` ("macOS", "macOS, Linux, Windows") from `BntoEntry.operatingSystems`. No page screenshots exist to regenerate. Not done: `llms-full.txt` doesn't list a recipe's OSes.

### PR 8: Scope dependencies to the steps that use them

**Branch:** `feat/scoped-requires`
**One sentence:** Allow `requires` on container nodes, checked only where the container can run.
**What:** `requires` on `PipelineNode`/`Definition` children. `bnto-engine/src/deps.rs` `collect_from_nodes` (L45-64) / `check_pipeline_dependencies` (L144-175) skip nodes that don't run on the host. Doctor shows the OS. Keep-awake's `caffeinate`/`osascript` come back (dropped in fix B), inside its macOS group.
**RED tests:**

- a macOS group's dependency isn't checked on Linux/Windows
- root `requires` is unchanged
- `bnto doctor` and `bnto install` honor it
  **Verification:** `task wasm:test && task cli:test`
  **Count changes:** none.
  **As built:** `PipelineNode.requires` (any node; `Definition.requires` already parsed on nested nodes, so the TS type only changed its comment; the JSON schema gained `requires` and a `Dependency` def). `bnto_engine::scoped_deps` is the one rule: per OS, root `requires` plus the `requires` and processor deps of every node that `runs_on` it, merged into `ScopedDependency { dependency, os }`. `deps.rs` exposes `collect_pipeline_dependencies` / `collect_catalog_dependencies` for one host (pre-flight, `bnto install`, `bnto dry-run`) and `scoped_*` variants for `bnto info` and `bnto doctor`. Doctor prints `ok caffeinate (macOS only)` on macOS and `skip caffeinate (macOS only)` elsewhere; `bnto install` says "has no external dependencies on Linux". keep-awake's deps came back on its `macos` group with `installHints.macos`; fix B's test now asks for a hint only on the OSes that need a dependency. The frozen editor's export dropped a top-level node's `requires` (it only kept the root's), so `rfNodesToDefinition` now carries it like `when`. CLI tests: `bnto/tests/scoped_requires.rs` (doctor, install, info, pre-flight; the doctor test runs in the `rust` and `rust-windows` jobs). Not done: `bnto check` doesn't refuse `requires` on `input`/`output`.

### PR 9: PowerShell script mode

**Branch:** `feat/powershell-script-mode`
**One sentence:** Run a literal PowerShell script with values passed only as `BNTO_ARG_n` environment variables.
Its own design doc first. `powershell.exe -NoProfile -NonInteractive -EncodedCommand`, by absolute path, cleared env + `BNTO_ARG_1..N`. Security specs mirror SEC-20..26 in Windows CI.
**As built:** design in [powershell-script-mode.md](../decisions/powershell-script-mode.md). `"shell": "powershell"` (Windows PowerShell 5.1, Windows only, under `%SystemRoot%`) and `"pwsh"` (PowerShell 7, fixed paths on all three OSes) in `bnto-shell/src/powershell.rs`: `-NoLogo -NoProfile -NonInteractive -OutputFormat Text -EncodedCommand <base64 UTF-16LE of a fixed prelude + the script>` and nothing after it (the first CI runs caught errors arriving as `#< CLIXML`; pwsh 7 stops with an explicit `-OutputFormat`, and 5.1 never does, so `clixml.rs` decodes a PowerShell step's stderr back to text); values only as `$env:BNTO_ARG_1..N` + `BNTO_ARG_COUNT` (`run.rs` puts them in the child's env instead of argv; file mode fills `{{output_dir}}` in them). The prelude silences progress records, makes stdout UTF-8 and turns off ANSI colors. A step's `env` may not name `BNTO_ARG_*`; a script whose command line would pass 30,000 characters is refused on every OS. `__PSLockdownPolicy` joined the cleared-env allowlist; `PSModulePath` stays out. Consent shows `pwsh <script> $env:BNTO_ARG_n: <args>`; `bnto dry-run` shows `BNTO_ARG_1=…` per value. SEC-32..37 in `bnto/tests/shell_security/powershell_mode.rs` run per installed PowerShell; CI sets `BNTO_REQUIRE_PWSH=1` on Linux (`pwsh`) and Windows (both), so they can't skip there. Not done: PSScriptAnalyzer for bundled PowerShell scripts (lands with the first one, PR 11).

### PR 10 / 11: Keep awake on Linux / Windows

**Branches:** `feat/keep-awake-linux` (after PRs 5, 6, 8), `feat/keep-awake-windows` (after 5, 6, 8, 9)
**One sentence each:** Add the `systemd-inhibit` (Linux) / PowerShell (Windows) keep-awake session as a platform-guarded group.
**What:** Move the fields up to a `session` group shared by all OS groups (PR 5).
**RED tests:** OS-gated CLI round-trips (status/stop with no session; start→status→stop; temp `BNTO_HOME`). Linux: activity simulation without X11/`xdotool` starts anyway with a notice; skip when `systemd-inhibit` is absent.

**Linux design (PR 10, researched Oct 22, 2026):**

- **Sleep:** `systemd-inhibit --who=keep-awake --what=sleep[:idle] --mode=block <sleep> <secs>`, detached like `caffeinate`. It holds the lock while its `sleep` child runs. `idle` is added when the display should stay on.
- **Display:** logind's `idle` inhibitor only blocks suspend-on-idle. GNOME ignores it for blanking and locking (a GNOME developer, [Red Hat bug 1077579](https://bugzilla.redhat.com/show_bug.cgi?id=1077579)); KDE Plasma 6 honors it. So with `display=keep-on`, when `gnome-session-inhibit` is installed and the session bus is reachable, a second inhibitor (`--inhibit idle:suspend`) holds the display; otherwise the start message says where the display stays on.
- **Who may inhibit:** polkit's defaults ([`org.freedesktop.login1.policy`](https://github.com/systemd/systemd/blob/main/src/login/org.freedesktop.login1.policy)) allow `inhibit-block-sleep` only from a session on a _local_ console (`allow_active`/`allow_inactive`); an SSH login or a service falls under `allow_any`, which is `auth_admin_keep`, and a non-interactive caller can't satisfy it. So a start there fails with "systemd refused to keep this computer awake: <reason>" rather than claiming a session it doesn't have. Upstream allows `inhibit-block-idle` for anyone, but Ubuntu's CI runner refused it while granting sleep, so a refused `sleep:idle` retries as `sleep` alone and the start says the display may still sleep. CI grants the runner user `inhibit-block-sleep` through a polkit rule and sets `BNTO_REQUIRE_KEEP_AWAKE=1`, so the round trip holds a real lock there.
- **Activity simulation:** an X11 session (no `WAYLAND_DISPLAY`), `xdotool` and `xprintidle`. A `/bin/sh` loop nudges the cursor 1px with XTEST once `xprintidle` reaches the threshold, like the macOS JXA loop. On Wayland one app can't move the cursor for another, so the session starts without it and says so.
- **Desktop variables:** script mode clears the environment, so the step's `env` passes `DISPLAY`, `XAUTHORITY`, `WAYLAND_DISPLAY`, `DBUS_SESSION_BUS_ADDRESS` and `XDG_RUNTIME_DIR` from `{{env.*}}` by name (consent lists them). Empty means unset.
- **Programs by absolute path** (`/usr/bin`, then `/bin`), never PATH, as on macOS. `ours()` matches each pid's command line (`systemd-inhibit --who=keep-awake`, `gnome-session-inhibit --app-id keep-awake`, `/bin/sh -c … keep-awake-nudger`) so a reused pid isn't killed.
- **Considered:** `xset s off -dpms` (X11-only and global, so it has to be undone) and the `org.freedesktop.portal.Inhibit` portal (the inhibit lasts only while the caller stays on the bus, which `gdbus` doesn't). Rejected.

**Windows design (PR 11):**

- **Session:** `SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED [| ES_DISPLAY_REQUIRED])` holds only while its thread lives, so the step starts a hidden, detached `powershell.exe` (`Start-Process -WindowStyle Hidden`, which hands it no handles, so the step finishes at once) that holds the request until the session ends. bnto releases a finished step's Job Object rather than killing it (`job_object.rs`), so the session outlives the step, as `nohup` does on unix; cancel, timeout or a crashed bnto still kill it.
- **Activity simulation:** the same process polls `GetLastInputInfo` every 15s and moves the cursor 1px and back with `mouse_event` once you've been idle the threshold. No permission is needed.
- **No values in code:** the session's code is a literal `{ … }` scriptblock in the step script, so PSScriptAnalyzer lints it, and its `.ToString()` goes to `-EncodedCommand`. Its values reach it as `KEEP_AWAKE_*` environment variables set before `Start-Process`.
- **Ours:** the state file records the session's pid and process start time; a pid counts only while the process there started at that time and before the session's end. The first version matched `Win32_Process.CommandLine` against the encoded code, but WMI queries pushed steps past their 60s limit on CI's Windows runner; `Get-Process` is fast, and no other process can share both a pid and a start time.
- **PowerShell 5.1 details:** flag values are written in decimal, since `0x80000001` parses as a negative Int32; idle time is worked out modulo 2^32, since `[Environment]::TickCount` wraps; the home folder is `$homeDir`, since `$home` is read-only.
- **Lint gate:** PSScriptAnalyzer over bundled PowerShell steps (SEC-38), required in Windows CI.
- **First-run time:** the Windows step's limit is 180s, not 60s. On CI's Windows runner the first runs of the script took over 60s and later ones under a second (each serialized test's timestamps), which points at a first-sight scan of new PowerShell content, most likely Defender's cloud check, rather than a hang. A user's very first start could hit the same wait.

## Follow-ups

Gaps the PRs above left open on purpose, collected from their "As built" and "Not done" notes. None blocks keep-awake.

- **macOS tests in CI:** `bnto/tests/keep_awake.rs` is macOS-gated and CI has no macOS job, so the macOS round trips run only locally (fix A).
- **Unsupported host in the TUI and `bnto dry-run`:** only `bnto run` refuses; a TUI run there skips every step (PR 6). `bnto check` doesn't flag a recipe that runs nowhere.
- **Web:** `llms-full.txt` doesn't list a recipe's OSes (PR 7); there's no `system` mascot, and three descriptions still say "CLI/desktop only" with no desktop app yet (fix C).
- **Tooling:** `-CommandWithArgs` for pwsh 7.4+ isn't used (PR 9): it was an escape hatch in case `-EncodedCommand` caused trouble, and it hasn't.

## CLI journey matrix (`.claude/journeys/cli.md`, added in PR 3)

| ID  | Journey                                                                                         |
| --- | ----------------------------------------------------------------------------------------------- |
| CJ1 | Guard true runs; guard false is reported skipped, and its files pass through                    |
| CJ2 | If/else pair on a field, flipped with `--param`                                                 |
| CJ3 | Per-OS groups: the host branch runs, the others are skipped (expected branch from `cfg!`)       |
| CJ4 | No branch for the host → `bnto run` stops before any step runs, naming the supported OSes       |
| CJ5 | Consent lists guarded shell steps with their guard                                              |
| CJ6 | `bnto check` refuses an unknown op and `when` on input/output                                   |
| CJ7 | `--dry-run` shows the steps skipped on this host                                                |
| CJ8 | A guard on `{{item.*}}` inside a loop processes only matching files and passes the rest through |

## Dependency Chain

```
PR 1 (keep-awake macOS)                 -- merged #662
|-- PR 2 (ShellCheck gate)              -- merged #665
|-- Fixes A-F (review of PRs 1-2)       -- land before PR 3
PR 3 (when guard, engine)               -- independent
|-- PR 4 (skipped-step surfaces)        -- depends on PR 3
|-- PR 6 (recipe OS support)            -- depends on PR 1, 3
|   |-- PR 7 (web OS badge)             -- depends on PR 6
|   |-- PR 8 (scoped requires)          -- depends on PR 6
PR 5 (scoped fields)                    -- independent
PR 9 (PowerShell script mode)           -- independent
PR 10 (Linux)   -- depends on PR 5, 6, 8
PR 11 (Windows) -- depends on PR 5, 6, 8, 9
```

## Verification (end to end, after PR 6)

- `bnto info keep-awake` → "Runs on: macOS".
- On macOS: start→status→stop round-trips (temp `BNTO_HOME`).
- On Linux/Windows CI: `bnto run keep-awake` refuses before any step runs.
- `bnto run` of a CJ3 fixture on all three CI OSes prints the host branch and two skipped lines.
