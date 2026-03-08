// =============================================================================
// Pipeline Executor — The Engine's Brain
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the heart of the engine. It takes a pipeline definition (a list of
// nodes), a batch of input files, and a registry of processors, then:
//   1. Walks the nodes in order
//   2. Skips I/O marker nodes (input, output)
//   3. For each processing node, iterates files and calls the right processor
//   4. Chains outputs → inputs between sequential nodes
//   5. Handles container nodes (loop, group, parallel) via recursion
//   6. Emits structured progress events at every stage
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
// - `ProgressReporter` (progress.rs) — wraps per-file progress into pipeline events
// - `NodeRegistry` (registry.rs) — looks up the right processor for each node
// - `PipelineReporter` (events.rs) — emits structured events to the consumer
//
// The executor is PURE RUST — no WASM dependencies, no JS types. It works
// with `cargo test` natively. The WASM bridge (`bnto-wasm/src/execute.rs`)
// wraps this with JS type conversions.

use crate::errors::BntoError;
use crate::events::{PipelineEvent, PipelineReporter};
use crate::pipeline::{
    PipelineDefinition, PipelineFile, PipelineFileResult, PipelineNode, PipelineResult,
    is_container_node, is_io_node,
};
use crate::processor::NodeInput;
use crate::progress::ProgressReporter;
use crate::registry::NodeRegistry;

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

    // --- Step 3: Emit PipelineStarted event ---
    reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    });

    // --- Step 4: Execute processing nodes sequentially ---
    // The current batch of files starts as the input files. After each
    // node processes them, the output becomes the input for the next node.
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        // Execute this node (handles both primitive and container nodes).
        // Pass `total_files` (the original input count) so that progress
        // events always report "X of N" relative to the user's input batch,
        // even when container nodes (loop) split files into single-file batches.
        let result = execute_node(
            node,
            current_files,
            registry,
            reporter,
            node_index,
            total_nodes,
            total_files,
            0, // file_offset — starts at 0 for top-level nodes
            &now_ms,
        )?;

        // Track total files processed across all nodes.
        total_files_processed += result.files_processed;

        // Chain: this node's output becomes the next node's input.
        current_files = result.output_files;
    }

    // --- Step 5: Calculate total duration and emit PipelineCompleted ---
    let duration_ms = now_ms() - start_ms;

    reporter.emit(PipelineEvent::PipelineCompleted {
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
// Internal: Node Execution Result
// =============================================================================

/// The result of executing a single node (or container sub-pipeline).
/// Used internally to chain outputs between nodes.
struct NodeExecutionResult {
    /// The output files from this node (become input for the next node).
    output_files: Vec<PipelineFile>,
    /// How many files this node processed (for progress tracking).
    files_processed: usize,
}

// =============================================================================
// Internal: Execute a Single Node
// =============================================================================

/// Execute a single node — either a primitive processor or a container.
///
/// This is the recursive workhorse. For primitive nodes, it iterates files
/// and calls the processor. For container nodes, it recurses into children.
fn execute_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    node_index: usize,
    total_nodes: usize,
    pipeline_total_files: usize,
    file_offset: usize,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = now_ms();

    // --- Emit NodeStarted event ---
    reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
    });

    // --- Decide: container or primitive? ---
    // Pass pipeline_total_files and file_offset so that progress events
    // always report relative to the user's original input batch.
    let result = if is_container_node(&node.node_type) {
        execute_container_node(
            node,
            files,
            registry,
            reporter,
            pipeline_total_files,
            file_offset,
            now_ms,
        )
    } else {
        execute_primitive_node(
            node,
            files,
            registry,
            reporter,
            pipeline_total_files,
            file_offset,
        )
    };

    // --- Handle success or failure ---
    match result {
        Ok(exec_result) => {
            // Emit NodeCompleted on success.
            let duration_ms = now_ms() - node_start;
            reporter.emit(PipelineEvent::NodeCompleted {
                node_id: node.id.clone(),
                duration_ms,
                files_processed: exec_result.files_processed,
            });
            Ok(exec_result)
        }
        Err(error) => {
            // Emit NodeFailed, then PipelineFailed, then propagate.
            reporter.emit(PipelineEvent::NodeFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            reporter.emit(PipelineEvent::PipelineFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            Err(error)
        }
    }
}

// =============================================================================
// Internal: Execute a Primitive (Leaf) Node
// =============================================================================

/// Execute a primitive node: look up the processor, iterate files, call it.
fn execute_primitive_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    pipeline_total_files: usize,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // --- Step 1: Resolve the processor from the registry ---
    let processor = registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
            // Build a descriptive error message including the compound key.
            let operation = node
                .params
                .get("operation")
                .and_then(|v| v.as_str())
                .unwrap_or("default");
            BntoError::InvalidInput(format!(
                "No processor registered for '{}:{}' (node '{}')",
                node.node_type, operation, node.id
            ))
        })?;

    // --- Step 2: Process each file ---
    // `local_file_count` is how many files THIS node received (used for
    // pre-allocation and the final files_processed count).
    // `pipeline_total_files` is the ORIGINAL input count from the user
    // (used in progress events so the UI says "X of 4", not "1 of 1").
    let local_file_count = files.len();

    // PERFORMANCE: Pre-allocate with capacity. Most processors produce 1 output
    // per input, so `local_file_count` is a good estimate. Avoids repeated
    // reallocation and copying as the Vec grows.
    let mut output_files: Vec<PipelineFile> = Vec::with_capacity(local_file_count);

    // PERFORMANCE: Clone node.id once outside the loop. Each file iteration
    // needs the node_id for progress events, but cloning inside the loop
    // allocates a new String per file. One clone, reused via references.
    let node_id = node.id.clone();

    // PERFORMANCE: Clone params once outside the loop. Every file iteration
    // needs the same params map for the processor input. Cloning inside
    // the loop would deep-copy the entire JSON map per file.
    let params_for_input = node.params.clone();

    for (file_index, file) in files.into_iter().enumerate() {
        // Create a per-file ProgressReporter that converts to FileProgress events.
        // This bridges the existing per-file progress system with the new
        // pipeline event system.
        let file_progress_reporter = {
            let node_id_for_closure = node_id.clone();
            ProgressReporter::new(move |percent, message| {
                // Note: We can't call reporter.emit() here because it would
                // require capturing `reporter` in the closure, which conflicts
                // with the borrow checker. Instead, we use a simpler approach:
                // the node-level FileProgress events are emitted from the
                // outer scope after each file completes.
                //
                // For now, the per-file ProgressReporter is a no-op at the
                // pipeline level. The FileProgress event is emitted below
                // with the correct file_index and total_files.
                let _ = (percent, message, &node_id_for_closure);
            })
        };

        // PERFORMANCE: Capture filename before it's moved into NodeInput.
        // This avoids a clone — we take ownership here and use a reference
        // for progress messages.
        let file_name = file.name;

        // Emit FileProgress at 0% to signal "starting this file".
        // Use `file_offset + file_index` so the UI reports the global position
        // (e.g., "3 of 4") even when a loop container sends one file at a time.
        reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: pipeline_total_files,
            percent: 0,
            message: format!("Processing {}...", &file_name),
        });

        // Build the NodeInput from our PipelineFile.
        let input = NodeInput {
            data: file.data,
            filename: file_name.clone(),
            mime_type: Some(file.mime_type),
            params: params_for_input.clone(),
        };

        // Call the processor.
        let output = processor.process(input, &file_progress_reporter)?;

        // Emit FileProgress at 100% to signal "done with this file".
        reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: pipeline_total_files,
            percent: 100,
            message: format!("Completed {}", &file_name),
        });

        // Convert NodeOutput files to PipelineFiles for chaining.
        // Attach the processor's metadata to each output file so stats
        // (compression ratio, original size, etc.) survive through the chain.
        for output_file in output.files {
            output_files.push(PipelineFile {
                name: output_file.filename,
                data: output_file.data,
                mime_type: output_file.mime_type,
                metadata: output.metadata.clone(),
            });
        }
    }

    Ok(NodeExecutionResult {
        files_processed: local_file_count,
        output_files,
    })
}

