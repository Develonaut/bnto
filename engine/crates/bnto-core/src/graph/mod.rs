// Level graph — turns one level's `nodes` + `edges` (the root, or one
// container's children) into an execution order and the routing of files
// between nodes. No edges means today's linear chain.

mod error;
mod order;

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::pipeline::{PipelineNode, is_io_node};
use crate::pipeline_edge::PipelineEdge;

pub use error::GraphError;

const INPUT: &str = "input";
const OUTPUT: &str = "output";

/// Where a node's files come from, or where a level's output comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source<'a> {
    /// The files the level receives: run files at the root, the batch for a
    /// group, the iteration's file for a loop.
    LevelInput,
    /// The output of the sibling node with this id.
    Node(&'a str),
}

/// How one level runs.
#[derive(Debug)]
pub enum Flow<'a> {
    /// No edges: processing nodes in `nodes` order, each feeding the next.
    Sequence(Vec<&'a PipelineNode>),
    /// Edges: nodes run in topological order, files routed along edges.
    Graph(LevelGraph<'a>),
}

/// A validated level graph.
#[derive(Debug)]
pub struct LevelGraph<'a> {
    /// Processing nodes in execution order (I/O nodes excluded).
    pub order: Vec<&'a PipelineNode>,
    /// Each processing node's sources, in the order of the level's `edges`.
    /// Empty when the node starts with no files.
    pub inputs: BTreeMap<&'a str, Vec<Source<'a>>>,
    /// What the level hands back: sources of edges into `output` in edge
    /// order, or, with no `output` node, the sinks in `nodes` order.
    pub outputs: Vec<Source<'a>>,
}

/// Build the flow for one level from its `nodes` and `edges`.
///
/// No edges gives `Sequence` (today's chain). Otherwise the edges are
/// validated and the processing nodes ordered topologically, ready nodes
/// taken in `nodes` order. The executor and `bnto check` share this so a
/// recipe is refused for the same reasons in both.
pub fn level_flow<'a>(
    nodes: &'a [PipelineNode],
    edges: &'a [PipelineEdge],
) -> Result<Flow<'a>, GraphError> {
    let level = Level::new(nodes);
    if edges.is_empty() {
        return Ok(Flow::Sequence(level.processing));
    }
    let order = checked_order(&level, nodes, edges).map_err(|mut errors| errors.swap_remove(0))?;
    Ok(Flow::Graph(LevelGraph {
        inputs: inputs(&level, edges),
        outputs: outputs(&level, edges),
        order,
    }))
}

/// Every reason `level_flow` would refuse one level, in a fixed order: each
/// edge's problem in `edges` order, then each disconnected node in `nodes`
/// order, then one cycle. Empty when the level runs (or has no edges).
///
/// `level_flow` refuses with the first item, so a recipe `bnto check`
/// passes is one the executor accepts. The cycle search only follows edges
/// between processing nodes at this level. No unknown, into-`input` or
/// out-of-`output` edge is one, and a repeated edge adds no new path, so it
/// searches the valid subgraph. It names one cycle per level; another shows
/// up once that one is fixed.
pub fn level_errors(nodes: &[PipelineNode], edges: &[PipelineEdge]) -> Vec<GraphError> {
    if edges.is_empty() {
        return Vec::new();
    }
    checked_order(&Level::new(nodes), nodes, edges)
        .err()
        .unwrap_or_default()
}

/// The execution order, or every error at the level (never empty).
fn checked_order<'a>(
    level: &Level<'a>,
    nodes: &[PipelineNode],
    edges: &[PipelineEdge],
) -> Result<Vec<&'a PipelineNode>, Vec<GraphError>> {
    let mut errors = edge_errors(level, edges);
    errors.extend(disconnected(nodes, edges));
    match order::topological(&level.processing, edges) {
        Ok(order) if errors.is_empty() => Ok(order),
        Ok(_) => Err(errors),
        Err(cycle) => {
            errors.push(cycle);
            Err(errors)
        }
    }
}

/// One level's nodes, indexed for edge lookups.
struct Level<'a> {
    by_id: HashMap<&'a str, &'a PipelineNode>,
    processing: Vec<&'a PipelineNode>,
    has_input: bool,
    has_output: bool,
}

impl<'a> Level<'a> {
    fn new(nodes: &'a [PipelineNode]) -> Self {
        let mut by_id = HashMap::new();
        for node in nodes {
            by_id.entry(node.id.as_str()).or_insert(node);
        }
        let has_type = |t: &str| nodes.iter().any(|n| n.node_type == t);
        Level {
            by_id,
            processing: nodes.iter().filter(|n| !is_io_node(&n.node_type)).collect(),
            has_input: has_type(INPUT),
            has_output: has_type(OUTPUT),
        }
    }

