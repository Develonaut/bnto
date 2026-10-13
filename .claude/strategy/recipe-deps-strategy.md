# Recipe-Level Dependency Declarations

**Created:** April 22, 2026
**Status:** Core shipped — recipe-level `requires`, `shell-command`, `download-video` conversion, version checks, `bnto doctor` coverage of recipe deps (#530), host-aware binary probing (#525), and per-platform install hints in `info`/`doctor`/run pre-flight (#531). Open: `bnto install` does not use per-platform hints yet (§4, §6); recipe variables (§7)
**Updated:** October 13, 2026
**Related:** [engine-expansion.md](engine-expansion.md), [engine-node-patterns.md](../scopes/rust/engine-node-patterns.md), [PLAN.md](../PLAN.md)

---

## Problem

The engine's dependency system only supports processor-level dependencies. Each processor declares `requires: Vec<Dependency>` on `NodeMetadata`, which works when a dedicated processor wraps an external tool (e.g., `video-download` processor wraps `yt-dlp`). But if a recipe uses a generic processor like `shell-command` to invoke external tools, the dependency information is lost.

```
# Today: dependency info lives on the processor
video-download processor → requires: [yt-dlp, ffmpeg]

# After conversion to connector-as-recipe:
shell-command processor → requires: []  (generic — doesn't know what you'll run)
```

The engine can't warn users about missing tools because `shell-command` itself has no external dependencies — it's the **recipe** that introduces them.

---

## Why This Matters

Every connector-as-recipe will have this problem:

| Recipe                 | External dep       | Generic node used |
| ---------------------- | ------------------ | ----------------- |
| `download-video`       | `yt-dlp`, `ffmpeg` | `shell-command`   |
| Blender rendering      | `blender`          | `shell-command`   |
| Batch video processing | `ffmpeg`           | `shell-command`   |
| Community CLI wrappers | varies             | `shell-command`   |

Without recipe-level `requires`, the connector-as-recipe architecture is blind to dependencies. Users get opaque "command not found" errors instead of actionable install hints.

---

## Current State

| Component                         | Status  | Location (symbol)                                                                                                              |
| --------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `Dependency` struct               | Shipped | `bnto-core/src/metadata.rs` — `Dependency`                                                                                     |
| Per-platform install hints        | Shipped | `bnto-core/src/metadata/install_hints.rs` — `InstallHints`, `HostPlatform`; `Dependency::host_install_hint()`                  |
| `NodeMetadata::requires`          | Shipped | `bnto-core/src/metadata.rs` — `NodeMetadata`                                                                                   |
| `PipelineDefinition.requires`     | Shipped | `bnto-core/src/pipeline.rs` — `PipelineDefinition`                                                                             |
| `collect_pipeline_dependencies()` | Shipped | `bnto-engine/src/deps.rs`                                                                                                      |
| `collect_catalog_dependencies()`  | Shipped | `bnto-engine/src/deps.rs` — processor deps plus every catalog recipe's `requires` (#530)                                       |
| `check_dependencies()`            | Shipped | `bnto-engine/src/deps.rs` — probes PATH with the host platform's lookup (#525), checks versions via `bnto_core::check_version` |
| `check_pipeline_dependencies()`   | Shipped | `bnto-engine/src/deps.rs` — run pre-flight                                                                                     |
| `bnto doctor`                     | Shipped | `bnto/src/doctor.rs` — `run_doctor`                                                                                            |
| `bnto info <recipe>`              | Shipped | `bnto/src/info.rs` — `print_dependencies`                                                                                      |
| `bnto install <recipe>`           | Shipped | `bnto/src/install.rs` — `run_install` (runs `<pm> install <binary>`; ignores `installHints`)                                   |
| `shell-command` node              | Shipped | `bnto-shell` crate — `ShellCommand`                                                                                            |

---

## Design

### 1. Add `requires` to `PipelineDefinition`

```rust
// bnto-core/src/pipeline.rs
pub struct PipelineDefinition {
    pub nodes: Vec<PipelineNode>,
    #[serde(default)]
    pub settings: Option<PipelineSettings>,
    /// Recipe-level dependencies (external tools this recipe needs).
    /// Merged with per-node processor dependencies during pre-flight check.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<Dependency>,
}
```

Recipe JSON has a top-level `requires` array. `installHint` is the generic fallback; `installHints` holds exact per-platform commands (package IDs differ between package managers), and an unset platform falls back to `installHint`. This is the shape `download-video.bnto.json` ships with:

```json
{
  "requires": [
    {
      "binary": "yt-dlp",
      "installHint": "see https://github.com/yt-dlp/yt-dlp#installation",
      "installHints": {
        "macos": "brew install yt-dlp",
        "linux": "pipx install yt-dlp",
        "windows": "winget install yt-dlp.yt-dlp"
      },
      "homepage": "https://github.com/yt-dlp/yt-dlp"
    },
    {
      "binary": "ffmpeg",
      "installHint": "see https://ffmpeg.org/download.html",
      "installHints": {
        "macos": "brew install ffmpeg",
        "linux": "sudo apt install ffmpeg",
        "windows": "winget install Gyan.FFmpeg"
      },
      "homepage": "https://ffmpeg.org"
    }
  ],
  "nodes": [...]
}
```

### 2. Update `collect_pipeline_dependencies()` to merge both sources

```rust
// bnto-engine/src/deps.rs
pub fn collect_pipeline_dependencies(
    definition: &PipelineDefinition,
    registry: &NodeRegistry,
) -> Vec<Dependency> {
    let mut seen = HashSet::new();
    let mut deps = Vec::new();

    // Recipe-level deps first
    for dep in &definition.requires {
        if seen.insert(dep.binary.clone()) {
            deps.push(dep.clone());
        }
    }

    // Then per-node processor deps (existing logic)
    collect_from_nodes(&definition.nodes, registry, &mut seen, &mut deps);
    deps
}
```

### 3. Everything downstream works unchanged

- `check_pipeline_dependencies()` — already consumes the output of `collect_pipeline_dependencies()`
- `bnto doctor` — uses `collect_catalog_dependencies()`, which merges processor metadata with the recipe-level `requires` of every catalog recipe (built-in and user library)
- `bnto info` — already calls `collect_pipeline_dependencies()`, so recipe-level deps surface automatically
- TUI detail screen — already shows deps via the info path

### 4. `shell-command` node type

A new processor enabling recipes to invoke arbitrary CLI tools.

```rust
// New crate or module in bnto-engine
// Node type: "shell-command"
// Category: System
// Platforms: ["cli", "server", "desktop"]
// Requires: [] (generic — recipe declares its own deps)

parameters: {
    command: String,    // Binary to execute (e.g., "yt-dlp")
    args: Vec<String>,  // Arguments (supports template variables)
    timeout: u64,       // Max execution time in seconds (default: 300)
    env: Map,           // Additional environment variables
}
```

Uses `ProcessContext::run_command()` — pure Rust, no shell injection. The processor splits command + args explicitly (no `sh -c`).

### 5. Convert `download-video` recipe

Rewrite `download-video.bnto.json` to use `shell-command` + recipe-level `requires`:

```json
{
  "id": "download-video",
  "requires": [
    {
      "binary": "yt-dlp",
      "installHint": "see https://github.com/yt-dlp/yt-dlp#installation",
      "installHints": { "macos": "brew install yt-dlp", "windows": "winget install yt-dlp.yt-dlp" }
    },
    {
      "binary": "ffmpeg",
      "installHint": "see https://ffmpeg.org/download.html",
      "installHints": { "macos": "brew install ffmpeg", "windows": "winget install Gyan.FFmpeg" }
    }
  ],
  "nodes": [
    { "id": "input", "type": "input", "parameters": { "mode": "url" } },
    {
      "id": "download",
      "type": "shell-command",
      "parameters": {
        "command": "yt-dlp",
        "args": ["${URL}", "-o", "${OUTPUT_DIR}/%(title)s.%(ext)s"]
      }
    },
    { "id": "output", "type": "output", "parameters": { "mode": "download" } }
  ]
}
```

Delete `bnto-video` crate entirely — no customers, no backward compat needed. Remove from workspace, `bnto-engine` Cargo.toml, registry, and feature gates.

---

## User Experience

Output below is from a macOS host. `info`, `doctor`, and the run pre-flight print the host platform's `installHints` entry (falling back to `installHint`), so the same recipe shows `winget install ...` on Windows.

```
$ bnto info download-video

Download Video (connector recipe)
Download video from URLs using yt-dlp.

  Category:   video
  Input:      url
  Nodes:      shell-command

  Dependencies:
    yt-dlp
      Install: brew install yt-dlp
    ffmpeg
      Install: brew install ffmpeg

  Run with: bnto run download-video <url>
```

```
$ bnto run download-video https://youtube.com/watch?v=xyz

Dependency requirements not met:
  - yt-dlp (install: brew install yt-dlp)
  - ffmpeg (install: brew install ffmpeg)
```

---

## Work Items — Dependency Chain

### Core (ship together in 1-2 PRs)

#### 1. Recipe-level `requires` field

- Add `requires: Vec<Dependency>` to `PipelineDefinition`
- Add `Deserialize` to `Dependency` struct
- Update `collect_pipeline_dependencies()` to merge recipe-level + node-level deps
- ~20 lines of Rust + tests
- **Tests:** Deserialization with/without requires, merge deduplication, backward compat (existing recipes still parse)

#### 2. `shell-command` node type

- New processor in `bnto-engine` (or new `bnto-shell` crate)
- Uses `ProcessContext::run_command()`
- Parameters: `command`, `args`, `timeout`, `env`
- Platforms: `["cli", "server", "desktop"]`
- **Tests:** Happy path, timeout, missing command, exit code handling, env var injection

#### 3. Convert `download-video` recipe

- Rewrite `download-video.bnto.json` to use `shell-command` + recipe-level `requires`
- Delete `bnto-video` crate entirely
- Remove from workspace, `bnto-engine/Cargo.toml`, registry, feature gates
- Update golden tests, codegen
- **Tests:** Recipe parses, deps are collected, `bnto info download-video` shows deps

### Follow-ups (ordered backlog, each unblocks the next)

#### 4. `bnto install <recipe>` command — Shipped, not hint-aware

- Auto-install recipe dependencies
- Reads `requires` from recipe definition
- Detects OS/package manager (`brew`, `apt`, `choco`, `pacman`)
- Runs install commands with user confirmation
- Falls back to manual install hints
- **Current behavior:** detects `brew`, `apt`, `pacman`, or `choco` (`bnto/src/package_manager.rs`) and runs `<pm> install <binary>` built from the binary name. It does not read `installHints`, so a package whose ID differs from its binary name fails to resolve, and `winget` (which the shipped Windows hints target, e.g. `winget install Gyan.FFmpeg`) is not a detected package manager. The host hint is printed only when no package manager is detected
- **Open:** run the host `installHints` entry (with confirmation) instead of synthesizing `<pm> install <binary>`. Also, `bnto install --all` collects processor deps only (`collect_all_dependencies`), so it misses recipe-level `requires`; `bnto doctor` uses `collect_catalog_dependencies` and does not

#### 5. Version constraint enforcement — Shipped

- `check_dependencies()` calls `bnto_core::check_version` when `Dependency.version` is non-empty: runs `<binary> --version`, parses it, validates against the constraint
- Pre-flight fails and `bnto doctor` reports `OUTDATED` when the installed version is too old

#### 6. Per-platform install hints — Shipped (display only, #531)

- `Dependency` gains optional `installHints: { macos?, linux?, windows? }` (`bnto-core/src/metadata/install_hints.rs`)
- `Dependency::host_install_hint()` picks the compile-target platform's hint, falling back to the generic `installHint` (WASM/BSD, or platform unset)
- `bnto run` pre-flight, `bnto doctor`, and `bnto info` display the host hint
- **Open:** `bnto install` does not use them yet — it still runs `<pm> install <binary>` and shows the host hint only when no package manager is detected (see §4)
- Recipe authors set exact package IDs per platform (e.g. `winget install yt-dlp.yt-dlp`), since IDs differ between package managers

#### 7. Recipe variables & template expressions

- `${NAME}` syntax in recipe parameters
- Variable declarations with types (`string`, `path`, `secret`, `select`)
- Resolution chain: CLI flags -> env vars -> secrets store -> config -> defaults -> interactive prompt
- Enables `${URL}`, `${FORMAT}`, `${QUALITY}` in download-video connector recipe

---

## Implications for Community Recipes

Recipe-level `requires` is what makes community recipes viable:

1. **Author declares deps** — Recipe JSON is self-documenting
2. **Engine validates** — Pre-flight check catches missing tools before execution
3. **User opts in** — They see what's needed before running
4. **No engine changes** — Community recipes use JSON + primitive nodes, no Rust code

Same pattern as GitHub Actions (`uses: actions/setup-node@v4`), Docker (`FROM python:3.12`), or Homebrew formulas — declare your dependencies, let the runtime resolve them.

---

## Resolved Decisions

1. **Recipe-level deps merge with node-level deps.** Not replace. A recipe that uses `shell-command` (no node deps) to call `yt-dlp` declares `requires: [yt-dlp]` at the recipe level. A recipe that uses `video-download` processor (which already declares `requires: [yt-dlp]`) doesn't need recipe-level deps — but could add them redundantly (deduplication handles it).

2. **`Dependency` struct shared between node and recipe level.** Same fields, same JSON shape, same deduplication logic. No separate "RecipeDependency" type.

3. **`shell-command` does not use `sh -c`.** Command and args are split explicitly via `ProcessContext::run_command()`. No shell injection vector. The processor validates that `command` is not empty and that the binary exists on PATH before execution.

4. **`bnto-video` crate deletion is safe.** The crate has no external consumers (not published separately). The `video-download` processor is only used by the built-in recipe. Converting the recipe to `shell-command` + recipe-level `requires` replaces all functionality.
