use super::*;

// =========================================================================
// Edges — a level with edges runs in topological order, files routed along
// its edges (fresh starts, fan-in in edge order, fan-out, level output).
// =========================================================================

/// Prefixes each file's name with its `prefix` param.
struct PrefixProcessor;

impl crate::processor::NodeProcessor for PrefixProcessor {
    fn name(&self) -> &str {
        "prefix"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let prefix = input.params["prefix"].as_str().unwrap_or_default();
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: format!("{prefix}{}", input.filename),
                mime_type: "application/octet-stream".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// A batch node that echoes its files and records how many it received.
struct CaptureProcessor;

impl crate::processor::NodeProcessor for CaptureProcessor {
    fn name(&self) -> &str {
        "capture"
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
        let received = input.files.len();
        let files = input
            .files
            .into_iter()
            .map(|f| OutputFile {
                data: f.data,
                filename: f.filename,
                mime_type: "application/octet-stream".to_string(),
                metadata: serde_json::Map::new(),
            })
            .collect();
        let mut metadata = serde_json::Map::new();
        metadata.insert("received".to_string(), received.into());
        Ok(NodeOutput { files, metadata })
    }
}

fn edge_registry() -> NodeRegistry {
    let mut registry = mock_registry();
    registry.register("test-prefix", Box::new(PrefixProcessor));
    registry.register("test-capture", Box::new(CaptureProcessor));
    registry
}

fn run_with(
    json: &str,
    files: Vec<PipelineFile>,
    reporter: &PipelineReporter,
) -> Result<PipelineResult, BntoError> {
    let def = parse_def(json);
    let registry = edge_registry();
    execute_pipeline(&def, files, &registry, reporter, &NoopContext, fake_now)
}

fn run(json: &str, files: Vec<PipelineFile>) -> PipelineResult {
    run_with(json, files, &PipelineReporter::new_noop()).expect("pipeline should succeed")
}

fn names(result: &PipelineResult) -> Vec<&str> {
    result.files.iter().map(|f| f.name.as_str()).collect()
}

fn started_nodes(events: &[PipelineEvent]) -> Vec<(String, usize)> {
    events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeStarted {
                node_id,
                node_index,
                parent_node_id: None,
                ..
            } => Some((node_id.clone(), *node_index)),
            _ => None,
        })
        .collect()
}

#[test]
fn test_a_node_with_no_incoming_edge_starts_with_no_files() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "emit", "type": "test-echo" },
            { "id": "capture", "type": "test-capture" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "emit" },
            { "id": "e2", "source": "emit", "target": "output" },
            { "id": "e3", "source": "capture", "target": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"a")],
    );
    assert_eq!(result.node_metadata["capture"]["received"], 0);
    assert_eq!(names(&result), vec!["a.txt"]);
}

const FAN_IN: &str = r#"{
    "settings": { "iteration": "ITERATION" },
    "nodes": [
        { "id": "input", "type": "input" },
        { "id": "a", "type": "test-prefix", "parameters": { "prefix": "A-" } },
        { "id": "b", "type": "test-prefix", "parameters": { "prefix": "B-" } },
        { "id": "capture", "type": "test-capture" },
        { "id": "output", "type": "output" }
    ],
    "edges": [
        { "id": "e1", "source": "input", "target": "a" },
        { "id": "e2", "source": "input", "target": "b" },
        { "id": "e3", "source": "b", "target": "capture" },
        { "id": "e4", "source": "a", "target": "capture" },
        { "id": "e5", "source": "capture", "target": "output" }
    ]
}"#;

#[test]
fn test_fan_in_follows_edge_order_not_node_order() {
    let json = FAN_IN.replace("ITERATION", "explicit");
    let result = run(&json, vec![make_file("x.txt", b"x")]);
    assert_eq!(names(&result), vec!["B-x.txt", "A-x.txt"]);
    assert_eq!(result.node_metadata["capture"]["received"], 2);
}

#[test]
fn test_fan_in_follows_edge_order_with_auto_iteration() {
    let json = FAN_IN.replace("ITERATION", "auto");
    let result = run(
        &json,
        vec![make_file("x.txt", b"x"), make_file("y.txt", b"y")],
    );
    assert_eq!(
        names(&result),
        vec!["B-x.txt", "B-y.txt", "A-x.txt", "A-y.txt"]
    );
}

