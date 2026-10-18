# bnto-file

File operation nodes. Rename, filter, decode, collect, copy, inspect, and render files.

## Overview

`bnto-file` provides `NodeProcessor` implementations for file operations. Processors either transform filenames (rename), filter the pipeline (filter), decode file content (decode), collect files from disk (collect), copy files to a destination (copy), extract metadata, or write one text file from the items (render). Runs in the browser via WASM (rename, filter, decode, metadata, render) or natively for CLI/desktop (collect, copy).

## Processors

| Processor      | Node Type       | What It Does                                                                         | Platforms       |
| -------------- | --------------- | ------------------------------------------------------------------------------------ | --------------- |
| `RenameFiles`  | `file-rename`   | Transform filenames via find/replace, case changes, prefix/suffix, pattern templates | browser, cli    |
| `FileDecode`   | `file-decode`   | Decode each file's content from a text encoding into its bytes. `encoding: base64` (default): standard alphabet, padded or not, whitespace and line breaks ignored; URL-safe (`-`, `_`) is refused for now. Keeps the input's name unless `filename` gives one (`{{name}}` = input name without its last extension, `{{ext}}` = that extension); MIME type from the output name. Records `decoded_size`. A file that isn't valid base64 fails the run, naming it | browser, cli |
| `FileFilter`   | `file-filter`   | Keep or drop files by extension, name or path pattern (glob/regex), and size          | browser, cli    |
| `FileMetadata` | `file-metadata` | Extract file metadata (size, extension, MIME type, modified time, SHA-256 hash)      | browser, cli    |
| `FileRender`   | `file-render`   | Write one in-memory text file (Markdown table, report) from a template: header, one row per item filled from `{{item.KEY}}` / `{{file}}`, footer; `{{count}}` anywhere; `escape: markdown` escapes values; a key an item lacks fails the run; `rows` in node metadata | browser, cli |
| `FileCollect`  | `file-collect`  | Traverse the directory its input references (a path reference, e.g. a folder from `file-list`, or a CLI dir) and collect files matching a glob pattern, named relative to it | cli |
| `FileCopy`     | `file-copy`     | Place output files in a destination directory; conflict: skip, overwrite, rename, newer (`newer` flags each file `copied` / `up_to_date` / `skipped_newer` and records the node's totals) | cli             |
| `FileList`     | `file-list`     | Turn path lists (input files, NUL- or newline-separated, plus a `paths` param) into path references under a base folder, each with its `full_path`; missing paths skipped and counted, and `entries` lists every path in order with its `full_path` and `status` (`found` / `missing`) | cli |
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
├── decode/           # FileDecode processor (file-decode)
│   ├── mod.rs              # NodeProcessor impl (per-file), decoded output
│   ├── params.rs           # Parameter schema + typed reads
│   ├── encoding.rs         # Encoding enum, base64 decode, error reasons
│   └── output_name.rs      # `filename` template ({{name}}, {{ext}})
├── filter.rs         # FileFilter processor + metadata
├── metadata.rs       # FileMetadata processor + metadata
├── render/           # FileRender processor (file-render)
│   ├── mod.rs              # NodeProcessor impl (batch), layout, mime, rows metadata
│   ├── params.rs           # Parameter schema + typed reads
│   ├── fill.rs             # {{item.KEY}} / {{file}} per item, {{count}}
│   └── escape.rs           # none | markdown escaping of filled values
├── collect.rs        # FileCollect processor + metadata (native-only)
├── copy.rs           # FileCopy processor + metadata (native-only)
├── copy_newer.rs     # file-copy `newer` mode: copy only when the source is newer
├── list/             # FileList processor (file-list, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), outputs + entries / missing metadata
│   ├── listed.rs           # One listed path: name, `/`-separated full path, found or missing
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
