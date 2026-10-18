// Execution order — Kahn's topological sort over one level's processing
// nodes. Ready nodes are taken in `nodes` order, so a chain whose edges
// follow `nodes` keeps today's order. Leftover nodes mean a cycle, which is
// traced back so the error can name it.

use std::collections::{BTreeSet, HashMap};

use super::GraphError;
use crate::pipeline::PipelineNode;
use crate::pipeline_edge::PipelineEdge;

/// Edges between processing nodes, as (source, target) indices into `nodes`.
/// Edges touching `input` / `output` don't constrain the order.
fn links(nodes: &[&PipelineNode], edges: &[PipelineEdge]) -> Vec<(usize, usize)> {
    let mut index = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        index.entry(node.id.as_str()).or_insert(i);
    }
    edges
        .iter()
        .filter_map(|e| {
            Some((
                *index.get(e.source.as_str())?,
                *index.get(e.target.as_str())?,
            ))
        })
        .collect()
}

/// Order `nodes` so every edge's source runs before its target, or name a
/// cycle when that's impossible.
pub(super) fn topological<'a>(
    nodes: &[&'a PipelineNode],
    edges: &[PipelineEdge],
) -> Result<Vec<&'a PipelineNode>, GraphError> {
    let links = links(nodes, edges);
    let mut indegree = vec![0usize; nodes.len()];
    for &(_, target) in &links {
        indegree[target] += 1;
    }
    let order = kahn(&links, &mut indegree);
    if order.len() < nodes.len() {
        let path = cycle_path(&links, &indegree);
        let path = path.into_iter().map(|i| nodes[i].id.clone()).collect();
        return Err(GraphError::Cycle { path });
    }
    Ok(order.into_iter().map(|i| nodes[i]).collect())
}

/// Place nodes whose sources have all been placed, lowest index first.
/// Leaves `indegree` > 0 on every node it could not place.
fn kahn(links: &[(usize, usize)], indegree: &mut [usize]) -> Vec<usize> {
    let mut ready: BTreeSet<usize> = (0..indegree.len()).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::with_capacity(indegree.len());
    while let Some(next) = ready.pop_first() {
        order.push(next);
        for &(_, target) in links.iter().filter(|&&(source, _)| source == next) {
            indegree[target] -= 1;
            if indegree[target] == 0 {
                ready.insert(target);
            }
        }
    }
    order
}

/// Trace a cycle among the nodes Kahn left unplaced. Each of them still has
/// an unplaced predecessor, so walking predecessors must revisit a node; the
/// revisited stretch, reversed, is the cycle in edge direction.
fn cycle_path(links: &[(usize, usize)], indegree: &[usize]) -> Vec<usize> {
    let start = indegree.iter().position(|&d| d > 0);
    let mut walk = vec![start.expect("a cycle leaves nodes unplaced")];
    loop {
        let here = *walk.last().expect("walk starts non-empty");
        let prev = unplaced_predecessor(links, indegree, here);
        if let Some(at) = walk.iter().position(|&i| i == prev) {
            let mut cycle = walk.split_off(at);
            cycle.push(prev);
            cycle.reverse();
            return cycle;
        }
        walk.push(prev);
    }
}

/// The first unplaced node with an edge into `here`, in edge order.
fn unplaced_predecessor(links: &[(usize, usize)], indegree: &[usize], here: usize) -> usize {
    links
        .iter()
        .find(|&&(source, target)| target == here && indegree[source] > 0)
        .map(|&(source, _)| source)
        .expect("an unplaced node has an unplaced predecessor")
}
