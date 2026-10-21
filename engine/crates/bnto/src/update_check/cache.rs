// The update check's result, cached for a day at `<cache dir>/update-check.json`
// so commands only ever read a file, never wait on the network.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::version::Version;

/// How long a check's result is trusted before checking again.
pub const CHECK_EVERY_SECS: u64 = 24 * 60 * 60;

/// One check's result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    /// When the check ran, in Unix seconds.
    pub checked_at: u64,
    /// The newest release it found.
    pub latest: String,
}

/// Whether to check again: no result yet, a day has passed, or the result
/// claims to be from the future (the clock moved back).
pub fn needs_check(cached: Option<&CheckResult>, now: u64) -> bool {
    cached.is_none_or(|c| c.checked_at > now || now - c.checked_at > CHECK_EVERY_SECS)
}

/// The cached release, when it's newer than `current`.
pub fn update_from(cached: Option<&CheckResult>, current: Version) -> Option<Version> {
    Version::parse(&cached?.latest).filter(|latest| *latest > current)
}

/// The cached result, if there's a readable one.
pub fn load(path: &Path) -> Option<CheckResult> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// Save `result`, atomically so a reader never sees half a file.
pub fn save(path: &Path, result: &CheckResult) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string(result).map_err(std::io::Error::other)?;
    crate::storage::atomic::atomic_write(path, json.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(checked_at: u64, latest: &str) -> CheckResult {
        CheckResult {
            checked_at,
            latest: latest.into(),
        }
    }

    #[test]
    fn a_fresh_result_needs_no_check() {
        assert!(!needs_check(Some(&result(1000, "0.16.1")), 1000 + 60));
    }

    #[test]
    fn a_day_old_or_missing_result_needs_a_check() {
        assert!(needs_check(
            Some(&result(1000, "0.16.1")),
            1000 + CHECK_EVERY_SECS + 1
        ));
        assert!(needs_check(None, 1000));
    }

    #[test]
    fn a_result_from_the_future_needs_a_check() {
        // A clock set back shouldn't silence checks forever.
        assert!(needs_check(Some(&result(5000, "0.16.1")), 1000));
    }

    #[test]
    fn a_newer_cached_release_is_an_update() {
        let cached = result(1000, "0.17.0");
        assert_eq!(
            update_from(Some(&cached), Version(0, 16, 1)),
            Some(Version(0, 17, 0))
        );
        assert_eq!(update_from(Some(&cached), Version(0, 17, 0)), None);
        assert_eq!(
            update_from(Some(&result(1000, "garbage")), Version(0, 16, 1)),
            None
        );
        assert_eq!(update_from(None, Version(0, 16, 1)), None);
    }

    #[test]
    fn saves_and_loads_through_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cache").join("update-check.json");
        save(&path, &result(42, "0.17.0")).unwrap();
        assert_eq!(load(&path), Some(result(42, "0.17.0")));
    }

    #[test]
    fn a_missing_or_corrupt_file_loads_as_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("update-check.json");
        assert_eq!(load(&path), None);
        std::fs::write(&path, "{not json").unwrap();
        assert_eq!(load(&path), None);
    }
}
