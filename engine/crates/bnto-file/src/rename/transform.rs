// Stem transformations — sanitize, find/replace, case, prefix, suffix.

use regex::Regex;

use super::param_access::{get_int_param, get_string_param};

/// Apply all transformations in order: sanitize -> find/replace -> case -> prefix -> suffix.
pub(super) fn apply_all_transformations(
    original_stem: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    let mut stem = apply_sanitize(original_stem.to_string(), params);

    if let Some(find_str) = get_string_param(params, "find") {
        let replace_str = get_string_param(params, "replace").unwrap_or_default();
        stem = apply_find_replace(&stem, &find_str, &replace_str);
    }
    if let Some(case_str) = get_string_param(params, "case") {
        stem = apply_case_transform(&stem, &case_str);
    }
    if let Some(prefix) = get_string_param(params, "prefix") {
        stem = format!("{prefix}{stem}");
    }
    if let Some(suffix) = get_string_param(params, "suffix") {
        stem = format!("{stem}{suffix}");
    }
    stem
}

/// Sanitize runs first — clean the raw filename before other transforms.
/// No-op when the `sanitize` param is absent.
fn apply_sanitize(stem: String, params: &serde_json::Map<String, serde_json::Value>) -> String {
    let Some(mode) = get_string_param(params, "sanitize") else {
        return stem;
    };
    let separator = get_string_param(params, "separator").unwrap_or_else(|| "-".to_string());
    let sanitized = match mode.as_str() {
        "strip" => bnto_core::case::strip_non_ascii(&stem, &separator),
        "normalize" => bnto_core::case::normalize_unicode(&stem),
        _ => bnto_core::case::slug_with_separator(&stem, &separator),
    };
    let max_length = get_int_param(params, "maxLength").unwrap_or(0);
    apply_max_length(&sanitized, max_length)
}

/// Truncate stem to maxLength characters. 0 means no limit.
fn apply_max_length(stem: &str, max_length: usize) -> String {
    if max_length == 0 || stem.len() <= max_length {
        return stem.to_string();
    }
    stem.chars().take(max_length).collect()
}

/// Apply find/replace. Tries regex first; falls back to literal if invalid regex.
fn apply_find_replace(stem: &str, find: &str, replace: &str) -> String {
    match Regex::new(find) {
        Ok(re) => re.replace_all(stem, replace).to_string(),
        Err(_) => stem.replace(find, replace),
    }
}

/// Apply case transformation: "lower", "upper", "title". Unknown values pass through.
fn apply_case_transform(stem: &str, case: &str) -> String {
    match case {
        "lower" => bnto_core::case::lower(stem),
        "upper" => bnto_core::case::upper(stem),
        "title" => bnto_core::case::capitalize(stem),
        _ => stem.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_capitalize() {
        assert_eq!(bnto_core::case::capitalize("hello"), "Hello");
        assert_eq!(bnto_core::case::capitalize("PHOTO"), "Photo");
        assert_eq!(bnto_core::case::capitalize("hello world"), "Hello world");
        assert_eq!(bnto_core::case::capitalize(""), "");
        assert_eq!(bnto_core::case::capitalize("a"), "A");
    }

    #[test]
    fn test_apply_find_replace_literal_no_match() {
        // When the find string doesn't match anything, return unchanged.
        let result = apply_find_replace("hello", "xyz", "abc");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_apply_find_replace_all_occurrences() {
        // Regex replace_all should replace ALL occurrences, not just the first.
        let result = apply_find_replace("aaa-bbb-aaa", "aaa", "xxx");
        assert_eq!(result, "xxx-bbb-xxx");
    }

    // =========================================================================
    // Sanitize Helper Function Tests
    // =========================================================================

    #[test]
    fn test_slug_with_separator() {
        assert_eq!(
            bnto_core::case::slug_with_separator("Hello World", "-"),
            "hello-world"
        );
        assert_eq!(
            bnto_core::case::slug_with_separator("file (copy).bak", "-"),
            "file-copy-bak"
        );
        assert_eq!(
            bnto_core::case::slug_with_separator("ALLCAPS", "-"),
            "allcaps"
        );
        assert_eq!(
            bnto_core::case::slug_with_separator("already-clean", "-"),
            "already-clean"
        );
    }

    #[test]
    fn test_strip_non_ascii() {
        assert_eq!(bnto_core::case::strip_non_ascii("hello", "-"), "hello");
        assert_eq!(bnto_core::case::strip_non_ascii("café", "-"), "caf");
        assert_eq!(
            bnto_core::case::strip_non_ascii("naïve résumé", "-"),
            "nave-rsum"
        );
    }

    #[test]
    fn test_apply_max_length_fn() {
        assert_eq!(apply_max_length("hello", 3), "hel");
        assert_eq!(apply_max_length("hi", 10), "hi");
        assert_eq!(apply_max_length("test", 0), "test");
    }
}
