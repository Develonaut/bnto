//! Relative paths as output-file names, with `/` separators on every OS.
//!
//! File names travel between nodes and are split on `/` downstream, so a
//! name built from a Windows path must not carry `\`.

use std::path::Path;

/// Join a relative path's components with `/`, whatever the host separator.
pub fn slash_path(rel: &Path) -> String {
    rel.components()
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
}
