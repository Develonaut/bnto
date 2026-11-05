//! Whether a file name joined onto a folder would land outside it.
//!
//! Names travel between nodes as `/`-separated relative paths, and a recipe
//! builds them (`file-render`'s `filename`, a `file-rename` prefix or
//! replace), so a node that joins one onto a folder checks it first (ISS-128).
//! `/` and `\` both separate, and a drive letter counts, on every OS, so a
//! recipe means the same thing wherever it runs.

use std::path::Path;

/// Why a name is refused, for messages: "\"../x\" would leave … (absolute,
/// a drive, or \"..\")".
pub const LEAVES_FOLDER_WHY: &str = "absolute, a drive, or \"..\"";

/// True when `name`, joined onto a folder, would land outside it: it starts
/// at a root (`/x`, `\x`, which Windows reads as the drive's root), names a
/// drive (`C:`, `C:\x`, `\\server\share`), or has a `..` segment. `.` and
/// empty segments stay in the folder, so `./a.txt` (as `find` prints it)
/// passes.
pub fn leaves_folder(name: &str) -> bool {
    is_absolute_anywhere(name) || name.split(['/', '\\']).any(|s| s == "..")
}

/// True when some OS reads `value` as starting at a root or a drive: `/x`,
/// `\x`, `C:x`, `C:\x` or `\\server\share`, whichever OS runs.
pub fn is_absolute_anywhere(value: &str) -> bool {
    let bytes = value.as_bytes();
    let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    value.starts_with(['/', '\\']) || drive || Path::new(value).is_absolute()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_relative_name_stays() {
        for name in [
            "a.txt",
            "sub/b.txt",
            "sub\\b.txt",
            "./a.txt",
            "a/./b",
            "..hidden",
            "a..b/c",
        ] {
            assert!(!leaves_folder(name), "{name}");
        }
    }

    #[test]
    fn a_climb_leaves() {
        for name in ["..", "../x", "a/../../x", "a\\..\\x", "x/.."] {
            assert!(leaves_folder(name), "{name}");
        }
    }

    #[test]
    fn a_root_or_drive_leaves() {
        for name in [
            "/etc/x",
            "\\x",
            "\\\\server\\share\\x",
            "C:x",
            "c:\\x",
            "D:/x",
        ] {
            assert!(leaves_folder(name), "{name}");
        }
    }

    #[test]
    fn a_git_segment_alone_is_not_refused_here() {
        // `.git/...` is refused for project recipes only (step 7k).
        assert!(!leaves_folder(".git/hooks/post-checkout"));
    }
}
