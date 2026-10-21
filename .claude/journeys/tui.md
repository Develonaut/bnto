# TUI — User Journey Test Matrix

**Domain:** `bnto tui`, the terminal UI. Covers home, library, recipe detail, file picking, consent, live execution and results.
**Status:** Snapshots (#681) and journeys TJ1–TJ3, TJ5 and TJ6 run on every PR in the CI Gate. The other journeys are planned.
**Last Updated:** October 21, 2026

---

## Why This Matters

Many people use bnto through the TUI. Its bugs are visual and only show at certain terminal sizes or mid-run: a list that clips, a bar that overlaps, output that vanishes. Unit tests on the model can't see them. These journeys check what the user actually sees, like the web's Playwright journeys.

## Two layers

| Layer            | What                                                                                  | Where                                                                         | Run                                                                                |
| ---------------- | ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Screen snapshots | Each screen rendered in-process at 120x36 and 80x24, stored as `insta` text           | `engine/crates/bnto/src/tui/screen_snapshots.rs`, `tui/snapshots/`            | `task tui:snapshots` (update: `task tui:snapshots:update`, review with `git diff`) |
| Journeys         | The real `bnto tui` in a pseudo-terminal, driven by keys, read through a VT100 screen | `engine/crates/bnto/tests/tui_journeys.rs`, harness in `tests/tui/harness.rs` | `task tui:test` (both layers)                                                      |

Every journey runs with temp `HOME`, `BNTO_HOME` and config dirs, and never touches a real `~/.bnto`. Waits poll for text and fail with the screen the user saw. Wait for _every_ part you assert on, because a read can land mid-draw.

## Gate Map

| Step     | What Happens                                      | Gate                         | Error Behavior                     |
| -------- | ------------------------------------------------- | ---------------------------- | ---------------------------------- |
| Launch   | Raw mode, alternate screen, first frame           | `tui::run`                   | Terminal restored on exit or panic |
| Navigate | Keys become `AppMessage`s, then `update`          | `keys::handle_key`           | Unmapped keys are ignored          |
| Detail   | Recipe fields form, input picker, Run             | `screens::detail`, tonkotsu  | Field validation inline            |
| Consent  | Untrusted shell recipes show every command        | `trust_gate::consent_needed` | `n`/Esc cancels and nothing runs   |
| Execute  | Pipeline on a bridge thread, events to the screen | `tui::bridge`                | `PipelineFailed` → failed state    |
| Results  | Outputs, sizes, output folder                     | `screens::results`           | —                                  |

## Journey Matrix

### Navigation and layout

| #   | Journey                       | Steps                                     | Pass Criteria                                                                       | Test                                                                       |
| --- | ----------------------------- | ----------------------------------------- | ----------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| TJ1 | Home loads                    | Launch at 120x36                          | Library and Recipes panes, and the status bar with recipe count and `v<version>`    | `tj1_home_shows_the_catalog_and_version`                                   |
| TJ2 | Reach a recipe below the fold | Tab to Recipes, Down to Keep Awake, Enter | The pane scrolls so the cursor stays visible, and the detail shows the Action field | `tj2_scroll_to_keep_awake_and_open_it`                                     |
| TJ7 | Narrow terminal               | Launch at 80x24, visit each screen        | Nothing clipped mid-word; the bottom-bar hints and status never overlap             | Snapshots + `the_bottom_bar_status_is_never_overwritten` (planned journey) |
| TJ8 | Settings and theme            | Open Settings, change theme               | Theme applies immediately and persists in `BNTO_HOME/config.toml`                   | Planned                                                                    |

### Running recipes

| #    | Journey                             | Steps                                                               | Pass Criteria                                                                                                                                                      | Test                                                    |
| ---- | ----------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| TJ3  | Compress an image                   | Select a fixture image, Run                                         | The results screen lists `<name>-compressed.jpg`; output under `BNTO_HOME/output`                                                                                  | `tj3_compress_an_image_and_see_the_result`              |
| TJ4  | Multi-step loop shows live progress | Run the explicit-loop compress-and-rename fixture on several images | During the run, `loop (n/N)` advances, every child step is listed from the start, and file rows show names and a distinct done marker. Results are in input order. | Planned (audit R5–R8)                                   |
| TJ5  | Shell output streams and stays      | Run a shell recipe that prints a line every 0.4s; approve consent   | Consent lists the full script. Lines appear while it runs (line 2 before line 5), and all lines are still readable on the results screen.                          | `tj5_shell_output_streams_live_and_stays_after_the_run` |
| TJ6  | A message recipe shows its message  | keep-awake with Action = Check status                               | The results screen shows "No active session."                                                                                                                      | `tj6_keep_awake_status_shows_its_message`               |
| TJ9  | A failing step                      | Run a recipe whose command exits non-zero                           | Failed state names the step, the error and the last output lines; Esc returns to the detail                                                                        | Planned                                                 |
| TJ10 | Upgrade notice                      | Launch with a seeded newer-version cache                            | Status line shows `bnto X available, run bnto upgrade`                                                                                                             | With [cli-upgrade.md](../strategy/cli-upgrade.md) PR 3  |

## Known gaps

See the run output audit in [tui-testing.md](../strategy/tui-testing.md#run-output-audit-october-21-2026) (R1–R9). Journeys TJ5 and TJ6 covered R2 and R1 (fixed); TJ4 is the RED test for R5–R8. Windows journeys are `#[ignore]`d until ConPTY runs prove stable; the snapshots already run on Windows.
