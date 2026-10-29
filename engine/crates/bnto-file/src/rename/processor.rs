// RenameFiles — NodeProcessor wiring for `file-rename`.
//
// Orchestrates the transform/naming/metadata modules and injects a
// per-file `__counter` param in batch mode.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{
    BatchFile, BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;

use super::naming::build_final_filename;
use super::param_access::get_int_param;
use super::params::build_rename_parameters;
use super::result_metadata::build_rename_metadata;
use super::transform::apply_all_transformations;
use bnto_core::split_extension;

/// The rename-files node processor. Stateless — config comes from `NodeInput.params`.
/// File data passes through unchanged; only the filename is modified.
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

// --- NodeProcessor Implementation ---

impl NodeProcessor for RenameFiles {
    fn name(&self) -> &str {
        "file-rename"
    }

    /// Self-describing metadata: find, replace, case, prefix, suffix, pattern,
    /// counterStart, counterPad, extension. Accepts any file type.
    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "file-rename".to_string(),
            name: "Rename Files".to_string(),
            description: "Rename files using patterns, find/replace, case rules, and counters"
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: build_rename_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    /// A param the pattern has no token for, which would do nothing.
    fn validate_raw(&self, raw: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        super::raw_checks::raw_problems(raw)
    }

    /// Transform a file's name according to the params. Data passes through unchanged.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        require_filename(&input.filename)?;

        progress.report(10, "Parsing filename...");
        let (original_stem, original_ext) = split_extension(&input.filename);

        progress.report(30, "Applying transformations...");
        let stem = apply_all_transformations(original_stem, &input.params);

        progress.report(60, "Building new filename...");
        let new_filename = build_final_filename(&stem, original_ext, &input.params);

        progress.report(90, "Preparing output...");
        let metadata = build_rename_metadata(&input.filename, &new_filename, &input.params);

        progress.report(100, "Done!");
        Ok(build_rename_output(input.data, new_filename, metadata))
    }

    /// Override batch processing to inject auto-incrementing counter values.
    /// Each file gets a `__counter` internal param based on its position + counterStart.
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let counter_start = get_int_param(&input.params, "counterStart").unwrap_or(1);
        let total = input.files.len();
        let mut all_files = Vec::new();
        let mut combined_metadata = serde_json::Map::new();

        for (i, file) in input.files.into_iter().enumerate() {
            let pct = ((i as u32) * 100) / (total as u32).max(1);
            progress.report(pct, &format!("Renaming file {} of {total}...", i + 1));
            let single_input = counted_input(file, &input.params, counter_start + i);
            let output = self.process(single_input, progress, ctx)?;
            all_files.extend(output.files);
            combined_metadata = output.metadata;
        }

        Ok(NodeOutput {
            files: all_files,
            metadata: combined_metadata,
        })
    }
}

fn require_filename(filename: &str) -> Result<(), BntoError> {
    if filename.is_empty() {
        return Err(BntoError::InvalidInput(
            "Filename cannot be empty".to_string(),
        ));
    }
    Ok(())
}

/// Wrap one batch file as a single-file input, injecting its `__counter` value.
fn counted_input(
    file: BatchFile,
    params: &serde_json::Map<String, serde_json::Value>,
    counter_value: usize,
) -> NodeInput {
    let mut params = params.clone();
    params.insert(
        "__counter".to_string(),
        serde_json::Value::Number(serde_json::Number::from(counter_value)),
    );
    NodeInput {
        data: file.data,
        filename: file.filename,
        mime_type: file.mime_type,
        params,
    }
}

