# bnto-cli

CLI for running `.bnto.json` recipes. Compress images, clean CSVs, rename files.

## Overview

`bnto-cli` is the native command-line interface for the bnto engine. It uses `bnto-engine` for processor registration and pipeline execution, so the CLI runs the exact same node processors as the browser WASM build.

Binary name: `bnto`

## Usage

```bash
# Run a recipe against input files
bnto run recipe.bnto.json photo.jpg logo.png

# Specify output directory
bnto run compress-images.bnto.json *.jpg --output ./compressed

# List available processors
bnto list
```

## Directory Structure

```
src/
├── main.rs         # CLI entry point (clap parser, run/list commands)
├── io.rs           # File I/O (read input files, write pipeline results)
└── progress.rs     # Stderr progress reporter
tests/
├── helpers/mod.rs  # Shared test helpers (fixtures, temp dirs)
├── recipe_tests.rs # Integration tests for recipe execution
└── cli_commands.rs # Integration tests for CLI commands (help, list, errors)
```

## Development

```bash
cargo build -p bnto-cli        # Build the binary
cargo test -p bnto-cli         # Run integration tests
./target/debug/bnto --help     # Run locally
```
