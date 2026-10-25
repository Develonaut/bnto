# TUI Snapshots + Journey Tests -- 3 PRs

**Created:** October 21, 2026
**Status:** Approved, October 21, 2026; next in the queue (before the [CLI upgrade notices](cli-upgrade.md), then keep-awake PR 3). PRs 1 (#681) and 2 (#682) merged; PR 3 in review.
**Related:** [tui-strategy.md](tui-strategy.md) (it already calls for golden snapshot tests for rendering), [tui-user-journey.md](tui-user-journey.md), [journeys/web.md](../journeys/web.md), [pre-commit.md](../rules/pre-commit.md)

## Context

The web app has Playwright journeys and screenshot snapshots. The TUI has only unit tests on the model and a few `TestBackend` render checks, so a layout bug can ship unseen. On October 21 the home Recipes pane clipped everything past Standardize CSV (fixed in #680). It surfaced only because Claude drove the real TUI through tmux by hand.

Goal: the TUI gets the same two layers the web has. Agents can then validate a TUI change themselves by running one command, instead of a hand-driven tmux session.

| Web (today)                                       | TUI (proposed)                                                                                                                                                                     |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Page-level screenshots (`toHaveScreenshot`)       | **Screen snapshots:** each screen rendered in-process to ratatui's `TestBackend` at fixed sizes and stored as text with `insta`                                                    |
| User-journey specs (`journeys/browser/*.spec.ts`) | **Journey tests:** the real `bnto tui` binary in a pseudo-terminal (PTY), driven by keystrokes, with the screen read back through a terminal emulator and asserted and snapshotted |
| `.claude/journeys/web.md` matrix                  | `.claude/journeys/tui.md` matrix (TJ1, TJ2, …)                                                                                                                                     |

**What changes:** test infrastructure and dev-dependencies only (`insta`, `portable-pty`, `vt100`), plus a `task tui:test` entry point and a journey matrix. **What does NOT change:** the TUI's behaviour and release binaries. Dev-dependencies aren't compiled into `cargo install bnto`.

## Options considered

| Option                                                                                                                        | Verdict                                                                                                                                                                                                                                                                                   |
| ----------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`TestBackend` + `insta`** for screen snapshots                                                                              | **Use.** It's the official Ratatui recipe: deterministic, millisecond-fast, the same on Linux, macOS and Windows CI, and snapshot diffs are plain text that reviews well in a PR. It can't catch real-terminal problems (key decoding, resize, raw mode), which is what journeys are for. |
| **PTY + `vt100`, own thin harness** (`portable-pty` from WezTerm, ConPTY on Windows; `vt100` turns output into a screen grid) | **Use for journeys.** It exercises the real binary as users run it. Both crates are mature. The harness is small: spawn, send keys, wait for text, read the screen.                                                                                                                       |
| `testty`, `termlens`, `ratatui-testlib` (crates that package the PTY approach, some "Playwright for the terminal")            | Evaluate in PR 2's spike, adopt only if mature. These are young crates; a 150-line harness on mature primitives carries less risk than a fast-moving dependency.                                                                                                                          |
| `@microsoft/tui-test` (xterm.js, Playwright-style API and snapshots)                                                          | Not chosen. It adds a Node test runner to a Rust CLI's tests, and it's early-stage. Its Playwright-like API is appealing, so revisit if the Rust harness proves painful.                                                                                                                  |
| tmux scripting (what Claude used on Oct 21)                                                                                   | Keep for ad-hoc agent debugging only. Not in CI: Unix-only, timing-based and flaky.                                                                                                                                                                                                       |

## Conventions

- Every journey runs with a temp `HOME` and `BNTO_HOME`, and telemetry and the update check off. It never touches a real `~/.bnto`.
- Snapshots are text; a selected row shows as its `▸` marker. Styles stay out of snapshots unless a test is about style, which keeps diffs readable.
- Fixed sizes, `120x36` and `80x24`. The narrow size catches overflow like the help-bar overlap.
- Waits poll for text with a timeout, never a fixed sleep (Playwright's auto-wait model). A wait passes only on a settled screen (the same text on two reads a poll apart). The budget is a generous 30 s, so a busy machine can't fail a correct journey; set `BNTO_TUI_WAIT_SECS` to change it.
- A journey that presses a key and then checks the screen to decide its next key uses `Tui::step` (or waits for the change it expects, as `select` does), never `press` and an immediate read. On a busy machine the redraw lags, so an immediate read sees the frame from before the key and the journey presses past its target. This was the cause of the "TUI journeys time out when the machine is busy" failures, not the budget.
- Snapshot updates work like the web's two-pass rule: `cargo insta review` (or `INSTA_UPDATE=always`), then a clean re-run.

---

## PR 1: Snapshot every TUI screen at two terminal sizes

**Branch:** `test/tui-screen-snapshots`
**One sentence:** Store each TUI screen's rendered text as an `insta` snapshot at 120x36 and 80x24.

**What:**

- An `insta` dev-dependency and a shared helper `render_snapshot(model, width, height) -> String` in the bnto crate's `tui` tests.
- Snapshots of home (each pane focused), detail (keep-awake, download-video, a file recipe), file picker, consent prompt, execution in progress, results and settings.
- A `task tui:snapshots` alias.
- Fix the help-bar/status overlap (key hints running into "22 recipes · v0.16.1" at 120 columns) here, because its 80x24 snapshot would capture the bug.

**RED tests:** the overlap snapshot first. At 80x24 the help hints and status must not overlap, and hints that don't fit are dropped cleanly or truncated.
**Verification:** `task cli:test` (Linux and Windows CI run the same snapshots)
**Count changes:** none.

## PR 2: Drive the real TUI in a pseudo-terminal

**Branch:** `test/tui-pty-harness`
**One sentence:** Add a test harness that runs `bnto tui` in a PTY and reads its screen like a user sees it.

**What:**

- `bnto/tests/tui/harness.rs` with:
  - `Tui::launch(size)`: sandboxed homes, the real `CARGO_BIN_EXE_bnto`
  - `press(keys)`
  - `wait_for(text, timeout)`
  - `screen() -> String` from `vt100`
  - `quit()`, with the kill-on-drop guard pattern from `keep_awake.rs`
- A short spike first compares the harness against `testty` and `termlens`, and records the choice in this doc.
- First journeys:
  - **TJ1:** home loads and shows 22 recipes and the version
  - **TJ2:** scroll to Keep Awake, open its detail, and check that it shows the Action field
  - **TJ3:** run Compress Images on a fixture through the picker and reach the results screen

**RED tests:** TJ1–TJ3. TJ2 fails against the code before #680.
**Verification:** `cargo test -p bnto --test tui` on Linux and macOS. On Windows it runs in CI over ConPTY, gated `#[ignore]` there until it's stable.
**Count changes:** none.

## PR 3: Write the TUI journey matrix and the agent workflow

**Branch:** `docs/tui-journeys`
**One sentence:** Document the TUI journeys and how agents validate a TUI change with them.

**What:**

- `.claude/journeys/tui.md` with TJ1–TJn: home, library, detail, field editing, consent, execution, results, settings and themes, narrow terminal, and the upgrade notice (from [cli-upgrade.md](cli-upgrade.md)).
- A section in `pre-commit.md`: "touched the TUI → run `task tui:test`, review snapshots".
- `/rust-expert` and `/quality-engineer` point at it.
- Further journeys are added with the features that need them.

**Verification:** `task docs:validate`
**Count changes:** none.

## Found while building PR 1

- **Fixed (#683): Detail descriptions were cut mid-word.** A long field description ends at the panel edge ("check what's c") instead of wrapping. It's visible in the `detail_*` snapshots. To fix in its own PR, which will update those snapshots.
- **Fixed (#684), one question open: Settings reads telemetry outside `BNTO_HOME`, and unit tests wrote the real file.** `TelemetryConfig` lives at `<config dir>/bnto/telemetry.json` (`~/Library/Application Support` on macOS), not `BNTO_HOME`. TUI unit tests toggling telemetry called `telemetry::set_enabled`, which rewrote the developer's real file. It was confirmed modified by a `cargo test` run on Oct 21, and it could silently re-enable telemetry for someone who had turned it off. **Fixed:** `config_path()` returns `None` in test builds, so the snapshot mask is gone. **Also fixed (Ryan chose one setting):** there were two telemetry settings. The TUI toggle wrote both `config.toml` `[telemetry] enabled` (in `BNTO_HOME`) and `telemetry.json`, while telemetry and the Settings screen read only `telemetry.json`. Now config.toml is the one setting, and an old `telemetry.json` opt-out migrates into it ([tui-data-persistence.md](tui-data-persistence.md#telemetry-one-setting-october-2026)).

## Quality gate

Decided (Ryan, Oct 21): the TUI tests are an automated quality gate, like the web's E2E suite. Snapshots and journeys run on every PR in the required CI Gate. They run in the Rust job's `cargo test --package bnto`, and again in a named "TUI tests (snapshots + journeys)" step so failures read as TUI failures. Locally they run in `task check` (via `task tui:test`). Windows journeys stay `#[ignore]`d until ConPTY runs prove stable in CI; enabling them is tracked here. Snapshots already run on Windows.

## PR 2 spike: harness choice (October 21, 2026)

**Decided: our own harness on `portable-pty` + `vt100`.** On crates.io, `portable-pty` (22.5M downloads) and `vt100` (15.3M) are mature. The packaged options are young: `testty` (~2k downloads, 0.18 already, versions moving fast), `termlens` (~6k) and `ratatui-testlib` (~6.6k, 0.1). The harness (`bnto/tests/tui/harness.rs`) is about 200 lines.

Lessons from wiring it up:

- **Wait for the whole frame.** A read can land mid-draw: the pane title is on screen before the status bar. Journeys use `wait_until` with every part they check, never `wait_for` on the first part and then assert the rest.
- **Start in an empty `work/` folder, not HOME.** On macOS bnto writes config under `$HOME/Library`, which a file list would show first.
- **Keys:** pause briefly between keys, so a lone Esc isn't merged into the next escape sequence.

Runs in about 1.5s for three journeys and was 10/10 stable locally. Linux CI runs them in `cargo test --package bnto`. Windows is `#[ignore]`d until ConPTY runs prove stable.

Found while writing TJ3, **fixed in #685**: when an output file was _larger_ than its input, the results screen said "Savings: 5 KB → 10 KB (0% smaller)". It now says "Size: 5 KB → 10 KB (100% larger)".

## Run output audit (October 21, 2026)

These came from driving real runs in the TUI (tmux, sandboxed homes) and comparing them with `bnto run`:

- an explicit-loop recipe on 6 images (`tests/fixtures/explicit/compress-and-rename.bnto.json`)
- a shell recipe that prints a line every 0.4s
- keep-awake with Action "Check status"

**Works:** live streaming (lines appear as the command prints them), the elapsed timer, loop iteration counts (`loop (2/6)`), children indented under their container, the consent prompt (full script, y/n, same as the CLI), and per-file progress messages.

| #   | Finding                                                                                                                                                                                                                                    | Severity | Fix                                                                                                                                                                                                                |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R1  | **Fixed: a recipe's output message was never shown.** Output nodes in `message` mode, such as keep-awake's "No active session." or "Session started…", end on "No output files." The CLI prints them. keep-awake is unusable from the TUI. | High     | Show the output message on the results screen. Journey TJ6 (keep-awake status shows "No active session.") is the RED test.                                                                                         |
| R2  | **Fixed: streamed output disappeared at results.** Everything shown under OUTPUT during the run is gone once it finishes, including the last lines of a failing command.                                                                   | High     | Keep the run's output log on the results screen. Journey TJ5 (shell lines are still readable after the run) is the RED test.                                                                                       |
| R3  | **Fixed: Tab looked trapped by a list field** (see below). On a shell recipe's raw params, Tab stops at Arguments and never reaches Run. Down/`j` work.                                                                                    | Medium   | Scroll the focused field into view, measured after wrapping. Journey TJ11.                                                                                                                                         |
| R4  | A shell recipe with no declared fields exposes the shell node's raw params (Command, Arguments, Output Mode, Timeout, …) on its detail screen.                                                                                             | Medium   | **Fixed:** hidden (Ryan, Oct 21). `ParameterDef::run_option` (false for every shell-run param) gates the no-fields fallback; `surfaceable` still gates authoring. A recipe with nothing to set opens on Run. TJ13. |
| R5  | **Fixed:** Completed and in-progress rows share a marker: mid-run, "Completed photo1-compressed.jpg" still shows `◉`, and states differ only by color. The CLI uses `✓`.                                                                   | Low      | `✓` for done, keeping color as the secondary cue. Snapshot.                                                                                                                                                        |
| R6  | **Fixed:** Waiting files show as "file 2", "file 3" … though their names are known up front.                                                                                                                                               | Low      | Name them from the selected files. Snapshot.                                                                                                                                                                       |
| R7  | **Fixed:** A loop's later steps appear only when they start (`file-rename` pops in mid-run), so the layout shifts.                                                                                                                         | Low      | List every child step from the start as pending. Snapshot.                                                                                                                                                         |
| R8  | **Fixed:** The results screen lists files out of input order (5, 6, 3, 2, 1, 4).                                                                                                                                                           | Low      | Keep input order. Unit test.                                                                                                                                                                                       |
| R9  | CLI: the first-run telemetry notice prints even with `BNTO_TELEMETRY_DISABLED=1` or `DO_NOT_TRACK`.                                                                                                                                        | Low      | Skip the notice when env disables telemetry. Unit test.                                                                                                                                                            |
| R10 | **Fixed:** output was squeezed off small terminals. FILES took a row per file, so at 80x24 OUTPUT had no room, and a run over many files hid it on any size.                                                                               | Medium   | FILES shows a window around the active file plus "… N more", or a single summary line when cramped. OUTPUT keeps four rows.                                                                                        |

Order: R1+R2 (one PR, with TJ5/TJ6), then R3, then R5–R8 as one "execution screen polish" PR with snapshots, then R9. R4 was decided: hide (see its row).

**R3 fixed.** Tab was never trapped: Arguments is a plain text field and Tab moved focus past it. The viewport didn't follow. It scrolled to an estimate of 3 rows per field, but the shell params' descriptions wrap to 4–6 rows (#683), so from Output Mode on, the focused field and Run sat below the fold at 80x24. Tab on the hidden Run then started the run. Now the main loop scrolls the focus into view before each draw, measured from the wrapped lines (`tui/detail_fit.rs`, tonkotsu `focused_field_lines`). It does this only when focus moves, so PageDown/`J` scrolling sticks. Journey TJ11 is the RED test.

## Dependency Chain

```
PR 1 (screen snapshots + help-bar fix)
|-- PR 2 (PTY harness + TJ1-TJ3)
    |-- PR 3 (journey matrix + agent workflow)
```

## Sources

- [Ratatui: Testing with insta snapshots](https://ratatui.rs/recipes/testing/snapshots/)
- [@microsoft/tui-test](https://npmjs.com/package/@microsoft/tui-test)
- [testty](https://docs.rs/crate/testty/0.10.4), [termlens](https://docs.rs/crate/termlens/0.2.0), [ratatui-testlib](https://docs.rs/ratatui-testlib)
