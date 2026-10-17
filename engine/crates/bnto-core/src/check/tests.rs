use super::*;
use crate::context::ProcessContext;
use crate::errors::BntoError;
use crate::processor::{NodeInput, NodeOutput, NodeProcessor};
use crate::progress::ProgressReporter;
use serde_json::json;

/// Accepts any params.
struct Lenient;

/// Refuses a `mode` other than "a" or "b".
struct Strict;

impl NodeProcessor for Lenient {
    fn name(&self) -> &str {
        "lenient"
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        unreachable!("check never runs a node")
    }
}

impl NodeProcessor for Strict {
    fn name(&self) -> &str {
        "strict"
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        match params.get("mode").and_then(|v| v.as_str()) {
            None | Some("a") | Some("b") => Vec::new(),
            Some(other) => vec![format!("mode must be \"a\" or \"b\", got \"{other}\"")],
        }
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        unreachable!("check never runs a node")
    }
}

fn registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry.register("lenient", Box::new(Lenient));
    registry.register("strict", Box::new(Strict));
    registry
}

fn check(def: serde_json::Value) -> Vec<CheckFinding> {
    let definition: PipelineDefinition = serde_json::from_value(def).unwrap();
    check_pipeline(&definition, &registry())
}

/// A recipe with one `lenient` node whose params are `params`.
fn one_node(params: serde_json::Value) -> serde_json::Value {
    json!({ "nodes": [{ "id": "step", "type": "lenient", "parameters": params }] })
}

// --- Valid recipes ---

#[test]
fn test_valid_recipe_with_io_nodes_and_a_loop_passes() {
    let findings = check(json!({ "nodes": [
        { "id": "in", "type": "input", "parameters": { "mode": "file-upload" } },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "a", "type": "lenient" },
            { "id": "b", "type": "strict", "parameters": { "mode": "a" } }
        ] },
        { "id": "out", "type": "output", "parameters": { "mode": "none" } }
    ] }));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_every_known_namespace_passes_without_values() {
    let findings = check(one_node(json!({
        "a": "{{fields.x}}", "b": "{{env.HOME}}", "c": "{{ctx.date}}",
        "d": "{{node.prev.name}}", "e": "{{item.new_name}}", "f": "{{vars.Out}}/Art",
        "g": "{{entry.status}}: {{entry}}"
    })));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_placeholder_without_a_namespace_passes() {
    // Node-local templates such as file-rename's `{{name}}.{{ext}}`.
    let findings = check(one_node(
        json!({ "pattern": "{{name}}-{{counter}}.{{ext}}" }),
    ));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_validate_sees_field_defaults_not_the_placeholder() {
    let findings = check(json!({ "nodes": [{
        "id": "s", "type": "strict",
        "parameters": { "mode": "{{fields.mode}}" },
        "fields": { "mode": { "type": "enum", "label": "Mode", "options": [], "default": "b" } }
    }] }));
    assert_eq!(findings, Vec::new());
}

// --- Unknown node types ---

#[test]
fn test_unknown_node_type_fails_naming_it() {
    let findings = check(json!({ "nodes": [{ "id": "zap", "type": "image-zap" }] }));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].node_id, "zap");
    assert_eq!(findings[0].node_type, "image-zap");
    assert!(findings[0].message.contains("image-zap"), "{findings:?}");
}

#[test]
fn test_container_types_are_known() {
    let findings = check(json!({ "nodes": [
        { "id": "g", "type": "group", "nodes": [] },
        { "id": "p", "type": "parallel", "nodes": [{ "id": "a", "type": "lenient" }] }
    ] }));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_nested_container_children_are_checked() {
    let findings = check(
        json!({ "nodes": [{ "id": "outer", "type": "loop", "nodes": [
        { "id": "inner", "type": "group", "nodes": [{ "id": "deep", "type": "nope" }] }
    ] }] }),
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].node_id, "deep");
}

