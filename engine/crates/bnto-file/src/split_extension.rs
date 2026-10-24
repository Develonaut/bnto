// Stem/extension split for item names, which may carry a relative folder path.

/// Split a name on the last dot of its last path segment: `sub/a.csv.b64`
/// gives (`sub/a.csv`, `b64`), and `v1.2/README` has no extension. A leading
/// dot (`.env`, `dir/.env`) is no extension. Names travel with `/` separators
/// on every OS (`bnto_core::slash_path`), so `/` is the only separator.
pub(crate) fn split_extension(name: &str) -> (&str, &str) {
    let segment_start = name.rfind('/').map_or(0, |i| i + 1);
    match name[segment_start..].rfind('.') {
        Some(dot) if dot > 0 => {
            let at = segment_start + dot;
            (&name[..at], &name[at + 1..])
        }
        _ => (name, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_the_last_dot() {
        assert_eq!(split_extension("photo.jpg"), ("photo", "jpg"));
    }

    #[test]
    fn multiple_dots_keep_all_but_the_last_in_the_stem() {
        assert_eq!(split_extension("archive.tar.gz"), ("archive.tar", "gz"));
    }

    #[test]
    fn no_dot_is_no_extension() {
        assert_eq!(split_extension("README"), ("README", ""));
    }

    #[test]
    fn dotfile_is_no_extension() {
        assert_eq!(split_extension(".gitignore"), (".gitignore", ""));
    }

    #[test]
    fn ignores_dots_in_folders() {
        // ISS-23: the dot in `v1.2` is the folder's, not an extension.
        assert_eq!(split_extension("v1.2/README"), ("v1.2/README", ""));
    }

    #[test]
    fn extension_in_dotted_folder() {
        assert_eq!(split_extension("v1.2/notes.txt"), ("v1.2/notes", "txt"));
    }

    #[test]
    fn dotfile_in_folder_is_no_extension() {
        assert_eq!(split_extension("dir/.env"), ("dir/.env", ""));
    }

    #[test]
    fn no_extension_in_dotted_folder() {
        assert_eq!(split_extension("a.b/c"), ("a.b/c", ""));
    }

    #[test]
    fn multiple_dots_in_folder_split_on_the_last() {
        assert_eq!(split_extension("a/b.tar.gz"), ("a/b.tar", "gz"));
    }

    #[test]
    fn nested_folders_use_the_last_segment() {
        assert_eq!(
            split_extension("a.x/b.y/c.csv.b64"),
            ("a.x/b.y/c.csv", "b64")
        );
    }
}
