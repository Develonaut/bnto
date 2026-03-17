// =============================================================================
// Rename Files Node — Transform Filenames in the Browser
// =============================================================================
//
// A filename transformer (no filesystem in the browser). File data passes
// through unchanged — only the name is transformed.
//
// Transformation order (when multiple are specified):
//   1. find/replace — modify specific parts of the filename
//   2. case — transform the entire stem to lower/upper/title
//   3. prefix — prepend a string to the stem
//   4. suffix — append a string to the stem (before extension)
//   5. pattern — if provided, replaces the ENTIRE filename using a template

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use regex::Regex;

// =============================================================================
// The RenameFiles Struct
// =============================================================================

/// Stateless rename-files node processor.
/// File data passes through UNCHANGED — only the filename is modified.
pub struct RenameFiles;

impl RenameFiles {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RenameFiles {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// NodeProcessor Implementation
// =============================================================================

impl NodeProcessor for RenameFiles {
    fn name(&self) -> &str {
        "rename-files"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "file-system".to_string(),
            operation: "rename".to_string(),
            name: "Rename Files".to_string(),
            description: "Transform filenames using patterns, find/replace, and case rules"
                .to_string(),
            category: NodeCategory::File,
            // Empty accepts = any file type (we only transform the filename string).
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: vec![
                ParameterDef {
                    name: "find".to_string(),
                    label: "Find".to_string(),
                    description: "Text or regex pattern to search for in the filename".to_string(),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "replace".to_string(),
                    label: "Replace".to_string(),
                    description: "Replacement text (used with Find)".to_string(),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "case".to_string(),
                    label: "Case".to_string(),
                    description: "Transform the filename to a specific case".to_string(),
                    param_type: ParameterType::Enum {
                        options: vec![
                            "lower".to_string(),
                            "upper".to_string(),
                            "title".to_string(),
                        ],
                    },
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "prefix".to_string(),
                    label: "Prefix".to_string(),
                    description: "Text to prepend to the filename".to_string(),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "suffix".to_string(),
                    label: "Suffix".to_string(),
                    description: "Text to append before the file extension".to_string(),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "pattern".to_string(),
                    label: "Pattern".to_string(),
                    description:
                        "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})"
                            .to_string(),
                    placeholder: Some("{{name}}-compressed.{{ext}}".to_string()),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "rename".to_string(),
                    })),
                    ..Default::default()
                },
            ],
        }
    }

    /// Transform a file's name according to the configured parameters.
    /// File data (bytes) passes through unchanged.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        if input.filename.is_empty() {
            return Err(BntoError::InvalidInput(
                "Filename cannot be empty".to_string(),
            ));
        }

        progress.report(10, "Parsing filename...");

        // Split "photo.jpg" -> stem="photo", ext="jpg"
        let (original_stem, original_ext) = split_filename(&input.filename);

        progress.report(30, "Applying transformations...");

        let mut stem = original_stem.to_string();

        // --- Find/Replace ---
        // Tries regex first; falls back to literal string match if invalid regex.
        if let Some(find_str) = get_string_param(&input.params, "find") {
            let replace_str = get_string_param(&input.params, "replace").unwrap_or_default();
            stem = apply_find_replace(&stem, &find_str, &replace_str);
        }

        // --- Case Transformation ---
        if let Some(case_str) = get_string_param(&input.params, "case") {
            stem = apply_case_transform(&stem, &case_str);
        }

        // --- Prefix ---
        if let Some(prefix) = get_string_param(&input.params, "prefix") {
            stem = format!("{prefix}{stem}");
        }

        // --- Suffix ---
        if let Some(suffix) = get_string_param(&input.params, "suffix") {
            stem = format!("{stem}{suffix}");
        }

        progress.report(60, "Building new filename...");

        // --- Pattern (overrides everything) ---
        // Uses template variables: {{name}}, {{ext}}, {{index}}, {{date}}
        let new_filename = if let Some(pattern) = get_string_param(&input.params, "pattern") {
            let index = get_string_param(&input.params, "index").unwrap_or_else(|| "1".to_string());
            let date = get_current_date();
            apply_pattern(&pattern, original_stem, original_ext, &index, &date)
        } else if original_ext.is_empty() {
            stem
        } else {
            format!("{stem}.{original_ext}")
        };

        progress.report(90, "Preparing output...");

        // --- Build output (same data, new filename) ---
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalFilename".to_string(),
            serde_json::Value::String(input.filename.clone()),
        );
        metadata.insert(
            "newFilename".to_string(),
            serde_json::Value::String(new_filename.clone()),
        );

        let transforms = build_transforms_list(&input.params);
        metadata.insert(
            "transformsApplied".to_string(),
            serde_json::Value::Array(
                transforms
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );

        progress.report(100, "Done!");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: new_filename,
                // Generic MIME type — we only transform the filename, not the content.
                mime_type: "application/octet-stream".to_string(),
            }],
            metadata,
        })
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Split a filename into stem and extension (on the LAST dot).
///
/// ".gitignore" -> (".gitignore", ""), "archive.tar.gz" -> ("archive.tar", "gz")
fn split_filename(filename: &str) -> (&str, &str) {
    match filename.rfind('.') {
        // Dot at pos 0 means dotfile (e.g. ".gitignore") — treat as stem only.
        Some(pos) if pos > 0 => {
            let stem = &filename[..pos];
            let ext = &filename[pos + 1..];
            (stem, ext)
        }
        _ => (filename, ""),
    }
}

