# bnto-spreadsheet

Spreadsheet processing nodes. Clean, convert, merge, and rename columns.

## Overview

`bnto-spreadsheet` provides four `NodeProcessor` implementations for spreadsheet transformation. Uses the pure-Rust `csv` crate for parsing and writing. Runs in the browser via WASM or natively for desktop/CLI.

## Processors

| Processor           | Node Type             | What It Does                                                             |
| ------------------- | --------------------- | ------------------------------------------------------------------------ |
| `CleanSpreadsheet`  | `spreadsheet-clean`   | Remove empty rows, trim whitespace, deduplicate rows (each configurable) |
| `ConvertFormat`     | `spreadsheet-convert` | Convert CSV to JSON                                                      |
| `MergeSpreadsheets` | `spreadsheet-merge`   | Merge multiple CSV files into one                                        |
| `RenameColumns`     | `spreadsheet-rename`  | Rename columns via a map of `oldName → newName`                          |

## Directory Structure

```
src/
├── lib.rs              # Public exports
├── clean.rs            # CleanSpreadsheet processor + metadata
├── convert.rs          # ConvertFormat processor + metadata
├── merge.rs            # MergeSpreadsheets processor + metadata
├── rename.rs           # RenameColumns processor + metadata
└── wasm_bridge.rs      # #[wasm_bindgen] bridge functions
tests/
├── wasm_clean.rs       # Clean operation tests
└── wasm_rename.rs      # Rename operation tests
```

## Development

```bash
cargo test -p bnto-spreadsheet   # Native unit tests
task wasm:test                   # Full WASM integration tests
```
