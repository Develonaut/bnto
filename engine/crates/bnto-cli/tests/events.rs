// Progress event sequence tests — verify the executor emits the right events in order.

mod common;

use common::{make_file, run_pipeline_with_events};

use bnto_core::PipelineEvent;

static SMALL_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

#[test]
fn single_node_pipeline_emits_correct_event_sequence() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let (_, events) = run_pipeline_with_events(def, files).unwrap();

    // Expected sequence: PipelineStarted → NodeStarted → FileProgress* → NodeCompleted → PipelineCompleted
    assert!(
        events.len() >= 4,
        "Expected at least 4 events, got {}",
        events.len()
    );

    assert!(
        matches!(events.first(), Some(PipelineEvent::PipelineStarted { .. })),
        "First event should be PipelineStarted"
    );
    assert!(
        matches!(events.get(1), Some(PipelineEvent::NodeStarted { .. })),
        "Second event should be NodeStarted"
    );
    assert!(
        matches!(events.last(), Some(PipelineEvent::PipelineCompleted { .. })),
        "Last event should be PipelineCompleted"
    );

    // NodeCompleted should appear before PipelineCompleted.
    let node_completed_idx = events
        .iter()
        .position(|e| matches!(e, PipelineEvent::NodeCompleted { .. }));
    assert!(
        node_completed_idx.is_some(),
        "Should have NodeCompleted event"
    );
    assert!(
        node_completed_idx.unwrap() < events.len() - 1,
        "NodeCompleted should appear before PipelineCompleted"
    );
}

#[test]
fn multi_node_pipeline_emits_node_events_for_each_node() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image", "parameters": { "operation": "resize", "width": 50 } },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let (_, events) = run_pipeline_with_events(def, files).unwrap();

    let node_started_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .count();
    let node_completed_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
        .count();

    assert_eq!(
        node_started_count, 2,
        "Should emit NodeStarted for each processing node"
    );
    assert_eq!(
        node_completed_count, 2,
        "Should emit NodeCompleted for each processing node"
    );
}

#[test]
fn pipeline_started_reports_correct_counts() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image", "parameters": { "operation": "resize", "width": 50 } },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![
        make_file("a.jpg", SMALL_JPG, "image/jpeg"),
        make_file("b.jpg", SMALL_JPG, "image/jpeg"),
    ];
    let (_, events) = run_pipeline_with_events(def, files).unwrap();

    if let Some(PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    }) = events.first()
    {
        assert_eq!(
            *total_nodes, 2,
            "Should report 2 processing nodes (excludes I/O)"
        );
        assert_eq!(*total_files, 2, "Should report 2 input files");
    } else {
        panic!("First event should be PipelineStarted");
    }
}

#[test]
fn pipeline_completed_reports_files_processed() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![
        make_file("a.jpg", SMALL_JPG, "image/jpeg"),
        make_file("b.jpg", SMALL_JPG, "image/jpeg"),
    ];
    let (_, events) = run_pipeline_with_events(def, files).unwrap();

    if let Some(PipelineEvent::PipelineCompleted {
        total_files_processed,
        ..
    }) = events.last()
    {
        assert!(
            *total_files_processed >= 2,
            "Should process at least 2 files, got {}",
            total_files_processed
        );
    } else {
        panic!("Last event should be PipelineCompleted");
    }
}

#[test]
fn io_only_pipeline_emits_started_and_completed() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let (_, events) = run_pipeline_with_events(def, files).unwrap();

    // With only I/O nodes, there are 0 processing nodes. We should still get
    // PipelineStarted and PipelineCompleted.
    assert!(events.len() >= 2);
    assert!(matches!(
        events.first(),
        Some(PipelineEvent::PipelineStarted { .. })
    ));
    assert!(matches!(
        events.last(),
        Some(PipelineEvent::PipelineCompleted { .. })
    ));
}
