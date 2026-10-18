// Pipeline edge — the execution view of a connection between two sibling
// nodes. Mirrors the TypeScript `Edge` (`id` / `source` / `target`); port
// handles are an editor concern, so serde drops them here.

use serde::{Deserialize, Serialize};

/// A directed connection from one node to a sibling at the same level.
/// Files flow from `source` to `target`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PipelineEdge {
    /// The edge's id, used to name it in error messages.
    pub id: String,
    /// Id of the node the files come from.
    pub source: String,
    /// Id of the node the files go to.
    pub target: String,
}
