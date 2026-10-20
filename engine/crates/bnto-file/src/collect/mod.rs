// file-collect — Traverse directories and glob-match files into the pipeline.
//
// CLI-only processor: reads files from the filesystem via ProcessContext.
// Walks the directory its input references: a disk reference's path (as
// `file-list` emits a listed folder), else the input's filename (a CLI dir
// input). Outputs each matched file as a pipeline entry, named relative to
// the walked directory.

use bnto_core::metadata::{NodeCategory, NodeMetadata, ParameterDef, ParameterType};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::PathBuf;

mod walker;

use walker::Walker;

/// File-collect processor — directory traversal + glob matching.
pub struct FileCollect;

impl FileCollect {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileCollect {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileCollect {
    fn name(&self) -> &str {
        "file-collect"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-collect".to_string(),
            name: "Collect Files".to_string(),
            description: "Collect files from a folder that match a glob pattern into the pipeline."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_collect_params(),
            input_cardinality: Default::default(), // PerFile — directory path is the "file"
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let root = walk_root(&input);
        if !root.is_dir() {
            return Err(BntoError::InvalidInput(format!(
                "Not a directory: {}",
                root.display()
            )));
        }

        let mut walker = Walker::new(&root, &input, progress);
        walker.walk(&root)?;
        Ok(NodeOutput {
            files: walker.found.into_iter().map(output_file).collect(),
            metadata: serde_json::Map::new(),
        })
    }
}

/// A matched file as a path reference (zero-copy — no file reads).
fn output_file((output_name, full_path): (String, PathBuf)) -> OutputFile {
    OutputFile {
        data: FileData::Path(full_path),
        mime_type: guess_mime_type(&output_name),
        filename: output_name,
        metadata: serde_json::Map::new(),
    }
}

pub(super) fn param_str<'a>(input: &'a NodeInput, key: &str) -> Option<&'a str> {
    input.params.get(key).and_then(|v| v.as_str())
}

pub(super) fn param_bool(input: &NodeInput, key: &str) -> Option<bool> {
    input.params.get(key).and_then(|v| v.as_bool())
}

// --- Parameter Definitions ---

fn build_collect_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "pattern".to_string(),
            label: "Pattern".to_string(),
            description: "Glob pattern to match files (e.g., \"*.jpg\", \"**/*.svg\"). Default: \"*\" (all files).".to_string(),
            param_type: ParameterType::String,
            default: Some(serde_json::json!("*")),
            placeholder: Some("*.jpg".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "recursive".to_string(),
            label: "Recursive".to_string(),
            description: "Search subfolders when collecting files.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "flatten".to_string(),
            label: "Flatten".to_string(),
            description: "Strip folder structure from output filenames. When true, all files appear as if in the same folder.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
    ]
}

// --- Core Logic ---

/// The directory to walk: the one a disk reference points at (`file-list`
/// names it by its relative path), else the input's filename (a CLI dir
/// input carries its absolute path there, with no data).
fn walk_root(input: &NodeInput) -> PathBuf {
    match &input.data {
        FileData::Path(path) => path.clone(),
        _ => PathBuf::from(&input.filename),
    }
}

/// Guess MIME type from file extension.
fn guess_mime_type(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "webp" => "image/webp".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "gif" => "image/gif".to_string(),
        "csv" => "text/csv".to_string(),
        "json" => "application/json".to_string(),
        "txt" | "md" => "text/plain".to_string(),
        "pdf" => "application/pdf".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[cfg(test)]
mod tests;
