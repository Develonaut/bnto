// Auto-Iteration — implicit per-file looping for flat recipes.
//
// When `settings.iteration = "auto"`, the executor wraps contiguous per-file
// processor sequences in implicit per-file loops. This produces identical
// output to explicit loop containers but without requiring recipe authors
// to manually wrap their processors in loop/group nodes.

use crate::errors::BntoError;
use crate::metadata::InputCardinality;
use crate::pipeline::{PipelineFile, PipelineNode, is_container_node};
use crate::registry::NodeRegistry;

use super::{ChainPosition, NodeExecutionResult, PipelineContext, run_node_chain};

/// Execute a flat node list with auto-iteration: contiguous per-file
/// processors are wrapped in implicit per-file loops. Containers and
/// batch processors act as iteration barriers — they break the sequence.
pub(super) fn run_auto_iteration<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
) -> Result<NodeExecutionResult, BntoError> {
    let runs = partition_into_runs(nodes, ctx.registry);
    let mut current_files = files;
    let mut total_processed: usize = 0;
    let mut all_warnings: Vec<String> = Vec::new();

    for run in &runs {
        let result = run_one(ctx, nodes, run, current_files)?;
        total_processed += result.files_processed;
        all_warnings.extend(result.warnings);
        current_files = result.output_files;
    }

    Ok(NodeExecutionResult {
        output_files: current_files,
        files_processed: total_processed,
        warnings: all_warnings,
    })
}

/// Execute one run with the strategy its kind needs.
fn run_one<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    run: &Run,
    files: Vec<PipelineFile>,
) -> Result<NodeExecutionResult, BntoError> {
    match run {
        Run::PerFileSequence { start, len } => {
            let seq = &nodes[*start..*start + *len];
            let position = ChainPosition::starting_at(*start, nodes.len());
            run_per_file_sequence(ctx, seq, files, position)
        }
        Run::Container { index } | Run::Batch { index } => {
            run_single_node(ctx, nodes, *index, files)
        }
        Run::Source { index } => run_single_node(ctx, nodes, *index, vec![]),
    }
}

/// Run the node at `index` as a one-node chain, numbered within the whole pipeline.
fn run_single_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    index: usize,
    files: Vec<PipelineFile>,
) -> Result<NodeExecutionResult, BntoError> {
    let node = &nodes[index..index + 1];
    let position = ChainPosition::starting_at(index, nodes.len());
    run_node_chain(ctx, node, files, 0, position)
}

/// A contiguous run of nodes that share an execution strategy.
enum Run {
    /// Contiguous per-file primitive nodes — wrapped in an implicit
    /// per-file loop. Each file passes through the full sequence.
    PerFileSequence { start: usize, len: usize },
    /// A container node — dispatched as-is (containers define own iteration).
    Container { index: usize },
    /// A batch processor — receives all files at once. Breaks per-file sequences.
    Batch { index: usize },
    /// A source processor — runs once with no input files. Breaks per-file sequences.
    Source { index: usize },
}

/// Check if a primitive node has batch cardinality by consulting the registry.
fn is_batch_node(node: &PipelineNode, registry: &NodeRegistry) -> bool {
    let params = &node.params;
    registry
        .resolve(&node.node_type, params)
        .map(|p| p.metadata().input_cardinality == InputCardinality::Batch)
        .unwrap_or(false)
}

/// Check if a primitive node has source cardinality by consulting the registry.
fn is_source_node(node: &PipelineNode, registry: &NodeRegistry) -> bool {
    let params = &node.params;
    registry
        .resolve(&node.node_type, params)
        .map(|p| p.metadata().input_cardinality == InputCardinality::Source)
        .unwrap_or(false)
}

/// Partition the flat node list into runs based on node type and cardinality.
/// Containers and batch nodes act as barriers that break per-file sequences.
fn partition_into_runs(nodes: &[&PipelineNode], registry: &NodeRegistry) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut seq_start: Option<usize> = None;

    for (i, node) in nodes.iter().enumerate() {
        if is_container_node(&node.node_type) {
            // Flush any pending per-file sequence
            if let Some(start) = seq_start.take() {
                runs.push(Run::PerFileSequence {
                    start,
                    len: i - start,
                });
            }
            runs.push(Run::Container { index: i });
        } else if is_batch_node(node, registry) {
            // Batch nodes also break per-file sequences
            if let Some(start) = seq_start.take() {
                runs.push(Run::PerFileSequence {
                    start,
                    len: i - start,
                });
            }
            runs.push(Run::Batch { index: i });
        } else if is_source_node(node, registry) {
            // Source nodes break per-file sequences — they run once with no input
            if let Some(start) = seq_start.take() {
                runs.push(Run::PerFileSequence {
                    start,
                    len: i - start,
                });
            }
            runs.push(Run::Source { index: i });
        } else if seq_start.is_none() {
            seq_start = Some(i);
        }
    }

    // Flush trailing per-file sequence
    if let Some(start) = seq_start {
        runs.push(Run::PerFileSequence {
            start,
            len: nodes.len() - start,
        });
    }

    runs
}

/// Run a sequence of primitive nodes once per file, collecting all outputs.
/// Same semantics as `execute_loop` in container.rs. `position` places the
/// sequence within the pipeline.
fn run_per_file_sequence<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    position: ChainPosition,
) -> Result<NodeExecutionResult, BntoError> {
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;
    let mut all_warnings: Vec<String> = Vec::new();

    for (i, file) in files.into_iter().enumerate() {
        let result = run_node_chain(ctx, nodes, vec![file], i, position)?;
        total_processed += result.files_processed;
        all_warnings.extend(result.warnings);
        all_output_files.extend(result.output_files);
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
        warnings: all_warnings,
    })
}
