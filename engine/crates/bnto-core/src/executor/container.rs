// Container Node Execution — handles loop, group, and parallel containers.
// Containers organize child nodes rather than processing files directly.
// Loop runs children per-file; group/parallel run on the full batch.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::{PipelineDefinition, PipelineFile, PipelineNode, is_io_node};

use super::loop_config::{OnErrorStrategy, parse_loop_config};
use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef, run_node_chain};

/// Passthrough result — returns input files unchanged with zero processing.
fn passthrough(files: Vec<PipelineFile>) -> NodeExecutionResult {
    NodeExecutionResult {
        files_processed: 0,
        output_files: files,
        warnings: Vec::new(),
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
        settings: None,
        requires: Vec::new(),
        secrets: Vec::new(),
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
        "loop" => execute_loop(
            ctx,
            &node.id,
            &node.params,
            &sub_definition,
            files,
            file_offset,
        ),
        "group" | "parallel" => execute_group(ctx, &node.id, &sub_definition, files, file_offset),
        _ => Ok(passthrough(files)),
    }
}

/// Run the sub-pipeline once per file, collecting all outputs.
/// Sets `loop_item` from each file's metadata so child nodes can use `{{item.*}}`.
///
/// When `onError: "continue"`, failed iterations are skipped with a warning
/// instead of killing the entire loop. If ALL iterations fail, the loop fails.
fn execute_loop<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    container_id: &str,
    loop_params: &serde_json::Map<String, serde_json::Value>,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let config = parse_loop_config(loop_params);
    let total_iterations = files.len();
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;
    let mut warnings: Vec<String> = Vec::new();

    for (i, file) in files.into_iter().enumerate() {
        let iter_start = (ctx.now_ms)();

        ctx.reporter.emit(PipelineEvent::IterationStarted {
            node_id: container_id.to_string(),
            iteration: i,
            total_iterations,
        });

        let loop_ctx = build_loop_context(ctx, container_id, &file);
        let result = execute_sub_pipeline(&loop_ctx, sub_definition, vec![file], file_offset + i);

        match result {
            Ok(exec_result) => {
                let iter_duration = (ctx.now_ms)() - iter_start;
                let files_produced = exec_result.output_files.len();
                total_processed += exec_result.files_processed;
                warnings.extend(exec_result.warnings);
                all_output_files.extend(exec_result.output_files);

                ctx.reporter.emit(PipelineEvent::IterationCompleted {
                    node_id: container_id.to_string(),
                    iteration: i,
                    total_iterations,
                    duration_ms: iter_duration,
                    files_produced,
                });
            }
            Err(error) => {
                let iter_duration = (ctx.now_ms)() - iter_start;
                let error_msg = error.to_string();

                ctx.reporter.emit(PipelineEvent::IterationFailed {
                    node_id: container_id.to_string(),
                    iteration: i,
                    total_iterations,
                    error: error_msg.clone(),
                });

                match config.on_error {
                    OnErrorStrategy::FailFast => return Err(error),
                    OnErrorStrategy::Continue => {
                        warnings.push(format!(
                            "Iteration {}/{}: {}",
                            i + 1,
                            total_iterations,
                            error_msg
                        ));
                    }
                }

                // Emit zero-file completion so TUI progress tracking stays consistent.
                ctx.reporter.emit(PipelineEvent::IterationCompleted {
                    node_id: container_id.to_string(),
                    iteration: i,
                    total_iterations,
                    duration_ms: iter_duration,
                    files_produced: 0,
                });
            }
        }
    }

    // If all iterations failed, the loop itself fails.
    if all_output_files.is_empty() && !warnings.is_empty() {
        return Err(BntoError::ProcessingFailed(format!(
            "All {} iterations failed",
            total_iterations
        )));
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
        warnings,
    })
}

/// Build a PipelineContext for a single loop iteration.
fn build_loop_context<'a, F: Fn() -> u64 + Copy>(
    ctx: &'a PipelineContext<'a, F>,
    container_id: &str,
    file: &PipelineFile,
) -> PipelineContext<'a, F> {
    let item_data = if file.metadata.is_empty() {
        None
    } else {
        Some(file.metadata.clone())
    };
    PipelineContext {
        registry: ctx.registry,
        reporter: ctx.reporter,
        process_ctx: ctx.process_ctx,
        pipeline_total_files: ctx.pipeline_total_files,
        now_ms: ctx.now_ms,
        loop_item: item_data,
        parent_node_id: Some(container_id.to_string()),
    }
}

/// Run the sub-pipeline once on the full batch of files.
fn execute_group<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    container_id: &str,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let group_ctx = PipelineContext {
        registry: ctx.registry,
        reporter: ctx.reporter,
        process_ctx: ctx.process_ctx,
        pipeline_total_files: ctx.pipeline_total_files,
        now_ms: ctx.now_ms,
        loop_item: ctx.loop_item.clone(),
        parent_node_id: Some(container_id.to_string()),
    };
    execute_sub_pipeline(&group_ctx, sub_definition, files, file_offset)
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

    let (output_files, files_processed, warnings) =
        run_node_chain(ctx, &processing_nodes, files, file_offset)?;

    Ok(NodeExecutionResult {
        files_processed,
        output_files,
        warnings,
    })
}
