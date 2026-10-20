// file-filter's name pattern, compiled once and reused while it stays the
// same. The executor calls file-filter once per file, so compiling on every
// call would build a regex for every file in the run.

use std::cell::RefCell;

use regex::Regex;

use crate::glob::Glob;

/// A compiled `namePattern`.
enum Compiled {
    Glob(Glob),
    /// `None` for a regex that doesn't compile: it matches nothing.
    Regex(Option<Regex>),
}

impl Compiled {
    fn new(pattern: &str, regex: bool) -> Self {
        if regex {
            Self::Regex(Regex::new(pattern).ok())
        } else {
            Self::Glob(Glob::new(pattern))
        }
    }

    fn matches(&self, target: &str) -> bool {
        match self {
            Self::Glob(glob) => glob.matches(target),
            Self::Regex(regex) => regex.as_ref().is_some_and(|re| re.is_match(target)),
        }
    }
}

/// A pattern as written, with what it compiled to.
struct Entry {
    pattern: String,
    regex: bool,
    compiled: Compiled,
}

/// The last name pattern compiled. A run's files share one pattern unless
/// it reads `{{item.*}}`, so keeping the last one covers the common case.
#[derive(Default)]
pub(super) struct PatternCache {
    last: RefCell<Option<Entry>>,
}

impl PatternCache {
    /// Whether `target` matches `pattern` (a regex when `regex`, else a
    /// glob), compiling it only when it differs from the last one.
    pub(super) fn matches(&self, pattern: &str, regex: bool, target: &str) -> bool {
        let mut last = self.last.borrow_mut();
        let cached = last
            .as_ref()
            .is_some_and(|entry| entry.pattern == pattern && entry.regex == regex);
        if !cached {
            *last = Some(Entry {
                pattern: pattern.to_string(),
                regex,
                compiled: Compiled::new(pattern, regex),
            });
        }
        last.as_ref()
            .is_some_and(|entry| entry.compiled.matches(target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A cache whose last entry claims `pattern` but holds `compiled`, so a
    /// match shows whether the cache reused it or compiled afresh.
    fn seeded(pattern: &str, regex: bool, compiled: Compiled) -> PatternCache {
        let cache = PatternCache::default();
        *cache.last.borrow_mut() = Some(Entry {
            pattern: pattern.to_string(),
            regex,
            compiled,
        });
        cache
    }

    #[test]
    fn reuses_the_last_compile_for_the_same_pattern() {
        let cache = seeded("*.txt", false, Compiled::new("*.md", false));
        assert!(cache.matches("*.txt", false, "a.md"));
        assert!(!cache.matches("*.txt", false, "a.txt"));
    }

    #[test]
    fn compiles_again_when_the_pattern_changes() {
        let cache = seeded("*.txt", false, Compiled::new("*.md", false));
        assert!(cache.matches("*.png", false, "a.png"));
        assert!(!cache.matches("*.png", false, "a.md"));
    }

    #[test]
    fn compiles_again_when_the_mode_changes() {
        let cache = seeded("a.b", false, Compiled::new("a.b", false));
        assert!(
            cache.matches("a.b", true, "axb"),
            "regex `.` is any character"
        );
        assert!(!cache.matches("a.b", false, "axb"), "glob `.` is literal");
    }

    #[test]
    fn a_regex_that_does_not_compile_matches_nothing() {
        let cache = PatternCache::default();
        assert!(!cache.matches("[bad", true, "[bad"));
        assert!(!cache.matches("[bad", true, ""));
    }
}
