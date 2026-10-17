// Count keys — which nodes add up which output metadata keys across runs.
//
// Built once per run from the definition, so a merge anywhere in the run
// (a loop's iterations, auto-iteration's per-file passes) knows a node's
// count keys by id without going back to the registry.

use crate::pipeline::PipelineNode;
use crate::registry::NodeRegistry;

use super::node_outputs::CountKeys;

/// The count keys of every node in `nodes`, container children included.
/// A node that declares none (see `NodeProcessor::count_keys`) is left out.
pub(super) fn count_keys_for(nodes: &[PipelineNode], registry: &NodeRegistry) -> CountKeys {
    let mut keys = CountKeys::new();
    collect(nodes, registry, &mut keys);
    keys
}

fn collect(nodes: &[PipelineNode], registry: &NodeRegistry, keys: &mut CountKeys) {
    for node in nodes {
        if let Some(children) = &node.children {
            collect(children, registry, keys);
        }
        let counted = registry
            .resolve(&node.node_type, &node.params)
            .map(|p| p.count_keys())
            .unwrap_or_default();
        if !counted.is_empty() {
            keys.insert(node.id.clone(), counted);
        }
    }
}
