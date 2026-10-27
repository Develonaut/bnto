# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** from the next batch in the [Work order](#work-order), by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-103**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

## Work order

Agreed with Ryan on October 27, 2026. Take one batch at a time, one PR per issue (or per tightly coupled pair), and finish a batch before starting the next. When an issue is fixed or a new one is logged, update this list in the same PR.

Done, October 27, 2026: batch 1, engine file operations (ISS-83 #803, ISS-87 #804, ISS-82 #805). Batch 2 so far: ISS-98 #806, ISS-66 #807, ISS-97 #808, ISS-101. Batches keep their numbers.

| #   | Batch                      | Issues                                                 | Why                                                                                                                                                                                                                                                                                                             |
| --- | -------------------------- | ------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2   | TUI correctness            | ISS-102                                                | Footer hints for the modes still showing a screen's usual keys: search boxes, field editing, the editor's node picker                                                                                                                                                                                           |
| 3   | Checks before a run starts | ISS-75, ISS-74, ISS-79, ISS-96, ISS-73                 | `-p` can't give a file param a path (ISS-75, first: ISS-74's hint names one); a required param refused after consent and `-o` (ISS-74); an unclosed `{{` in a folder name (ISS-79); a dry-run footer that always ends in `<input>` (ISS-96); the web runner starting a run with a required param empty (ISS-73) |
| 4   | Engine file nodes          | ISS-17, ISS-33, ISS-34, ISS-100                        | ISS-34 changes a shipped node's behavior, so `/workflow-expert` reviews it first, and ISS-17 makes that skill's node inventory current before it does; one shared extension split for a dotted folder (ISS-33); `file-move`'s refusals claiming "nothing moved" mid-loop (ISS-100)                              |
| 5   | Engine templating          | ISS-35, ISS-58                                         | A value filled in from data is filled again as a template (ISS-35); a loop fills `{{item.*}}` keys a batch node should fill itself (ISS-58). If the [template-filters plan](PLAN.md#engine-from-the-october-21-dogfood-sessions) is written first, it takes both                                                |
| 6   | Small code cleanups        | ISS-90, ISS-88, ISS-91, ISS-93, ISS-94, ISS-84, ISS-85 | Dead exports, dot-notation leftovers, `React.*` types, a stale README, a stale codegen file, the E2E helper grab bag                                                                                                                                                                                            |
| 7   | Tooling and tests          | ISS-86, ISS-92, ISS-95, ISS-89, ISS-67, ISS-8, ISS-18  | Prettier and `scripts/` type checks unenforced, a misleading crate check, a `seo:audit` hint, a test lock, CLI tests touching the real `~/.bnto`, a slow Windows test                                                                                                                                           |
| 8   | Analytics                  | ISS-29, ISS-57, ISS-70                                 | Exception capture off and runs without one outcome event (ISS-29); a wrong `out_of_memory` category (ISS-57); recipe-page downloads never recorded (ISS-70)                                                                                                                                                     |
| 9   | Web performance            | ISS-99, ISS-37                                         | Public pages render per request (ISS-99); the homepage's ~500 KB of JS (ISS-37). Larger and riskier, so last                                                                                                                                                                                                    |

**Waiting on Ryan:**

- ISS-12 (ship the PostHog key in released CLIs, or say telemetry is off), then ISS-7, whose page says which.
- ISS-31 (a PostHog project setting, not a repo change).
- ISS-44 (pick option a or b).
- ISS-45 (what progressive writes mean in each output mode).

## Open issues

| ID      | Issue                                                                                                           | Area        | Status                                           |
| ------- | --------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------ |
| ISS-7   | The CLI's telemetry notice links `bnto.io/telemetry`, a 404                                                     | web docs    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-8   | Most CLI integration tests run `bnto` against the real `~/.bnto`                                                | cli test    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-12  | Released CLIs send no telemetry: the PostHog key is never set                                                   | cli         | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-17  | The `/workflow-expert` skill's node inventory is stale                                                          | docs        | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-18  | A Windows process test times out waiting 20 s for PowerShell                                                    | engine test | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-29  | Errors on bnto.io go unseen: exception capture is off                                                           | core        | Open                                             |
| ISS-31  | Another app's events land in bnto.io's PostHog project                                                          | analytics   | Open                                             |
| ISS-33  | Output names and extension reads split on a dot in a folder name                                                | engine      | Open                                             |
| ISS-34  | `file-rename`'s `prefix`, `case` and `sanitize` rename the folders too                                          | engine      | Open                                             |
| ISS-35  | Inside a loop, a data value is filled in as a template                                                          | engine      | Open                                             |
| ISS-37  | The homepage loads ~500 KB of JS that competes with its CSS                                                     | web perf    | Open                                             |
| ISS-44  | In the browser, a renamed or removed param's error says to run `bnto migrate` (needs Ryan's decision)           | web         | Open                                             |
| ISS-45  | Progressive loop writes in message/none mode land in the folder unlisted                                        | cli tui     | Open                                             |
| ISS-57  | `errorCategory` reports `out_of_memory` for any message containing `oom`                                        | core        | Open                                             |
| ISS-58  | Inside a loop, a batch node's `{{item.*}}` can't read a key added in the iteration                              | engine      | Open                                             |
| ISS-67  | `update_notice.rs` opens its ptys in parallel with no lock                                                      | cli test    | Open                                             |
| ISS-70  | Recipe pages record no download a user clicks for                                                               | web         | Open                                             |
| ISS-73  | The web runner starts a run with a required param left empty                                                    | web         | Open                                             |
| ISS-74  | A required param with no value is refused only after consent and after `-o` is made                             | cli         | Open                                             |
| ISS-75  | The CLI can't give Watermark Images an overlay: `-p` takes base64 text, not a file                              | cli         | Open                                             |
| ISS-79  | An unclosed `{{` in the output folder becomes part of its name, and `bnto check` passes it                      | cli         | Open                                             |
| ISS-84  | E2E specs pass `navigateToRecipe` a page heading it ignores                                                     | web test    | Open                                             |
| ISS-85  | `e2e/helpers.ts` is a 16-export grab bag the grab-bag check skips                                               | web test    | Open                                             |
| ISS-86  | Nothing checks Prettier formatting in CI or `task check`; 54 TS files fail it                                   | tooling     | Open                                             |
| ISS-88  | `FileUpload` context errors and two web comments name dot-notation components                                   | ui          | Open                                             |
| ISS-89  | In a fresh worktree, `task seo:audit` fails at the first URL with no hint: there's no `.env.local`              | tooling     | Open                                             |
| ISS-90  | `@bnto/form` exports `FormStoreProvider` and four hooks nothing outside it uses                                 | form        | Open                                             |
| ISS-91  | 28 TS files use `React.*` namespace types the import rule bans                                                  | web         | Open                                             |
| ISS-92  | Nothing type-checks the `scripts/` folders: `tsx` runs them unchecked                                           | tooling     | Open                                             |
| ISS-93  | `@bnto/nodes`'s README lists `src/` files and folders that don't exist                                          | docs        | Open                                             |
| ISS-94  | Codegen never removes a stale `iterationModes.ts` from `src/generated/catalog/`                                 | tooling     | Open                                             |
| ISS-95  | The own-version check reports a failed crate download as a changed crate                                        | tooling     | Open                                             |
| ISS-96  | A dry run's `Run with:` footer ends in `<input>` even for a recipe that takes none                              | cli         | Open                                             |
| ISS-99  | Every web page renders per request; recipe pages' `generateStaticParams` doesn't make them static               | web perf    | Open                                             |
| ISS-100 | `file-move`'s older refusals say "nothing moved" in a loop iteration after earlier ones moved files             | engine      | Open                                             |
| ISS-102 | The TUI footer shows a screen's usual hints during its search boxes, field editing and the editor's node picker | tui         | Open                                             |

