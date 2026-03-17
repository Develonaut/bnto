use super::*;

// =========================================================================
// Error Handling Tests
// =========================================================================

#[test]
fn test_unknown_node_type_returns_error() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "unknown", "params": { "operation": "magic" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("unknown:magic"),
        "Error should include the compound key: {}",
        err
    );
}

#[test]
fn test_processor_failure_emits_node_failed() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test", "params": { "operation": "fail" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());

    let events = recorder.events();

    let has_node_failed = events
        .iter()
        .any(|e| matches!(e, PipelineEvent::NodeFailed { .. }));
    let has_pipeline_failed = events
        .iter()
        .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. }));

    assert!(has_node_failed, "Should emit NodeFailed event");
    assert!(has_pipeline_failed, "Should emit PipelineFailed event");
}

#[test]
fn test_error_mid_pipeline_stops_execution() {
    // First node succeeds (echo), second fails, third never runs.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "n1", "type": "test", "params": { "operation": "echo" } },
            { "id": "n2", "type": "test", "params": { "operation": "fail" } },
            { "id": "n3", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());

    let events = recorder.events();

    let completed: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
        .collect();

    assert_eq!(completed.len(), 1, "Only n1 should complete");

    if let PipelineEvent::NodeCompleted { node_id, .. } = completed[0] {
        assert_eq!(node_id, "n1");
    }

    let n3_started = events
        .iter()
        .any(|e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "n3"));
    assert!(!n3_started, "n3 should never start");
}
