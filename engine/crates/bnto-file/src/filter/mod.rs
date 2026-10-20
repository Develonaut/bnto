// file-filter — Drop files from the pipeline that don't match criteria.
//
// A gating node: files that pass the filter continue downstream, files that
// don't are silently removed (empty output). Operates on filename + data
// length — no filesystem access needed, so it works in browser AND CLI.

use bnto_core::metadata::{NodeCategory, NodeMetadata};
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

mod params;
mod pattern;

use params::build_filter_params;
use pattern::PatternCache;

/// File-filter processor — drop files that don't match extension, pattern, or size criteria.
#[derive(Default)]
pub struct FileFilter {
    /// The name pattern, compiled once for the run's files.
    patterns: PatternCache,
}

impl FileFilter {
    pub fn new() -> Self {
        Self::default()
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
            description: "Keep or drop files by extension, name or path pattern, and size. Files that match every criterion are kept (or dropped, with keep off).".to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string(), "cli".to_string()],
            parameters: build_filter_params(),
            input_cardinality: Default::default(), // PerFile
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        [match_error(params), regex_error(params)]
            .into_iter()
            .flatten()
            .collect()
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Evaluating filter criteria...");

        let file_size =
            input.data.len().map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to read file size: {e}"))
            })? as usize;
        // `keep: false` inverts the filter: matching files are the ones dropped.
        let passes =
            file_matches_criteria(&self.patterns, &input.filename, file_size, &input.params)
                == keeps_matches(&input.params);

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

// --- Core Logic (pure functions) ---

/// Check if a file passes ALL active filter criteria.
///
/// All criteria use AND logic — the file must pass every active filter.
/// A filter is "active" if its param is non-empty / non-zero.
fn file_matches_criteria(
    patterns: &PatternCache,
    filename: &str,
    file_size: usize,
    params: &serde_json::Map<String, serde_json::Value>,
) -> bool {
    // Check extension filter.
    if !matches_extension(filename, params) {
        return false;
    }

    // Check name pattern filter.
    if !matches_name_pattern(patterns, filename, params) {
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

/// `match` must name what the pattern tests.
fn match_error(params: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    match params.get("match").and_then(|v| v.as_str()) {
        None | Some("name") | Some("path") => None,
        Some(other) => Some(format!(
            "match must be \"name\" or \"path\", got \"{other}\""
        )),
    }
}

/// In regex mode `name_pattern` must compile; at run time a bad one would
/// match nothing. A pattern holding a `{{` placeholder is only known once
/// that resolves, so it isn't judged here.
fn regex_error(params: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    if params.get("pattern_mode").and_then(|v| v.as_str()) != Some("regex") {
        return None;
    }
    let pattern = params.get("name_pattern").and_then(|v| v.as_str())?;
    if pattern.contains("{{") {
        return None;
    }
    let error = regex::Regex::new(pattern).err()?;
    Some(format!(
        "name_pattern \"{pattern}\" is not a valid regex: {}",
        regex_reason(&error)
    ))
}

/// The one-line reason from a regex error. Syntax errors render over several
/// lines (pattern, caret, `error: …`); the last line carries the reason.
fn regex_reason(error: &regex::Error) -> String {
    let text = error.to_string();
    let last = text.lines().last().unwrap_or_default().trim();
    last.strip_prefix("error: ").unwrap_or(last).to_string()
}

/// Check if the filename matches the name pattern (glob or regex).
///
/// If `name_pattern` param is empty or missing, all names pass.
fn matches_name_pattern(
    patterns: &PatternCache,
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

    // Any mode but `regex` is a glob. An invalid regex matches nothing.
    let regex = params.get("pattern_mode").and_then(|v| v.as_str()) == Some("regex");
    patterns.matches(pattern, regex, pattern_target(filename, params))
}

/// The part of the filename the pattern tests: the last segment by default,
/// or the whole `/`-separated relative path when `match` is `path`.
fn pattern_target<'a>(
    filename: &'a str,
    params: &serde_json::Map<String, serde_json::Value>,
) -> &'a str {
    match params.get("match").and_then(|v| v.as_str()) {
        Some("path") => filename,
        _ => filename.rsplit('/').next().unwrap_or(filename),
    }
}

/// Whether matching files are kept (`keep`, default true) or dropped.
fn keeps_matches(params: &serde_json::Map<String, serde_json::Value>) -> bool {
    params.get("keep").and_then(|v| v.as_bool()).unwrap_or(true)
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
mod tests;