---

### ISS-7: The CLI's telemetry notice links `bnto.io/telemetry`, a 404

**Found:** October 23, 2026 (fixing ISS-2).

The first-run notice (`engine/crates/bnto/src/telemetry/consent.rs:27`) says bnto "collects anonymous usage data" and sends people to `https://bnto.io/telemetry` for more. That URL renders the `[bnto]` route's "Page Not Found" (`curl -sL https://www.bnto.io/telemetry | grep -o not-found-heading`), and no page says what the CLI collects: `privacy/privacy.mdx` lists PostHog without mentioning the CLI, and the CLI recipe FAQ (`apps/web/lib/cliRecipeFaq.ts:9`) has one line. Add a `/telemetry` page listing the four events (`cli_command`, `cli_recipe_run`, `cli_error`, `cli_tui_session`), their properties (bundled slugs only, else `"custom"`; bucketed counts, sizes and durations), the anonymous id and the three opt-outs, or point the notice at a page that has them.

### ISS-8: Most CLI integration tests run `bnto` against the real `~/.bnto`

**Found:** October 23, 2026 (fixing ISS-3).

`cli_commands.rs` and the run helpers in `tests/helpers/mod.rs` now use `helpers::Sandbox` (temp HOME, BNTO_HOME, config dir and working directory, telemetry off). About 17 other files in `engine/crates/bnto/tests/` still spawn `Command::new(bnto_bin())` with no HOME or BNTO_HOME: `assert_files`, `check_edges`, `check_folders`, `check_run_time`, `edges`, `edit_fields`, `file_delete`, `file_inspect`, `file_join`, `progressive_output`, `report_file_names`, `run_args`, `run_json`, `run_temp_files`, `shell_each`, `shell_options`, and some runs in `dry_run_temp_files` and `run_timeout`. Each run writes a session log into the developer's `~/.bnto/state/logs`: `cargo test -p bnto --test edges --test run_json` added 3. A run that resolves a slug would also read their library. Switch them to `Sandbox::new().command()`.

