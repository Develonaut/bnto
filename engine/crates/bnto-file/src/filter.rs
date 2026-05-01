// file-filter — Drop files from the pipeline that don't match criteria.
//
// A gating node: files that pass the filter continue downstream, files that
// don't are silently removed (empty output). Operates on filename + data
// length — no filesystem access needed, so it works in browser AND CLI.

use bnto_core::metadata::{
    Constraints, NodeCategory, NodeMetadata, OptionEntry, ParameterDef, ParameterType,
};
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

/// File-filter processor — drop files that don't match extension, pattern, or size criteria.
pub struct FileFilter;

impl FileFilter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileFilter {
    fn name(&self) -> &str {
        "file-filter"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-filter".to_string(),
            name: "Filter Files".to_string(),
            description: "Keep only files matching extension, name pattern, or size criteria. Non-matching files are dropped from the pipeline.".to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string(), "cli".to_string()],
            parameters: build_filter_params(),
            input_cardinality: Default::default(), // PerFile
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Evaluating filter criteria...");

        let passes = file_matches_criteria(&input.filename, input.data.len(), &input.params);

        progress.report(100, "Done");

        if passes {
            // File matches — pass it through unchanged.
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                    metadata: serde_json::Map::new(),
                }],
                metadata: serde_json::Map::new(),
            })
        } else {
            // File doesn't match — drop it (empty output).
            Ok(NodeOutput {
                files: vec![],
                metadata: serde_json::Map::new(),
            })
        }
    }
}

// --- Parameter Definitions ---

fn build_filter_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "extensions".to_string(),
            label: "Extensions".to_string(),
            description: "Comma-separated list of file extensions to keep (e.g., \"jpg,png,svg\"). Leave empty to allow all extensions.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("jpg,png,svg".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "name_pattern".to_string(),
            label: "Name Pattern".to_string(),
            description: "Glob pattern to match against filenames (e.g., \"photo*\", \"*.backup.*\"). Uses glob syntax with * and ? wildcards.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("photo*".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "pattern_mode".to_string(),
            label: "Pattern Mode".to_string(),
            description: "How to interpret the name pattern.".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "glob".to_string(),
                        label: "Glob (*, ?)".to_string(),
                    },
                    OptionEntry {
                        value: "regex".to_string(),
                        label: "Regex".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("glob")),
            ..Default::default()
        },
        ParameterDef {
            name: "min_size".to_string(),
            label: "Minimum Size".to_string(),
            description: "Minimum file size in bytes. Files smaller than this are dropped.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            suffix: Some("bytes".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "max_size".to_string(),
            label: "Maximum Size".to_string(),
            description: "Maximum file size in bytes. Files larger than this are dropped. 0 means no limit.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            suffix: Some("bytes".to_string()),
            ..Default::default()
        },
    ]
}

// --- Core Logic (pure functions) ---

/// Check if a file passes ALL active filter criteria.
///
/// All criteria use AND logic — the file must pass every active filter.
/// A filter is "active" if its param is non-empty / non-zero.
fn file_matches_criteria(
    filename: &str,
    file_size: usize,
    params: &serde_json::Map<String, serde_json::Value>,
) -> bool {
    // Check extension filter.
    if !matches_extension(filename, params) {
        return false;
    }

    // Check name pattern filter.
    if !matches_name_pattern(filename, params) {
        return false;
    }

    // Check size range filter.
    if !matches_size(file_size, params) {
        return false;
    }

    true
}

/// Check if the file's extension is in the allowed list.
///
/// If `extensions` param is empty or missing, all extensions pass.
fn matches_extension(filename: &str, params: &serde_json::Map<String, serde_json::Value>) -> bool {
    let extensions_str = params
        .get("extensions")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Empty = no filter, everything passes.
    if extensions_str.trim().is_empty() {
        return true;
    }

    // Extract the file extension (without the dot).
    let file_ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    // Parse the comma-separated extension list.
    extensions_str
        .split(',')
        .map(|ext| {
            ext.trim()
                .to_lowercase()
                .trim_start_matches('.')
                .to_string()
        })
        .any(|allowed| allowed == file_ext)
}