fn build_rename_output(
    data: FileData,
    filename: String,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data,
            filename,
            mime_type: "application/octet-stream".to_string(),
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

// =============================================================================
// Unit Tests — Pure Rust, no WASM boundary needed
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;
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
            data: FileData::Bytes(data.to_vec()),
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
    // Pattern Template Tests
    // =========================================================================

    #[test]
    fn test_pattern_name_and_ext() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{name}}-compressed.{{ext}}");
        let input = make_input(b"file data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "photo-compressed.jpg");
    }

    #[test]
    fn test_pattern_with_index() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("pattern", "file-{{index}}.{{ext}}"), ("index", "5")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "file-5.jpg");
    }

    #[test]
    fn test_pattern_with_default_index() {
        // When no index is provided, it defaults to "1".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "file-{{index}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "file-1.jpg");
    }

    #[test]
    fn test_pattern_with_date() {
        // In native tests, get_current_date() returns "2026-02-25".
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "{{date}}-{{name}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "new-photo.jpg");
    }

    #[test]
    fn test_suffix_only() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("suffix", "-final");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "photo-final.jpg");
    }

    #[test]
    fn test_prefix_and_suffix_combined() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "file-fixed-name.txt");
    }

    // =========================================================================
    // Path-Aware Find/Replace Tests
    // =========================================================================
    //
    // When file-collect outputs filenames with directory components
    // (flatten=false), find/replace operates on the full relative path.
    // This enables directory reorganization via regex.

    #[test]
    fn test_find_replace_restructures_directory_path() {
        // "Krieg Artillery/VIDEO.mp4" → regex splits first word from rest
        // → "Krieg/Artillery/VIDEO.mp4"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"^(\w+)\s+([^/]+)/"), ("replace", "$1/$2/")]);
        let input = make_input(b"data", "Krieg Artillery/VIDEO.mp4", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "Krieg/Artillery/VIDEO.mp4");
    }

    #[test]
    fn test_find_replace_leaves_single_word_dirs_unchanged() {
        // "Krieg/SOLDIER.mp4" has no space before the slash — regex doesn't match.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"^(\w+)\s+([^/]+)/"), ("replace", "$1/$2/")]);
        let input = make_input(b"data", "Krieg/SOLDIER.mp4", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "Krieg/SOLDIER.mp4");
    }

    #[test]
    fn test_find_replace_handles_multi_word_suffix() {
        // "Krieg Fire Horse/video.mp4" → "Krieg/Fire Horse/video.mp4"
        // Only the first word becomes the parent; the rest is the subfolder.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"^(\w+)\s+([^/]+)/"), ("replace", "$1/$2/")]);
        let input = make_input(b"data", "Krieg Fire Horse/video.mp4", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "Krieg/Fire Horse/video.mp4");
    }

    #[test]
    fn test_find_replace_path_no_directory_unchanged() {
        // Flat filenames (no directory component) pass through unaffected.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", r"^(\w+)\s+([^/]+)/"), ("replace", "$1/$2/")]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "photo.jpg");
    }

    // =========================================================================
    // Pattern wraps the other name params (ISS-107)
    // =========================================================================

    fn renamed(filename: &str, pairs: &[(&str, &str)]) -> String {
        let input = make_input(b"data", filename, string_params(pairs));
        let output = RenameFiles::new()
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        output.files[0].filename.clone()
    }

    #[test]
    fn test_pattern_name_carries_the_prefix() {
        let name = renamed(
            "a.jpg",
            &[("pattern", "{{name}}.{{ext}}"), ("prefix", "x-")],
        );
        assert_eq!(name, "x-a.jpg");
    }

    #[test]
    fn test_pattern_name_carries_the_sanitized_name() {
        let pairs = [
            ("pattern", "{{counter}}-{{name}}.{{ext}}"),
            ("sanitize", "slugify"),
        ];
        assert_eq!(renamed("My File.jpg", &pairs), "1-my-file.jpg");
    }

    #[test]
    fn test_pattern_name_carries_the_case_and_keeps_folders() {
        let pairs = [("pattern", "{{name}}-v2.{{ext}}"), ("case", "upper")];
        assert_eq!(renamed("sub/a.jpg", &pairs), "sub/A-v2.jpg");
    }

    #[test]
    fn test_pattern_name_carries_a_find_replace_move() {
        let pairs = [
            ("pattern", "{{name}}.{{ext}}"),
            ("find", "/"),
            ("replace", "-"),
        ];
        assert_eq!(renamed("sub/a.jpg", &pairs), "sub-a.jpg");
    }

    // =========================================================================
    // Params a pattern leaves no token for are refused (ISS-107)
    // =========================================================================

    fn raw(pairs: &[(&str, &str)]) -> Vec<String> {
        RenameFiles::new().validate_raw(&string_params(pairs))
    }

    #[test]
    fn test_a_name_param_with_no_name_token_is_refused() {
        let problems = raw(&[("pattern", "{{item.new_name}}"), ("prefix", "x-")]);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("'prefix'"), "{problems:?}");
        assert!(problems[0].contains("{{name}}"), "{problems:?}");
    }

    #[test]
    fn test_each_name_param_left_out_is_named_in_one_problem() {
        let pairs = [
            ("pattern", "final.{{ext}}"),
            ("sanitize", "slugify"),
            ("case", "upper"),
        ];
        let problems = raw(&pairs);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("'sanitize' and 'case'"),
            "{problems:?}"
        );
    }

    #[test]
    fn test_an_extension_with_no_ext_token_is_refused() {
        let problems = raw(&[("pattern", "photo"), ("extension", "png")]);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("'extension'"), "{problems:?}");
        assert!(problems[0].contains("{{ext}}"), "{problems:?}");
    }

    #[test]
    fn test_a_number_param_with_no_token_is_refused() {
        // The rename reads a number as text, so a numeric suffix would be
        // dropped too.
        let mut params = string_param("pattern", "final");
        params.insert("suffix".to_string(), serde_json::json!(2));
        let problems = RenameFiles::new().validate_raw(&params);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("'suffix'"), "{problems:?}");
    }

    #[test]
    fn test_params_the_pattern_uses_are_not_refused() {
        for pairs in [
            &[("pattern", "{{name}}.{{ext}}"), ("prefix", "x-")][..],
            &[("pattern", "x"), ("prefix", "")][..],
            &[("pattern", "{{fields.p}}"), ("prefix", "x-")][..],
            &[("pattern", "{{counter}}.{{ext}}"), ("counterStart", "5")][..],
            &[("prefix", "x-"), ("extension", "png")][..],
        ] {
            assert!(raw(pairs).is_empty(), "{pairs:?}: {:?}", raw(pairs));
        }
    }

    #[test]
    fn test_prefix_and_case_rename_the_file_not_its_folders() {
        // ISS-34: these used to rename the folders too, moving the file.
        let processor = RenameFiles::new();
        let params = string_params(&[("prefix", "x-"), ("case", "upper")]);
        let input = make_input(b"data", "v1.2/My File.jpg", params);

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "v1.2/x-MY FILE.jpg");
    }

    #[test]
    fn test_suffix_keeps_file_in_dotted_folder() {
        // ISS-23: the dot in `v1.2` used to split the name, so the suffix
        // landed in the folder name and moved the file to `v1-x.2/`.
        let processor = RenameFiles::new();
        let params = string_param("suffix", "-x");
        let input = make_input(b"data", "v1.2/README", params);

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "v1.2/README-x");
    }

    #[test]
    fn test_suffix_keeps_dotfile_whole() {
        // A leading dot is no extension: `dir/.env` used to split into
        // `dir/` + `env`, so the suffix gave `dir/-x.env`.
        let processor = RenameFiles::new();
        let params = string_param("suffix", "-x");
        let input = make_input(b"data", "dir/.env", params);

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "dir/.env-x");
    }

    #[test]
    fn test_suffix_goes_before_extension_in_dotted_folder() {
        // Guard: a name with an extension already split correctly.
        let processor = RenameFiles::new();
        let params = string_param("suffix", "-x");
        let input = make_input(b"data", "v1.2/notes.txt", params);

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "v1.2/notes-x.txt");
    }

    #[test]
    fn test_extension_override_keeps_dotted_folder() {
        // `{{ext}}` and `extension` used to see `2/README` as the extension.
        let processor = RenameFiles::new();
        let params = string_param("extension", "md");
        let input = make_input(b"data", "v1.2/README", params);

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "v1.2/README.md");
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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Note: case transforms the STEM only, not the extension.
        // The extension comes from the original filename's extension.
        assert_eq!(output.files[0].filename, "photo.JPG");
    }

    #[test]
    fn test_case_upper() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "upper");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "PHOTO.jpg");
    }

    #[test]
    fn test_case_title() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("case", "title");
        let input = make_input(b"data", "hello world.txt", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(
            output.files[0].data.clone().into_bytes().unwrap(),
            original_data
        );
    }

    #[test]
    fn test_file_with_no_extension() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "README", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "backup-archive.tar.gz");
    }

    #[test]
    fn test_empty_filename_returns_error() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = make_input(b"data", "", empty_params());

        let result = processor.process(input, &progress, &NoopContext);

        assert!(result.is_err());
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
        // Order: find/replace → case → prefix → suffix
        // Starting stem: "Photo" (from "Photo.jpg")
        // After case="lower": "photo"
        // After prefix="v2-": "v2-photo"
        // After suffix="-edited": "v2-photo-edited"
        // Final: "v2-photo-edited.jpg"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "v2-"), ("suffix", "-edited"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "v2-photo-edited.jpg");
    }

    #[test]
    fn test_find_replace_with_case() {
        // Order: find/replace → case
        // Starting stem: "IMG_1234" (from "IMG_1234.jpg")
        // After find/replace (IMG→photo): "photo_1234"
        // After case="upper": "PHOTO_1234"
        // Final: "PHOTO_1234.jpg"
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("find", "IMG"), ("replace", "photo"), ("case", "upper")]);
        let input = make_input(b"data", "IMG_1234.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let original = output.metadata.get("originalFilename").unwrap();
        assert_eq!(original.as_str().unwrap(), "photo.jpg");
    }

    #[test]
    fn test_metadata_includes_new_filename() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("prefix", "new-");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let new_name = output.metadata.get("newFilename").unwrap();
        assert_eq!(new_name.as_str().unwrap(), "new-photo.jpg");
    }

    #[test]
    fn test_metadata_includes_transforms_applied() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("prefix", "new-"), ("case", "lower")]);
        let input = make_input(b"data", "Photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let transforms = output.metadata.get("transformsApplied").unwrap();
        let transforms_arr = transforms.as_array().unwrap();
        // Should include "case" and "prefix" (order defined by build_transforms_list).
        assert!(transforms_arr.len() >= 2);
    }

    // =========================================================================
    // Counter Tests
    // =========================================================================

    #[test]
    fn test_counter_default_start() {
        // {{counter}} defaults to 1 when no __counter injected
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("pattern", "file-{{counter}}.{{ext}}");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "file-1.jpg");
    }

    #[test]
    fn test_counter_with_injected_value() {
        // process_batch injects __counter for each file
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("pattern", "img-{{counter}}.{{ext}}");
        params.insert(
            "__counter".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "img-5.jpg");
    }

    #[test]
    fn test_counter_zero_pad() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("pattern", "file-{{counter}}.{{ext}}");
        params.insert(
            "__counter".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );
        params.insert(
            "counterPad".to_string(),
            serde_json::Value::Number(serde_json::Number::from(4)),
        );
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "file-0003.jpg");
    }

    #[test]
    fn test_counter_batch_auto_increment() {
        use bnto_core::processor::{BatchFile, BatchInput};

        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("pattern", "img-{{counter}}.{{ext}}")]);
        let batch = BatchInput {
            files: vec![
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"a".to_vec()),
                    filename: "a.jpg".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"b".to_vec()),
                    filename: "b.jpg".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"c".to_vec()),
                    filename: "c.jpg".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
            ],
            params,
        };

        let output = processor
            .process_batch(batch, &progress, &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "img-1.jpg");
        assert_eq!(output.files[1].filename, "img-2.jpg");
        assert_eq!(output.files[2].filename, "img-3.jpg");
    }

    #[test]
    fn test_counter_batch_custom_start() {
        use bnto_core::processor::{BatchFile, BatchInput};

        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("pattern", "img-{{counter}}.{{ext}}");
        params.insert(
            "counterStart".to_string(),
            serde_json::Value::Number(serde_json::Number::from(10)),
        );
        let batch = BatchInput {
            files: vec![
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"a".to_vec()),
                    filename: "a.jpg".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"b".to_vec()),
                    filename: "b.jpg".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
            ],
            params,
        };

        let output = processor
            .process_batch(batch, &progress, &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "img-10.jpg");
        assert_eq!(output.files[1].filename, "img-11.jpg");
    }

    #[test]
    fn test_counter_batch_with_padding() {
        use bnto_core::processor::{BatchFile, BatchInput};

        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("pattern", "file-{{counter}}.{{ext}}");
        params.insert(
            "counterPad".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );
        let batch = BatchInput {
            files: vec![
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"a".to_vec()),
                    filename: "a.txt".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
                BatchFile {
                    data: bnto_core::processor::FileData::Bytes(b"b".to_vec()),
                    filename: "b.txt".to_string(),
                    mime_type: None,
                    metadata: serde_json::Map::new(),
                },
            ],
            params,
        };

        let output = processor
            .process_batch(batch, &progress, &NoopContext)
            .unwrap();

        assert_eq!(output.files[0].filename, "file-001.txt");
        assert_eq!(output.files[1].filename, "file-002.txt");
    }

    // =========================================================================
    // Extension Replacement Tests
    // =========================================================================

    #[test]
    fn test_extension_replacement() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("extension", "png");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "photo.png");
    }

    #[test]
    fn test_extension_replacement_with_pattern() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[
            ("pattern", "{{name}}-converted.{{ext}}"),
            ("extension", "webp"),
        ]);
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        // {{ext}} should resolve to "webp" (the override), not "jpg"
        assert_eq!(output.files[0].filename, "photo-converted.webp");
    }

    #[test]
    fn test_extension_empty_string_uses_original() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("extension", "");
        let input = make_input(b"data", "photo.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "photo.jpg");
    }

    #[test]
    fn test_extension_adds_to_extensionless_file() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("extension", "txt");
        let input = make_input(b"data", "README", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "README.txt");
    }

    // =========================================================================
    // Sanitize Mode Tests (merged from file-sanitize)
    // =========================================================================

    #[test]
    fn test_sanitize_slugify_basic() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "slugify");
        let input = make_input(b"test data", "Hello World (2024).txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "hello-world-2024.txt");
    }

    #[test]
    fn test_sanitize_slugify_consecutive_specials() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "slugify");
        let input = make_input(b"test data", "file---name...here.pdf", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "file-name-here.pdf");
    }

    #[test]
    fn test_sanitize_slugify_custom_separator() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("sanitize", "slugify"), ("separator", "_")]);
        let input = make_input(b"test data", "Hello World.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "hello_world.txt");
    }

    #[test]
    fn test_sanitize_slugify_no_extension() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "slugify");
        let input = make_input(b"test data", "My Document", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "my-document");
    }

    #[test]
    fn test_sanitize_strip_removes_non_ascii() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "strip");
        let input = make_input(b"test data", "café résumé.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "caf-rsum.txt");
    }

    #[test]
    fn test_sanitize_strip_preserves_ascii_alphanumeric() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "strip");
        let input = make_input(b"test data", "file123.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "file123.txt");
    }

    #[test]
    fn test_sanitize_normalize_nfc() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        // U+0065 (e) + U+0301 (combining acute) = NFD form of "é"
        let nfd = "re\u{0301}sume\u{0301}.txt";
        let params = string_param("sanitize", "normalize");
        let input = make_input(b"test data", nfd, params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "r\u{00e9}sum\u{00e9}.txt");
    }

    #[test]
    fn test_sanitize_max_length_truncates_stem() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("sanitize", "slugify");
        params.insert(
            "maxLength".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );
        let input = make_input(b"test data", "a-very-long-filename.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "a-ver.txt");
    }

    #[test]
    fn test_sanitize_max_length_zero_means_no_limit() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let mut params = string_param("sanitize", "slugify");
        params.insert(
            "maxLength".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0)),
        );
        let input = make_input(b"test data", "a-very-long-filename.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "a-very-long-filename.txt");
    }

    #[test]
    fn test_sanitize_data_passes_through() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let input = NodeInput {
            data: FileData::Bytes(b"original content".to_vec()),
            filename: "Hello World.txt".to_string(),
            mime_type: None,
            params: string_param("sanitize", "slugify"),
        };
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(
            output.files[0].data.clone().into_bytes().unwrap(),
            b"original content"
        );
    }

    #[test]
    fn test_sanitize_metadata_includes_transform() {
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_param("sanitize", "strip");
        let input = make_input(b"data", "test.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let transforms = output.metadata.get("transformsApplied").unwrap();
        let arr = transforms.as_array().unwrap();
        assert!(arr.iter().any(|v| v.as_str() == Some("sanitize")));
    }

    #[test]
    fn test_sanitize_then_prefix() {
        // Sanitize runs first, then prefix is applied to the cleaned stem.
        let processor = RenameFiles::new();
        let progress = noop_progress();
        let params = string_params(&[("sanitize", "slugify"), ("prefix", "clean-")]);
        let input = make_input(b"data", "Hello World.txt", params);
        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files[0].filename, "clean-hello-world.txt");
    }
}
