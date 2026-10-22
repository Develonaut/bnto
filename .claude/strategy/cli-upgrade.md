# CLI Upgrade Notices + `bnto upgrade` -- 3 PRs

**Created:** October 21, 2026
**Status:** Shipped in v0.17.0 (PR 1 #693, PR 3 #695). Follow-up (October 22, 2026): an open TUI looks again every hour (`fix/tui-recheck-updates`).
**Related:** [releases.md](../scopes/process/releases.md#one-number-everywhere), [tui-data-persistence.md](tui-data-persistence.md), [environment-variables.md](../environment-variables.md)

## Context

bnto can't tell you a newer version exists, and updating means knowing to run `cargo install bnto` yourself. Ryan runs `bnto tui` all day in another terminal and only finds out about new releases by accident. Claude Code's model is the target: the CLI and TUI mention a new version, and one command installs it.

Since #677, one version number covers the release tag, crates.io and `bnto --version`, so "is there a newer bnto" means "is crates.io's newest `bnto` above `env!("CARGO_PKG_VERSION")`".

**Decided (Ryan, Oct 21):**

- The command is **`bnto upgrade`**. `update` could later mean updating recipes or the library.
- The check is **on by default with an opt-out**, like Claude Code and most CLIs.

**What changes:** a new `bnto upgrade` subcommand, a once-a-day update check with a cached result, a one-line notice in the CLI and TUI, a `[updates]` config section and a `BNTO_NO_UPDATE_CHECK` env var. **What does NOT change:** recipes, nodes, the engine, WASM and the web app. The check sends nothing about the user; it reads crates.io's public index.

## Design

| Question                            | Choice                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Where the latest version comes from | crates.io's sparse index, `https://index.crates.io/bn/to/bnto`: one JSON line per published version (`vers`, `yanked`). Small, cacheable, no API token or rate limit. The newest non-yanked, non-pre-release version wins.                                                                                                                                                                                                 |
| How often                           | At most once every 24 hours. The result is cached at `<cache_dir>/update-check.json` (`{ "checkedAt", "latest" }`) and written with `storage::atomic_write`.                                                                                                                                                                                                                                                               |
| Never slow a command                | Commands read only the cache. A stale cache is refreshed by a **detached `bnto __check-for-updates` process** (a hidden subcommand in its own process group, or with no console on Windows), which survives the command exiting. _Changed in PR 1:_ the plan was a background thread, but most commands exit (often via `process::exit`) within milliseconds, so it would rarely finish and the cache would rarely update. |
| When it's skipped                   | `BNTO_NO_UPDATE_CHECK=1`, `[updates] check = false` in `config.toml`, CI (`is_ci()`, moved out of `telemetry/` into a shared module), and stderr not being a terminal (piped or scripted runs).                                                                                                                                                                                                                            |
| CLI notice                          | Before a command runs, one line on stderr: `bnto 0.17.0 is available (you have 0.16.1). Run bnto upgrade.` Never on stdout. _Changed in PR 1:_ printing after the command doesn't work when commands end with `process::exit`. The TUI (which gets its own notice in PR 3) and `bnto upgrade` don't show it.                                                                                                               |
| `bnto upgrade`                      | When the running binary lives in cargo's bin dir (`$CARGO_HOME/bin`, default `~/.cargo/bin`), run `cargo install bnto --locked` with no shell and inherited stdio, then print old → new. Otherwise print how to upgrade for the detected install, without guessing. `--check` reports whether an upgrade exists, refreshing the cache now, and installs nothing.                                                           |
| Windows                             | A running `.exe` can't be overwritten, and `bnto upgrade` is itself running. Rename the current exe to `bnto.old.exe` (allowed while it runs), let cargo install the new one, and delete `bnto.old.exe` on the next start. Verified in Windows CI.                                                                                                                                                                         |
| TUI notice                          | Read the cache at startup, then look again every hour on a background thread for as long as the TUI is open (`update_check/watch.rs`). A look reads the cache, so it picks up another `bnto`'s check, and asks the network only once the cache is a day old. When newer, the status line's right side shows `bnto 0.17.0 available, run bnto upgrade`. No modal, no network on the render thread.                          |

---

## PR 1: Tell CLI users when a newer bnto is out

**Branch:** `feat/update-check` from `main`
**One sentence:** Check crates.io for a newer bnto at most once a day, and mention it after a command.

**What:**

- An `update_check` module in `engine/crates/bnto/src/` with:
  - index parsing
  - version comparison
  - cache read/write
  - the skip rules
  - the background refresh
  - the notice text
- The CI check moves from `telemetry/ci.rs` into a shared module that telemetry and the update check both use.
- `TomlConfig` gains an `[updates]` section (`check: bool`, default true). Because the field has a default, existing files read fine; if the config schema needs it, add a migration that moves `CURRENT_VERSION` to 4.

**Files:** ~6 new (`update_check/{mod,index,cache,notice}.rs`, shared `ci.rs`, CLI test), ~5 modified (`main.rs`/`dispatch.rs`, `storage/config.rs`, `telemetry/`, `environment-variables.md`, README install section).

**Key API:**

```rust
/// The newest stable, non-yanked version in a crates.io index file.
fn latest_in_index(index: &str) -> Option<Version>;
/// The cached newer version, if any, and whether the cache needs a refresh.
fn cached_update(cache: &Path, now: SystemTime, current: &Version) -> (Option<Version>, bool);
/// Start a background refresh when checks are on and the cache is stale.
pub fn refresh_in_background(paths: &BntoPaths, config: &TomlConfig);
/// The one-line notice, or None when bnto is current.
pub fn notice(current: &Version, latest: &Version) -> Option<String>;
```

**RED tests:**

- index parsing picks the highest version, skips yanked and pre-release versions, and ignores malformed lines
- version comparison is numeric (`0.10.0` > `0.9.0`)
- a fresh cache isn't refreshed; a 25-hour-old one is
- a missing or corrupt cache counts as stale, never an error
- each skip rule disables both the check and the notice (env, config, CI, non-TTY)
- CLI: with a temp `BNTO_HOME` whose cache says `99.0.0`, a command prints the notice on stderr and nothing extra on stdout; with `BNTO_NO_UPDATE_CHECK=1` it prints nothing
- no CLI test touches the network (seeded caches only)

**Verification:** `task wasm:lint && task cli:test`
**Count changes:** none.

## PR 2: Upgrade bnto in place with `bnto upgrade`

**Branch:** `feat/bnto-upgrade`
**One sentence:** Add a `bnto upgrade` command that installs the newest release through cargo.

**What:** It detects the install method from `std::env::current_exe()` and `CARGO_HOME`. For cargo installs it runs `cargo install bnto --locked` with fixed arguments, no shell and the user's terminal. For anything else it prints instructions. `--check` refreshes and reports without installing. It also handles the Windows rename and cleans up `bnto.old.exe` at startup.

**Files:** ~3 new (`upgrade.rs`, `upgrade/install_method.rs`, CLI test), ~3 modified (`cli.rs`, `dispatch.rs`, `main.rs` startup cleanup).

**RED tests:**

- an exe under `$CARGO_HOME/bin` (and the default `~/.cargo/bin`) counts as a cargo install; one elsewhere doesn't, and the instructions say so
- the planned command is exactly `cargo install bnto --locked`
- already up to date → "bnto 0.16.1 is the latest", exit 0, nothing installed
- `--check` with a seeded newer cache reports it and installs nothing
- Windows (cfg-gated): the running exe is renamed before installing, and a leftover `bnto.old.exe` is removed on the next start
- no test runs a real `cargo install`; a fake `cargo` on `PATH` records its args

**Verification:** `task wasm:lint && task cli:test` (Windows job in CI)
**Count changes:** none.

## PR 3: Show the upgrade notice in the TUI

**Branch:** `feat/tui-upgrade-notice`
**One sentence:** Show an available bnto upgrade in the TUI's status line.

**What:** At startup the TUI reads the cached result and starts the PR 1 background refresh. When the refresh lands, it sends a message (`tui/app/message.rs`) so the notice can appear mid-session. _Follow-up (Oct 22):_ that refresh ran once, at startup, so a TUI left open all day never saw a release that landed after it started. It now repeats hourly (`watch::run`) and sends each newer release once. The status line (`tui/render.rs` `render_status_line_with_message`) shows the hint on the right.

**RED tests:**

- the model with a newer version renders the hint
- a current version or disabled checks render none
- an arriving refresh message sets it
- rendering never touches the network

**Verification:** `task cli:test`
**Count changes:** none.

## Dependency Chain

```
PR 1 (update check + CLI notice)
|-- PR 2 (bnto upgrade)        -- uses PR 1's cache and refresh
|-- PR 3 (TUI notice)          -- uses PR 1's cache and refresh
```

PRs 2 and 3 are independent of each other.

## Journeys (add to `.claude/journeys/cli.md`)

| ID  | Journey                                                                                                                                                                            |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CU1 | A newer release exists → any CLI command ends with one stderr line naming it and `bnto upgrade`                                                                                    |
| CU2 | `BNTO_NO_UPDATE_CHECK=1`, `[updates] check = false`, CI or piped output → no notice and no network                                                                                 |
| CU3 | `bnto upgrade` on a cargo install → `cargo install bnto --locked` runs, old → new is printed                                                                                       |
| CU4 | `bnto upgrade` when current → says so, installs nothing                                                                                                                            |
| CU5 | TUI running when a release lands → status line shows the upgrade hint within the session (`watch.rs` unit tests: within a day, or within the hour once another `bnto` has checked) |
