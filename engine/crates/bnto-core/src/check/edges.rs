// Edge check — every reason one level's edges would be refused at run time,
// as findings on the node at fault. Uses the executor's own `level_errors`,
// so check and run never disagree.

use super::{CheckFinding, finding};
use crate::graph::{GraphError, level_errors};
use crate::pipeline::PipelineNode;
use crate::pipeline_edge::PipelineEdge;

const SAME_LEVEL: &str = "edges only connect nodes at the same level";
const DISCONNECTED: &str = "no edge connects it; when a level has edges, every node must be on one";
const INPUT: &str = "input";
const OUTPUT: &str = "output";

/// One level: the root, or a container's children.
pub(super) struct Level<'a> {
    pub nodes: &'a [PipelineNode],
    pub edges: &'a [PipelineEdge],
    /// The container whose children these are; `None` at the root.
    pub owner: Option<&'a PipelineNode>,
}

/// Findings for every graph error at `level`. `root` is the whole recipe's
/// top level, searched to say where an id from another level lives.
pub(super) fn edge_findings(level: &Level<'_>, root: &[PipelineNode]) -> Vec<CheckFinding> {
    level_errors(level.nodes, level.edges)
        .iter()
        .map(|error| error_finding(level, root, error))
        .collect()
}

/// The finding for one error, on the edge end that isn't the problem.
fn error_finding(level: &Level<'_>, root: &[PipelineNode], error: &GraphError) -> CheckFinding {
    let message = error.to_string();
    match error {
        GraphError::UnknownNode { edge, id } => unknown_finding(level, root, edge, id),
        GraphError::DuplicateEdge { edge } => {
            let source = find_edge(level, edge, |_| true).map(|e| &e.source);
            on_node(level, source, message)
        }
        GraphError::IntoInput { edge } => {
            let source = find_edge(level, edge, |e| is(level, &e.target, INPUT));
            on_node(level, source.map(|e| &e.source), message)
        }
        GraphError::FromOutput { edge } => {
            let target = find_edge(level, edge, |e| is(level, &e.source, OUTPUT));
            on_node(level, target.map(|e| &e.target), message)
        }
        GraphError::Cycle { path } => on_node(level, path.first(), message),
        GraphError::Disconnected { node } => on_node(level, Some(node), DISCONNECTED.into()),
    }
}

/// An edge naming `id`, which isn't at this level: reported on the edge's
/// other end, saying where `id` actually is.
fn unknown_finding(level: &Level<'_>, root: &[PipelineNode], edge: &str, id: &str) -> CheckFinding {
    let found = find_edge(level, edge, |e| e.source == id || e.target == id);
    let other = found.map(|e| if e.source == id { &e.target } else { &e.source });
    let message = unknown_message(edge, id, locate(root, None, id));
    match other.and_then(|other| node_at(level, other)) {
        Some(node) => finding(node, None, message),
        None => orphan(level, id, message),
    }
}

fn unknown_message(edge: &str, id: &str, place: Option<Option<&str>>) -> String {
    let named = format!("edge \"{edge}\" connects to \"{id}\"");
    match place {
        None => format!("{named}, which is not a node in this recipe"),
        Some(None) => format!("{named}, which is a top-level node; {SAME_LEVEL}"),
        Some(Some(container)) => {
            format!("{named}, which is inside container \"{container}\"; {SAME_LEVEL}")
        }
    }
}

/// Where `id` is in the recipe: `Some(None)` at the top level,
/// `Some(Some(container))` among that container's children, `None` nowhere.
fn locate<'a>(
    nodes: &'a [PipelineNode],
    parent: Option<&'a str>,
    id: &str,
) -> Option<Option<&'a str>> {
    if nodes.iter().any(|n| n.id == id) {
        return Some(parent);
    }
    nodes.iter().find_map(|n| {
        let children = n.children.as_deref().unwrap_or_default();
        locate(children, Some(n.id.as_str()), id)
    })
}

/// The first edge with id `edge` that `matches` (edge ids may repeat).
fn find_edge<'a>(
    level: &Level<'a>,
    edge: &str,
    matches: impl Fn(&PipelineEdge) -> bool,
) -> Option<&'a PipelineEdge> {
    level.edges.iter().find(|e| e.id == edge && matches(e))
}

fn node_at<'a>(level: &Level<'a>, id: &str) -> Option<&'a PipelineNode> {
    level.nodes.iter().find(|n| n.id == id)
}

fn is(level: &Level<'_>, id: &str, node_type: &str) -> bool {
    node_at(level, id).is_some_and(|n| n.node_type == node_type)
}

/// A finding on the node `id` at this level.
fn on_node(level: &Level<'_>, id: Option<&String>, message: String) -> CheckFinding {
    let id = id.map(String::as_str).unwrap_or_default();
    match node_at(level, id) {
        Some(node) => finding(node, None, message),
        None => orphan(level, id, message),
    }
}

/// A finding with no node at this level to put it on: the container that
/// owns the level, or at the root the missing id itself.
fn orphan(level: &Level<'_>, id: &str, message: String) -> CheckFinding {
    match level.owner {
        Some(owner) => finding(owner, None, message),
        None => CheckFinding {
            node_id: id.to_string(),
            node_type: "unknown".to_string(),
            param: None,
            message,
        },
    }
}
