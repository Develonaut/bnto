// Pipeline Executor — top-level orchestrator for pipeline execution.
//
// Walks nodes in order, chaining outputs to inputs. Split across three files:
//   - `mod.rs` — public API, node dispatch, shared types
//   - `primitive.rs` — leaf node execution (image compress, file rename, etc.)
//   - `container.rs` — container node execution (loop, group, parallel)
//
// Pure Rust — no WASM deps. Works with `cargo test` natively.
// The WASM bridge (`bnto-wasm/src/execute.rs`) handles JS type conversions.

mod auto_iteration;
mod container;
mod primitive;
pub mod resolve;

use crate::context::ProcessContext;
use crate::errors::BntoError;
use crate::events::{NodeInfo, PipelineEvent, PipelineReporter};
use crate::pipeline::{
    IterationMode, PipelineDefinition, PipelineFile, PipelineFileResult, PipelineNode,
    PipelineResult, is_container_node, is_io_node,
};
use crate::registry::NodeRegistry;

#[cfg(test)]
use crate::processor::NodeInput;
#[cfg(test)]
use crate::progress::ProgressReporter;

// --- Shared Types ---

type PipelineNodeRef<'a> = &'a PipelineNode;

/// Bundles shared immutable state for executor functions, keeping
/// signatures clean and under clippy's 7-argument limit.
struct PipelineContext<'a, F: Fn() -> u64 + Copy> {
    registry: &'a NodeRegistry,
    reporter: &'a PipelineReporter,
    /// System access boundary for processors (commands, temp files, env vars).
    process_ctx: &'a dyn ProcessContext,
    /// Original input file count — used in progress events so the UI
    /// reports global position even inside loop container iterations.
    pipeline_total_files: usize,
    /// Returns current time in ms (injected for testability in WASM).
    now_ms: F,
}

/// Result of executing a single node or container sub-pipeline.
struct NodeExecutionResult {
    output_files: Vec<PipelineFile>,
    files_processed: usize,
}

// --- Public API ---

/// Filter out I/O marker nodes (input/output) that don't perform processing.
fn filter_processing_nodes(definition: &PipelineDefinition) -> Vec<&PipelineNode> {
    definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect()
}

/// Convert final pipeline files to result format, preserving processor metadata.
fn build_pipeline_result(files: Vec<PipelineFile>, duration_ms: u64) -> PipelineResult {
    let result_files = files
        .into_iter()
        .map(|f| PipelineFileResult {
            name: f.name,
            data: f.data,
            mime_type: f.mime_type,
            metadata: f.metadata,
        })
        .collect();
    PipelineResult {
        files: result_files,
        duration_ms,
    }
}

/// Execute a complete pipeline: walk nodes, iterate files, chain outputs.
///
/// Main entry point for the engine. `now_ms` is injected for testability
/// (no `std::time::Instant` in WASM — uses `js_sys::Date::now()` instead).
pub fn execute_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    process_ctx: &dyn ProcessContext,
    now_ms: impl Fn() -> u64 + Copy,
) -> Result<PipelineResult, BntoError> {
    let start_ms = now_ms();
    let processing_nodes = filter_processing_nodes(definition);
    let ctx = PipelineContext {
        registry,
        reporter,
        process_ctx,
        pipeline_total_files: files.len(),
        now_ms,
    };

    let node_infos: Vec<NodeInfo> = processing_nodes
        .iter()
        .map(|n| NodeInfo {
            id: n.id.clone(),
            node_type: n.node_type.clone(),
        })
        .collect();

    ctx.reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes: processing_nodes.len(),
        total_files: files.len(),
        nodes: node_infos,
    });

    let (current_files, total_files_processed) = match definition.resolved_iteration() {
        IterationMode::Explicit => run_node_chain(&ctx, &processing_nodes, files, 0)?,
        IterationMode::Auto => auto_iteration::run_auto_iteration(&ctx, &processing_nodes, files)?,
    };

    let duration_ms = (ctx.now_ms)() - start_ms;
    ctx.reporter.emit(PipelineEvent::PipelineCompleted {
        duration_ms,
        total_files_processed,
    });

    Ok(build_pipeline_result(current_files, duration_ms))
}

/// Chain nodes sequentially, passing each node's output as the next node's input.
fn run_node_chain<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<(Vec<PipelineFile>, usize), BntoError> {
    let total_nodes = nodes.len();
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in nodes.iter().enumerate() {
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

    Ok((current_files, total_files_processed))
}

// --- Internal: Node Dispatch ---

/// Emit failure events (NodeFailed + PipelineFailed) and return the error.
fn emit_node_failure<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    error: BntoError,
) -> BntoError {
    let error_msg = error.to_string();
    ctx.reporter.emit(PipelineEvent::NodeFailed {
        node_id: node_id.to_string(),
        error: error_msg.clone(),
    });
    ctx.reporter.emit(PipelineEvent::PipelineFailed {
        node_id: node_id.to_string(),
        error: error_msg,
    });
    error
}

/// Execute a single node — dispatches to primitive or container handler.
///
/// Emits NodeStarted, delegates to the appropriate handler, then emits
/// NodeCompleted or NodeFailed. Container children call back into this
/// for recursive execution.
fn execute_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    node_index: usize,
    total_nodes: usize,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = (ctx.now_ms)();
    emit_node_started(ctx, node, node_index, total_nodes);

    let result = dispatch_node(ctx, node, files, file_offset);

    match result {
        Ok(exec_result) => {
            emit_node_completed(ctx, &node.id, node_start, exec_result.files_processed);
            Ok(exec_result)
        }
        Err(error) => Err(emit_node_failure(ctx, &node.id, error)),
    }
}

fn emit_node_started<F: Fn() -> u64 + Copy>(
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
    });
}

fn emit_node_completed<F: Fn() -> u64 + Copy>(
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
    });
}

fn dispatch_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    if is_container_node(&node.node_type) {
        container::execute_container_node(ctx, node, files, file_offset)
    } else {
        primitive::execute_primitive_node(ctx, node, files, file_offset)
    }
}

#[cfg(test)]
mod tests;
