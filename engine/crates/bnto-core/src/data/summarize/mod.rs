// summarize — Group a run's items by fields; each group reports its count and totals.
//
// A batch node, pure and browser-capable, like n8n's Summarize or a SQL
// GROUP BY: `groupBy` names the fields whose values make a group, `sum` the
// numeric fields each group totals. Out comes one item per group, in the
// order its first item arrived, carrying only the group's values, `count` and
// `sum_<field>`; it has no data, like spreadsheet-read's row items, and the
// items summarized don't pass through. See `group.rs` and `total.rs`.

mod group;
mod params;
mod total;

use serde_json::{Map, Value};

use crate::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use crate::processor::{
    BatchFile, BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use crate::progress::ProgressReporter;
use crate::{BntoError, ProcessContext};

use params::{SummarizeParams, build_summarize_parameters};

/// Summarize processor — one item out per group of items in.
pub struct Summarize;

impl Summarize {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Summarize {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for Summarize {
    fn name(&self) -> &str {
        "summarize"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "summarize".to_string(),
            name: "Summarize".to_string(),
            description: "Group items by metadata fields and output one item per group with \
                          its count and the totals of numeric fields (sum_<field>). Groups \
                          keep first-seen order; the items summarized don't pass through."
                .to_string(),
            category: NodeCategory::Data,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_summarize_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &Map<String, Value>) -> Vec<String> {
        SummarizeParams::read(params).err().into_iter().collect()
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
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = SummarizeParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let groups = group::summarize(&input.files, &params).map_err(BntoError::InvalidInput)?;
        let mut metadata = Map::new();
        metadata.insert("groupCount".to_string(), groups.len().into());
        let files = groups.into_iter().enumerate().map(summary_item).collect();
        Ok(NodeOutput { files, metadata })
    }
}

/// The `i`th group's item: no data, named `group{i}`, its fields as metadata.
fn summary_item((i, metadata): (usize, Map<String, Value>)) -> OutputFile {
    OutputFile {
        data: FileData::Bytes(Vec::new()),
        filename: format!("group{i}"),
        mime_type: "application/octet-stream".to_string(),
        metadata,
    }
}

#[cfg(test)]
mod tests;
