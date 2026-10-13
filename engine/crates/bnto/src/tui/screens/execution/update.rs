// Execution screen update — routes each message to a focused transition.

use super::{ExecutionMessage, ExecutionModel, ExecutionStatus};

/// Pure state transition for the execution screen.
pub fn update(mut model: ExecutionModel, msg: ExecutionMessage) -> ExecutionModel {
    use ExecutionMessage as M;
    match msg {
        M::PipelineStarted {
            total_nodes,
            total_files,
            node_info,
        } => model.start_pipeline(total_nodes, total_files, node_info),
        M::NodeStarted { node_id, node_type } => model.start_node(&node_id, node_type),
        M::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        } => model.record_file_progress(&node_id, file_index, total_files, percent, message),
        M::NodeCompleted {
            node_id,
            duration_ms,
        } => model.complete_node(&node_id, duration_ms),
        M::NodeFailed { node_id, error } => model.fail_node(&node_id, error),
        M::PipelineCompleted { duration_ms, .. } => model.complete_pipeline(duration_ms),
        M::PipelineFailed { error, .. } => model.fail_pipeline(error),
        M::CommandOutput { line, .. } => model.push_command_output(&line),
        M::IterationStarted {
            node_id,
            iteration,
            total_iterations,
        } => model.start_iteration(&node_id, iteration, total_iterations),
        M::IterationCompleted {
            node_id, iteration, ..
        } => model.set_iteration(&node_id, iteration),
        M::IterationFailed {
            node_id,
            iteration,
            error,
            ..
        } => model.fail_iteration(&node_id, iteration, &error),
        M::ChildNodeStarted {
            parent_node_id,
            node_id,
            node_type,
        } => model.start_child(&parent_node_id, node_id, node_type),
        M::ChildNodeCompleted {
            parent_node_id,
            node_id,
            duration_ms,
        } => model.complete_child(&parent_node_id, &node_id, duration_ms),
        M::OutputsReady {
            files,
            output_dir,
            warnings,
        } => model.set_outputs(files, output_dir, warnings),
        M::Cancel => model.status = ExecutionStatus::Cancelled,
        M::Tick { elapsed_ms } => model.elapsed_ms = elapsed_ms,
    }
    model
}
