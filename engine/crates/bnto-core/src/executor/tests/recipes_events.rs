// Event sequence verification and smoke tests for recipe structures.
use super::recipes::*;
use super::*;

#[test]
fn test_recipe_compress_images_event_sequence() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("a.jpg", b"aaa"), make_file("b.jpg", b"bbb")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // Should start with PipelineStarted.
    assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

    // Should end with PipelineCompleted.
    assert!(matches!(
        events.last().unwrap(),
        PipelineEvent::PipelineCompleted { .. }
    ));

    // Should have NodeStarted for the batch-compress group (sub-recipe).
    let group_started = events.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress"),
    );
    assert!(
        group_started,
        "Should emit NodeStarted for sub-recipe group node"
    );

    // Should have NodeStarted for the loop inside the sub-recipe.
    let loop_started = events.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-loop"),
    );
    assert!(
        loop_started,
        "Should emit NodeStarted for loop node inside sub-recipe"
    );

    // Should have NodeStarted for the child processor (runs per file).
    let child_started_count = events
        .iter()
        .filter(|e| {
            matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-image")
        })
        .count();
    assert_eq!(
        child_started_count, 2,
        "Child processor should start once per file"
    );

    // Should have NodeCompleted for the child processor (runs per file).
    let child_completed_count = events
        .iter()
        .filter(|e| {
            matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "compress-image")
        })
        .count();
    assert_eq!(
        child_completed_count, 2,
        "Child processor should complete once per file"
    );
}

#[test]
fn test_recipe_clean_csv_event_sequence() {
    let def = parse_def(clean_csv_json());
    let registry = recipe_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("data.csv", b"csv-content")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    let events = recorder.events();

    // PipelineStarted should report the csv-cleaner group as 1 processing
    // node at the top level (I/O nodes excluded). The group's children are
    // counted separately during sub-pipeline execution.
    if let PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    } = &events[0]
    {
        assert_eq!(
            *total_nodes, 1,
            "1 top-level processing node (csv-cleaner group), I/O excluded"
        );
        assert_eq!(*total_files, 1);
    } else {
        panic!("First event should be PipelineStarted");
    }

    // NodeStarted for the csv-cleaner group + clean processor inside it = 2.
    let node_started_count = events
        .iter()
        .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
        .count();
    assert_eq!(
        node_started_count, 2,
        "Group + processor = 2 NodeStarted events"
    );
}

// --- Smoke Tests: All 6 Recipes Deserialize ---

#[test]
fn test_all_six_recipe_structures_deserialize() {
    // Verify every recipe structure can be parsed without error.
    // All 6 use the compositional pattern: Input → Group(sub-recipe) → Output.
    let recipes = [
        compress_images_json(),
        clean_csv_json(),
        rename_files_json(),
        // Resize: Input → Group → Loop → [image:resize] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "resize", "width": 200 } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Convert: Input → Group → Loop → [image:convert] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "convert", "format": "webp" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                    { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename", "columns": {} } }
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
    ];

    for (i, json) in recipes.iter().enumerate() {
        let result: Result<PipelineDefinition, _> = serde_json::from_str(json);
        assert!(
            result.is_ok(),
            "Recipe {} failed to deserialize: {:?}",
            i,
            result.err()
        );
    }
}

#[test]
fn test_all_six_recipes_execute_with_mocks() {
    // Run every recipe with the compositional sub-recipe pattern.
    let recipes = [
        compress_images_json(),
        clean_csv_json(),
        rename_files_json(),
        // Resize: Input → Group → Loop → [image:resize] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "resize" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Convert: Input → Group → Loop → [image:convert] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                    { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "image", "parameters": { "operation": "convert" } }
                    ]}
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
        // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
        r#"{
            "nodes": [
                { "id": "in", "type": "input", "parameters": {} },
                { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                    { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename" } }
                ]},
                { "id": "out", "type": "output", "parameters": {} }
            ]
        }"#,
    ];

    let registry = recipe_registry();
    let files = vec![make_file("test-file.dat", b"test-data")];

    for (i, json) in recipes.iter().enumerate() {
        let def = parse_def(json);
        let reporter = PipelineReporter::new_noop();
        let result = execute_pipeline(&def, files.clone(), &registry, &reporter, fake_now);
        assert!(
            result.is_ok(),
            "Recipe {} failed to execute: {:?}",
            i,
            result.err()
        );
        assert!(
            !result.unwrap().files.is_empty(),
            "Recipe {} produced no output files",
            i
        );
    }
}
