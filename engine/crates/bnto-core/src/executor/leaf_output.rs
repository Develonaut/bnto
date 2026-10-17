// Leaf Output — gathers what a primitive node's processor calls produce:
// the pipeline files they hand on, and the node-level metadata the node
// records for `{{node.<id>.*}}`.

use crate::pipeline::PipelineFile;
use crate::processor::NodeOutput;

use super::NodeExecutionResult;
use super::node_outputs::single;

type Params = serde_json::Map<String, serde_json::Value>;

/// The files and node record of one primitive node's run.
#[derive(Default)]
pub(super) struct LeafOutput {
    files: Vec<PipelineFile>,
    /// Every call's `NodeOutput.metadata`, merged in call order: a per-file
    /// node's later file wins a key that several files set.
    record: Params,
}

impl LeafOutput {
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
        self.record.extend(output.metadata);
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
