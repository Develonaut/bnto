// Pass-through output for control nodes, which steer a run but never change
// the files it carries.

use crate::processor::{BatchFile, NodeOutput, OutputFile};

/// The files as they arrived, data (a disk reference stays one) and
/// metadata included.
pub(super) fn pass_through(files: Vec<BatchFile>) -> NodeOutput {
    let files = files
        .into_iter()
        .map(|f| OutputFile {
            data: f.data,
            filename: f.filename,
            mime_type: f
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            metadata: f.metadata,
        })
        .collect();
    NodeOutput {
        files,
        metadata: serde_json::Map::new(),
    }
}
