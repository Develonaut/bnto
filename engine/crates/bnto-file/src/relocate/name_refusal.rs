// The one check on a name joined under a folder, before anything is written
// (ISS-128): one that would leave it, one with no file name (only `.` and
// empty segments, which is the folder itself), or one with a part this OS
// can't hold. Shared by `file-copy` and the CLI's and TUI's output write.

use bnto_core::{LEAVES_FOLDER_WHY, display_text, leaves_folder, without_dot_segments};

use super::check_platform_segments;

/// Why `name` can't be joined under `folder` (as messages name it: "the
/// output folder"), or `None`. The name is escaped here, where the refusal
/// is built: the TUI splits an error into rows at each line break, so a raw
/// newline in a name could forge a row. Escaped text needs no second escape,
/// so a later pass over the whole error changes nothing.
pub fn name_refusal(name: &str, folder: &str) -> Option<String> {
    let shown = display_text(name);
    if leaves_folder(name) {
        return Some(format!(
            "\"{shown}\" would leave {folder} ({LEAVES_FOLDER_WHY})"
        ));
    }
    if without_dot_segments(name).is_empty() {
        return Some(format!("\"{shown}\" has no file name"));
    }
    let why = check_platform_segments(name).err()?;
    Some(format!("\"{shown}\": {}", display_text(&why)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_or_dotted_name_passes() {
        for name in ["a.txt", "./sub/a.txt", "sub//a.txt", "..hidden"] {
            assert_eq!(name_refusal(name, "the folder"), None, "{name}");
        }
    }

    #[test]
    fn a_name_that_leaves_names_the_folder() {
        let why = name_refusal("../x", "the destination folder").unwrap();
        assert!(why.starts_with("\"../x\" would leave the destination folder"));
    }

    #[test]
    fn a_name_of_only_dot_or_empty_segments_has_no_file_name() {
        for name in [".", "", "./.", "./", ".//."] {
            let why = name_refusal(name, "the folder").unwrap();
            assert!(why.ends_with("has no file name"), "{name:?}: {why}");
        }
    }

    #[test]
    fn a_name_is_escaped_where_the_refusal_is_built() {
        let why = name_refusal("../a\nb", "the folder").unwrap();
        assert!(why.contains("\"../a\\nb\""), "{why}");
        assert!(!why.contains('\n'), "{why}");
        let why = name_refusal("a\0\nb", "the folder").unwrap();
        assert!(!why.contains('\n') && !why.contains('\0'), "{why}");
    }
}
