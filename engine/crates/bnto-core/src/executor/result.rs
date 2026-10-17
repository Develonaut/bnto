// Pipeline result — the run's final files plus every node's output record,
// converted from the root chain's execution result.

use serde_json::Value;

use crate::pipeline::{NodeMetadataById, PipelineFileResult, PipelineResult};

use super::NodeExecutionResult;
use super::node_outputs::NodeRecords;

/// Convert the root chain's result into the run's result, keeping processor
/// metadata on each file and each node's record by id.
pub(super) fn build_pipeline_result(
    result: NodeExecutionResult,
    duration_ms: u64,
) -> PipelineResult {
    let files = result
        .output_files
        .into_iter()
        .map(|f| PipelineFileResult {
            name: f.name,
            data: f.data,
            mime_type: f.mime_type,
            metadata: f.metadata,
        })
        .collect();
    PipelineResult {
        files,
        duration_ms,
        warnings: result.warnings,
        node_metadata: node_metadata(result.node_outputs),
    }
}

/// The records as JSON objects; every record the executor makes is one.
fn node_metadata(records: NodeRecords) -> NodeMetadataById {
    records
        .into_iter()
        .filter_map(|(id, record)| match record {
            Value::Object(metadata) => Some((id, metadata)),
            _ => None,
        })
        .collect()
}
