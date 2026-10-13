// Filename assembly — stem/extension splitting, pattern templates, and counters.

use super::param_access::{get_int_param, get_string_param};

/// Split a filename into stem and extension on the last dot.
/// ".gitignore" and "README" have no extension (empty string).
pub(super) fn split_filename(filename: &str) -> (&str, &str) {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => (&filename[..pos], &filename[pos + 1..]),
        _ => (filename, ""),
    }
}

/// If a pattern is provided, apply it (overriding the transformed stem).
/// Otherwise reconstruct from transformed stem + resolved extension.
pub(super) fn build_final_filename(
    stem: &str,
    original_stem: &str,
    original_ext: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    let ext = resolve_extension(original_ext, params);

    if let Some(pattern) = get_string_param(params, "pattern") {
        let index = get_string_param(params, "index").unwrap_or_else(|| "1".to_string());
        let counter = format_counter(params);
        let date = get_current_date();
        apply_pattern(&pattern, original_stem, &ext, &index, &counter, &date)
    } else if ext.is_empty() {
        stem.to_string()
    } else {
        format!("{stem}.{ext}")
    }
}

/// Extension override: if "extension" param is set, use it instead of original.
fn resolve_extension(
    original_ext: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    get_string_param(params, "extension")
        .filter(|e| !e.is_empty())
        .unwrap_or_else(|| original_ext.to_string())
}

/// Apply a template pattern: {{name}}, {{ext}}, {{index}}, {{counter}}, {{date}}.
fn apply_pattern(
    pattern: &str,
    name: &str,
    ext: &str,
    index: &str,
    counter: &str,
    date: &str,
) -> String {
    pattern
        .replace("{{name}}", name)
        .replace("{{ext}}", ext)
        .replace("{{index}}", index)
        .replace("{{counter}}", counter)
        .replace("{{date}}", date)
}

/// Format the counter value with zero-padding.
/// Reads `__counter` (injected by process_batch) or falls back to counter_start.
fn format_counter(params: &serde_json::Map<String, serde_json::Value>) -> String {
    let counter_value = get_int_param(params, "__counter")
        .or_else(|| get_int_param(params, "counter_start"))
        .unwrap_or(1);
    let pad = get_int_param(params, "counter_pad").unwrap_or(0);

    if pad > 0 {
        format!("{:0>width$}", counter_value, width = pad)
    } else {
        counter_value.to_string()
    }
}

/// Get current date as YYYY-MM-DD. Uses js_sys::Date in WASM, fixed date in native tests.
fn get_current_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1;
        let day = date.get_date();
        format!("{year:04}-{month:02}-{day:02}")
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-02-25".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // split_filename Tests
    // =========================================================================

    #[test]
    fn test_split_simple_filename() {
        let (stem, ext) = split_filename("photo.jpg");
        assert_eq!(stem, "photo");
        assert_eq!(ext, "jpg");
    }

    #[test]
    fn test_split_filename_multiple_dots() {
        // "archive.tar.gz" — extension is the LAST part after the last dot.
        let (stem, ext) = split_filename("archive.tar.gz");
        assert_eq!(stem, "archive.tar");
        assert_eq!(ext, "gz");
    }

    #[test]
    fn test_split_filename_no_extension() {
        let (stem, ext) = split_filename("README");
        assert_eq!(stem, "README");
        assert_eq!(ext, "");
    }

    #[test]
    fn test_split_dotfile() {
        // ".gitignore" starts with a dot — the whole thing is the stem.
        let (stem, ext) = split_filename(".gitignore");
        assert_eq!(stem, ".gitignore");
        assert_eq!(ext, "");
    }

    // =========================================================================
    // Pattern / Date Helper Tests
    // =========================================================================

    #[test]
    fn test_apply_pattern() {
        let result = apply_pattern(
            "{{date}}-{{name}}-{{index}}.{{ext}}",
            "photo",
            "jpg",
            "3",
            "1",
            "2026-02-25",
        );
        assert_eq!(result, "2026-02-25-photo-3.jpg");
    }

    #[test]
    fn test_get_current_date_format() {
        let date = get_current_date();
        // Should be YYYY-MM-DD format (10 characters, dashes at positions 4 and 7).
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
    }

    // =========================================================================
    // Counter Helper Tests
    // =========================================================================

    #[test]
    fn test_format_counter_default() {
        let params = serde_json::Map::new();
        assert_eq!(format_counter(&params), "1");
    }

    #[test]
    fn test_format_counter_with_padding() {
        let mut params = serde_json::Map::new();
        params.insert(
            "__counter".to_string(),
            serde_json::Value::Number(serde_json::Number::from(7)),
        );
        params.insert(
            "counter_pad".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );
        assert_eq!(format_counter(&params), "007");
    }

    #[test]
    fn test_format_counter_no_padding() {
        let mut params = serde_json::Map::new();
        params.insert(
            "__counter".to_string(),
            serde_json::Value::Number(serde_json::Number::from(42)),
        );
        assert_eq!(format_counter(&params), "42");
    }
}
