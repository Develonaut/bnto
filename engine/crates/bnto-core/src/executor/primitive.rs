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
/// Each file's metadata is `inherited` (what the input file carried) with the
/// node's own metadata on top: its per-file metadata when non-empty, else the
/// shared NodeOutput.metadata. The node wins a shared key.
fn collect_output(
    output: crate::processor::NodeOutput,
    inherited: &serde_json::Map<String, serde_json::Value>,
    output_files: &mut Vec<PipelineFile>,
) {
    for f in output.files {
        let own = if f.metadata.is_empty() {
            output.metadata.clone()
        } else {
            f.metadata
        };
        let mut metadata = inherited.clone();
        metadata.extend(own);
        output_files.push(PipelineFile {
            name: f.filename,
            data: f.data,
            mime_type: f.mime_type,
            metadata,
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
    let inherited = file.metadata;
    emit_file_progress(
        ctx,
        node_id,
        global_file_index,
        0,
        format!("Processing {}...", file_name),
    );

    // Pass FileData through directly — processors that need bytes call into_bytes() themselves.
    // This enables zero-copy for file-rename and similar processors.
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
        format!("Completed {}", file_name),
    );
    collect_output(output, &inherited, output_files);
    Ok(())
}

type Params = serde_json::Map<String, serde_json::Value>;
type FieldValues = std::collections::BTreeMap<String, serde_json::Value>;

/// A node's `{{fields.*}}` values. They don't vary per file, so compute once.
fn node_field_values(node: PipelineNodeRef) -> FieldValues {
    if node.fields.is_empty() {
        return FieldValues::new();
    }
    super::resolve::collect_field_values(&node.fields, &FieldValues::new())
}

/// Resolve all template namespaces in node params (fields, env, ctx, node, item).
/// `item` is the `{{item.*}}` source; `file` names the file it came from.
/// Fails when a `{{item.*}}` reference has no value to fill it.
fn resolve_with<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    field_values: &FieldValues,
    item: &Option<Params>,
    file: Option<&str>,
) -> Result<Params, BntoError> {
    require_filled_items(node, item, file)?;
    let tpl_ctx = super::template::TemplateContext {
        field_values,
        process_ctx: ctx.process_ctx,
        node_outputs: &ctx.node_outputs,
        loop_item: item,
    };
    Ok(super::template::resolve_templates(&node.params, &tpl_ctx))
}

/// Error when a `{{item.*}}` param reference has nothing to fill it, naming
/// the node, the key, the file and the keys that are there.
fn require_filled_items(
    node: PipelineNodeRef,
    item: &Option<Params>,
    file: Option<&str>,
) -> Result<(), BntoError> {
    let unfilled = super::unfilled_items::unfilled_item_keys(&node.params, item);
    let Some(key) = unfilled.first() else {
        return Ok(());
    };
    let target = file.map(|f| format!(" for {f}")).unwrap_or_default();
    let available = match item {
        Some(item) if !item.is_empty() => {
            format!(
                "it has: {}",
                item.keys().cloned().collect::<Vec<_>>().join(", ")
            )
        }
        _ => "it has no item values".to_string(),
    };
    Err(BntoError::InvalidInput(format!(
        "Node '{}': {{{{item.{key}}}}} has no value{target} ({available})",
        node.id
    )))
}

/// Resolve a node's params once, against the enclosing loop's item.
fn resolve_node_params<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
) -> Result<Params, BntoError> {
    resolve_with(ctx, node, &node_field_values(node), &ctx.loop_item, None)
}

/// The `{{item.*}}` source for one file: the file's own metadata (what every
/// earlier node attached), under the innermost loop's item. The loop item wins
/// a shared key, so a loop over CSV rows keeps reading its row's columns.
fn file_item(loop_item: &Option<Params>, file: &PipelineFile) -> Option<Params> {
    if file.metadata.is_empty() {
        return loop_item.clone();
    }
    let mut item = file.metadata.clone();
    if let Some(loop_item) = loop_item {
        item.extend(loop_item.clone());
    }
    Some(item)
}

/// True when any param references `{{item.*}}`, so it must resolve per file.
fn mentions_item(params: &Params) -> bool {
    params.values().any(|v| v.to_string().contains("{{item."))
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
    match processor.metadata().input_cardinality {
        InputCardinality::Batch => {
            let params = resolve_node_params(ctx, node)?;
            process_batch(ctx, processor, node, &params, files, file_offset)
        }
        InputCardinality::Source => {
            let params = resolve_node_params(ctx, node)?;
            process_source(ctx, processor, node, &params, file_offset)
        }
        InputCardinality::PerFile => process_per_file(ctx, processor, node, files, file_offset),
    }
}

/// Run a per-file processor on each file. Params that reference `{{item.*}}`
/// resolve per file against that file's item; others resolve once.
fn process_per_file<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    processor: &dyn crate::processor::NodeProcessor,
    node: PipelineNodeRef,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let file_count = files.len();
    let field_values = node_field_values(node);
    let shared = shared_params(ctx, node, &field_values)?;
    let mut output_files = Vec::with_capacity(file_count);
    for (i, file) in files.into_iter().enumerate() {
        let params = params_for_file(ctx, node, &field_values, &shared, &file)?;
        let (id, index) = (&node.id, file_offset + i);
        process_single_file(ctx, processor, id, &params, file, index, &mut output_files)?;
    }
    Ok(NodeExecutionResult {
        files_processed: file_count,
        output_files,
        warnings: Vec::new(),
    })
}

/// One resolution for every file, or `None` when the node reads `{{item.*}}`
/// and so must resolve per file.
fn shared_params<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    field_values: &FieldValues,
) -> Result<Option<Params>, BntoError> {
    if mentions_item(&node.params) {
        return Ok(None);
    }
    resolve_with(ctx, node, field_values, &ctx.loop_item, None).map(Some)
}

/// The params one file runs with: the shared resolution when the node never
/// reads `{{item.*}}`, otherwise a resolution against that file's item.
fn params_for_file<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    field_values: &FieldValues,
    shared: &Option<Params>,
    file: &PipelineFile,
) -> Result<Params, BntoError> {
    if let Some(params) = shared {
        return Ok(params.clone());
    }
    let item = file_item(&ctx.loop_item, file);
    resolve_with(ctx, node, field_values, &item, Some(&file.name))
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
        data: crate::processor::FileData::Bytes(Vec::new()),
        filename: String::new(),
        mime_type: None,
        params: resolved_params.clone(),
    };

    let output_reporter = build_output_reporter(ctx, &node.id);
    let output = processor.process(input, &output_reporter, ctx.process_ctx)?;

    emit_file_progress(ctx, &node.id, file_offset, 100, "Complete".to_string());

    let mut output_files = Vec::new();
    collect_output(output, &serde_json::Map::new(), &mut output_files);
    Ok(NodeExecutionResult {
        files_processed: 0,
        output_files,
        warnings: Vec::new(),
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
        .map(|f| {
            let data = f.data.into_bytes().map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to read input file {}: {e}", f.name))
            })?;
            Ok(BatchFile {
                data,
                filename: f.name,
                mime_type: Some(f.mime_type),
            })
        })
        .collect::<Result<Vec<_>, BntoError>>()?;

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

    let mut output_files = Vec::with_capacity(output.files.len());
    collect_output(output, &serde_json::Map::new(), &mut output_files);
    Ok(NodeExecutionResult {
        files_processed: file_count,
        output_files,
        warnings: Vec::new(),
    })
}
