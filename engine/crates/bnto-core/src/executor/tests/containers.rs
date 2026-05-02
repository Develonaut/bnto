use super::*;

// =========================================================================
// Container Execution Tests
// =========================================================================

// --- Event tests for containers ---

#[test]
fn test_loop_emits_iteration_started_events() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-echo" }
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
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();
    let iteration_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::IterationStarted { .. }))
        .collect();

    assert_eq!(
        iteration_events.len(),
        3,
        "3-item loop should emit 3 IterationStarted"
    );

    for (i, event) in iteration_events.iter().enumerate() {
        if let PipelineEvent::IterationStarted {
            node_id,
            iteration,
            total_iterations,
        } = event
        {
            assert_eq!(node_id, "loop-1");
            assert_eq!(*iteration, i);
            assert_eq!(*total_iterations, 3);
        }
    }
}

#[test]
fn test_loop_child_nodes_have_parent_id() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-echo" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // Child NodeStarted events should have parent_node_id = Some("loop-1").
    let child_started: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "child"))
        .collect();

    assert_eq!(child_started.len(), 2, "Child runs once per file in loop");
    for event in &child_started {
        if let PipelineEvent::NodeStarted { parent_node_id, .. } = event {
            assert_eq!(
                parent_node_id.as_deref(),
                Some("loop-1"),
                "Child NodeStarted should reference loop parent"
            );
        }
    }

    // Child NodeCompleted events should also have parent_node_id.
    let child_completed: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "child"))
        .collect();

    assert_eq!(child_completed.len(), 2);
    for event in &child_completed {
        if let PipelineEvent::NodeCompleted { parent_node_id, .. } = event {
            assert_eq!(parent_node_id.as_deref(), Some("loop-1"));
        }
    }
}

#[test]
fn test_group_child_nodes_have_parent_id() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group-1", "type": "group",
                "children": [
                    { "id": "child", "type": "test-uppercase" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.txt", b"aaa")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    let child_started = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "child"))
        .expect("Should have NodeStarted for group child");

    if let PipelineEvent::NodeStarted { parent_node_id, .. } = child_started {
        assert_eq!(
            parent_node_id.as_deref(),
            Some("group-1"),
            "Group child should reference group parent"
        );
    }
}

#[test]
fn test_top_level_nodes_have_no_parent_id() {
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

    let files = vec![make_file("a.txt", b"aaa")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    let started = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "proc"))
        .expect("Should have NodeStarted for top-level node");

    if let PipelineEvent::NodeStarted { parent_node_id, .. } = started {
        assert_eq!(
            *parent_node_id, None,
            "Top-level NodeStarted should have no parent"
        );
    }

    let completed = events
        .iter()
        .find(|e| matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "proc"))
        .expect("Should have NodeCompleted for top-level node");

    if let PipelineEvent::NodeCompleted { parent_node_id, .. } = completed {
        assert_eq!(
            *parent_node_id, None,
            "Top-level NodeCompleted should have no parent"
        );
    }
}

#[test]
fn test_loop_emits_iteration_completed_events() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-echo" }
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
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();
    let completed_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::IterationCompleted { .. }))
        .collect();

    assert_eq!(
        completed_events.len(),
        3,
        "3-item loop should emit 3 IterationCompleted"
    );

    for (i, event) in completed_events.iter().enumerate() {
        if let PipelineEvent::IterationCompleted {
            node_id,
            iteration,
            total_iterations,
            files_produced,
            ..
        } = event
        {
            assert_eq!(node_id, "loop-1");
            assert_eq!(*iteration, i);
            assert_eq!(*total_iterations, 3);
            assert_eq!(*files_produced, 1, "Each iteration produces 1 file");
        }
    }
}

#[test]
fn test_loop_iteration_events_alternate_correctly() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test-echo" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // Verify IterationStarted is always followed by IterationCompleted for same iteration.
    let iteration_events: Vec<&PipelineEvent> = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                PipelineEvent::IterationStarted { node_id, .. }
                | PipelineEvent::IterationCompleted { node_id, .. }
                if node_id == "loop-1"
            )
        })
        .collect();

    // Should be: Started(0), Completed(0), Started(1), Completed(1)
    assert_eq!(iteration_events.len(), 4);
    assert!(matches!(
        iteration_events[0],
        PipelineEvent::IterationStarted { iteration: 0, .. }
    ));
    assert!(matches!(
        iteration_events[1],
        PipelineEvent::IterationCompleted { iteration: 0, .. }
    ));
    assert!(matches!(
        iteration_events[2],
        PipelineEvent::IterationStarted { iteration: 1, .. }
    ));
    assert!(matches!(
        iteration_events[3],
        PipelineEvent::IterationCompleted { iteration: 1, .. }
    ));
}

#[test]
fn test_loop_node_runs_children_per_file() {
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

#[test]
fn test_group_node_runs_children_on_full_batch() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group-1", "type": "group",
                "children": [
                    { "id": "child", "type": "test-uppercase" }
                ]
            },
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

#[test]
fn test_nested_loop_inside_group() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group-1", "type": "group",
                "children": [
                    {
                        "id": "loop-1", "type": "loop",
                        "children": [
                            { "id": "proc", "type": "test-echo" }
                        ]
                    }
                ]
            },
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
    assert_eq!(result.files[0].name, "a.txt");
}

#[test]
fn test_container_with_no_children_is_passthrough() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "loop-1", "type": "loop", "children": [] },
            { "id": "out", "type": "output" }
        ]
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
