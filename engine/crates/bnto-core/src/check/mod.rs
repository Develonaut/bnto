// Recipe check — find what would break a run without running it.
//
// Walks the nodes the way the executor does (I/O markers skipped, container
// children entered) and reports unknown node types, params a node's
// `validate` refuses, malformed `{{…}}` placeholders, `always` on an I/O
// marker, where it has no effect, and every edge problem at every level.

mod edges;
mod placeholder;

use std::collections::BTreeMap;

use crate::executor::resolve::{collect_field_values, resolve_fields};
use crate::pipeline::{PipelineDefinition, PipelineNode, is_container_node, is_io_node};
use crate::registry::NodeRegistry;
use edges::{Level, edge_findings};

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
    let root = Level {
        nodes: &definition.nodes,
        edges: &definition.edges,
        owner: None,
    };
    let recipe = Recipe {
        root: &definition.nodes,
        registry,
    };
    check_level(&root, &recipe, &mut findings);
    findings
}

/// What every level's check needs from the whole recipe.
struct Recipe<'a> {
    root: &'a [PipelineNode],
    registry: &'a NodeRegistry,
}

/// Check a level's edges, then its nodes in order, recursing into container
/// children.
fn check_level(level: &Level<'_>, recipe: &Recipe<'_>, findings: &mut Vec<CheckFinding>) {
    findings.extend(edge_findings(level, recipe.root));
    for node in level.nodes {
        check_node(node, recipe, findings);
    }
}

fn check_node(node: &PipelineNode, recipe: &Recipe<'_>, findings: &mut Vec<CheckFinding>) {
    if is_io_node(&node.node_type) {
        findings.extend(io_always_finding(node));
    } else if is_container_node(&node.node_type) {
        findings.extend(placeholder_findings(node));
        let children = Level {
            nodes: node.children.as_deref().unwrap_or_default(),
            edges: &node.edges,
            owner: Some(node),
        };
        check_level(&children, recipe, findings);
    } else {
        findings.extend(processor_findings(node, recipe.registry));
        findings.extend(placeholder_findings(node));
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

/// `always` on an I/O marker: the executor never runs it, so a recipe that
/// marks its output step expecting files to be written after a failure
/// would be silently wrong. The closing steps themselves need the mark.
fn io_always_finding(node: &PipelineNode) -> Option<CheckFinding> {
    let message = "\"always\" has no effect on an input or output node; \
                   mark the processing steps that should run after a failure";
    node.always
        .then(|| finding(node, None, message.to_string()))
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
