use super::*;
use serde_json::{Value, json};

// =========================================================================
// {{node.<id>.<key>}} — a node's output metadata, read by the nodes after it.
// =========================================================================

/// Batch node: echoes its files and reports `count` and `names` for them.
struct CountProcessor;

impl crate::processor::NodeProcessor for CountProcessor {
    fn name(&self) -> &str {
        "count"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            input_cardinality: crate::metadata::InputCardinality::Batch,
            ..crate::processor::NodeProcessor::metadata(&EchoProcessor)
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        crate::processor::NodeProcessor::process(&EchoProcessor, input, progress, ctx)
    }

    fn process_batch(
        &self,
        input: crate::processor::BatchInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let names: Vec<Value> = input
            .files
            .iter()
            .map(|f| f.filename.clone().into())
            .collect();
        let metadata = json!({ "count": names.len(), "names": names });
        let files = input.files.into_iter().map(batch_file_to_output).collect();
        Ok(NodeOutput {
            files,
            metadata: metadata.as_object().unwrap().clone(),
        })
    }
}

fn batch_file_to_output(f: crate::processor::BatchFile) -> OutputFile {
    OutputFile {
        data: f.data,
        filename: f.filename,
        mime_type: "application/octet-stream".to_string(),
        metadata: f.metadata,
    }
}

/// Per-file node: reports `last` = this file's name and `seen_<name>` = true.
struct PerFileMetaProcessor;

impl crate::processor::NodeProcessor for PerFileMetaProcessor {
    fn name(&self) -> &str {
        "per-file-meta"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let mut metadata = serde_json::Map::new();
        metadata.insert("last".to_string(), input.filename.clone().into());
        metadata.insert(format!("seen_{}", input.filename), true.into());
        Ok(NodeOutput {
            files: vec![passthrough_file(input)],
            metadata,
        })
    }
}

/// Source node: makes one file and reports `made` = "yes".
struct MakeProcessor;

impl crate::processor::NodeProcessor for MakeProcessor {
    fn name(&self) -> &str {
        "make"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            input_cardinality: crate::metadata::InputCardinality::Source,
            ..crate::processor::NodeProcessor::metadata(&EchoProcessor)
        }
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let mut metadata = serde_json::Map::new();
        metadata.insert("made".to_string(), "yes".into());
        Ok(NodeOutput {
            files: vec![passthrough_file(NodeInput {
                data: FileData::Bytes(b"made".to_vec()),
                filename: "made".to_string(),
                mime_type: None,
                params: serde_json::Map::new(),
            })],
            metadata,
        })
    }
}

/// Per-file node: attaches its resolved params to the file as metadata, so a
/// test can read back exactly what each param resolved to, JSON type and all.
struct SeeParamsProcessor;

impl crate::processor::NodeProcessor for SeeParamsProcessor {
    fn name(&self) -> &str {
        "see-params"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = input.params.clone();
        let mut file = passthrough_file(input);
        file.metadata = params;
        Ok(NodeOutput {
            files: vec![file],
            metadata: serde_json::Map::new(),
        })
    }
}

fn passthrough_file(input: NodeInput) -> OutputFile {
    OutputFile {
        data: input.data,
        filename: input.filename,
        mime_type: "application/octet-stream".to_string(),
        metadata: serde_json::Map::new(),
    }
}

fn registry() -> NodeRegistry {
    let mut registry = mock_registry();
    registry.register("test-count", Box::new(CountProcessor));
    registry.register("test-per-file-meta", Box::new(PerFileMetaProcessor));
    registry.register("test-make", Box::new(MakeProcessor));
    registry.register("test-see", Box::new(SeeParamsProcessor));
    registry
}

/// Run with files `a` and `b`; return each output file's metadata.
fn run_metadata(def_json: &str) -> Vec<serde_json::Map<String, Value>> {
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    let files = vec![make_file("a", b"a"), make_file("b", b"b")];
    let result = execute_pipeline(&def, files, &registry(), &reporter, &NoopContext, fake_now);
    result
        .unwrap()
        .files
        .into_iter()
        .map(|f| f.metadata)
        .collect()
}

