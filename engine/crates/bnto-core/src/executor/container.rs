// Container node execution — recurses into child sub-pipelines.
//
// Container types:
//   - loop     — run children once PER file (for-each)
//   - group    — run children once on ALL files (batch)
//   - parallel — same as group for now (concurrent execution is future work)

use crate::errors::BntoError;
use crate::pipeline::{PipelineDefinition, PipelineFile, PipelineNode, is_io_node};

use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef, execute_node};

// =============================================================================
// Public (to parent module) API
// =============================================================================

/// Execute a container node by recursing into its children.
pub(super) fn execute_container_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let children = match &node.children {
        Some(c) => c,
        None => {
            // Container with no children = passthrough.
            return Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            });
        }
    };

    if children.is_empty() {
        return Ok(NodeExecutionResult {
            files_processed: 0,
            output_files: files,
        });
    }

    // Build a sub-pipeline from children. Clone needed because we still
    // borrow `node` through `children`.
    let sub_definition = PipelineDefinition {
        nodes: children.clone(),
    };

    match node.node_type.as_str() {
        "loop" => execute_loop(ctx, &sub_definition, files, file_offset),
        // "parallel" is identical to "group" for now.
        "group" | "parallel" => execute_group(ctx, &sub_definition, files, file_offset),
        _ => {
            // Unknown container type — passthrough (defensive).
            Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            })
        }
    }
}

// =============================================================================
// Internal: Loop Container
// =============================================================================

/// Run the sub-pipeline once PER file. Results collected into one output Vec.
fn execute_loop<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;

    for (i, file) in files.into_iter().enumerate() {
        let single_file_batch = vec![file];

        let result = execute_sub_pipeline(ctx, sub_definition, single_file_batch, file_offset + i)?;

        total_processed += result.files_processed;
        all_output_files.extend(result.output_files);
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
    })
}

// =============================================================================
// Internal: Group / Parallel Container
// =============================================================================

/// Run the sub-pipeline once on the FULL batch of files.
fn execute_group<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let result = execute_sub_pipeline(ctx, sub_definition, files, file_offset)?;
    Ok(NodeExecutionResult {
        files_processed: result.files_processed,
        output_files: result.output_files,
    })
}

// =============================================================================
// Internal: Sub-Pipeline Execution
// =============================================================================

/// Execute a sub-pipeline (children of a container).
/// Same as execute_pipeline but WITHOUT PipelineStarted/PipelineCompleted events
/// — those belong to the outer pipeline only.
fn execute_sub_pipeline<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        let result = execute_node(
            ctx,
            node,
            current_files,
            node_index,
            total_nodes,
            file_offset,
        )?;
        total_files_processed += result.files_processed;
        current_files = result.output_files;
    }

    Ok(NodeExecutionResult {
        files_processed: total_files_processed,
        output_files: current_files,
    })
}
