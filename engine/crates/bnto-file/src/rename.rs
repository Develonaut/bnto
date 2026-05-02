// Rename Files Node — transform filenames in the browser.
//
// Browser version is a filename transformer: takes a file's current name,
// applies transformation rules, and returns the same file data with the new name.
//
// Transformation order: sanitize -> find/replace -> case -> prefix -> suffix -> pattern.
// Sanitize runs first (if set) to clean the raw filename before other transforms.
// Pattern has "final say" — it overrides everything using template variables.
// Counter: {{counter}} auto-increments per-file in batch mode.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{
    BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use regex::Regex;

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
    /// counter_start, counter_pad, extension. Accepts any file type.
    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "file-rename".to_string(),
            name: "Rename Files".to_string(),
            description:
                "Transform filenames using patterns, find/replace, case rules, and counters"
                    .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: build_rename_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    /// Transform a file's name according to the params. Data passes through unchanged.
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
        let (original_stem, original_ext) = split_filename(&input.filename);

        progress.report(30, "Applying transformations...");
        let stem = apply_all_transformations(original_stem, &input.params);

        progress.report(60, "Building new filename...");
        let new_filename = build_final_filename(&stem, original_stem, original_ext, &input.params);

        progress.report(90, "Preparing output...");
        let metadata = build_rename_metadata(&input.filename, &new_filename, &input.params);

        progress.report(100, "Done!");
        Ok(build_rename_output(input.data, new_filename, metadata))
    }

    /// Override batch processing to inject auto-incrementing counter values.
    /// Each file gets a `__counter` internal param based on its position + counter_start.
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let counter_start = get_int_param(&input.params, "counter_start").unwrap_or(1);
        let total = input.files.len();
        let mut all_files = Vec::new();
        let mut combined_metadata = serde_json::Map::new();

        for (i, file) in input.files.into_iter().enumerate() {
            let pct = ((i as u32) * 100) / (total as u32).max(1);
            progress.report(pct, &format!("Renaming file {} of {total}...", i + 1));

            let mut params = input.params.clone();
            // Inject the counter value for this file's position
            let counter_value = counter_start + i;
            params.insert(
                "__counter".to_string(),
                serde_json::Value::Number(serde_json::Number::from(counter_value)),
            );

            let single_input = NodeInput {
                data: FileData::Bytes(file.data),
                filename: file.filename,
                mime_type: file.mime_type,
                params,
            };
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

// --- Metadata Parameter Definitions ---

fn build_rename_parameters() -> Vec<bnto_core::metadata::ParameterDef> {
    vec![
        rename_sanitize_param(),
        rename_separator_param(),
        rename_max_length_param(),
        rename_string_param(
            "find",
            "Find",
            "Text or regex pattern to search for in the filename",
        ),
        rename_string_param("replace", "Replace", "Replacement text (used with Find)"),
        rename_case_param(),
        rename_string_param("prefix", "Prefix", "Text to prepend to the filename"),
        rename_string_param(
            "suffix",
            "Suffix",
            "Text to append before the file extension",
        ),
        rename_pattern_param(),
        rename_counter_start_param(),
        rename_counter_pad_param(),
        rename_extension_param(),
    ]
}

/// A simple string parameter for the rename node.
fn rename_string_param(name: &str, label: &str, desc: &str) -> bnto_core::metadata::ParameterDef {
    bnto_core::metadata::ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        ..Default::default()
    }
}

fn rename_case_param() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "case".to_string(),
        label: "Case".to_string(),
        description: "Transform the filename to a specific case".to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                OptionEntry {
                    value: "lower".to_string(),
                    label: "lowercase".to_string(),
                },
                OptionEntry {
                    value: "upper".to_string(),
                    label: "UPPERCASE".to_string(),
                },
                OptionEntry {
                    value: "title".to_string(),
                    label: "Title Case".to_string(),
                },
            ],
        },
        ..Default::default()
    }
}

fn rename_pattern_param() -> bnto_core::metadata::ParameterDef {
    bnto_core::metadata::ParameterDef {
        name: "pattern".to_string(),
        label: "Pattern".to_string(),
        description:
            "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}})"
                .to_string(),
        placeholder: Some("{{name}}-{{counter}}.{{ext}}".to_string()),
        ..Default::default()
    }
}

fn rename_counter_start_param() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "counter_start".to_string(),
        label: "Counter Start".to_string(),
        description: "Starting number for the {{counter}} variable".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(serde_json::Number::from(1))),
        constraints: Some(Constraints {
            min: Some(0.0),
            max: None,
            required: false,
        }),
        ..Default::default()
    }
}

fn rename_counter_pad_param() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "counter_pad".to_string(),
        label: "Counter Padding".to_string(),
        description: "Zero-pad width for the counter (e.g., 3 → 001, 002)".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(serde_json::Number::from(0))),
        constraints: Some(Constraints {
            min: Some(0.0),
            max: Some(10.0),
            required: false,
        }),
        ..Default::default()
    }
}

fn rename_extension_param() -> bnto_core::metadata::ParameterDef {
    bnto_core::metadata::ParameterDef {
        name: "extension".to_string(),
        label: "Extension".to_string(),
        description: "Replace the file extension (without dot)".to_string(),
        placeholder: Some("png".to_string()),
        ..Default::default()
    }
}

