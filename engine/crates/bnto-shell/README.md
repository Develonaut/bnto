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

A command that exits non-zero fails the step with `Processing failed: Command '<cmd>' failed (exit <code>): <stderr>`, but still hands over what it made (`BntoError::with_output`): its stdout in stdout mode (same name and metadata as on success, within `maxOutputSize`), or the files it wrote in file mode. A recipe's closing steps (nodes marked `"always": true`) receive them, so a tool's log survives the tool failing. A step that fails before the command starts (a refused command, a tool missing from `search_path`) hands over nothing.

## Security

All commands pass through `validate.rs` before execution:

- **Shell denylist**: bash, sh, zsh, powershell, and other shell interpreters are rejected. Commands must be bare binary names (e.g., `ffmpeg`, not `bash -c "ffmpeg ..."`).
- **Path validation**: absolute paths, relative paths, and parent traversal are rejected. Commands are resolved via PATH by the OS.
- **`search_path`** (optional): a list of folders to find the command in, in order, instead of PATH. The command is still a bare, validated name, the shell denylist still applies, and PATH is not consulted as a fallback, so a wrong folder fails naming the folders. The command runs by its resolved full path and the child's environment (PATH included) is unchanged. Relative folders are under the working directory. The consent prompt shows the folders as written (`Build.bat (from {{env.ROOT}}/bin)`), so approving a recipe approves where its tools come from.
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
├── lib.rs           # Public exports
├── execute.rs       # ShellCommand processor (param parsing, mode dispatch)
├── args.rs          # Conditional arg groups + {{url}}/{{input}} placeholders
├── stdout_mode.rs   # "stdout" output mode
├── file_mode.rs     # "file" output mode ({{output_dir}} temp dir)
├── failing_tool.rs  # (tests) a context whose command prints, writes a log, then exits non-zero
├── output_files.rs  # Collects written files as path references
├── parameters.rs    # Node parameter schema
├── search_path.rs   # `search_path`: find a bare command in named folders, not PATH
└── validate.rs      # Security validation (command denylist, env sanitization)
```

## Development

```bash
cargo test -p bnto-shell    # Native unit tests (security + processor)
```