#[test]
fn test_fan_out_gives_every_consumer_the_same_files() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "test-echo" },
            { "id": "b", "type": "test-prefix", "parameters": { "prefix": "B-" } },
            { "id": "c", "type": "test-prefix", "parameters": { "prefix": "C-" } },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "b" },
            { "id": "e3", "source": "a", "target": "c" },
            { "id": "e4", "source": "b", "target": "output" },
            { "id": "e5", "source": "c", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"hello")],
    );
    assert_eq!(names(&result), vec!["B-x.txt", "C-x.txt"]);
    assert_eq!(file_data_bytes(&result.files[0].data), b"hello");
    assert_eq!(file_data_bytes(&result.files[1].data), b"hello");
}

#[test]
fn test_fan_out_of_a_path_file_shares_the_path() {
    let path = std::path::PathBuf::from("shared/on-disk.bin");
    let file = PipelineFile {
        data: FileData::Path(path.clone()),
        ..make_file("on-disk.bin", b"")
    };
    let result = run(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "b", "type": "test-echo" },
            { "id": "c", "type": "test-echo" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "b" },
            { "id": "e2", "source": "input", "target": "c" },
            { "id": "e3", "source": "b", "target": "output" },
            { "id": "e4", "source": "c", "target": "output" }
        ]
    }"#,
        vec![file],
    );
    assert_eq!(result.files.len(), 2);
    for f in &result.files {
        assert_eq!(f.data, FileData::Path(path.clone()));
    }
}

#[test]
fn test_nodes_run_and_report_in_execution_order() {
    let recorder = RecordingReporter::new();
    run_with(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "b", "type": "test-prefix", "parameters": { "prefix": "B-" } },
            { "id": "a", "type": "test-prefix", "parameters": { "prefix": "A-" } },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "b" },
            { "id": "e3", "source": "b", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap();
    let events = recorder.events();
    let PipelineEvent::PipelineStarted { nodes, .. } = &events[0] else {
        panic!("first event should be PipelineStarted, got {:?}", events[0]);
    };
    let ids: Vec<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(ids, vec!["a", "b"]);
    let started = started_nodes(&events);
    assert_eq!(started, vec![("a".to_string(), 0), ("b".to_string(), 1)]);
}

#[test]
fn test_root_output_is_edges_into_output_in_edge_order() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "test-prefix", "parameters": { "prefix": "A-" } },
            { "id": "b", "type": "test-prefix", "parameters": { "prefix": "B-" } },
            { "id": "sink", "type": "test-metadata" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "input", "target": "b" },
            { "id": "e3", "source": "input", "target": "sink" },
            { "id": "e4", "source": "b", "target": "output" },
            { "id": "e5", "source": "a", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"xxxx")],
    );
    assert_eq!(names(&result), vec!["B-x.txt", "A-x.txt"]);
    assert_eq!(result.node_metadata["sink"]["originalSize"], 4);
}

#[test]
fn test_a_node_reads_an_earlier_non_ancestor_record() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "x", "type": "test-metadata" },
            { "id": "y", "type": "test-prefix",
              "parameters": { "prefix": "{{node.x.originalSize}}-" } },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "x" },
            { "id": "e2", "source": "input", "target": "y" },
            { "id": "e3", "source": "x", "target": "output" },
            { "id": "e4", "source": "y", "target": "output" }
        ]
    }"#,
        vec![make_file("a.txt", b"abcd")],
    );
    assert_eq!(names(&result), vec!["a.txt", "4-a.txt"]);
    assert_eq!(file_data_bytes(&result.files[1].data), b"abcd");
}

