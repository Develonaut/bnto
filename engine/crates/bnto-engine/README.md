# bnto-engine

Shared engine layer — registry creation and pipeline convenience for all consumers.

## Overview

`bnto-engine` provides the default processor registry and a `run_pipeline()` convenience function so both `bnto-wasm` (browser) and `bnto-cli` (native binary) share the same processor wiring without duplicating registration code.

## Exports

| Function                    | What It Does                                                       |
| --------------------------- | ------------------------------------------------------------------ |
| `create_default_registry()` | Returns a `NodeRegistry` pre-loaded with all browser-capable nodes |
| `run_pipeline()`            | Parses JSON, creates registry, executes pipeline in one call       |

## Registered Processors

| Node Type            | Crate        | Processor            |
| -------------------- | ------------ | -------------------- |
| `image-compress`     | `bnto-image` | `CompressImages`     |
| `image-resize`       | `bnto-image` | `ResizeImages`       |
| `image-convert`      | `bnto-image` | `ConvertImageFormat` |
| `spreadsheet-clean`  | `bnto-csv`   | `CleanCsv`           |
| `spreadsheet-rename` | `bnto-csv`   | `RenameCsvColumns`   |
| `file-rename`        | `bnto-file`  | `RenameFiles`        |

## Development

```bash
cargo test -p bnto-engine      # Unit tests (registry, pipeline, generated recipes)
```
