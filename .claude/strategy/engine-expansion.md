# Engine Expansion Strategy

**Created:** April 2, 2026
**Status:** Proposed — Sprint 9 (M3)
**Related:** [ROADMAP.md](../ROADMAP.md), [PLAN.md](../PLAN.md), [engine-node-patterns.md](../rules/engine-node-patterns.md)

---

## Context

Bnto has reached a fork after v0.2.0 (14 recipes, schema-driven config, editor reconnect). The founder's energy is in the CLI/Rust/TUI space, not more browser/React work. The current web recipes are a delivered asset — they work, they're indexed, they serve users. But the next interesting work is in the engine.

**The pivot:** Re-orient bnto as a CLI/TUI tool for developers. The web/browser recipes are a bonus showcase, not the primary product. Desktop (Tauri) deprioritized in favor of enriching the engine and CLI directly.

---

## Dependency System

Node types that require external tools (yt-dlp, ffmpeg, imagemagick) need a way to declare and verify those dependencies before pipeline execution.

### Design

```rust
/// A dependency on an external binary.
pub struct Dependency {
    /// Binary name (e.g., "yt-dlp", "ffmpeg").
    pub binary: String,
    /// Semver constraint (e.g., ">=2023.0.0"). Empty = any version.
    pub version_constraint: String,
    /// Human-readable install hint (e.g., "brew install yt-dlp").
    pub install_hint: String,
    /// Homepage URL for the tool.
    pub homepage: String,
}
```

Each processor's `metadata()` gains a `requires: Vec<Dependency>` field. Browser-only processors return an empty vec.

### Dependency Checker

Before pipeline execution, the engine checks all required binaries:

```
bnto run download-video.bnto.json
  → Engine reads definition → finds video-download node
  → Checks: is `yt-dlp` in PATH? Version satisfies constraint?
  → Missing? Print install hint and exit with clear error
  → Present? Proceed with pipeline
```

### `bnto doctor`

A diagnostic command that checks all dependencies for all registered processors:

```
$ bnto doctor
Checking dependencies...

  yt-dlp    ✓ 2024.1.0  (>= 2023.0.0)
  ffmpeg    ✗ not found  — brew install ffmpeg (https://ffmpeg.org)
  magick    ✓ 7.1.0     (>= 7.0.0)

1 missing dependency. Install it and try again.
```

---

## ProcessContext

A trait giving processors controlled system access. This is the bridge between pure WASM processors (no system access) and CLI/desktop processors (full system access).

### Design

```rust
/// Controlled system access for processors that need it.
pub trait ProcessContext: Send + Sync {
    /// Run an external command, capturing stdout.
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>>;

    /// Create a temporary file, returning its path.
    fn temp_file(&self, suffix: &str) -> Result<PathBuf>;

    /// Read an environment variable.
    fn env_var(&self, key: &str) -> Option<String>;

    /// Get the working directory for this execution.
    fn work_dir(&self) -> &Path;
}
```

### Implementations

| Target  | Implementation     | System access                          |
| ------- | ------------------ | -------------------------------------- |
| Browser | `NoopContext`      | All methods return `Err`. Pure WASM.   |
| CLI     | `NativeContext`    | Full system access via `std::process`. |
| Desktop | `SandboxedContext` | Scoped to user-approved directories.   |

### Processor Integration

The `NodeProcessor::process()` signature gains an optional context parameter:

```rust
fn process(
    &self,
    input: NodeInput,
    params: &serde_json::Value,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput>;
```

Existing browser-only processors ignore the context. New CLI-only processors use it to run external commands.

---

## Node Taxonomy

With `platforms` now a first-class array (instead of a lossy `browserCapable` boolean), nodes are classified by where they can run:

| Platform combination           | Example nodes                               | Availability         |
| ------------------------------ | ------------------------------------------- | -------------------- |
| `["browser"]`                  | image-compress, csv-clean, file-rename      | Browser + CLI + all  |
| `["browser", "server"]`        | http-request (CORS in browser, full server) | All targets          |
| `["cli", "server", "desktop"]` | video-download (needs yt-dlp)               | CLI, server, desktop |
| `["cli", "desktop"]`           | shell-command (local execution only)        | CLI, desktop         |
| `["server"]`                   | ai-inference (needs API key proxy)          | Server only (Pro)    |

**Key insight:** `"browser"` implies the node can run everywhere (WASM compiles to all targets). Non-browser nodes need specific runtimes. The `platforms` array is the source of truth for filtering across all surfaces (editor palette, recipe capability checks, CLI availability).

