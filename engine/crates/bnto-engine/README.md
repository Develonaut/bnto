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

| Node Type             | Crate        | Processor            |
| --------------------- | ------------ | -------------------- |
| `image-compress`      | `bnto-image` | `CompressImages`     |
| `image-resize`        | `bnto-image` | `ResizeImages`       |
| `image-convert`       | `bnto-image` | `ConvertImageFormat` |
| `spreadsheet-clean`   | `bnto-csv`   | `CleanCsv`           |
| `spreadsheet-rename`  | `bnto-csv`   | `RenameCsvColumns`   |
| `spreadsheet-convert` | `bnto-csv`   | `CsvToJson`          |
| `spreadsheet-merge`   | `bnto-csv`   | `MergeCsv`           |
| `file-rename`         | `bnto-file`  | `RenameFiles`        |
| `image-strip-exif`    | `bnto-image` | `StripExif`          |
| `image-overlay`       | `bnto-image` | `OverlayImage`       |
| `video-download`      | `bnto-video` | `VideoDownload`      |

## Development

```bash
cargo test -p bnto-engine      # Unit tests (registry, pipeline, generated recipes)
```
