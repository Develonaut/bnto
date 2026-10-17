# bnto-spreadsheet

Spreadsheet processing nodes. Clean, convert, export, merge, look up, read, and rename columns.

## Overview

`bnto-spreadsheet` provides seven `NodeProcessor` implementations for spreadsheet work. Uses the pure-Rust `csv` crate for parsing and writing. Runs in the browser via WASM or natively for desktop/CLI; `spreadsheet-lookup` and `spreadsheet-export` are native-only because they read or write a file on disk.

## Processors

| Processor           | Node Type             | What It Does                                                                   | Platforms    |
| ------------------- | --------------------- | ------------------------------------------------------------------------------ | ------------ |
| `CleanSpreadsheet`  | `spreadsheet-clean`   | Remove empty rows, trim whitespace, deduplicate rows (each configurable)       | browser, cli |
| `ConvertFormat`     | `spreadsheet-convert` | Convert CSV to JSON                                                            | browser, cli |
| `SpreadsheetExport` | `spreadsheet-export`  | Write the files passing through, with their metadata, as one CSV row each      | cli          |
| `SpreadsheetLookup` | `spreadsheet-lookup`  | Attach the CSV row whose key matches each file's name to that file as metadata | cli          |
| `MergeSpreadsheets` | `spreadsheet-merge`   | Merge multiple CSV files into one                                              | browser, cli |
| `RenameColumns`     | `spreadsheet-rename`  | Rename columns via a map of `oldName → newName`                                | browser, cli |
| `ReadSpreadsheet`   | `spreadsheet-read`    | Read a CSV's rows or a JSON array's elements as one item each, with a row cap  | browser, cli |

## Reading Rows

`spreadsheet-read` turns a table file into one item per row: an empty file whose metadata holds the row's values, so a loop or a per-file node reads them with `{{item.<key>}}`. `format` picks the parser: `auto` (default) reads a `.json` file (any case) or one typed `application/json` as JSON and anything else as CSV; `csv` and `json` force one. Content isn't sniffed. Items are named `<stem>_row<i>.csv` or `.json`, and the node reports `rowCount` and `columnCount`. `maxRows` (default 100,000) caps both formats.

**CSV:** each record is a row, its cells keyed by header (`col0`, `col1`, … with `hasHeaders: false`), every value a string. `delimiter` and `hasHeaders` apply to CSV only.

**JSON:** the file must hold one top-level array; each element is a row, and an empty array gives no items. A file that isn't valid JSON or isn't an array fails the run, naming it.

- An object's fields become keys, and **nested objects flatten into dotted keys**: `{"meta": {"id": "x"}}` gives `meta.id` = `"x"`, read as `{{item.meta.id}}`.
- Values keep their JSON type (string, number, boolean, null).
- An array, or an empty object, is a leaf: it's kept as JSON under its key (`tags` = `["a", "b"]`), not split by index.
- An element that isn't an object (`"a"`, `2`, `[3]`) is kept under `value`.
- Two fields that read as the same key (`{"a.b": 1, "a": {"b": 2}}`) fail the run, naming the file and key, rather than one silently winning.
- `columnCount` is the number of distinct keys across all elements.

```
input (items.json) → spreadsheet-read → file-rename (pattern: "{{item.meta.id}}.txt") → output
```

## Looking Up Rows

`spreadsheet-lookup` joins each file to a row of the CSV at `table` (the VLOOKUP pattern). The row whose `key_column` equals the file's pipeline name exactly, or only its last path segment with `match_on: filename`, becomes the file's metadata, so later nodes read its columns with `{{item.<column>}}`. File data passes through untouched.

A key may appear on one row only; rows with an empty key are skipped. Keys are compared as written: pipeline names use `/` on every OS, and no Unicode normalization is applied. A relative `table` path resolves against the current directory.

A file with no row is handled per `unmatched`:

- `keep` (default): passes on flagged `unmatched: 1` (a matched file carries `unmatched: 0`). The CLI names those files after the run, on a dry run too. In a rename chain the rename fails on the first such file instead, so run the lookup on its own to list every file without a row.
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

## Exporting Items

`spreadsheet-export` writes the files passing through as a CSV at `path` and passes them on unchanged; it never reads file content. One row per file: `name_column` (default `file`) holds the file's pipeline name, then every metadata key seen across the files, in the order first seen (alphabetical within one file), with an empty cell where a file lacks a key. Arrays and objects are written as JSON. It's a batch node, so it writes once per run, and the run output names the path and row count.

- **The folder must exist**, and a relative `path` resolves against the current directory. Keep the CSV outside a folder the recipe collects, or the next run collects it too.
- **An existing file is kept** and fails the run, since it may be an earlier run's record; set `overwrite` to replace it. The CSV is written to a temporary file and renamed into place, so an interrupted write never leaves half a record.
- **A dry run writes it too**: it's a report, not a change to the files, and the run output says "(dry run)".
- **It records what the run planned.** It runs before `output`, so if `output` fails partway, the CSV lists renames that didn't all land.
- Names that start with `=`, `+`, `-` or `@` may open as formulas in a spreadsheet app; the CSV keeps them as written so bnto reads them back exactly.

After `spreadsheet-lookup` and `file-rename`, the CSV is the run's record: each file's new name (`file`), the row's columns (`name`, `new_name`, …) and `unmatched`. It drives the undo: `spreadsheet-lookup` with `table` set to the record, `key_column: file` and `match_on: filename`, then `file-rename` with `pattern: "{{item.name}}"`. An undo that also exports needs another `name_column`, since its files carry the record's `file` column.

## Directory Structure

```
src/
├── lib.rs              # Public exports
├── clean.rs            # CleanSpreadsheet processor + metadata
├── convert.rs          # ConvertFormat processor + metadata
├── export/             # SpreadsheetExport processor (spreadsheet-export)
│   ├── mod.rs              # Module wiring + re-export
│   ├── processor.rs        # NodeProcessor impl, CSV write, pass-through
│   ├── params.rs           # Parameter schema and typed reads
│   └── table.rs            # Header union and row building
├── lookup/             # SpreadsheetLookup processor (spreadsheet-lookup)
│   ├── mod.rs              # Module wiring + re-export
│   ├── processor.rs        # NodeProcessor impl, unmatched handling
│   ├── params.rs           # Parameter schema and typed reads
│   ├── table.rs            # CSV parsing and exact-key index
│   └── source.rs           # Table loading, cached across a run
├── merge.rs            # MergeSpreadsheets processor + metadata
├── read/               # ReadSpreadsheet processor (spreadsheet-read)
│   ├── mod.rs              # Module wiring + re-export
│   ├── processor.rs        # NodeProcessor impl, format dispatch, row cap
│   ├── params.rs           # Parameter schema, typed reads, format choice
│   ├── csv_rows.rs         # CSV records to rows
│   ├── json_rows.rs        # JSON array elements to rows, nested fields flattened
│   ├── table.rs            # Row items and node metadata
│   └── json_tests.rs       # JSON reading tests
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
