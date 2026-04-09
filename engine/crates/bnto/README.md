# bnto

CLI and interactive TUI for running `.bnto.json` recipes. Compress images, clean CSVs, rename files.

## Overview

`bnto` is the native command-line interface for the bnto engine. It uses `bnto-engine` for processor registration and pipeline execution, so the CLI runs the exact same node processors as the browser WASM build.

Running `bnto` with no arguments launches the interactive TUI when a terminal is detected. Pass `--no-interactive` to suppress the TUI in scripts or CI.

## Install

```bash
cargo install bnto
```

## Usage

```bash
# Launch interactive TUI (default when running in a terminal)
bnto

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

# Explicitly launch the TUI
bnto tui

# Suppress TUI in scripts/CI
bnto --no-interactive
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

The TUI has 5 screens representing a complete recipe execution flow:

| Screen        | Purpose                                     |
| ------------- | ------------------------------------------- |
| **Browser**   | Browse, search, and select a recipe         |
| **Detail**    | View recipe details, override parameters    |
| **Picker**    | Browse filesystem, multi-select input files |
| **Execution** | Live per-file and per-node progress         |
| **Results**   | Output summary, open files, run another     |

Navigation flows forward through the screens and back via `Esc`. `q` quits from any screen.

### Theming

The TUI uses the same Motorway color palette as the web app, adapted for terminal rendering via 256-color ANSI RGB. Colors are generated from `theme/palette.toml` (the shared source of truth) into `tui/palette.rs`. Rounded box-drawing characters (`╭╮╰╯`) match the web app's generous border radius. Every screen displays a contextual help bar at the bottom showing available keybindings.

## Directory Structure

```
src/
+-- main.rs           # CLI entry point (clap parser, TTY detection)
+-- context.rs        # NativeContext for system access (commands, temp files, env)
+-- info.rs           # `bnto info` command (recipe details)
+-- input.rs          # Input preparation (files, URLs, param overrides)
+-- io.rs             # File I/O (read inputs, write pipeline results)
+-- list.rs           # `bnto list` command (grouped recipe listing)
+-- progress.rs       # Stderr progress reporter (indicatif bars)
+-- tui/
    +-- mod.rs        # TUI entry point, event loop, terminal setup/teardown
    +-- app.rs        # App state machine (Screen enum, AppModel, update())
    +-- event.rs      # Terminal event polling, key-to-message mapping
    +-- theme.rs      # Style helpers, border set (colors from generated palette)
    +-- screens/      # Per-screen rendering (placeholder, Wave 1)
    +-- widgets/      # Shared TUI widgets (placeholder, Wave 1)
tests/
+-- cli_commands.rs   # Integration tests for CLI commands
+-- golden_tests.rs   # Golden tests (byte-exact output verification)
+-- recipe_tests.rs   # Integration tests for recipe execution
+-- helpers/          # Shared test helpers (fixtures, temp dirs)
```

## Development

```bash
cargo build -p bnto          # Build the binary
cargo test -p bnto           # Run all tests (unit + integration + golden)
./target/debug/bnto --help   # Run locally
./target/debug/bnto          # Launch TUI
```

## Testing

The TEA pattern makes most TUI logic testable as pure Rust:

- **State transitions**: `update()` is a pure function tested with plain assertions
- **Key mapping**: `map_global_key()` maps crossterm events to app messages
- **Screen hints**: Every screen produces non-empty help bar hints
- **CLI commands**: Integration tests verify `--help`, `list`, `info`, `doctor`, `--no-interactive`
- **Golden tests**: Byte-exact output comparison for all 15 built-in recipes