/// Extract a string parameter from the params map.
/// Handles both JSON strings and numbers (e.g. `"index": 3` -> Some("3")).
fn get_string_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    })
}

/// Apply find/replace to a string. Tries regex first, falls back to literal.
fn apply_find_replace(stem: &str, find: &str, replace: &str) -> String {
    match Regex::new(find) {
        Ok(re) => re.replace_all(stem, replace).to_string(),
        Err(_) => stem.replace(find, replace),
    }
}

/// Apply a case transformation: "lower", "upper", or "title".
/// Unknown values pass through unchanged.
fn apply_case_transform(stem: &str, case: &str) -> String {
    match case {
        "lower" => stem.to_lowercase(),
        "upper" => stem.to_uppercase(),
        "title" => to_title_case(stem),
        _ => stem.to_string(),
    }
}

/// Convert to title case (first letter uppercase, rest lowercase).
fn to_title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // to_uppercase() returns an iterator because some chars expand (e.g. 'ß' -> "SS").
            let upper_first: String = first.to_uppercase().collect();
            let lower_rest: String = chars.as_str().to_lowercase();
            format!("{upper_first}{lower_rest}")
        }
    }
}

/// Apply a template pattern to generate a new filename.
///
/// Template variables: {{name}}, {{ext}}, {{index}}, {{date}}
fn apply_pattern(pattern: &str, name: &str, ext: &str, index: &str, date: &str) -> String {
    pattern
        .replace("{{name}}", name)
        .replace("{{ext}}", ext)
        .replace("{{index}}", index)
        .replace("{{date}}", date)
}

/// Get the current date as YYYY-MM-DD.
///
/// Uses js_sys::Date in WASM; returns a fixed date in native tests
/// for determinism.
fn get_current_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1; // JS months are 0-indexed
        let day = date.get_date();
        format!("{year:04}-{month:02}-{day:02}")
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-02-25".to_string()
    }
}

