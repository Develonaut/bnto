// A file's stamp: its size and modified time. One file has one stamp, whatever
// path reaches it, so code that looks for "the same file under another name"
// compares stamps first and runs the identity check (`same_file`) only on
// matches. Shared by the executor's held files and bnto-file's readers.

use std::path::Path;
use std::time::SystemTime;

/// Size and modified time of a file.
pub type Stamp = (u64, Option<SystemTime>);

/// The stamp of the file at `path`, following links; `None` when nothing is there.
pub fn stamp(path: &Path) -> Option<Stamp> {
    let meta = std::fs::metadata(path).ok()?;
    Some((meta.len(), meta.modified().ok()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_file_and_a_hard_link_to_it_share_a_stamp() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        std::fs::write(&file, "AAAA").unwrap();
        let link = dir.path().join("link.txt");
        std::fs::hard_link(&file, &link).unwrap();
        assert_eq!(stamp(&file), stamp(&link));
        assert_eq!(stamp(&file).unwrap().0, 4);
    }

    #[test]
    fn test_a_missing_file_has_no_stamp() {
        let dir = tempfile::tempdir().unwrap();
        assert!(stamp(&dir.path().join("missing.txt")).is_none());
    }
}
