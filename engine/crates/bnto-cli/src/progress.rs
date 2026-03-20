// Progress reporting — prints pipeline events to stderr.

use bnto_core::{PipelineEvent, PipelineReporter};

/// Create a reporter that prints progress events to stderr.
pub fn stderr_reporter() -> PipelineReporter {
    PipelineReporter::new(|event: PipelineEvent| {
        match &event {
            PipelineEvent::NodeStarted {
                node_type,
                node_index,
                total_nodes,
                ..
            } => {
                eprintln!("  [{}/{}] {}", node_index + 1, total_nodes, node_type,);
            }
            PipelineEvent::NodeCompleted {
                duration_ms,
                files_processed,
                ..
            } => {
                eprintln!(
                    "        {} file{} in {}ms",
                    files_processed,
                    if *files_processed == 1 { "" } else { "s" },
                    duration_ms,
                );
            }
            PipelineEvent::NodeFailed { node_id, error, .. } => {
                eprintln!("  FAILED {node_id}: {error}");
            }
            PipelineEvent::PipelineFailed { error, .. } => {
                eprintln!("  Pipeline failed: {error}");
            }
            // PipelineStarted, FileProgress, PipelineCompleted — handled by main.
            _ => {}
        }
    })
}
