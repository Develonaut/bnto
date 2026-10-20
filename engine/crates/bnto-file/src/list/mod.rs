// file-list — Turn lists of paths into file references.
//
// CLI-only batch node. Paths come from its input files' text (one per line,
// or NUL-separated, as `git ls-files -z` or `find -print0` print them) and
// from its `paths` param; both are relative to `base`. Each path that exists
// becomes a path reference named by that relative path, so later nodes keep
// its folders, with its `full_path` in its metadata. A missing path is skipped
// and counted, not an error; a path that would leave `base` refuses the run.
// The node's `entries` keep every path in listed order, found or missing.

mod entries;
mod listed;
mod params;

use std::path::{Path, PathBuf};

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use listed::Listed;
use params::{ListParams, build_list_parameters};

/// File-list processor — path lists in, path references out.
pub struct FileList;

impl FileList {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileList {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileList {
    fn name(&self) -> &str {
        "file-list"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-list".to_string(),
            name: "List Files".to_string(),
            description: "List files from lists of paths: each listed path under a base folder \
                          that exists joins the pipeline; missing ones are skipped and counted. \
                          `entries` lists every path in order with its full path and status."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_list_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        ListParams::read(params).err().into_iter().collect()
    }

    /// Required by the trait; the executor always runs this node as a batch.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let file = BatchFile {
            data: input.data,
            filename: input.filename,
            mime_type: input.mime_type,
            metadata: serde_json::Map::new(),
        };
        let batch = BatchInput {
            files: vec![file],
            params: input.params,
        };
        self.process_batch(batch, progress, ctx)
    }

    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = ListParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let base = base_dir(&params, ctx)?;
        let mut raw = read_lists(&input.files)?;
        raw.extend(params.paths);
        let names =
            entries::checked_names(raw).map_err(|bad| BntoError::InvalidInput(refused(&bad)))?;
        let listed: Vec<Listed> = names
            .into_iter()
            .map(|name| Listed::check(&base, name))
            .collect();
        let found = listed.iter().filter(|l| l.found).count();
        progress.report_output(&summary(found, listed.len() - found, &base));
        Ok(outputs(listed))
    }
}

/// `base`, or the work dir when it isn't set.
fn base_dir(params: &ListParams, ctx: &dyn ProcessContext) -> Result<PathBuf, BntoError> {
    match &params.base {
        Some(base) => Ok(PathBuf::from(base)),
        None => ctx.work_dir().map(Path::to_path_buf),
    }
}

/// Every entry of every input list, in order.
fn read_lists(files: &[BatchFile]) -> Result<Vec<String>, BntoError> {
    let mut raw = Vec::new();
    for file in files {
        let bytes = file.data.to_bytes().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read path list {}: {e}", file.filename))
        })?;
        let text = std::str::from_utf8(&bytes).map_err(|_| {
            BntoError::InvalidInput(format!("Path list {} is not UTF-8 text", file.filename))
        })?;
        raw.extend(entries::split_list(text));
    }
    Ok(raw)
}

fn refused(bad: &[String]) -> String {
    let quoted: Vec<String> = bad.iter().map(|p| format!("\"{p}\"")).collect();
    format!(
        "File list refused: {} would leave the base folder (absolute, a drive, or \"..\")",
        quoted.join(", ")
    )
}

fn summary(found: usize, missing: usize, base: &Path) -> String {
    let paths = if found == 1 { "path" } else { "paths" };
    let mut text = format!("Listed {found} {paths} under {}", base.display());
    if missing > 0 {
        text.push_str(&format!(", {missing} missing"));
    }
    text
}

/// A path reference per found path; `entries` (every path, in order),
/// `missing` and `missing_count` in the node's metadata.
fn outputs(listed: Vec<Listed>) -> NodeOutput {
    let entries: Vec<serde_json::Value> = listed.iter().map(Listed::entry).collect();
    let missing: Vec<String> = listed
        .iter()
        .filter(|l| !l.found)
        .map(|l| l.name.clone())
        .collect();
    let files = listed
        .into_iter()
        .filter(|l| l.found)
        .map(Listed::into_output)
        .collect();
    let mut metadata = serde_json::Map::new();
    metadata.insert("missing_count".to_string(), missing.len().into());
    metadata.insert("missing".to_string(), missing.into());
    metadata.insert("entries".to_string(), entries.into());
    NodeOutput { files, metadata }
}

#[cfg(test)]
mod tests;
