# bnto

CLI and interactive TUI for running `.bnto.json` recipes. Compress images, clean CSVs, rename files.

## Overview

`bnto` is the native command-line interface for the bnto engine. It uses `bnto-engine` for processor registration and pipeline execution, so the CLI runs the exact same node processors as the browser WASM build.

`bnto tui` launches an interactive terminal UI with 6 screens: recipe browser, detail/config, file picker, execution progress, results summary, and settings. Three themes from the Motorway palette (los-angeles, tokyo, monaco) with runtime switching.

The detail screen renders type-aware controls for recipe parameters: boolean toggles (`[x]`/`[ ]`), enum cycling (`◂ label ▸`), bounded number stepping with auto step size, and text input for strings. Parameters support conditional visibility (`visible_when`), descriptions on focus, unit suffix annotations, and reset-to-default.

## Install

```bash
cargo install bnto
```

## Usage

```bash
# Launch the interactive TUI
bnto tui

# TUI with a specific theme
bnto tui --theme tokyo       # dark theme
bnto tui --theme monaco      # sunset theme

# Run a recipe against input files
bnto run compress-images photo.jpg logo.png

# Run with a recipe file and output directory
bnto run recipe.bnto.json *.jpg --output ./compressed

# Override parameters
bnto run compress-images photo.jpg --param quality=50

# Fill {{vars.KEY}} placeholders from JSON files (repeatable; later files win,
# and a BNTO_VAR_<KEY> environment variable beats every file)
bnto run backup.bnto.json --vars machine.json

# Also print the run's summary as one JSON line on stdout (for scripts)
bnto run backup.bnto.json --vars machine.json --yes --json

# Stop the whole run after 10 minutes (overrides the recipe's settings.timeout)
bnto run backup.bnto.json --timeout 600

# List available built-in recipes
bnto list

# Show details about a recipe (dependencies show this OS's install hint)
bnto info compress-images

# Check recipe files without running them: unknown node types, parameters a
# node refuses (e.g. a file-filter regex that doesn't compile), malformed
# {{…}} placeholders, and every edge problem at every level (cycles, edges to
# unknown or other-level nodes, unwired nodes). A required param left out for
# `bnto run -p node:param=value` to give is a note, not a finding; one no `-p`
# can give (an object, or a node `-p` can't name) is a finding. Prints one line
# per finding, then a line per note, then `Check: PASS (<n> recipes)` (exit 0)
# or `Check: FAIL (<n> findings)` (exit 1), with `, <n> notes` added if any
bnto check backup.bnto.json recipes/*.bnto.json

# A folder checks every .bnto.json under it, recursively, sorted by path, so a
# new recipe can't miss the check (and it works where the shell expands no
# globs, like Windows). Files and folders mix. node_modules/ and target/ are
# skipped while walking, symlinked folders aren't followed, and a folder with
# no recipes in it is a finding that fails the check
bnto check recipes/ backup.bnto.json

# Check external dependencies (processor and recipe-level `requires`)
bnto doctor

# Install a recipe's missing dependencies (prints each exact command, then asks)
bnto install download-video
```

### Output names

