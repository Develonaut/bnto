# bnto-file

File operation nodes. Rename, filter, collect, copy, and inspect files.

## Overview

`bnto-file` provides `NodeProcessor` implementations for file operations. Processors either transform filenames (rename), filter the pipeline (filter), collect files from disk (collect), copy files to a destination (copy), or extract metadata. Runs in the browser via WASM (rename, filter, metadata) or natively for CLI/desktop (collect, copy).

## Processors

| Processor      | Node Type       | What It Does                                                                         | Platforms       |
| -------------- | --------------- | ------------------------------------------------------------------------------------ | --------------- |
| `RenameFiles`  | `file-rename`   | Transform filenames via find/replace, case changes, prefix/suffix, pattern templates | browser, cli    |
| `FileFilter`   | `file-filter`   | Keep or drop files by extension, name or path pattern (glob/regex), and size          | browser, cli    |
| `FileMetadata` | `file-metadata` | Extract file metadata (size, extension, MIME type, modified time, SHA-256 hash)      | browser, cli    |
| `FileCollect`  | `file-collect`  | Traverse the directory its input references (a path reference, e.g. a folder from `file-list`, or a CLI dir) and collect files matching a glob pattern, named relative to it | cli |
| `FileCopy`     | `file-copy`     | Place output files in a destination directory; conflict: skip, overwrite, rename, newer | cli             |
| `FileList`     | `file-list`     | Turn path lists (input files, NUL- or newline-separated, plus a `paths` param) into path references under a base folder; missing paths skipped and counted | cli |
| `FileMove`     | `file-move`     | Move files into folders under a destination by the path in each name; rename only, refuses any run that would replace a file | cli |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── rename/           # RenameFiles processor (file-rename)
│   ├── mod.rs              # Module wiring + re-export
│   ├── processor.rs        # NodeProcessor impl, batch counter injection
│   ├── params.rs           # Parameter schema (metadata)
│   ├── param_access.rs     # Typed reads from the params map
│   ├── transform.rs        # Sanitize, find/replace, case, prefix/suffix
│   ├── naming.rs           # Stem/ext split, pattern templates, counters
│   └── result_metadata.rs  # originalFilename/newFilename/transformsApplied
├── filter.rs         # FileFilter processor + metadata
├── metadata.rs       # FileMetadata processor + metadata
├── collect.rs        # FileCollect processor + metadata (native-only)
├── copy.rs           # FileCopy processor + metadata (native-only)
├── copy_newer.rs     # file-copy `newer` mode: copy only when the source is newer
├── list/             # FileList processor (file-list, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), outputs + missing metadata
│   ├── params.rs           # Parameter schema + typed reads
│   └── entries.rs          # List splitting; refuses absolute, drive, `..`
├── move/             # FileMove processor (file-move, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), outputs
│   ├── params.rs           # Parameter schema + typed reads
│   ├── plan.rs             # Names → moves; refuses absolute, `..`, bad names
│   └── root.rs             # Destination resolution, folder/link/drive checks
├── relocate/         # Conflict check + ordered rename runner (file-move, output overwrite)
└── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
```

## Development

```bash
cargo test -p bnto-file           # Native unit tests
task wasm:test                    # Full WASM integration tests
```
