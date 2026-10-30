// assert — Fail the run when a comparison doesn't hold.
//
// A generic control node: compares `value` to `expected` with `op` and, when
// the comparison holds, passes its files on unchanged. It runs as a batch so
// it checks even when no files arrive. When `value` or `expected` reads
// `{{file}}` or `{{item.*}}`, the comparison runs once per incoming file and
// every failing file is named in the one error; with no files it checks
// nothing. A dry run checks nothing.

mod params;

use crate::compare;
use crate::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use crate::processor::{BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor};
use crate::progress::ProgressReporter;
use crate::{BntoError, ProcessContext};

use super::pass_through::pass_through;
use super::placeholder::{self, Scope};

type Item = serde_json::Map<String, serde_json::Value>;
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
            description: "Assert a comparison on each item (equals, not equals, after, before); \
                          fail the run when it doesn't hold, otherwise pass files through \
                          unchanged."
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
        let loop_item = ctx.loop_item();
        if ctx.dry_run() {
            for check in checks(&params, &input.files, loop_item.as_ref())? {
                progress.report_output(&format!("Would check: {}", check.describe()));
            }
            return Ok(pass_through(input.files));
        }
        let reasons = failures(&params, &input.files, loop_item.as_ref());
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
fn failures(params: &AssertParams, files: &[BatchFile], loop_item: Option<&Item>) -> Vec<String> {
    if !reads_file(params) {
        return compare::compare(&params.value, params.op, &params.expected)
            .err()
            .into_iter()
            .collect();
    }
    files
        .iter()
        .filter_map(|f| file_failure(params, f, loop_item))
        .collect()
}

fn reads_file(params: &AssertParams) -> bool {
    placeholder::reads_file(&params.value) || placeholder::reads_file(&params.expected)
}

/// Why the check fails for one file, named by it; `None` when it holds.
fn file_failure(
    params: &AssertParams,
    file: &BatchFile,
    loop_item: Option<&Item>,
) -> Option<String> {
    let checked = filled(params, file, loop_item)
        .and_then(|check| compare::compare(&check.value, check.op, &check.expected));
    checked.err().map(|why| format!("{}: {why}", file.filename))
}

/// The checks a run makes, filled: once, or once per file.
fn checks(
    params: &AssertParams,
    files: &[BatchFile],
    loop_item: Option<&Item>,
) -> Result<Vec<AssertParams>, BntoError> {
    if !reads_file(params) {
        return Ok(vec![params.clone()]);
    }
    let fill = |file: &BatchFile| {
        filled(params, file, loop_item)
            .map_err(|why| BntoError::InvalidInput(format!("{}: {why}", file.filename)))
    };
    files.iter().map(fill).collect()
}

/// The check for one file: `{{file}}` and `{{item.*}}` filled in one pass,
/// from its metadata with the loop's item on top.
fn filled(
    params: &AssertParams,
    file: &BatchFile,
    loop_item: Option<&Item>,
) -> Result<AssertParams, String> {
    let item = placeholder::file_item(&file.metadata, loop_item);
    let scope = Scope {
        file: Some(&file.filename),
        item: Some(&item),
        ..Scope::default()
    };
    let fill = |text: &str| {
        placeholder::fill_text(text, scope).map_err(|m| {
            let keys: Vec<&str> = item.keys().map(String::as_str).collect();
            format!("has no item \"{}\" (it has: {})", m.key, keys.join(", "))
        })
    };
    Ok(AssertParams {
        value: fill(&params.value)?,
        expected: fill(&params.expected)?,
        ..params.clone()
    })
}

#[cfg(test)]
mod tests;
