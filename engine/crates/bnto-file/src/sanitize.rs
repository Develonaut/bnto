// Sanitize Files Node — clean up filenames for web-safe or cross-platform use.
//
// Three modes: slugify (web URLs), strip (ASCII-only), normalize (Unicode NFC).
// File data passes through unchanged; only the filename is modified.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

/// The file-sanitize processor. Cleans filenames using one of three modes.
pub struct SanitizeFiles;

impl SanitizeFiles {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SanitizeFiles {
    fn default() -> Self {
        Self::new()
    }
}

// --- NodeProcessor Implementation ---

impl NodeProcessor for SanitizeFiles {
    fn name(&self) -> &str {
        "file-sanitize"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "file-sanitize".to_string(),
            name: "Sanitize Filenames".to_string(),
            description: "Clean filenames for web-safe or cross-platform use".to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: build_sanitize_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        if input.filename.is_empty() {
            return Err(BntoError::InvalidInput(
                "Filename cannot be empty".to_string(),
            ));
        }

        progress.report(10, "Parsing filename...");
        let (stem, ext) = split_filename(&input.filename);

        let mode = get_string_param(&input.params, "mode").unwrap_or_else(|| "slugify".to_string());
        let separator =
            get_string_param(&input.params, "separator").unwrap_or_else(|| "-".to_string());
        let max_length = get_int_param(&input.params, "max_length").unwrap_or(0);

        progress.report(40, "Sanitizing filename...");
        let sanitized_stem = match mode.as_str() {
            "strip" => strip_non_ascii(stem, &separator),
            "normalize" => normalize_unicode(stem),
            _ => slugify(stem, &separator),
        };

        progress.report(70, "Applying length limit...");
        let final_stem = apply_max_length(&sanitized_stem, max_length);

        progress.report(90, "Preparing output...");
        let new_filename = if ext.is_empty() {
            final_stem.clone()
        } else {
            format!("{final_stem}.{ext}")
        };

        let metadata = build_sanitize_metadata(&input.filename, &new_filename, &mode);

        progress.report(100, "Done!");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: new_filename,
                mime_type: "application/octet-stream".to_string(),
            }],
            metadata,
        })
    }
}

// --- Metadata Parameter Definitions ---

fn build_sanitize_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    use bnto_core::metadata::*;
    vec![
        ParameterDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            description: "Sanitization strategy".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "slugify".to_string(),
                        label: "Slugify (web-safe)".to_string(),
                    },
                    OptionEntry {
                        value: "strip".to_string(),
                        label: "Strip non-ASCII".to_string(),
                    },
                    OptionEntry {
                        value: "normalize".to_string(),
                        label: "Normalize Unicode".to_string(),
                    },
                ],
            },
            default: Some(serde_json::Value::String("slugify".to_string())),
            ..Default::default()
        },
        ParameterDef {
            name: "separator".to_string(),
            label: "Separator".to_string(),
            description: "Character to replace spaces and special characters".to_string(),
            default: Some(serde_json::Value::String("-".to_string())),
            ..Default::default()
        },
        ParameterDef {
            name: "max_length".to_string(),
            label: "Max Length".to_string(),
            description: "Maximum filename length (0 = no limit)".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::Value::Number(serde_json::Number::from(0))),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            ..Default::default()
        },
    ]
}

// --- Sanitization Functions ---

/// Slugify: lowercase, replace non-alphanumeric with separator, collapse runs.
fn slugify(stem: &str, separator: &str) -> String {
    let lowered = stem.to_lowercase();
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

    // Trim trailing separator
    if result.ends_with(separator) {
        result.truncate(result.len() - separator.len());
    }

    result
}

/// Strip: remove all non-ASCII characters, replace spaces/punctuation with separator.
fn strip_non_ascii(stem: &str, separator: &str) -> String {
    let mut result = String::with_capacity(stem.len());
    let mut prev_was_sep = true;

    for ch in stem.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch);
            prev_was_sep = false;
        } else if ch.is_ascii() && !prev_was_sep {
            // ASCII punctuation/spaces become separator
            result.push_str(separator);
            prev_was_sep = true;
        }
        // Non-ASCII characters are silently dropped
    }

    if result.ends_with(separator) {
        result.truncate(result.len() - separator.len());
    }

    result
}

/// Normalize: apply Unicode NFC normalization (compose accented characters).
fn normalize_unicode(stem: &str) -> String {
    unicode_normalization::UnicodeNormalization::nfc(stem).collect()
}

/// Truncate stem to max_length characters. 0 means no limit.
fn apply_max_length(stem: &str, max_length: usize) -> String {
    if max_length == 0 || stem.len() <= max_length {
        return stem.to_string();
    }
    stem.chars().take(max_length).collect()
}

// --- Helpers ---

/// Split filename on the last dot, same logic as rename.rs.
fn split_filename(filename: &str) -> (&str, &str) {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => (&filename[..pos], &filename[pos + 1..]),
        _ => (filename, ""),
    }
}

fn get_string_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        _ => None,
    })
}

fn get_int_param(params: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<usize> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_u64().map(|n| n as usize),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    })
}

