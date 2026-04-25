//! Built-in validators for common field constraints.
//!
//! Each validator is a function that returns a `ValidatorFn`. The simple
//! `not_empty` is a plain function pointer. Parameterized validators
//! (`min_len`, `range`, `pattern`) return boxed closures.

use std::sync::Arc;

use crate::field::ValidatorFn;

/// Reject empty strings.
pub fn not_empty(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        Some("Cannot be empty".to_string())
    } else {
        None
    }
}

/// Reject strings shorter than `n` characters.
pub fn min_len(n: usize) -> ValidatorFn {
    Arc::new(move |s: &str| {
        if s.len() < n {
            Some(format!("Must be at least {n} characters"))
        } else {
            None
        }
    })
}

/// Reject numeric values outside `[lo, hi]`.
///
/// Non-numeric strings are rejected with a parse error message.
pub fn range(lo: f64, hi: f64) -> ValidatorFn {
    Arc::new(move |s: &str| {
        let val: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => return Some("Must be a number".to_string()),
        };
        if val < lo || val > hi {
            Some(format!("Must be between {lo} and {hi}"))
        } else {
            None
        }
    })
}

/// Reject strings that don't match the given regex pattern.
///
/// Uses a simple `contains`-style match (no regex dependency).
/// For exact matching, anchor with `^` and `$` in your own validator.
pub fn pattern(pat: &str) -> ValidatorFn {
    let pat = pat.to_string();
    Arc::new(move |s: &str| {
        if s.contains(pat.as_str()) {
            None
        } else {
            Some(format!("Must match pattern: {pat}"))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- not_empty ---

    #[test]
    fn test_not_empty_rejects_empty() {
        assert!(not_empty("").is_some());
    }

    #[test]
    fn test_not_empty_rejects_whitespace() {
        assert!(not_empty("   ").is_some());
    }

    #[test]
    fn test_not_empty_accepts_value() {
        assert!(not_empty("hello").is_none());
    }

    // --- min_len ---

    #[test]
    fn test_min_len_rejects_short() {
        let v = min_len(3);
        assert!(v("ab").is_some());
    }

    #[test]
    fn test_min_len_accepts_exact() {
        let v = min_len(3);
        assert!(v("abc").is_none());
    }

    #[test]
    fn test_min_len_accepts_longer() {
        let v = min_len(3);
        assert!(v("abcd").is_none());
    }

    // --- range ---

    #[test]
    fn test_range_rejects_below() {
        let v = range(0.0, 100.0);
        assert!(v("-1").is_some());
    }

    #[test]
    fn test_range_rejects_above() {
        let v = range(0.0, 100.0);
        assert!(v("101").is_some());
    }

    #[test]
    fn test_range_accepts_within() {
        let v = range(0.0, 100.0);
        assert!(v("50").is_none());
    }

    #[test]
    fn test_range_accepts_boundaries() {
        let v = range(0.0, 100.0);
        assert!(v("0").is_none());
        assert!(v("100").is_none());
    }

    #[test]
    fn test_range_rejects_non_numeric() {
        let v = range(0.0, 100.0);
        let err = v("abc");
        assert!(err.is_some());
        assert!(err.unwrap().contains("number"));
    }

    // --- pattern ---

    #[test]
    fn test_pattern_rejects_no_match() {
        let v = pattern("hello");
        assert!(v("world").is_some());
    }

    #[test]
    fn test_pattern_accepts_match() {
        let v = pattern("hello");
        assert!(v("say hello").is_none());
    }

    #[test]
    fn test_pattern_accepts_exact() {
        let v = pattern("abc");
        assert!(v("abc").is_none());
    }
}
