// Params a recipe leaves out for `bnto run -p` to give.
//
// A left-out param (no key, or a required one given blank text or null) is
// needed when the run refuses the recipe without it (the node's `validate`,
// or a required param with no value) and stops refusing once it has a value.
// A needed param is left for run time when `-p nodeId:key=value` can give
// it. The rule mirrors how the CLI applies one (`bnto/src/input.rs`): it sets
// the first node with that id, searching nodes in order and each container's
// children before the next node; it sets a declared field of that name
// instead of the param; and its value is text, a number or true/false. A
// needed param it can't give that way no run can give, since `--vars` only
// fills `{{vars.*}}` the recipe has.

use crate::metadata::{ParameterDef, ParameterType};
use crate::pipeline::PipelineNode;
use crate::processor::NodeProcessor;
use crate::required_params::{is_blank, missing_required};

type Params = serde_json::Map<String, serde_json::Value>;

/// A param a recipe leaves out for the run to give.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunTimeParam {
    /// The id of the node the param belongs to.
    pub node_id: String,
    /// That node's type key.
    pub node_type: String,
    /// The param's name.
    pub param: String,
    /// True for a file param, which `-p` gives as a path to read.
    pub takes_file: bool,
}

impl std::fmt::Display for RunTimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "node \"{}\" ({}), param \"{}\": left for run time",
            self.node_id, self.node_type, self.param
        )
    }
}

/// What the run refuses in a node (`refusals`) once its left-out params are
/// sorted.
#[derive(Default)]
pub(super) struct LeftOut {
    /// What the run still refuses with every givable param given.
    pub problems: Vec<String>,
    /// Needed params `-p` can give: each name, and whether it takes a file.
    pub givable: Vec<(String, bool)>,
    /// Needed params no run can give, each with the reason.
    pub blocked: Vec<(String, String)>,
}

/// The node being checked: its definition, processor and resolved params.
pub(super) struct Subject<'a> {
    pub node: &'a PipelineNode,
    pub processor: &'a dyn NodeProcessor,
    pub params: &'a Params,
}

/// Sort the needed params `subject` leaves out, given `written`, what the
/// run refuses as written. A required param given as blank text or null is
/// left out too: the run's pre-flight treats it so, and the web's catalog
/// needs a required param's key present, so a bundled recipe writes `""`.
pub(super) fn left_out(
    subject: &Subject<'_>,
    written: Vec<String>,
    root: &[PipelineNode],
) -> LeftOut {
    let mut left = LeftOut::default();
    let mut given = subject.params.clone();
    let declared = subject.processor.metadata().parameters;
    let absent = declared.iter().filter(|d| is_left_out(subject.params, d));
    for def in absent.filter(|d| is_needed(subject, d, &written)) {
        match unreachable_reason(subject.node, def, root) {
            Some(reason) => left.blocked.push((def.name.clone(), reason)),
            None => {
                given.insert(def.name.clone(), stand_in(def));
                let takes_file = matches!(def.param_type, ParameterType::File { .. });
                left.givable.push((def.name.clone(), takes_file));
            }
        }
    }
    let after = refusals(subject.processor, &given);
    left.problems = written.into_iter().filter(|m| after.contains(m)).collect();
    left
}

/// True when `params` gives `def` nothing: no key, or a required param
/// given blank text or null.
fn is_left_out(params: &Params, def: &ParameterDef) -> bool {
    let required = def.constraints.as_ref().is_some_and(|c| c.required);
    params
        .get(&def.name)
        .is_none_or(|value| required && is_blank(value))
}

/// What a run refuses in `params`: what `validate` refuses, plus each
/// required param with no value that `validate` lets through, which the
/// run's pre-flight refuses.
pub(super) fn refusals(processor: &dyn NodeProcessor, params: &Params) -> Vec<String> {
    let validated = processor.validate(params);
    let declared = processor.declared_params().unwrap_or_default();
    let unrefused: Vec<String> = missing_required(params, &declared)
        .into_iter()
        .filter(|m| {
            let def = declared.iter().find(|d| d.name == m.param);
            !def.is_some_and(|def| refused_without(processor, params, &validated, def))
        })
        .map(|m| m.message)
        .collect();
    [validated, unrefused].concat()
}

/// True when giving `def` a value takes away something `validate` refused.
fn refused_without(
    processor: &dyn NodeProcessor,
    params: &Params,
    validated: &[String],
    def: &ParameterDef,
) -> bool {
    let mut given = params.clone();
    given.insert(def.name.clone(), stand_in(def));
    let after = processor.validate(&given);
    validated.iter().any(|m| !after.contains(m))
}

/// True when giving `def` a value takes away something the run refused.
fn is_needed(subject: &Subject<'_>, def: &ParameterDef, written: &[String]) -> bool {
    let mut given = subject.params.clone();
    given.insert(def.name.clone(), stand_in(def));
    let after = refusals(subject.processor, &given);
    written.iter().any(|m| !after.contains(m))
}

/// Why `-p` can't give `def` to `node`, or `None` when it can.
fn unreachable_reason(
    node: &PipelineNode,
    def: &ParameterDef,
    root: &[PipelineNode],
) -> Option<String> {
    if let Some(reason) = id_reason(&node.id) {
        return Some(reason);
    }
    if !first_with_id(root, &node.id).is_some_and(|first| std::ptr::eq(first, node)) {
        return Some(format!(
            "-p {}:… sets an earlier node with the same id",
            node.id
        ));
    }
    if node.fields.contains_key(&def.name) {
        return Some(format!(
            "-p {}:{}=… sets the node's field of that name instead",
            node.id, def.name
        ));
    }
    type_reason(&def.param_type)
}

/// `-p` splits at the first `=` and then the first `:`, so it can't name a
/// node whose id is empty or holds either.
fn id_reason(id: &str) -> Option<String> {
    if id.is_empty() {
        return Some("-p can't name a node with an empty id".to_string());
    }
    id.chars()
        .find(|c| matches!(c, ':' | '='))
        .map(|c| format!("-p can't name a node whose id holds '{c}'"))
}

/// The node `-p` sets for `id`: the first in order, each container's
/// children searched before the next node.
fn first_with_id<'a>(nodes: &'a [PipelineNode], id: &str) -> Option<&'a PipelineNode> {
    nodes.iter().find_map(|node| {
        if node.id == id {
            return Some(node);
        }
        first_with_id(node.children.as_deref().unwrap_or_default(), id)
    })
}

/// `-p` values are text, numbers or true/false; a file param takes a path
/// the CLI reads, so only an object is out of reach.
fn type_reason(param_type: &ParameterType) -> Option<String> {
    matches!(param_type, ParameterType::Object)
        .then(|| "-p gives text, numbers or true/false, not an object".to_string())
}

/// A value of `def`'s type that lets `validate` look past its absence.
fn stand_in(def: &ParameterDef) -> serde_json::Value {
    let text = || {
        def.placeholder
            .clone()
            .unwrap_or_else(|| "run-time".to_string())
    };
    match &def.param_type {
        ParameterType::Number => def
            .constraints
            .as_ref()
            .and_then(|c| c.min)
            .unwrap_or(0.0)
            .into(),
        ParameterType::Boolean => false.into(),
        ParameterType::Enum { options } => options
            .first()
            .map_or_else(text, |o| o.value.clone())
            .into(),
        ParameterType::Object => serde_json::Value::Object(Params::new()),
        ParameterType::String | ParameterType::File { .. } => text().into(),
    }
}

#[cfg(test)]
mod tests;
