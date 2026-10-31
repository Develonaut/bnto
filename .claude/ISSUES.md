# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** from the next batch in the [Work order](#work-order), by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-111**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

## Work order

Agreed with Ryan on October 30, 2026, replacing the October 27 order's batches 5–9. Take one batch at a time, one PR per issue (or per tightly coupled pair), and finish a batch before starting the next. When an issue is fixed or a new one is logged, update this list in the same PR.

Done, October 27, 2026: batch 1, engine file operations (ISS-83 #803, ISS-87 #804, ISS-82 #805); batch 2, TUI correctness (ISS-98 #806, ISS-66 #807, ISS-97 #808, ISS-101 #809, ISS-102 #810). Done, October 29, 2026: batch 3, checks before a run starts (ISS-75 #811, ISS-74 #812, ISS-96 #813, ISS-79 #814, ISS-73 #815); batch 4, engine file nodes (ISS-17 #816, ISS-34 #817, ISS-33 #818, ISS-100 #819, ISS-107 #820). Done, October 30, 2026: ISS-58, and ISS-35 for five nodes (#821). Done, October 31, 2026: ISS-109 (#823). Batches keep their numbers.

Batches group issues that share code and verification, so each runs while its build and test setup is warm: the engine batches (5–7) first, then the web batches (8–10; batch 10's ISS-12 is the CLI's telemetry), then codegen and tooling. Within that, silent wrong results come first and hygiene last.

| #   | Batch                    | Issues, in PR order                                         | Why                                                                                                                                                                                                                                                                                                                                                                                                                              |
| --- | ------------------------ | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 5   | Loop semantics           | ISS-110, ISS-108                                            | A `when` guard, and a batch step the executor fills, read only the loop's item inside an iteration (ISS-110); ISS-109's fix makes per-file steps read the file's current item, and these should too. A loop's failure line doesn't say which iteration failed (ISS-108). All in `bnto-core`'s executor                                                                                                                           |
| 6   | Template unification     | ISS-35                                                      | `file-rename` and `file-decode` move onto the shared one-pass fill. These are the first PRs of the template-filters plan ([PLAN.md backlog](PLAN.md#backlog)), which must unify the interpolators before adding filters; write that plan first                                                                                                                                                                                   |
| 7   | TUI and CLI behavior     | ISS-103, ISS-104, ISS-45                                    | The node picker swallowing `j`/`k` (ISS-103); a file param shown as a text box (ISS-104); progressive writes in message/none mode (ISS-45). `tui:test` and `cli:test`                                                                                                                                                                                                                                                            |
| 8   | Web performance          | ISS-99, then ISS-37                                         | Public pages render per request, which costs compute that static pages don't (ISS-99), then the homepage's ~500 KB of JS (mobile LCP 4.9 s, ISS-37). Moved up from last: first of the web batches. Its own plan, profiling first; ISS-99 first because it changes what ISS-37 measures                                                                                                                                           |
| 9   | Web cleanups             | ISS-86, ISS-91, ISS-88, ISS-90, ISS-84, then ISS-85, ISS-44 | Prettier first, so later PRs don't fight the formatter (ISS-86); `React.*` types (ISS-91); dot-notation names (ISS-88); dead `@bnto/form` exports (ISS-90); the E2E heading check, then the helpers split in the same file (ISS-84, ISS-85); old params in the browser, which also moves the migrations out of the CLI crate (ISS-44). `ui:lint`, `task e2e`, `seo:audit`; ISS-44 also needs the engine's tests and a WASM build |
| 10  | Telemetry and analytics  | ISS-12, then ISS-7 (same release), ISS-57, ISS-29, ISS-70   | Released CLIs send nothing (ISS-12) and the notice links a 404 (ISS-7); a wrong `out_of_memory` category (ISS-57); exception capture off (ISS-29); recipe-page downloads not recorded (ISS-70)                                                                                                                                                                                                                                   |
| 11  | Codegen and catalog      | ISS-105 + ISS-94 (one PR), ISS-93, ISS-106                  | Codegen leaving stale files behind (ISS-105, ISS-94); a stale `@bnto/nodes` README (ISS-93); `platforms` leaving out `cli` (ISS-106). `task nodes:generate`                                                                                                                                                                                                                                                                      |
| 12  | Tooling and test hygiene | ISS-92, ISS-95, ISS-89, ISS-67, ISS-18, ISS-8 (two PRs)     | `scripts/` never type-checked (ISS-92); a misleading crate check (ISS-95); a `seo:audit` hint (ISS-89); a pty lock (ISS-67); a slow Windows test (ISS-18); CLI tests touching the real `~/.bnto` ([open-issues-plan.md](strategy/open-issues-plan.md) PRs 5–6, ISS-8)                                                                                                                                                            |

After batch 6, the rest of the template-filters plan (the filters themselves, a feature rather than a defect) can run straight on or after batch 12; Ryan decides when.

**Waiting on Ryan:**

- ISS-31: move the other app (Darkmatter) to a PostHog project of its own. That's outside this repo; the issue closes once the review's `$host` breakdown shows only bnto.io.

## Open issues

| ID      | Issue                                                                                                                   | Area        | Status                                           |
| ------- | ----------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------ |
| ISS-7   | The CLI's telemetry notice links `bnto.io/telemetry`, a 404                                                             | web docs    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-8   | Most CLI integration tests run `bnto` against the real `~/.bnto`                                                        | cli test    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-12  | Released CLIs send no telemetry: the PostHog key is never set                                                           | cli         | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-18  | A Windows process test times out waiting 20 s for PowerShell                                                            | engine test | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-29  | Errors on bnto.io go unseen: exception capture is off                                                                   | core        | Open                                             |
| ISS-31  | Another app's events land in bnto.io's PostHog project                                                                  | analytics   | Open                                             |
| ISS-35  | `file-rename` and `file-decode` fill their own tokens over values the executor filled                                   | engine      | Open                                             |
| ISS-37  | The homepage loads ~500 KB of JS that competes with its CSS                                                             | web perf    | Open                                             |
| ISS-44  | In the browser, a renamed or removed param's error says to run `bnto migrate`                                           | web         | Open                                             |
| ISS-45  | Progressive loop writes in message/none mode land in the folder unlisted                                                | cli tui     | Open                                             |
| ISS-57  | `errorCategory` reports `out_of_memory` for any message containing `oom`                                                | core        | Open                                             |
| ISS-67  | `update_notice.rs` opens its ptys in parallel with no lock                                                              | cli test    | Open                                             |
| ISS-70  | Recipe pages record no download a user clicks for                                                                       | web         | Open                                             |
| ISS-84  | E2E specs pass `navigateToRecipe` a page heading it ignores                                                             | web test    | Open                                             |
| ISS-85  | `e2e/helpers.ts` is a 16-export grab bag the grab-bag check skips                                                       | web test    | Open                                             |
| ISS-86  | Nothing checks Prettier formatting in CI or `task check`; 54 TS files fail it                                           | tooling     | Open                                             |
| ISS-88  | `FileUpload` context errors and two web comments name dot-notation components                                           | ui          | Open                                             |
| ISS-89  | In a fresh worktree, `task seo:audit` fails at the first URL with no hint: there's no `.env.local`                      | tooling     | Open                                             |
| ISS-90  | `@bnto/form` exports `FormStoreProvider` and four hooks nothing outside it uses                                         | form        | Open                                             |
| ISS-91  | 28 TS files use `React.*` namespace types the import rule bans                                                          | web         | Open                                             |
| ISS-92  | Nothing type-checks the `scripts/` folders: `tsx` runs them unchecked                                                   | tooling     | Open                                             |
| ISS-93  | `@bnto/nodes`'s README lists `src/` files and folders that don't exist                                                  | docs        | Open                                             |
| ISS-94  | Codegen never removes a stale `iterationModes.ts` from `src/generated/catalog/`                                         | tooling     | Open                                             |
| ISS-95  | The own-version check reports a failed crate download as a changed crate                                                | tooling     | Open                                             |
| ISS-99  | Every web page renders per request; recipe pages' `generateStaticParams` doesn't make them static                       | web perf    | Open                                             |
| ISS-103 | The TUI editor's node picker can't filter by `j` or `k`: they move the cursor                                           | tui         | Open                                             |
| ISS-104 | The TUI's recipe form shows a file param (Watermark Images' overlay) as a text box, not a file picker                   | tui         | Open                                             |
| ISS-105 | Node codegen never deletes a removed node's doc: `packages/@bnto/nodes/docs/video-download.md` is left over             | codegen     | Open                                             |
| ISS-106 | The catalog's `platforms` leaves out `cli` for 13 nodes the CLI runs, and their generated docs say "Platforms: browser" | engine      | Open                                             |
| ISS-108 | A loop's failure line repeats its step's "this step moved nothing", though the loop moved files in earlier iterations   | engine      | Open                                             |
| ISS-110 | Inside a loop, a `when` guard and an executor-filled batch step read only the loop's item                               | engine      | Open                                             |

---

### ISS-7: The CLI's telemetry notice links `bnto.io/telemetry`, a 404

**Found:** October 23, 2026 (fixing ISS-2).

The first-run notice (`engine/crates/bnto/src/telemetry/consent.rs:27`) says bnto "collects anonymous usage data" and sends people to `https://bnto.io/telemetry` for more. That URL renders the `[bnto]` route's "Page Not Found" (`curl -sL https://www.bnto.io/telemetry | grep -o not-found-heading`), and no page says what the CLI collects: `privacy/privacy.mdx` lists PostHog without mentioning the CLI, and the CLI recipe FAQ (`apps/web/lib/cliRecipeFaq.ts:9`) has one line. Add a `/telemetry` page listing the four events (`cli_command`, `cli_recipe_run`, `cli_error`, `cli_tui_session`), their properties (bundled slugs only, else `"custom"`; bucketed counts, sizes and durations), the anonymous id and the three opt-outs, or point the notice at a page that has them.

- **Decided (Ryan, October 30, 2026):** ships after ISS-12, in the same release, so the page says telemetry is on and that PostHog discards IPs. Plan: [open-issues-plan.md](strategy/open-issues-plan.md) PR 8.

### ISS-8: Most CLI integration tests run `bnto` against the real `~/.bnto`

**Found:** October 23, 2026 (fixing ISS-3).

`cli_commands.rs` and the run helpers in `tests/helpers/mod.rs` now use `helpers::Sandbox` (temp HOME, BNTO_HOME, config dir and working directory, telemetry off). About 17 other files in `engine/crates/bnto/tests/` still spawn `Command::new(bnto_bin())` with no HOME or BNTO_HOME: `assert_files`, `check_edges`, `check_folders`, `check_run_time`, `edges`, `edit_fields`, `file_delete`, `file_inspect`, `file_join`, `progressive_output`, `report_file_names`, `run_args`, `run_json`, `run_temp_files`, `shell_each`, `shell_options`, and some runs in `dry_run_temp_files` and `run_timeout`. Each run writes a session log into the developer's `~/.bnto/state/logs`: `cargo test -p bnto --test edges --test run_json` added 3. A run that resolves a slug would also read their library. Switch them to `Sandbox::new().command()`.

### ISS-12: Released CLIs send no telemetry: the PostHog key is never set

**Found:** October 24, 2026 (planning ISS-7).

`engine/crates/bnto/src/telemetry/mod.rs:57` reads the key from the `BNTO_POSTHOG_KEY` env var at run time, and telemetry is inactive without one (`:60`). Nothing sets it: there's no `option_env!`, no build script, and nothing in `release.yml` (`grep -rn BNTO_POSTHOG_KEY` finds only `mod.rs` and a test). So shipped CLIs collect nothing, while the first-run notice says bnto "collects anonymous usage data" and the code comment calls the key "safe to embed". `bnto telemetry status` also reports "Effective: true" without checking for a key (`mod.rs:134`).

- **Decided (Ryan, October 26 and 30, 2026):** turn it on, with the key as a constant in the source, so `cargo install` builds send too, and `BNTO_POSTHOG_KEY` kept as a runtime override. PostHog project keys are public and write-only; the web bundle already ships one. `status` says when there's no key. Plan: [open-issues-plan.md](strategy/open-issues-plan.md) PR 7.

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
- **Decided (Ryan, October 30, 2026):** turn on exception autocapture with a `before_send` that removes message text, since an engine error can name the user's files (ISS-57's `bedroom.jpg`); keep the type and stack.
- **Regression test:** a unit test on the PostHog adapter's init options that exception capture is on, and a check that every run emits exactly one of `recipe_run_completed` / `recipe_run_failed`.

### ISS-31: Another app's events land in bnto.io's PostHog project

**Found:** October 24, 2026 (PostHog review).

- **What PostHog shows:** 190 events (`$pageview`, `$web_vitals` and a custom event) from three hosts outside bnto.io, June 14 to October 7, 2026, arriving through a different `/ingest` proxy. The project's test-account filters exclude only localhost, an internal cohort and one IP, so these count in web analytics.
- **Fix:** this is a PostHog config fix, not a repo change. Either give that app its own project key, or add a `$host` filter (`^(www\.)?bnto\.io$`) to the test-account filters.
- **Decided (Ryan, October 30, 2026):** move the other app (Darkmatter, which shares the project) to a project of its own. A `$host` test-account filter would also hide Darkmatter's events from its own filtered views.
- **No repo test:** it's project config. After the fix, rerun the review's `$host` breakdown to confirm only bnto.io remains.

### ISS-35: `file-rename` and `file-decode` fill their own tokens over values the executor filled

**Found:** October 24, 2026 (cold review of the ISS-24 fix). Narrowed October 29, 2026: `report`, `assert`, `edit-fields`, `file-render` and `file-copy` now fill every token in one pass, and `file-rename`'s pattern and `file-decode`'s `filename` fill their own tokens in one pass ([batch-node-loop-items.md](decisions/batch-node-loop-items.md)).

`file-rename` and `file-decode` still get params the executor filled (`{{item.*}}`, `{{fields.*}}`, `{{env.*}}`, `{{node.*}}`), then fill their own tokens (`{{name}}`, `{{ext}}`, `{{index}}`, `{{counter}}`, `{{date}}`) over the result. Reproduced with the CLI: a loop over CSV rows (`title` = `{{name}}`) of `file-rename` with `pattern: "{{item.title}}.txt"` writes `people_row0.txt`, not `{{name}}.txt`. Only `shell-run` declares `reserved_tokens` (`bnto-shell/src/run.rs:104`), which SEC-27's `check_reserved` uses to refuse exactly this.

- **Fix:** have both nodes take their params unfilled and fill every token through `bnto_core::control::placeholder::Scope` with their own tokens added (`resolves_loop_items`, as the five batch nodes do), or declare their tokens in `reserved_tokens`. File-rename is a batch node; file-decode is per file, so it would need the executor to hand a per-file node its params unfilled too.
- **Regression test:** a `{{item.*}}` value holding `{{name}}` in `file-rename`'s pattern and `file-decode`'s `filename` comes out as written.

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

- **Options:**
  - **(a) Migrate in the browser:** run the same migrations when a recipe is opened or imported (they're pure JSON rewrites), so old recipes upgrade silently.
  - **(b) Ignore removed params:** pre-flight skips `REMOVED_PARAMS` (they did nothing by definition), while `bnto check` still warns and points at `bnto migrate`. This departs from "undeclared params fail loudly" for removals only.
- **Decided (Ryan, October 30, 2026):** (a), the engine's own migrations, so there's one implementation and renames are covered too. They live in the CLI crate (`bnto/src/migrate/`), which `bnto-wasm` doesn't depend on, so the fix first moves them into `bnto-engine` (or `bnto-core`), then exports them through WASM for the editor to call on open and import. Needs the engine's tests and a WASM build besides the web checks.
- **Regression test:** an old recipe with `precision` opened in the web editor runs, with no CLI step.

### ISS-45: Progressive loop writes in message/none mode land in the folder unlisted

**Found:** October 24, 2026 (second cold review of the ISS-21 fix; reproduced with the CLI, temp `HOME` and `BNTO_HOME`).

A loop with `outputPersistence: "progressive"` copies each iteration's files into the output node's `directory` even when the output is in `message` or `none` mode, which promise to write nothing: `file-list-loop-progressive.bnto.json` with `mode: "message"` and a `directory` left `a.txt` and `b.txt` there. `bnto run` (`progressive_dir`, `run.rs:230`) and the TUI both do it. Outside preview and overwrite mode, the TUI bridge's reporter copies each finished iteration's files on the pipeline's thread, into the folder that `resolve_output_dir` (`tui/bridge.rs:215`) resolves and creates: see `progressive_dir` and `channel_reporter`, `tui/bridge/reporter.rs:18-44`. Since the ISS-21 fix (#750), the TUI's results list doesn't show those files. In write mode, an iteration file that a later step renames also stays on disk under its old name.

- **Decided (Ryan, October 30, 2026):** progressive persistence happens only in write mode. Message and none mode promise to write nothing (the other option was always, with those files listed).
- **Fix:** skip progressive writes outside write mode, in `bnto run` and the TUI bridge alike.
- **Regression test:** a progressive loop in message mode with a `directory` leaves the folder untouched, in `bnto run` and the TUI bridge.

### ISS-57: `errorCategory` reports `out_of_memory` for any message containing `oom`

**Found:** October 25, 2026 (fixing ISS-27, the error categories).

`categorizeError` (`packages/core/src/lib/categorizeError.ts`) matches `/out of memory|OOM/i`: case-insensitive, with no word boundary. Any message with "oom" in it (`room`, `zoom`, `bloom`) is reported as `out_of_memory`, and the pattern runs before the `load failed`, `file_unreadable` and `decode_failed` patterns, so it can take their messages too. Some engine messages carry a file name (`engine/crates/bnto-core/src/executor/primitive.rs:148`), so a run on `bedroom.jpg` that fails with `Invalid input: Node 'rename': {{item.name}} has no value for bedroom.jpg (it has no item values)` is reported as `out_of_memory` instead of `unknown`.

- **Fix:** match `OOM` only as a whole, upper-case word (`/out of memory/i` or `/\bOOM\b/`).
- **Regression test:** the message above isn't `out_of_memory`, and neither is `"zoom"` or `"Classroom"`, while `"out of memory"` and `"OOM"` still are.

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

### ISS-84: E2E specs pass `navigateToRecipe` a page heading it ignores

**Found:** October 25, 2026 (fixing ISS-69, linting the E2E specs).

`navigateToRecipe(page, slug, _h1?)` (`apps/web/e2e/helpers.ts:45`) waits for the `recipe-heading` test ID and never reads `_h1`. It checked the heading's text until #208 standardized selectors on `getByTestId`, which kept the parameter. 64 calls in 18 specs still pass a heading: 62 as text (`navigateToRecipe(page, "clean-csv", "Clean CSV Online Free")`) and 2 as `recipe.h1` (`pipeline-smoke.spec.ts:72`, `recipe-switching.spec.ts:110`). Each reads as if it checks the page title, and a page that renders the wrong heading passes.

- **Fix:** either assert the heading text again (`expect(page.getByTestId("recipe-heading")).toHaveText(h1)`), or drop the parameter and the 64 arguments. Default (October 30, 2026): assert it again.
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

### ISS-99: Every web page renders per request; recipe pages' `generateStaticParams` doesn't make them static

**Found:** October 27, 2026 (the v0.19.0 release's Lighthouse failure; see gotchas.md, "A `loading.tsx` Can Steal a Page's LCP").

`next build` lists every route but a few metadata files as `ƒ (Dynamic)`, `/[bnto]` included, though `app/(app)/[bnto]/page.tsx` sets `generateStaticParams` and `dynamicParams = false` and says "Pre-render all registered slugs at build time". So the SEO pages render on every request instead of being served as static HTML. The likely cause is the root layout's `ConvexAuthNextjsServerProvider` (`apps/web/app/layout.tsx:85`), which reads the auth cookie; not yet confirmed.

- **Fix:** find what opts the tree out of static rendering (`next build` with the provider moved off the public routes, or Next's dynamic-usage logging), and keep the public pages static. Then update gotchas.md's "A `loading.tsx` Can Steal a Page's LCP", which assumes pages render per request.
- **Regression test:** `next build` lists `/[bnto]` (and `/`, `/faq`, `/pricing`, `/privacy`) as static (`●` or `○`).

### ISS-103: The TUI editor's node picker can't filter by `j` or `k`: they move the cursor

**Found:** October 27, 2026 (cold review of the ISS-102 fix).

In the editor's add-a-node picker, typing filters the node list (`PickerInput`). But `keys/editor.rs`'s picker branch matches `KeyCode::Down | KeyCode::Char('j')` and `KeyCode::Up | KeyCode::Char('k')` before `KeyCode::Char(ch)`, so `j` and `k` move the cursor and never reach the filter: a search for "json" or "mkdir" can't be typed.

- **Fix:** in the picker, only the arrow keys move the cursor; every character goes to the filter.
- **Regression test:** with the picker open, `j` adds "j" to the filter and Down moves the cursor.

### ISS-104: The TUI's recipe form shows a file param (Watermark Images' overlay) as a text box, not a file picker

**Found:** October 27, 2026 (second cold review of the ISS-75 fix).

Since ISS-75, a file param's value is a path the CLI reads, and the TUI's Detail form reaches the same overrides (`tui/bridge/precheck.rs`), so typing a path into Overlay Image works. But `param_to_field` in `tui/screens/detail_bridge.rs` has no `ParameterType::File` arm: it falls to the `_ => tonkotsu::text(…)` fallback (line 77), so the user types a path by hand, relative to the folder `bnto tui` started in. tonkotsu has a file browser field (`field_builder.rs:231`, `file_path()`).

- **Fix:** build a `ParameterType::File` param with `tonkotsu::file_path`, filtered to its `accept` types if the browser supports that, and give the run the chosen absolute path.
- **Regression test:** a `detail_bridge` test that the overlay param becomes a file path field, and a snapshot of the Watermark Images form.

### ISS-105: Node codegen never deletes a removed node's doc

**Found:** October 29, 2026 (fixing ISS-17, pointing `/workflow-expert` at the generated node docs).

`task nodes:generate` writes `packages/@bnto/nodes/docs/<type>.md` for every node type in the catalog (`packages/@bnto/nodes/scripts/codegen/docs/writeNodeReadmes.ts`), but never removes one whose type is gone: unlike the per-processor folders (`scripts/codegen/writePerItemFolder.ts`), it doesn't call `cleanStaleFiles`. `docs/video-download.md` has been left over since #444 removed the `video-download` processor (download-video became a `shell-run` recipe), so the folder lists 40 docs for 39 node types, and a reader following the docs finds a node bnto doesn't have.

- **Fix:** call `cleanStaleFiles(DOCS_DIR, …)` with the expected `<type>.md` names in `writeNodeReadmes`, and regenerate (which deletes `video-download.md`).
- **Regression test:** a codegen test that a doc for a type not in the catalog is removed, or a check that `docs/` holds exactly one file per catalog node type.

### ISS-106: The catalog's `platforms` leaves out `cli` for nodes the CLI runs

**Found:** October 29, 2026 (cold review of the ISS-17 fix).

Every processor runs in the CLI, which registers them all (`compress-images` runs there). But 13 processors declare `platforms: ["browser"]` alone, among them every image node, `file-rename` and the CSV nodes (`engine/catalog.snapshot.json`), while others list `["browser", "cli", "desktop"]`. The generated node docs print the field as is, so `packages/@bnto/nodes/docs/image-compress.md` says "**Platforms:** browser" and a reader takes it for a browser-only node. `vector-optimize` says `["browser", "native"]`, a value no other node uses.

- **Fix:** decide what `platforms` means (everywhere a node runs, or only whether it runs in the browser, which is what `metadata.rs`'s browser filter reads), then make every processor's list say it, and the docs with it. Default (October 30, 2026): everywhere a node runs.
- **Regression test:** a catalog test that every processor lists `cli` (or whatever the agreed values are), and no value outside the agreed set.

### ISS-108: A loop's failure line repeats its step's "this step moved nothing" after earlier iterations moved files

**Found:** October 29, 2026 (cold review of the ISS-100 fix).

Since ISS-100 every `file-move` refusal says "this step moved nothing", true of the refusing iteration. But a container passes a child's error up as its own: the CLI prints `FAILED mv: File move refused, this step moved nothing: 1 conflict`, then `FAILED each: …` and `Pipeline failed: …` with the same text, and the loop `each` did move `b.txt` in its first iteration (`Moved 1 file` shows just above). Both iterations also show as `[1/1] file-move`, so nothing marks which iteration refused. `file-copy`'s "nothing more copied" plus its list of earlier copies avoids this.

- **Fix:** have a container's failure line say which iteration failed (`each, iteration 2 of 2: …`), or have `file-move` list the moves earlier iterations made, as `file-copy` lists earlier copies (the run would record moves). Default (October 30, 2026): name the iteration.
- **Regression test:** the ISS-100 loop's output names the failing iteration, or lists `b.txt` as moved earlier.

### ISS-110: Inside a loop, a `when` guard can't read a key added in the iteration

**Found:** October 29, 2026 (`/workflow-expert` review of the ISS-58 fix).

A `when` guard is decided against the loop's item only (`executor/guard.rs`, `skip_reason`: `loop_item: &ctx.loop_item`), not the file's metadata. Inside a loop, a guard on a key that an earlier step of the iteration added (`file-inspect`'s `extension`, an `edit-fields` value) sees no value, though a per-file step's params read it. Reproduced with the CLI: a loop over CSV rows of `edit-fields` setting `x: "y"`, then `file-rename` guarded on `{{item.x}}` equals `y`, prints `file-rename skipped: when {{item.x}} equals y: got "{{item.x}}", expected "y"`. `when_guard.rs` CJ8 runs `file-inspect` before the loop, so it doesn't show this.

- **Also:** a batch step the executor fills (`summarize`, `file-join`, `file-sort`, `file-list`, `file-move`, `file-delete`, `spreadsheet-merge`, `spreadsheet-export`, and `shell-run` outside `each`) gets its `{{item.*}}` from the loop's item alone too (`resolve_node_params`), so it misses a key a step changed, though ISS-109 made per-file steps read it.
- **Fix:** one rule for anything decided once per step, guards and these batch fills: the item a per-file step would read, from the files the step gets: one file's item, the loop's item with none, and with several the value they agree on, refusing files that disagree ([loop-item-current.md](decisions/loop-item-current.md)). Split from ISS-109 so each PR has one concern.
- **Regression test:** a loop of `file-inspect` then a step guarded on `{{item.extension}}` runs for the matching file.