### ISS-12: Released CLIs send no telemetry: the PostHog key is never set

**Found:** October 24, 2026 (planning ISS-7).

`engine/crates/bnto/src/telemetry/mod.rs:57` reads the key from the `BNTO_POSTHOG_KEY` env var at run time, and telemetry is inactive without one (`:60`). Nothing sets it: there's no `option_env!`, no build script, and nothing in `release.yml` (`grep -rn BNTO_POSTHOG_KEY` finds only `mod.rs` and a test). So shipped CLIs collect nothing, while the first-run notice says bnto "collects anonymous usage data" and the code comment calls the key "safe to embed". `bnto telemetry status` also reports "Effective: true" without checking for a key (`mod.rs:134`). Needs Ryan's decision: ship the key, or say telemetry is off.

### ISS-17: The `/workflow-expert` skill's node inventory is stale

**Found:** October 21, 2026 (dogfood sessions); checked October 24, 2026.

`.claude/skills/workflow-expert/SKILL.md:30` still says recipes use "the 6 implemented browser operations", and its operations table predates most of today's nodes. The skill reviews every new node and param, so a stale inventory steers it wrong.

### ISS-18: A Windows process test times out waiting 20 s for PowerShell

**Found:** October 24, 2026 (v0.18.2 release run 40096930376).

`terminate_all_kills_orphaned_grandchild_on_windows` (`engine/crates/bnto/src/context.rs:743`) starts `cmd /c start /b powershell … $PID; Start-Sleep 30` and waits once, `recv_timeout(20 s)`, for PowerShell to print its PID (`:758-759`). On the release's Windows runner PowerShell didn't print in time: `called Result::unwrap() on an Err value: Timeout`. The same commit passed the same job on #743's CI an hour earlier, so the failure is timing, not code. It failed the release's CI Gate. Wait on PowerShell's start with a longer bound, or start a lighter process for the grandchild, so a slow runner can't fail a release.

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

### ISS-44: In the browser, a renamed or removed param's error says to run `bnto migrate`

