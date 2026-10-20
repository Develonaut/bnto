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
| `image-pad`           | `bnto-image`       | `PadImages`          |
| `vector-rasterize`    | `bnto-vector`      | `VectorRasterize`    |
| `vector-optimize`     | `bnto-vector`      | `OptimizeSvg`        |
| `assert`              | `bnto-core`        | `control::Assert`    |
| `report`              | `bnto-core`        | `control::Report`    |
| `edit-fields`         | `bnto-core`        | `data::EditFields`   |
| `summarize`           | `bnto-core`        | `data::Summarize`    |
| `file-decode`         | `bnto-file`        | `FileDecode`         |
| `file-filter`         | `bnto-file`        | `FileFilter`         |
| `file-inspect`        | `bnto-file`        | `FileInspect`        |
| `file-join`           | `bnto-file`        | `FileJoin`           |
| `file-render`         | `bnto-file`        | `FileRender`         |
| `file-sort`           | `bnto-file`        | `FileSort`           |
| `file-collect`        | `bnto-file`        | `FileCollect`        |
| `file-copy`           | `bnto-file`        | `FileCopy`           |
| `file-delete`         | `bnto-file`        | `FileDelete`         |
| `file-list`           | `bnto-file`        | `FileList`           |
| `file-move`           | `bnto-file`        | `FileMove`           |
| `spreadsheet-export`  | `bnto-spreadsheet` | `SpreadsheetExport`  |
| `spreadsheet-lookup`  | `bnto-spreadsheet` | `SpreadsheetLookup`  |
| `spreadsheet-read`    | `bnto-spreadsheet` | `ReadSpreadsheet`    |
| `shell-command`       | `bnto-shell`       | `ShellCommand`       |

## Development

```bash
cargo test -p bnto-engine      # Unit tests (registry, pipeline, generated recipes)
```