/// Build a list of which transformations were applied (for metadata).
fn build_transforms_list(params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    let mut transforms = Vec::new();

    if params.contains_key("find") {
        transforms.push("find/replace".to_string());
    }
    if params.contains_key("case") {
        transforms.push("case".to_string());
    }
    if params.contains_key("prefix") {
        transforms.push("prefix".to_string());
    }
    if params.contains_key("suffix") {
        transforms.push("suffix".to_string());
    }
    if params.contains_key("pattern") {
        transforms.push("pattern".to_string());
    }

    transforms
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a NodeInput with the given data, filename, and params.
    ///
    /// This helper keeps test code concise. Instead of constructing a
    /// full NodeInput struct every time, we call this with just the
    /// pieces we care about.
    fn make_input(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

    /// Create an empty params map — for tests where no transformations are applied.
    fn empty_params() -> serde_json::Map<String, serde_json::Value> {
        serde_json::Map::new()
    }

    /// Create a params map with a single string key-value pair.
    fn string_param(key: &str, value: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert(
            key.to_string(),
            serde_json::Value::String(value.to_string()),
        );
        params
    }

    /// Create a params map from a list of string key-value pairs.
    fn string_params(pairs: &[(&str, &str)]) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        for (key, value) in pairs {
            params.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        params
    }

    /// The no-op progress reporter — doesn't need a JS runtime.
    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

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
    // Pattern Template Tests
    // =========================================================================

    #[test]
    fn test_pattern_name_and_ext() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{name}}-compressed.{{ext}}");
        let input = make_input(b"file data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo-compressed.jpg");
    }

    #[test]
    fn test_pattern_with_index() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("pattern", "file-{{index}}.{{ext}}"), ("index", "5")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-5.jpg");
    }

    #[test]
    fn test_pattern_with_default_index() {
        // When no index is provided, it defaults to "1".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "file-{{index}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-1.jpg");
    }

    #[test]
    fn test_pattern_with_date() {
        // In native tests, get_current_date() returns "2026-02-25".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{date}}-{{name}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "2026-02-25-photo.jpg");
    }

    // =========================================================================
    // Prefix / Suffix Tests
    // =========================================================================

    #[test]
    fn test_prefix_only() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "new-photo.jpg");
    }

    #[test]
    fn test_suffix_only() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("suffix", "-final");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo-final.jpg");
    }

    #[test]
    fn test_prefix_and_suffix_combined() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "v2-photo-edited.jpg");
    }

    // =========================================================================
    // Find / Replace Tests
    // =========================================================================

    #[test]
    fn test_find_replace_literal() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "old"), ("replace", "new")]);
        let input = make_input(b"data", "old-photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "new-photo.jpg");
    }

    #[test]
    fn test_find_replace_regex() {
        // Use a regex pattern to capture and rewrite part of the filename.
        // "IMG_(\d+)" captures the number after "IMG_".
        // "photo_$1" replaces "IMG_1234" with "photo_1234".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"IMG_(\d+)"), ("replace", "photo_$1")]);
        let input = make_input(b"data", "IMG_1234.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo_1234.jpg");
    }

    #[test]
    fn test_find_replace_invalid_regex_falls_back_to_literal() {
        // "[invalid" is not valid regex (unclosed bracket).
        // It should fall back to literal string replacement.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "[invalid"), ("replace", "fixed")]);
        let input = make_input(b"data", "file-[invalid-name.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "file-fixed-name.txt");
    }

    // =========================================================================
    // Case Transformation Tests
    // =========================================================================

    #[test]
    fn test_case_lower() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "lower");
        let input = make_input(b"data", "PHOTO.JPG", params);

        let output = processor.process(input, &progress).unwrap();

        // Case transforms the STEM only, not the extension.
        assert_eq!(output.files[0].filename, "photo.JPG");
    }

    #[test]
    fn test_case_upper() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "upper");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "PHOTO.jpg");
    }

    #[test]
    fn test_case_title() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "title");
        let input = make_input(b"data", "hello world.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "Hello world.txt");
    }

    // =========================================================================
    // Pass-Through and Edge Case Tests
    // =========================================================================

    #[test]
    fn test_no_params_filename_unchanged() {
        // With no transformation params, the filename should pass through unchanged.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = make_input(b"data", "photo.jpg", empty_params());

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "photo.jpg");
    }

    #[test]
    fn test_data_passes_through_unchanged() {
        // The file DATA should be exactly the same — only the filename changes.
        let original_data = b"This is the original file content. It should not change!";
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "renamed-");
        let input = make_input(original_data, "file.txt", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].data, original_data);
    }

    #[test]
    fn test_file_with_no_extension() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "README", params);

        let output = processor.process(input, &progress).unwrap();

        // No extension, so no dot appended.
        assert_eq!(output.files[0].filename, "new-README");
    }

    #[test]
    fn test_file_with_multiple_dots() {
        // "archive.tar.gz" — extension is "gz", stem is "archive.tar"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "backup-");
        let input = make_input(b"data", "archive.tar.gz", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "backup-archive.tar.gz");
    }

    #[test]
    fn test_empty_filename_returns_error() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = make_input(b"data", "", empty_params());

        let result = processor.process(input, &progress);

        assert!(result.is_err());

        // NodeOutput doesn't impl Debug, so use `if let` instead of `unwrap_err()`.
        if let Err(e) = result {
            assert!(
                e.to_string().contains("empty"),
                "Error should mention empty filename: {e}"
            );
        }
    }

    // =========================================================================
    // Combined Transformation Tests
    // =========================================================================

    #[test]
    fn test_combined_prefix_case_suffix() {
        // Order: find/replace -> case -> prefix -> suffix
        // Starting stem: "Photo" -> case="lower": "photo"
        // -> prefix="v2-": "v2-photo" -> suffix="-edited": "v2-photo-edited"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "v2-photo-edited.jpg");
    }

    #[test]
    fn test_find_replace_with_case() {
        // Order: find/replace -> case
        // "IMG_1234" -> find/replace: "photo_1234" -> case="upper": "PHOTO_1234"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "IMG"), ("replace", "photo"), ("case", "upper")]);
        let input = make_input(b"data", "IMG_1234.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "PHOTO_1234.jpg");
    }

    // =========================================================================
    // Metadata Tests
    // =========================================================================

    #[test]
    fn test_metadata_includes_original_filename() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let original = output.metadata.get("originalFilename").unwrap();
        assert_eq!(original.as_str().unwrap(), "photo.jpg");
    }

    #[test]
    fn test_metadata_includes_new_filename() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let new_name = output.metadata.get("newFilename").unwrap();
        assert_eq!(new_name.as_str().unwrap(), "new-photo.jpg");
    }

    #[test]
    fn test_metadata_includes_transforms_applied() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "new-"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        let transforms = output.metadata.get("transformsApplied").unwrap();
        let transforms_arr = transforms.as_array().unwrap();
        // Should include "case" and "prefix" (order defined by build_transforms_list).
        assert!(transforms_arr.len() >= 2);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello"), "Hello");
        assert_eq!(to_title_case("PHOTO"), "Photo");
        assert_eq!(to_title_case("hello world"), "Hello world");
        assert_eq!(to_title_case(""), "");
        assert_eq!(to_title_case("a"), "A");
    }

    #[test]
    fn test_apply_pattern() {
        let result = apply_pattern(
            "{{date}}-{{name}}-{{index}}.{{ext}}",
            "photo",
            "jpg",
            "3",
            "2026-02-25",
        );
        assert_eq!(result, "2026-02-25-photo-3.jpg");
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

    #[test]
    fn test_get_current_date_format() {
        let date = get_current_date();
        // Should be YYYY-MM-DD format (10 characters, dashes at positions 4 and 7).
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
    }
}
