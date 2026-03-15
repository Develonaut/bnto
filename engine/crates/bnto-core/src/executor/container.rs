// =============================================================================
// Container Node Execution
// =============================================================================
//
// WHAT IS THIS FILE?
// This file handles "container" nodes — nodes that don't process files
// themselves but instead organize OTHER nodes. Think of them as folders
// that hold child nodes:
//
//   - `loop`     — run children once PER file (like a for-each loop)
//   - `group`    — run children once on ALL files together (like a batch)
//   - `parallel` — same as group for now (future: concurrent execution)
//
// Container nodes enable complex recipes. For example, a recipe that
// compresses images and THEN renames them uses a loop container: for each
// file, run compress -> rename in sequence.
//
// WHY IS THIS A SEPARATE FILE?
// Container execution is conceptually different from primitive execution.
// Primitives call a processor. Containers recurse into child sub-pipelines.
// Keeping them separate follows the Bento Box Principle: one responsibility
// per file. This file owns "how to recurse into container children."

use crate::errors::BntoError;
use crate::pipeline::{PipelineDefinition, PipelineFile, PipelineNode, is_io_node};

// Import shared types and the dispatch function from the parent module.
use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef, execute_node};

// =============================================================================
// Public (to parent module) API
// =============================================================================

/// Execute a container node (loop, group, parallel) by recursing into children.
///
/// Container nodes don't process files themselves — they hold child nodes
/// and decide HOW those children see the files:
///
/// - **loop**: Each child sub-pipeline gets ONE file at a time.
///   If there are 4 files, the children run 4 times (once per file).
///   This is how "compress each image individually" works.
///
/// - **group**: The children get ALL files at once as a batch.
///   The sub-pipeline runs exactly once with the full file set.
///
/// - **parallel**: Same as group for now. Future work will add
///   concurrent execution of children.
///
/// # Arguments
/// - `ctx` — shared pipeline state (registry, reporter, timing)
/// - `node` — the container node definition (includes `children`)
/// - `files` — the batch of files to distribute to children
/// - `file_offset` — where this batch starts in the global file list
pub(super) fn execute_container_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // --- Step 1: Get the children nodes ---
    // `match` is Rust's pattern matching — like a switch statement but more
    // powerful. Here we check if `node.children` is `Some(vec)` or `None`.
    let children = match &node.children {
        Some(c) => c,
        None => {
            // Container with no children = passthrough.
            // Just return the input files unchanged, as if this node wasn't here.
            return Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            });
        }
    };

    // If children exist but the list is empty, also passthrough.
    if children.is_empty() {
        return Ok(NodeExecutionResult {
            files_processed: 0,
            output_files: files,
        });
    }

    // --- Step 2: Build a sub-pipeline from the children ---
    // A sub-pipeline is just a PipelineDefinition with the children as its nodes.
    // We clone the children because `execute_sub_pipeline` needs owned data
    // while we still borrow `node` through `children`.
    let sub_definition = PipelineDefinition {
        nodes: children.clone(),
    };

    // --- Step 3: Dispatch based on container type ---
    // `as_str()` converts the String to a `&str` for pattern matching.
    match node.node_type.as_str() {
        "loop" => execute_loop(ctx, &sub_definition, files, file_offset),
        // "group" and "parallel" both run on the full batch.
        // "parallel" is identical to "group" for now — concurrent execution
        // is future work that will add a thread pool or async runtime.
        "group" | "parallel" => execute_group(ctx, &sub_definition, files, file_offset),
        _ => {
            // Unknown container type — treat as passthrough with no error.
            // This is defensive: if a new container type is added to the
            // definition format before the executor supports it, we don't crash.
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

/// Run the sub-pipeline once PER file.
///
/// Each iteration gets a single-file batch. Results are collected into one
/// output Vec. The `file_offset` increments so each iteration's progress
/// events report the correct global position (e.g., "file 2 of 4", not
/// "file 1 of 1").
fn execute_loop<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    sub_definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // Pre-allocate output storage. We expect roughly one output per input.
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;

    // `into_iter()` takes ownership of `files`. Each `file` is moved out
    // of the Vec (no copy). `enumerate()` gives us the index.
    for (i, file) in files.into_iter().enumerate() {
        // Wrap the single file in a Vec (sub-pipeline expects a batch).
        let single_file_batch = vec![file];

        // Run the entire sub-pipeline on just this one file.
        let result = execute_sub_pipeline(
            ctx,
            sub_definition,
            single_file_batch,
            file_offset + i, // each iteration is one file further in the global list
        )?;

        // Accumulate results from each iteration.
        total_processed += result.files_processed;
        // `extend` appends all elements from the iterator to the Vec.
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
///
/// All files are passed to the children together. The sub-pipeline runs
/// exactly once. This is the simplest container semantics.
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

/// Execute a sub-pipeline (the children of a container node).
///
/// This is essentially the same logic as `execute_pipeline` in `mod.rs`,
/// but WITHOUT the top-level PipelineStarted/PipelineCompleted events.
/// Those events belong to the outer pipeline — each container's children
/// don't emit their own pipeline-level events.
///
/// The sub-pipeline:
/// 1. Filters out I/O marker nodes (input/output)
/// 2. Walks remaining nodes sequentially
/// 3. Chains outputs -> inputs between nodes
fn execute_sub_pipeline<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // Filter out I/O nodes from children too — they're structural markers,
    // not processing nodes.
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    // Walk each child node sequentially, chaining outputs -> inputs.
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
        // Chain: this node's output becomes the next node's input.
        current_files = result.output_files;
    }

    Ok(NodeExecutionResult {
        files_processed: total_files_processed,
        output_files: current_files,
    })
}
