// Engine progress events → TUI execution messages.

use bnto_core::events::PipelineEvent;

use super::super::app::AppMessage;
use super::super::screens::execution::ExecutionMessage;

/// Map an engine PipelineEvent to a TUI AppMessage.
pub fn map_pipeline_event(event: PipelineEvent) -> AppMessage {
    AppMessage::Execution(execution_message(event))
}

/// Pipeline-level events map directly; node and iteration events delegate.
fn execution_message(event: PipelineEvent) -> ExecutionMessage {
    match event {
        PipelineEvent::PipelineStarted {
            total_nodes,
            total_files,
            nodes,
        } => ExecutionMessage::PipelineStarted {
            total_nodes,
            total_files,
            node_info: nodes.into_iter().map(|n| (n.id, n.node_type)).collect(),
        },
        PipelineEvent::PipelineCompleted {
            duration_ms,
            total_files_processed,
        } => ExecutionMessage::PipelineCompleted {
            duration_ms,
            total_files_processed,
        },
        PipelineEvent::PipelineFailed { node_id, error } => {
            ExecutionMessage::PipelineFailed { node_id, error }
        }
        PipelineEvent::CommandOutput { node_id, line } => {
            ExecutionMessage::CommandOutput { node_id, line }
        }
        PipelineEvent::NodeProgress {
            node_id, message, ..
        } => ExecutionMessage::NodeProgress { node_id, message },
        event => node_or_iteration_message(event),
    }
}

/// Node lifecycle and file progress events. Children of a container map to
/// the `Child*` variants so the execution screen can nest them.
fn node_or_iteration_message(event: PipelineEvent) -> ExecutionMessage {
    match event {
        PipelineEvent::NodeStarted {
            node_id,
            node_type,
            parent_node_id: Some(parent_node_id),
            ..
        } => ExecutionMessage::ChildNodeStarted {
            parent_node_id,
            node_id,
            node_type,
        },
        PipelineEvent::NodeStarted {
            node_id, node_type, ..
        } => ExecutionMessage::NodeStarted { node_id, node_type },
        PipelineEvent::NodeCompleted {
            node_id,
            duration_ms,
            parent_node_id,
            ..
        } => node_completed(node_id, duration_ms, parent_node_id),
        PipelineEvent::NodeFailed { node_id, error, .. } => {
            ExecutionMessage::NodeFailed { node_id, error }
        }
        PipelineEvent::NodeSkipped {
            node_id,
            node_type,
            parent_node_id,
            ..
        } => ExecutionMessage::NodeSkipped {
            node_id,
            node_type,
            parent_node_id,
        },
        event => file_or_iteration_message(event),
    }
}

/// Completion for a top-level node or a container child.
fn node_completed(
    node_id: String,
    duration_ms: u64,
    parent_node_id: Option<String>,
) -> ExecutionMessage {
    match parent_node_id {
        Some(parent_node_id) => ExecutionMessage::ChildNodeCompleted {
            parent_node_id,
            node_id,
            duration_ms,
        },
        None => ExecutionMessage::NodeCompleted {
            node_id,
            duration_ms,
        },
    }
}

/// File progress and per-iteration events.
fn file_or_iteration_message(event: PipelineEvent) -> ExecutionMessage {
    match event {
        PipelineEvent::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        } => ExecutionMessage::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        },
        PipelineEvent::IterationStarted {
            node_id,
            iteration,
            total_iterations,
        } => ExecutionMessage::IterationStarted {
            node_id,
            iteration,
            total_iterations,
        },
        event => iteration_outcome_message(event),
    }
}

/// Iteration completion and failure events.
fn iteration_outcome_message(event: PipelineEvent) -> ExecutionMessage {
    match event {
        PipelineEvent::IterationCompleted {
            node_id,
            iteration,
            total_iterations,
            duration_ms,
            files_produced,
            output_files,
        } => ExecutionMessage::IterationCompleted {
            node_id,
            iteration,
            total_iterations,
            duration_ms,
            files_produced,
            output_files,
        },
        PipelineEvent::IterationFailed {
            node_id,
            iteration,
            total_iterations,
            error,
        } => ExecutionMessage::IterationFailed {
            node_id,
            iteration,
            total_iterations,
            error,
        },
        other => unreachable!("event mapped by an earlier tier: {other:?}"),
    }
}
