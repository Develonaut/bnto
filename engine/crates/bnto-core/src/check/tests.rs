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

/// Publishes real metadata declaring one param, `quality`.
struct Declared;

impl NodeProcessor for Declared {
    fn name(&self) -> &str {
        "declared"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            node_type: "declared".to_string(),
            name: "Declared".to_string(),
            description: String::new(),
            category: crate::metadata::NodeCategory::Data,
            accepts: vec![],
            platforms: vec![],
            parameters: vec![crate::metadata::ParameterDef {
                name: "quality".to_string(),
                ..Default::default()
            }],
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

fn registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry.register("lenient", Box::new(Lenient));
    registry.register("strict", Box::new(Strict));
    registry.register("declared", Box::new(Declared));
    registry
}

// --- Undeclared params and renamed types ---

#[test]
fn test_a_param_the_node_does_not_declare_is_a_finding_on_that_param() {
    let findings = check(json!({ "nodes": [{
        "id": "step", "type": "declared", "parameters": { "quality": 80, "qualty": 70 }
    }] }));

    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].param.as_deref(), Some("qualty"));
    assert_eq!(findings[0].message, "declared has no param \"qualty\"");
}

#[test]
fn test_declared_params_alone_pass() {
    let findings = check(json!({ "nodes": [{
        "id": "step", "type": "declared", "parameters": { "quality": 80 }
    }] }));
    assert!(findings.is_empty(), "{findings:?}");
}

#[test]
fn test_a_processor_with_placeholder_metadata_takes_any_param() {
    assert!(check(one_node(json!({ "anything": 1 }))).is_empty());
}

#[test]
fn test_an_undeclared_param_inside_a_container_is_found() {
    let findings = check(json!({ "nodes": [{
        "id": "loop", "type": "loop", "parameters": { "mode": "forEach" },
        "nodes": [{ "id": "step", "type": "declared", "parameters": { "extra": 1 } }]
    }] }));
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].node_id, "step");
}

#[test]
fn test_a_renamed_node_type_points_at_bnto_migrate() {
    let findings = check(json!({ "nodes": [{ "id": "s", "type": "shell-command" }] }));
    assert_eq!(
        findings[0].message,
        "unknown node type \"shell-command\" \
         (renamed to \"shell-run\"; run `bnto migrate` on the recipe)"
    );
}

fn check(def: serde_json::Value) -> Vec<CheckFinding> {
    let definition: PipelineDefinition = serde_json::from_value(def).unwrap();
    check_pipeline(&definition, &registry()).findings
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

// --- Edges ---

/// A root level `input`, `lenient` nodes `middle`, `output`, wired by
/// `edges` as `[id, source, target]` triples.
fn wired(middle: &[&str], edges: &[(&str, &str, &str)]) -> serde_json::Value {
    let mut nodes = vec![json!({ "id": "input", "type": "input" })];
    nodes.extend(
        middle
            .iter()
            .map(|id| json!({ "id": id, "type": "lenient" })),
    );
    nodes.push(json!({ "id": "output", "type": "output" }));
    json!({ "nodes": nodes, "edges": edge_list(edges) })
}

fn edge_list(edges: &[(&str, &str, &str)]) -> Vec<serde_json::Value> {
    edges
        .iter()
        .map(|(id, s, t)| json!({ "id": id, "source": s, "target": t }))
        .collect()
}

fn lines(findings: &[CheckFinding]) -> Vec<String> {
    findings.iter().map(ToString::to_string).collect()
}

#[test]
fn test_a_valid_edge_graph_passes() {
    let findings = check(wired(
        &["a", "b"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "b", "output"),
        ],
    ));
    assert_eq!(findings, Vec::new());
}

#[test]
fn test_a_cycle_is_one_finding_naming_its_path() {
    let findings = check(wired(
        &["a", "b"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "b", "a"),
            ("e4", "b", "output"),
        ],
    ));
    assert_eq!(
        lines(&findings),
        [r#"node "a" (lenient): edges form a cycle: a → b → a"#]
    );
}

#[test]
fn test_an_edge_to_an_unknown_node_is_reported_on_its_known_end() {
    let findings = check(wired(
        &["a"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "zz"),
            ("e3", "a", "output"),
        ],
    ));
    assert_eq!(
        lines(&findings),
        [r#"node "a" (lenient): edge "e2" connects to "zz", which is not a node in this recipe"#]
    );
}

#[test]
fn test_a_duplicate_edge_is_reported_on_its_source() {
    let findings = check(wired(
        &["a", "b"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "b"),
            ("e3", "a", "b"),
            ("e4", "b", "output"),
        ],
    ));
    assert_eq!(
        lines(&findings),
        [r#"node "a" (lenient): edge "e3" repeats an earlier edge between the same two nodes"#]
    );
}

