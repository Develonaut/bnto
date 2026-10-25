// The batch's output: each file as its copy returned it, under the metadata
// it came in with, and the node's counts added up. A per-file node gets both
// from the executor; a batch node does them itself.

use bnto_core::BntoError;
use bnto_core::processor::NodeOutput;
use serde_json::{Map, Value, json};

use crate::copy_newer::OUTCOME_KEYS;

/// `output`'s files, each with `inherited` (the input file's metadata) under
/// its own: the file's metadata when it has any, else the output's, as the
/// executor merges them for a per-file node.
pub(super) fn inheriting(output: NodeOutput, inherited: &Map<String, Value>) -> NodeOutput {
    let NodeOutput { files, metadata } = output;
    let files = files
        .into_iter()
        .map(|mut file| {
            let own = if file.metadata.is_empty() {
                metadata.clone()
            } else {
                std::mem::take(&mut file.metadata)
            };
            file.metadata = inherited.clone();
            file.metadata.extend(own);
            file
        })
        .collect();
    NodeOutput { files, metadata }
}

/// `error`, carrying the files copied before it, so the run's closing steps
/// get them as they would from a per-file node.
pub(super) fn handed_over(error: BntoError, done: Vec<NodeOutput>) -> BntoError {
    if done.is_empty() {
        return error;
    }
    BntoError::with_output(error, combined(done))
}

/// Every file's output in order; the node-level counts added up.
pub(crate) fn combined(outputs: Vec<NodeOutput>) -> NodeOutput {
    let mut files = Vec::new();
    let mut metadata = Map::new();
    for output in outputs {
        add_counts(&mut metadata, &output.metadata);
        files.extend(output.files);
    }
    NodeOutput { files, metadata }
}

/// Add `counts`' outcome flags (`newer` mode) to `total`.
fn add_counts(total: &mut Map<String, Value>, counts: &Map<String, Value>) {
    for key in OUTCOME_KEYS {
        if let Some(n) = counts.get(key).and_then(Value::as_u64) {
            let sum = total.get(key).and_then(Value::as_u64).unwrap_or(0) + n;
            total.insert(key.to_string(), json!(sum));
        }
    }
}
