// A file-name glob, compiled once and matched against many names: every
// entry of file-collect's walk, every file through file-filter.
//
// `*` is any run of characters, `?` is one character, and everything else
// is literal. The whole name must match, case-sensitively.

use regex::Regex;

/// A compiled file-name glob.
pub(crate) enum Glob {
    /// `*`: every name, without running a regex.
    Everything,
    Regex(Regex),
    /// A pattern too big for the regex engine to build.
    Nothing,
}

impl Glob {
    pub(crate) fn new(pattern: &str) -> Self {
        if pattern == "*" {
            return Self::Everything;
        }
        match Regex::new(&glob_regex(pattern)) {
            Ok(regex) => Self::Regex(regex),
            Err(_) => Self::Nothing,
        }
    }

    pub(crate) fn matches(&self, name: &str) -> bool {
        match self {
            Self::Everything => true,
            Self::Regex(regex) => regex.is_match(name),
            Self::Nothing => false,
        }
    }
}

/// The anchored regex for `pattern`. `(?s)` lets `*` and `?` cross a
/// newline, which a Unix file name may hold.
fn glob_regex(pattern: &str) -> String {
    let mut regex = String::from("(?s)^");
    for ch in pattern.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            _ => regex.push_str(&regex::escape(ch.encode_utf8(&mut [0; 4]))),
        }
    }
    regex.push('$');
    regex
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches(pattern: &str, name: &str) -> bool {
        Glob::new(pattern).matches(name)
    }

    #[test]
    fn star_matches_any_run_of_characters() {
        assert!(matches("*.txt", "hello.txt"));
        assert!(!matches("*.txt", "hello.jpg"));
        assert!(matches("*", "anything"));
        assert!(matches("*", ""));
        assert!(matches("photo*", "photo_001.jpg"));
        assert!(matches("a*b*c", "a-b-c"));
        assert!(!matches("a*b*c", "a-c-b"));
    }

    #[test]
    fn question_matches_one_character() {
        assert!(matches("file?.txt", "file1.txt"));
        assert!(!matches("file?.txt", "file12.txt"));
        assert!(!matches("file?.txt", "file.txt"));
        assert!(
            matches("?.txt", "é.txt"),
            "? is one character, not one byte"
        );
    }

    #[test]
    fn matches_the_whole_name_case_sensitively() {
        assert!(!matches("*.txt", "a.txt.bak"));
        assert!(!matches("a.txt", "xa.txt"));
        assert!(!matches("*.TXT", "a.txt"));
    }

    #[test]
    fn treats_regex_characters_literally() {
        assert!(matches("a.b", "a.b"));
        assert!(!matches("a.b", "axb"));
        assert!(matches("c++.txt", "c++.txt"));
        assert!(!matches("c++.txt", "ccc.txt"));
        assert!(matches("photo (1).jpg", "photo (1).jpg"));
        assert!(matches("[draft].md", "[draft].md"));
        assert!(!matches("[draft].md", "d.md"));
        assert!(matches("{a,b}.txt", "{a,b}.txt"));
        assert!(!matches("{a,b}.txt", "a.txt"));
        assert!(matches("^a$|b", "^a$|b"));
        assert!(matches(r"a\b", r"a\b"));
        assert!(matches("#-&~ x.txt", "#-&~ x.txt"));
        assert!(matches("*.tar.gz", "x.tar.gz"));
    }

    #[test]
    fn star_matches_a_name_with_a_newline() {
        // Unix names may hold a newline; `*` still means every file.
        assert!(matches("*", "a\nb"));
        assert!(matches("a*b", "a\nb"));
        assert!(matches("a?b", "a\nb"));
    }

    #[test]
    fn slash_is_literal() {
        // `**` is no folder wildcard: a `/` must appear in the name.
        assert!(!matches("**/*.svg", "logo.svg"));
        assert!(!matches("sub/*.svg", "logo.svg"));
    }

    #[test]
    fn star_alone_skips_the_regex() {
        assert!(matches!(Glob::new("*"), Glob::Everything));
        assert!(matches!(Glob::new("*.md"), Glob::Regex(_)));
    }
}
