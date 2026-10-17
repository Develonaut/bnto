// Path list entries — split a list's text and check each path stays under the base.

use std::collections::HashSet;

/// A list's entries: NUL-separated when it holds a NUL (`git ls-files -z`,
/// `find -print0`), else one per line. Empty entries drop out.
pub(super) fn split_list(text: &str) -> Vec<String> {
    let entries: Vec<&str> = if text.contains('\0') {
        text.split('\0').collect()
    } else {
        text.lines().collect()
    };
    entries
        .into_iter()
        .map(|e| e.trim_end_matches('\r'))
        .filter(|e| !e.is_empty())
        .map(str::to_string)
        .collect()
}

/// Each path with `/` separators, first occurrence kept; or every path that
/// would leave the base (absolute, a drive letter, or `..`).
pub(super) fn checked_names(raw: Vec<String>) -> Result<Vec<String>, Vec<String>> {
    let mut seen = HashSet::new();
    let mut names = Vec::new();
    let mut refused = Vec::new();
    for path in raw {
        if escapes_base(&path) {
            refused.push(path);
        } else {
            let name = path.replace('\\', "/");
            if seen.insert(name.clone()) {
                names.push(name);
            }
        }
    }
    if refused.is_empty() {
        Ok(names)
    } else {
        Err(refused)
    }
}

/// Absolute paths, drive letters (on every OS, so a list means the same
/// thing wherever it runs) and `..` segments all leave the base.
fn escapes_base(path: &str) -> bool {
    let bytes = path.as_bytes();
    let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    path.starts_with(['/', '\\'])
        || drive
        || std::path::Path::new(path).is_absolute()
        || path.split(['/', '\\']).any(|s| s == "..")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nul_wins_over_newlines() {
        assert_eq!(split_list("a\nb\0c"), ["a\nb", "c"]);
    }

    #[test]
    fn a_dotted_name_is_not_a_climb() {
        assert!(checked_names(vec!["..hidden".into(), "a..b/c".into()]).is_ok());
    }
}
