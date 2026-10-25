// Level flow — run one level (the root, or a container's children) by its
// `Flow`. A level without edges, or whose edges are today's chain, runs as
// the chain it always did. A graph runs segment by segment in execution
// order, its files routed along the edges (`routed.rs`, `segments.rs`).
//
// A failure stops the level: the failing segment closes its own chain, then
// every later step is skipped except `always` ones (`flow_closing.rs`).

use std::ops::Range;

use crate::errors::BntoError;
use crate::graph::{Flow, GraphError, LevelGraph, level_flow};
use crate::pipeline::{
    IterationMode, PipelineDefinition, PipelineFile, PipelineNode, is_container_node,
};
use crate::pipeline_edge::PipelineEdge;

use super::auto_iteration::run_auto_iteration;
use super::closing::ChainFailure;
use super::flow_closing::close_graph;
use super::routed::Routed;
use super::segments::{as_chain, segments};
use super::{ChainPosition, NodeExecutionResult, PipelineContext, run_node_chain};

// --- Building flows ---

/// The root level's flow, after refusing any level whose edges can't run
/// (root first, then every container's children), before anything runs.
pub(super) fn root_flow(definition: &PipelineDefinition) -> Result<Flow<'_>, BntoError> {
    let flow = level_flow_of(&definition.nodes, &definition.edges, None)?;
    check_containers(&definition.nodes)?;
    Ok(flow)
}

/// One level's flow; `container` names the container whose children these are.
pub(super) fn level_flow_of<'a>(
    nodes: &'a [PipelineNode],
    edges: &'a [PipelineEdge],
    container: Option<&str>,
) -> Result<Flow<'a>, BntoError> {
    level_flow(nodes, edges).map_err(|error| refused(container, error))
}

/// Refuse the first container, depth first, whose children's edges can't run.
fn check_containers(nodes: &[PipelineNode]) -> Result<(), BntoError> {
    for node in nodes.iter().filter(|n| is_container_node(&n.node_type)) {
        if let Some(children) = node.children.as_deref() {
            level_flow_of(children, &node.edges, Some(&node.id))?;
            check_containers(children)?;
        }
    }
    Ok(())
}

fn refused(container: Option<&str>, error: GraphError) -> BntoError {
    BntoError::InvalidInput(match container {
        None => format!("Invalid edges: {error}"),
        Some(id) => format!("Invalid edges in container '{id}': {error}"),
    })
}

// --- Running a level ---

/// Run a level's processing nodes with the level's iteration strategy.
pub(super) fn run_level<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    flow: &Flow<'_>,
    files: Vec<PipelineFile>,
    file_offset: usize,
    mode: IterationMode,
) -> Result<NodeExecutionResult, ChainFailure> {
    match flow {
        Flow::Sequence(nodes) => run_chain(ctx, nodes, files, file_offset, mode),
        Flow::Graph(graph) => match as_chain(graph) {
            Some(nodes) => run_chain(ctx, nodes, files, file_offset, mode),
            None => run_graph(ctx, graph, files, file_offset, mode),
        },
    }
}

/// Run a level that is one chain, as every level without edges runs.
fn run_chain<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    file_offset: usize,
    mode: IterationMode,
) -> Result<NodeExecutionResult, ChainFailure> {
    let position = ChainPosition::whole(nodes.len());
    run_segment(ctx, nodes, files, file_offset, position, mode)
}

/// Run one linear run of nodes, placed among the level's nodes by `position`.
fn run_segment<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    nodes: &[&PipelineNode],
    files: Vec<PipelineFile>,
    file_offset: usize,
    position: ChainPosition,
    mode: IterationMode,
) -> Result<NodeExecutionResult, ChainFailure> {
    match mode {
        IterationMode::Explicit => run_node_chain(ctx, nodes, files, file_offset, position),
        IterationMode::Auto => run_auto_iteration(ctx, nodes, files, position),
    }
}

/// Run a graph level segment by segment. Every node records into the
/// level's layer, so a node reads any node that ran before it at this level.
/// The level hands back the files its output sources name, in their order.
/// A segment sees the files kept for later segments as held (`held.rs`).
#[expect(clippy::too_many_lines)]
fn run_graph<'a, F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    graph: &LevelGraph<'a>,
    files: Vec<PipelineFile>,
    file_offset: usize,
    mode: IterationMode,
) -> Result<NodeExecutionResult, ChainFailure> {
    let mut level_ctx = ctx.layer_with_held();
    let mut routed = Routed::new(graph, files);
    let mut level = NodeExecutionResult::holding(Vec::new());
    for range in segments(graph) {
        let end = range.end;
        let last: &'a PipelineNode = graph.order[end - 1];
        let last = last.id.as_str();
        let result =
            match run_graph_segment(&level_ctx, graph, &mut routed, range, file_offset, mode) {
                Ok(result) => result,
                Err(failure) => {
                    let ctx = &mut level_ctx;
                    return Err(close_graph(
                        ctx,
                        graph,
                        &mut routed,
                        failure,
                        end,
                        file_offset,
                    ));
                }
            };
        let records = level.then(result);
        level_ctx.node_outputs.add(records, ctx.count_keys);
        routed.hand_over(last, std::mem::take(&mut level.output_files));
    }
    level.output_files = routed.take(&graph.outputs);
    level.node_outputs = level_ctx.node_outputs.into_own();
    Ok(level)
}

/// Run the segment at `range` of the execution order on its sources' files.
fn run_graph_segment<'a, F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    graph: &LevelGraph<'a>,
    routed: &mut Routed<'a>,
    range: Range<usize>,
    file_offset: usize,
    mode: IterationMode,
) -> Result<NodeExecutionResult, ChainFailure> {
    let position = ChainPosition::starting_at(range.start, graph.order.len());
    let nodes = &graph.order[range];
    let files = routed.take(&graph.inputs[nodes[0].id.as_str()]);
    if let Some(held) = &ctx.held {
        held.replace(routed.held_files());
    }
    run_segment(ctx, nodes, files, file_offset, position, mode)
}
