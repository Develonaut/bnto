// Recipe check — find what would break a run without running it.
//
// Walks the nodes the way the executor does (I/O markers skipped, container
// children entered) and reports unknown node types, params a node's
// `validate` refuses, and malformed `{{…}}` placeholders.

mod placeholder;

use std::collections::BTreeMap;

use crate::executor::resolve::{collect_field_values, resolve_fields};
use crate::pipeline::{PipelineDefinition, PipelineNode, is_container_node, is_io_node};
use crate::registry::NodeRegistry;

/// One problem `check_pipeline` found in a recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckFinding {
    /// The id of the node the problem is in.
    pub node_id: String,
    /// That node's type key.
    pub node_type: String,
    /// The top-level param at fault, when one is.
    pub param: Option<String>,
    /// What is wrong.
    pub message: String,
}

impl std::fmt::Display for CheckFinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node \"{}\" ({})", self.node_id, self.node_type)?;
        if let Some(param) = &self.param {
            write!(f, ", param \"{param}\"")?;
        }
        write!(f, ": {}", self.message)
    }
}

/// Check every node in `definition` against `registry` without running it.
///
/// Placeholders are checked for form only, so a recipe needs no vars, env or
/// inputs to pass. An empty result means the recipe is sound.
pub fn check_pipeline(
    definition: &PipelineDefinition,
    registry: &NodeRegistry,
) -> Vec<CheckFinding> {
    let mut findings = Vec::new();
    check_nodes(&definition.nodes, registry, &mut findings);
    findings
}

/// Check `nodes` in order, recursing into container children.
fn check_nodes(nodes: &[PipelineNode], registry: &NodeRegistry, findings: &mut Vec<CheckFinding>) {
    for node in nodes.iter().filter(|n| !is_io_node(&n.node_type)) {
        if is_container_node(&node.node_type) {
            findings.extend(placeholder_findings(node));
            check_nodes(
                node.children.as_deref().unwrap_or_default(),
                registry,
                findings,
            );
        } else {
            findings.extend(processor_findings(node, registry));
            findings.extend(placeholder_findings(node));
        }
    }
}

/// An unknown type, or what the node's processor refuses in its params.
///
/// `validate` sees the params as a run with no overrides would: field
/// defaults filled in, other placeholders left as written.
fn processor_findings(node: &PipelineNode, registry: &NodeRegistry) -> Vec<CheckFinding> {
    let Some(processor) = registry.resolve(&node.node_type, &node.params) else {
        let message = format!("unknown node type \"{}\"", node.node_type);
        return vec![finding(node, None, message)];
    };
    let field_values = collect_field_values(&node.fields, &BTreeMap::new());
    let params = resolve_fields(&node.params, &field_values);
    processor
        .validate(&params)
        .into_iter()
        .map(|message| finding(node, None, message))
        .collect()
}

/// Malformed placeholders anywhere in the node's params.
fn placeholder_findings(node: &PipelineNode) -> Vec<CheckFinding> {
    node.params
        .iter()
        .flat_map(|(key, value)| {
            placeholder::value_problems(value)
                .into_iter()
                .map(move |message| finding(node, Some(key), message))
        })
        .collect()
}

fn finding(node: &PipelineNode, param: Option<&str>, message: String) -> CheckFinding {
    CheckFinding {
        node_id: node.id.clone(),
        node_type: node.node_type.clone(),
        param: param.map(str::to_string),
        message,
    }
}

#[cfg(test)]
mod tests;
