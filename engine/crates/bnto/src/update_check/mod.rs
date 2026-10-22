// The update check: is a newer bnto on crates.io? Results are cached for a
// day; a stale cache is refreshed by a detached `bnto` process so no command
// waits on the network (.claude/strategy/cli-upgrade.md).

mod cache;
mod fetch;
mod index;
mod notice;
mod policy;
mod spawn;
mod version;
mod watch;

pub use version::Version;

use std::io::IsTerminal;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::storage::BntoPaths;
use crate::storage::config::TomlConfig;

/// Before a command runs: print the notice when a newer bnto is cached, and
/// start a background refresh when the cache is stale. Does nothing when
/// checks are off (opted out, CI, or stderr isn't a terminal).
pub fn notice_and_refresh(paths: &BntoPaths) {
    if !policy::checks_enabled(context(&TomlConfig::load(paths))) {
        return;
    }
    let Some(current) = version::Version::current() else {
        return;
    };
    let cached = cache::load(&cache_path(paths));
    if let Some(latest) = cache::update_from(cached.as_ref(), current) {
        eprintln!("{}", notice::notice(current, latest));
    }
    if cache::needs_check(cached.as_ref(), now()) {
        spawn::spawn_refresh();
    }
}

/// For the TUI: the notice from the cache now, and a channel that delivers
/// a notice whenever a newer release turns up while the TUI is open (an
/// hourly look on a background thread; see `watch.rs`).
pub fn for_tui(paths: &BntoPaths) -> (Option<String>, Option<mpsc::Receiver<String>>) {
    if !policy::tui_checks_enabled(context(&TomlConfig::load(paths))) {
        return (None, None);
    }
    let Some(current) = version::Version::current() else {
        return (None, None);
    };
    let path = cache_path(paths);
    let shown = cache::update_from(cache::load(&path).as_ref(), current);
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || watch::run(&path, current, shown, tx));
    (shown.map(notice::tui_notice), Some(rx))
}

/// The hidden `__check-for-updates` command: check once and cache the
/// result. Offline or on error, nothing is written and the next run retries.
pub fn run_check(paths: &BntoPaths) {
    if let Some(latest) = latest_release() {
        record(paths, latest);
    }
}

/// Ask for the newest release now, skipping the cache.
pub fn latest_release() -> Option<Version> {
    fetch::fetch_latest()
}

/// Cache `latest` as just checked, so the notice reflects it.
pub fn record(paths: &BntoPaths, latest: Version) {
    let result = cache::CheckResult {
        checked_at: now(),
        latest: latest.to_string(),
    };
    let _ = cache::save(&cache_path(paths), &result);
}

/// Where the last check's result lives.
fn cache_path(paths: &BntoPaths) -> PathBuf {
    paths.cache_dir().join("update-check.json")
}

fn context(config: &TomlConfig) -> policy::Context {
    let env = std::env::var("BNTO_NO_UPDATE_CHECK").ok();
    policy::Context {
        config_allows: config.updates.check,
        env_opted_out: policy::env_opts_out(env.as_deref()),
        in_ci: crate::telemetry::ci::is_ci(),
        stderr_is_terminal: std::io::stderr().is_terminal(),
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
