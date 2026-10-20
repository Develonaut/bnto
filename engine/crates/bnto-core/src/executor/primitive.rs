// Primitive Node Execution — runs leaf nodes (image compress, file rename, etc.)
// on a batch of files. Looks up the processor from the registry, iterates
// files, calls the processor per-file, and collects results.

use crate::errors::BntoError;
use crate::events::PipelineEvent;
use crate::metadata::InputCardinality;
use crate::pipeline::PipelineFile;
use crate::processor::{BatchFile, BatchInput, NodeInput};
use crate::progress::ProgressReporter;
use crate::renamed::with_node_type_hint;

use super::leaf_output::LeafOutput;
use super::node_reporter::build_node_reporter;
use super::{NodeExecutionResult, PipelineContext, PipelineNodeRef};

/// Resolve the processor for a node, returning a descriptive error on miss.
fn resolve_processor<'a, F: Fn() -> u64 + Copy>(
    ctx: &'a PipelineContext<F>,
    node: PipelineNodeRef,
) -> Result<&'a dyn crate::processor::NodeProcessor, BntoError> {
    ctx.registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
            let message = format!(
                "No processor registered for '{}' (node '{}')",
                node.node_type, node.id
            );
            BntoError::InvalidInput(with_node_type_hint(message, &node.node_type))
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

/// A per-file node's run: its processor, id and the reporter every file shares.
struct NodeRun<'a> {
    processor: &'a dyn crate::processor::NodeProcessor,
    id: &'a str,
    progress: ProgressReporter,
}

/// Process a single file through a processor, emitting progress events.
fn process_single_file<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    run: &NodeRun,
    params: &serde_json::Map<String, serde_json::Value>,
    file: PipelineFile,
    global_file_index: usize,
    out: &mut LeafOutput,
) -> Result<(), BntoError> {
    let (processor, node_id) = (run.processor, run.id);
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

    let output = match processor.process(input, &run.progress, ctx.process_ctx) {
        Ok(output) => output,
        Err(error) => return Err(out.fail(error, &inherited)),
    };

    emit_file_progress(
        ctx,
        node_id,
        global_file_index,
        100,
        format!("Completed {}", file_name),
    );
    out.collect(output, &inherited);
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
    Ok(resolve_templates(ctx, node, field_values, item))
}

/// Resolve all template namespaces, leaving an unfilled `{{item.*}}` as written.
fn resolve_templates<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    field_values: &FieldValues,
    item: &Option<Params>,
) -> Params {
    let tpl_ctx = super::template::TemplateContext {
        field_values,
        process_ctx: ctx.process_ctx,
        node_outputs: &ctx.node_outputs,
        loop_item: item,
        run_started_ms: Some(ctx.started_ms),
    };
    super::template::resolve_templates(&node.params, &tpl_ctx)
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

/// Resolve a node's params once, against the enclosing loop's item. Outside
/// a loop, a node that fills `{{item.*}}` itself, per file, gets those as
/// written instead of refused.
fn resolve_node_params<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: PipelineNodeRef,
    processor: &dyn crate::processor::NodeProcessor,
) -> Result<Params, BntoError> {
    let field_values = node_field_values(node);
    let resolved = if processor.resolves_items() && ctx.loop_item.is_none() {
        resolve_templates(ctx, node, &field_values, &None)
    } else {
        resolve_with(ctx, node, &field_values, &ctx.loop_item, None)?
    };
    let reserved = processor.reserved_tokens();
    super::reserved_tokens::check_reserved(reserved, &node.params, &resolved, &node.id)?;
    Ok(resolved)
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
            let params = resolve_node_params(ctx, node, processor)?;
            process_batch(ctx, processor, node, &params, files, file_offset)
        }
        InputCardinality::Source => {
            let params = resolve_node_params(ctx, node, processor)?;
            process_source(ctx, processor, node, &params, file_offset)
        }
        InputCardinality::PerFile => process_per_file(ctx, processor, node, files, file_offset),
    }
}

/// Run a per-file processor on each file. Params that reference `{{item.*}}`
/// resolve per file against that file's item; others resolve once.
///
/// The node's `{{node.<id>.*}}` record merges every call's node-level
/// metadata in file order, so a later file wins a key that several set and
/// the processor's count keys add up.
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
    let mut out = LeafOutput::counting(processor.count_keys());
    // One reporter for the whole run, so its throttle spans every file.
    let run = NodeRun {
        processor,
        id: &node.id,
        progress: build_node_reporter(ctx, &node.id),
    };
    for (i, file) in files.into_iter().enumerate() {
        ctx.check_time_limit()
            .map_err(|e| out.fail(e, &Params::new()))?;
        let params = params_for_file(ctx, node, &field_values, &shared, &file)
            .map_err(|e| out.fail(e, &Params::new()))?;
        process_single_file(ctx, &run, &params, file, file_offset + i, &mut out)?;
    }
    Ok(out.into_result(&node.id, file_count))
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

    let progress = build_node_reporter(ctx, &node.id);
    let output = processor.process(input, &progress, ctx.process_ctx)?;

    emit_file_progress(ctx, &node.id, file_offset, 100, "Complete".to_string());

    let mut out = LeafOutput::counting(processor.count_keys());
    out.collect(output, &Params::new());
    Ok(out.into_result(&node.id, 0))
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

    // Data passes through as-is: a disk reference stays a reference, and
    // processors that need bytes read them (`FileData::to_bytes`).
    let batch_files: Vec<BatchFile> = files
        .into_iter()
        .map(|f| BatchFile {
            data: f.data,
            filename: f.name,
            mime_type: Some(f.mime_type),
            metadata: f.metadata,
        })
        .collect();

    let batch_input = BatchInput {
        files: batch_files,
        params: resolved_params.clone(),
    };

    let progress = build_node_reporter(ctx, &node.id);
    let output = processor.process_batch(batch_input, &progress, ctx.process_ctx)?;

    emit_file_progress(
        ctx,
        &node.id,
        file_offset,
        100,
        format!("Completed batch of {} files", file_count),
    );

    let mut out = LeafOutput::counting(processor.count_keys());
    out.collect(output, &Params::new());
    Ok(out.into_result(&node.id, file_count))
}
