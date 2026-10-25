# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-76**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

| ID     | Issue                                                                                                 | Area        | Status                                           |
| ------ | ----------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------ |
| ISS-7  | The CLI's telemetry notice links `bnto.io/telemetry`, a 404                                           | web docs    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-8  | Most CLI integration tests run `bnto` against the real `~/.bnto`                                      | cli test    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-10 | `task check` skips the crate publish checks CI runs                                                   | tooling     | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-12 | Released CLIs send no telemetry: the PostHog key is never set                                         | cli         | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-16 | `@bnto/nodes` lint skips `scripts/`; codegen script is 1,381 lines                                    | tooling     | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-17 | The `/workflow-expert` skill's node inventory is stale                                                | docs        | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-18 | A Windows process test times out waiting 20 s for PowerShell                                          | engine test | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-22 | The output node's `directory` leaves `{{env.*}}` as a literal folder name                             | cli         | Open                                             |
| ISS-29 | Errors on bnto.io go unseen: exception capture is off                                                 | core        | Open                                             |
| ISS-31 | Another app's events land in bnto.io's PostHog project                                                | analytics   | Open                                             |
| ISS-33 | Output names and extension reads split on a dot in a folder name                                      | engine      | Open                                             |
| ISS-34 | `file-rename`'s `prefix`, `case` and `sanitize` rename the folders too                                | engine      | Open                                             |
| ISS-35 | Inside a loop, a data value is filled in as a template                                                | engine      | Open                                             |
| ISS-37 | The homepage loads ~500 KB of JS that competes with its CSS                                           | web perf    | Open                                             |
| ISS-38 | CI and release skip jobs for root files those jobs read                                               | ci          | Open                                             |
| ISS-39 | The pre-commit hook in a worktree replays stale Turbo lint results                                    | tooling     | Open                                             |
| ISS-43 | The TUI labels overwrite results with the first file's folder only                                    | tui         | Open                                             |
| ISS-44 | In the browser, a renamed or removed param's error says to run `bnto migrate` (needs Ryan's decision) | web         | Open                                             |
| ISS-45 | Progressive loop writes in message/none mode land in the folder unlisted                              | cli tui     | Open                                             |
| ISS-46 | The TUI's Open action uses the first folder for overwrite results                                     | tui         | Open                                             |
| ISS-47 | Recipe pages preload a far-below-the-fold mascot and don't prioritize their LCP image                 | web perf    | Open                                             |
| ISS-49 | A half-written `.next-e2e` from an interrupted E2E run breaks the web build                           | tooling     | Open                                             |
| ISS-51 | The Turbo-inputs check guards lint only, not test or build reads                                      | tooling     | Open                                             |
| ISS-53 | An interrupted move can leave `.bnto-rename-*` files that later runs collect                          | engine      | Open                                             |
| ISS-54 | The `scripts/seo` tests never run                                                                     | tooling     | Scheduled ([plan](strategy/changelog.md))        |
| ISS-55 | A printed command line with a word that starts with `=` fails in zsh                                  | cli         | Open                                             |
| ISS-56 | A dry run's command whose program path needs quotes won't paste into PowerShell                       | cli         | Open                                             |
| ISS-57 | `errorCategory` reports `out_of_memory` for any message containing `oom`                              | core        | Open                                             |
| ISS-58 | Inside a loop, a batch node's `{{item.*}}` can't read a key added in the iteration                    | engine      | Open                                             |
| ISS-59 | **Inside a loop, `file-copy` can still replace an input another iteration reads**                     | engine      | Open                                             |
| ISS-60 | A batch node can't see the run's time limit and keeps starting work past it                           | engine      | Open                                             |
| ISS-61 | Two docs name hook and function files in kebab-case; the code uses camelCase                          | docs        | Open                                             |
| ISS-62 | Three docs let a component keep 80-100 lines of logic inline; ESLint caps a `.tsx` function at 60     | docs        | Open                                             |
| ISS-64 | A dry run's `Run with: bnto run …` footer drops the dry run's `-p` and `--vars`                       | cli         | Open                                             |
| ISS-65 | Nothing lints the root `scripts/` folder                                                              | tooling     | Open                                             |
| ISS-66 | Down past the last home recipe hides the selection until you press Up as many times                   | tui         | Open                                             |
| ISS-67 | `update_notice.rs` opens its ptys in parallel with no lock                                            | cli test    | Open                                             |
| ISS-68 | `FormStoreContext.tsx` exports four hooks and a provider from one `.tsx` file                         | form        | Open                                             |
| ISS-69 | E2E specs are never type-checked or linted                                                            | web test    | Open                                             |
| ISS-70 | Recipe pages record no download a user clicks for                                                     | web         | Open                                             |
| ISS-71 | Two `task seo:audit` runs at once collide on port 3000                                                | tooling     | Open                                             |
| ISS-73 | The web runner starts a run with a required param left empty                                          | web         | Open                                             |
| ISS-74 | A required param with no value is refused only after consent and after `-o` is made                   | cli         | Open                                             |
| ISS-75 | The CLI can't give Watermark Images an overlay: `-p` takes base64 text, not a file                    | cli         | Open                                             |

---

### ISS-7: The CLI's telemetry notice links `bnto.io/telemetry`, a 404

**Found:** October 23, 2026 (fixing ISS-2).

The first-run notice (`engine/crates/bnto/src/telemetry/consent.rs:27`) says bnto "collects anonymous usage data" and sends people to `https://bnto.io/telemetry` for more. That URL renders the `[bnto]` route's "Page Not Found" (`curl -sL https://www.bnto.io/telemetry | grep -o not-found-heading`), and no page says what the CLI collects: `privacy/privacy.mdx` lists PostHog without mentioning the CLI, and the CLI recipe FAQ (`apps/web/lib/cliRecipeFaq.ts:9`) has one line. Add a `/telemetry` page listing the four events (`cli_command`, `cli_recipe_run`, `cli_error`, `cli_tui_session`), their properties (bundled slugs only, else `"custom"`; bucketed counts, sizes and durations), the anonymous id and the three opt-outs, or point the notice at a page that has them.

### ISS-8: Most CLI integration tests run `bnto` against the real `~/.bnto`

**Found:** October 23, 2026 (fixing ISS-3).

