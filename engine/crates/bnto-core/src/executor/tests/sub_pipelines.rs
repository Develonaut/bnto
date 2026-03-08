use super::*;

// =========================================================================
// Thorough Event Reporting Tests
// =========================================================================
//
// These tests assert the EXACT sequence and content of PipelineEvents
// emitted during execution. They verify what the UI will see — progress
// bar accuracy, node highlighting, duration tracking, and error messages.

#[test]
fn test_single_file_full_event_sequence() {
    // Verify the exact event sequence for one file through one node.
    // This is the simplest case — the "golden path" the UI depends on.
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

    let files = vec![make_file("photo.jpg", b"image-data")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Assert the exact sequence: PipelineStarted → NodeStarted →
    // FileProgress(0%) → FileProgress(100%) → NodeCompleted → PipelineCompleted
    assert_eq!(
        events.len(),
        6,
        "Expected exactly 6 events for 1 file, 1 node"
    );

    // Event 0: PipelineStarted with correct counts.
    if let PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    } = &events[0]
    {
        assert_eq!(*total_nodes, 1, "1 processing node (I/O excluded)");
        assert_eq!(*total_files, 1);
    } else {
        panic!("Event 0 should be PipelineStarted, got {:?}", events[0]);
    }

    // Event 1: NodeStarted with correct ID and index.
    if let PipelineEvent::NodeStarted {
        node_id,
        node_index,
        total_nodes,
        node_type,
    } = &events[1]
    {
        assert_eq!(node_id, "proc");
        assert_eq!(*node_index, 0);
        assert_eq!(*total_nodes, 1);
        assert_eq!(node_type, "test");
    } else {
        panic!("Event 1 should be NodeStarted, got {:?}", events[1]);
    }

    // Event 2: FileProgress at 0% (processing starts).
    if let PipelineEvent::FileProgress {
        node_id,
        file_index,
        total_files,
        percent,
        message,
    } = &events[2]
    {
        assert_eq!(node_id, "proc");
        assert_eq!(*file_index, 0);
        assert_eq!(*total_files, 1);
        assert_eq!(*percent, 0);
        assert!(
            message.contains("photo.jpg"),
            "Message should include filename: {}",
            message
        );
    } else {
        panic!("Event 2 should be FileProgress(0%), got {:?}", events[2]);
    }

    // Event 3: FileProgress at 100% (processing done).
    if let PipelineEvent::FileProgress {
        percent, message, ..
    } = &events[3]
    {
        assert_eq!(*percent, 100);
        assert!(
            message.contains("photo.jpg"),
            "Message should include filename: {}",
            message
        );
    } else {
        panic!("Event 3 should be FileProgress(100%), got {:?}", events[3]);
    }

    // Event 4: NodeCompleted with correct file count and duration.
    if let PipelineEvent::NodeCompleted {
        node_id,
        files_processed,
        duration_ms,
    } = &events[4]
    {
        assert_eq!(node_id, "proc");
        assert_eq!(*files_processed, 1);
        // Duration should be non-negative (we use fake_now so it's 0).
        assert_eq!(*duration_ms, 0, "fake_now produces 0ms duration");
    } else {
        panic!("Event 4 should be NodeCompleted, got {:?}", events[4]);
    }

    // Event 5: PipelineCompleted with correct totals.
    if let PipelineEvent::PipelineCompleted {
        total_files_processed,
        duration_ms,
    } = &events[5]
    {
        assert_eq!(*total_files_processed, 1);
        assert_eq!(*duration_ms, 0);
    } else {
        panic!("Event 5 should be PipelineCompleted, got {:?}", events[5]);
    }
}

#[test]
fn test_multi_file_progress_tracking() {
    // 3 files through 1 node. Verify each file gets its own
    // FileProgress(0%) and FileProgress(100%) with correct indices.
    // This is what powers "File 2/3 at 50%" in the UI progress bar.
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
        make_file("a.jpg", b"aaa"),
        make_file("b.jpg", b"bbb"),
        make_file("c.jpg", b"ccc"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Collect all FileProgress events grouped by file_index.
    let progress_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { .. }))
        .collect();

    // 3 files × 2 progress events each (0% + 100%) = 6 FileProgress events.
    assert_eq!(
        progress_events.len(),
        6,
        "3 files × (0% + 100%) = 6 progress events"
    );

    // Verify the file_index and total_files are correct for each event.
    // Events should come in pairs: (file 0, 0%), (file 0, 100%),
    // (file 1, 0%), (file 1, 100%), (file 2, 0%), (file 2, 100%).
    let expected_sequence = [
        (0, 0, "a.jpg"),
        (0, 100, "a.jpg"),
        (1, 0, "b.jpg"),
        (1, 100, "b.jpg"),
        (2, 0, "c.jpg"),
        (2, 100, "c.jpg"),
    ];

    for (i, (expected_idx, expected_pct, expected_name)) in expected_sequence.iter().enumerate()
    {
        if let PipelineEvent::FileProgress {
            file_index,
            total_files,
            percent,
            message,
            ..
        } = &progress_events[i]
        {
            assert_eq!(
                *file_index, *expected_idx,
                "Event {} file_index: expected {}, got {}",
                i, expected_idx, file_index
            );
            assert_eq!(*total_files, 3, "total_files should always be 3");
            assert_eq!(
                *percent, *expected_pct as u32,
                "Event {} percent: expected {}, got {}",
                i, expected_pct, percent
            );
            assert!(
                message.contains(expected_name),
                "Event {} message should contain '{}': {}",
                i,
                expected_name,
                message
            );
        } else {
            panic!("Expected FileProgress at index {}", i);
        }
    }

    // PipelineCompleted should report total files processed = 3.
    if let PipelineEvent::PipelineCompleted {
        total_files_processed,
        ..
    } = events.last().unwrap()
    {
        assert_eq!(*total_files_processed, 3);
    } else {
        panic!("Last event should be PipelineCompleted");
    }
}

