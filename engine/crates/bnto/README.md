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

# List available built-in recipes
bnto list

# Show details about a recipe
bnto info compress-images

# Check external dependencies
bnto doctor
```

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
+-- main.rs                # CLI entry point (clap parser)
+-- context.rs             # NativeContext for system access (commands, temp files, env)
+-- doctor.rs              # `bnto doctor` command (dependency checks)
+-- info.rs                # `bnto info` command (recipe details)
+-- input.rs               # Input preparation (files, URLs, param overrides)
+-- io.rs                  # File I/O (read inputs, write pipeline results)
+-- list.rs                # `bnto list` command (grouped recipe listing)
+-- progress.rs            # Stderr progress reporter (indicatif bars)
+-- tui/
    +-- mod.rs             # TUI entry point, event loop, terminal setup/teardown
    +-- app.rs             # App state machine (Screen enum, AppModel, update())
    +-- bridge.rs          # Pipeline execution bridge (engine -> TUI progress)
    +-- config.rs          # Persistent settings (theme, output dir)
    +-- event.rs           # Terminal event polling
    +-- format.rs          # File size and path formatting
    +-- keys.rs            # Key-to-message mapping (global + per-screen)
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
    |   +-- execution.rs   # Execution progress model + update
    |   +-- picker.rs      # File picker model + update
    |   +-- picker_update.rs # Picker key handling
    |   +-- picker_loader.rs # Async directory loading
    |   +-- results.rs     # Results display model + update
    |   +-- settings.rs    # Settings screen model + update
    |   +-- viewport.rs    # Scrollable viewport logic
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
- **CLI commands**: Integration tests verify `--help`, `list`, `info`, `doctor`
- **Golden tests**: Byte-exact output comparison for all built-in recipes
