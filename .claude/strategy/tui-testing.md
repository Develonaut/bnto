# TUI Snapshots + Journey Tests -- 3 PRs

**Created:** October 21, 2026
**Status:** Approved, October 21, 2026; next in the queue (before the [CLI upgrade notices](cli-upgrade.md), then keep-awake PR 3). PR 1 in review.
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
- Waits poll for text with a timeout, never a fixed sleep (Playwright's auto-wait model).
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

- **Detail descriptions are cut mid-word.** A long field description ends at the panel edge ("check what's c") instead of wrapping. It's visible in the `detail_*` snapshots. To fix in its own PR, which will update those snapshots.
- **Settings reads telemetry outside `BNTO_HOME`.** `TelemetryConfig::load()` reads the real `~/.config/bnto/telemetry.json` (`dirs::config_dir`), so a sandboxed TUI still shows the user's own telemetry choice. Snapshots mask the value (`◂ [telemetry] ▸`). A follow-up should route the telemetry config through `BntoPaths`, so tests and `BNTO_HOME` users stay isolated.

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
