// Auto-Iteration — implicit per-file looping for flat recipes.
//
// When `settings.iteration = "auto"`, the executor wraps contiguous per-file
// processor sequences in implicit per-file loops. This produces identical
// output to explicit loop containers but without requiring recipe authors
// to manually wrap their processors in loop/group nodes.

use crate::metadata::InputCardinality;
use crate::pipeline::{PipelineFile, PipelineNode, is_container_node};
use crate::registry::NodeRegistry;

use super::closing::ChainFailure;
use super::node_outputs::{NodeRecords, merge_records};
use super::{ChainPosition, NodeExecutionResult, PipelineContext, run_node_chain};

/// Execute a flat node list with auto-iteration: contiguous per-file
/// processors are wrapped in implicit per-file loops. Containers and
/// batch processors act as iteration barriers — they break the sequence.
/// Like a chain, each run's output metadata is visible to every later run.
///
/// A failure inside a run closes that run's own chain first (a per-file
/// sequence closes the failing file's pass); then the `always` nodes of
/// every later run run as one explicit chain on what it handed over.
/// `position` places `nodes` among the level's nodes.
pub(super) fn run_auto_iteration<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    position: ChainPosition,
) -> Result<NodeExecutionResult, ChainFailure> {
    let runs = partition_into_runs(nodes, ctx.registry);
    let mut auto_ctx = ctx.layer();
    let mut chain = NodeExecutionResult::holding(files);
    for run in &runs {
        let files = std::mem::take(&mut chain.output_files);
        match run_one(&auto_ctx, nodes, run, files, position) {
            Ok(result) => {
                let records = chain.then(result);
                auto_ctx.node_outputs.add(records, ctx.count_keys);
            }
            Err(failure) => {
                let end = run.end();
                let rest = position.offset(end);
                return Err(failure.close(&mut auto_ctx, &nodes[end..], 0, rest));
            }
        }
    }
    chain.node_outputs = auto_ctx.node_outputs.into_own();
    Ok(chain)
}

/// Execute one run with the strategy its kind needs.
fn run_one<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    run: &Run,
    files: Vec<PipelineFile>,
    position: ChainPosition,
) -> Result<NodeExecutionResult, ChainFailure> {
    match run {
        Run::PerFileSequence { start, len } => {
            let seq = &nodes[*start..*start + *len];
            run_per_file_sequence(ctx, seq, files, position.offset(*start))
        }
        Run::Container { index } | Run::Batch { index } => {
            run_single_node(ctx, nodes, *index, files, position)
        }
        Run::Source { index } => run_single_node(ctx, nodes, *index, vec![], position),
    }
}

/// Run the node at `index` as a one-node chain, numbered within the whole level.
fn run_single_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    index: usize,
    files: Vec<PipelineFile>,
    position: ChainPosition,
) -> Result<NodeExecutionResult, ChainFailure> {
    let node = &nodes[index..index + 1];
    run_node_chain(ctx, node, files, 0, position.offset(index))
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

impl Run {
    /// Index of the first node after this run.
    fn end(&self) -> usize {
        match self {
            Run::PerFileSequence { start, len } => start + len,
            Run::Container { index } | Run::Batch { index } | Run::Source { index } => index + 1,
        }
    }
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
#[expect(clippy::too_many_lines)]
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
/// Same semantics as `execute_loop` in container.rs, output metadata included:
/// each file's pass is independent, and the passes' records merge in order.
/// `position` places the sequence within the pipeline.
fn run_per_file_sequence<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    position: ChainPosition,
) -> Result<NodeExecutionResult, ChainFailure> {
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;
    let mut all_warnings: Vec<String> = Vec::new();
    let mut records = NodeRecords::new();

    for (i, file) in files.into_iter().enumerate() {
        let result = run_node_chain(ctx, nodes, vec![file], i, position)?;
        total_processed += result.files_processed;
        all_warnings.extend(result.warnings);
        merge_records(&mut records, result.node_outputs, ctx.count_keys);
        all_output_files.extend(result.output_files);
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
        warnings: all_warnings,
        node_outputs: records,
    })
}
