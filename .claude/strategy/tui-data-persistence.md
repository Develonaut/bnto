# TUI Data Persistence

**Created:** April 17, 2026
**Status:** Planning — first implementation task in next sprint
**Related:** [tui-strategy.md](tui-strategy.md), [tui-user-journey.md](tui-user-journey.md)

---

## Problem

The current TUI persistence is fragile. Config lives in a single JSON file at `dirs::config_dir()/bnto/tui.json`. Save errors are silently discarded (`let _ = config.save()`). Writes are non-atomic. There's no separation between user-editable settings, app-managed data (recipe library), and ephemeral state (history, recents). Users lose their settings between development versions.

This document defines the storage architecture that underpins the entire TUI user journey — library management, config persistence, execution history, and state recovery.

---

## Design Principles

### XDG-Compliant, CLI-Convention Paths

Follow the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/) for directory layout. On macOS, use `~/.config/` for config (CLI convention, not `~/Library/Application Support/`) because bnto is a CLI tool, not a GUI app. This matches how Helix, bat, and starship handle macOS paths.

### Separate Concerns Into Separate Directories

Different kinds of data have different lifecycles:

| Category   | Lifecycle                           | Loss impact                      | Example                           |
| ---------- | ----------------------------------- | -------------------------------- | --------------------------------- |
| **Config** | User-editable, version-controllable | High — user's preferences gone   | theme, output dir, picker path    |
| **Data**   | App-managed, persistent             | High — user's recipes gone       | recipe library (.bnto.json files) |
| **State**  | Survives restarts, OK to rebuild    | Low — convenience lost           | execution history, recent files   |
| **Cache**  | Fully disposable                    | None — regenerated automatically | (future: WASM cache, thumbnails)  |

Mixing these in one directory means a cache cleanup deletes recipes, or a config reset loses history.

### Atomic Writes, No Silent Failures

Write to a temp file, then rename. This is atomic on all filesystems — a crash mid-write never corrupts the target file. Save errors are surfaced to the user via the status bar, never silently discarded.

### TOML for Config, JSON for State

TOML is the Rust CLI ecosystem standard for user-editable config (Helix, Alacritty, Starship, Zellij). It supports comments, is human-readable, and users expect it. JSON is fine for app-managed state files that users don't hand-edit.

---

## Directory Layout

```
~/.config/bnto/                    CONFIG — user-editable
    config.toml                    theme, output dir, picker path

~/.local/share/bnto/               DATA — app-managed, persistent
    recipes/                       My Library — user's .bnto.json files
      compress-for-web.bnto.json
      my-csv-pipeline.bnto.json

~/.local/state/bnto/               STATE — survives restarts, OK to lose
    history.json                   execution history
    recent.json                    recently used recipes + directories

~/.cache/bnto/                     CACHE — fully disposable
    (future use)
```

### Platform Mapping

| Directory | Linux                  | macOS                                       | Windows                      |
| --------- | ---------------------- | ------------------------------------------- | ---------------------------- |
| Config    | `~/.config/bnto/`      | `~/.config/bnto/`                           | `%APPDATA%\bnto\`            |
| Data      | `~/.local/share/bnto/` | `~/Library/Application Support/bnto/`       | `%APPDATA%\bnto\data\`       |
| State     | `~/.local/state/bnto/` | `~/Library/Application Support/bnto/state/` | `%LOCALAPPDATA%\bnto\state\` |
| Cache     | `~/.cache/bnto/`       | `~/Library/Caches/bnto/`                    | `%LOCALAPPDATA%\bnto\cache\` |

**macOS config exception:** Config uses `~/.config/bnto/` (XDG-style) instead of `~/Library/Application Support/bnto/` because bnto is a CLI tool. This matches the convention used by Helix, bat, and starship. Data/state/cache follow Apple conventions because those directories hold app-managed artifacts the user doesn't hand-edit.

**macOS state fallback:** macOS has no native equivalent to `~/.local/state/`. We use `~/Library/Application Support/bnto/state/` as a subdirectory of the data dir. This keeps state persistent but clearly separated from the recipe library.

---

## `BntoPaths` — Centralized Path Resolution

One struct, resolved once at startup, shared everywhere. Replaces the scattered `dirs::config_dir().map(...)` calls.

```rust
/// Resolved paths for all bnto storage directories.
/// Created once at startup, passed to all subsystems.
pub struct BntoPaths {
    pub config: PathBuf,     // ~/.config/bnto/
    pub data: PathBuf,       // ~/.local/share/bnto/
    pub state: PathBuf,      // ~/.local/state/bnto/
    pub cache: PathBuf,      // ~/.cache/bnto/
}

