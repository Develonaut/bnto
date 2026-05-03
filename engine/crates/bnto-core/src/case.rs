// Centralized string case/formatting — the Rust equivalent of JS's `change-case`.
//
// Every case transformation in the engine goes through this module.
// No external deps beyond `unicode-normalization` (already a workspace dep).

/// Lowercase the entire string.
pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

/// Uppercase the entire string.
pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

/// Capitalize first letter of a single word, lowercase the rest.
/// "hello" -> "Hello", "HELLO" -> "Hello"
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper_first: String = first.to_uppercase().collect();
            let lower_rest: String = chars.as_str().to_lowercase();
            format!("{upper_first}{lower_rest}")
        }
    }
}

/// Title Case — capitalize each whitespace-delimited word.
/// "hello world" -> "Hello World"
pub fn title(s: &str) -> String {
    s.split_whitespace()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert to slug — lowercase, replace non-alphanumeric with hyphens, collapse consecutive.
/// "Hello World!" -> "hello-world", "  Multiple   Spaces  " -> "multiple-spaces"
pub fn slug(s: &str) -> String {
    slug_with_separator(s, "-")
}

/// Slug with a custom separator character.
pub fn slug_with_separator(s: &str, separator: &str) -> String {
    let lowered = s.to_lowercase();
    let mut result = String::with_capacity(lowered.len());
    let mut prev_was_sep = true; // avoid leading separator

    for ch in lowered.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch);
            prev_was_sep = false;
        } else if !prev_was_sep {
            result.push_str(separator);
            prev_was_sep = true;
        }
    }

    if result.ends_with(separator) {
        result.truncate(result.len() - separator.len());
    }

    result
}

/// Replace hyphens and underscores with spaces.
/// "vehicles-and-monsters" -> "vehicles and monsters"
pub fn deslug(s: &str) -> String {
    s.chars()
        .map(|c| if c == '-' || c == '_' { ' ' } else { c })
        .collect()
}

/// Deslug + title case (the common composition).
/// "vehicles-and-monsters" -> "Vehicles And Monsters"
pub fn deslug_title(s: &str) -> String {
    title(&deslug(s))
}

/// Remove non-ASCII characters, replace spaces/punctuation with separator.
/// Preserves case (unlike slug which lowercases).
pub fn strip_non_ascii(s: &str, separator: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_was_sep = true;

    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch);
            prev_was_sep = false;
        } else if ch.is_ascii() && !prev_was_sep {
            result.push_str(separator);
            prev_was_sep = true;
        }
    }

    if result.ends_with(separator) {
        result.truncate(result.len() - separator.len());
    }

    result
}

