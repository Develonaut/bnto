// Progress output — formats pipeline events for the terminal or as JSON lines.

use bnto_core::{PipelineEvent, PipelineReporter};

/// Create a reporter that prints human-readable progress to stderr.
pub fn terminal_reporter(quiet: bool) -> PipelineReporter {
    if quiet {
        return PipelineReporter::new_noop();
    }

    PipelineReporter::new(|event| {
        eprint_event(&event);
    })
}

/// Create a reporter that emits one JSON line per event to stderr.
pub fn json_reporter() -> PipelineReporter {
    PipelineReporter::new(|event| {
        if let Ok(json) = serde_json::to_string(&event) {
            eprintln!("{}", json);
        }
    })
}

fn eprint_event(event: &PipelineEvent) {
    match event {
        PipelineEvent::PipelineStarted {
            total_nodes,
            total_files,
        } => {
            eprintln!(
                "Pipeline started: {} node(s), {} file(s)",
                total_nodes, total_files
            );
        }
        PipelineEvent::NodeStarted {
            node_type,
            node_index,
            total_nodes,
            ..
        } => {
            eprintln!("  [{}/{}] {}", node_index + 1, total_nodes, node_type);
        }
        PipelineEvent::FileProgress { message, .. } => {
            eprintln!("    {}", message);
        }
        PipelineEvent::NodeCompleted {
            duration_ms,
            files_processed,
            ..
        } => {
            eprintln!("    Done ({} file(s), {}ms)", files_processed, duration_ms);
        }
        PipelineEvent::NodeFailed { node_id, error, .. } => {
            eprintln!("    FAILED [{}]: {}", node_id, error);
        }
        PipelineEvent::PipelineCompleted {
            duration_ms,
            total_files_processed,
        } => {
            eprintln!(
                "Pipeline completed: {} file(s) in {}ms",
                total_files_processed, duration_ms
            );
        }
        PipelineEvent::PipelineFailed { error, .. } => {
            eprintln!("Pipeline FAILED: {}", error);
        }
    }
}
