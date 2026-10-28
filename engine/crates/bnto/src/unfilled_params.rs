// `bnto run`, `bnto dry-run` and a TUI run stop before consent when a step
// refuses its params as the recipe wrote them: a required param with no
// value, one it doesn't declare, or one `validate_raw` refuses (ISS-74). The
// engine's pre-flight refuses the same run (`raw_param_problems`); checking
// first means nobody approves it and no `-o` folder is made for it.

use std::ptr;

use bnto_core::check::unreachable_reason;
use bnto_core::executor::raw_params::{RawParamProblem, raw_param_problems};
use bnto_core::metadata::{ParameterDef, ParameterType};
use bnto_core::{InputMode, PipelineDefinition, PipelineNode, first_processing_node_id};

use crate::file_param::hint_value;
use crate::fill_with::FillWith;

/// One line per step in `def` (with `-p` applied) that refuses its params,
/// saying how to give a required one the `fill` way. A param the run's input
/// fills (a URL or text recipe's `url` or `text`) isn't refused: the input
/// hasn't been applied yet.
pub fn param_lines(def: &PipelineDefinition, fill: FillWith) -> Vec<String> {
    let registry = bnto_engine::create_registry();
    let from_input = input_param(def);
    raw_param_problems(&def.nodes, &registry)
        .iter()
        .filter(|problem| !filled_by_input(problem, from_input.as_ref()))
        .map(|problem| line(problem, &def.nodes, fill))
        .collect()
}

/// The node and param a URL or text input fills (`input::prepare_inputs`).
fn input_param(def: &PipelineDefinition) -> Option<(String, &'static str)> {
    let key = match bnto_core::resolve_input_mode(def) {
        InputMode::Url => "url",
        InputMode::Text => "text",
        InputMode::FileUpload | InputMode::None => return None,
    };
    first_processing_node_id(def).map(|id| (id, key))
}

fn filled_by_input(problem: &RawParamProblem, from_input: Option<&(String, &str)>) -> bool {
    let missing = problem.missing.as_ref().map(|def| def.name.as_str());
    from_input.is_some_and(|(id, key)| problem.node.id == *id && missing == Some(*key))
}

/// The engine's refusal, with how to give a required param when one can.
fn line(problem: &RawParamProblem, root: &[PipelineNode], fill: FillWith) -> String {
    let Some(def) = &problem.missing else {
        return problem.to_string();
    };
    match fill {
        FillWith::Flag => match unreachable_reason(problem.node, def, root) {
            None => format!("{problem}: pass {}", flag(problem.node, def)),
            Some(reason) => format!("{problem} ({reason})"),
        },
        FillWith::Form if on_form(problem.node, def, root) => {
            format!("{} is empty: fill it in on the recipe's form", label(def))
        }
        FillWith::Form => problem.to_string(),
    }
}

/// `-p node:param=<value>`, or `=<path>` for a file param.
fn flag(node: &PipelineNode, def: &ParameterDef) -> String {
    let takes_file = matches!(def.param_type, ParameterType::File { .. });
    format!("-p {}:{}={}", node.id, def.name, hint_value(takes_file))
}

/// Whether the TUI's recipe form shows `def`: a run option of a top-level
/// step (`tui/screens/detail_loader.rs`).
fn on_form(node: &PipelineNode, def: &ParameterDef, root: &[PipelineNode]) -> bool {
    def.surfaceable && def.run_option && root.iter().any(|top| ptr::eq(top, node))
}

fn label(def: &ParameterDef) -> &str {
    if def.label.is_empty() {
        &def.name
    } else {
        &def.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(json: &str) -> PipelineDefinition {
        serde_json::from_str(json).unwrap()
    }

    /// An overlay step `stamp` with `overlay` set to `value`, inside a group
    /// when `nested`.
    fn overlay(value: &str, nested: bool) -> PipelineDefinition {
        let step = format!(
            r#"{{ "id": "stamp", "type": "image-overlay", "parameters": {{ "overlay": "{value}" }} }}"#
        );
        let nodes = if nested {
            format!(r#"{{ "id": "g", "type": "group", "nodes": [{step}] }}"#)
        } else {
            step
        };
        definition(&format!(r#"{{ "nodes": [{nodes}] }}"#))
    }

    #[test]
    fn a_required_file_param_names_the_p_with_a_path() {
        let lines = param_lines(&overlay("", false), FillWith::Flag);
        assert_eq!(
            lines,
            [
                "Node 'stamp': param \"overlay\" (Overlay Image) is required: \
              pass -p stamp:overlay=<path>"
            ]
        );
    }

    #[test]
    fn a_given_param_passes() {
        assert!(param_lines(&overlay("aGk=", false), FillWith::Flag).is_empty());
    }

    #[test]
    fn a_param_p_cant_reach_says_why_instead() {
        let def = definition(
            r#"{ "nodes": [{ "id": "stamp", "type": "image-overlay",
                "fields": { "overlay": { "type": "string", "label": "Logo", "default": "x" } },
                "parameters": { "overlay": "" } }] }"#,
        );
        let lines = param_lines(&def, FillWith::Flag);
        assert_eq!(lines.len(), 1);
        assert!(
            lines[0].ends_with("(-p stamp:overlay=… sets the node's field of that name instead)"),
            "{}",
            lines[0]
        );
    }

    #[test]
    fn other_refusals_keep_the_engines_words() {
        let def = definition(
            r#"{ "nodes": [{ "id": "stamp", "type": "image-overlay",
                "parameters": { "overlay": "aGk=", "opacty": 1 } }] }"#,
        );
        let lines = param_lines(&def, FillWith::Flag);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("Node 'stamp': "), "{}", lines[0]);
        assert!(!lines[0].contains("-p"), "{}", lines[0]);
    }

    #[test]
    fn a_url_recipes_input_param_is_left_to_the_input() {
        let def = definition(
            r#"{ "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "url" } },
                { "id": "dl", "type": "video-download", "parameters": { "url": "" } }] }"#,
        );
        assert!(param_lines(&def, FillWith::Flag).is_empty());
    }

    // --- The TUI's wording ---

    #[test]
    fn on_the_form_a_top_level_param_names_its_label() {
        let lines = param_lines(&overlay("", false), FillWith::Form);
        assert_eq!(
            lines,
            ["Overlay Image is empty: fill it in on the recipe's form"]
        );
    }

    #[test]
    fn on_the_form_a_nested_param_keeps_the_engines_words() {
        let lines = param_lines(&overlay("", true), FillWith::Form);
        assert_eq!(
            lines,
            ["Node 'stamp': param \"overlay\" (Overlay Image) is required"]
        );
    }
}
