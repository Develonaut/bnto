// Error and edge-case tests with recipe-style JSON structures.
use super::recipes::*;
use super::*;

#[test]
fn test_recipe_with_only_io_nodes_passthrough() {
    // Recipe with only input + output. No processing nodes.
    let json = r#"{
        "nodes": [
            {
                "id": "input", "type": "input", "version": "1.0.0",
                "name": "Input", "position": {"x": 0, "y": 0}, "metadata": {},
                "parameters": {}, "inputPorts": [], "outputPorts": []
            },
            {
                "id": "output", "type": "output", "version": "1.0.0",
                "name": "Output", "position": {"x": 0, "y": 0}, "metadata": {},
                "parameters": {}, "inputPorts": [], "outputPorts": []
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].data, b"hello");
}

#[test]
fn test_recipe_empty_files_no_error() {
    let def = parse_def(compress_images_json());
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();
    assert!(result.files.is_empty());
}

#[test]
fn test_recipe_container_io_children_skipped() {
    // A loop containing input + output + processor. I/O children are skipped.
    let json = r#"{
        "nodes": [
            {
                "id": "the-loop", "type": "loop",
                "parameters": { "mode": "forEach" },
                "nodes": [
                    { "id": "inner-input", "type": "input", "parameters": {} },
                    {
                        "id": "proc", "type": "image-compress"
                    },
                    { "id": "inner-output", "type": "output", "parameters": {} }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // I/O nodes inside loop are skipped, only the processor runs.
    assert_eq!(result.files.len(), 1);
}

#[test]
fn test_recipe_unregistered_operation_inside_loop() {
    // Loop contains a node with an operation that has no processor.
    let json = r#"{
        "nodes": [
            {
                "id": "the-loop", "type": "loop",
                "parameters": { "mode": "forEach" },
                "nodes": [
                    {
                        "id": "bad-node", "type": "spreadsheet-pivot"
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = recipe_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("data.csv", b"csv-data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("spreadsheet-pivot"),
        "Error should name the missing key: {}",
        err
    );
}

#[test]
fn test_recipe_failure_inside_nested_container() {
    // Group → Loop → FailProcessor. Error should propagate up.
    let json = r#"{
        "nodes": [
            {
                "id": "group-1", "type": "group",
                "parameters": {},
                "nodes": [
                    {
                        "id": "the-loop", "type": "loop",
                        "parameters": { "mode": "forEach" },
                        "nodes": [
                            {
                                "id": "fail-proc", "type": "test-fail"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let def = parse_def(json);
    let registry = mock_registry();
    let recorder = RecordingReporter::new();
    let reporter = recorder.reporter();

    let files = vec![make_file("test.txt", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

    assert!(result.is_err());

    let events = recorder.events();
    let has_pipeline_failed = events
        .iter()
        .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. }));
    assert!(
        has_pipeline_failed,
        "Should emit PipelineFailed for nested failure"
    );
}