**Found:** October 24, 2026 (cold review of the ISS-15 fix, which removes `vector-optimize`'s `precision`; confirmed by running the committed WASM in Node).

The engine refuses an undeclared param before any step runs, for every consumer, and its hint for a renamed or removed one says to run `bnto migrate` (`bnto-core/src/renamed.rs`, applied in `executor/raw_params.rs:35`). In the browser that's a CLI command the user may not have. The editor keeps unknown params as they are (`definitionToGraph.ts:68`, `rfNodesToDefinition.ts:53`) and its config panel no longer shows them, so someone who opens an old exported recipe, or has an old draft in sessionStorage, can't clear it from the UI. The earlier renames (`PARAMS`, `NODE_TYPES`) behave the same way.

- **Decision for Ryan:**
  - **(a) Migrate in the browser:** run the same migrations when a recipe is opened or imported (they're pure JSON rewrites), so old recipes upgrade silently.
  - **(b) Ignore removed params:** pre-flight skips `REMOVED_PARAMS` (they did nothing by definition), while `bnto check` still warns and points at `bnto migrate`. This departs from "undeclared params fail loudly" for removals only.
- **Regression test:** an old recipe with `precision` opened in the web editor runs (a) or is accepted with a warning (b), with no CLI step.

### ISS-45: Progressive loop writes in message/none mode land in the folder unlisted

**Found:** October 24, 2026 (second cold review of the ISS-21 fix; reproduced with the CLI, temp `HOME` and `BNTO_HOME`).

A loop with `outputPersistence: "progressive"` copies each iteration's files into the output node's `directory` even when the output is in `message` or `none` mode, which promise to write nothing: `file-list-loop-progressive.bnto.json` with `mode: "message"` and a `directory` left `a.txt` and `b.txt` there. `bnto run` (`progressive_dir`, `run.rs:230`) and the TUI both do it. Outside preview and overwrite mode, the TUI bridge's reporter copies each finished iteration's files on the pipeline's thread, into the folder that `resolve_output_dir` (`tui/bridge.rs:215`) resolves and creates: see `progressive_dir` and `channel_reporter`, `tui/bridge/reporter.rs:18-44`. Since the ISS-21 fix (#750), the TUI's results list doesn't show those files. In write mode, an iteration file that a later step renames also stays on disk under its old name.

- **Fix:** decide what progressive persistence means per mode: either only in write mode, or always, with those files listed. Then make the CLI and the TUI agree.
- **Regression test:** a progressive loop in message mode with a `directory` leaves the folder untouched (or lists what it wrote), in `bnto run` and the TUI bridge.

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

### ISS-67: `update_notice.rs` opens its ptys in parallel with no lock

**Found:** October 25, 2026 (cold review of the ISS-32 fix).

`engine/crates/bnto/tests/update_notice.rs:47` calls `native_pty_system().openpty(size)` from `run_in_terminal`, and its four pty tests run in parallel in one process. The ISS-32 fix found that on macOS some of many concurrent launches in one process fail with `failed to openpty: Unknown error: -6` (5 of 160 launches in a probe of 3 processes × 12 threads, 0 of 288 with launches taken in turn, likely `ptsname`'s shared buffer). The TUI journey harness now opens one pty at a time (`OPEN_PTY` in `tests/tui/harness.rs`); this file doesn't. Not yet seen failing in a real run.

- **Fix:** take the same lock around `openpty` and the spawn in `run_in_terminal`, or share the harness's launch.
- **Regression test:** run the file's tests with `--test-threads=16` beside a parallel build several times without a failure.

### ISS-70: Recipe pages record no download a user clicks for

**Found:** October 25, 2026 (fixing ISS-28, the automatic download's telemetry).

On a recipe page, `result_downloaded` is sent only for the automatic download after a run (`trigger: "auto"`):

- **Per-file downloads send nothing.** The result list's download button calls `actions.downloadResult` (`apps/web/app/(app)/[bnto]/_components/RecipeStepper/RecipeStepperResultList/RecipeStepperResultListRoot.tsx:40`), which only downloads (`_stores/recipeStepperActions.ts`, `downloadResult`).
- **The `downloadAll` action has no caller.** No component calls it, so its `trigger: "manual"` event can't fire. The `download-all-button` test ID is the editor's (`packages/editor/src/components/RunPanel/RunHeader.tsx:104`), and the `execution.downloadAll` string ("Download all") in `@bnto/i18n` isn't used on the recipe page.
- **Not covered here:** the editor (`packages/editor`) makes no telemetry calls at all, so its automatic download (`actions/runExecution.ts:124`) and its "Download all" button (`components/RunPanel/useRunHandlers.ts:35`) send no `result_downloaded` either. The editor is frozen, so whether it should is a separate decision.
- **Fix:** record per-file downloads (a `trigger` of their own, e.g. `"file"`), and either give the recipe page a "Download all" button or delete the unused action.
- **Regression test:** a unit test that `downloadResult` sends `result_downloaded`, and a check that every action in `RecipeStepperActions` has a caller.

### ISS-73: The web runner starts a run with a required param left empty

**Found:** October 25, 2026 (cold review of the ISS-25 fix).

- **What happens:** on Watermark Images, Run is enabled once files are added (`RunButton.tsx:43`, `disabled: !hasFiles`), with no overlay chosen. Since ISS-25's fix the engine refuses the run before any step and the banner names the overlay. But `runAction` (`recipeStepperActions.ts:127-155`) still sends `recipe_run_started` (line 140) and then, through `captureFailed` (line 174), `recipe_run_failed`, so PostHog counts every forgotten overlay as a failed run, which is how ISS-25 was found.
- **Why:** the runner checks only that files exist. Which params are required is already in TypeScript: the generated schemas (`overlay: z.string()`), and the form marks the field required (`packages/@bnto/form/src/SchemaField.tsx:34`). What's missing is a check for a blank value.
- **Fix:** a pure function, from the recipe's processor schemas and the stepper's config, that lists required params with no value (absent, null or blank text, the engine's rule in `bnto_core::required_params`). Run stays disabled, and labelled with the first one, until it's empty.
- **Regression test:** a unit test for the function, and the watermark E2E: Run is disabled until an overlay is uploaded, and no `recipe_run_failed` is sent.

### ISS-74: A required param with no value is refused only after consent and after `-o` is made

**Found:** October 25, 2026 (cold review of the ISS-25 fix).

- **What happens:** `bnto run watermark-images photo.jpg --yes -o out` exits 1 with `Node 'overlay': param "overlay" (Overlay Image) is required`, and leaves an empty `out/`. A recipe that needs approval asks for it first, then is refused. `bnto dry-run watermark-images photo.jpg` ends with `Run with: bnto run watermark-images <input>`, a command that would be refused, and the run's message doesn't name the `-p` that fills it.
- **Why:** the check is in the engine's pre-flight (`executor/raw_params.rs`, called from `execute_pipeline`, `executor/mod.rs:193`), which `bnto run` reaches after `check_trust` (`bnto/src/run.rs:85`) and after `progressive_dir` makes the `-o` folder (`run.rs:230-234`). Undeclared params and `validate_raw` refusals have always been checked there too. Unfilled `{{fields.*}}`, inputs and the output folder's placeholders (since the ISS-22 fix) are refused before consent ("so nobody approves a run the engine would refuse", `run.rs:75-83`).
- **Fix:** have `bnto run` and `bnto dry-run` run the raw-param checks (undeclared, required with no value, `validate_raw`) on the definition with `-p` applied, before consent and before any folder is made, naming the `-p node:param=…` that fills a required one, as `refuse_unfilled` does for fields. The TUI makes its pre-consent checks in `tui/bridge/precheck.rs` (`check_then_consent`, [decision](decisions/tui-consent-in-bridge.md)); add the same check there.
- **Regression test:** a CLI test: a bundled recipe left with an empty required param is refused before consent, names the `-p`, and creates no `-o` folder. The dry run says so too.

### ISS-75: The CLI can't give Watermark Images an overlay: `-p` takes base64 text, not a file

**Found:** October 25, 2026 (second cold review of the ISS-25 fix).

- **What happens:** `bnto check` notes Watermark Images' overlay as `left for run time; give it with bnto run -p overlay:overlay=<value>`, but the value must be the image as base64 text. `bnto run watermark-images photo.jpg --yes -o out -p overlay:overlay=photo.jpg` fails with `Invalid base64 in overlay parameter` (`engine/crates/bnto-image/src/overlay/params.rs:69`). Nothing in `engine/crates/bnto/src` reads a file-type param from a path, so the recipe is out of reach from the CLI in practice; `check/run_time/mod.rs:180` documents that "a file param takes its base64 text".
- **Fix:** let `-p` give a `File` param as a path that the CLI reads and encodes, and say so in the note. ISS-74's `-p` hint should then name a path, not a blob.
- **Regression test:** a CLI test: `bnto run watermark-images photo.jpg -p overlay:overlay=<logo.png path>` writes a watermarked image.

### ISS-79: An unclosed `{{` in the output folder becomes part of its name, and `bnto check` passes it

**Found:** October 25, 2026 (cold review of the ISS-22 fix).

The ISS-22 fix refuses every `{{…}}` in the output node's `directory` that no run fills, but only closed ones. `fill_output_directory` reads placeholders with the executor's `interpolate`, which keeps an unclosed `{{` as text (`bnto-core/src/executor/interpolate.rs:15-17`), so `directory: "out-{{ctx.date"` names a folder `out-{{ctx.date`. `bnto check` passes it (`Check: PASS (1 recipe)`): its malformed-placeholder finding never sees I/O nodes, since `check_node` returns early for them (`bnto-core/src/check/mod.rs:137-141`).

- **Fix:** refuse an unclosed `{{` in the folder in `fill_output_directory`, and have `bnto check` run its placeholder-form check on the output node's `directory`.
- **Regression test:** `out-{{ctx.date` is refused by `bnto run` with no folder made, and `bnto check` flags it.

### ISS-84: E2E specs pass `navigateToRecipe` a page heading it ignores

**Found:** October 25, 2026 (fixing ISS-69, linting the E2E specs).

`navigateToRecipe(page, slug, _h1?)` (`apps/web/e2e/helpers.ts:45`) waits for the `recipe-heading` test ID and never reads `_h1`. It checked the heading's text until #208 standardized selectors on `getByTestId`, which kept the parameter. 64 calls in 18 specs still pass a heading: 62 as text (`navigateToRecipe(page, "clean-csv", "Clean CSV Online Free")`) and 2 as `recipe.h1` (`pipeline-smoke.spec.ts:72`, `recipe-switching.spec.ts:110`). Each reads as if it checks the page title, and a page that renders the wrong heading passes.

- **Fix:** either assert the heading text again (`expect(page.getByTestId("recipe-heading")).toHaveText(h1)`), or drop the parameter and the 64 arguments.
- **Regression test:** if the check comes back, a spec that navigates with the wrong heading fails.

### ISS-85: `e2e/helpers.ts` is a 16-export grab bag the grab-bag check skips

**Found:** October 25, 2026 (cold review of the ISS-69 fix).

[code-standards.md](rules/code-standards.md) allows one exported function per file and bans `helpers.ts` grab bags, which `scripts/hooks/check-grab-bags.sh` rejects by name at commit. `apps/web/e2e/helpers.ts` exports 16 functions and constants (`navigateToRecipe`, `uploadFiles`, `runAndComplete`, `downloadAndVerify`, …), but the hook's lefthook entry and `task check:grab-bags` both leave out `**/e2e/**` (`lefthook.yml:79`, `Taskfile.yml:178`), so it's never checked. Since ISS-69 the E2E specs are linted, but no lint rule counts exports.

- **Fix:** split `helpers.ts` by domain under `e2e/helpers/` (some already live there: `assertions.ts`, `editor.ts`, `parseCsv.ts`) with one export per file, update the imports, and stop excluding `e2e/` from the grab-bag check.
- **Regression test:** `task check:grab-bags -- apps/web/e2e/helpers.ts` fails until the split, then the file is gone.

### ISS-86: Nothing checks Prettier formatting in CI or `task check`; 54 TS files fail it

**Found:** October 25, 2026 (cold review of the ISS-69 fix).

Formatting is applied only by the pre-commit hook (`prettier --write {staged_files}`, `lefthook.yml`), which a commit without hooks skips. Neither `.github/workflows/ci.yml` nor `task check` runs `prettier --check` (the root `format:check` script has no caller). `pnpm prettier --check "apps/**/*.{ts,tsx}" "packages/**/*.{ts,tsx}"` reports 54 files, most in `packages/core/src` (15), `packages/ui/src` (11) and `packages/@bnto/backend` (11), and `apps/web/e2e/telemetryHelper.ts`, untouched since #32.

- **Fix:** format the files in one PR, then add `pnpm format:check` (or a narrower glob) to `task check` and CI's TypeScript job.
- **Regression test:** an unformatted file fails `task check` and CI.

### ISS-88: `FileUpload` context errors and two web comments name dot-notation components

**Found:** October 25, 2026 (cold review of the ISS-80/ISS-81 fix).

[gotchas.md](rules/gotchas.md) bans dot-notation components, and `@bnto/ui` exports `FileUpload`'s parts flat (`packages/ui/src/interaction/FileUpload/index.ts`: `FileUploadDropzone`, `FileUploadItem`, `FileUploadClear`, …). Its context hooks still name the parts with dots in their runtime errors, so a misplaced part throws "`FileUpload.Dropzone` must be used within `FileUpload`":

- the `consumer` strings passed to `useFileUploadContext` / `useFileUploadItemContext` in `FileUploadClear.tsx:19`, `FileUploadDropzone.tsx:22`, `FileUploadItemDelete.tsx:19-20` and `FileUploadItemMetadata.tsx:12`;
- the item context's error, "must be used within `FileUpload.Item`" (`context.ts:43`).

Two `apps/web` comments do the same: `app/editor/layout.tsx:12` ("`AppShell.Main` default clearance"; the export is `AppShellMain`) and `components/blocks/AuthGate.tsx:22` (the section header `AuthGate.Action`).

- **Fix:** use the flat names (`FileUploadDropzone`, `FileUploadItem`, `AppShellMain`, `AuthGateAction`).
- **Regression test:** a `@bnto/ui` test that rendering `FileUploadDropzone` outside `FileUpload` throws an error naming `FileUploadDropzone`.

### ISS-89: In a fresh worktree, `task seo:audit` fails at the first URL with no hint: there's no `.env.local`

**Found:** October 25, 2026 (proving the ISS-71 fix in a new worktree).

`apps/web/.env.local` is gitignored, so a new worktree has none. `pnpm --filter @bnto/web build` passes without it, but the built server answers every page with a 500 (`Error: Missing environment variable NEXT_PUBLIC_CONVEX_URL`, from `next start`), and the audit stops at its first URL with Lighthouse's `ERRORED_DOCUMENT_REQUEST`. Neither the task's output nor the docs say why. Copying the main checkout's `apps/web/.env.local` fixes it.

- **Fix:** have `seo:audit` refuse before the build when `NEXT_PUBLIC_CONVEX_URL` is unset, naming `apps/web/.env.local` and [environment-variables.md](environment-variables.md).
- **Regression test:** `task seo:audit` in a checkout with no `apps/web/.env.local` fails before building, naming the file.

### ISS-90: `@bnto/form` exports `FormStoreProvider` and four hooks nothing outside it uses

**Found:** October 25, 2026 (cold review of the ISS-68 fix).

`packages/@bnto/form/src/index.ts:13-17` re-exports `FormStoreProvider`, `useFormValue`, `useFormValues`, `useFormFiles` and `useFormOnChange`. Searching `packages` and `apps` outside `packages/@bnto/form` for those names finds nothing: consumers import only `SchemaForm`, `SchemaField` and their types (`apps/web/app/(app)/[bnto]/_components/DynamicRecipeConfig.tsx:6`, `packages/editor/src/index.ts:101-104`). `SchemaForm` mounts the provider itself, and the controls read it from inside the package, so the five exports are dead ([code-standards.md](rules/code-standards.md), YAGNI). `useFormValue` has no caller even inside the package, only its test. knip doesn't flag them because it treats a package entry's exports as public.

- **Fix:** drop the five re-exports from `index.ts`, and delete `useFormValue` and its test unless a control needs it.
- **Regression test:** none; `task check` (build, lint, knip) passes without the exports.

### ISS-91: 28 TS files use `React.*` namespace types the import rule bans

**Found:** October 25, 2026 (cold review of the ISS-68 fix).

[code-standards.md](rules/code-standards.md) ("Import Discipline") says to use named type imports (`import type { ComponentProps } from "react"`), never `React.ComponentProps` and the like. A search for `React\.[A-Z]` in `packages/` and `apps/` `.ts`/`.tsx` files lists 28 files, e.g. `children: React.ReactNode` (`packages/@bnto/form/src/FormStoreContext.tsx:30`, `packages/core/src/BntoCoreProvider.tsx:10`), `React.ChangeEvent` (`packages/@bnto/form/src/controls/TextControl.tsx:9`), `React.CSSProperties` (`apps/web/app/globalErrorStyles.ts`) and `React.ComponentProps` (`apps/web/components/blocks/NavButton.tsx:21`). They compile through the global `React` namespace from `@types/react`, so nothing catches them.

- **Fix:** switch each to a named `import type` from `"react"`, and add a lint rule that bans `React.` type references (`no-restricted-syntax` on a `TSQualifiedName` whose left side is `React`) so new ones fail.
- **Regression test:** a file that writes `React.ReactNode` fails `task ui:lint`.

### ISS-92: Nothing type-checks the `scripts/` folders: `tsx` runs them unchecked

**Found:** October 25, 2026 (fixing ISS-65).

The root `scripts/` folder and `packages/@bnto/nodes/scripts/` and `packages/@bnto/registry/scripts/` run through `pnpm exec tsx`, which strips types without checking them. The repo root has no `tsconfig.json`, and the two packages' `tsconfig.json` files `include` only `src` (`packages/@bnto/nodes/tsconfig.json`), so `turbo run build` (`tsc --noEmit`) never sees a script. ESLint's `baseConfig` uses typescript-eslint's non-type-checked rules, so lint doesn't catch type errors either. While fixing ISS-65, ESLint's autofix for `unicorn/no-useless-undefined` turned `writeConfig(url, undefined)` into `writeConfig(url)` against a required `port: number | undefined` parameter (`scripts/generate-lighthouserc.ts`): a type error that `task check:scripts` and its tests passed.

- **Fix:** a `tsconfig.json` per scripts folder (or a root one over `scripts/**`) and a `tsc --noEmit` over each in `task check:scripts`, the packages' `build`, and CI.
- **Regression test:** a type error in a `scripts/` file fails `task check`.

### ISS-93: `@bnto/nodes`'s README lists `src/` files and folders that don't exist

**Found:** October 25, 2026 (fixing ISS-16).

`packages/@bnto/nodes/README.md`'s "Directory Structure" tree and its "Generated vs Hand-Written" table describe an older layout. They list `src/generated/catalog.ts` and `src/generated/schemas.ts` (the monolithic files codegen now deletes on every run, `scripts/codegen/paths.ts` `LEGACY_FILES`), a `src/recipes/` folder of hand-written recipes (recipes are generated into `src/generated/recipes.ts`), `src/recipe.ts`, and `src/schemas/imageCompress.ts`, `inferFieldType.ts` and other per-operation configs. None of these exist. The generated `catalog/`, `schemas/`, `paramFieldInfo.ts` and the node CRUD files (`addNode.ts`, `moveNode.ts`, …) aren't listed. The `/technical-writer` skill owns package READMEs.

- **Fix:** rewrite the tree and the table from the package as it is.
- **Regression test:** none (docs); every path the README names exists.

### ISS-94: Codegen never removes a stale `iterationModes.ts` from `src/generated/catalog/`

**Found:** October 26, 2026 (cold review of the ISS-16 split; the bug predates it).

Codegen writes `catalog/iterationModes.ts` only when the definition schema has an iteration enum (`packages/@bnto/nodes/scripts/codegen/catalog/writeCatalogFiles.ts`), and nothing cleans the `catalog/` root, unlike `nodeTypes/` and `processors/` (`writePerItemFolder`). With the enum gone, the old file stays on disk with an outdated version header while `catalog/index.ts` stops exporting it. Reproduced with old and new scripts by deleting `definitionSchema` from a scratch snapshot. The old script built an `expectedCatalogFiles` set for this and never used it; the split dropped that dead set.

- **Fix:** remove `iterationModes.ts` when there are no modes (or clean the `catalog/` root against the files written).
- **Regression test:** codegen on a snapshot without the enum, after one with it, leaves no `catalog/iterationModes.ts`.

### ISS-95: The own-version check reports a failed crate download as a changed crate

**Found:** October 26, 2026 (cold review of the ISS-10 fix; the bug predates it).

In `scripts/check-own-version-crates.sh`, `differs_from_published` runs inside an `if`, so `set -e` doesn't apply in it. When the crates.io index answers but the `.crate` download (`curl … | tar xz`) fails, the function goes on: `diff` compares against a missing folder and exits 2, and the leading `!` turns that into "differs". The script then says `tonkotsu changed since X was published` and asks for a bump that isn't needed. It fails, so it can't pass a bad change, but the message misleads. Read from the code; not reproduced.

- **Fix:** return a distinct status when the download or `cargo package` fails, and report it like an unreachable crates.io.
- **Regression test:** with the download URL failing (a proxy that lets the index through, or a stubbed `curl`), the script doesn't print "changed since".

### ISS-96: A dry run's `Run with:` footer ends in `<input>` even for a recipe that takes none

**Found:** October 26, 2026 (cold review of the ISS-64 fix; the bug predates it).

`DryRunHint::run_command` (`engine/crates/bnto/src/dry_run_hint.rs`) always ends the footer with `<input>`. For a recipe whose input node has `"mode": "none"`, the suggested `bnto run … <input>` fails once a value replaces the placeholder: `Error: This recipe takes no input; got 1 argument(s)`. `bnto info` already picks the ending by input mode (`run_hint` in `engine/crates/bnto/src/info.rs`: none, `<url>`, `<text>` or `<files...>`).

- **Fix:** end the footer by the recipe's input mode, as `run_hint` does, from one shared function.
- **Regression test:** a dry run of a `mode: "none"` recipe ends `Run with: bnto run <path>` with no placeholder; a file recipe ends `<files...>`.

### ISS-99: Every web page renders per request; recipe pages' `generateStaticParams` doesn't make them static

**Found:** October 27, 2026 (the v0.19.0 release's Lighthouse failure; see gotchas.md, "A `loading.tsx` Can Steal a Page's LCP").

`next build` lists every route but a few metadata files as `ƒ (Dynamic)`, `/[bnto]` included, though `app/(app)/[bnto]/page.tsx` sets `generateStaticParams` and `dynamicParams = false` and says "Pre-render all registered slugs at build time". So the SEO pages render on every request instead of being served as static HTML. The likely cause is the root layout's `ConvexAuthNextjsServerProvider` (`apps/web/app/layout.tsx:85`), which reads the auth cookie; not yet confirmed.

- **Fix:** find what opts the tree out of static rendering (`next build` with the provider moved off the public routes, or Next's dynamic-usage logging), and keep the public pages static. Then update gotchas.md's "A `loading.tsx` Can Steal a Page's LCP", which assumes pages render per request.
- **Regression test:** `next build` lists `/[bnto]` (and `/`, `/faq`, `/pricing`, `/privacy`) as static (`●` or `○`).

### ISS-100: `file-move`'s older refusals say "nothing moved" in a loop iteration after earlier ones moved files

**Found:** October 27, 2026 (cold review of the ISS-82 fix).

`move/plan.rs`'s `refused` prefixes every `file-move` refusal with "File move refused, nothing moved": a move that would replace a file (`refuse_conflicts`), a bad destination, a bad path. That's true of a flat batch. Inside a loop it describes only the refusing iteration: `loop { file-move into O }` over `b.txt`, then an `a.txt` whose destination `O/a.txt` already exists, moves `b.txt` in iteration 1 and then says "nothing moved". Reproduced with the `bnto` built from the ISS-82 branch, in a sandbox: `O/` ends with `a.txt` (the old one) and `b.txt`, and the run fails "File move refused, nothing moved: 1 conflict". The ISS-82 refusals say "this step moved nothing" instead, so the node now words the same situation two ways.

- **Fix:** say "this step moved nothing" in `refused`, or name the moves earlier iterations made, as `file-copy` names earlier copies (`copy_batch/refusal.rs`, `recorded_writes`); the second needs the run to record moves.
- **Regression test:** the loop above fails without claiming nothing moved, and `O/b.txt` holds `b.txt`'s content.

### ISS-102: The TUI footer shows a screen's usual hints during its search boxes, field editing and the editor's node picker

**Found:** October 27, 2026 (cold review of the ISS-101 fix).

Since ISS-101 the footer composes a mode's hints, the screen's and a base set (`tui/footer_hints.rs`, `decisions/tui-footer-hints.md`). Modes that take every key hide the base `q quit` (`keys::captures_keys`), but only some have their own hints: the editor's and library's questions, a library rename, consent, a run in progress, the settings picker. In the rest (a search box on Browser, Picker or Library; editing a field on Editor, Detail or Wizard; the editor's node picker) the footer still lists the screen's usual keys (`↑↓ navigate  / search  Enter select  r rename…`), though typing goes into the box. Two more: the settings picker keeps its folder hints (`h/l`, `.`, Tab) while its search box is open, and a finished or failed run still says `Esc cancel`, where Esc goes back to the recipe (`app_helpers/home_detail.rs`).

- **Fix:** a `mode_hints` branch for each, from its key handler (a search: `Enter done  Esc clear`; a field: `Enter save  Esc cancel`).
- **Regression test:** `footer_hints.rs` cases for a browser search and an editor field show only their keys.
