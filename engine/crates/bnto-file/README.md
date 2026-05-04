# bnto-file

File operation nodes. Rename, filter, collect, copy, and inspect files.

## Overview

`bnto-file` provides `NodeProcessor` implementations for file operations. Processors either transform filenames (rename), filter the pipeline (filter), collect files from disk (collect), copy files to a destination (copy), or extract metadata. Runs in the browser via WASM (rename, filter, metadata) or natively for CLI/desktop (collect, copy).

## Processors

| Processor      | Node Type       | What It Does                                                                         | Platforms       |
| -------------- | --------------- | ------------------------------------------------------------------------------------ | --------------- |
| `RenameFiles`  | `file-rename`   | Transform filenames via find/replace, case changes, prefix/suffix, pattern templates | browser, cli    |
| `FileFilter`   | `file-filter`   | Drop files that don't match extension, glob/regex pattern, or size criteria           | browser, cli    |
| `FileMetadata` | `file-metadata` | Extract file metadata (size, extension, MIME type, SHA-256 hash)                     | browser, cli    |
| `FileCollect`  | `file-collect`  | Traverse a directory and collect files matching a glob pattern into the pipeline      | cli             |
| `FileCopy`     | `file-copy`     | Place output files in a destination directory with conflict handling                  | cli             |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── rename.rs         # RenameFiles processor + metadata
├── filter.rs         # FileFilter processor + metadata
├── metadata.rs       # FileMetadata processor + metadata
├── collect.rs        # FileCollect processor + metadata (native-only)
├── copy.rs           # FileCopy processor + metadata (native-only)
└── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
```

## Development

```bash
cargo test -p bnto-file           # Native unit tests
task wasm:test                    # Full WASM integration tests
```
