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

use std::io::IsTerminal;
use std::path::PathBuf;
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

/// The hidden `__check-for-updates` command: check once and cache the
/// result. Offline or on error, nothing is written and the next run retries.
pub fn run_check(paths: &BntoPaths) {
    let Some(latest) = fetch::fetch_latest() else {
        return;
    };
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
