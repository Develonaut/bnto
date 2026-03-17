// Pipeline executor — walks nodes, iterates files, chains outputs.
//
// Split across three files:
//   - mod.rs (this file) — public API, node dispatch, shared types
//   - primitive.rs — executing leaf nodes (image compress, file rename, etc.)
//   - container.rs — executing container nodes (loop, group, parallel)
//
// Pure Rust, no WASM dependencies. The WASM bridge (bnto-wasm/src/execute.rs)
// wraps this with JS type conversions.

mod container;
mod primitive;

use crate::errors::BntoError;
use crate::events::{PipelineEvent, PipelineReporter};
use crate::pipeline::{
    PipelineDefinition, PipelineFile, PipelineFileResult, PipelineNode, PipelineResult,
    is_container_node, is_io_node,
};
use crate::registry::NodeRegistry;

// Used by test module via `use super::*`.
#[cfg(test)]
use crate::processor::NodeInput;
#[cfg(test)]
use crate::progress::ProgressReporter;

// =============================================================================
// Shared Types
// =============================================================================

type PipelineNodeRef<'a> = &'a PipelineNode;

/// Bundles shared, immutable state that every executor function needs.
/// Keeps function signatures under clippy's 7-argument limit.
struct PipelineContext<'a, F: Fn() -> u64 + Copy> {
    registry: &'a NodeRegistry,
    reporter: &'a PipelineReporter,
    /// Original input file count — used in FileProgress so the UI says "X of 4"
    /// even when loop containers split files into single-file batches.
    pipeline_total_files: usize,
    /// Injected clock for testability (no std::time in WASM).
    now_ms: F,
}

/// Internal result of executing a single node or container sub-pipeline.
struct NodeExecutionResult {
    output_files: Vec<PipelineFile>,
    files_processed: usize,
}

// =============================================================================
// Public API
// =============================================================================

/// Execute a complete pipeline: walk nodes, iterate files, chain outputs.
///
/// `now_ms` is injected because WASM has no std::time::Instant — callers
/// provide `js_sys::Date::now` (browser) or a fake clock (tests).
pub fn execute_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    now_ms: impl Fn() -> u64 + Copy,
) -> Result<PipelineResult, BntoError> {
    let start_ms = now_ms();

    // Filter I/O marker nodes — they're structural, not processing.
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let total_files = files.len();

    let ctx = PipelineContext {
        registry,
        reporter,
        pipeline_total_files: total_files,
        now_ms,
    };

    ctx.reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    });

    // Execute nodes sequentially, chaining outputs -> inputs.
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        let result = execute_node(&ctx, node, current_files, node_index, total_nodes, 0)?;

        total_files_processed += result.files_processed;
        current_files = result.output_files;
    }

    let duration_ms = (ctx.now_ms)() - start_ms;

    ctx.reporter.emit(PipelineEvent::PipelineCompleted {
        duration_ms,
        total_files_processed,
    });

    // Preserve processor metadata (compression ratio, etc.) in final results.
    let result_files: Vec<PipelineFileResult> = current_files
        .into_iter()
        .map(|f| PipelineFileResult {
            name: f.name,
            data: f.data,
            mime_type: f.mime_type,
            metadata: f.metadata,
        })
        .collect();

    Ok(PipelineResult {
        files: result_files,
        duration_ms,
    })
}

// =============================================================================
// Internal: Node Dispatch
// =============================================================================

/// Execute a single node — dispatches to primitive or container handler.
/// Emits NodeStarted, then NodeCompleted/NodeFailed + PipelineFailed.
fn execute_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    node_index: usize,
    total_nodes: usize,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = (ctx.now_ms)();

    ctx.reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
    });

    let result = if is_container_node(&node.node_type) {
        container::execute_container_node(ctx, node, files, file_offset)
    } else {
        primitive::execute_primitive_node(ctx, node, files, file_offset)
    };

    match result {
        Ok(exec_result) => {
            let duration_ms = (ctx.now_ms)() - node_start;
            ctx.reporter.emit(PipelineEvent::NodeCompleted {
                node_id: node.id.clone(),
                duration_ms,
                files_processed: exec_result.files_processed,
            });
            Ok(exec_result)
        }
        Err(error) => {
            ctx.reporter.emit(PipelineEvent::NodeFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            ctx.reporter.emit(PipelineEvent::PipelineFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            Err(error)
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
