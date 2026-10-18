// Container Node Execution — handles loop, group, and parallel containers.
// Containers organize child nodes rather than processing files directly.
// Loop runs children per-file; group/parallel run on the full batch.

use crate::errors::BntoError;
use crate::events::{PipelineEvent, ProgressOutputFile};
use crate::pipeline::{PipelineDefinition, PipelineFile};

use super::loop_config::{OnErrorStrategy, OutputPersistence, parse_loop_config};
use super::node_outputs::{NodeRecords, merge_records};
use super::{
    ChainPosition, NodeExecutionResult, PipelineContext, PipelineNodeRef, filter_processing_nodes,
    run_node_chain,
};

/// Passthrough result — returns input files unchanged with zero processing.
fn passthrough(files: Vec<PipelineFile>) -> NodeExecutionResult {
    NodeExecutionResult {
        files_processed: 0,
        output_files: files,
        warnings: Vec::new(),
        node_outputs: NodeRecords::new(),
    }
}

/// Extract and clone children (and the edges between them) into a
/// sub-pipeline definition.
/// Returns `None` if children are absent or empty (caller should passthrough).
fn clone_children_definitions(node: PipelineNodeRef) -> Option<PipelineDefinition> {
    let children = node.children.as_ref()?;
    if children.is_empty() {
        return None;
    }
    Some(PipelineDefinition {
        nodes: children.clone(),
        edges: node.edges.clone(),
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
///
/// Iterations are independent: children read the output metadata recorded
/// before the loop and by earlier siblings in their own iteration, never by
/// another iteration. Nodes after the loop read every successful iteration's
/// records merged in order, so a later iteration wins a key both set and a
/// count key adds up.
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
    let mut records = NodeRecords::new();

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
                merge_records(&mut records, exec_result.node_outputs, ctx.count_keys);

                let progress_files =
                    build_progress_files(&config.output_persistence, &exec_result.output_files);
                all_output_files.extend(exec_result.output_files);

                ctx.reporter.emit(PipelineEvent::IterationCompleted {
                    node_id: container_id.to_string(),
                    iteration: i,
                    total_iterations,
                    duration_ms: iter_duration,
                    files_produced,
                    output_files: progress_files,
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
                    output_files: Vec::new(),
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
        node_outputs: records,
    })
}

/// Convert iteration output files to `ProgressOutputFile` for event emission.
/// Returns an empty vec in deferred mode (default) to avoid cloning file data.
///
/// For `Path` variant files, this clones only the ~50-byte PathBuf reference,
/// not the multi-GB file content. The consumer (CLI progress reporter, TUI bridge)
/// uses `write_to()` to move the file to its destination.
fn build_progress_files(
    persistence: &OutputPersistence,
    files: &[PipelineFile],
) -> Vec<ProgressOutputFile> {
    match persistence {
        OutputPersistence::Deferred => Vec::new(),
        OutputPersistence::Progressive => files
            .iter()
            .map(|f| ProgressOutputFile {
                name: f.name.clone(),
                data: f.data.clone(),
                mime_type: f.mime_type.clone(),
            })
            .collect(),
    }
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
        loop_item: item_data,
        parent_node_id: Some(container_id.to_string()),
        ..ctx.layer()
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
        parent_node_id: Some(container_id.to_string()),
        ..ctx.layer()
    };
    execute_sub_pipeline(&group_ctx, sub_definition, files, file_offset)
}

/// Execute a sub-pipeline (container children). Same as `execute_pipeline`
/// but without PipelineStarted/PipelineCompleted events. The children
/// follow the closing-step rule among themselves; a failed sub-pipeline's
/// error hands its last files on to the enclosing chain.
fn execute_sub_pipeline<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let processing_nodes = filter_processing_nodes(definition);
    let position = ChainPosition::whole(processing_nodes.len());
    run_node_chain(ctx, &processing_nodes, files, file_offset, position)
        .map_err(super::closing::ChainFailure::into_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineNode;

    fn container(json: &str) -> PipelineNode {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn sub_definition_carries_the_containers_edges() {
        let node = container(
            r#"{ "id": "g", "type": "group",
                 "nodes": [ { "id": "a", "type": "x" }, { "id": "b", "type": "x" } ],
                 "edges": [ { "id": "e1", "source": "a", "target": "b" } ] }"#,
        );
        let sub = clone_children_definitions(&node).expect("has children");
        assert_eq!(sub.edges, node.edges);
        assert_eq!(sub.nodes.len(), 2);
    }

    #[test]
    fn sub_definition_of_an_edgeless_container_has_no_edges() {
        let node =
            container(r#"{ "id": "g", "type": "loop", "nodes": [ { "id": "a", "type": "x" } ] }"#);
        let sub = clone_children_definitions(&node).expect("has children");
        assert!(sub.edges.is_empty());
    }
}
