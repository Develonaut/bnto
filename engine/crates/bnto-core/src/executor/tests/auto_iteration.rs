use super::*;

// =========================================================================
// Auto-Iteration Tests
// =========================================================================

// --- Source Node Tests ---

#[test]
fn test_source_node_executes_once_with_no_files() {
    // Source processor runs once with empty data, params populated.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "src", "type": "test-source", "params": { "url": "https://example.com/video" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    // No input files — source processor generates its own output
    let result =
        execute_pipeline(&def, vec![], &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "video.mp4");
    assert!(
        String::from_utf8_lossy(&result.files[0].data)
            .contains("downloaded-from:https://example.com/video")
    );
}

#[test]
fn test_source_node_ignores_input_files() {
    // Even if files are passed to the pipeline, a Source processor runs
    // once with empty input data and ignores the files.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "src", "type": "test-source", "params": { "url": "https://example.com/vid" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Source processor only runs once, producing 1 file regardless of input count
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "vid.mp4");
}

#[test]
fn test_auto_iteration_source_breaks_sequence() {
    // In auto mode, a source node breaks per-file sequences.
    // Nodes before source get per-file iteration, source runs once,
    // nodes after source get the source output as their input.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "upper", "type": "test-uppercase" },
            { "id": "src", "type": "test-source", "params": { "url": "https://example.com/v" } },
            { "id": "echo", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Source breaks the sequence: uppercase runs per-file on inputs (but output
    // is discarded by source), source generates 1 file, echo echoes it.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "v.mp4");
}

#[test]
fn test_auto_iteration_source_runs_with_empty_files() {
    // Source node in auto mode runs with no files and produces output.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "src", "type": "test-source", "params": { "url": "https://example.com/output" } },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let result =
        execute_pipeline(&def, vec![], &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "output.mp4");
}

// --- Backward Compatibility ---

#[test]
fn test_explicit_mode_flat_pipeline_unchanged() {
    // Explicit iteration with a flat pipeline behaves exactly as before.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "explicit" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}

#[test]
fn test_no_settings_defaults_to_explicit_behavior() {
    // Missing settings field behaves identically to explicit mode.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}

// --- Auto Mode: Single Processor ---

#[test]
fn test_auto_single_processor_per_file() {
    // Auto mode with one processor: each file processed independently.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 3);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
    assert_eq!(result.files[2].name, "C.TXT");
}

// --- Auto Mode: Multi-Processor Sequence ---

#[test]
fn test_auto_multi_processor_per_file_pipeline() {
    // Auto mode with two processors: each file runs through the full
    // sequence (uppercase → echo) before the next file starts.
    // In explicit mode, all files go through uppercase, then all go through echo.
    // In auto mode, file ordering is per-file: a→(upper,echo), b→(upper,echo).
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "upper", "type": "test-uppercase" },
            { "id": "echo", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Both files are uppercased, output order matches input order.
    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}

// --- Auto Mode: Container Passthrough ---

#[test]
fn test_auto_mode_preserves_explicit_containers() {
    // Auto mode with an explicit loop container: the container dispatches
    // as-is (containers define their own iteration strategy).
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-uppercase" }
                ]
            },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}

// --- Equivalence Proofs ---

#[test]
fn test_auto_matches_explicit_loop_single_processor() {
    // Prove: auto flat recipe produces identical output to explicit loop.
    let auto_def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let explicit_def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "proc", "type": "test-uppercase" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files_a = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let files_b = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];

    let auto_result = execute_pipeline(
        &auto_def,
        files_a,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    let explicit_result = execute_pipeline(
        &explicit_def,
        files_b,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();

    assert_eq!(auto_result.files.len(), explicit_result.files.len());
    for (auto_file, explicit_file) in auto_result.files.iter().zip(explicit_result.files.iter()) {
        assert_eq!(auto_file.name, explicit_file.name);
        assert_eq!(auto_file.data, explicit_file.data);
        assert_eq!(auto_file.mime_type, explicit_file.mime_type);
    }
}

#[test]
fn test_auto_matches_explicit_loop_multi_processor() {
    // The key equivalence test: auto flat multi-node sequence produces
    // identical output to an explicit loop wrapping the same sequence.
    let auto_def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "upper", "type": "test-uppercase" },
            { "id": "echo", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let explicit_def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "upper", "type": "test-uppercase" },
                    { "id": "echo", "type": "test-echo" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files_a = vec![
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    let files_b = vec![
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];

    let auto_result = execute_pipeline(
        &auto_def,
        files_a,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    let explicit_result = execute_pipeline(
        &explicit_def,
        files_b,
        &registry,
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();

    assert_eq!(auto_result.files.len(), explicit_result.files.len());
    for (auto_file, explicit_file) in auto_result.files.iter().zip(explicit_result.files.iter()) {
        assert_eq!(auto_file.name, explicit_file.name);
        assert_eq!(auto_file.data, explicit_file.data);
        assert_eq!(auto_file.mime_type, explicit_file.mime_type);
    }
}

// --- Progress Events ---

#[test]
fn test_auto_mode_progress_events_correct() {
    // Auto mode emits correct PipelineStarted/NodeStarted/NodeCompleted/PipelineCompleted.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // First event: PipelineStarted
    assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

    // Last event: PipelineCompleted
    assert!(matches!(
        events.last().unwrap(),
        PipelineEvent::PipelineCompleted { .. }
    ));

    // Should have NodeStarted events for the processor (once per file in auto mode)
    let node_started_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .count();
    assert!(
        node_started_count >= 1,
        "Expected at least 1 NodeStarted event, got {}",
        node_started_count
    );

    // NodeCompleted events should match NodeStarted events
    let node_completed_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
        .count();
    assert_eq!(
        node_started_count, node_completed_count,
        "NodeStarted ({}) and NodeCompleted ({}) counts should match",
        node_started_count, node_completed_count
    );
}

// --- Edge Cases ---

#[test]
fn test_auto_mode_empty_files() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-uppercase" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let result =
        execute_pipeline(&def, vec![], &registry, &reporter, &NoopContext, fake_now).unwrap();
    assert!(result.files.is_empty());
}

#[test]
fn test_auto_mode_io_only_passthrough() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "test.txt");
}

#[test]
fn test_auto_mode_mixed_primitive_and_container() {
    // Primitive before container, container, then primitive after.
    // Auto-iteration wraps each primitive run per-file, containers dispatch as-is.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "upper", "type": "test-uppercase" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-echo" }
                ]
            },
            { "id": "echo", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ],
        "settings": { "iteration": "auto" }
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    // Files uppercased, then looped (per-file echo), then echoed per-file
    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "A.TXT");
    assert_eq!(result.files[1].name, "B.TXT");
}