fn build_sanitize_metadata(
    original: &str,
    new: &str,
    mode: &str,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "originalFilename".to_string(),
        serde_json::Value::String(original.to_string()),
    );
    metadata.insert(
        "newFilename".to_string(),
        serde_json::Value::String(new.to_string()),
    );
    metadata.insert(
        "mode".to_string(),
        serde_json::Value::String(mode.to_string()),
    );
    metadata
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    fn make_input(filename: &str, params: serde_json::Map<String, serde_json::Value>) -> NodeInput {
        NodeInput {
            data: b"test data".to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

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

    fn string_param(key: &str, value: &str) -> serde_json::Map<String, serde_json::Value> {
        string_params(&[(key, value)])
    }

    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

    // =========================================================================
    // Slugify Mode Tests
    // =========================================================================

    #[test]
    fn test_slugify_basic() {
        let processor = SanitizeFiles::new();
        let input = make_input("Hello World (2024).txt", string_param("mode", "slugify"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "hello-world-2024.txt");
    }

    #[test]
    fn test_slugify_consecutive_specials() {
        let processor = SanitizeFiles::new();
        let input = make_input("file---name...here.pdf", string_param("mode", "slugify"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        // Consecutive non-alphanumeric collapse into single separator
        assert_eq!(output.files[0].filename, "file-name-here.pdf");
    }

    #[test]
    fn test_slugify_custom_separator() {
        let processor = SanitizeFiles::new();
        let params = string_params(&[("mode", "slugify"), ("separator", "_")]);
        let input = make_input("Hello World.txt", params);
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "hello_world.txt");
    }

    #[test]
    fn test_slugify_no_extension() {
        let processor = SanitizeFiles::new();
        let input = make_input("My Document", string_param("mode", "slugify"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "my-document");
    }

    // =========================================================================
    // Strip Mode Tests
    // =========================================================================

    #[test]
    fn test_strip_removes_non_ascii() {
        let processor = SanitizeFiles::new();
        let input = make_input("café résumé.txt", string_param("mode", "strip"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        // Non-ASCII chars (é) dropped, spaces become separator
        assert_eq!(output.files[0].filename, "caf-rsum.txt");
    }

    #[test]
    fn test_strip_preserves_ascii_alphanumeric() {
        let processor = SanitizeFiles::new();
        let input = make_input("file123.txt", string_param("mode", "strip"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "file123.txt");
    }

    // =========================================================================
    // Normalize Mode Tests
    // =========================================================================

    #[test]
    fn test_normalize_nfc() {
        let processor = SanitizeFiles::new();
        // U+0065 (e) + U+0301 (combining acute) = NFD form of "é"
        let nfd = "re\u{0301}sume\u{0301}.txt";
        let input = make_input(nfd, string_param("mode", "normalize"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        // NFC composes into single codepoints
        assert_eq!(output.files[0].filename, "r\u{00e9}sum\u{00e9}.txt");
    }

    // =========================================================================
    // Max Length Tests
    // =========================================================================

    #[test]
    fn test_max_length_truncates_stem() {
        let processor = SanitizeFiles::new();
        let mut params = string_param("mode", "slugify");
        params.insert(
            "max_length".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );
        let input = make_input("a-very-long-filename.txt", params);
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        // Stem truncated to 5 chars, extension preserved
        assert_eq!(output.files[0].filename, "a-ver.txt");
    }

    #[test]
    fn test_max_length_zero_means_no_limit() {
        let processor = SanitizeFiles::new();
        let mut params = string_param("mode", "slugify");
        params.insert(
            "max_length".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0)),
        );
        let input = make_input("a-very-long-filename.txt", params);
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "a-very-long-filename.txt");
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_filename_returns_error() {
        let processor = SanitizeFiles::new();
        let input = make_input("", string_param("mode", "slugify"));
        let result = processor.process(input, &noop_progress(), &NoopContext);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_passes_through_unchanged() {
        let processor = SanitizeFiles::new();
        let input = NodeInput {
            data: b"original content".to_vec(),
            filename: "Hello World.txt".to_string(),
            mime_type: None,
            params: string_param("mode", "slugify"),
        };
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].data, b"original content");
    }

    #[test]
    fn test_default_mode_is_slugify() {
        let processor = SanitizeFiles::new();
        // No mode param — should default to slugify
        let input = make_input("Hello World.txt", serde_json::Map::new());
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "hello-world.txt");
    }

    #[test]
    fn test_metadata_includes_mode() {
        let processor = SanitizeFiles::new();
        let input = make_input("test.txt", string_param("mode", "strip"));
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        let mode = output.metadata.get("mode").unwrap().as_str().unwrap();
        assert_eq!(mode, "strip");
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_slugify_fn() {
        assert_eq!(slugify("Hello World", "-"), "hello-world");
        assert_eq!(slugify("file (copy).bak", "-"), "file-copy-bak");
        assert_eq!(slugify("ALLCAPS", "-"), "allcaps");
        assert_eq!(slugify("already-clean", "-"), "already-clean");
    }

    #[test]
    fn test_strip_fn() {
        assert_eq!(strip_non_ascii("hello", "-"), "hello");
        assert_eq!(strip_non_ascii("café", "-"), "caf");
        assert_eq!(strip_non_ascii("naïve résumé", "-"), "nave-rsum");
    }

    #[test]
    fn test_apply_max_length_fn() {
        assert_eq!(apply_max_length("hello", 3), "hel");
        assert_eq!(apply_max_length("hi", 10), "hi");
        assert_eq!(apply_max_length("test", 0), "test");
    }
}
