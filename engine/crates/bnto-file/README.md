# bnto-file

File operation nodes. Rename, filter, sort, decode, collect, copy, delete, inspect, render, and join files.

## Overview

`bnto-file` provides `NodeProcessor` implementations for file operations. Processors either transform filenames (rename), filter or reorder the pipeline (filter, sort), decode file content (decode), collect files from disk (collect), copy files to a destination (copy), delete the files flowing in (delete), attach facts about each file (inspect), write one text file from the items (render), or join the files flowing in into one (join). Runs in the browser via WASM (rename, filter, sort, decode, inspect, render, join) or natively for CLI/desktop (collect, copy, delete, list, move).

## Processors

| Processor     | Node Type      | What It Does                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Platforms    |
| ------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| `RenameFiles` | `file-rename`  | Transform file names. `find`/`replace` and `pattern` see the whole relative path, folders included, so they can move a file between folders; `sanitize`, `case`, `prefix` and `suffix` change only the name after the last `/`. `pattern`'s `{{name}}` is the name after those params, and a param the pattern has no token for (`prefix` with no `{{name}}`, `extension` with no `{{ext}}`) is refused before the run                                                                                                                                                                                                                                                      | browser, cli |
| `FileDecode`  | `file-decode`  | Decode each file's content from a text encoding into its bytes. `encoding: base64` (default): standard alphabet, padded or not, whitespace and line breaks ignored; URL-safe (`-`, `_`) is refused for now. Keeps the input's name unless `filename` gives one (`{{name}}` = input name without its last extension, `{{ext}}` = that extension); MIME type from the output name. Records `decoded_size`. A file that isn't valid base64 fails the run, naming it                                                                                                                                                                                                            | browser, cli |
| `FileFilter`  | `file-filter`  | Keep or drop files by extension, name or path pattern (glob/regex), and size                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | browser, cli |
| `FileSort`    | `file-sort`    | Reorder items by metadata fields (`by: "source, -title"`, `-` = descending, later fields break ties); stable, numbers numeric, text by code point (or `case: insensitive`), missing / null / empty values last in either direction; data and metadata untouched                                                                                                                                                                                                                                                                                                                                                                                                             | browser, cli |
| `FileInspect` | `file-inspect` | Extract file metadata (folder, size, extension, MIME type, modified time, SHA-256 hash). `filename` is the item's relative name as it arrived; `folder` is its folder part with `/` separators (`a/b/c.mp4` gives `a/b`, `\` also splits), `""` for a top-level, plain or command-line name                                                                                                                                                                                                                                                                                                                                                                                 | browser, cli |
| `FileRender`  | `file-render`  | Write one in-memory text file (Markdown table, report) from a template: header, one row per item filled from `{{item.KEY}}` / `{{file}}`, footer; `{{count}}` anywhere; inside a loop a row reads the item's own metadata over the loop's item, and header, footer and filename read the loop's item; `escape: markdown` escapes values; a key an item lacks fails the run; `rows` in node metadata                                                                                                                                                                                                                                                                         | browser, cli |
| `FileJoin`    | `file-join`    | Join the files flowing in into one file named by `filename`, in arrival order (upstream steps choose and order them; files from several incoming edges arrive in edge order), `separator` (default none) between them; raw bytes; disk files stream into a temp file, never loaded whole; a disk file that's gone fails the run, naming it; zero files give an empty file; `joined` in node metadata                                                                                                                                                                                                                                                                        | browser, cli |
| `FileCollect` | `file-collect` | Traverse the directory its input references (a path reference, e.g. a folder from `file-list`, or a CLI dir) and collect files matching a glob pattern, named relative to it. Skips any `.bnto-staged-*` folder (files a CLI or TUI write set aside for a swap) and any `.bnto-rename-*` file or folder (one a move parked to break a swap), left when a run failed or was interrupted                                                                                                                                                                                                                                                                                      | cli          |
| `FileCopy`    | `file-copy`    | Place output files in a destination directory; conflict: skip, overwrite, rename, newer (`newer` flags each file `copied` / `up_to_date` / `skipped_newer` and records the node's totals). Runs as a batch: a copy that would replace a file another file of the batch is read from, or one a later step of the run still reads (another loop iteration's or branch's, `ProcessContext::held_elsewhere`), refuses the batch before it copies anything, naming any copies the run made before; `{{item.KEY}}` in params is filled per file (inside a loop, over the loop's item); once the run passes its time limit no more copies start and the run fails with the timeout | cli          |
| `FileDelete`  | `file-delete`  | Delete the files flowing in (path references, e.g. from `file-list` / `file-filter`); each must resolve under `base` (no `..` or link escapes), a folder or in-memory item refuses the whole run before anything is deleted, a missing file is skipped; a file a later step of the run still reads (another loop iteration's or branch's, `ProcessContext::held_elsewhere`) refuses the batch before anything is deleted; items pass through flagged `deleted` / `skipped`, totals in node metadata; once the run passes its time limit no more deletes start and the run fails with the timeout                                                                            | cli          |
| `FileList`    | `file-list`    | Turn path lists (input files, NUL- or newline-separated, plus a `paths` param) into path references under a base folder, each with its `full_path`; missing paths skipped and counted, and `entries` lists every path in order with its `full_path` and `status` (`found` / `missing`)                                                                                                                                                                                                                                                                                                                                                                                      | cli          |
| `FileMove`    | `file-move`    | Move files into folders under a destination by the path in each name; rename only, refuses any run that would replace a file, or move one a later step of the run still reads (another loop iteration's or branch's); once the run passes its time limit no more moves start (a swap under way finishes) and the run fails with the timeout                                                                                                                                                                                                                                                                                                                                 | cli          |

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
│   ├── naming.rs           # Pattern templates, extension override, counters
│   ├── raw_checks.rs       # validate_raw: a param the pattern has no token for
│   └── result_metadata.rs  # originalFilename/newFilename/transformsApplied
├── decode/           # FileDecode processor (file-decode)
│   ├── mod.rs              # NodeProcessor impl (per-file), decoded output
│   ├── params.rs           # Parameter schema + typed reads
│   ├── encoding.rs         # Encoding enum, base64 decode, error reasons
│   └── output_name.rs      # `filename` template ({{name}}, {{ext}})
├── glob.rs           # File-name glob, compiled once; shared by collect and filter
├── filter/           # FileFilter processor (file-filter)
│   ├── mod.rs              # NodeProcessor impl (per-file), extension / pattern / size criteria
│   ├── params.rs           # Parameter schema
│   └── pattern.rs          # namePattern compiled once, reused while it stays the same
├── inspect.rs        # FileInspect processor + metadata
├── render/           # FileRender processor (file-render)
│   ├── mod.rs              # NodeProcessor impl (batch), layout, mime, rows metadata
│   ├── params.rs           # Parameter schema + typed reads
│   ├── fill.rs             # {{item.KEY}} / {{file}} per item, {{count}}
│   └── escape.rs           # none | markdown escaping of filled values
├── sort/             # FileSort processor (file-sort)
│   ├── mod.rs              # NodeProcessor impl (batch), stable keyed sort
│   ├── params.rs           # Parameter schema + `by` / `case` parsing
│   └── key.rs              # One value's sort key; the total order over types
├── join/             # FileJoin processor (file-join)
│   ├── mod.rs              # NodeProcessor impl (batch), output file + joined count
│   ├── params.rs           # Parameter schema + typed reads
│   └── write.rs            # Missing-file check; in-memory or streamed-to-temp join
├── collect/          # FileCollect processor (file-collect, native-only)
│   ├── mod.rs              # NodeProcessor impl, parameter schema, MIME guess
│   └── walker.rs           # Directory walk; entry types from read_dir, links followed
├── copy.rs           # FileCopy processor + metadata (native-only, batch)
├── copy_batch/       # file-copy as a batch (native-only)
│   ├── mod.rs              # Per-file inputs, copy each in order
│   ├── fill.rs             # `{{item.KEY}}` filled per file, a sole placeholder keeps its type
│   ├── refusal.rs          # Refuse a copy over a file the batch or a later step reads
│   └── output.rs           # Inherited metadata, files handed over on failure, totals
├── copy_newer.rs     # file-copy `newer` mode: copy only when the source is newer
├── delete/           # FileDelete processor (file-delete, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), check-all-then-delete, outputs + counts
│   ├── params.rs           # Parameter schema + typed reads
│   └── target.rs           # Base resolution; folders, `..` and links resolved and checked
├── list/             # FileList processor (file-list, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), outputs + entries / missing metadata
│   ├── listed.rs           # One listed path: name, `/`-separated full path, found or missing
│   ├── params.rs           # Parameter schema + typed reads
│   └── entries.rs          # List splitting; refuses absolute, drive, `..`
├── move/             # FileMove processor (file-move, native-only)
│   ├── mod.rs              # NodeProcessor impl (batch), outputs
│   ├── params.rs           # Parameter schema + typed reads
│   ├── plan.rs             # Names → moves; refuses absolute, `..`, bad names
│   ├── root.rs             # Destination resolution, folder/link/drive checks
│   └── stopped.rs          # A stopped batch's error: the files under the destination (out of time or a failed rename)
├── readers.rs        # Which files a batch reads from a destination (file-copy, output write)
├── relocate/         # Conflict check, ordered rename runner (optionally checked between moves), parked-file list (file-move, output overwrite)
└── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
```

## Development

```bash
cargo test -p bnto-file           # Native unit tests
task wasm:test                    # Full WASM integration tests
```
