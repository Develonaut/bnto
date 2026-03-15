// =============================================================================
// Primitive Node Execution
// =============================================================================
//
// WHAT IS THIS FILE?
// This file handles the execution of "primitive" (leaf) nodes — the nodes that
// actually DO something to files. Examples: compress an image, rename a file,
// clean CSV columns. These are the opposite of "container" nodes (loop, group,
// parallel) which just organize other nodes.
//
// A primitive node has a processor (the actual logic) registered in the
// NodeRegistry. This module looks up the right processor, iterates over the
// input files, calls the processor for each one, and collects the results.
//
// WHY IS THIS A SEPARATE FILE?
// The executor was originally one big file. Splitting it into focused modules
// follows the Bento Box Principle: one responsibility per file. This file owns
// "how to run a single processing node on a batch of files." Container logic
// lives in `container.rs`. The top-level orchestration lives in `mod.rs`.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::PipelineFile;
use crate::processor::NodeInput;
use crate::progress::ProgressReporter;

// Import shared types from the parent module.
// `pub(super)` means these are visible within the `executor` module but not
// outside it — they're internal implementation details.
use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef};

// =============================================================================
// Public (to parent module) API
// =============================================================================

/// Execute a primitive node: look up the processor, iterate files, call it.
///
/// "Primitive" means this node is a leaf — it has actual processing logic
/// (like image compression or file renaming), not children to recurse into.
///
/// # How it works
/// 1. Find the right processor in the registry (e.g., "image:compress")
/// 2. Loop over each input file
/// 3. Call the processor for each file, collecting output files
/// 4. Emit progress events so the UI can show "Processing file 2 of 4..."
///
/// # Arguments
/// - `ctx` — shared pipeline state (registry, reporter, timing)
/// - `node` — the node definition (type, id, params)
/// - `files` — the batch of files to process
/// - `file_offset` — where this batch starts in the global file list
///   (important for progress: "file 3 of 4" even inside a loop container)
pub(super) fn execute_primitive_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // --- Step 1: Resolve the processor from the registry ---
    // The registry maps (node_type, operation) pairs to processor implementations.
    // For example, ("image", "compress") -> ImageCompressProcessor.
    let processor = ctx
        .registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
            // Build a descriptive error message including the compound key.
            // `and_then` chains Option operations: first get the "operation" key
            // from the params map, then try to convert the JSON value to a string.
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
    // The `?` at the end means: if `.ok_or_else()` produced an `Err`, return
    // it immediately from this function. Otherwise, unwrap the `Ok` value
    // into `processor`.

    // --- Step 2: Prepare for the file loop ---
    // `local_file_count` is how many files THIS node received (used for
    // pre-allocation and the final files_processed count).
    // `pipeline_total_files` (on ctx) is the ORIGINAL input count from the user
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

    // --- Step 3: Process each file ---
    // `into_iter()` takes ownership of the `files` Vec. Each `file` is moved
    // out of the Vec — no copying. `enumerate()` gives us the index too.
    for (file_index, file) in files.into_iter().enumerate() {
        // Create a per-file ProgressReporter that bridges the existing
        // per-file progress system with pipeline-level events.
        let file_progress_reporter = {
            // Clone the node_id for the closure. The closure needs its own
            // copy because it might outlive this loop iteration.
            let node_id_for_closure = node_id.clone();
            ProgressReporter::new(move |percent, message| {
                // Note: We can't call reporter.emit() here because it would
                // require capturing `reporter` in the closure, which conflicts
                // with the borrow checker (reporter is already borrowed by ctx).
                //
                // For now, the per-file ProgressReporter is a no-op at the
                // pipeline level. The FileProgress events are emitted from the
                // outer scope (below) with the correct file_index and total.
                let _ = (percent, message, &node_id_for_closure);
            })
        };

        // PERFORMANCE: Capture filename before it's moved into NodeInput.
        // `file.name` is a String — moving it into `file_name` transfers
        // ownership without copying the underlying heap data.
        let file_name = file.name;

        // Emit FileProgress at 0% to signal "starting this file".
        // Use `file_offset + file_index` so the UI reports the global position
        // (e.g., "3 of 4") even when a loop container sends one file at a time.
        ctx.reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: ctx.pipeline_total_files,
            percent: 0,
            message: format!("Processing {}...", &file_name),
        });

        // Build the NodeInput struct from our PipelineFile.
        // This is the standard input shape that all processors expect.
        let input = NodeInput {
            data: file.data,
            filename: file_name.clone(),
            mime_type: Some(file.mime_type),
            params: params_for_input.clone(),
        };

        // Call the processor — this is where the actual work happens
        // (e.g., image compression, CSV cleaning, file renaming).
        // The `?` propagates any error up to the caller.
        let output = processor.process(input, &file_progress_reporter)?;

        // Emit FileProgress at 100% to signal "done with this file".
        ctx.reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: ctx.pipeline_total_files,
            percent: 100,
            message: format!("Completed {}", &file_name),
        });

        // Convert NodeOutput files to PipelineFiles for chaining to the next node.
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

    // Return the collected outputs and file count.
    Ok(NodeExecutionResult {
        files_processed: local_file_count,
        output_files,
    })
}
