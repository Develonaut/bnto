//! Relative paths as output-file names, with `/` separators on every OS.
//!
//! File names travel between nodes and are split on `/` downstream, so a
//! name built from a Windows path must not carry `\`.

use std::path::{Component, Path};

/// Join a relative path's components with `/`, whatever the host separator.
///
/// A root or drive/UNC prefix is dropped, so an absolute path that slips
/// through (e.g. a failed `strip_prefix` fallback) still yields a relative
/// name instead of one that escapes the output directory when joined.
pub fn slash_path(rel: &Path) -> String {
    rel.components()
        .filter(|c| !matches!(c, Component::RootDir | Component::Prefix(_)))
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_path_joins_with_slash() {
        let rel = Path::new("a").join("b").join("deep.mp4");
        assert_eq!(slash_path(&rel), "a/b/deep.mp4");
    }

    #[test]
    fn single_name_is_unchanged() {
        assert_eq!(slash_path(Path::new("video.mp4")), "video.mp4");
    }

    #[test]
    fn empty_path_is_empty() {
        assert_eq!(slash_path(Path::new("")), "");
    }

    #[test]
    fn absolute_path_drops_root() {
        // A leading "/" would make the name absolute ("//a/b.txt") and let a
        // downstream join escape the output directory.
        let abs = Path::new(std::path::MAIN_SEPARATOR_STR)
            .join("a")
            .join("b.txt");
        assert_eq!(slash_path(&abs), "a/b.txt");
    }

    #[cfg(windows)]
    #[test]
    fn windows_drive_prefix_is_dropped() {
        assert_eq!(slash_path(Path::new(r"C:\a\b.txt")), "a/b.txt");
        assert_eq!(slash_path(Path::new(r"\\server\share\a.txt")), "a.txt");
    }
}