/// NFC Unicode normalization (compose accented characters).
pub fn normalize_unicode(s: &str) -> String {
    unicode_normalization::UnicodeNormalization::nfc(s).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- lower ---

    #[test]
    fn lower_basic() {
        assert_eq!(lower("Hello World"), "hello world");
    }

    #[test]
    fn lower_unicode() {
        assert_eq!(lower("CAFÉ"), "café");
    }

    #[test]
    fn lower_empty() {
        assert_eq!(lower(""), "");
    }

    // --- upper ---

    #[test]
    fn upper_basic() {
        assert_eq!(upper("hello world"), "HELLO WORLD");
    }

    #[test]
    fn upper_unicode() {
        assert_eq!(upper("café"), "CAFÉ");
    }

    #[test]
    fn upper_empty() {
        assert_eq!(upper(""), "");
    }

    // --- capitalize ---

    #[test]
    fn capitalize_basic() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn capitalize_all_caps() {
        assert_eq!(capitalize("HELLO"), "Hello");
    }

    #[test]
    fn capitalize_empty() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn capitalize_unicode_first_char() {
        assert_eq!(capitalize("über"), "Über");
    }

    #[test]
    fn capitalize_single_char() {
        assert_eq!(capitalize("a"), "A");
    }

    // --- title ---

    #[test]
    fn title_multi_word() {
        assert_eq!(title("hello world"), "Hello World");
    }

    #[test]
    fn title_mixed_case() {
        assert_eq!(title("hELLO wORLD"), "Hello World");
    }

    #[test]
    fn title_extra_spaces() {
        // split_whitespace collapses multiple spaces
        assert_eq!(title("hello   world"), "Hello World");
    }

    #[test]
    fn title_single_word() {
        assert_eq!(title("hello"), "Hello");
    }

    #[test]
    fn title_empty() {
        assert_eq!(title(""), "");
    }

    // --- slug ---

    #[test]
    fn slug_spaces() {
        assert_eq!(slug("Hello World"), "hello-world");
    }

    #[test]
    fn slug_special_chars() {
        assert_eq!(slug("file (copy).bak"), "file-copy-bak");
    }

    #[test]
    fn slug_consecutive_separators() {
        assert_eq!(slug("file---name...here"), "file-name-here");
    }

    #[test]
    fn slug_leading_trailing() {
        assert_eq!(slug("  Hello World!  "), "hello-world");
    }

    #[test]
    fn slug_unicode() {
        // Non-ASCII alphanumeric gets stripped by is_ascii_alphanumeric
        assert_eq!(slug("Café Résumé"), "caf-r-sum");
    }

    #[test]
    fn slug_already_clean() {
        assert_eq!(slug("already-clean"), "already-clean");
    }

    #[test]
    fn slug_with_custom_separator() {
        assert_eq!(slug_with_separator("Hello World", "_"), "hello_world");
    }

    // --- deslug ---

    #[test]
    fn deslug_hyphens() {
        assert_eq!(deslug("vehicles-and-monsters"), "vehicles and monsters");
    }

    #[test]
    fn deslug_underscores() {
        assert_eq!(deslug("my_project_name"), "my project name");
    }

    #[test]
    fn deslug_mixed() {
        assert_eq!(deslug("my-project_name"), "my project name");
    }

    #[test]
    fn deslug_no_separators() {
        assert_eq!(deslug("hello"), "hello");
    }

    #[test]
    fn deslug_empty() {
        assert_eq!(deslug(""), "");
    }

    // --- deslug_title ---

    #[test]
    fn deslug_title_hyphens() {
        assert_eq!(
            deslug_title("vehicles-and-monsters"),
            "Vehicles And Monsters"
        );
    }

    #[test]
    fn deslug_title_underscores() {
        assert_eq!(deslug_title("my_project_name"), "My Project Name");
    }

    #[test]
    fn deslug_title_single_word() {
        assert_eq!(deslug_title("hello"), "Hello");
    }

    #[test]
    fn deslug_title_empty() {
        assert_eq!(deslug_title(""), "");
    }

    // --- strip_non_ascii ---

    #[test]
    fn strip_non_ascii_accented() {
        assert_eq!(strip_non_ascii("café résumé", "-"), "caf-rsum");
    }

    #[test]
    fn strip_non_ascii_emoji() {
        assert_eq!(strip_non_ascii("hello 🎉 world", "-"), "hello-world");
    }

    #[test]
    fn strip_non_ascii_clean_input() {
        assert_eq!(strip_non_ascii("hello", "-"), "hello");
    }

    #[test]
    fn strip_non_ascii_preserves_case() {
        assert_eq!(strip_non_ascii("Café", "-"), "Caf");
    }

    // --- normalize_unicode ---

    #[test]
    fn normalize_composed_vs_decomposed() {
        // U+0065 (e) + U+0301 (combining acute) = NFD form of "é"
        let nfd = "re\u{0301}sume\u{0301}";
        let nfc = normalize_unicode(nfd);
        assert_eq!(nfc, "r\u{00e9}sum\u{00e9}");
    }

    #[test]
    fn normalize_already_nfc() {
        let nfc = "résumé";
        assert_eq!(normalize_unicode(nfc), nfc);
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize_unicode(""), "");
    }
}