#[test]
fn test_a_cycle_fails_before_the_run_starts() {
    let recorder = RecordingReporter::new();
    let error = run_with(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "test-echo" },
            { "id": "b", "type": "test-echo" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "b" },
            { "id": "e3", "source": "b", "target": "a" },
            { "id": "e4", "source": "b", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap_err();
    assert!(matches!(error, BntoError::InvalidInput(_)), "{error:?}");
    assert!(error.to_string().contains("a → b → a"), "{error}");
    assert!(recorder.events().is_empty(), "{:?}", recorder.events());
}

#[test]
fn test_a_cycle_inside_a_container_fails_before_the_run_starts() {
    let recorder = RecordingReporter::new();
    let error = run_with(
        r#"{
        "nodes": [
            { "id": "g", "type": "group",
              "nodes": [
                  { "id": "a", "type": "test-echo" },
                  { "id": "b", "type": "test-echo" }
              ],
              "edges": [
                  { "id": "c1", "source": "a", "target": "b" },
                  { "id": "c2", "source": "b", "target": "a" }
              ] }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap_err();
    assert!(matches!(error, BntoError::InvalidInput(_)), "{error:?}");
    assert!(error.to_string().contains("'g'"), "{error}");
    assert!(recorder.events().is_empty(), "{:?}", recorder.events());
}

#[test]
fn test_a_disconnected_node_fails_before_the_run_starts() {
    let recorder = RecordingReporter::new();
    let error = run_with(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "a", "type": "test-echo" },
            { "id": "lost", "type": "test-echo" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "a" },
            { "id": "e2", "source": "a", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("\"lost\""), "{error}");
    assert!(recorder.events().is_empty());
}

// --- Container levels ---

#[test]
fn test_a_loop_child_that_is_not_the_first_root_gets_nothing() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "each", "type": "loop",
              "nodes": [
                  { "id": "first", "type": "test-prefix", "parameters": { "prefix": "F-" } },
                  { "id": "lonely", "type": "test-capture" },
                  { "id": "join", "type": "test-capture" }
              ],
              "edges": [
                  { "id": "c1", "source": "first", "target": "join" },
                  { "id": "c2", "source": "lonely", "target": "join" }
              ] }
        ]
    }"#,
        vec![make_file("x.txt", b"x"), make_file("y.txt", b"y")],
    );
    assert_eq!(names(&result), vec!["F-x.txt", "F-y.txt"]);
    assert_eq!(result.node_metadata["lonely"]["received"], 0);
    assert_eq!(result.node_metadata["join"]["received"], 1);
}

#[test]
fn test_a_container_outputs_its_sinks_in_node_order() {
    let result = run(
        r#"{
        "nodes": [
            { "id": "g", "type": "group",
              "nodes": [
                  { "id": "a", "type": "test-prefix", "parameters": { "prefix": "A-" } },
                  { "id": "b", "type": "test-prefix", "parameters": { "prefix": "B-" } },
                  { "id": "c", "type": "test-prefix", "parameters": { "prefix": "C-" } }
              ],
              "edges": [
                  { "id": "c1", "source": "a", "target": "c" },
                  { "id": "c2", "source": "a", "target": "b" }
              ] }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
    );
    assert_eq!(names(&result), vec!["B-A-x.txt", "C-A-x.txt"]);
}

#[test]
fn test_container_children_report_their_execution_position() {
    let recorder = RecordingReporter::new();
    run_with(
        r#"{
        "nodes": [
            { "id": "g", "type": "group",
              "nodes": [
                  { "id": "b", "type": "test-echo" },
                  { "id": "a", "type": "test-echo" }
              ],
              "edges": [ { "id": "c1", "source": "a", "target": "b" } ] }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap();
    let children: Vec<(String, usize)> = recorder
        .events()
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeStarted {
                node_id,
                node_index,
                parent_node_id: Some(_),
                ..
            } => Some((node_id.clone(), *node_index)),
            _ => None,
        })
        .collect();
    assert_eq!(children, vec![("a".to_string(), 0), ("b".to_string(), 1)]);
}

// --- Failure inside a branch (PR 3 completes cross-branch `always`) ---

#[test]
fn test_a_failing_branch_fails_the_run_once() {
    let recorder = RecordingReporter::new();
    let error = run_with(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "bad", "type": "test-fail" },
            { "id": "keep", "type": "test-echo", "always": true },
            { "id": "other", "type": "test-echo" },
            { "id": "output", "type": "output" }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "bad" },
            { "id": "e2", "source": "bad", "target": "keep" },
            { "id": "e3", "source": "input", "target": "other" },
            { "id": "e4", "source": "keep", "target": "output" },
            { "id": "e5", "source": "other", "target": "output" }
        ]
    }"#,
        vec![make_file("x.txt", b"x")],
        &recorder.reporter(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("intentional test failure"));
    let events = recorder.events();
    let started: Vec<String> = started_nodes(&events).into_iter().map(|s| s.0).collect();
    assert_eq!(started, vec!["bad", "keep"]);
    let failed = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::PipelineFailed { .. }))
        .count();
    assert_eq!(failed, 1);
    assert!(matches!(
        events.last(),
        Some(PipelineEvent::PipelineFailed { .. })
    ));
}
