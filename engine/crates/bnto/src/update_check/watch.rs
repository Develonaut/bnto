// The TUI's update watch: an open TUI looks again every hour, so a release
// that lands mid-session reaches the status line without a restart. A look
// reads the cache and asks the network only once the cache is a day old, so
// a TUI left open keeps the once-a-day check (.claude/strategy/cli-upgrade.md).

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use super::cache::{self, CheckResult};
use super::notice;
use super::version::Version;

/// How often an open TUI looks again. Cheap: a file read, and a network
/// request only when the cached result is older than a day.
pub const LOOK_EVERY: Duration = Duration::from_secs(60 * 60);

/// One look: refresh the cache with `fetch` when it's stale, then the release
/// to announce: newer than `current` and than the one already `shown`.
/// Offline (`fetch` gives nothing), the cache is left as it was.
pub fn look(
    cache_path: &Path,
    current: Version,
    shown: Option<Version>,
    now: u64,
    fetch: impl FnOnce() -> Option<Version>,
) -> Option<Version> {
    let mut cached = cache::load(cache_path);
    if cache::needs_check(cached.as_ref(), now)
        && let Some(latest) = fetch()
    {
        let fresh = CheckResult {
            checked_at: now,
            latest: latest.to_string(),
        };
        let _ = cache::save(cache_path, &fresh);
        cached = Some(fresh);
    }
    let latest = cache::update_from(cached.as_ref(), current)?;
    shown.is_none_or(|shown| latest > shown).then_some(latest)
}

/// Look now and then every [`LOOK_EVERY`], sending each newly found release's
/// notice, until the TUI drops its end of `tx`.
pub fn run(
    cache_path: &Path,
    current: Version,
    mut shown: Option<Version>,
    tx: mpsc::Sender<String>,
) {
    loop {
        if let Some(latest) = look(
            cache_path,
            current,
            shown,
            super::now(),
            super::latest_release,
        ) {
            if tx.send(notice::tui_notice(latest)).is_err() {
                return;
            }
            shown = Some(latest);
        }
        std::thread::sleep(LOOK_EVERY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_000_000;
    const CURRENT: Version = Version(0, 17, 1);

    fn cache_at(dir: &Path, checked_at: u64, latest: &str) -> std::path::PathBuf {
        let path = dir.join("update-check.json");
        let result = CheckResult {
            checked_at,
            latest: latest.into(),
        };
        cache::save(&path, &result).unwrap();
        path
    }

    fn no_fetch() -> Option<Version> {
        panic!("a fresh cache never asks the network")
    }

    #[test]
    fn a_stale_cache_is_refreshed_and_a_newer_release_announced() {
        let dir = tempfile::tempdir().unwrap();
        let path = cache_at(dir.path(), NOW - cache::CHECK_EVERY_SECS - 1, "0.17.1");
        let found = look(&path, CURRENT, None, NOW, || Some(Version(0, 18, 0)));
        assert_eq!(found, Some(Version(0, 18, 0)));
        let saved = cache::load(&path).unwrap();
        assert_eq!((saved.checked_at, saved.latest.as_str()), (NOW, "0.18.0"));
    }

    #[test]
    fn a_fresh_cache_is_read_without_the_network() {
        let dir = tempfile::tempdir().unwrap();
        let path = cache_at(dir.path(), NOW - 60, "0.18.0");
        assert_eq!(
            look(&path, CURRENT, None, NOW, no_fetch),
            Some(Version(0, 18, 0))
        );
    }

    #[test]
    fn a_release_found_by_another_bnto_reaches_the_open_tui() {
        // A `bnto run` in another terminal refreshed the cache after the TUI started.
        let dir = tempfile::tempdir().unwrap();
        let path = cache_at(dir.path(), NOW - 60, "0.17.1");
        assert_eq!(look(&path, CURRENT, None, NOW, no_fetch), None);
        cache_at(dir.path(), NOW, "0.18.0");
        assert_eq!(
            look(&path, CURRENT, None, NOW + 1, no_fetch),
            Some(Version(0, 18, 0))
        );
    }

    #[test]
    fn a_release_already_shown_is_not_sent_again() {
        let dir = tempfile::tempdir().unwrap();
        let path = cache_at(dir.path(), NOW - 60, "0.18.0");
        assert_eq!(
            look(&path, CURRENT, Some(Version(0, 18, 0)), NOW, no_fetch),
            None
        );
    }

    #[test]
    fn a_later_release_replaces_the_one_shown() {
        let dir = tempfile::tempdir().unwrap();
        let path = cache_at(dir.path(), NOW - 60, "0.18.1");
        let found = look(&path, CURRENT, Some(Version(0, 18, 0)), NOW, no_fetch);
        assert_eq!(found, Some(Version(0, 18, 1)));
    }

    #[test]
    fn offline_leaves_the_cache_and_announces_nothing_new() {
        let dir = tempfile::tempdir().unwrap();
        let stale = NOW - cache::CHECK_EVERY_SECS - 1;
        let path = cache_at(dir.path(), stale, "0.17.1");
        assert_eq!(look(&path, CURRENT, None, NOW, || None), None);
        assert_eq!(cache::load(&path).unwrap().checked_at, stale);
    }
}
