// file-copy's batch: each file's params filled with its own `{{item.*}}`, the
// copies checked before any runs, then each file copied in order.
//
// The node runs as a batch so it sees every file before copying one (ISS-52).
// An `overwrite` or `newer` copy onto a file another file of the batch is read
// from would lose that file's bytes, for its own copy and for every step after
// (they read it through the pass-through), so the batch is refused. The
// executor does three things for a per-file node that a batch node does
// itself: fill `{{item.*}}` (`fill`), keep each file's metadata, and hand the
// files done before a failure to the run's closing steps (`output`).

mod fill;
mod output;
mod refusal;

use bnto_core::BntoError;
use bnto_core::processor::{BatchFile, BatchInput, NodeInput, NodeOutput};
use serde_json::{Map, Value};

pub(crate) use output::combined;
pub(crate) use refusal::refuse_replacing_read_files;

/// One file's copy: its input, params filled, and the metadata it came in with.
pub(crate) struct PerFile {
    pub(crate) input: NodeInput,
    pub(crate) inherited: Map<String, Value>,
}

/// One `PerFile` per file; a `{{item.KEY}}` a file lacks fails the batch.
pub(crate) fn per_file(input: BatchInput) -> Result<Vec<PerFile>, BntoError> {
    let BatchInput { files, params } = input;
    let one = |file: BatchFile| {
        Ok(PerFile {
            input: NodeInput {
                params: fill::filled(&params, &file)?,
                data: file.data,
                filename: file.filename,
                mime_type: file.mime_type,
            },
            inherited: file.metadata,
        })
    };
    files.into_iter().map(one).collect()
}

/// Run `copy` on each file in order. A failure hands over the files already
/// copied with its error.
pub(crate) fn copy_each(
    files: Vec<PerFile>,
    mut copy: impl FnMut(NodeInput) -> Result<NodeOutput, BntoError>,
) -> Result<NodeOutput, BntoError> {
    let mut done = Vec::new();
    for PerFile { input, inherited } in files {
        match copy(input) {
            Ok(out) => done.push(output::inheriting(out, &inherited)),
            Err(error) => return Err(output::handed_over(error, done)),
        }
    }
    Ok(combined(done))
}

#[cfg(test)]
mod held_tests;
#[cfg(test)]
mod tests;
