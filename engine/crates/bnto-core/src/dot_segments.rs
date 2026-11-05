//! A file name less its `.` and empty segments, so two spellings of one
//! file (`./a.txt`, `a.txt`, `sub//a.txt`) plan as one. Names keep `.`
//! segments on purpose (`find . -print0` lists `./a`), so the places that
//! join a name onto a folder drop them first.

use std::borrow::Cow;

/// `name` without its `.` and empty segments, `/` and `\` both separating
/// (as in `leaves_folder`). Each kept segment keeps the separator written
/// before it. Check `leaves_folder` on the name as written first: a leading
/// `/` is an empty segment here, so `/x` would come back as `x`.
pub fn without_dot_segments(name: &str) -> Cow<'_, str> {
    let spare = |s: &str| s.is_empty() || s == ".";
    if !name.split(['/', '\\']).any(spare) {
        return Cow::Borrowed(name);
    }
    let mut kept = String::with_capacity(name.len());
    // Each segment paired with the separator written before it.
    let before = std::iter::once('/').chain(name.chars().filter(|c| matches!(c, '/' | '\\')));
    for (separator, segment) in before.zip(name.split(['/', '\\'])) {
        if spare(segment) {
            continue;
        }
        if !kept.is_empty() {
            kept.push(separator);
        }
        kept.push_str(segment);
    }
    Cow::Owned(kept)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_name_is_borrowed() {
        assert!(matches!(
            without_dot_segments("sub/a.txt"),
            Cow::Borrowed(_)
        ));
        assert!(matches!(without_dot_segments("..hidden"), Cow::Borrowed(_)));
    }

    #[test]
    fn dot_and_empty_segments_drop_out() {
        let cases = [
            ("./a.txt", "a.txt"),
            ("a/./b", "a/b"),
            ("sub//a.txt", "sub/a.txt"),
            ("./sub/./a.txt", "sub/a.txt"),
            ("a/b/", "a/b"),
            (".", ""),
        ];
        for (name, expected) in cases {
            assert_eq!(without_dot_segments(name), expected, "{name}");
        }
    }

    #[test]
    fn each_kept_segment_keeps_its_own_separator() {
        assert_eq!(without_dot_segments(".\\a\\b"), "a\\b");
        assert_eq!(without_dot_segments("a\\./b"), "a/b");
    }

    #[test]
    fn dotted_names_that_are_not_dot_segments_stay() {
        assert_eq!(without_dot_segments("./a..b/.env"), "a..b/.env");
    }
}
