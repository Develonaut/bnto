// =============================================================================
// Pipeline Executor — The Engine's Brain
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the top-level orchestrator for pipeline execution. It takes a
// pipeline definition (a list of nodes), a batch of input files, and a
// registry of processors, then walks the nodes in order, chaining each
// node's outputs into the next node's inputs.
//
// The actual work is split across three files:
//   - `mod.rs` (this file) — public API, node dispatch, shared types
//   - `primitive.rs` — executing leaf nodes (image compress, file rename, etc.)
//   - `container.rs` — executing container nodes (loop, group, parallel)
//
// WHY IS THIS IN RUST (not JavaScript)?
// Previously, ALL orchestration lived in JS (`executePipeline.ts`). The Rust
// engine only knew how to process ONE file at a time. This meant:
//   - The "intelligence" was in the wrong layer (JS, not the engine)
//   - Every new consumer (CLI, desktop, server) would need to reimplement
//     the orchestration logic
//   - Complex recipes (loops, groups, nested nodes) would need to be built
//     twice (JS for browser, native for everything else)
//
// By moving the executor to Rust, we get ONE implementation that works
// everywhere: browser (WASM), CLI (native binary), desktop (Tauri), server.
//
// HOW IT FITS WITH EXISTING CODE:
// - `NodeProcessor` trait (processor.rs) — the executor calls this per-file
// - `ProgressReporter` (progress.rs) — wraps per-file progress into events
// - `NodeRegistry` (registry.rs) — looks up the right processor for each node
// - `PipelineReporter` (events.rs) — emits structured events to the consumer
//
// The executor is PURE RUST — no WASM dependencies, no JS types. It works
// with `cargo test` natively. The WASM bridge (`bnto-wasm/src/execute.rs`)
// wraps this with JS type conversions.

// --- Submodules ---
// Each submodule handles one kind of node execution.
mod container;
mod primitive;

use crate::errors::BntoError;
use crate::events::{PipelineEvent, PipelineReporter};
use crate::pipeline::{
    PipelineDefinition, PipelineFile, PipelineFileResult, PipelineNode, PipelineResult,
    is_container_node, is_io_node,
};
use crate::registry::NodeRegistry;

// These imports are used by the test module (via `use super::*`).
// They're not needed by mod.rs itself — primitive.rs and container.rs
// import them directly. The `#[cfg(test)]` gate prevents unused-import
// warnings in non-test builds.
#[cfg(test)]
use crate::processor::NodeInput;
#[cfg(test)]
use crate::progress::ProgressReporter;

// =============================================================================
// Shared Types — Used by Both Submodules
// =============================================================================

/// A borrowed reference to a PipelineNode.
///
/// This type alias keeps function signatures clean. Both `primitive.rs` and
/// `container.rs` receive borrowed node references — they don't own the nodes.
///
/// RUST CONCEPT: `&'_ PipelineNode`
/// The `'_` is a lifetime placeholder. It means "the compiler will figure out
/// how long this reference lives." We don't need to spell it out because the
/// borrow is always shorter than the pipeline execution.
type PipelineNodeRef<'a> = &'a PipelineNode;

/// Bundles the shared, immutable state that every executor function needs.
///
/// Without this struct, each internal function would need 4+ extra parameters
/// (registry, reporter, pipeline_total_files, now_ms). Bundling them keeps
/// function signatures clean and under clippy's 7-argument limit.
///
/// RUST CONCEPT: `<'a, F: Fn() -> u64 + Copy>`
/// - `'a` is a lifetime parameter — it says "this struct borrows data that
///   must live at least as long as the struct itself."
/// - `F: Fn() -> u64 + Copy` is a trait bound — `F` is any type that can be
///   called like a function returning `u64` AND can be cheaply copied (like
///   a function pointer or small closure with no captured heap data).
struct PipelineContext<'a, F: Fn() -> u64 + Copy> {
    /// Maps node types to processor implementations.
    registry: &'a NodeRegistry,
    /// Receives structured progress events for the consumer.
    reporter: &'a PipelineReporter,
    /// The ORIGINAL number of input files the user provided.
    /// Used in FileProgress events so the UI says "X of 4", not "1 of 1"
    /// even when loop containers split files into single-file batches.
    pipeline_total_files: usize,
    /// Returns current time in milliseconds (injected for testability).
    now_ms: F,
}

/// The result of executing a single node (or container sub-pipeline).
/// Used internally to chain outputs between nodes.
struct NodeExecutionResult {
    /// The output files from this node (become input for the next node).
    output_files: Vec<PipelineFile>,
    /// How many files this node processed (for progress tracking).
    files_processed: usize,
}