With `--output` (the `output` step's `write` mode), every result lands as its own file. Names that match, or differ only in case (`Logo.svg` and `logo.svg`, one file on Windows and macOS), are told apart the same way on every OS: the later file is written as `logo_1.svg`, `logo_2.svg`…, the suffix `file-copy`'s `rename` conflict mode uses, and a warning names each one. The `Done.` count is the number of files on disk. Names are only kept apart from each other: a file already in the output folder from an earlier run is replaced.

### Time limit

A recipe can limit its whole run with `"settings": { "timeout": <seconds> }` (a whole number above 0); `bnto run --timeout <seconds>` overrides it, or limits a recipe that sets none. With neither, a run has no limit. When the limit passes, every step the run started is stopped, child processes included (a shell-command's whole process tree), no further step, file, loop item or `always` closing step starts, and the run fails with `Pipeline failed: Run timed out after <seconds>s` as its last stderr line and exit code 1. `--json` reports the same message as `error`. The limit covers the whole run, from dependency checks to the last step; a shell-command's own `timeout` still stops just that command. `bnto dry-run` shows the recipe's limit, and `bnto check` reports a `settings.timeout` that isn't a whole number of seconds above 0.

### `bnto run --json`

`--json` prints the run's summary as exactly one JSON line on stdout, whether the run passes or fails. Progress, warnings and the human summary line stay on stderr, unchanged, and the exit code is the same (0 on a pass, 1 on a failure). The human `Done.` / `Pipeline failed:` line is printed last on stderr, after every warning, and both it and the JSON line are built from the same summary.

```json
{
  "status": "pass",
  "files_written": 1,
  "duration_ms": 5,
  "nodes": [
    { "id": "collect", "node_type": "file-collect", "files_processed": 1, "metadata": {} },
    {
      "id": "copy",
      "node_type": "file-copy",
      "files_processed": 1,
      "metadata": { "copied": 1, "up_to_date": 0, "skipped_newer": 0 }
    }
  ],
  "error": null
}
```

| Field              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`           | `"pass"` or `"fail"`                                                                                                                                                                                                                                                                                                                                                                                                       |
| `files_written`    | Files the output step wrote, or renamed in place in overwrite mode; `0` for `message` / `none` output and on a failure. On `--dry-run`, the files that would be written                                                                                                                                                                                                                                                    |
| `duration_ms`      | Pipeline time on a pass; time since the run started on a failure                                                                                                                                                                                                                                                                                                                                                           |
| `nodes`            | Top-level processing nodes in pipeline order: `id`, `node_type`, `files_processed` summed over every time the node ran (nodes inside a container count toward the container), and `metadata`, the node's output record as `{{node.<id>.*}}` reads it at the end of the run. A node that never finished shows `0` and `{}`                                                                                                  |
| `nodes[].metadata` | What the node reported, as a JSON object; `{}` when it reported nothing (a container's own entry is always `{}`). Keys its processor declares as counts are totals for the whole node, e.g. file-copy in `newer` mode reports `copied`, `up_to_date` and `skipped_newer` (on `--dry-run`, `copied` counts the files that would be copied); other keys hold the last file's value. A failed run reports `{}` for every node |
| `error`            | The failure message (the text after `Pipeline failed:`, `Error writing output:`, `Error reading recipe:` or `Error:` on stderr); `null` on a pass                                                                                                                                                                                                                                                                          |

A run declined at the shell-command consent prompt prints no JSON line; pass `--yes` when scripting.

`bnto install` runs the recipe's `installHints` command for this OS (e.g. `winget install Gyan.FFmpeg`) through the platform shell; without one it falls back to the detected package manager (`brew`/`apt`/`pacman`/`choco install <binary>`), or prints the hint when nothing can run. `--yes` skips the prompt for bundled recipes only — commands written by library or file recipes are always shown and confirmed.

## TUI Architecture

The interactive terminal UI follows the **TEA (The Elm Architecture)** pattern: a pure state machine where all state transitions are testable without a terminal.

```
Model (state)  -->  View (render)  -->  Event (input)  -->  Update (pure fn)
     ^                                                          |
     +----------------------------------------------------------+
```

### Core Pattern

- **Model** (`AppModel`): Plain Rust struct holding current screen and app state. No side effects, no I/O.
- **Update** (`update()`): Pure function `(AppModel, AppMessage) -> AppModel`. All screen transitions live here. Testable with `cargo test`, no terminal needed.
- **View** (`draw()`): Renders the model to a ratatui `Frame`. Visual concern only.
- **Event** (`poll_event()`): Maps crossterm key events to `AppMessage` variants. Thin translation layer.

This mirrors the web editor's architecture: `EditorState` (Model), pure action functions (Update), React components (View), hook wrappers (Event).

### Screens

The TUI has 6 screens representing a complete recipe execution flow:

| Screen        | Purpose                                                |
| ------------- | ------------------------------------------------------ |
| **Browser**   | Browse, search, and select a recipe                    |
| **Detail**    | View recipe details, override parameters               |
| **Picker**    | Browse filesystem, multi-select input files            |
| **Execution** | Live per-file and per-node progress with elapsed timer |
| **Results**   | Output summary with sizes, savings, open files         |
| **Settings**  | Switch themes, configure output directory              |

Navigation flows forward through the screens and back via `Esc`. `q` quits from any screen. `s` opens Settings from the Browser screen.

### Theming

The TUI uses the same Motorway color palette as the web app, adapted for terminal rendering via 256-color ANSI RGB. Colors are generated from `theme/palette.toml` (the shared source of truth) into `tui/palette.rs`. Rounded box-drawing characters (`╭╮╰╯`) match the web app's generous border radius. Every screen displays a contextual help bar at the bottom showing available keybindings.

## Directory Structure

```
src/
+-- main.rs                # CLI entry point (storage, logger, catalog, dispatch)
+-- cli.rs                 # clap definitions for every subcommand
+-- dispatch.rs            # Routes a parsed subcommand to its implementation
+-- run.rs                 # `bnto run` flow (consent, engine run, progressive output)
+-- run_output.rs          # Writes run results and prints the summary
+-- run_verdict.rs         # A run's last stderr line and its `--json` summary line, from one RunSummary
+-- run_limit.rs           # A run's time limit: the recipe's settings.timeout or `--timeout`
+-- run_watchdog.rs        # Kills a run's child processes and ends it once its time limit passes
+-- vars.rs                # `--vars` files and `{{vars.*}}` resolution before a run
+-- consent.rs             # Shell-command trust/consent gate (run + dry-run)
+-- inspect.rs             # `bnto list` / `info` / `dry-run`
+-- check.rs               # `bnto check` (recipe files checked on disk, findings + notes + verdict)
+-- check_targets.rs       # `bnto check` arguments to recipe files (a folder walked recursively, sorted)
+-- launch.rs              # `bnto tui` launcher
+-- context.rs             # NativeContext for system access (commands, temp files, env)
+-- scratch.rs             # Per-run scratch dir holding a run's temp files (removed on drop)
+-- doctor.rs              # `bnto doctor` command (processor + recipe-level dependency checks)
+-- info.rs                # `bnto info` command (recipe details)
+-- input.rs               # Input preparation (files, URLs, param overrides)
+-- install.rs             # `bnto install` command (plan, confirm, run, verify)
+-- install_plan.rs        # Per-dependency install step (host hint / package manager / manual)
+-- install_trust.rs       # Whether `--yes` may run recipe-authored install commands
+-- io.rs                  # File I/O (read inputs, write pipeline results)
+-- output_names.rs        # Keeps result names apart in the output folder (case ignored)
+-- list.rs                # `bnto list` command (grouped recipe listing)
+-- progress.rs            # Stderr progress reporter (pipeline events -> lines)
+-- progress_line.rs       # A step's live line: bar of files, or spinner + running message
+-- live_bar.rs            # Draws the live line with indicatif (self-ticking, hidden off a TTY)
+-- progress_indent.rs     # Step layout: container children indented, container-relative numbers
+-- tui/
    +-- mod.rs             # TUI entry point, event loop, terminal setup/teardown
    +-- app.rs             # App state machine (Screen enum, AppModel, update())
    +-- app/               # AppMessage enum and AppModel construction
    +-- bridge.rs          # Pipeline execution bridge (engine -> TUI progress)
    +-- bridge/            # Engine event mapping and output-file helpers
    +-- config.rs          # Persistent settings (theme, output dir)
    +-- event.rs           # Terminal event polling
    +-- format.rs          # File size and path formatting
    +-- keys/              # Key-to-message mapping (router + one file per screen)
    +-- palette.rs         # Generated color constants from theme/palette.toml
    +-- render.rs          # Root render dispatcher
    +-- render_detail.rs   # Detail screen renderer
    +-- render_execution.rs # Execution screen renderer
    +-- render_picker.rs   # File picker renderer
    +-- render_results.rs  # Results screen renderer
    +-- screen.rs          # Screen titles and help hints
    +-- theme.rs           # Theme variants and style helpers
    +-- screens/
    |   +-- browser.rs     # Recipe browser model + update
    |   +-- controls/      # Type-aware parameter controls
    |   |   +-- boolean.rs    # [x]/[ ] toggle
    |   |   +-- enum_select.rs # ◂ label ▸ cycling
    |   |   +-- number.rs     # Arrow-key stepping with auto step size
    |   +-- detail.rs      # Recipe detail model + update
    |   +-- detail_loader.rs # Recipe detail loading + integration tests
    |   +-- editor.rs      # Recipe editor model + update (routes overlays first)
    |   +-- editor/        # Node list ops, inline form sync, picker overlay, confirm prompts
    |   +-- execution.rs   # Execution progress model
    |   +-- execution/     # ExecutionMessage, update router, pipeline + node transitions
    |   +-- picker.rs      # File picker model + update
    |   +-- picker_update.rs # Picker key handling
    |   +-- picker_loader.rs # Async directory loading
    |   +-- results.rs     # Results display model + update
    |   +-- settings.rs    # Settings screen model + update
    |   +-- viewport.rs    # Scrollable viewport logic
    |   +-- wizard.rs      # Guided recipe creation model + update
    |   +-- wizard/        # Step transitions, category catalog, config form + editor hand-off
    |   +-- nav_history.rs # Directory navigation history
    +-- widgets/
        +-- help_bar.rs    # Contextual keybinding hints footer
        +-- search_input.rs # Text input with cursor
        +-- file_list.rs   # File listing widget
        +-- status_line.rs # Bottom status bar
tests/
+-- cli_commands.rs        # Integration tests for CLI commands
+-- golden_tests.rs        # Golden tests (byte-exact output verification)
+-- recipe_tests.rs        # Integration tests for recipe execution
+-- helpers/               # Shared test helpers (fixtures, temp dirs)
```

## Development

```bash
cargo build -p bnto          # Build the binary
cargo test -p bnto           # Run all tests (unit + integration + golden)
./target/debug/bnto --help   # Run locally
./target/debug/bnto          # Launch TUI
```

## Testing

The TEA pattern makes most TUI logic testable as pure Rust. 450+ TUI tests:

- **State transitions**: `update()` is a pure function tested with plain assertions
- **Type-aware controls**: Boolean toggle, enum cycling, bounded number stepping, reset-to-default
- **Schema integration**: Real recipes loaded through engine registry, param types and metadata verified
- **Key mapping**: Global and per-screen key handlers tested without a terminal
- **Screen models**: Browser search/filter, detail param editing, picker navigation/selection, execution progress, results formatting, settings persistence
- **Viewport scrolling**: Cursor tracking, page up/down, boundary wrapping
- **Navigation history**: Directory stack push/pop, forward/back
- **Bridge**: Pipeline execution event relay to TUI progress model
- **Config persistence**: Settings serialization/deserialization to disk
- **CLI commands**: Integration tests verify `--help`, `list`, `info`, `doctor`, `check`
- **Golden tests**: Byte-exact output comparison for all built-in recipes
