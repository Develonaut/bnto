// A recipe's container steps, read from its definition so the running screen
// lists them before they start instead of adding each as it begins.

use bnto_core::PipelineDefinition;
use bnto_core::pipeline::PipelineNode;

/// `(id, type)` of the processing steps inside container `node_id`, in
/// recipe order. Empty when the definition doesn't parse or has no such node.
pub(super) fn container_steps(definition_json: &str, node_id: &str) -> Vec<(String, String)> {
    let Ok(def) = serde_json::from_str::<PipelineDefinition>(definition_json) else {
        return Vec::new();
    };
    let Some(children) = find(&def.nodes, node_id).and_then(|n| n.children.as_ref()) else {
        return Vec::new();
    };
    children
        .iter()
        .filter(|c| c.node_type != "input" && c.node_type != "output")
        .map(|c| (c.id.clone(), c.node_type.clone()))
        .collect()
}

fn find<'a>(nodes: &'a [PipelineNode], id: &str) -> Option<&'a PipelineNode> {
    nodes.iter().find_map(|node| {
        if node.id == id {
            Some(node)
        } else {
            find(node.children.as_deref().unwrap_or_default(), id)
        }
    })
}
