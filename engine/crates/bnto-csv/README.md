# bnto-csv

CSV processing nodes — clean and rename columns.

## Overview

`bnto-csv` provides two `NodeProcessor` implementations for CSV transformation. Uses the pure-Rust `csv` crate for parsing and writing. Runs in the browser via WASM or natively for desktop/CLI.

## Processors

| Processor          | Node Type            | What It Does                                                             |
| ------------------ | -------------------- | ------------------------------------------------------------------------ |
| `CleanCsv`         | `spreadsheet-clean`  | Remove empty rows, trim whitespace, deduplicate rows (each configurable) |
| `RenameCsvColumns` | `spreadsheet-rename` | Rename columns via a map of `oldName → newName`                          |

## Directory Structure

```
src/
├── lib.rs              # Public exports
├── clean.rs            # CleanCsv processor + metadata
├── rename_columns.rs   # RenameCsvColumns processor + metadata
└── wasm_bridge.rs      # #[wasm_bindgen] bridge functions
tests/
├── wasm_clean.rs       # Clean operation tests
└── wasm_rename.rs      # Rename operation tests
```

## Development

```bash
cargo test -p bnto-csv            # Native unit tests
task wasm:test                    # Full WASM integration tests
```