fn rename_sanitize_param() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "sanitize".to_string(),
        label: "Sanitize".to_string(),
        description: "Clean the filename before other transforms".to_string(),
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
        ..Default::default()
    }
}

fn rename_separator_param() -> bnto_core::metadata::ParameterDef {
    bnto_core::metadata::ParameterDef {
        name: "separator".to_string(),
        label: "Separator".to_string(),
        description: "Character to replace spaces and special characters (used with Sanitize)"
            .to_string(),
        default: Some(serde_json::Value::String("-".to_string())),
        ..Default::default()
    }
}

fn rename_max_length_param() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "max_length".to_string(),
        label: "Max Length".to_string(),
        description: "Maximum filename length after sanitize (0 = no limit)".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(serde_json::Number::from(0))),
        constraints: Some(Constraints {
            min: Some(0.0),
            max: None,
            required: false,
        }),
        ..Default::default()
    }
}

// --- Transformation Pipeline ---

/// Apply all transformations in order: sanitize -> find/replace -> case -> prefix -> suffix.
fn apply_all_transformations(
    original_stem: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    let mut stem = original_stem.to_string();

    // Sanitize runs first — clean the raw filename before other transforms.
    if let Some(mode) = get_string_param(params, "sanitize") {
        let separator = get_string_param(params, "separator").unwrap_or_else(|| "-".to_string());
        stem = match mode.as_str() {
            "strip" => strip_non_ascii(&stem, &separator),
            "normalize" => normalize_unicode(&stem),
            _ => slugify(&stem, &separator),
        };
        let max_length = get_int_param(params, "max_length").unwrap_or(0);
        stem = apply_max_length(&stem, max_length);
    }

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

/// If a pattern is provided, apply it (overriding the transformed stem).
/// Otherwise reconstruct from transformed stem + resolved extension.
fn build_final_filename(
    stem: &str,
    original_stem: &str,
    original_ext: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> String {
    // Extension override: if "extension" param is set, use it instead of original
    let ext = get_string_param(params, "extension")
        .filter(|e| !e.is_empty())
        .unwrap_or_else(|| original_ext.to_string());

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

// --- Result Metadata ---

fn build_rename_metadata(
    original_filename: &str,
    new_filename: &str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "originalFilename".to_string(),
        serde_json::Value::String(original_filename.to_string()),
    );
    metadata.insert(
        "newFilename".to_string(),
        serde_json::Value::String(new_filename.to_string()),
    );

    let transforms = build_transforms_list(params);
    metadata.insert(
        "transformsApplied".to_string(),
        serde_json::Value::Array(
            transforms
                .into_iter()
                .map(serde_json::Value::String)
                .collect(),
        ),
    );

    metadata
}

// --- Sanitize Functions ---

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
            result.push_str(separator);
            prev_was_sep = true;
        }
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

// --- Helper Functions ---

/// Split a filename into stem and extension on the last dot.
/// ".gitignore" and "README" have no extension (empty string).
fn split_filename(filename: &str) -> (&str, &str) {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => (&filename[..pos], &filename[pos + 1..]),
        _ => (filename, ""),
    }
}

/// Extract a string param from the JSON map. Handles both string and number values.
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
        "lower" => stem.to_lowercase(),
        "upper" => stem.to_uppercase(),
        "title" => to_title_case(stem),
        _ => stem.to_string(),
    }
}

/// Title case: first character uppercase, rest lowercase. Handles Unicode.
fn to_title_case(s: &str) -> String {
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

/// Extract an integer param from the JSON map.
fn get_int_param(params: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<usize> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_u64().map(|n| n as usize),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    })
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

/// Build a list of which transformations were applied, for metadata display.
fn build_transforms_list(params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    let mut transforms = Vec::new();
    if params.contains_key("sanitize") {
        transforms.push("sanitize".to_string());
    }
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
    if params.contains_key("extension") {
        transforms.push("extension".to_string());
    }
    if params.contains_key("counter_start") || params.contains_key("counter_pad") {
        transforms.push("counter".to_string());
    }
    transforms
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
            "1",
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
            "counter_pad".to_string(),
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
                    data: b"a".to_vec(),
                    filename: "a.jpg".to_string(),
                    mime_type: None,
                },
                BatchFile {
                    data: b"b".to_vec(),
                    filename: "b.jpg".to_string(),
                    mime_type: None,
                },
                BatchFile {
                    data: b"c".to_vec(),
                    filename: "c.jpg".to_string(),
                    mime_type: None,
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
            "counter_start".to_string(),
            serde_json::Value::Number(serde_json::Number::from(10)),
        );
        let batch = BatchInput {
            files: vec![
                BatchFile {
                    data: b"a".to_vec(),
                    filename: "a.jpg".to_string(),
                    mime_type: None,
                },
                BatchFile {
                    data: b"b".to_vec(),
                    filename: "b.jpg".to_string(),
                    mime_type: None,
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
            "counter_pad".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );
        let batch = BatchInput {
            files: vec![
                BatchFile {
                    data: b"a".to_vec(),
                    filename: "a.txt".to_string(),
                    mime_type: None,
                },
                BatchFile {
                    data: b"b".to_vec(),
                    filename: "b.txt".to_string(),
                    mime_type: None,
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
            "max_length".to_string(),
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
            "max_length".to_string(),
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

    // =========================================================================
    // Sanitize Helper Function Tests
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
