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
            { "id": "proc", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    assert!(
        events.len() >= 4,
        "Expected at least 4 events, got {}",
        events.len()
    );
    assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));
    assert!(matches!(events[1], PipelineEvent::NodeStarted { .. }));
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
            { "id": "n1", "type": "test-echo" },
            { "id": "n2", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"hello")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    let node_started: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .collect();

    assert_eq!(node_started.len(), 2, "Should have 2 NodeStarted events");

    if let PipelineEvent::NodeStarted {
        node_id,
        node_index,
        ..
    } = &node_started[0]
    {
        assert_eq!(node_id, "n1");
        assert_eq!(*node_index, 0);
    }

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
            { "id": "proc", "type": "test-echo" },
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
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();
    let progress_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { percent: 0, .. }))
        .collect();

    assert_eq!(progress_events.len(), 3);

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

/// Regression: loop containers must report GLOBAL file count, not per-iteration
/// batch size (1). This was the "Processing 1 of 1" bug.
#[test]
fn test_loop_container_reports_global_file_count() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1",
                "type": "loop",
                "params": { "mode": "forEach" },
                "children": [
                    { "id": "proc", "type": "test-echo" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![
        make_file("a.png", b"aaa"),
        make_file("b.png", b"bbb"),
        make_file("c.png", b"ccc"),
        make_file("d.png", b"ddd"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    let progress_starts: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { percent: 0, .. }))
        .collect();

    assert_eq!(progress_starts.len(), 4);

    for (i, event) in progress_starts.iter().enumerate() {
        if let PipelineEvent::FileProgress {
            file_index,
            total_files,
            ..
        } = event
        {
            assert_eq!(
                *total_files, 4,
                "File {} should report total_files=4, got {}",
                i, total_files
            );
            assert_eq!(
                *file_index, i,
                "File {} should have file_index={}, got {}",
                i, i, file_index
            );
        }
    }
}

/// Same regression test with nested containers: group > loop > processor.
/// Mirrors the compress-images recipe structure.
#[test]
fn test_nested_group_loop_reports_global_file_count() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "batch-group",
                "type": "group",
                "children": [
                    {
                        "id": "compress-loop",
                        "type": "loop",
                        "params": { "mode": "forEach" },
                        "children": [
                            { "id": "compress", "type": "test-echo" }
                        ]
                    }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![
        make_file("img1.jpg", b"111"),
        make_file("img2.jpg", b"222"),
        make_file("img3.jpg", b"333"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();
    let progress_starts: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::FileProgress { percent: 0, .. }))
        .collect();

    assert_eq!(progress_starts.len(), 3);

    for (i, event) in progress_starts.iter().enumerate() {
        if let PipelineEvent::FileProgress {
            file_index,
            total_files,
            ..
        } = event
        {
            assert_eq!(
                *total_files, 3,
                "Nested container: total_files should be 3, got {}",
                total_files
            );
            assert_eq!(
                *file_index, i,
                "Nested container: file_index should be {}, got {}",
                i, file_index
            );
        }
    }
}

// =========================================================================
// Live Node Progress
// =========================================================================

/// The `NodeProgress` events of one run of `node_type` on one file.
fn live_progress(node_type: &str, now: fn() -> u64) -> Vec<(String, u64, Option<u64>, String)> {
    let def = parse_def(&format!(
        r#"{{ "nodes": [
            {{ "id": "in", "type": "input" }},
            {{ "id": "walk", "type": "{node_type}" }},
            {{ "id": "out", "type": "output" }}
        ] }}"#
    ));
    let recorder = RecordingReporter::new();
    let files = vec![make_file("folder", b"")];
    execute_pipeline(
        &def,
        files,
        &mock_registry(),
        &recorder.reporter(),
        &NoopContext,
        now,
    )
    .unwrap();
    recorder
        .events()
        .into_iter()
        .filter_map(|e| match e {
            PipelineEvent::NodeProgress {
                node_id,
                done,
                total,
                message,
            } => Some((node_id, done, total, message)),
            _ => None,
        })
        .collect()
}

#[test]
fn node_count_reports_reach_the_pipeline_as_node_progress() {
    let events = live_progress("test-counting", fake_now);
    assert_eq!(
        events[0],
        ("walk".to_string(), 1, None, "Found 1 files".to_string())
    );
}

#[test]
fn node_percent_reports_reach_the_pipeline_out_of_100() {
    let events = live_progress("test-slow", fake_now);
    assert_eq!(
        events[0],
        (
            "walk".to_string(),
            25,
            Some(100),
            "Quarter done".to_string()
        )
    );
}

#[test]
fn live_progress_is_throttled_while_the_clock_stands_still() {
    // fake_now never moves, so only the first of the five counts gets out.
    let events = live_progress("test-counting", fake_now);
    assert_eq!(events.len(), 1);
}

thread_local! {
    static TICKS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// A clock that moves 60ms every time anything reads it.
fn ticking_now() -> u64 {
    TICKS.with(|t| {
        t.set(t.get() + 60);
        t.get()
    })
}

#[test]
fn live_progress_gets_through_once_per_interval_as_time_passes() {
    // Reads 60ms apart against a 100ms interval: every other count passes.
    let dones: Vec<u64> = live_progress("test-counting", ticking_now)
        .into_iter()
        .map(|(_, done, _, _)| done)
        .collect();
    assert_eq!(dones, vec![1, 3, 5]);
}
