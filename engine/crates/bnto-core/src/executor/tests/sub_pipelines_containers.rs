// Container-specific event sequences (group, loop, nesting).
use super::*;

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
                        "id": "leaf", "type": "test-echo"
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
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

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
        "Expected at least 4 NodeStarted: group + loop + 2x leaf, got {}",
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
        .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "leaf"))
        .collect();
    assert_eq!(
        leaf_progress.len(),
        4,
        "2 files x (0% + 100%) = 4 leaf progress events"
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
            { "id": "n1", "type": "test-echo" },
            { "id": "n2", "type": "test-uppercase" },
            { "id": "n3", "type": "test-echo" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"data")];
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    let events = recorder.events();

    // PipelineStarted should report 3 nodes (not 5).
    if let PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
        ..
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
