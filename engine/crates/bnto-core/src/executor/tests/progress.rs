use super::*;

// =========================================================================
// Progress Event Tests
// =========================================================================

#[test]
fn test_single_node_emits_correct_event_sequence() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Expected sequence:
    // 1. PipelineStarted
    // 2. NodeStarted
    // 3. FileProgress (0%)
    // 4. FileProgress (100%)
    // 5. NodeCompleted
    // 6. PipelineCompleted
    assert!(
        events.len() >= 4,
        "Expected at least 4 events, got {}",
        events.len()
    );

    // First event is PipelineStarted.
    assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

    // Second event is NodeStarted.
    assert!(matches!(events[1], PipelineEvent::NodeStarted { .. }));

    // Last event is PipelineCompleted.
    assert!(matches!(
        events.last().unwrap(),
        PipelineEvent::PipelineCompleted { .. }
    ));
}

#[test]
fn test_multi_node_events_in_order() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "n1", "type": "test", "params": { "operation": "echo" } },
            { "id": "n2", "type": "test", "params": { "operation": "uppercase" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Collect NodeStarted events.
    let node_started: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .collect();

    assert_eq!(node_started.len(), 2, "Should have 2 NodeStarted events");

    // First NodeStarted should be for n1.
    if let PipelineEvent::NodeStarted {
        node_id,
        node_index,
        ..
    } = &node_started[0]
    {
        assert_eq!(node_id, "n1");
        assert_eq!(*node_index, 0);
    }

    // Second NodeStarted should be for n2.
    if let PipelineEvent::NodeStarted {
        node_id,
        node_index,
        ..
    } = &node_started[1]
    {
        assert_eq!(node_id, "n2");
        assert_eq!(*node_index, 1);
    }
}

#[test]
fn test_file_progress_includes_correct_indices() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();
    let progress_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { percent: 0, .. }))
        .collect();

    // Should have 3 FileProgress(0%) events — one per file.
    assert_eq!(progress_events.len(), 3);

    // Verify file indices.
    for (i, event) in progress_events.iter().enumerate() {
        if let PipelineEvent::FileProgress {
            file_index,
            total_files,
            ..
        } = event
        {
            assert_eq!(*file_index, i);
            assert_eq!(*total_files, 3);
        }
    }
}
