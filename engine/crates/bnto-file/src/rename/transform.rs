// Stem transformations — sanitize, find/replace, case, prefix, suffix.
//
// A stem can carry a relative folder path (`sub/a` from `file-collect` with
// `flatten: false`). `find`/`replace` sees the whole path, so it can move a
// file between folders; the rest change only the name after the last `/`
// (ISS-34). `pattern` (naming.rs) also sees the whole path, and its
// `{{name}}` is this module's result (ISS-107).

use regex::Regex;

use super::param_access::{get_int_param, get_string_param};

/// Apply all transformations in order: sanitize -> find/replace -> case -> prefix -> suffix.
pub(super) fn apply_all_transformations(
    original_stem: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    let mut stem = on_name(original_stem, |name| {
        apply_sanitize(name.to_string(), params)
    });
    if let Some(find_str) = get_string_param(params, "find") {
        let replace_str = get_string_param(params, "replace").unwrap_or_default();
        stem = apply_find_replace(&stem, &find_str, &replace_str);
    }
    on_name(&stem, |name| apply_name_edits(name, params))
}

/// `edit` applied to the part of `stem` after its last `/`, the folders before
/// it kept as they are.
fn on_name(stem: &str, edit: impl FnOnce(&str) -> String) -> String {
    let at = stem.rfind('/').map_or(0, |i| i + 1);
    format!("{}{}", &stem[..at], edit(&stem[at..]))
}

/// Case, then prefix, then suffix, on a name with no folders.
fn apply_name_edits(name: &str, params: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut name = name.to_string();
    if let Some(case_str) = get_string_param(params, "case") {
        name = apply_case_transform(&name, &case_str);
    }
    if let Some(prefix) = get_string_param(params, "prefix") {
        name = format!("{prefix}{name}");
    }
    if let Some(suffix) = get_string_param(params, "suffix") {
        name = format!("{name}{suffix}");
    }
    name
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

    fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    fn text(value: &str) -> serde_json::Value {
        serde_json::Value::String(value.to_string())
    }

    // =========================================================================
    // Which part of the path each param changes (ISS-34)
    // =========================================================================

    #[test]
    fn prefix_and_case_change_the_name_not_its_folders() {
        let p = params(&[("prefix", text("x-")), ("case", text("upper"))]);
        assert_eq!(apply_all_transformations("sub/a", &p), "sub/x-A");
    }

    #[test]
    fn suffix_goes_on_the_name() {
        let p = params(&[("suffix", text("-min"))]);
        assert_eq!(apply_all_transformations("a/b/photo", &p), "a/b/photo-min");
    }

    #[test]
    fn title_case_capitalizes_the_name_not_the_first_folder() {
        let p = params(&[("case", text("title"))]);
        assert_eq!(apply_all_transformations("sub/a", &p), "sub/A");
    }

    #[test]
    fn sanitize_keeps_the_folders_byte_for_byte() {
        let p = params(&[("sanitize", text("slugify"))]);
        let out = apply_all_transformations("My Dir/v1.2/My File", &p);
        assert_eq!(out, "My Dir/v1.2/my-file");
    }

    #[test]
    fn max_length_cuts_the_name_not_the_path() {
        let p = params(&[("sanitize", text("slugify")), ("maxLength", 3.into())]);
        assert_eq!(apply_all_transformations("photos/abcdef", &p), "photos/abc");
    }

    #[test]
    fn find_replace_still_sees_the_whole_path() {
        let p = params(&[("find", text("/")), ("replace", text("-"))]);
        assert_eq!(apply_all_transformations("sub/a", &p), "sub-a");
    }

    #[test]
    fn case_and_prefix_apply_to_the_name_find_replace_leaves() {
        // `find` moves the file into `2024/`; the prefix goes on `photo`.
        let p = params(&[
            ("find", text("_")),
            ("replace", text("/")),
            ("prefix", text("x-")),
        ]);
        assert_eq!(apply_all_transformations("2024_photo", &p), "2024/x-photo");
    }

    #[test]
    fn a_name_with_no_folder_changes_as_before() {
        let p = params(&[
            ("sanitize", text("slugify")),
            ("prefix", text("x-")),
            ("case", text("upper")),
        ]);
        assert_eq!(apply_all_transformations("My File", &p), "x-MY-FILE");
    }

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
