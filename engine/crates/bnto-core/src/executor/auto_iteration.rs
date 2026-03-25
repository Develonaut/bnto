// Auto-Iteration — implicit per-file looping for flat recipes.
//
// When `settings.iteration = "auto"`, the executor wraps contiguous per-file
// processor sequences in implicit per-file loops. This produces identical
// output to explicit loop containers but without requiring recipe authors
// to manually wrap their processors in loop/group nodes.

use crate::errors::BntoError;
use crate::pipeline::{PipelineFile, PipelineNode, is_container_node};

use super::{NodeExecutionResult, PipelineContext, run_node_chain};

/// Execute a flat node list with auto-iteration: contiguous primitive nodes
/// are run once per file (implicit per-file loop), containers dispatch as-is.
pub(super) fn run_auto_iteration<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
) -> Result<(Vec<PipelineFile>, usize), BntoError> {
    let runs = partition_into_runs(nodes);
    let mut current_files = files;
    let mut total_processed: usize = 0;

    for run in &runs {
        match run {
            Run::PerFileSequence { start, len } => {
                let seq = &nodes[*start..*start + *len];
                let result = run_per_file_sequence(ctx, seq, current_files)?;
                total_processed += result.files_processed;
                current_files = result.output_files;
            }
            Run::Container { index } => {
                let container = &nodes[*index..*index + 1];
                let (output, processed) = run_node_chain(ctx, container, current_files, 0)?;
                total_processed += processed;
                current_files = output;
            }
        }
    }

    Ok((current_files, total_processed))
}

/// A contiguous run of nodes that share an execution strategy.
enum Run {
    /// Contiguous primitive (non-container) nodes — wrapped in an implicit
    /// per-file loop. Each file passes through the full sequence.
    PerFileSequence { start: usize, len: usize },
    /// A container node — dispatched as-is (containers define own iteration).
    Container { index: usize },
}

/// Partition the flat node list into runs based on node type.
fn partition_into_runs(nodes: &[&PipelineNode]) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut seq_start: Option<usize> = None;

    for (i, node) in nodes.iter().enumerate() {
        if is_container_node(&node.node_type) {
            // Flush any pending primitive sequence
            if let Some(start) = seq_start.take() {
                runs.push(Run::PerFileSequence {
                    start,
                    len: i - start,
                });
            }
            runs.push(Run::Container { index: i });
        } else if seq_start.is_none() {
            seq_start = Some(i);
        }
    }

    // Flush trailing primitive sequence
    if let Some(start) = seq_start {
        runs.push(Run::PerFileSequence {
            start,
            len: nodes.len() - start,
        });
    }

    runs
}

/// Run a sequence of primitive nodes once per file, collecting all outputs.
/// Same semantics as `execute_loop` in container.rs.
fn run_per_file_sequence<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
) -> Result<NodeExecutionResult, BntoError> {
    let mut all_output_files: Vec<PipelineFile> = Vec::new();
    let mut total_processed: usize = 0;

    for (i, file) in files.into_iter().enumerate() {
        let (output, processed) = run_node_chain(ctx, nodes, vec![file], i)?;
        total_processed += processed;
        all_output_files.extend(output);
    }

    Ok(NodeExecutionResult {
        files_processed: total_processed,
        output_files: all_output_files,
    })
}