// =============================================================================
// Internal: Execute a Container Node
// =============================================================================

/// Execute a container node (loop, group, parallel) by recursing into children.
///
/// Container semantics:
/// - `loop` — run children sub-pipeline once PER file (each iteration gets one file)
/// - `group` — run children sub-pipeline once on the FULL batch
/// - `parallel` — same as group for now (concurrent execution is future work)
fn execute_container_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    pipeline_total_files: usize,
    file_offset: usize,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    // Get children, defaulting to empty if none (passthrough).
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

    // If no children, passthrough.
    if children.is_empty() {
        return Ok(NodeExecutionResult {
            files_processed: 0,
            output_files: files,
        });
    }

    // Build a sub-pipeline definition from the children.
    let sub_definition = crate::pipeline::PipelineDefinition {
        nodes: children.clone(),
    };

    match node.node_type.as_str() {
        "loop" => {
            // --- Loop: run sub-pipeline once PER file ---
            // Each iteration gets a single-file batch. Results are collected.
            // We increment file_offset so each iteration's progress events
            // report the correct global position (e.g., file 2 of 4, not 1 of 1).
            let mut all_output_files: Vec<PipelineFile> = Vec::new();
            let mut total_processed: usize = 0;

            for (i, file) in files.into_iter().enumerate() {
                let single_file_batch = vec![file];
                let result = execute_sub_pipeline(
                    &sub_definition,
                    single_file_batch,
                    registry,
                    reporter,
                    pipeline_total_files,
                    file_offset + i, // each iteration is one file further
                    now_ms,
                )?;
                total_processed += result.files_processed;
                all_output_files.extend(result.output_files);
            }

            Ok(NodeExecutionResult {
                files_processed: total_processed,
                output_files: all_output_files,
            })
        }

        // "group" and "parallel" both run the sub-pipeline on the full batch.
        // "parallel" is the same as "group" for now — concurrent execution
        // is future work.
        "group" | "parallel" => {
            let result = execute_sub_pipeline(
                &sub_definition,
                files,
                registry,
                reporter,
                pipeline_total_files,
                file_offset,
                now_ms,
            )?;
            Ok(NodeExecutionResult {
                files_processed: result.files_processed,
                output_files: result.output_files,
            })
        }

        _ => {
            // Unknown container type — treat as passthrough with a warning.
            Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            })
        }
    }
}

// =============================================================================
// Internal: Execute a Sub-Pipeline (for container children)
// =============================================================================

/// Execute a sub-pipeline (the children of a container node).
///
/// This is essentially `execute_pipeline` but without the top-level
/// PipelineStarted/PipelineCompleted events (those belong to the
/// outer pipeline, not each container's children).
fn execute_sub_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    pipeline_total_files: usize,
    file_offset: usize,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    // Filter out I/O nodes from children too.
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
            node,
            current_files,
            registry,
            reporter,
            node_index,
            total_nodes,
            pipeline_total_files,
            file_offset,
            now_ms,
        )?;
        total_files_processed += result.files_processed;
        current_files = result.output_files;
    }

    Ok(NodeExecutionResult {
        files_processed: total_files_processed,
        output_files: current_files,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
