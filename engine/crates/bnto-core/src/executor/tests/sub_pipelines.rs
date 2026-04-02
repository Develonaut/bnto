// Sub-pipeline event sequence tests for container nodes.
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
            { "id": "proc", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("photo.jpg", b"image-data")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

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
        assert_eq!(node_type, "test-echo");
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
            { "id": "proc", "type": "test-echo" },
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
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

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

    for (i, (expected_idx, expected_pct, expected_name)) in expected_sequence.iter().enumerate() {
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
