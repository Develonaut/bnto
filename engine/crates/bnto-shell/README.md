# bnto-shell

Shell command execution for native targets. Runs external CLI tools via `ProcessContext::run_command()`.

## Overview

`bnto-shell` provides the `shell-command` processor — a generic `NodeProcessor` that executes external CLI tools. This enables connector-as-recipe architecture: recipes can wrap any CLI tool (ffmpeg, yt-dlp, imagemagick) without needing a dedicated Rust processor for each one.

Native-only: registered behind `#[cfg(feature = "native")]` in `bnto-engine`. Browser (WASM) gets `NoopContext` which blocks execution.

## Processors

| Processor      | Node Type       | What It Does                                                         |
| -------------- | --------------- | -------------------------------------------------------------------- |
| `ShellCommand` | `shell-command` | Execute external CLI tools with security validation and output modes  |

## Output Modes

- **`stdout`** (default): captures command stdout as a single output file
- **`file`**: runs command in a temp dir, collects written files as output. Use `{{output_dir}}` in args to inject the temp directory path. Designed for tools like yt-dlp and ffmpeg that write to disk. Output files use `FileData::Path` (zero-copy, not read into memory).

## Security

All commands pass through `validate.rs` before execution:

- **Shell denylist**: bash, sh, zsh, powershell, and other shell interpreters are rejected. Commands must be bare binary names (e.g., `ffmpeg`, not `bash -c "ffmpeg ..."`).
- **Path validation**: absolute paths, relative paths, and parent traversal are rejected. Commands are resolved via PATH by the OS.
- **Environment sanitization**: `LD_PRELOAD`, `DYLD_*`, `PATH`, `HOME`, `SHELL`, `TMPDIR` are silently stripped from recipe-provided env vars.

## Conditional Args

The `args` array supports conditional flag groups via nested arrays:

```json
"args": ["--always", ["--cookies-from-browser", "{{fields.browser}}"], "{{fields.thumbnail}}"]
```

After template resolution, empty strings drop silently, and nested arrays where any element is empty drop the entire group. This enables optional flags without recipe-level `if/else`.

## Directory Structure

```
src/
├── lib.rs        # Public exports
├── execute.rs    # ShellCommand processor + output modes
└── validate.rs   # Security validation (command denylist, env sanitization)
```

## Development

```bash
cargo test -p bnto-shell    # Native unit tests (security + processor)
```
