// Edge cases: multi-node combos, error events, empty files.
use super::*;

#[test]
fn test_multi_node_multi_file_progress() {
    // 2 files through 2 nodes. Verify that file_index resets per node
    // and total_files reflects the files available at each stage.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "n1", "type": "test-echo" },
            { "id": "n2", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("x.txt", b"hello"), make_file("y.txt", b"world")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // Collect FileProgress events per node.
    let n1_progress: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "n1"))
        .collect();
    let n2_progress: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "n2"))
        .collect();

    // Each node processes 2 files -> 2 x (0% + 100%) = 4 events per node.
    assert_eq!(
        n1_progress.len(),
        4,
        "n1 should emit 4 progress events (2 files x 2)"
    );
    assert_eq!(
        n2_progress.len(),
        4,
        "n2 should emit 4 progress events (2 files x 2)"
    );

    // Verify file_index resets to 0 for the second node.
    if let PipelineEvent::FileProgress { file_index, .. } = n2_progress[0] {
        assert_eq!(
            *file_index, 0,
            "file_index should reset to 0 for n2's first file"
        );
    }

    // Verify NodeCompleted reports correct file counts for each node.
    let completed: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
        .collect();
    assert_eq!(completed.len(), 2);

    for node_completed in &completed {
        if let PipelineEvent::NodeCompleted {
            files_processed, ..
        } = node_completed
        {
            assert_eq!(*files_processed, 2, "Each node processed 2 files");
        }
    }

    // PipelineCompleted should report total_files_processed = 4 (2 files x 2 nodes).
    if let PipelineEvent::PipelineCompleted {
        total_files_processed,
        ..
    } = events.last().unwrap()
    {
        assert_eq!(
            *total_files_processed, 4,
            "2 files x 2 nodes = 4 total processed"
        );
    }
}

#[test]
fn test_node_completed_fields_are_correct() {
    // Verify NodeCompleted reports accurate files_processed and duration.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![
        make_file("1.txt", b"one"),
        make_file("2.txt", b"two"),
        make_file("3.txt", b"three"),
        make_file("4.txt", b"four"),
        make_file("5.txt", b"five"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // Find NodeCompleted for "proc".
    let completed = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "proc"))
        .expect("Should have NodeCompleted for 'proc'");

    if let PipelineEvent::NodeCompleted {
        node_id,
        files_processed,
        ..
    } = completed
    {
        assert_eq!(node_id, "proc");
        assert_eq!(*files_processed, 5, "Should report 5 files processed");
    }
}

#[test]
fn test_error_events_contain_useful_information() {
    // Verify NodeFailed and PipelineFailed events include the error
    // message and correct node_id so the UI can show meaningful errors.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "broken", "type": "test-fail" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    let _ = execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now);

    let events = recorder.events();

    // Find NodeFailed event and verify its fields.
    let node_failed = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeFailed { .. }))
        .expect("Should have NodeFailed event");

    if let PipelineEvent::NodeFailed { node_id, error, .. } = node_failed {
        assert_eq!(
            node_id, "broken",
            "NodeFailed should reference the failing node"
        );
        assert!(
            error.contains("intentional test failure"),
            "Error message should be descriptive: {}",
            error
        );
    }

    // Find PipelineFailed event and verify it also references the failing node.
    let pipeline_failed = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::PipelineFailed { .. }))
        .expect("Should have PipelineFailed event");

    if let PipelineEvent::PipelineFailed { node_id, error } = pipeline_failed {
        assert_eq!(
            node_id, "broken",
            "PipelineFailed should reference the failing node"
        );
        assert!(!error.is_empty(), "Error message should not be empty");
    }
}

#[test]
fn test_empty_files_emit_pipeline_events() {
    // Even with 0 files, PipelineStarted and PipelineCompleted should
    // still fire. The UI needs these to transition state correctly.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files: Vec<PipelineFile> = vec![];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // Should still get PipelineStarted (total_files: 0).
    assert!(matches!(
        &events[0],
        PipelineEvent::PipelineStarted { total_files: 0, .. }
    ));

    // Should still get PipelineCompleted.
    assert!(matches!(
        events.last().unwrap(),
        PipelineEvent::PipelineCompleted {
            total_files_processed: 0,
            ..
        }
    ));
}