#[test]
fn test_multi_node_multi_file_progress() {
    // 2 files through 2 nodes. Verify that file_index resets per node
    // and total_files reflects the files available at each stage.
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

    let files = vec![make_file("x.txt", b"hello"), make_file("y.txt", b"world")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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

    // Each node processes 2 files → 2 × (0% + 100%) = 4 events per node.
    assert_eq!(
        n1_progress.len(),
        4,
        "n1 should emit 4 progress events (2 files × 2)"
    );
    assert_eq!(
        n2_progress.len(),
        4,
        "n2 should emit 4 progress events (2 files × 2)"
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

    // PipelineCompleted should report total_files_processed = 4 (2 files × 2 nodes).
    if let PipelineEvent::PipelineCompleted {
        total_files_processed,
        ..
    } = events.last().unwrap()
    {
        assert_eq!(
            *total_files_processed, 4,
            "2 files × 2 nodes = 4 total processed"
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
            { "id": "proc", "type": "test", "params": { "operation": "echo" } },
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
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Find NodeCompleted for "proc".
    let completed = events
        .iter()
        .find(
            |e| matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "proc"),
        )
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
            { "id": "broken", "type": "test", "params": { "operation": "fail" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    let _ = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    let events = recorder.events();

    // Find NodeFailed event and verify its fields.
    let node_failed = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeFailed { .. }))
        .expect("Should have NodeFailed event");

    if let PipelineEvent::NodeFailed { node_id, error } = node_failed {
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
fn test_container_node_event_nesting() {
    // Group → Loop → Processor: verify events nest correctly.
    // The UI uses this to know which node is "active" at each level.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group1", "type": "group",
                "nodes": [{
                    "id": "loop1", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "leaf", "type": "test",
                        "params": { "operation": "echo" }
                    }]
                }]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Collect all NodeStarted events in order.
    let started: Vec<(&str, &str)> = events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeStarted {
                node_id, node_type, ..
            } => Some((node_id.as_str(), node_type.as_str())),
            _ => None,
        })
        .collect();

    // Group starts first, then loop, then leaf (once per file).
    assert!(
        started.len() >= 4,
        "Expected at least 4 NodeStarted: group + loop + 2× leaf, got {}",
        started.len()
    );
    assert_eq!(started[0], ("group1", "group"), "Group should start first");
    assert_eq!(started[1], ("loop1", "loop"), "Loop should start second");
    // Leaf starts once per file inside the loop.
    assert_eq!(started[2].0, "leaf", "Leaf should start for first file");
    assert_eq!(started[3].0, "leaf", "Leaf should start for second file");

    // Verify NodeCompleted nesting: leaf completes before loop, loop before group.
    let completed_ids: Vec<&str> = events
        .iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeCompleted { node_id, .. } => Some(node_id.as_str()),
            _ => None,
        })
        .collect();

    // Leaf completes twice (per file), then loop, then group.
    assert!(
        completed_ids.len() >= 4,
        "Expected at least 4 NodeCompleted events"
    );
    assert_eq!(completed_ids[0], "leaf", "First leaf completes");
    assert_eq!(completed_ids[1], "leaf", "Second leaf completes");

    // FileProgress events should reference the leaf node (the processor).
    let leaf_progress: Vec<&PipelineEvent> = events
        .iter()
        .filter(
            |e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "leaf"),
        )
        .collect();
    assert_eq!(
        leaf_progress.len(),
        4,
        "2 files × (0% + 100%) = 4 leaf progress events"
    );
}

#[test]
fn test_pipeline_started_excludes_io_nodes() {
    // PipelineStarted.total_nodes should NOT count input/output nodes.
    // The UI uses this to calculate "Node 2 of 5" progress indicators.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "n1", "type": "test", "params": { "operation": "echo" } },
            { "id": "n2", "type": "test", "params": { "operation": "uppercase" } },
            { "id": "n3", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"data")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // PipelineStarted should report 3 nodes (not 5).
    if let PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    } = &events[0]
    {
        assert_eq!(*total_nodes, 3, "3 processing nodes, 2 I/O excluded");
        assert_eq!(*total_files, 1);
    } else {
        panic!("First event should be PipelineStarted");
    }

    // Each NodeStarted should have total_nodes = 3.
    for event in &events {
        if let PipelineEvent::NodeStarted { total_nodes, .. } = event {
            assert_eq!(
                *total_nodes, 3,
                "NodeStarted.total_nodes should match pipeline total"
            );
        }
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
            { "id": "proc", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files: Vec<PipelineFile> = vec![];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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
