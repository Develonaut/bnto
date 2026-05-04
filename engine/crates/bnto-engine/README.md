# bnto-engine

Shared engine layer. Registry creation and pipeline convenience for all consumers.

## Overview

`bnto-engine` provides the processor registry and a `run_pipeline()` convenience function so both `bnto-wasm` (browser) and `bnto` (native binary) share the same processor wiring without duplicating registration code.

## Exports

| Function                    | What It Does                                                                     |
| --------------------------- | -------------------------------------------------------------------------------- |
| `create_registry()`         | Returns a `NodeRegistry` with all processors (browser + native with `native` ff) |
| `create_browser_registry()` | Returns a `NodeRegistry` with only browser-safe (WASM-capable) processors        |
| `run_pipeline()`            | Parses JSON, creates full registry, executes pipeline in one call                |

## Registered Processors

| Node Type             | Crate              | Processor            |
| --------------------- | ------------------ | -------------------- |
| `image-compress`      | `bnto-image`       | `CompressImages`     |
| `image-resize`        | `bnto-image`       | `ResizeImages`       |
| `image-convert`       | `bnto-image`       | `ConvertImageFormat` |
| `spreadsheet-clean`   | `bnto-spreadsheet` | `CleanSpreadsheet`   |
| `spreadsheet-rename`  | `bnto-spreadsheet` | `RenameColumns`      |
| `spreadsheet-convert` | `bnto-spreadsheet` | `ConvertFormat`      |
| `spreadsheet-merge`   | `bnto-spreadsheet` | `MergeSpreadsheets`  |
| `file-rename`         | `bnto-file`        | `RenameFiles`        |
| `image-strip-exif`    | `bnto-image`       | `StripExif`          |
| `image-overlay`       | `bnto-image`       | `OverlayImage`       |
| `vector-rasterize`    | `bnto-vector`      | `VectorRasterize`    |
| `vector-optimize`     | `bnto-vector`      | `OptimizeSvg`        |
| `file-filter`         | `bnto-file`        | `FileFilter`         |
| `file-metadata`       | `bnto-file`        | `FileMetadata`       |
| `file-collect`        | `bnto-file`        | `FileCollect`        |
| `file-copy`           | `bnto-file`        | `FileCopy`           |
| `spreadsheet-read`    | `bnto-spreadsheet` | `ReadSpreadsheet`    |
| `shell-command`       | `bnto-shell`       | `ShellCommand`       |

## Development

```bash
cargo test -p bnto-engine      # Unit tests (registry, pipeline, generated recipes)
```