// --- validate() ---

#[test]
fn test_validate_message_fails_naming_the_node() {
    let findings = check(json!({ "nodes": [
        { "id": "s", "type": "strict", "parameters": { "mode": "c" } }
    ] }));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].node_id, "s");
    assert_eq!(findings[0].node_type, "strict");
    assert!(findings[0].message.contains("mode"), "{findings:?}");
}

#[test]
fn test_io_nodes_are_not_validated() {
    let findings = check(
        json!({ "nodes": [{ "id": "out", "type": "output", "parameters": { "mode": "{{zzz.x" } }] }),
    );
    assert_eq!(findings, Vec::new());
}

// --- Placeholders ---

#[test]
fn test_unclosed_placeholder_fails_naming_the_param() {
    let findings = check(one_node(json!({ "destination": "{{vars.X/Art" })));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].param.as_deref(), Some("destination"));
    assert!(findings[0].message.contains("{{vars.X"), "{findings:?}");
}

#[test]
fn test_placeholder_opened_inside_another_is_unclosed() {
    let findings = check(one_node(json!({ "p": "{{vars.A {{vars.B}}" })));
    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("{{vars.A"), "{findings:?}");
}

#[test]
fn test_unknown_namespace_fails_naming_it() {
    let findings = check(one_node(json!({ "p": "x/{{varz.Out}}" })));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].param.as_deref(), Some("p"));
    assert!(findings[0].message.contains("varz"), "{findings:?}");
}

#[test]
fn test_placeholders_in_arrays_and_objects_are_checked() {
    let findings = check(one_node(json!({
        "args": ["ok", "{{bad.x}}"],
        "opts": { "deep": ["{{vars.Y"] }
    })));
    let params: Vec<_> = findings.iter().map(|f| f.param.as_deref()).collect();
    assert_eq!(params, vec![Some("args"), Some("opts")]);
}

#[test]
fn test_unknown_type_still_has_its_placeholders_checked() {
    let findings = check(json!({ "nodes": [
        { "id": "z", "type": "nope", "parameters": { "p": "{{vars.X" } }
    ] }));
    assert_eq!(findings.len(), 2);
}

#[test]
fn test_container_params_are_checked_for_placeholders() {
    let findings = check(json!({ "nodes": [
        { "id": "each", "type": "loop", "parameters": { "label": "{{oops.x}}" }, "nodes": [] }
    ] }));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].node_id, "each");
}

// --- Display ---

#[test]
fn test_finding_displays_node_type_and_param() {
    let finding = CheckFinding {
        node_id: "copy".into(),
        node_type: "file-copy".into(),
        param: Some("destination".into()),
        message: "unclosed placeholder".into(),
    };
    assert_eq!(
        finding.to_string(),
        "node \"copy\" (file-copy), param \"destination\": unclosed placeholder"
    );
}

#[test]
fn test_finding_without_param_displays_node_only() {
    let finding = CheckFinding {
        node_id: "z".into(),
        node_type: "nope".into(),
        param: None,
        message: "unknown node type \"nope\"".into(),
    };
    assert_eq!(
        finding.to_string(),
        "node \"z\" (nope): unknown node type \"nope\""
    );
}

// --- always ---

#[test]
fn test_always_on_a_processing_node_passes() {
    let findings = check(json!({ "nodes": [
        { "id": "a", "type": "lenient" },
        { "id": "b", "type": "lenient", "always": true },
        { "id": "g", "type": "group", "always": true, "nodes": [{ "id": "c", "type": "lenient" }] }
    ] }));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_always_on_an_io_node_is_reported() {
    let findings = check(json!({ "nodes": [
        { "id": "a", "type": "lenient" },
        { "id": "out", "type": "output", "always": true }
    ] }));
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].node_id, "out");
    assert_eq!(findings[0].param, None);
    assert!(findings[0].message.contains("always"), "{findings:?}");
}
