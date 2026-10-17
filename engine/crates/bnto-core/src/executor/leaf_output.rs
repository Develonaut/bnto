// Leaf Output — gathers what a primitive node's processor calls produce:
// the pipeline files they hand on, and the node-level metadata the node
// records for `{{node.<id>.*}}`.

use crate::pipeline::PipelineFile;
use crate::processor::NodeOutput;

use super::NodeExecutionResult;
use super::node_outputs::{merge_record, single};

type Params = serde_json::Map<String, serde_json::Value>;

/// The files and node record of one primitive node's run.
pub(super) struct LeafOutput {
    files: Vec<PipelineFile>,
    /// Every call's `NodeOutput.metadata`, merged in call order: a per-file
    /// node's later file wins a key that several files set, and its count
    /// keys add up.
    record: Params,
    /// The processor's count keys (`NodeProcessor::count_keys`).
    count_keys: &'static [&'static str],
}

impl LeafOutput {
    /// An empty output for a node whose processor adds up `count_keys`.
    pub(super) fn counting(count_keys: &'static [&'static str]) -> Self {
        Self {
            files: Vec::new(),
            record: Params::new(),
            count_keys,
        }
    }

    /// Collect one call's output files, and merge its metadata into the record.
    /// Each file's metadata is `inherited` (what the input file carried) with the
    /// node's own metadata on top: its per-file metadata when non-empty, else the
    /// shared NodeOutput.metadata. The node wins a shared key.
    pub(super) fn collect(&mut self, output: NodeOutput, inherited: &Params) {
        for f in output.files {
            let own = if f.metadata.is_empty() {
                output.metadata.clone()
            } else {
                f.metadata
            };
            let mut metadata = inherited.clone();
            metadata.extend(own);
            self.files.push(PipelineFile {
                name: f.filename,
                data: f.data,
                mime_type: f.mime_type,
                metadata,
            });
        }
        merge_record(&mut self.record, output.metadata, self.count_keys);
    }

    /// The node's result, its record filed under `node_id`.
    pub(super) fn into_result(self, node_id: &str, files_processed: usize) -> NodeExecutionResult {
        NodeExecutionResult {
            output_files: self.files,
            files_processed,
            warnings: Vec::new(),
            node_outputs: single(node_id, self.record),
        }
    }
}
