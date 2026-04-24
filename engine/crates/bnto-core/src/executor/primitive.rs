// Primitive Node Execution — runs leaf nodes (image compress, file rename, etc.)
// on a batch of files. Looks up the processor from the registry, iterates
// files, calls the processor per-file, and collects results.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::metadata::InputCardinality;
use crate::pipeline::PipelineFile;
use crate::processor::{BatchFile, BatchInput, NodeInput};
use crate::progress::ProgressReporter;

use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef};

/// Build a ProgressReporter that forwards output lines to PipelineEvent::CommandOutput.
fn build_output_reporter<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
) -> ProgressReporter {
    let reporter = ctx.reporter.clone();
    let nid = node_id.to_string();
    ProgressReporter::with_output(
        |_, _| {},
        move |line| {
            reporter.emit(PipelineEvent::CommandOutput {
                node_id: nid.clone(),
                line: line.to_string(),
            });
        },
    )
}

/// Resolve the processor for a node, returning a descriptive error on miss.
fn resolve_processor<'a, F: Fn() -> u64 + Copy>(
    ctx: &'a PipelineContext<F>,
    node: PipelineNodeRef,
) -> Result<&'a dyn crate::processor::NodeProcessor, BntoError> {
    ctx.registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
            BntoError::InvalidInput(format!(
                "No processor registered for '{}' (node '{}')",
                node.node_type, node.id
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

    // Reporter that forwards command output lines to PipelineEvent::CommandOutput.
    let output_reporter = build_output_reporter(ctx, node_id);
    let output = processor.process(input, &output_reporter, ctx.process_ctx)?;

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

/// Resolve `{{fields.*}}` templates in node params using the node's own field declarations.
fn resolve_node_params<F: Fn() -> u64 + Copy>(
    _ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
) -> serde_json::Map<String, serde_json::Value> {
    if node.fields.is_empty() {
        return node.params.clone();
    }
    let field_values =
        super::resolve::collect_field_values(&node.fields, &std::collections::BTreeMap::new());
    super::resolve::resolve_fields(&node.params, &field_values)
}

/// Execute a primitive (leaf) node on a batch of files.
///
/// Resolves the processor from the registry, then dispatches based on
/// input cardinality: PerFile processors iterate one file at a time,
/// Batch processors receive all files at once.
pub(super) fn execute_primitive_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let processor = resolve_processor(ctx, node)?;
    let cardinality = processor.metadata().input_cardinality;
    let local_file_count = files.len();
    let resolved_params = resolve_node_params(ctx, node);

    match cardinality {
        InputCardinality::Batch => {
            process_batch(ctx, processor, node, &resolved_params, files, file_offset)
        }
        InputCardinality::Source => {
            process_source(ctx, processor, node, &resolved_params, file_offset)
        }
        InputCardinality::PerFile => {
            let mut output_files: Vec<PipelineFile> = Vec::with_capacity(local_file_count);
            for (file_index, file) in files.into_iter().enumerate() {
                process_single_file(
                    ctx,
                    processor,
                    &node.id,
                    &resolved_params,
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
    }
}

/// Run a source processor once with empty input data.
/// Source processors generate output from params alone — no input files.
fn process_source<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    processor: &dyn crate::processor::NodeProcessor,
    node: PipelineNodeRef,
    resolved_params: &serde_json::Map<String, serde_json::Value>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    emit_file_progress(
        ctx,
        &node.id,
        file_offset,
        0,
        "Generating output...".to_string(),
    );

    let input = NodeInput {
        data: Vec::new(),
        filename: String::new(),
        mime_type: None,
        params: resolved_params.clone(),
    };

    let output_reporter = build_output_reporter(ctx, &node.id);
    let output = processor.process(input, &output_reporter, ctx.process_ctx)?;

    emit_file_progress(ctx, &node.id, file_offset, 100, "Complete".to_string());

    let mut output_files = Vec::new();
    collect_output(output, &mut output_files);
    Ok(NodeExecutionResult {
        files_processed: 0,
        output_files,
    })
}

/// Process all files at once through a batch processor.
fn process_batch<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    processor: &dyn crate::processor::NodeProcessor,
    node: PipelineNodeRef,
    resolved_params: &serde_json::Map<String, serde_json::Value>,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let file_count = files.len();

    emit_file_progress(
        ctx,
        &node.id,
        file_offset,
        0,
        format!("Processing batch of {} files...", file_count),
    );

    let batch_files: Vec<BatchFile> = files
        .into_iter()
        .map(|f| BatchFile {
            data: f.data,
            filename: f.name,
            mime_type: Some(f.mime_type),
        })
        .collect();

    let batch_input = BatchInput {
        files: batch_files,
        params: resolved_params.clone(),
    };

    let output_reporter = build_output_reporter(ctx, &node.id);
    let output = processor.process_batch(batch_input, &output_reporter, ctx.process_ctx)?;

    emit_file_progress(
        ctx,
        &node.id,
        file_offset,
        100,
        format!("Completed batch of {} files", file_count),
    );

    let metadata = output.metadata;
    let mut output_files = Vec::with_capacity(output.files.len());
    for f in output.files {
        output_files.push(PipelineFile {
            name: f.filename,
            data: f.data,
            mime_type: f.mime_type,
            metadata: metadata.clone(),
        });
    }

    Ok(NodeExecutionResult {
        files_processed: file_count,
        output_files,
    })
}