impl BntoPaths {
    /// Resolve paths from environment and platform defaults.
    /// Priority: BNTO_HOME > XDG env vars > platform defaults.
    pub fn resolve() -> Option<Self> { ... }

    /// Path to the user's recipe library directory.
    pub fn recipes_dir(&self) -> PathBuf {
        self.data.join("recipes")
    }

    /// Path to the main config file.
    pub fn config_file(&self) -> PathBuf {
        self.config.join("config.toml")
    }

    /// Path to execution history.
    pub fn history_file(&self) -> PathBuf {
        self.state.join("history.json")
    }

    /// Path to recently used recipes/directories.
    pub fn recent_file(&self) -> PathBuf {
        self.state.join("recent.json")
    }

    /// Ensure all directories exist. Called once at startup.
    pub fn ensure_dirs(&self) -> Result<(), std::io::Error> { ... }
}
```

### Environment Variable Overrides

| Variable          | What it overrides                                         | Example                            |
| ----------------- | --------------------------------------------------------- | ---------------------------------- |
| `BNTO_HOME`       | All directories (config/data/state/cache under this root) | `BNTO_HOME=~/dotfiles/bnto`        |
| `BNTO_CONFIG_DIR` | Config directory only                                     | `BNTO_CONFIG_DIR=/tmp/bnto-config` |
| `XDG_CONFIG_HOME` | Config base (standard XDG)                                | Respected by default               |
| `XDG_DATA_HOME`   | Data base (standard XDG)                                  | Respected by default               |
| `XDG_STATE_HOME`  | State base (standard XDG)                                 | Respected by default               |
| `XDG_CACHE_HOME`  | Cache base (standard XDG)                                 | Respected by default               |

**Resolution order:**

1. `BNTO_HOME` — if set, all directories derive from `$BNTO_HOME/{config,data,state,cache}/`
2. `BNTO_CONFIG_DIR` — overrides config only (useful for testing)
3. XDG env vars — standard overrides for each category
4. Platform defaults — `dirs` crate resolution (with macOS config exception)

`BNTO_HOME` is the "escape hatch" for developers and CI. Set it once, everything goes there. Standard XDG vars are respected for users who have custom XDG setups.

---

## Config File Format

Switch from JSON to TOML. Add schema versioning for future migration.

```toml
# bnto configuration
# This file is user-editable. Changes take effect on next TUI launch.
version = 1

[tui]
theme = "tokyo"

[output]
dir = "/Users/ryan/bnto-output"

[picker]
default_path = "/Users/ryan/photos"

