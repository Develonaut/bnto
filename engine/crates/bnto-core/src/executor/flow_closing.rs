// Closing a graph level after a failure. The failing segment has already
// closed its own chain; what it handed over stands as its last node's
// output. Every later step in execution order then either runs, if it is
// marked `always`, on what its incoming edges carry, or is skipped (no
// events) and passes its input through, so an `always` step after a fan-in
// still gets every branch's files in edge order. The level then hands over
// what its output sources carry, as a passing level would.

use crate::graph::LevelGraph;

use super::closing::ChainFailure;
use super::routed::Routed;
use super::{ChainPosition, PipelineContext};

/// Close `graph` after the segment ending at `failed_end` (an index into
/// the execution order) failed with `failure`.
pub(super) fn close_graph<'a, F: Fn() -> u64 + Copy>(
    ctx: &mut PipelineContext<F>,
    graph: &LevelGraph<'a>,
    routed: &mut Routed<'a>,
    mut failure: ChainFailure,
    failed_end: usize,
    file_offset: usize,
) -> ChainFailure {
    let position = ChainPosition::whole(graph.order.len());
    routed.hand_over(
        graph.order[failed_end - 1].id.as_str(),
        failure.take_files(),
    );
    for (index, node) in graph.order.iter().enumerate().skip(failed_end) {
        let mut files = routed.take(&graph.inputs[node.id.as_str()]);
        if node.always {
            failure = failure.close_with(ctx, node, files, position.offset(index), file_offset);
            files = failure.take_files();
        }
        routed.hand_over(node.id.as_str(), files);
    }
    let output = routed.take(&graph.outputs);
    failure.handing_over(output)
}
