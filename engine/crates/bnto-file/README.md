# bnto-file

File operation nodes. Filename transformation.

## Overview

`bnto-file` provides a `NodeProcessor` for renaming files. It transforms filenames only; file content passes through unchanged. Uses regex for pattern matching. Runs in the browser via WASM or natively for desktop/CLI.

## Processors

| Processor     | Node Type     | What It Does                                                                         |
| ------------- | ------------- | ------------------------------------------------------------------------------------ |
| `RenameFiles` | `file-rename` | Transform filenames via find/replace, case changes, prefix/suffix, pattern templates |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── rename.rs         # RenameFiles processor + metadata
└── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
```

## Development

```bash
cargo test -p bnto-file           # Native unit tests
task wasm:test                    # Full WASM integration tests
```
