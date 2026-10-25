// file-delete — Delete the files flowing into it.
//
// CLI-only batch node, the delete counterpart of file-copy: upstream steps
// (file-list, file-filter) choose the files, this one removes them. Every
// item is checked before any is deleted, so one file outside `base` (via
// `..` or a link), a folder, or an in-memory item refuses the whole run, on
// a dry run too. A file already gone is skipped and counted. Items pass
// through flagged `deleted` or `skipped`, as file-copy's do. Once the run's
// time limit passes, no more deletes start (ISS-60).

mod params;
mod target;

use std::collections::HashSet;
use std::path::PathBuf;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{
    BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, Value};

use params::{DeleteParams, build_delete_parameters};
use target::Target;

/// Per-item outcome flags, also the node's totals (added up across calls).
const COUNT_KEYS: [&str; 2] = ["deleted", "skipped"];

/// File-delete processor — remove the incoming files from disk.
pub struct FileDelete;

impl FileDelete {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileDelete {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileDelete {
    fn name(&self) -> &str {
        "file-delete"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-delete".to_string(),
            name: "Delete Files".to_string(),
            description: "Delete the files flowing in, each under a base folder; missing files \
                          are skipped and counted, and a folder or a file outside the base \
                          refuses the run."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_delete_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &Map<String, Value>) -> Vec<String> {
        DeleteParams::read(params).err().into_iter().collect()
    }

    fn count_keys(&self) -> &'static [&'static str] {
        &COUNT_KEYS
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
            metadata: Map::new(),
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
        let params = DeleteParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let root = target::base_root(&params.base).map_err(BntoError::InvalidInput)?;
        let targets = plan(&input.files, &root, &params.base).map_err(BntoError::InvalidInput)?;
        if !ctx.dry_run()
            && let Err((done, error)) = delete_all(&input.files, &targets, ctx)
        {
            return Err(handed_over(error, input.files, &targets, done));
        }
        let deleted = targets.iter().filter(|t| **t != Target::Missing).count();
        let skipped = targets.len() - deleted;
        progress.report_output(&summary(deleted, skipped, &params.base, ctx));
        Ok(outputs(input.files, &targets))
    }
}

/// Every item's target, checked before anything is deleted. A file listed
/// twice is deleted once; its later items count as skipped.
fn plan(files: &[BatchFile], root: &std::path::Path, base: &str) -> Result<Vec<Target>, String> {
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut targets = Vec::with_capacity(files.len());
    for file in files {
        let target = match target::check(file, root, base)? {
            Target::File(path) if !seen.insert(path.clone()) => Target::Missing,
            other => other,
        };
        targets.push(target);
    }
    Ok(targets)
}

/// Delete each target in order, first asking whether the run's time is up
/// (ISS-60). On a failure, how many items were finished before it.
fn delete_all(
    files: &[BatchFile],
    targets: &[Target],
    ctx: &dyn ProcessContext,
) -> Result<(), (usize, BntoError)> {
    for (i, (file, target)) in files.iter().zip(targets).enumerate() {
        if let Target::File(path) = target {
            ctx.check_time_limit().map_err(|e| (i, e))?;
            std::fs::remove_file(path).map_err(|e| {
                let why = format!("Failed to delete {}: {e}", file.filename);
                (i, BntoError::ProcessingFailed(why))
            })?;
        }
    }
    Ok(())
}

/// `error`, carrying the first `done` items when the run's time ran out, as
/// a per-file node hands over the files it finished. A failed delete hands
/// over nothing, as before (ISS-84).
fn handed_over(
    error: BntoError,
    files: Vec<BatchFile>,
    targets: &[Target],
    done: usize,
) -> BntoError {
    if done == 0 || !error.is_timeout() {
        return error;
    }
    let finished = files.into_iter().take(done).collect();
    BntoError::with_output(error, outputs(finished, &targets[..done]))
}

fn summary(deleted: usize, skipped: usize, base: &str, ctx: &dyn ProcessContext) -> String {
    let verb = if ctx.dry_run() {
        "Would delete"
    } else {
        "Deleted"
    };
    let files = if deleted == 1 { "file" } else { "files" };
    let mut text = format!("{verb} {deleted} {files} under {base}");
    if skipped > 0 {
        text.push_str(&format!(", {skipped} missing"));
    }
    text
}

/// Each item handed on unchanged, its metadata kept and flagged with what
/// happened to it; the node's metadata carries the totals.
#[expect(clippy::too_many_lines)]
fn outputs(files: Vec<BatchFile>, targets: &[Target]) -> NodeOutput {
    let mut totals = [0usize; 2];
    let files = files
        .into_iter()
        .zip(targets)
        .map(|(f, target)| {
            let slot = usize::from(*target == Target::Missing);
            totals[slot] += 1;
            let mut metadata = f.metadata;
            for (i, key) in COUNT_KEYS.iter().enumerate() {
                metadata.insert(key.to_string(), usize::from(i == slot).into());
            }
            OutputFile {
                data: f.data,
                filename: f.filename,
                mime_type: f
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
                metadata,
            }
        })
        .collect();
    let metadata = COUNT_KEYS
        .iter()
        .zip(totals)
        .map(|(k, n)| (k.to_string(), n.into()))
        .collect();
    NodeOutput { files, metadata }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod time_tests;