/// Check if the filename matches the name pattern (glob or regex).
///
/// If `name_pattern` param is empty or missing, all names pass.
fn matches_name_pattern(
    filename: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> bool {
    let pattern = params
        .get("name_pattern")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Empty = no filter, everything passes.
    if pattern.trim().is_empty() {
        return true;
    }

    let mode = params
        .get("pattern_mode")
        .and_then(|v| v.as_str())
        .unwrap_or("glob");

    // Extract just the filename part (no directory path).
    let name_only = filename.rsplit('/').next().unwrap_or(filename);

    match mode {
        "regex" => match regex::Regex::new(pattern) {
            Ok(re) => re.is_match(name_only),
            Err(_) => false, // Invalid regex = no match.
        },
        _ => glob_match(pattern, name_only),
    }
}

/// Simple glob matching with `*` (any chars) and `?` (single char).
///
/// Converts glob to regex under the hood for correct semantics.
fn glob_match(pattern: &str, text: &str) -> bool {
    // Convert glob to regex: escape regex special chars, then
    // convert * → .* and ? → .
    let mut regex_str = String::from("^");
    for ch in pattern.chars() {
        match ch {
            '*' => regex_str.push_str(".*"),
            '?' => regex_str.push('.'),
            // Escape regex metacharacters.
            '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                regex_str.push('\\');
                regex_str.push(ch);
            }
            _ => regex_str.push(ch),
        }
    }
    regex_str.push('$');

    match regex::Regex::new(&regex_str) {
        Ok(re) => re.is_match(text),
        Err(_) => false,
    }
}

