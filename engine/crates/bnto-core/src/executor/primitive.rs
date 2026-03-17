// Primitive Node Execution — runs leaf nodes (image compress, file rename, etc.)
// on a batch of files. Looks up the processor from the registry, iterates
// files, calls the processor per-file, and collects results.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::pipeline::PipelineFile;
use crate::processor::NodeInput;
use crate::progress::ProgressReporter;

use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef};

/// Resolve the processor for a node, returning a descriptive error on miss.
fn resolve_processor<'a, F: Fn() -> u64 + Copy>(
    ctx: &'a PipelineContext<F>,
    node: PipelineNodeRef,
) -> Result<&'a dyn crate::processor::NodeProcessor, BntoError> {
    ctx.registry
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
        })
}

/// Emit a FileProgress event for a node.
fn emit_file_progress<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    global_file_index: usize,
    percent: u32,
    message: String,
) {
    ctx.reporter.emit(PipelineEvent::FileProgress {
        node_id: node_id.to_string(),
        file_index: global_file_index,
        total_files: ctx.pipeline_total_files,
        percent,
        message,
    });
}

/// Collect processor output files into the pipeline result vector.
fn collect_output(output: crate::processor::NodeOutput, output_files: &mut Vec<PipelineFile>) {
    for f in output.files {
        output_files.push(PipelineFile {
            name: f.filename,
            data: f.data,
            mime_type: f.mime_type,
            metadata: output.metadata.clone(),
        });
    }
}

/// Process a single file through a processor, emitting progress events.
fn process_single_file<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    processor: &dyn crate::processor::NodeProcessor,
    node_id: &str,
    params: &serde_json::Map<String, serde_json::Value>,
    file: PipelineFile,
    global_file_index: usize,
    output_files: &mut Vec<PipelineFile>,
) -> Result<(), BntoError> {
    let file_name = file.name;
    emit_file_progress(
        ctx,
        node_id,
        global_file_index,
        0,
        format!("Processing {}...", &file_name),
    );

    let input = NodeInput {
        data: file.data,
        filename: file_name.clone(),
        mime_type: Some(file.mime_type),
        params: params.clone(),
    };

    // No-op per-file reporter — pipeline-level FileProgress events are
    // emitted here directly with correct global indices.
    let noop_reporter = ProgressReporter::new(|_, _| {});
    let output = processor.process(input, &noop_reporter)?;

    emit_file_progress(
        ctx,
        node_id,
        global_file_index,
        100,
        format!("Completed {}", &file_name),
    );
    collect_output(output, output_files);
    Ok(())
}

/// Execute a primitive (leaf) node on a batch of files.
///
/// Resolves the processor from the registry, iterates each file through it,
/// and collects outputs. Emits FileProgress events per file.
pub(super) fn execute_primitive_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let processor = resolve_processor(ctx, node)?;
    let local_file_count = files.len();
    let mut output_files: Vec<PipelineFile> = Vec::with_capacity(local_file_count);

    for (file_index, file) in files.into_iter().enumerate() {
        process_single_file(
            ctx,
            processor,
            &node.id,
            &node.params,
            file,
            file_offset + file_index,
            &mut output_files,
        )?;
    }

    Ok(NodeExecutionResult {
        files_processed: local_file_count,
        output_files,
    })
}
