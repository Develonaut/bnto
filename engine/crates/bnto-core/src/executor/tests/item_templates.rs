use super::*;

// =========================================================================
// {{item.*}} Templates — resolved against the metadata of the file a node
// is processing, in flat recipes and inside loops alike.
// =========================================================================

/// Attaches `new_name` = "renamed-<filename>" and `tag` = the `tag` param.
struct TagProcessor;

impl crate::processor::NodeProcessor for TagProcessor {
    fn name(&self) -> &str {
        "tag"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "new_name".to_string(),
            format!("renamed-{}", input.filename).into(),
        );
        if let Some(tag) = input.params.get("tag") {
            metadata.insert("tag".to_string(), tag.clone());
        }
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename,
                mime_type: "application/octet-stream".to_string(),
                metadata,
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Names each file after its `name` param, as resolved by the executor.
struct NameFromParamProcessor;

impl crate::processor::NodeProcessor for NameFromParamProcessor {
    fn name(&self) -> &str {
        "name-from-param"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let name = input.params["name"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: name,
                mime_type: "application/octet-stream".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

fn item_registry() -> NodeRegistry {
    let mut registry = mock_registry();
    registry.register("test-tag", Box::new(TagProcessor));
    registry.register("test-name", Box::new(NameFromParamProcessor));
    registry
}

fn file_with_metadata(name: &str, metadata: serde_json::Value) -> PipelineFile {
    PipelineFile {
        metadata: metadata.as_object().unwrap().clone(),
        ..make_file(name, b"data")
    }
}

fn run(def_json: &str, files: Vec<PipelineFile>) -> Result<PipelineResult, BntoError> {
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    execute_pipeline(
        &def,
        files,
        &item_registry(),
        &reporter,
        &NoopContext,
        fake_now,
    )
}

fn run_error(def_json: &str, files: Vec<PipelineFile>) -> String {
    match run(def_json, files) {
        Ok(result) => panic!(
            "expected an error, got files {:?}",
            result.files.iter().map(|f| &f.name).collect::<Vec<_>>()
        ),
        Err(e) => e.to_string(),
    }
}

fn run_names(def_json: &str, files: Vec<PipelineFile>) -> Vec<String> {
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    let result = execute_pipeline(
        &def,
        files,
        &item_registry(),
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    result.files.into_iter().map(|f| f.name).collect()
}

#[test]
fn test_flat_recipe_resolves_item_from_each_files_metadata() {
    let names = run_names(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "tag", "type": "test-tag" },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.new_name}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a"), make_file("b.txt", b"b")],
    );
    assert_eq!(names, vec!["renamed-a.txt", "renamed-b.txt"]);
}

#[test]
fn test_flat_recipe_reads_metadata_the_input_file_carries() {
    let names = run_names(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.row}}.txt" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![
            file_with_metadata("a.txt", serde_json::json!({ "row": "first" })),
            file_with_metadata("b.txt", serde_json::json!({ "row": "second" })),
        ],
    );
    assert_eq!(names, vec!["first.txt", "second.txt"]);
}

#[test]
fn test_item_in_a_mixed_template_string() {
    let names = run_names(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "tag", "type": "test-tag", "parameters": { "tag": "x" } },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.tag}}_{{item.new_name}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a")],
    );
    assert_eq!(names, vec!["x_renamed-a.txt"]);
}

#[test]
fn test_inside_a_loop_a_node_sees_what_earlier_children_attached() {
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "tag", "type": "test-tag" },
                    { "id": "name", "type": "test-name", "parameters": { "name": "{{item.new_name}}" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a")],
    );
    assert_eq!(names, vec!["renamed-a.txt"]);
}

#[test]
fn test_inside_a_loop_the_loop_item_stays_visible_under_file_metadata() {
    // The loop item (`row`) and what `tag` attached (`new_name`) both resolve.
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "tag", "type": "test-tag" },
                    { "id": "name", "type": "test-name", "parameters": { "name": "{{item.row}}-{{item.new_name}}" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![file_with_metadata(
            "a.txt",
            serde_json::json!({ "row": "r1" }),
        )],
    );
    assert_eq!(names, vec!["r1-renamed-a.txt"]);
}

#[test]
fn test_missing_item_key_fails_naming_the_node_key_and_file() {
    // A name built from a value that isn't there must never be used.
    let err = run_error(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "tag", "type": "test-tag" },
            { "id": "name", "type": "test-name", "parameters": { "name": "x{{item.nope}}.txt" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("Pack/a.txt", b"a")],
    );
    assert!(err.contains("'name'"), "{err}");
    assert!(err.contains("{{item.nope}}"), "{err}");
    assert!(err.contains("Pack/a.txt"), "{err}");
}
#[test]
fn test_item_fails_when_the_file_has_no_metadata() {
    let err = run_error(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.new_name}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a")],
    );
    assert!(
        err.contains("{{item.new_name}}") && err.contains("a.txt"),
        "{err}"
    );
}

#[test]
fn test_one_unfilled_file_fails_the_whole_run() {
    // b.txt's metadata lacks `row`, so the run fails instead of naming it ".txt".
    let err = run_error(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.row}}.txt" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![
            file_with_metadata("a.txt", serde_json::json!({ "row": "first" })),
            file_with_metadata("b.txt", serde_json::json!({ "other": "x" })),
        ],
    );
    assert!(
        err.contains("{{item.row}}") && err.contains("b.txt"),
        "{err}"
    );
}

#[test]
fn test_params_without_item_templates_are_unaffected_by_missing_metadata() {
    let names = run_names(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "name", "type": "test-name", "parameters": { "name": "fixed.txt" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a")],
    );
    assert_eq!(names, vec!["fixed.txt"]);
}
#[test]
fn test_inside_a_loop_the_loop_item_wins_a_shared_key() {
    // A loop over CSV rows keeps reading its row's column even when a child
    // attaches metadata under the same key.
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "tag", "type": "test-tag" },
                    { "id": "name", "type": "test-name", "parameters": { "name": "{{item.new_name}}" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![file_with_metadata(
            "a.txt",
            serde_json::json!({ "new_name": "from-row.txt" }),
        )],
    );
    assert_eq!(names, vec!["from-row.txt"]);
}

#[test]
fn test_top_level_without_auto_iteration_resolves_per_file() {
    let names = run_names(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "tag", "type": "test-tag" },
            { "id": "name", "type": "test-name", "parameters": { "name": "{{item.new_name}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a"), make_file("b.txt", b"b")],
    );
    assert_eq!(names, vec!["renamed-a.txt", "renamed-b.txt"]);
}

#[test]
fn test_item_reads_only_what_the_node_before_attached() {
    // Each node's output metadata replaces the file's metadata, so `new_name`
    // from `tag` is gone after `test-metadata` and the reference fails. Pins
    // today's limit until per-file metadata accumulates across nodes.
    let err = run_error(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "tag", "type": "test-tag" },
            { "id": "meta", "type": "test-metadata" },
            { "id": "name", "type": "test-name", "parameters": { "name": "x{{item.new_name}}" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"aaaa")],
    );
    assert!(err.contains("{{item.new_name}}"), "{err}");
}