#[test]
fn test_an_edge_into_input_is_reported_on_its_source() {
    let findings = check(wired(
        &["a"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "output"),
            ("e3", "a", "input"),
        ],
    ));
    assert_eq!(
        lines(&findings),
        [r#"node "a" (lenient): edge "e3" leads into an input node"#]
    );
}

#[test]
fn test_an_edge_out_of_output_is_reported_on_its_target() {
    let findings = check(wired(
        &["a"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "output"),
            ("e3", "output", "a"),
        ],
    ));
    assert_eq!(
        lines(&findings),
        [r#"node "a" (lenient): edge "e3" leads out of an output node"#]
    );
}

#[test]
fn test_a_disconnected_node_is_one_finding_on_it() {
    let findings = check(wired(
        &["a", "b"],
        &[("e1", "input", "a"), ("e2", "a", "output")],
    ));
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].node_id, "b");
    assert_eq!(findings[0].node_type, "lenient");
    assert!(findings[0].message.contains("no edge"), "{findings:?}");
}

#[test]
fn test_every_edge_problem_at_a_level_is_reported() {
    let findings = check(wired(
        &["a", "b", "c", "d"],
        &[
            ("e1", "input", "a"),
            ("e2", "a", "zz"),
            ("e3", "a", "output"),
            ("e4", "a", "output"),
            ("e5", "c", "d"),
            ("e6", "d", "c"),
        ],
    ));
    let at: Vec<_> = findings.iter().map(|f| f.node_id.as_str()).collect();
    assert_eq!(at, ["a", "a", "b", "c"], "{findings:?}");
    for edge in ["\"e2\"", "\"e4\""] {
        assert!(
            findings.iter().any(|f| f.message.contains(edge)),
            "{findings:?}"
        );
    }
    assert!(findings[3].message.contains("c → d → c"), "{findings:?}");
}

#[test]
fn test_a_root_edge_into_a_container_child_names_the_container() {
    let findings = check(json!({
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "lenient" },
            { "id": "g", "type": "group", "nodes": [{ "id": "c", "type": "lenient" }] },
            { "id": "output", "type": "output" }
        ],
        "edges": edge_list(&[
            ("e1", "input", "a"), ("e2", "a", "g"), ("e3", "a", "c"), ("e4", "g", "output")
        ])
    }));
    assert_eq!(
        lines(&findings),
        [concat!(
            r#"node "a" (lenient): edge "e3" connects to "c", which is inside container "g"; "#,
            "edges only connect nodes at the same level"
        )]
    );
}

#[test]
fn test_a_container_edge_to_a_top_level_node_says_so() {
    let findings = check(json!({
        "nodes": [
            { "id": "a", "type": "lenient" },
            { "id": "g", "type": "loop",
              "nodes": [{ "id": "c", "type": "lenient" }, { "id": "d", "type": "lenient" }],
              "edges": edge_list(&[("ge1", "c", "d"), ("ge2", "d", "a")]) }
        ]
    }));
    assert_eq!(
        lines(&findings),
        [concat!(
            r#"node "d" (lenient): edge "ge2" connects to "a", which is a top-level node; "#,
            "edges only connect nodes at the same level"
        )]
    );
}

#[test]
fn test_edge_problems_inside_a_container_are_reported() {
    let findings = check(json!({
        "nodes": [{ "id": "g", "type": "group",
            "nodes": [{ "id": "c", "type": "lenient" }, { "id": "d", "type": "lenient" }],
            "edges": edge_list(&[("ge1", "c", "d"), ("ge2", "d", "c")]) }]
    }));
    assert_eq!(
        lines(&findings),
        [r#"node "c" (lenient): edges form a cycle: c → d → c"#]
    );
}

#[test]
fn test_an_edge_with_no_end_in_a_container_is_reported_on_the_container() {
    let findings = check(json!({
        "nodes": [{ "id": "g", "type": "group",
            "nodes": [{ "id": "c", "type": "lenient" }],
            "edges": edge_list(&[("ge1", "ghost", "phantom"), ("ge2", "c", "c2")]) }]
    }));
    let at: Vec<_> = findings.iter().map(|f| f.node_id.as_str()).collect();
    assert_eq!(at, ["g", "c"], "{findings:?}");
    assert!(findings[0].message.contains("\"ge1\""), "{findings:?}");
}

#[test]
fn test_an_edge_with_no_end_at_the_root_is_reported_on_the_missing_id() {
    let findings = check(json!({
        "nodes": [{ "id": "a", "type": "lenient" }],
        "edges": edge_list(&[("e1", "ghost", "phantom"), ("e2", "a", "a2")])
    }));
    assert_eq!(findings.len(), 2, "{findings:?}");
    assert_eq!(findings[0].node_id, "ghost");
    assert!(findings[0].message.contains("\"e1\""), "{findings:?}");
}
