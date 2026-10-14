# bnto-spreadsheet

Spreadsheet processing nodes. Clean, convert, merge, look up, read, and rename columns.

## Overview

`bnto-spreadsheet` provides six `NodeProcessor` implementations for spreadsheet work. Uses the pure-Rust `csv` crate for parsing and writing. Runs in the browser via WASM or natively for desktop/CLI; `spreadsheet-lookup` is native-only because it reads its table from disk.

## Processors

| Processor           | Node Type             | What It Does                                                                   | Platforms    |
| ------------------- | --------------------- | ------------------------------------------------------------------------------ | ------------ |
| `CleanSpreadsheet`  | `spreadsheet-clean`   | Remove empty rows, trim whitespace, deduplicate rows (each configurable)       | browser, cli |
| `ConvertFormat`     | `spreadsheet-convert` | Convert CSV to JSON                                                            | browser, cli |
| `SpreadsheetLookup` | `spreadsheet-lookup`  | Attach the CSV row whose key matches each file's name to that file as metadata | cli          |
| `MergeSpreadsheets` | `spreadsheet-merge`   | Merge multiple CSV files into one                                              | browser, cli |
| `RenameColumns`     | `spreadsheet-rename`  | Rename columns via a map of `oldName → newName`                                | browser, cli |
| `ReadSpreadsheet`   | `spreadsheet-read`    | Read CSV rows with configurable headers, delimiter, and row limit              | browser, cli |

## Looking Up Rows

`spreadsheet-lookup` joins each file to a row of the CSV at `table` (the VLOOKUP pattern). The row whose `key_column` equals the file's pipeline name exactly, or only its last path segment with `match_on: filename`, becomes the file's metadata, so later nodes read its columns with `{{item.<column>}}`. File data passes through untouched.

A key may appear on one row only; rows with an empty key are skipped. Keys are compared as written: pipeline names use `/` on every OS, and no Unicode normalization is applied. A relative `table` path resolves against the current directory.

A file with no row is handled per `unmatched`:

- `keep` (default): passes on flagged `unmatched: 1`. The CLI names those files after the run, on a dry run too, when nothing after the lookup replaces the file's metadata. Run the lookup on its own to list every file without a row.
- `drop`: left out of the output.
- `fail`: fails the run, naming the file.

A later node that reads a column the file doesn't carry (e.g. `{{item.new_name}}` on a kept unmatched file) fails the run, naming the file, so nothing is renamed to an unfilled template.

Renaming a library from a mapping table, and undoing it from the same table. The undo matches on the leaf name, so it needs every `new_name` in the table to be unique:

```
file-collect (flatten: false) → spreadsheet-lookup (key_column: name)
  → file-rename (pattern: "{{item.new_name}}") → output (mode: overwrite)

file-collect (flatten: false) → spreadsheet-lookup (key_column: new_name, match_on: filename)
  → file-rename (pattern: "{{item.name}}") → output (mode: overwrite)
```

## Directory Structure

```
src/
├── lib.rs              # Public exports
├── clean.rs            # CleanSpreadsheet processor + metadata
├── convert.rs          # ConvertFormat processor + metadata
├── lookup/             # SpreadsheetLookup processor (spreadsheet-lookup)
│   ├── mod.rs              # Module wiring + re-export
│   ├── processor.rs        # NodeProcessor impl, unmatched handling
│   ├── params.rs           # Parameter schema and typed reads
│   ├── table.rs            # CSV parsing and exact-key index
│   └── source.rs           # Table loading, cached across a run
├── merge.rs            # MergeSpreadsheets processor + metadata
├── read.rs             # ReadSpreadsheet processor + metadata
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
