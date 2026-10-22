// The newest bnto on crates.io, read from its sparse index file
// (https://index.crates.io/bn/to/bnto): one JSON object per published version.

use super::version::Version;

/// One index line; other fields are ignored.
#[derive(serde::Deserialize)]
struct IndexEntry {
    vers: String,
    #[serde(default)]
    yanked: bool,
}

/// The newest stable, non-yanked version in an index file. Lines that don't
/// parse are skipped, so a format change can't break the check.
pub fn latest_in_index(index: &str) -> Option<Version> {
    index
        .lines()
        .filter_map(|line| serde_json::from_str::<IndexEntry>(line).ok())
        .filter(|entry| !entry.yanked)
        .filter_map(|entry| Version::parse(&entry.vers))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX: &str = r#"{"name":"bnto","vers":"0.15.1","yanked":false}
{"name":"bnto","vers":"0.16.1","yanked":false}
{"name":"bnto","vers":"0.17.0","yanked":true}
{"name":"bnto","vers":"0.18.0-beta.1","yanked":false}
not json
{"name":"bnto","vers":"0.9.0","yanked":false}
"#;

    #[test]
    fn the_newest_stable_unyanked_version_wins() {
        assert_eq!(latest_in_index(INDEX), Some(Version(0, 16, 1)));
    }

    #[test]
    fn an_empty_index_has_no_version() {
        assert_eq!(latest_in_index(""), None);
    }
}
