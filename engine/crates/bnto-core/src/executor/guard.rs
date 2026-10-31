// A node's `when` guard, decided just before the node would start: resolve
// its value and expected with the node's own template context (the fields it
// and its containers declare, ctx, earlier records, the loop item), compare.

use super::template::TemplateContext;
use super::{NodeExecutionResult, PipelineContext};
use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::{PipelineFile, PipelineNode};
use crate::when::Guard;

/// What a guard decided for one call of a node.
pub(super) enum Decision {
    /// Run the node on these files.
    Run(Vec<PipelineFile>),
    /// The node was skipped and reported; this is its result.
    Skipped(NodeExecutionResult),
}

/// Run `node`, or report it skipped and hand `files` on.
pub(super) fn decide<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    position: (usize, usize),
    files: Vec<PipelineFile>,
) -> Result<Decision, BntoError> {
    Ok(match skip_reason(ctx, node)? {
        Some(reason) => Decision::Skipped(skip(ctx, node, position, reason, files)),
        None => Decision::Run(files),
    })
}

/// Why `node` is skipped, or `None` when it has no guard or its guard holds.
/// An invalid guard (an unknown op) is an error, like any bad param.
fn skip_reason<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
) -> Result<Option<String>, BntoError> {
    let Some(when) = &node.when else {
        return Ok(None);
    };
    let field_values = ctx.fields_for(node);
    let tpl = TemplateContext {
        field_values: &field_values,
        process_ctx: ctx.process_ctx,
        node_outputs: &ctx.node_outputs,
        loop_item: &ctx.loop_item,
        loop_row: ctx.loop_item.as_ref(),
        run_started_ms: Some(ctx.started_ms),
    };
    match when.decide_in(&tpl) {
        Ok(Guard::Run) => Ok(None),
        Ok(Guard::Skip { reason }) => Ok(Some(reason)),
        Err(message) => Err(BntoError::InvalidInput(format!("{}: {message}", node.id))),
    }
}

/// Report `node` skipped and hand its input files on, untouched and with no
/// record, as if the step weren't there.
fn skip<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    (node_index, total_nodes): (usize, usize),
    reason: String,
    files: Vec<PipelineFile>,
) -> NodeExecutionResult {
    ctx.reporter.emit(PipelineEvent::NodeSkipped {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
        parent_node_id: ctx.parent_node_id.clone(),
        reason,
    });
    NodeExecutionResult {
        output_files: files,
        files_processed: 0,
        warnings: Vec::new(),
        node_outputs: Default::default(),
    }
}
