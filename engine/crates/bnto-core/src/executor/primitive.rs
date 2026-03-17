// Primitive node execution — runs a processor on each file in the batch.
// Looks up the processor from the registry, iterates files, collects results.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::PipelineFile;
use crate::processor::NodeInput;
use crate::progress::ProgressReporter;

use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef};

// =============================================================================
// Public (to parent module) API
// =============================================================================

/// Execute a primitive (leaf) node: resolve processor, iterate files, call it.
///
/// `file_offset` ensures progress events report the correct global position
/// (e.g., "file 3 of 4") even inside a loop container.
pub(super) fn execute_primitive_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    // --- Resolve the processor from the registry ---
    let processor = ctx
        .registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
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

    let local_file_count = files.len();
    let mut output_files: Vec<PipelineFile> = Vec::with_capacity(local_file_count);

    // Clone once outside the loop to avoid per-file allocations.
    let node_id = node.id.clone();
    let params_for_input = node.params.clone();

    // --- Process each file ---
    for (file_index, file) in files.into_iter().enumerate() {
        // Per-file ProgressReporter bridging pipeline events.
        // Currently a no-op at pipeline level — FileProgress is emitted below.
        let file_progress_reporter = {
            let node_id_for_closure = node_id.clone();
            ProgressReporter::new(move |percent, message| {
                let _ = (percent, message, &node_id_for_closure);
            })
        };

        // Capture filename before moving into NodeInput.
        let file_name = file.name;

        // Emit 0% to signal "starting this file".
        ctx.reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: ctx.pipeline_total_files,
            percent: 0,
            message: format!("Processing {}...", &file_name),
        });

        let input = NodeInput {
            data: file.data,
            filename: file_name.clone(),
            mime_type: Some(file.mime_type),
            params: params_for_input.clone(),
        };

        let output = processor.process(input, &file_progress_reporter)?;

        // Emit 100% to signal "done with this file".
        ctx.reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index: file_offset + file_index,
            total_files: ctx.pipeline_total_files,
            percent: 100,
            message: format!("Completed {}", &file_name),
        });

        // Convert to PipelineFiles, preserving processor metadata for chaining.
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