/// Check if the file size falls within the min/max range.
///
/// min_size=0 means no minimum. max_size=0 means no maximum.
fn matches_size(file_size: usize, params: &serde_json::Map<String, serde_json::Value>) -> bool {
    let min_size = params.get("min_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    let max_size = params.get("max_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    // Check minimum.
    if min_size > 0 && file_size < min_size {
        return false;
    }

    // Check maximum (0 = no limit).
    if max_size > 0 && file_size > max_size {
        return false;
    }

    true
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::context::NoopContext;
    use bnto_core::progress::ProgressReporter;

    /// Helper: create a NodeInput with the given data, filename, and params.
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

    /// Helper: build a params map from key-value pairs.
    fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.clone());
        }
        map
    }

    // --- Trait basics ---

    #[test]
    fn test_name_returns_correct_key() {
        let processor = FileFilter::new();
        assert_eq!(processor.name(), "file-filter");
    }

    #[test]
    fn test_metadata_has_correct_category() {
        let processor = FileFilter::new();
        let meta = processor.metadata();
        assert_eq!(meta.category, NodeCategory::File);
        assert_eq!(meta.node_type, "file-filter");
    }

    #[test]
    fn test_validate_passes_with_no_params() {
        let processor = FileFilter::new();
        let errors = processor.validate(&serde_json::Map::new());
        assert!(errors.is_empty());
    }

    // --- Extension filtering ---

    #[test]
    fn test_filter_by_extension_keeps_matching() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(
            b"image data",
            "photo.jpg",
            params(&[("extensions", serde_json::json!("jpg,png"))]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "Matching file should pass through");
        assert_eq!(output.files[0].filename, "photo.jpg");
    }

    #[test]
    fn test_filter_by_extension_drops_non_matching() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(
            b"text data",
            "readme.txt",
            params(&[("extensions", serde_json::json!("jpg,png"))]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0, "Non-matching file should be dropped");
    }

    #[test]
    fn test_filter_extension_case_insensitive() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(
            b"image data",
            "photo.JPG",
            params(&[("extensions", serde_json::json!("jpg"))]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            1,
            "Extension matching should be case-insensitive"
        );
    }

    #[test]
    fn test_filter_extension_with_dots() {
        // User might write ".jpg" or "jpg" — both should work.
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(
            b"image data",
            "photo.jpg",
            params(&[("extensions", serde_json::json!(".jpg,.png"))]),
        );

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "Dotted extension should still match");
    }

    #[test]
    fn test_filter_no_extension_param_passes_all() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(b"data", "anything.xyz", serde_json::Map::new());

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "No filter should pass everything");
    }

    // --- Name pattern filtering ---

    #[test]
    fn test_filter_by_glob_pattern() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // "photo*" should match "photo_001.jpg"
        let input = make_input(
            b"data",
            "photo_001.jpg",
            params(&[("name_pattern", serde_json::json!("photo*"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "Glob pattern should match");

        // "photo*" should NOT match "document.pdf"
        let input = make_input(
            b"data",
            "document.pdf",
            params(&[("name_pattern", serde_json::json!("photo*"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0, "Glob pattern should not match");
    }

    #[test]
    fn test_filter_by_regex_pattern() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // Regex: match files starting with "img" followed by digits
        let input = make_input(
            b"data",
            "img42.png",
            params(&[
                ("name_pattern", serde_json::json!(r"^img\d+\.png$")),
                ("pattern_mode", serde_json::json!("regex")),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "Regex should match");

        // Same regex should NOT match "photo.png"
        let input = make_input(
            b"data",
            "photo.png",
            params(&[
                ("name_pattern", serde_json::json!(r"^img\d+\.png$")),
                ("pattern_mode", serde_json::json!("regex")),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0, "Regex should not match");
    }

    #[test]
    fn test_filter_glob_with_question_mark() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // "file?.txt" should match "file1.txt" but not "file10.txt"
        let input = make_input(
            b"data",
            "file1.txt",
            params(&[("name_pattern", serde_json::json!("file?.txt"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        let input = make_input(
            b"data",
            "file10.txt",
            params(&[("name_pattern", serde_json::json!("file?.txt"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0);
    }

    // --- Size filtering ---

    #[test]
    fn test_filter_by_min_size() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // 5 bytes, min_size=10 → dropped
        let input = make_input(
            b"small",
            "small.txt",
            params(&[("min_size", serde_json::json!(10))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            0,
            "File below min_size should be dropped"
        );

        // 20 bytes, min_size=10 → kept
        let input = make_input(
            &[0u8; 20],
            "big.txt",
            params(&[("min_size", serde_json::json!(10))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "File above min_size should pass");
    }

    #[test]
    fn test_filter_by_max_size() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // 20 bytes, max_size=10 → dropped
        let input = make_input(
            &[0u8; 20],
            "big.txt",
            params(&[("max_size", serde_json::json!(10))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            0,
            "File above max_size should be dropped"
        );

        // 5 bytes, max_size=10 → kept
        let input = make_input(
            b"small",
            "small.txt",
            params(&[("max_size", serde_json::json!(10))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "File below max_size should pass");
    }

    #[test]
    fn test_filter_size_zero_means_no_limit() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // max_size=0 → no limit (passes)
        let input = make_input(
            &[0u8; 1000],
            "big.txt",
            params(&[("max_size", serde_json::json!(0))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1, "max_size=0 should mean no limit");
    }

    // --- Combined filters (AND logic) ---

    #[test]
    fn test_filter_combined_extension_and_size() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // File is .jpg AND > 10 bytes → passes both
        let input = make_input(
            &[0u8; 20],
            "photo.jpg",
            params(&[
                ("extensions", serde_json::json!("jpg,png")),
                ("min_size", serde_json::json!(10)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            1,
            "File matching both criteria should pass"
        );

        // File is .jpg BUT < 10 bytes → fails size
        let input = make_input(
            b"tiny",
            "photo.jpg",
            params(&[
                ("extensions", serde_json::json!("jpg,png")),
                ("min_size", serde_json::json!(10)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            0,
            "File failing any criterion should be dropped"
        );

        // File is .txt AND > 10 bytes → fails extension
        let input = make_input(
            &[0u8; 20],
            "readme.txt",
            params(&[
                ("extensions", serde_json::json!("jpg,png")),
                ("min_size", serde_json::json!(10)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            0,
            "File failing extension should be dropped"
        );
    }

    #[test]
    fn test_filter_combined_pattern_and_extension() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // "photo*" + extensions "jpg" → photo_001.jpg passes
        let input = make_input(
            b"data",
            "photo_001.jpg",
            params(&[
                ("name_pattern", serde_json::json!("photo*")),
                ("extensions", serde_json::json!("jpg")),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);

        // "photo*" + extensions "jpg" → document.jpg fails pattern
        let input = make_input(
            b"data",
            "document.jpg",
            params(&[
                ("name_pattern", serde_json::json!("photo*")),
                ("extensions", serde_json::json!("jpg")),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0);
    }

    // --- Edge cases ---

    #[test]
    fn test_filter_preserves_file_data() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();
        let original_data = b"original file content";

        let input = make_input(original_data, "file.txt", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            output.files[0].data, original_data,
            "Data should pass through unchanged"
        );
    }

    #[test]
    fn test_filter_file_without_extension() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        // A file with no extension should be dropped when extension filter is active.
        let input = make_input(
            b"data",
            "Makefile",
            params(&[("extensions", serde_json::json!("txt"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 0);
    }

    #[test]
    fn test_filter_invalid_regex_drops_file() {
        let processor = FileFilter::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"data",
            "file.txt",
            params(&[
                ("name_pattern", serde_json::json!("[invalid")),
                ("pattern_mode", serde_json::json!("regex")),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files.len(),
            0,
            "Invalid regex should not match anything"
        );
    }

    // --- Pure function tests ---

    #[test]
    fn test_glob_match_star() {
        assert!(glob_match("*.txt", "hello.txt"));
        assert!(!glob_match("*.txt", "hello.jpg"));
        assert!(glob_match("photo*", "photo_001.jpg"));
        assert!(glob_match("*", "anything"));
    }

    #[test]
    fn test_glob_match_question() {
        assert!(glob_match("file?.txt", "file1.txt"));
        assert!(!glob_match("file?.txt", "file12.txt"));
    }

    #[test]
    fn test_matches_extension_whitespace_handling() {
        let p = params(&[("extensions", serde_json::json!(" jpg , png , svg "))]);
        assert!(matches_extension("photo.jpg", &p));
        assert!(matches_extension("icon.svg", &p));
        assert!(!matches_extension("doc.pdf", &p));
    }
}
