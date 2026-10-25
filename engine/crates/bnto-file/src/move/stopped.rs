// The error of a file-move batch that stopped partway. Out of time (ISS-60),
// it hands over the files that ended up under the destination, as a per-file
// node hands over the files it made before a failure.

use std::collections::HashSet;
use std::path::PathBuf;

use bnto_core::BntoError;
use bnto_core::processor::BatchFile;

use super::outputs;
use super::plan::Planned;
use crate::relocate::Stopped;

/// The batch's error. A failed move hands over nothing, as before.
pub(super) fn handed_over(stopped: Stopped, files: Vec<BatchFile>, planned: Planned) -> BntoError {
    let (error, moved) = match stopped {
        Stopped::Failed(why) => return BntoError::ProcessingFailed(why),
        Stopped::Halted { error, moved } => (error, moved),
    };
    let (files, planned) = landed(files, planned, &moved.into_iter().collect());
    if files.is_empty() {
        return error;
    }
    BntoError::with_output(error, outputs(files, planned, false))
}

/// The files at their destination: moved there, or already in place.
fn landed(
    files: Vec<BatchFile>,
    planned: Planned,
    moved: &HashSet<PathBuf>,
) -> (Vec<BatchFile>, Planned) {
    let mut kept = Vec::new();
    let mut kept_plan = Planned {
        moves: Vec::new(),
        names: Vec::new(),
    };
    let all = files.into_iter().zip(planned.moves).zip(planned.names);
    for ((file, m), name) in all {
        if m.src == m.dest || moved.contains(&m.dest) {
            kept.push(file);
            kept_plan.moves.push(m);
            kept_plan.names.push(name);
        }
    }
    (kept, kept_plan)
}