// =============================================================================
// Public API
// =============================================================================

/// Execute a complete pipeline: walk nodes, iterate files, chain outputs.
///
/// This is the main entry point for the engine. Every consumer (browser WASM,
/// CLI, desktop, server) calls this function with its own adapter for time
/// and progress reporting.
///
/// # Arguments
/// - `definition` — the parsed pipeline definition (nodes in order)
/// - `files` — the input files to process
/// - `registry` — maps node types to processor implementations
/// - `reporter` — receives structured progress events
/// - `now_ms` — returns current time in milliseconds (injected for testability)
///
/// # Returns
/// - `Ok(PipelineResult)` — all files processed successfully
/// - `Err(BntoError)` — a node failed, pipeline stopped
///
/// RUST CONCEPT: `impl Fn() -> u64`
/// The `now_ms` parameter is a closure that returns the current time.
/// We inject it instead of calling `std::time::Instant::now()` directly
/// because:
///   - In WASM, there's no `std::time::Instant` — we use `js_sys::Date::now()`
///   - In tests, we can inject a fake clock for deterministic timing
///   - This keeps the executor target-agnostic (no platform-specific imports)
pub fn execute_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    now_ms: impl Fn() -> u64 + Copy,
) -> Result<PipelineResult, BntoError> {
    // --- Step 1: Record the start time ---
    let start_ms = now_ms();

    // --- Step 2: Filter out I/O marker nodes to find processing nodes ---
    // I/O nodes ("input", "output") are structural markers in the recipe
    // definition. They tell the editor where files enter and leave the
    // pipeline, but they don't perform any processing.
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let total_files = files.len();

    // Bundle shared state into a context struct so internal functions
    // stay under clippy's 7-argument limit.
    let ctx = PipelineContext {
        registry,
        reporter,
        pipeline_total_files: total_files,
        now_ms,
    };

    // --- Step 3: Emit PipelineStarted event ---
    ctx.reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    });

    // --- Step 4: Execute processing nodes sequentially ---
    // The current batch of files starts as the input files. After each
    // node processes them, the output becomes the input for the next node.
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        // Execute this node (dispatches to primitive or container).
        let result = execute_node(
            &ctx,
            node,
            current_files,
            node_index,
            total_nodes,
            0, // file_offset — starts at 0 for top-level nodes
        )?;

        // Track total files processed across all nodes.
        total_files_processed += result.files_processed;

        // Chain: this node's output becomes the next node's input.
        current_files = result.output_files;
    }

    // --- Step 5: Calculate total duration and emit PipelineCompleted ---
    let duration_ms = (ctx.now_ms)() - start_ms;

    ctx.reporter.emit(PipelineEvent::PipelineCompleted {
        duration_ms,
        total_files_processed,
    });

    // --- Step 6: Convert final files to PipelineFileResults ---
    // Preserve metadata from processors (compression ratio, original size, etc.)
    // so the UI can display stats like "42% smaller".
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
///
/// This is the recursive workhorse of the executor. It:
/// 1. Emits a NodeStarted event
/// 2. Delegates to `primitive::execute_primitive_node` or
///    `container::execute_container_node` based on the node type
/// 3. Emits NodeCompleted (success) or NodeFailed + PipelineFailed (error)
///
/// Both submodules call back into this function for recursive execution
/// (e.g., a container's children may themselves be containers).
fn execute_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    node_index: usize,
    total_nodes: usize,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = (ctx.now_ms)();

    // --- Emit NodeStarted event ---
    ctx.reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
    });

    // --- Decide: container or primitive? ---
    // `is_container_node` checks if the type is "loop", "group", or "parallel".
    let result = if is_container_node(&node.node_type) {
        container::execute_container_node(ctx, node, files, file_offset)
    } else {
        primitive::execute_primitive_node(ctx, node, files, file_offset)
    };

    // --- Handle success or failure ---
    // `match` on the Result to emit the right event before returning.
    match result {
        Ok(exec_result) => {
            // Emit NodeCompleted on success.
            let duration_ms = (ctx.now_ms)() - node_start;
            ctx.reporter.emit(PipelineEvent::NodeCompleted {
                node_id: node.id.clone(),
                duration_ms,
                files_processed: exec_result.files_processed,
            });
            Ok(exec_result)
        }
        Err(error) => {
            // Emit NodeFailed, then PipelineFailed, then propagate the error.
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
