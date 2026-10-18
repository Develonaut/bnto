// Graph errors — why a level's edges can't be run. Each names the edge or
// node at fault so `bnto check` and the executor can point at it.

use std::fmt;

/// A problem with one level's edges, found before anything runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// An edge's source or target is not a node at this level (unknown, or
    /// inside another container).
    UnknownNode { edge: String, id: String },
    /// An edge repeats an earlier edge's source and target.
    DuplicateEdge { edge: String },
    /// An edge leads into an `input` node.
    IntoInput { edge: String },
    /// An edge leads out of an `output` node.
    FromOutput { edge: String },
    /// The edges form a cycle; `path` walks it in edge direction and ends
    /// where it starts (a self-loop is `[a, a]`).
    Cycle { path: Vec<String> },
    /// A processing or `output` node that no edge touches.
    Disconnected { node: String },
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode { edge, id } => write!(
                f,
                "edge \"{edge}\" connects to \"{id}\", which is not a node at this level"
            ),
            Self::DuplicateEdge { edge } => write!(
                f,
                "edge \"{edge}\" repeats an earlier edge between the same two nodes"
            ),
            Self::IntoInput { edge } => write!(f, "edge \"{edge}\" leads into an input node"),
            Self::FromOutput { edge } => write!(f, "edge \"{edge}\" leads out of an output node"),
            Self::Cycle { path } => write!(f, "edges form a cycle: {}", path.join(" → ")),
            Self::Disconnected { node } => {
                write!(f, "node \"{node}\" is not connected to any edge")
            }
        }
    }
}

impl std::error::Error for GraphError {}