[telemetry]
enabled = true
```

### Schema Versioning

The `version` field is a simple integer. On load:

1. Parse the TOML
2. Read `version` field (default: 1 if missing)
3. If version < current, run migration functions in order
4. Write back the migrated config (with updated version)

Migration functions are simple: `fn migrate_v1_to_v2(config: &mut toml::Value) -> Result<()>`. Each migration is a pure function that transforms the TOML structure.

### What Goes in Config vs What's Internal

| Setting             | In config.toml? | Why                                      |
| ------------------- | --------------- | ---------------------------------------- |
| Theme               | Yes             | User preference, editable                |
| Output directory    | Yes             | User preference, editable                |
| Picker default path | Yes             | User preference, editable                |
| Telemetry consent   | Yes             | User should be able to see and change it |
| Window size         | No (state)      | Not a preference, just restored state    |
| Last-used recipe    | No (state)      | Ephemeral convenience                    |
| Execution history   | No (state)      | App-managed, not user-editable           |

---

## Atomic Writes

Every write goes through a shared `atomic_write` function:

```rust
/// Write data to a file atomically.
/// Writes to a temp file in the same directory, then renames.
/// Rename is atomic on all POSIX filesystems and NTFS.
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    let dir = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no parent")
    })?;
    std::fs::create_dir_all(dir)?;

    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;
    tmp.write_all(data)?;
    tmp.flush()?;
    tmp.persist(path)?;
    Ok(())
}
```

**Replaces:** Direct `std::fs::write()` calls. The `tempfile` crate is already a workspace dependency.

---

## Error Handling

Current code silently discards save errors:

```rust
// CURRENT — errors vanish
let _ = config.save();
```

New code surfaces errors to the TUI status bar:

```rust
// NEW — errors shown to user
match config.save(&paths) {
    Ok(()) => status_bar.set("Settings saved"),
    Err(e) => status_bar.set_error(&format!("Failed to save: {e}")),
}
```

**Policy:** Save errors are surfaced as status bar messages. They don't block the user (no modal dialogs for a save failure). The user can continue working and retry later. Config remains in memory even if disk write fails.

---

## Migration from Current Layout

On first run with the new storage layout:

1. Check for old config at `dirs::config_dir()/bnto/tui.json`
2. If found: read JSON, convert to TOML, write to new location (`~/.config/bnto/config.toml`)
3. Check for old telemetry at `dirs::config_dir()/bnto/telemetry.json`
4. If found: read JSON, merge telemetry fields into new config.toml
5. Print one-time notice: "Migrated config to ~/.config/bnto/config.toml"
6. Leave old files in place (don't delete — user might downgrade or have other tools reading them)

**Migration is best-effort.** If old files can't be read (corrupted, permissions), start fresh with defaults. No crash, no error — just a clean slate.

---

## Recipe Library Operations

The library is a directory of `.bnto.json` files. Operations are filesystem operations:

| Operation            | Implementation                                                         |
| -------------------- | ---------------------------------------------------------------------- |
| **List**             | `glob("*.bnto.json")` in recipes dir, parse name/description from each |
| **Add from catalog** | Write engine's embedded recipe JSON to `recipes/{slug}.bnto.json`      |
| **Save edits**       | Atomic write to existing `.bnto.json` file                             |
| **Rename**           | Edit `name` field in JSON, atomic write                                |
| **Delete**           | `std::fs::remove_file()` with confirmation prompt                      |
| **Duplicate**        | Copy file with `-copy` suffix                                          |

**File naming:** When adding from the catalog, the filename is the recipe slug (`compress-images.bnto.json`). When creating via wizard, auto-generate from the recipe name (`my-csv-cleaner.bnto.json` — lowercase, hyphenated). On name collision, append a number (`compress-images-2.bnto.json`).

---

## Crate Dependencies

### Keep

- `dirs` (v6) — used for platform base directory detection. Already a workspace dependency.
- `serde` + `serde_json` — serialization. Already workspace dependencies.
- `tempfile` — temp file for atomic writes. Already a workspace dependency.

### Add

- `toml` — TOML parsing/writing for config files. Standard in the Rust ecosystem.

### Considered but not needed

- `directories` / `etcetera` — project-scoped paths. We build our own `BntoPaths` with the macOS config exception, which is simpler than adapting either crate's macOS behavior.
- `figment` — layered config merging. Overkill for our needs — we have one config file and a few env vars.
- `confy` — zero-boilerplate config. Too opinionated for our custom path resolution.

---

## Implementation Tasks

See PLAN.md Sprint 12A for the task breakdown. This is the first work item in the next sprint.

**Estimated scope:** ~4 PRs, ~35 tests

| PR  | What                                                                                      | Tests |
| --- | ----------------------------------------------------------------------------------------- | ----- |
| 1   | `BntoPaths` struct + resolution + `BNTO_HOME` override + `ensure_dirs()`                  | ~10   |
| 2   | `atomic_write` + TOML config format + schema versioning                                   | ~10   |
| 3   | Migration from old JSON location + telemetry merge                                        | ~8    |
| 4   | Wire into TUI: replace `config_path()`, surface save errors, update telemetry persistence | ~7    |

---

## Resolved Decisions

1. **TOML for config, JSON for state.** TOML is the Rust CLI standard for user-editable files. JSON is fine for app-managed state.

2. **`~/.config/bnto/` on macOS for config.** CLI tools use XDG-style dotfiles, not `~/Library/Application Support/`. Data/state/cache follow Apple conventions.

3. **`BntoPaths` over `directories` crate.** The macOS config exception requires custom logic. A thin struct is simpler than wrapping a crate's behavior.

4. **Atomic writes via tempfile.** `tempfile::NamedTempFile` + `persist()` is the standard pattern. Already a dependency.

5. **Schema versioning from day one.** `version = 1` in config.toml. Migration functions are pure transforms. Prevents the "can't change config format" trap.

6. **Leave old files on migration.** Don't delete `tui.json` after migrating. Users might downgrade or have scripts reading the old location.

7. **`BNTO_HOME` as escape hatch.** One env var overrides all paths. Essential for dev/testing/CI. Standard XDG vars also respected.

8. **Error surfacing via status bar.** Save failures are shown, not silently swallowed. No modal dialogs — just a status message.
