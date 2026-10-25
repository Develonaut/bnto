// A node's lifecycle events: NodeStarted, then NodeCompleted or NodeFailed,
// each naming the container it runs in.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::PipelineNode;

use super::PipelineContext;

/// Emit NodeFailed and return the error. The run's PipelineFailed comes
/// later, from `fail_pipeline`, once the closing steps have run.
pub(super) fn emit_node_failure<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    error: BntoError,
) -> BntoError {
    ctx.reporter.emit(PipelineEvent::NodeFailed {
        node_id: node_id.to_string(),
        error: error.to_string(),
        parent_node_id: ctx.parent_node_id.clone(),
    });
    error
}

pub(super) fn emit_node_started<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    node_index: usize,
    total_nodes: usize,
) {
    ctx.reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
        parent_node_id: ctx.parent_node_id.clone(),
    });
}

pub(super) fn emit_node_completed<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    node_start: u64,
    files_processed: usize,
) {
    let duration_ms = (ctx.now_ms)() - node_start;
    ctx.reporter.emit(PipelineEvent::NodeCompleted {
        node_id: node_id.to_string(),
        duration_ms,
        files_processed,
        parent_node_id: ctx.parent_node_id.clone(),
    });
}
