use crate::check::{CheckReport, RunTimeParam, check_pipeline};
use crate::context::ProcessContext;
use crate::errors::BntoError;
use crate::metadata::{Constraints, NodeCategory, NodeMetadata, ParameterDef, ParameterType};
use crate::pipeline::PipelineDefinition;
use crate::processor::{NodeInput, NodeOutput, NodeProcessor};
use crate::progress::ProgressReporter;
use crate::registry::NodeRegistry;
use serde_json::{Value, json};

/// Requires a text `base` and an object `values`; refuses a `mode` other
/// than "a".
struct Needy;

impl NodeProcessor for Needy {
    fn name(&self) -> &str {
        "needy"
    }

    fn validate(&self, params: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut errors = Vec::new();
        if params
            .get("base")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
        {
            errors.push("'base' is required".to_string());
        }
        if !params.get("values").is_some_and(Value::is_object) {
            errors.push("'values' is required".to_string());
        }
        if let Some(mode) = params.get("mode").and_then(Value::as_str)
            && mode != "a"
        {
            errors.push(format!("mode must be \"a\", got \"{mode}\""));
        }
        errors
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "needy".to_string(),
            name: "Needy".to_string(),
            description: String::new(),
            category: NodeCategory::Data,
            accepts: vec![],
            platforms: vec![],
            parameters: vec![
                required("base", ParameterType::String),
                required("values", ParameterType::Object),
                ParameterDef {
                    name: "mode".to_string(),
                    ..Default::default()
                },
            ],
            input_cardinality: Default::default(),
            requires: vec![],
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

/// Requires a text `path` in its metadata, but its `validate` never refuses.
struct Lax;

impl NodeProcessor for Lax {
    fn name(&self) -> &str {
        "lax"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "lax".to_string(),
            parameters: vec![required("path", ParameterType::String)],
            ..Needy.metadata()
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

fn required(name: &str, param_type: ParameterType) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        param_type,
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn check(def: Value) -> CheckReport {
    let definition: PipelineDefinition = serde_json::from_value(def).unwrap();
    let mut registry = NodeRegistry::new();
    registry.register("needy", Box::new(Needy));
    registry.register("lax", Box::new(Lax));
    check_pipeline(&definition, &registry)
}

/// A `needy` node with `values` set, so only `base` is in question.
fn needy(id: &str, mut params: Value) -> Value {
    params["values"] = json!({ "k": "v" });
    json!({ "id": id, "type": "needy", "parameters": params })
}

fn note(node_id: &str, param: &str) -> RunTimeParam {
    RunTimeParam {
        node_id: node_id.to_string(),
        node_type: "needy".to_string(),
        param: param.to_string(),
        takes_file: false,
    }
}

// --- Left for run time ---

#[test]
fn test_a_left_out_text_param_is_left_for_run_time() {
    let report = check(json!({ "nodes": [needy("clean", json!({}))] }));
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.run_time, vec![note("clean", "base")]);
}

#[test]
fn test_other_problems_on_the_node_still_fail() {
    let report = check(json!({ "nodes": [needy("clean", json!({ "mode": "z" }))] }));
    assert_eq!(report.findings.len(), 1, "{:?}", report.findings);
    assert!(report.findings[0].message.contains("mode must be"));
    assert_eq!(report.run_time, vec![note("clean", "base")]);
}

#[test]
fn test_a_param_inside_a_loop_is_left_for_run_time() {
    // `-p` finds a node at any depth, so a container doesn't hide one.
    let report = check(json!({ "nodes": [
        { "id": "each", "type": "loop", "nodes": [
            { "id": "group", "type": "group", "nodes": [needy("clean", json!({}))] }
        ] }
    ] }));
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.run_time, vec![note("clean", "base")]);
}

#[test]
fn test_a_sound_recipe_leaves_nothing_for_run_time() {
    let report = check(json!({ "nodes": [needy("clean", json!({ "base": "." }))] }));
    assert_eq!(report, CheckReport::default());
}

// --- Required in metadata, whatever `validate` says ---
// A run's pre-flight refuses a required param with no value, so check
// reports one even when the node's `validate` lets it through.

#[test]
fn test_a_required_param_validate_allows_left_out_is_left_for_run_time() {
    let report = check(json!({ "nodes": [
        { "id": "save", "type": "lax", "parameters": {} }
    ] }));
    assert_eq!(report.findings, Vec::new());
    let mut expected = note("save", "path");
    expected.node_type = "lax".to_string();
    assert_eq!(report.run_time, vec![expected]);
}

#[test]
fn test_a_required_param_validate_allows_left_empty_is_left_for_run_time() {
    let report = check(json!({ "nodes": [
        { "id": "save", "type": "lax", "parameters": { "path": "" } }
    ] }));
    assert_eq!(report.findings, Vec::new());
    let mut expected = note("save", "path");
    expected.node_type = "lax".to_string();
    assert_eq!(report.run_time, vec![expected]);
}

#[test]
fn test_a_blank_required_param_no_run_can_give_fails_naming_it() {
    let report = check(json!({ "nodes": [
        { "id": "a:b", "type": "lax", "parameters": { "path": "" } }
    ] }));
    let messages: Vec<&str> = report.findings.iter().map(|f| f.message.as_str()).collect();
    assert_eq!(messages.len(), 2, "{messages:?}");
    assert_eq!(messages[0], "param \"path\" is required");
    assert!(messages[1].contains("no run can give it"), "{messages:?}");
    assert_eq!(report.findings[1].param.as_deref(), Some("path"));
}

// --- Empty values ---
// A required param given blank text is left out, as the run's pre-flight
// treats it; the web catalog needs the key, so a bundled recipe writes `""`.

#[test]
fn test_an_empty_required_value_is_left_for_run_time() {
    let report = check(json!({ "nodes": [needy("clean", json!({ "base": "" }))] }));
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.run_time, vec![note("clean", "base")]);
}

#[test]
fn test_an_empty_optional_value_is_not_left_out() {
    let report = check(json!({ "nodes": [needy("clean", json!({ "base": ".", "mode": "" }))] }));
    assert_eq!(report.findings.len(), 1, "{:?}", report.findings);
    assert!(report.findings[0].message.contains("mode must be"));
    assert_eq!(report.run_time, Vec::new());
}

// --- No run can give it ---

#[test]
fn test_a_left_out_object_param_fails() {
    let report = check(json!({ "nodes": [
        { "id": "set", "type": "needy", "parameters": { "base": "." } }
    ] }));
    let messages: Vec<&str> = report.findings.iter().map(|f| f.message.as_str()).collect();
    assert_eq!(messages.len(), 2, "{messages:?}");
    assert_eq!(messages[0], "'values' is required");
    assert!(messages[1].contains("no run can give it"), "{messages:?}");
    assert!(messages[1].contains("object"), "{messages:?}");
    assert_eq!(report.findings[1].param.as_deref(), Some("values"));
    assert_eq!(report.run_time, Vec::new());
}

#[test]
fn test_a_node_id_with_a_colon_fails() {
    let report = check(json!({ "nodes": [needy("clean:up", json!({}))] }));
    assert_eq!(report.findings.len(), 2, "{:?}", report.findings);
    assert_eq!(report.findings[0].message, "'base' is required");
    assert!(
        report.findings[1].message.contains("':'"),
        "{:?}",
        report.findings
    );
    assert_eq!(report.run_time, Vec::new());
}

#[test]
fn test_a_node_id_with_an_equals_sign_fails() {
    let report = check(json!({ "nodes": [needy("a=b", json!({}))] }));
    assert_eq!(report.findings.len(), 2, "{:?}", report.findings);
    assert_eq!(report.run_time, Vec::new());
}

#[test]
fn test_a_node_behind_an_earlier_one_with_its_id_fails() {
    // `-p clean:base=…` sets the first "clean" it finds: the one in the loop.
    let report = check(json!({ "nodes": [
        { "id": "each", "type": "loop", "nodes": [needy("clean", json!({}))] },
        needy("clean", json!({}))
    ] }));
    assert_eq!(report.run_time, vec![note("clean", "base")]);
    assert_eq!(report.findings.len(), 2, "{:?}", report.findings);
    assert_eq!(report.findings[0].message, "'base' is required");
    assert!(
        report.findings[1].message.contains("earlier node"),
        "{:?}",
        report.findings
    );
}

#[test]
fn test_a_param_named_like_a_declared_field_fails() {
    // `-p clean:base=…` sets the field's default; the param stays unset.
    let mut node = needy("clean", json!({}));
    node["fields"] = json!({ "base": { "type": "string", "label": "Base" } });
    let report = check(json!({ "nodes": [node] }));
    assert_eq!(report.findings.len(), 2, "{:?}", report.findings);
    assert!(
        report.findings[1].message.contains("field"),
        "{:?}",
        report.findings
    );
    assert_eq!(report.run_time, Vec::new());
}

#[test]
fn test_run_time_param_reads_as_node_and_param() {
    assert_eq!(
        note("clean", "base").to_string(),
        "node \"clean\" (needy), param \"base\": left for run time"
    );
}
