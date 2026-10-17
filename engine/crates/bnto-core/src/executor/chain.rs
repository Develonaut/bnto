// Node Chains — run nodes in order, each one's output feeding the next.
//
// Every node list the executor walks is a chain: the top-level nodes, a
// container's children, an auto-iteration run. When a node fails, the
// chain's closing steps (`always` nodes) take over; see `closing.rs`.

use crate::pipeline::PipelineFile;

use super::closing::ChainFailure;
use super::node_outputs::NodeRecords;
use super::{ChainPosition, NodeExecutionResult, PipelineContext, PipelineNodeRef, execute_node};

/// Chain nodes sequentially, passing each node's output as the next node's input.
///
/// `position` places the chain among its siblings, so a chain that is a
/// slice of the pipeline still reports pipeline-wide progress. Each node
/// records its output metadata in the chain's layer, which every later node
/// in the chain reads; the result carries what the chain recorded.
pub(super) fn run_node_chain<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[PipelineNodeRef],
    files: Vec<PipelineFile>,
    file_offset: usize,
    position: ChainPosition,
) -> Result<NodeExecutionResult, ChainFailure> {
    let mut chain_ctx = ctx.layer();
    let mut chain = NodeExecutionResult::holding(files);
    let total = position.total_nodes;
    for (i, node) in nodes.iter().enumerate() {
        let files = std::mem::take(&mut chain.output_files);
        let index = position.first_index + i;
        match execute_node(&chain_ctx, node, files, index, total, file_offset) {
            Ok(result) => {
                let records = chain.then(result);
                chain_ctx.node_outputs.add(records, ctx.count_keys);
            }
            Err(error) => {
                let failure = ChainFailure::new(&node.id, error);
                let rest = &nodes[i + 1..];
                return Err(failure.close(&mut chain_ctx, rest, file_offset, position.after(i)));
            }
        }
    }
    chain.node_outputs = chain_ctx.node_outputs.into_own();
    Ok(chain)
}

impl NodeExecutionResult {
    /// A chain's result before any node ran: its input files, nothing counted.
    pub(super) fn holding(files: Vec<PipelineFile>) -> Self {
        Self {
            output_files: files,
            files_processed: 0,
            warnings: Vec::new(),
            node_outputs: NodeRecords::new(),
        }
    }

    /// Fold in the next node's result: its files replace the chain's, its
    /// counts and warnings add up. Returns its records, for the chain's layer.
    pub(super) fn then(&mut self, next: NodeExecutionResult) -> NodeRecords {
        self.output_files = next.output_files;
        self.files_processed += next.files_processed;
        self.warnings.extend(next.warnings);
        next.node_outputs
    }
}