---

## Video Node Type (`bnto-video`)

The first non-browser node type. A new crate wrapping yt-dlp for video downloads.

### Processor: `video-download`

```rust
// Node type: "video-download"
// Category: "video"
// Platforms: ["cli", "server", "desktop"]
// Requires: [{ binary: "yt-dlp", ... }]

parameters: {
    url: String,           // Video URL
    format: "mp4" | "webm" | "mkv" | "mp3",
    quality: "best" | "720p" | "480p" | "audio-only",
    output_template: String,  // Filename template
}
```

### Recipe: `download-video`

```json
{
  "id": "download-video",
  "type": "group",
  "nodes": [
    { "id": "input", "type": "input", "parameters": { "mode": "url" } },
    {
      "id": "download",
      "type": "video-download",
      "parameters": { "format": "mp4", "quality": "best" }
    },
    { "id": "output", "type": "output", "parameters": { "mode": "download" } }
  ]
}
```

This recipe uses `input.mode: "url"` — the first non-file-upload input mode. The CLI prompts for a URL; the browser (if ever surfaced) shows a URL input field.

### Test Strategy

- Unit tests with small test fixtures (short video clips or mocked yt-dlp output)
- Golden tests verifying deterministic output format
- Integration test confirming `bnto run download-video` works end-to-end

---

## TUI

Interactive terminal UI for browsing and running recipes. Likely built with `ratatui`.

### Entry Points

```bash
bnto tui           # Launch interactive TUI
bnto --tui         # Alternative flag
```

### Features

- Recipe browser (list all available recipes, search, filter by category)
- File picker (navigate filesystem, select input files)
- Progress display (per-node progress bars, file throughput)
- Results view (output files, sizes, timing)

### Design Principles

- TUI is a convenience layer over the same engine
- All TUI operations can be done via CLI flags instead
- TUI uses the same `PipelineExecutor` and progress events as CLI

---

## CLI Polish

Enhance the existing CLI to be a first-class developer tool.

### Commands

| Command                          | Purpose                                           |
| -------------------------------- | ------------------------------------------------- |
| `bnto run <recipe> [files...]`   | Run a recipe (enhanced with progress bars, color) |
| `bnto list`                      | List available recipes with descriptions          |
| `bnto info <recipe>`             | Show recipe details, required dependencies        |
| `bnto doctor`                    | Check all dependencies, report missing            |
| `bnto tui`                       | Launch interactive TUI                            |
| `bnto validate <file.bnto.json>` | Validate a recipe definition                      |

### Output Improvements

- Colored output (category-tinted node names)
- Progress bars per file (not just per node)
- Timing summary at end
- Machine-readable JSON output via `--json` flag

---

## Development Workflow

The engine-first development workflow for new capabilities:

```
1. Define processor in Rust (TDD-first)
     └─ engine/crates/bnto-{crate}/src/{processor}.rs
     └─ Unit tests, golden tests, parameter contract

2. Register in engine
     └─ bnto-engine/src/lib.rs — create_default_registry()
     └─ bnto-core/src/metadata.rs — NodeTypeInfo

3. Test via CLI
     └─ bnto run <recipe> <files>
     └─ Prove it works end-to-end

4. Codegen to TypeScript
     └─ task wasm:codegen
     └─ Verify platforms array propagates correctly

5. Decide surface
     └─ Browser-capable? → Appears on bnto.io automatically
     └─ CLI-only? → Appears in `bnto list`, TUI, not browser
     └─ Server? → Future Pro tier
```

This workflow ensures every capability is engine-tested before any UI work happens.

---

## What Does NOT Change

- The website stays as-is (no UI redesign)
- All 14 browser recipes continue working
- The editor stays as lightweight open+export
- The `@bnto/ui` design system stays as-is
- The `@bnto/nodes` codegen pipeline stays as-is (enhanced with `platforms`)
- Private business docs stay as-is (revisit later)

---

## Open Questions

1. **Should `bnto-video` be a separate crate or part of an existing one?** Recommendation: separate crate (`engine/crates/bnto-video/`). Each domain gets its own crate.

2. **TUI framework choice?** `ratatui` is the standard. `crossterm` for terminal abstraction. Evaluate when we get there.

3. **Should `ProcessContext` be on every `process()` call or only for nodes that declare dependencies?** Recommendation: every call (with `NoopContext` for browser). Simpler API surface, no conditional typing.