    fn is(&self, id: &str, node_type: &str) -> bool {
        self.by_id.get(id).is_some_and(|n| n.node_type == node_type)
    }

    /// The source an edge carries: the level's files when it leaves `input`.
    fn source_of(&self, edge: &'a PipelineEdge) -> Source<'a> {
        if self.is(&edge.source, INPUT) {
            Source::LevelInput
        } else {
            Source::Node(&edge.source)
        }
    }
}

/// Edges to unknown ids, into `input`, out of `output`, or repeated: at most
/// one error per edge, in edge order. A bad edge never counts as the
/// earlier edge a later one repeats.
fn edge_errors(level: &Level<'_>, edges: &[PipelineEdge]) -> Vec<GraphError> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for edge in edges {
        if let Err(error) = validate_edge(level, edge) {
            errors.push(error);
        } else if !seen.insert((edge.source.as_str(), edge.target.as_str())) {
            errors.push(GraphError::DuplicateEdge {
                edge: edge.id.clone(),
            });
        }
    }
    errors
}

/// Refuse one edge whose ends aren't siblings, or that enters `input` or
/// leaves `output`.
fn validate_edge(level: &Level<'_>, edge: &PipelineEdge) -> Result<(), GraphError> {
    for id in [&edge.source, &edge.target] {
        if !level.by_id.contains_key(id.as_str()) {
            let (edge, id) = (edge.id.clone(), id.clone());
            return Err(GraphError::UnknownNode { edge, id });
        }
    }
    let edge_id = edge.id.clone();
    if level.is(&edge.target, INPUT) {
        return Err(GraphError::IntoInput { edge: edge_id });
    }
    if level.is(&edge.source, OUTPUT) {
        return Err(GraphError::FromOutput { edge: edge_id });
    }
    Ok(())
}

/// Each processing or `output` node that no edge touches, in `nodes` order.
fn disconnected(nodes: &[PipelineNode], edges: &[PipelineEdge]) -> Vec<GraphError> {
    let touched: HashSet<&str> = edges
        .iter()
        .flat_map(|e| [e.source.as_str(), e.target.as_str()])
        .collect();
    nodes
        .iter()
        .filter(|n| n.node_type != INPUT && !touched.contains(n.id.as_str()))
        .map(|n| GraphError::Disconnected { node: n.id.clone() })
        .collect()
}

/// Each processing node's sources in edge order. With no `input` node, the
/// first node in `nodes` order with no incoming edge gets the level's files.
fn inputs<'a>(level: &Level<'a>, edges: &'a [PipelineEdge]) -> BTreeMap<&'a str, Vec<Source<'a>>> {
    let mut inputs: BTreeMap<&str, Vec<Source>> = level
        .processing
        .iter()
        .map(|n| (n.id.as_str(), Vec::new()))
        .collect();
    for edge in edges {
        if let Some(sources) = inputs.get_mut(edge.target.as_str()) {
            sources.push(level.source_of(edge));
        }
    }
    if !level.has_input {
        feed_first_root(level, &mut inputs);
    }
    inputs
}

/// At a level with no `input` node, hand the level's files to the first
/// node in `nodes` order that has no incoming edge.
fn feed_first_root<'a>(level: &Level<'a>, inputs: &mut BTreeMap<&'a str, Vec<Source<'a>>>) {
    let first_root = level
        .processing
        .iter()
        .find(|n| inputs[n.id.as_str()].is_empty());
    if let Some(root) = first_root {
        inputs.insert(&root.id, vec![Source::LevelInput]);
    }
}

/// The level's output: edges into `output` in edge order or, with no
/// `output` node, the sinks in `nodes` order.
fn outputs<'a>(level: &Level<'a>, edges: &'a [PipelineEdge]) -> Vec<Source<'a>> {
    if level.has_output {
        return edges
            .iter()
            .filter(|e| level.is(&e.target, OUTPUT))
            .map(|e| level.source_of(e))
            .collect();
    }
    let sources: HashSet<&str> = edges.iter().map(|e| e.source.as_str()).collect();
    level
        .processing
        .iter()
        .filter(|n| !sources.contains(n.id.as_str()))
        .map(|n| Source::Node(&n.id))
        .collect()
}

#[cfg(test)]
mod tests;