`cli_commands.rs` and the run helpers in `tests/helpers/mod.rs` now use `helpers::Sandbox` (temp HOME, BNTO_HOME, config dir and working directory, telemetry off). About 17 other files in `engine/crates/bnto/tests/` still spawn `Command::new(bnto_bin())` with no HOME or BNTO_HOME: `assert_files`, `check_edges`, `check_folders`, `check_run_time`, `edges`, `edit_fields`, `file_delete`, `file_inspect`, `file_join`, `progressive_output`, `report_file_names`, `run_args`, `run_json`, `run_temp_files`, `shell_each`, `shell_options`, and some runs in `dry_run_temp_files` and `run_timeout`. Each run writes a session log into the developer's `~/.bnto/state/logs`: `cargo test -p bnto --test edges --test run_json` added 3. A run that resolves a slug would also read their library. Switch them to `Sandbox::new().command()`.

### ISS-10: `task check` skips the crate publish checks CI runs

**Found:** October 23, 2026 (ISS-5, PR #740).

CI's Crate Publish List job runs `scripts/check-crate-publish-list.sh` and `scripts/check-own-version-crates.sh` (`.github/workflows/ci.yml:316-323`), but no task does, so `task check` can't catch them. #740 added `#[expect]` lines to tonkotsu without bumping its own version: `task check` exited 0 and CI failed (`tonkotsu-0.1.2/src/field.rs ... differ`). Add both scripts to `task check` (the own-version one needs network, to download the published crate), or say in pre-commit.md that a change to a crate with its own version needs that bump.

### ISS-12: Released CLIs send no telemetry: the PostHog key is never set

**Found:** October 24, 2026 (planning ISS-7).

`engine/crates/bnto/src/telemetry/mod.rs:57` reads the key from the `BNTO_POSTHOG_KEY` env var at run time, and telemetry is inactive without one (`:60`). Nothing sets it: there's no `option_env!`, no build script, and nothing in `release.yml` (`grep -rn BNTO_POSTHOG_KEY` finds only `mod.rs` and a test). So shipped CLIs collect nothing, while the first-run notice says bnto "collects anonymous usage data" and the code comment calls the key "safe to embed". `bnto telemetry status` also reports "Effective: true" without checking for a key (`mod.rs:134`). Needs Ryan's decision: ship the key, or say telemetry is off.

### ISS-16: `@bnto/nodes` lint skips `scripts/`; codegen script is 1,381 lines

**Found:** October 21, 2026 (dogfood sessions); checked October 24, 2026.

`packages/@bnto/nodes/eslint.config.mjs:6` ignores `scripts/**`, so `scripts/generate-from-catalog.ts` reached 1,381 lines against the 250-line cap, and its functions against the 20-line one, with nothing flagging it. Codegen stays in TypeScript (decided October 24, 2026).

### ISS-17: The `/workflow-expert` skill's node inventory is stale

**Found:** October 21, 2026 (dogfood sessions); checked October 24, 2026.

`.claude/skills/workflow-expert/SKILL.md:30` still says recipes use "the 6 implemented browser operations", and its operations table predates most of today's nodes. The skill reviews every new node and param, so a stale inventory steers it wrong.

### ISS-18: A Windows process test times out waiting 20 s for PowerShell

**Found:** October 24, 2026 (v0.18.2 release run 40096930376).

`terminate_all_kills_orphaned_grandchild_on_windows` (`engine/crates/bnto/src/context.rs:743`) starts `cmd /c start /b powershell … $PID; Start-Sleep 30` and waits once, `recv_timeout(20 s)`, for PowerShell to print its PID (`:758-759`). On the release's Windows runner PowerShell didn't print in time: `called Result::unwrap() on an Err value: Timeout`. The same commit passed the same job on #743's CI an hour earlier, so the failure is timing, not code. It failed the release's CI Gate. Wait on PowerShell's start with a longer bound, or start a lighter process for the grandchild, so a slow runner can't fail a release.

### ISS-22: The output node's `directory` leaves `{{env.*}}` as a literal folder name

**Found:** October 24, 2026 (evaluating [a feature request](requests/2026-10-24-folder-index.md)).

A recipe with `secrets: [{"key": "SOME_KEY"}]` and an output node `directory: "{{env.SOME_KEY}}"`, run with `SOME_KEY` set:

- `bnto check` passes.
- `bnto run` exits 0 and prints `1 file written to {{env.SOME_KEY}}/`.
- The file lands in `./{{env.SOME_KEY}}/note.txt`; the folder `SOME_KEY` names stays empty.

`resolve_effective_output_dir` (`engine/crates/bnto/src/run.rs`) and the TUI's copy (`tui/bridge.rs:247-249`) fill only `{{ctx.*}}` and `{{node.*}}`, and an unknown `{{ctx.*}}` key becomes `""`. So `out-{{ctx.nope}}-{{item.x}}` writes to `out--{{item.x}}/`. With `SOME_KEY` unset, the run is refused, but the folder is created first (`progressive_dir`, `run.rs`). Fill every namespace a run has (`env` from declared secrets, as `file-copy`'s `destination` does), refuse any placeholder left over before consent and before any folder is made (as `{{fields.*}}` are refused since #741 and #742), and have `bnto check` flag the ones no run can fill (`{{item.*}}`, a processing step's `{{node.*}}`). ISS-21, the TUI wipe this could have pointed at a real folder, is fixed (#750).

### ISS-29: Errors on bnto.io go unseen: exception capture is off

**Found:** October 24, 2026 (PostHog review).

- **What PostHog shows:** no `$exception` events in 180 days; every event reports `$exception_capture_enabled_server_side = false`. Only the error boundary's `app_error` reaches PostHog (1 event in 90 days).
- **Why:** `packages/core/src/adapters/posthog/posthogAdapter.ts:44-63` doesn't enable exception autocapture, and the project doesn't either.
- **Effect:** unhandled rejections are invisible. For example, `runAction` throws at `recipeStepperActions.ts:127` with no `recipe_run_failed`. Runs are also undercounted: 58 starts against 42 completions and 13 failures leaves 3 with no outcome event, and a run that completes with no results emits nothing (`:133`).
- **Why the tests missed it:** `posthogAdapter.ts` has no test file (only `flagAdapter.test.ts` sits beside it), so nothing asserts its init options.
- **Fix:** enable exception capture, and give every run exactly one outcome event.
- **Regression test:** a unit test on the PostHog adapter's init options that exception capture is on, and a check that every run emits exactly one of `recipe_run_completed` / `recipe_run_failed`.

### ISS-31: Another app's events land in bnto.io's PostHog project

**Found:** October 24, 2026 (PostHog review).

- **What PostHog shows:** 190 events (`$pageview`, `$web_vitals` and a custom event) from three hosts outside bnto.io, June 14 to October 7, 2026, arriving through a different `/ingest` proxy. The project's test-account filters exclude only localhost, an internal cohort and one IP, so these count in web analytics.
- **Fix:** this is a PostHog config fix, not a repo change. Either give that app its own project key, or add a `$host` filter (`^(www\.)?bnto\.io$`) to the test-account filters.
- **No repo test:** it's project config. After the fix, rerun the review's `$host` breakdown to confirm only bnto.io remains.

### ISS-33: Output names and extension reads split on a dot in a folder name

**Found:** October 24, 2026 (fixing ISS-23, which fixes `file-rename` alone). The fix for ISS-23 adds a shared split, `bnto-file/src/split_extension.rs`, that only looks for the dot in the last `/` segment and treats a leading dot as no extension.

Many other places take the last `.` in the whole relative name, so a file with no extension in a dotted folder (`v1.2/data`), or a dotfile (`dir/.env`), gets a wrong stem and extension. Reproduced in a sandbox with `main` at #745:

- `spreadsheet-clean` names `v1.2/data`'s output `v1-cleaned.2/data`: the file moves to another folder.
- `file-inspect` reports `extension` `2/readme` for `v1.2/README`, `2/data` for `v1.2/data` and `env` for `dir/.env` (`bnto-file/src/inspect.rs:183-190`).
- `file-filter` with `extensions: env` keeps `dir/.env` (`bnto-file/src/filter/mod.rs:145`).

The same split, found by reading (not each reproduced):

- **Output names that can move a file:** `bnto-image` `compress.rs:119`, `convert.rs:71`, `strip_exif.rs:47`, `resize/processor.rs:36`, `pad/processor.rs:41`, `overlay/processor.rs:39`; `bnto-vector/src/processor.rs:192`; `bnto-spreadsheet` `clean.rs:337`, `convert.rs:298`, `rename.rs:265`, `read/processor.rs:109`; `bnto-shell/src/stdout_mode.rs:86`.
- **Format guesses that fall back to a default:** `bnto-file` `collect/mod.rs:141`, `render/mod.rs:166`; `bnto-shell/src/output_files.rs:96`; `bnto-spreadsheet/src/read/params.rs:64`.

**Fix:** move the split to `bnto-core`, so every crate shares one, and call it everywhere above. The CLI's `bnto/src/output_names.rs:64-71` already splits correctly.

- **Regression test:** one table-driven test of the shared split (`v1.2/README`, `dir/.env`, `a/b.tar.gz`), plus a test per node family that a suffixed output of `v1.2/data` stays in `v1.2/`. A grep check in `task check` that no crate but `bnto-core` calls `rfind('.')` on a name would stop new copies.

### ISS-34: `file-rename`'s `prefix`, `case` and `sanitize` rename the folders too

**Found:** October 24, 2026 (fixing ISS-23).

The params say they change "the filename", but they act on the whole relative stem, folders included. With `file-collect`'s `flatten: false`, `prefix: x-` plus `case: upper` turns `sub/a.jpg` into `x-SUB/A.jpg` (reproduced in a sandbox with `main` at #745): the file moves into a new folder. `find`/`replace` sees the whole path on purpose (its tests restructure folders); the others shouldn't.

- **Fix:** apply `prefix`, `suffix`, `case`, `sanitize` and `maxLength` to the last `/` segment only; keep `find`/`replace` and `pattern` on the full path. Say so in each param's description. Check it with `/workflow-expert` against `node-vocabulary.md` first, since it changes a shipped node's behavior.
- **Regression test:** `prefix`, `case` and `sanitize` on `sub/a.jpg` leave `sub/`; `find`/`replace` still restructures folders.

### ISS-35: Inside a loop, a data value is filled in as a template

**Found:** October 24, 2026 (cold review of the ISS-24 fix; reproduced by the reviewer with a temp `HOME` and `BNTO_HOME`).

Inside a loop the executor fills `{{item.*}}` from the loop's item into a node's params before the node runs, so the item's _values_ become part of the template the node then fills. A CSV cell `note={{file}}` with `"message": "note={{item.note}}"`:

- outside a loop, `report` prints `note={{file}}`;
- inside a loop, it prints `note=people_row0.csv`.

The same happens through `{{fields.*}}`: a field passed as `-p 'x={{file}}'` into `"label": "{{fields.x}}"` gives the file's name. `report` and `assert` do it today (`placeholder::resolve_file`); the ISS-24 fix makes `edit-fields` do it too. Only `shell-run` declares `reserved_tokens` (`bnto-shell/src/run.rs:104`), which SEC-27's `check_reserved` uses to refuse exactly this.

- **Secrets are not reached:** with `secrets: [{"key": "SOME_KEY"}]` set, a cell reading `{{env.SOME_KEY}}` inside the same loop prints as written (`note={{env.SOME_KEY}}`), checked with `main` at #745. Only `{{file}}` and `{{item.*}}` were seen re-filled; recheck any namespace the fix touches.
- **Fix:** a value filled in from data is never re-scanned. Fill every namespace in one pass over the original template, or escape filled values, rather than adding `reserved_tokens` node by node.
- **Regression test:** inside a loop, a cell holding `{{file}}`, `{{item.x}}` or `{{env.KEY}}` comes out as written, in `report`, `assert` and `edit-fields`.

### ISS-37: The homepage loads ~500 KB of JS that competes with its CSS

**Found:** October 24, 2026 (PostHog `$web_vitals`: the homepage's p75 LCP is about 5 s, from 39 desktop samples over 90 days; then a DevTools trace). Follow-up to ISS-30, whose image fix shipped in #753: below-the-fold mascots load lazily and the hero at high priority.

On a trace of `https://www.bnto.io/` (mobile viewport, 4x CPU, Slow 4G) the HTML arrives at 675 ms but nothing paints until the render-blocking stylesheet arrives at about 3.2 s, because it shares the HTTP/2 connection with everything requested at the same time. The biggest share is JavaScript: three chunks of 70, 137 and 165 KB (compressed; 219, 432 and 579 KB decoded), about 500 KB in all, plus about 100 KB of preloaded fonts. The homepage is mostly static, server-rendered content.

- **Profile first:** what each chunk holds and why the homepage loads it (`next build` with the bundle analyzer, the trace's network waterfall). Likely suspects: the provider stack in `apps/web/app/providers/` (PostHog, Convex, React Query) loaded on a page that needs neither, client-only sections, and `@bnto/ui` barrel imports in client components (`performance.md` forbids them).
- **Fix:** whatever the profile shows. Measure each change on a Vercel preview with the same throttling, since `next start` serves HTTP/1.1 and hides the contention.
- **Not this:** `experimental.inlineCss`, which drops all CSS under Turbopack on Next 16.1.6: a built page loaded with zero stylesheets.
- **Lighthouse misleads here:** its simulated mobile run blames a 4.3 s "render delay" on the hero's text, which is server-rendered and unanimated; the real delay is the stylesheet.
- **Regression test:** a JS budget for the homepage, enforced the way `task check:budgets` (`recipes/size-budget.bnto.json`) enforces other sizes, so the bundle can't grow back unnoticed.

### ISS-38: CI and release skip jobs for root files those jobs read

**Found:** October 24, 2026 (fixing ISS-13, which adds `turbo.json` and `pnpm-workspace.yaml` to the TypeScript job's triggers).

A PR that changes only one of these files skips the jobs that read it, and the CI Gate accepts the skip:

- **Rust jobs:** `ci.yml`'s `engine` filter is `engine/**`, `recipes/**` and the workflow. The engine's tests read `test-fixtures/` at the repo root (e.g. `engine/crates/bnto-spreadsheet/tests/common/mod.rs:19`, `include_bytes!("../../../../../test-fixtures/csv/simple.csv")`), so a fixture-only change runs no Rust job.
- **TypeScript job:** the `web` filter misses `eslint.config.base.mjs`, which every package's `eslint.config.mjs` imports (`packages/core/eslint.config.mjs:1`), and the root files ISS-9's fix adds as task inputs (`README.md`, `.prettierrc`, `engine/Cargo.toml`, `engine/catalog.snapshot.json`, `engine/crates/bnto-core/src/lib.rs`). A lint-rule-only change runs no lint.
- **Release:** `release.yml`'s change detection is narrower than CI's: `engine` is `^engine/` (no `recipes/`, no `test-fixtures/`) and `web` is `^(apps/|packages/|engine/|turbo\.json$|pnpm-workspace\.yaml$)` since #747 (no `pnpm-lock.yaml`, `package.json`, `tsconfig*.json`). A dependency-only release skips the TypeScript job and the web deploy.

- **Fix:** derive the filters from what the jobs read, and keep `ci.yml` and `release.yml` from drifting: one list both use (a shared script, or the release job calling the same filter).
- **Regression test:** a check (in `task check`) that every root file a task or test reads is in the matching filter, the same way ISS-9's fix checks Turbo's inputs.

### ISS-39: The pre-commit hook in a worktree replays stale Turbo lint results

**Found:** October 24, 2026 (fixing ISS-9, reproduced by the fixer with a probe).

In a git worktree, with ISS-9's fix (#756) in place, a staged rule in `eslint.config.base.mjs` that fails every file passed the pre-commit hook: the hook's lint printed `@bnto/auth:lint: cache hit` with `main`'s old hash and the commit went through. Run outside the hook, the same task hashes differently and fails. Likely cause: git sets `GIT_DIR` during hooks, and Turbo then hashes a different tree (a comment in `formatVersion.test.ts` describes the same effect in worktrees). Not yet confirmed.

- **Fix:** run the hook's Turbo tasks with `GIT_DIR` unset (or `--force` for lint), and confirm the hash matches a plain run.
- **Regression test:** in a worktree, a staged lint-breaking change to the base config fails `git commit`.

### ISS-43: The TUI labels overwrite results with the first file's folder only

**Found:** October 24, 2026 (fixing ISS-21).

In overwrite mode the results screen names `source_dir`, the first selected file's folder, as "the output folder" (`engine/crates/bnto/src/tui/bridge.rs:229`). With files from several folders the label is wrong (the list itself is correct since the ISS-21 fix).

- **Fix:** in overwrite mode, say the files were renamed in place, or name each folder.
- **Regression test:** a screen snapshot of overwrite results with files from two folders.

### ISS-44: In the browser, a renamed or removed param's error says to run `bnto migrate`

**Found:** October 24, 2026 (cold review of the ISS-15 fix, which removes `vector-optimize`'s `precision`; confirmed by running the committed WASM in Node).

The engine refuses an undeclared param before any step runs, for every consumer, and its hint for a renamed or removed one says to run `bnto migrate` (`bnto-core/src/renamed.rs`, applied in `executor/raw_params.rs:35`). In the browser that's a CLI command the user may not have. The editor keeps unknown params as they are (`definitionToGraph.ts:68`, `rfNodesToDefinition.ts:53`) and its config panel no longer shows them, so someone who opens an old exported recipe, or has an old draft in sessionStorage, can't clear it from the UI. The earlier renames (`PARAMS`, `NODE_TYPES`) behave the same way.

- **Decision for Ryan:**
  - **(a) Migrate in the browser:** run the same migrations when a recipe is opened or imported (they're pure JSON rewrites), so old recipes upgrade silently.
  - **(b) Ignore removed params:** pre-flight skips `REMOVED_PARAMS` (they did nothing by definition), while `bnto check` still warns and points at `bnto migrate`. This departs from "undeclared params fail loudly" for removals only.
- **Regression test:** an old recipe with `precision` opened in the web editor runs (a) or is accepted with a warning (b), with no CLI step.

### ISS-45: Progressive loop writes in message/none mode land in the folder unlisted

**Found:** October 24, 2026 (second cold review of the ISS-21 fix; reproduced with the CLI, temp `HOME` and `BNTO_HOME`).

A loop with `outputPersistence: "progressive"` copies each iteration's files into the output node's `directory` even when the output is in `message` or `none` mode, which promise to write nothing: `file-list-loop-progressive.bnto.json` with `mode: "message"` and a `directory` left `a.txt` and `b.txt` there. `bnto run` (`progressive_dir`, `run.rs:214`) and the TUI both do it. Outside preview and overwrite mode, the TUI bridge's reporter copies each finished iteration's files on the pipeline's thread, into the folder that `resolve_output_dir` (`tui/bridge.rs:213`) resolves and creates: see `progressive_dir` and `channel_reporter`, `tui/bridge/reporter.rs:18-44`. Since the ISS-21 fix (#750), the TUI's results list doesn't show those files. In write mode, an iteration file that a later step renames also stays on disk under its old name.

- **Fix:** decide what progressive persistence means per mode: either only in write mode, or always, with those files listed. Then make the CLI and the TUI agree.
- **Regression test:** a progressive loop in message mode with a `directory` leaves the folder untouched (or lists what it wrote), in `bnto run` and the TUI bridge.

### ISS-46: The TUI's Open action uses the first folder for overwrite results

**Found:** October 24, 2026 (second cold review of the ISS-21 fix).

Since the ISS-21 fix, overwrite results from a picked folder or from several folders are listed, but `OutputFile` drops the path `FileResultMeta` carries. Open still joins `results.output_dir` with the name (`engine/crates/bnto/src/tui/app_helpers/results.rs:11`), and in overwrite mode that's the first selected file's folder (`source_dir`, `tui/bridge.rs:229`). For a picked `photos/` folder it opens `parent/renamed-a.txt`, which doesn't exist. With two folders, both rows read `renamed-a.txt` and both open the first folder's file. Related: ISS-43 (the folder label).

- **Fix:** carry each result's path into `OutputFile`, open that, and show which folder a row is in when names repeat.
- **Regression test:** an overwrite run over two folders, then Open on each row opens that row's file.

### ISS-47: Recipe pages preload a far-below-the-fold mascot and don't prioritize their LCP image

**Found:** October 24, 2026 (cold review of the homepage LCP fix, #753, measured with Playwright).

The recipe pages (the SEO landing pages) have the homepage's problem: `/compress-images` preloads `Logo.svg`, `sushi-thumbsup.svg` and `sushi-onigiri.svg`, and onigiri, the FAQ mascot (`_utils/sectionMascots.ts:18`, rendered by `app/(app)/[bnto]/_components/RecipeFaqSection.tsx:18`), sits about 7,000 px down on mobile. The likely LCP image, `RecipeHeroMascot` (250-630 px down), has no `fetchPriority`.

- **Fix:** as on the homepage (#753): lazy below-the-fold images, `fetchPriority="high"` on the hero.
- **Regression test:** extend `e2e/pages/homepage-loading.spec.ts`'s checks to a recipe page; the Lighthouse `lcp-discovery-insight` assertion already covers every recipe URL once its LCP is the image.

### ISS-49: A half-written `.next-e2e` from an interrupted E2E run breaks the web build

**Found:** October 24, 2026 (`task check` on `fix/homepage-lcp`).

`apps/web/tsconfig.json:38-39` includes `.next-e2e/types/**/*.ts` and `.next-e2e/dev/types/**/*.ts`, the generated route types of the isolated E2E server (`E2E_PORT`, which sets `NEXT_DIST_DIR=.next-e2e`). When that server is stopped mid-write, `routes.d.ts` is left truncated, and the next `task check` fails in `@bnto/web#build` with `Type error: Expression expected` at `.next-e2e/dev/types/routes.d.ts:87`. Deleting the gitignored folder fixes it.

- **Fix:** keep the E2E server's generated types out of the app's `tsconfig.json` (Next writes them for its own type-check), or have the isolated run clear `.next-e2e` when it starts.
- **Regression test:** a truncated `.next-e2e/dev/types/routes.d.ts` doesn't fail `pnpm --filter @bnto/web build`.

### ISS-51: The Turbo-inputs check guards lint only, not test or build reads

**Found:** October 24, 2026 (second cold review of the ISS-9 fix). Applies once that fix (#756) merges: the check it describes arrives with it.

`task check:turbo-inputs` (`scripts/turbo/checkLintInputs.ts`) fails when an ESLint config imports a file outside its package that the `lint` task doesn't hash. Nothing checks `test` or `build`. Their root reads (`README.md`, `.prettierrc`, `engine/Cargo.toml`, the catalog snapshot) are listed by hand, so a new one goes stale silently. `@bnto/nodes`' tests import `engine/catalog.snapshot.json` statically (`catalogValidation.test.ts:30`, `nodeTypes.test.ts:9`), which the check's `ts.preProcessFile` code would already catch if it scanned test sources.

- **Fix:** run the same import scan over each package's test and build sources against `test` and `build` inputs. Runtime `readFileSync` reads stay a documented manual step.
- **Regression test:** a package test that imports an unlisted root file fails the check.

### ISS-53: An interrupted move can leave `.bnto-rename-*` files that later runs collect

**Found:** October 24, 2026 (fixing the output node's write, which now reports its swap staging folder on Ctrl+C and has `file-collect` skip it).

`bnto_file::relocate` parks a file under a temporary `.bnto-rename-<n>-<leaf>` name while it runs a chain or swap of renames (`engine/crates/bnto-file/src/relocate/execute.rs:139`). An overwrite-mode output or `file-move` interrupted at that point (Ctrl+C runs `process::exit`, which skips cleanup) leaves the parked file under that name, with nothing reporting it. `file-collect` doesn't skip `.bnto-rename-*`, so a later run over the folder picks it up as an input.

- **Fix:** as the output node's write does for its staging folders (`engine/crates/bnto/src/ordered_write/staging.rs`): register parked names so the Ctrl+C handler names them, and have `file-collect` skip `.bnto-rename-*`.
- **Regression test:** a parked `.bnto-rename-*` file isn't collected, and an interrupted relocate prints where its parked files are.

### ISS-54: The `scripts/seo` tests never run

**Found:** October 24, 2026 (planning the changelog, [changelog.md](strategy/changelog.md)).

`scripts/seo/collectUrls.test.ts` and `scripts/seo/indexNowOutcome.test.ts` run in no workflow, Taskfile task or Turbo task. Outside `.claude/`, a grep for `scripts/seo` finds no runner but the two submit scripts' (`Taskfile.yml:246,250`, `release.yml:601,607`), and the root `test` script is `turbo run test`, which covers workspace packages only. A change that breaks the URL list or the IndexNow outcome parsing passes CI.

- **Fix:** a `task check:scripts` that runs them with `pnpm exec tsx --test` (as `check:turbo-inputs` runs `scripts/turbo`), added to `task check`, plus a CI step. Scheduled in the changelog plan: its PR 1 adds `check:scripts`, and its PR 2 runs both `scripts/seo` tests there and in CI, then deletes this issue.
- **Regression test:** `task check:scripts` fails when one of those tests fails.

### ISS-55: A printed command line with a word that starts with `=` fails in zsh

**Found:** October 25, 2026 (cold review of the ISS-50 fix).

`shell_word` (`engine/crates/bnto/src/shell_word.rs`) treats `=` as plain anywhere in a word, so a word that starts with `=` stays bare. zsh, macOS's default shell, expands a leading `=word` to that command's path: `zsh -c 'echo =1.2'` prints `zsh:1: 1.2 not found`. A declined run's `bnto dry-run … -p …` hint and a dry run's `$` lines both go through `shell_word`, so a pasted line with such a value fails in zsh. `sh` and PowerShell are fine.

- **Fix:** treat `=` as plain only after a word's first char.
- **Regression test:** `shell_word("=1.2")` is quoted, and `version=1.2` stays bare.

### ISS-56: A dry run's command whose program path needs quotes won't paste into PowerShell

**Found:** October 25, 2026 (cold review of the ISS-50 fix).

On Windows, `format_command_line` (`engine/crates/bnto/src/dry_run/shell.rs`) quotes the program like any other word, for PowerShell. PowerShell reads a quoted first word as a string, so `'C:\Program Files\tool.exe' arg` fails with an unexpected token; it runs only as `& 'C:\Program Files\tool.exe' arg`. Unquoted it failed too, before ISS-50's fix.

- **Fix:** on Windows, put `& ` before a program word that needed quotes.
- **Regression test:** a Windows unit test: a step whose `command` holds a space shows `& '<command>' …`.

### ISS-57: `errorCategory` reports `out_of_memory` for any message containing `oom`

**Found:** October 25, 2026 (fixing ISS-27, the error categories).

`categorizeError` (`packages/core/src/lib/categorizeError.ts`) matches `/out of memory|OOM/i`: case-insensitive, with no word boundary. Any message with "oom" in it (`room`, `zoom`, `bloom`) is reported as `out_of_memory`, and the pattern runs before the `load failed`, `file_unreadable` and `decode_failed` patterns, so it can take their messages too. Some engine messages carry a file name (`engine/crates/bnto-core/src/executor/primitive.rs:148`), so a run on `bedroom.jpg` that fails with `Invalid input: Node 'rename': {{item.name}} has no value for bedroom.jpg (it has no item values)` is reported as `out_of_memory` instead of `unknown`.

- **Fix:** match `OOM` only as a whole, upper-case word (`/out of memory/i` or `/\bOOM\b/`).
- **Regression test:** the message above isn't `out_of_memory`, and neither is `"zoom"` or `"Classroom"`, while `"out of memory"` and `"OOM"` still are.

### ISS-58: Inside a loop, a batch node's `{{item.*}}` can't read a key added in the iteration

**Found:** October 25, 2026 (fixing ISS-52, which makes `file-copy` a batch node; see [file-copy-batch.md](decisions/file-copy-batch.md)).

A per-file node inside a loop gets `{{item.*}}` from the file's current metadata, with the loop's item on top (`file_item`, `bnto-core/src/executor/primitive.rs`). A batch node gets it from the loop's item only: `resolve_node_params` resolves it there before the node runs, and refuses a key the loop's item lacks. The loop's item is the file's metadata when the iteration started (`build_loop_context`, `executor/container.rs`). So a key that a step inside the iteration adds (say `file-inspect`, then `file-copy` with `destination: "out/{{item.extension}}"`) fills for a per-file node but fails a batch one with "has no value". `file-copy` was per-file before ISS-52, so this used to work for it. `assert`, `report`, `edit-fields` and `file-render` behave the same way.

- **Fix:** for a node that fills items itself (`resolves_items`), inside a loop, fill only the loop's keys and leave the rest as written, so the node fills them from each file's metadata. Not for `shell-run`, which also declares `resolves_items` but fills only inside `each` groups: a leftover `{{item.*}}` elsewhere in its command must still be refused.
- **Regression test:** inside a loop, `file-inspect` then `file-copy` with `destination: "<dir>/{{item.extension}}"` copies each file into its extension's folder.

### ISS-59: Inside a loop, `file-copy` can still replace an input another iteration reads

**Found:** October 25, 2026 (cold review of the ISS-52 fix; reproduced by the reviewer). **Severity: data loss.**

The ISS-52 fix refuses a `file-copy` over a file another file of its batch is read from. Inside an explicit `loop`, each iteration hands the node one file, so it never sees the others. `F/a.txt` = `AAAA`, `F/b.txt` = `BBBB`, and a loop holding `file-rename` (`a` → `b`) and `file-copy` into `F` with `conflict: overwrite`, run on both files, still leaves `b.txt` = `AAAA`. The same recipe without the loop, flat or with `settings.iteration: "auto"`, is refused. `main` behaved the same before the fix, so this isn't a regression.

- **Fix:** give the node the run's other inputs to check against: the loop's whole batch, or every `Path` the run holds. That needs a way for the executor to pass more than the iteration's files to a node (a `ProcessContext` method, say). Or refuse an `overwrite`/`newer` copy over any file the run read as an input.
- **Regression test:** the repro above inside a `loop` keeps `BBBB` (or is refused before anything is copied).

### ISS-60: A batch node can't see the run's time limit and keeps starting work past it

**Found:** October 25, 2026 (second cold review of the ISS-52 fix, which makes `file-copy` a batch node).

The executor checks the run's `timeout` before each step, loop iteration and per-file node's file (`bnto-core/src/executor/time_limit.rs`, `primitive.rs`), so nothing new starts once it has passed. A batch node gets the whole batch in one call, and `ProcessContext` has no deadline, so it can't stop between files. After the limit, `file-copy` keeps starting copies until the CLI's watchdog ends the process about 2 s later (`bnto/src/run_watchdog.rs`). A copy cut off then can leave its destination truncated: `overwrite` copies with `std::fs::copy`, and `newer` truncates the target and writes in place (`copy_newer.rs`). `file-move` and `file-delete` have the same gap, though a rename or a delete cut off can't leave a half-written file.

- **Fix:** let a node ask whether the run's time is up (a `ProcessContext` method the executor provides, defaulting to "no"), and have `file-copy`, `file-move` and `file-delete` check it before each file, failing with the run's timeout and handing over what they finished.
- **Regression test:** a `file-copy` batch whose run passes its limit after the first file copies no more files and fails with the timeout.

### ISS-61: Two docs name hook and function files in kebab-case; the code uses camelCase

**Found:** October 25, 2026 (fixing ISS-19).

[code-standards.md](rules/code-standards.md)'s "One Export Per File" table gives `use-run-workflow.ts` and `validate-workflow.ts` as file names, and `.claude/skills/pickup/SKILL.md:303` says hooks are `use-kebab-case.ts` and pure functions `kebab-case.ts`. The same rules file's naming table says camelCase (`useRunWorkflow.ts`, `formatDuration.ts`), and so does the code: `git ls-files 'packages/**/use-*.ts' 'apps/**/use-*.ts'` finds no files, against 89 `use[A-Z]*.ts`.

- **Fix:** change both docs to camelCase names.
- **Regression test:** none; a docs fix.

### ISS-62: Three docs let a component keep 80-100 lines of logic inline; ESLint caps a `.tsx` function at 60

**Found:** October 25, 2026 (cold review of the ISS-19 fix).

These say a component keeps its logic inline until it reaches about 80 to 100 lines, and only then earns a hook:

- `.claude/scopes/web/components.md:11` (">80-100 lines of logic + JSX") and `:39` ("100+ lines");
- `.claude/skills/frontend-engineer/SKILL.md:41` ("Hook at ~80-100 lines of logic");
- `.claude/skills/code-review/SKILL.md:173` ("Hooks only when > 80 lines").

ESLint fails any `.tsx` function over 60 lines (`eslint.config.base.mjs`, the `.tsx` override of `max-lines-per-function`), so a component can't get there. [pre-commit.md](rules/pre-commit.md)'s Step 2 says "~20+ lines of logic", so the docs also disagree with each other.

- **Fix:** give all three one threshold that fits under 60 (pre-commit.md's ~20 lines of logic, or reuse, or testability).
- **Regression test:** none; a docs fix.

### ISS-64: A dry run's `Run with: bnto run …` footer drops the dry run's `-p` and `--vars`

**Found:** October 25, 2026 (second cold review of the ISS-40 fix).

`print_footer` (`engine/crates/bnto/src/dry_run/format.rs`) prints `Run with: bnto run <recipe> <input>` from the recipe argument alone. `bnto dry-run drive.bnto.json --vars vars.json` previews `$ echo D:/Art`, then suggests `bnto run drive.bnto.json <input>`, which stops on `unresolved {{vars.Drive}}` unless `BNTO_VAR_DRIVE` is set. A recipe with a required field fails the same way, since the `-p` values are dropped too.

- **Fix:** build the footer from the same `-p` values and `--vars` files, as `DryRunHint` does for the declined-run hint (`engine/crates/bnto/src/dry_run_hint.rs`), each one shell word.
- **Regression test:** `bnto dry-run <path> -p version=1.2.3 --vars vars.json` ends `Run with: bnto run <path> -p version=1.2.3 --vars vars.json <input>`.

### ISS-65: Nothing lints the root `scripts/` folder

**Found:** October 25, 2026 (cold review of the ISS-20 fix).

ESLint runs per workspace package (`turbo run lint`), and the workspaces are `packages/*`, `packages/@bnto/*` and `apps/*` (`pnpm-workspace.yaml`). The repo root has only `eslint.config.base.mjs`, which packages import, so the 14 TypeScript files in `scripts/` (`validate-docs.ts`, `generate-theme.ts`, `scripts/turbo/`, `scripts/seo/`, `scripts/docs/`) get no function-length, file-length or complexity checks. `validate-docs.ts`'s `main()` reached 21 lines unflagged (fixed with ISS-20). Like ISS-16, which is `@bnto/nodes` ignoring its own `scripts/`.

- **Fix:** a root `eslint.config.mjs` over `scripts/**` built on `baseConfig`, run by `task ui:lint` (or a `check:scripts` task) and CI, with any existing overruns on a suppressions baseline.
- **Regression test:** a function over 20 lines in `scripts/` fails lint.

### ISS-66: Down past the last home recipe hides the selection until you press Up as many times

**Found:** October 25, 2026 (fixing ISS-32: a journey that pressed Down too often ran off the Recipes pane).

On the home screen, `CursorDown` in the Recipes pane adds 1 to `recipes_cursor` with no upper bound (`engine/crates/bnto/src/tui/screens/home.rs:133-139`). Its comment says "app.rs clamps if needed", but nothing does: the only other write is `saturating_sub(1)` on Up. Past the last recipe no row matches the cursor (`render_home_panes.rs:84`), so the `▸` marker disappears and the pane scrolls back to the top. The user has to press Up once for every extra Down before a recipe is selected again. Enter still opens the last recipe, because only the confirm path clamps (`app_helpers/home_detail.rs:57`).

- **Fix:** stop at the last recipe in `CursorDown`, the way the Library pane bounds its cursor (it wraps with `%`; pick one behaviour for both panes), and correct the comment.
- **Regression test:** a `home.rs` unit test pressing Down more times than there are recipes keeps the cursor on the last one, and a render check that the marker stays on screen.

### ISS-67: `update_notice.rs` opens its ptys in parallel with no lock

**Found:** October 25, 2026 (cold review of the ISS-32 fix).

`engine/crates/bnto/tests/update_notice.rs:47` calls `native_pty_system().openpty(size)` from `run_in_terminal`, and its four pty tests run in parallel in one process. The ISS-32 fix found that on macOS some of many concurrent launches in one process fail with `failed to openpty: Unknown error: -6` (5 of 160 launches in a probe of 3 processes × 12 threads, 0 of 288 with launches taken in turn, likely `ptsname`'s shared buffer). The TUI journey harness now opens one pty at a time (`OPEN_PTY` in `tests/tui/harness.rs`); this file doesn't. Not yet seen failing in a real run.

- **Fix:** take the same lock around `openpty` and the spawn in `run_in_terminal`, or share the harness's launch.
- **Regression test:** run the file's tests with `--test-threads=16` beside a parallel build several times without a failure.

### ISS-68: `FormStoreContext.tsx` exports four hooks and a provider from one `.tsx` file

**Found:** October 25, 2026 (cold review of the ISS-63 fix).

`packages/@bnto/form/src/FormStoreContext.tsx:63` exports `FormStoreProvider`, `useFormValue`, `useFormValues`, `useFormFiles` and `useFormOnChange`. [code-standards.md](rules/code-standards.md) allows one exported hook per file. Being in a `.tsx` file also puts the four hooks under ESLint's 60-line `.tsx` limit instead of the 20 a hook gets. ISS-63 called `SliderPresets.tsx` the only `.tsx` file exporting a hook, but its search matched `export function use…`, not an `export { … }` list. `git grep -nE "export \{[^}]*use[A-Z]" -- 'packages/*.tsx' 'apps/*.tsx'` finds only this file. `SchemaForm.tsx`, `controls/WatermarkPreviewControl.tsx` and the package's `index.ts` import from it.

- **Fix:** keep the context and `FormStoreProvider` in the `.tsx` file and move each hook to its own `.ts` file (`useFormValue.ts`, `useFormValues.ts`, `useFormFiles.ts`, `useFormOnChange.ts`), updating the three importers.
- **Regression test:** a test per hook that pins what it reads from the provider; lint passing on the moved files.

### ISS-69: E2E specs are never type-checked or linted

**Found:** October 25, 2026 (fixing ISS-28, the automatic download's telemetry).

`apps/web/eslint.config.mjs` ignores `e2e/**`, and the only type-check, `next build` (`task check`, the pre-commit `typecheck` hook), passes while `pnpm exec tsc --noEmit -p apps/web` reports an error in a spec:

```
e2e/pages/homepage-loading.spec.ts(24,14): error TS2353: Object literal may only specify known properties, and 'reducedMotion' does not exist in type 'Fixtures<…>'.
```

`reducedMotion` isn't a `test.use` option, so that line does nothing. Motion is reduced anyway, by `use.contextOptions.reducedMotion` in `apps/web/playwright.config.ts`. But [pre-commit.md](rules/pre-commit.md) tells every spec to "Always set `test.use({ reducedMotion: "reduce" })`", which is the line that fails to type-check (and only that one spec does it).

- **Fix:** type-check the specs (a `tsc --noEmit` step over `apps/web/e2e` in `task check`, the hook and CI), and lint them. Drop the `test.use` line and the pre-commit.md instruction, since the config already reduces motion.
- **Regression test:** a type error in an E2E spec fails `task check`.

### ISS-70: Recipe pages record no download a user clicks for

**Found:** October 25, 2026 (fixing ISS-28, the automatic download's telemetry).

On a recipe page, `result_downloaded` is sent only for the automatic download after a run (`trigger: "auto"`):

- **Per-file downloads send nothing.** The result list's download button calls `actions.downloadResult` (`apps/web/app/(app)/[bnto]/_components/RecipeStepper/RecipeStepperResultList/RecipeStepperResultListRoot.tsx:40`), which only downloads (`_stores/recipeStepperActions.ts`, `downloadResult`).
- **The `downloadAll` action has no caller.** No component calls it, so its `trigger: "manual"` event can't fire. The `download-all-button` test ID is the editor's (`packages/editor/src/components/RunPanel/RunHeader.tsx:104`), and the `execution.downloadAll` string ("Download all") in `@bnto/i18n` isn't used on the recipe page.
- **Not covered here:** the editor (`packages/editor`) makes no telemetry calls at all, so its automatic download (`actions/runExecution.ts:124`) and its "Download all" button (`components/RunPanel/useRunHandlers.ts:35`) send no `result_downloaded` either. The editor is frozen, so whether it should is a separate decision.
- **Fix:** record per-file downloads (a `trigger` of their own, e.g. `"file"`), and either give the recipe page a "Download all" button or delete the unused action.
- **Regression test:** a unit test that `downloadResult` sends `result_downloaded`, and a check that every action in `RecipeStepperActions` has a caller.

### ISS-71: Two `task seo:audit` runs at once collide on port 3000

**Found:** October 25, 2026 (running `task seo:audit` for ISS-28 while another worktree's agent ran it).

`scripts/generate-lighthouserc.ts:24,60` points every audit at `http://localhost:3000` and starts the server with `pnpm --filter @bnto/web start`, Next's default port. Two checkouts auditing at once share that port: one run's Lighthouse loads the other checkout's server, or a server that is shutting down. On October 25 the ISS-28 run failed at `/pricing` with `CHROME_INTERSTITIAL_ERROR` ("Chrome prevented page load with an interstitial") while port 3000 belonged to `next-server` in another worktree. The E2E runs already avoid this with `E2E_PORT` (`apps/web/playwright.config.ts`).

- **Fix:** the URLs can already move: the generator takes `--base-url` (line 24). What's fixed is `startServerCommand` (line 60, Next's default port) and the `seo:audit` task, which passes no `--base-url` (`Taskfile.yml:246`). Take a port from an env var (as `E2E_PORT` does) and use it for both `--base-url` and `next start --port`.
- **Regression test:** two `task seo:audit` runs with different ports at the same time both pass.

### ISS-73: The web runner starts a run with a required param left empty

**Found:** October 25, 2026 (cold review of the ISS-25 fix).

- **What happens:** on Watermark Images, Run is enabled once files are added (`RunButton.tsx:43`, `disabled: !hasFiles`), with no overlay chosen. Since ISS-25's fix the engine refuses the run before any step and the banner names the overlay. But `runAction` (`recipeStepperActions.ts:127-155`) still sends `recipe_run_started` (line 140) and then, through `captureFailed` (line 174), `recipe_run_failed`, so PostHog counts every forgotten overlay as a failed run, which is how ISS-25 was found.
- **Why:** the runner checks only that files exist. Which params are required is already in TypeScript: the generated schemas (`overlay: z.string()`), and the form marks the field required (`packages/@bnto/form/src/SchemaField.tsx:34`). What's missing is a check for a blank value.
- **Fix:** a pure function, from the recipe's processor schemas and the stepper's config, that lists required params with no value (absent, null or blank text, the engine's rule in `bnto_core::required_params`). Run stays disabled, and labelled with the first one, until it's empty.
- **Regression test:** a unit test for the function, and the watermark E2E: Run is disabled until an overlay is uploaded, and no `recipe_run_failed` is sent.

### ISS-74: A required param with no value is refused only after consent and after `-o` is made

**Found:** October 25, 2026 (cold review of the ISS-25 fix).

- **What happens:** `bnto run watermark-images photo.jpg --yes -o out` exits 1 with `Node 'overlay': param "overlay" (Overlay Image) is required`, and leaves an empty `out/`. A recipe that needs approval asks for it first, then is refused. `bnto dry-run watermark-images photo.jpg` ends with `Run with: bnto run watermark-images <input>`, a command that would be refused, and the run's message doesn't name the `-p` that fills it.
- **Why:** the check is in the engine's pre-flight (`executor/raw_params.rs`, called from `execute_pipeline`, `executor/mod.rs:193`), which `bnto run` reaches after `check_trust` (`bnto/src/run.rs:78`) and `run_target` (`run.rs:91`). Undeclared params and `validate_raw` refusals have always been checked there too. Unfilled `{{fields.*}}` are refused before consent ("so nobody approves a run the engine would refuse", `run.rs`), and ISS-22 is the same folder-first problem for `{{env.*}}`.
- **Fix:** have `bnto run` and `bnto dry-run` run the raw-param checks (undeclared, required with no value, `validate_raw`) on the definition with `-p` applied, before consent and before any folder is made, naming the `-p node:param=…` that fills a required one, as `refuse_unfilled` does for fields.
- **Regression test:** a CLI test: a bundled recipe left with an empty required param is refused before consent, names the `-p`, and creates no `-o` folder. The dry run says so too.

### ISS-75: The CLI can't give Watermark Images an overlay: `-p` takes base64 text, not a file

**Found:** October 25, 2026 (second cold review of the ISS-25 fix).

- **What happens:** `bnto check` notes Watermark Images' overlay as `left for run time; give it with bnto run -p overlay:overlay=<value>`, but the value must be the image as base64 text. `bnto run watermark-images photo.jpg --yes -o out -p overlay:overlay=photo.jpg` fails with `Invalid base64 in overlay parameter` (`engine/crates/bnto-image/src/overlay/params.rs:69`). Nothing in `engine/crates/bnto/src` reads a file-type param from a path, so the recipe is out of reach from the CLI in practice; `check/run_time/mod.rs:180` documents that "a file param takes its base64 text".
- **Fix:** let `-p` give a `File` param as a path that the CLI reads and encodes, and say so in the note. ISS-74's `-p` hint should then name a path, not a blob.
- **Regression test:** a CLI test: `bnto run watermark-images photo.jpg -p overlay:overlay=<logo.png path>` writes a watermarked image.
