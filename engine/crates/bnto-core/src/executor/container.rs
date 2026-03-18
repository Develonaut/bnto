// Container Node Execution — handles loop, group, and parallel containers.
// Containers organize child nodes rather than processing files directly.
// Loop runs children per-file; group/parallel run on the full batch.

use crate::errors::BntoError;
use crate::pipeline::{PipelineDefinition, PipelineFile, PipelineNode, is_io_node};

use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef, run_node_chain};

/// Passthrough result — returns input files unchanged with zero processing.
fn passthrough(files: Vec<PipelineFile>) -> NodeExecutionResult {
    NodeExecutionResult {
        files_processed: 0,
        output_files: files,
    }
}

/// Extract and clone children into a sub-pipeline definition.
/// Returns `None` if children are absent or empty (caller should passthrough).
fn clone_children_definitions(node: PipelineNodeRef) -> Option<PipelineDefinition> {
    let children = node.children.as_ref()?;
    if children.is_empty() {
        return None;
    }
    Some(PipelineDefinition {
        nodes: children.clone(),
    })
}

/// Execute a container node (loop, group, parallel) by recursing into children.
///
/// - **loop**: children get ONE file at a time (per-file iteration)
/// - **group/parallel**: children get ALL files as a batch
pub(super) fn execute_container_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let sub_definition = match clone_children_definitions(node) {
        Some(def) => def,
        None => return Ok(passthrough(files)),
    };

    match node.node_type.as_str() {
        "loop" => execute_loop(ctx, &sub_definition, files, file_offset),
        "group" | "parallel" => execute_group(ctx, &sub_definition, files, file_offset),
        _ => Ok(passthrough(files)),
    }
}

/// Run the sub-pipeline once per file, collecting all outputs.
fn execute_loop<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;

    for (i, file) in files.into_iter().enumerate() {
        let result = execute_sub_pipeline(ctx, sub_definition, vec![file], file_offset + i)?;
        total_processed += result.files_processed;
        all_output_files.extend(result.output_files);
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
    })
}

/// Run the sub-pipeline once on the full batch of files.
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

/// Execute a sub-pipeline (container children). Same as `execute_pipeline`
/// but without PipelineStarted/PipelineCompleted events.
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

    let (output_files, files_processed) =
        run_node_chain(ctx, &processing_nodes, files, file_offset)?;

    Ok(NodeExecutionResult {
        files_processed,
        output_files,
    })
}