/// The values of `key` across every output file.
fn seen(def_json: &str, key: &str) -> Vec<Value> {
    run_metadata(def_json)
        .into_iter()
        .map(|m| m.get(key).cloned().unwrap_or(Value::Null))
        .collect()
}

#[test]
fn test_later_node_reads_batch_metadata_as_sole_placeholder() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.count}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!(2), json!(2)]);
}

#[test]
fn test_later_node_interpolates_batch_metadata() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "label": "found {{node.count.count}} files" } }
    ] }"#;
    assert_eq!(seen(def, "label"), vec![json!("found 2 files"); 2]);
}

#[test]
fn test_sole_placeholder_keeps_an_array_an_array() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.names}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!(["a", "b"]); 2]);
}

#[test]
fn test_node_cannot_read_a_later_nodes_metadata() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.count}}" } },
        { "id": "count", "type": "test-count" }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!("{{node.count.count}}"); 2]);
}

#[test]
fn test_key_no_node_set_stays_unresolved() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.nope}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!("{{node.count.nope}}"); 2]);
}

#[test]
fn test_input_node_data_still_resolves_after_processing_nodes() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.in.filename}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!("a"); 2]);
}

#[test]
fn test_later_node_reads_source_metadata() {
    let def = r#"{ "nodes": [
        { "id": "make", "type": "test-make" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.make.made}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!("yes")]);
}

#[test]
fn test_per_file_metadata_merges_with_later_files_winning() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "pf", "type": "test-per-file-meta" },
        { "id": "see", "type": "test-see", "parameters": {
            "last": "{{node.pf.last}}", "a": "{{node.pf.seen_a}}", "b": "{{node.pf.seen_b}}"
        } }
    ] }"#;
    let metadata = run_metadata(def);
    for m in &metadata {
        assert_eq!(m["last"], json!("b"));
        assert_eq!((&m["a"], &m["b"]), (&json!(true), &json!(true)));
    }
}

#[test]
fn test_group_child_reads_metadata_from_before_the_group() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "grp", "type": "group", "nodes": [
            { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.count}}" } }
        ] }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!(2); 2]);
}

#[test]
fn test_group_child_metadata_reaches_later_siblings_and_nodes_after_the_group() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "grp", "type": "group", "nodes": [
            { "id": "count", "type": "test-count" },
            { "id": "inside", "type": "test-see", "parameters": { "inside": "{{node.count.count}}" } }
        ] },
        { "id": "after", "type": "test-see", "parameters": { "after": "{{node.count.count}}" } }
    ] }"#;
    let metadata = run_metadata(def);
    for m in &metadata {
        assert_eq!((&m["inside"], &m["after"]), (&json!(2), &json!(2)));
    }
}

#[test]
fn test_loop_iterations_are_independent_and_merge_after_the_loop() {
    let def = r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "pf", "type": "test-per-file-meta" },
            { "id": "inside", "type": "test-see", "parameters": {
                "own": "{{node.pf.last}}", "other": "{{node.pf.seen_a}}"
            } }
        ] },
        { "id": "after", "type": "test-see", "parameters": {
            "last": "{{node.pf.last}}", "a": "{{node.pf.seen_a}}"
        } }
    ] }"#;
    let metadata = run_metadata(def);
    // Iteration for `b` sees only its own `pf` run, not iteration `a`'s.
    assert_eq!(metadata[1]["own"], json!("b"));
    assert_eq!(metadata[1]["other"], json!("{{node.pf.seen_a}}"));
    // After the loop, every iteration's keys are merged; the last one wins `last`.
    for m in &metadata {
        assert_eq!((&m["last"], &m["a"]), (&json!("b"), &json!(true)));
    }
}

#[test]
fn test_auto_iteration_carries_batch_metadata_into_the_per_file_run() {
    let def = r#"{ "settings": { "iteration": "auto" }, "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count" },
        { "id": "see", "type": "test-see", "parameters": { "value": "{{node.count.count}}" } }
    ] }"#;
    assert_eq!(seen(def, "value"), vec![json!(2); 2]);
}
