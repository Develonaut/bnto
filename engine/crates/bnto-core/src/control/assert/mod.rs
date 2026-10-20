// assert — Fail the run when a comparison doesn't hold.
//
// A generic control node: compares `value` to `expected` with `op` and, when
// the comparison holds, passes its files on unchanged. It runs as a batch so
// it checks even when no files arrive. When `value` or `expected` reads
// `{{file}}` or `{{item.*}}`, the comparison runs once per incoming file and
// every failing file is named in the one error; with no files it checks
// nothing. A dry run checks nothing.

mod compare;
mod params;

use crate::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use crate::processor::{BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor};
use crate::progress::ProgressReporter;
use crate::{BntoError, ProcessContext};

use super::pass_through::pass_through;
use super::placeholder;
use params::{AssertParams, build_assert_parameters};

/// Assert processor — a comparison that fails the run when it doesn't hold.
pub struct Assert;

impl Assert {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Assert {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for Assert {
    fn name(&self) -> &str {
        "assert"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "assert".to_string(),
            name: "Assert".to_string(),
            description: "Fail the run when a comparison doesn't hold (equals, not equals, \
                          after, before); otherwise pass files through unchanged."
                .to_string(),
            category: NodeCategory::Control,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_assert_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        AssertParams::read(params).err().into_iter().collect()
    }

    fn resolves_items(&self) -> bool {
        true
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
        let params = AssertParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        if ctx.dry_run() {
            progress.report_output(&format!("Would check: {}", params.describe()));
            return Ok(pass_through(input.files));
        }
        let reasons = failures(&params, &input.files);
        if !reasons.is_empty() {
            let lead = params.message.as_deref().unwrap_or("Assertion failed");
            return Err(BntoError::ProcessingFailed(format!(
                "{lead}: {}",
                reasons.join("; ")
            )));
        }
        Ok(pass_through(input.files))
    }
}

/// Every reason the check fails: once, or once per file when it reads
/// `{{file}}` or `{{item.*}}` (no files, nothing to check).
fn failures(params: &AssertParams, files: &[BatchFile]) -> Vec<String> {
    if !(placeholder::reads_file(&params.value) || placeholder::reads_file(&params.expected)) {
        return compare::compare(&params.value, params.op, &params.expected)
            .err()
            .into_iter()
            .collect();
    }
    files
        .iter()
        .filter_map(|f| file_failure(params, f))
        .collect()
}

/// Why the check fails for one file, named by it; `None` when it holds.
fn file_failure(params: &AssertParams, file: &BatchFile) -> Option<String> {
    let resolve = |text: &str| {
        placeholder::resolve_file(text, &file.filename, &file.metadata)
            .map_err(|key| format!("has no item \"{key}\""))
    };
    let checked = resolve(&params.value).and_then(|value| {
        let expected = resolve(&params.expected)?;
        compare::compare(&value, params.op, &expected)
    });
    checked.err().map(|why| format!("{}: {why}", file.filename))
}

#[cfg(test)]
mod tests;
