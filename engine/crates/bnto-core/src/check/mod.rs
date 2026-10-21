// Recipe check — find what would break a run without running it.
//
// Walks the nodes the way the executor does (I/O markers skipped, container
// children entered) and reports unknown node types, params a node's
// `validate` refuses, malformed `{{…}}` placeholders, `always` on an I/O
// marker, where it has no effect, and every edge problem at every level.
// A param left out for `bnto run -p` to give is a note, not a problem.

mod edges;
mod placeholder;
mod run_time;
mod when;

use std::collections::BTreeMap;

use crate::executor::resolve::{collect_field_values, resolve_fields};
use crate::pipeline::{PipelineDefinition, PipelineNode, is_container_node, is_io_node};
use crate::processor::NodeProcessor;
use crate::registry::NodeRegistry;
use crate::renamed::with_node_type_hint;
use crate::undeclared_params::undeclared_params;
use edges::{Level, edge_findings};
pub use run_time::RunTimeParam;
use run_time::{LeftOut, Subject, left_out};

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

/// What `check_pipeline` found in a recipe.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CheckReport {
    /// Problems that would break a run. Empty means the recipe is sound.
    pub findings: Vec<CheckFinding>,
    /// Params the recipe leaves out for the run to give with `-p`.
    pub run_time: Vec<RunTimeParam>,
}

/// Check every node in `definition` against `registry` without running it.
///
/// Placeholders are checked for form only, so a recipe needs no vars, env or
/// inputs to pass, and a param `-p` can give may be left out.
pub fn check_pipeline(definition: &PipelineDefinition, registry: &NodeRegistry) -> CheckReport {
    let mut report = CheckReport::default();
    let root = Level {
        nodes: &definition.nodes,
        edges: &definition.edges,
        owner: None,
    };
    let recipe = Recipe {
        root: &definition.nodes,
        registry,
    };
    check_level(&root, &recipe, &mut report);
    report
}

/// What every level's check needs from the whole recipe.
struct Recipe<'a> {
    root: &'a [PipelineNode],
    registry: &'a NodeRegistry,
}

/// Check a level's edges, then its nodes in order, recursing into container
/// children.
fn check_level(level: &Level<'_>, recipe: &Recipe<'_>, report: &mut CheckReport) {
    report.findings.extend(edge_findings(level, recipe.root));
    for node in level.nodes {
        check_node(node, recipe, report);
    }
}

fn check_node(node: &PipelineNode, recipe: &Recipe<'_>, report: &mut CheckReport) {
    if is_io_node(&node.node_type) {
        report.findings.extend(io_always_finding(node));
        report.findings.extend(when::io_when_finding(node));
        return;
    }
    report.findings.extend(when::when_findings(node));
    if is_container_node(&node.node_type) {
        report.findings.extend(placeholder_findings(node));
        let children = Level {
            nodes: node.children.as_deref().unwrap_or_default(),
            edges: &node.edges,
            owner: Some(node),
        };
        check_level(&children, recipe, report);
    } else {
        check_processor(node, recipe, report);
        report.findings.extend(placeholder_findings(node));
    }
}

/// An unknown type, or what the node's processor refuses in its params, with
/// the params it needs sorted into notes and problems.
///
/// `validate` sees the params as a run with no overrides would: field
/// defaults filled in, other placeholders left as written.
fn check_processor(node: &PipelineNode, recipe: &Recipe<'_>, report: &mut CheckReport) {
    let Some(processor) = recipe.registry.resolve(&node.node_type, &node.params) else {
        report.findings.push(unknown_type_finding(node));
        return;
    };
    report.findings.extend(undeclared_findings(node, processor));
    let raw = processor.validate_raw(&node.params).into_iter();
    report
        .findings
        .extend(raw.map(|problem| finding(node, None, problem)));
    let field_values = collect_field_values(&node.fields, &BTreeMap::new());
    let params = resolve_fields(&node.params, &field_values);
    let written = processor.validate(&params);
    if written.is_empty() {
        return;
    }
    let subject = Subject {
        node,
        processor,
        params: &params,
    };
    add_left_out(node, left_out(&subject, written, recipe.root), report);
}

/// Report `left`: what `validate` still refuses, why each blocked param is
/// out of reach, and a note per param left for run time.
fn add_left_out(node: &PipelineNode, left: LeftOut, report: &mut CheckReport) {
    let problems = left.problems.into_iter().map(|m| finding(node, None, m));
    report.findings.extend(problems);
    for (param, reason) in left.blocked {
        let message = format!("required, and no run can give it: {reason}");
        report.findings.push(finding(node, Some(&param), message));
    }
    report
        .run_time
        .extend(left.givable.into_iter().map(|param| RunTimeParam {
            node_id: node.id.clone(),
            node_type: node.node_type.clone(),
            param,
        }));
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

/// No processor has the node's type; say so, with the new name when a
/// migration renamed it.
fn unknown_type_finding(node: &PipelineNode) -> CheckFinding {
    let message = format!("unknown node type \"{}\"", node.node_type);
    finding(node, None, with_node_type_hint(message, &node.node_type))
}

/// A finding per param the node gives that its processor doesn't declare.
fn undeclared_findings(node: &PipelineNode, processor: &dyn NodeProcessor) -> Vec<CheckFinding> {
    let Some(declared) = processor.declared_params() else {
        return Vec::new();
    };
    undeclared_params(&node.node_type, &node.params, &declared)
        .into_iter()
        .map(|u| finding(node, Some(&u.param), u.message))
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
