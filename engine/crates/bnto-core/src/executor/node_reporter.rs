// The ProgressReporter a node runs with. Relays what the node reports as
// pipeline events: live progress as throttled `NodeProgress`, command output
// lines as `CommandOutput`.

use std::sync::Arc;

use crate::events::PipelineEvent;
use crate::progress::{LiveProgress, ProgressReporter};

use super::PipelineContext;

/// How often a node's live progress may reach the pipeline. Fast enough to
/// look live, slow enough not to flood a terminal or a Worker's postMessage.
const LIVE_PROGRESS_INTERVAL_MS: u64 = 100;

/// Build the reporter for one run of a node.
pub(super) fn build_node_reporter<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
) -> ProgressReporter {
    let (live, output) = (ctx.reporter.clone(), ctx.reporter.clone());
    let (live_id, output_id) = (node_id.to_string(), node_id.to_string());
    let clock = Arc::clone(&ctx.live_clock);
    ProgressReporter::with_live(
        move |progress, message| live.emit(node_progress(&live_id, progress, message)),
        move |line| {
            output.emit(PipelineEvent::CommandOutput {
                node_id: output_id.clone(),
                line: line.to_string(),
            });
        },
    )
    .throttled(move || clock(), LIVE_PROGRESS_INTERVAL_MS)
}

fn node_progress(node_id: &str, progress: LiveProgress, message: &str) -> PipelineEvent {
    let (done, total) = progress.done_and_total();
    PipelineEvent::NodeProgress {
        node_id: node_id.to_string(),
        done,
        total,
        message: message.to_string(),
    }
}
