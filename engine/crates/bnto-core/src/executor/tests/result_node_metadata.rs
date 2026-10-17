use super::*;
use serde_json::{Map, Value, json};

// =========================================================================
// PipelineResult.node_metadata — each node's recorded output metadata, and
// count keys a node declares adding up across its files and iterations.
// =========================================================================

/// Batch node: echoes its files and reports `count` for them, plus its
/// resolved params as `params` when it has any.
struct CountBatch;

impl crate::processor::NodeProcessor for CountBatch {
    fn name(&self) -> &str {
        "count-batch"
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
        let mut metadata = Map::from_iter([("count".to_string(), json!(input.files.len()))]);
        if !input.params.is_empty() {
            metadata.insert("params".to_string(), Value::Object(input.params));
        }
        let files = input
            .files
            .into_iter()
            .map(|f| OutputFile {
                data: f.data,
                filename: f.filename,
                mime_type: "application/octet-stream".to_string(),
                metadata: f.metadata,
            })
            .collect();
        Ok(NodeOutput { files, metadata })
    }
}

/// Per-file node: `hit` is 1 for file `a`, `miss` 1 for any other, both
/// declared as counts; `last` is the file's name and is not a count.
struct HitCounter;

impl crate::processor::NodeProcessor for HitCounter {
    fn name(&self) -> &str {
        "hit-counter"
    }

    fn count_keys(&self) -> &'static [&'static str] {
        &["hit", "miss"]
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let hit = u8::from(input.filename == "a");
        let metadata = json!({ "hit": hit, "miss": 1 - hit, "last": input.filename });
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename,
                mime_type: "application/octet-stream".to_string(),
                metadata: Map::new(),
            }],
            metadata: metadata.as_object().unwrap().clone(),
        })
    }
}

fn registry() -> NodeRegistry {
    let mut registry = mock_registry();
    registry.register("test-count-batch", Box::new(CountBatch));
    registry.register("test-hit-counter", Box::new(HitCounter));
    registry
}

/// Run with files `a`, `b` and `c`.
fn run(def_json: &str) -> PipelineResult {
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    let files = vec![
        make_file("a", b"a"),
        make_file("b", b"b"),
        make_file("c", b"c"),
    ];
    execute_pipeline(&def, files, &registry(), &reporter, &NoopContext, fake_now).unwrap()
}

fn object(value: Value) -> Map<String, Value> {
    value.as_object().unwrap().clone()
}

#[test]
fn test_result_carries_a_batch_nodes_metadata_under_its_id() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "count", "type": "test-count-batch" },
        { "id": "out", "type": "output" }
    ] }"#);
    assert_eq!(result.node_metadata["count"], object(json!({ "count": 3 })));
}

#[test]
fn test_result_has_an_empty_record_for_a_node_that_reported_none() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "echo", "type": "test-echo" }
    ] }"#);
    assert_eq!(result.node_metadata["echo"], Map::new());
}

#[test]
fn test_result_leaves_out_the_input_nodes_seeded_record() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "echo", "type": "test-echo" }
    ] }"#);
    let ids: Vec<&String> = result.node_metadata.keys().collect();
    assert_eq!(ids, vec!["echo"]);
}

/// `hit-counter` over `a`, `b`, `c`: one hit, two misses, `c` last.
fn summed() -> Map<String, Value> {
    object(json!({ "hit": 1, "miss": 2, "last": "c" }))
}

#[test]
fn test_per_file_node_adds_up_its_count_keys_over_its_files() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "hits", "type": "test-hit-counter" }
    ] }"#);
    assert_eq!(result.node_metadata["hits"], summed());
}

#[test]
fn test_auto_iteration_adds_up_count_keys_over_its_per_file_runs() {
    let result = run(r#"{ "settings": { "iteration": "auto" }, "nodes": [
        { "id": "in", "type": "input" },
        { "id": "echo", "type": "test-echo" },
        { "id": "hits", "type": "test-hit-counter" }
    ] }"#);
    assert_eq!(result.node_metadata["hits"], summed());
}

#[test]
fn test_loop_adds_up_a_childs_count_keys_over_its_iterations() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "hits", "type": "test-hit-counter" }
        ] }
    ] }"#);
    assert_eq!(result.node_metadata["hits"], summed());
}

#[test]
fn test_each_node_keeps_its_own_record() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "hits", "type": "test-hit-counter" },
        { "id": "count", "type": "test-count-batch" },
        { "id": "upper", "type": "test-uppercase" }
    ] }"#);
    assert_eq!(result.node_metadata["hits"], summed());
    assert_eq!(result.node_metadata["count"], object(json!({ "count": 3 })));
    assert_eq!(result.node_metadata["upper"], Map::new());
}

#[test]
fn test_later_node_reads_the_summed_count() {
    let result = run(r#"{ "nodes": [
        { "id": "in", "type": "input" },
        { "id": "hits", "type": "test-hit-counter" },
        { "id": "see", "type": "test-count-batch", "parameters": { "misses": "{{node.hits.miss}}" } }
    ] }"#);
    assert_eq!(
        result.node_metadata["see"]["params"],
        json!({ "misses": 2 })
    );
}
