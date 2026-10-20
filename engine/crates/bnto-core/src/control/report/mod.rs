// report — Print a templated line to the run's output.
//
// A generic control node, like NiFi's LogMessage or a Node-RED debug node:
// prints `message` and passes its files on unchanged. It never fails the run
// on what it prints, only on a recipe mistake (no message, an `each` that
// isn't a list, a placeholder key that isn't there).
//
// It runs as a batch so it prints even when no files arrive. With `each`
// (a list, usually a sole `{{node.<id>.<key>}}`) it prints a line per entry,
// in order; else, when `message` reads `{{file}}` or `{{item.*}}`, a line
// per incoming file; else one line. A dry run prints what it would report.

mod lines;
mod params;

use crate::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use crate::processor::{BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor};
use crate::progress::ProgressReporter;
use crate::{BntoError, ProcessContext};

use super::pass_through::pass_through;
use params::{ReportParams, build_report_parameters};

/// Report processor — prints a templated line without failing the run.
pub struct Report;

impl Report {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for Report {
    fn name(&self) -> &str {
        "report"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "report".to_string(),
            name: "Report".to_string(),
            description: "Report a templated line in the run's output: once, once per file, \
                          or once per entry of a list; files pass through unchanged."
                .to_string(),
            category: NodeCategory::Control,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_report_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        ReportParams::read(params).err().into_iter().collect()
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
        let params = ReportParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        if ctx.dry_run() {
            progress.report_output(&format!("Would report: {}", params.message));
            return Ok(pass_through(input.files));
        }
        // Fill every line before printing any, so a refusal prints nothing.
        let lines = lines::lines(&params, &input.files).map_err(BntoError::InvalidInput)?;
        for line in &lines {
            progress.report_output(line);
        }
        Ok(pass_through(input.files))
    }
}

#[cfg(test)]
mod tests;
