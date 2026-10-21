# CLI — User Journey Test Matrix

**Domain:** `bnto run` and `bnto check` on recipes with conditional steps (`when` guards).
**Status:** CJ1, CJ3, CJ6 and CJ8 run on every PR (Linux, macOS and Windows CI). The others land with the PRs named in the table.
**Last Updated:** October 21, 2026

---

## Why This Matters

A guard decides whether a step runs on this machine, for this file, with these fields. When it goes wrong, a user's files are changed on the wrong OS, or a step quietly never happens. The engine's unit tests (`bnto-core/src/executor/tests/when.rs`) prove the rules. These journeys prove what a user sees from the built binary.

Every journey runs the real `bnto` with temp `HOME` and `BNTO_HOME`, and never touches a real `~/.bnto`. Per-OS journeys work out the expected branch from `cfg!`, so one test passes on each CI OS.

## Gate Map

| Step       | What Happens                                        | Gate                                 | Error Behavior                        |
| ---------- | --------------------------------------------------- | ------------------------------------ | ------------------------------------- |
| Check      | Unknown op, bad placeholder, `when` on input/output | `bnto-core/src/check/when.rs`        | A finding; `Check: FAIL`, exit 1      |
| Consent    | Every shell command listed, guarded or not          | `bnto/src/consent_lines.rs` (SEC-16) | `n` cancels and nothing runs          |
| Pre-flight | Raw param checks walk every branch                  | `executor/raw_params.rs`             | The run is refused before any step    |
| Guard      | `value`/`expected` resolved, compared               | `executor/guard.rs`                  | Unknown op fails the run at that node |
| Skip       | `NodeSkipped`, input files passed on, no record     | `guard::skip`                        | —                                     |

## Journey Matrix

| #   | Journey                                    | Steps                                                                       | Pass Criteria                                                         | Test / PR                                                        |
| --- | ------------------------------------------ | --------------------------------------------------------------------------- | --------------------------------------------------------------------- | ---------------------------------------------------------------- |
| CJ1 | A guard runs or skips its step             | `bnto run` a rename guarded on `{{ctx.platform}}`, holding and not          | Holding: the file is renamed. Not: the file is written untouched      | `cj1_*` in `tests/when_guard.rs`. PR 4 adds the "skipped" line   |
| CJ2 | If/else on a field, flipped with `--param` | Two steps with opposite guards on one field; run, then `-p field=other`     | Exactly one branch's change each time                                 | PR 5 (needs container-scoped fields)                             |
| CJ3 | Per-OS groups                              | Three groups guarded on `{{ctx.platform}}` = macos / linux / windows        | Only the host's branch changes the file                               | `cj3_only_the_hosts_group_runs`                                  |
| CJ4 | No branch for the host                     | `bnto run` a recipe whose branches all miss this OS                         | Stops before any step runs, naming the OSes it runs on                | PR 6                                                             |
| CJ5 | Consent lists guarded shell steps          | Untrusted recipe with guarded shell steps                                   | Every command is listed, guarded or not (now); with its guard (PR 4)  | `consent_lines.rs` unit test now; journey in PR 4                |
| CJ6 | `bnto check` refuses bad guards            | A recipe with an unknown op and `when` on its input node                    | One finding each, naming node and problem; `Check: FAIL (2 findings)` | `cj6_check_refuses_an_unknown_op_and_a_guard_on_an_io_node`      |
| CJ7 | `--dry-run` shows what this host skips     | `bnto run --dry-run` a per-OS recipe                                        | The other OSes' steps are listed as skipped                           | PR 4                                                             |
| CJ8 | A guard on each file in a loop             | `file-inspect`, then a loop whose rename is guarded on `{{item.extension}}` | Only matching files are renamed; the rest pass through untouched      | `cj8_a_guard_on_item_handles_matching_files_and_passes_the_rest` |
