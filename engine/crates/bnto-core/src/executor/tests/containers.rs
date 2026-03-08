use super::*;

// =========================================================================
// Container Execution Tests
// =========================================================================

#[test]
fn test_loop_node_runs_children_per_file() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "child", "type": "test", "params": { "operation": "uppercase" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Each file processed individually through the loop.
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
                    { "id": "child", "type": "test", "params": { "operation": "uppercase" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("a.txt", b"aaa"), make_file("b.txt", b"bbb")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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
                            { "id": "proc", "type": "test", "params": { "operation": "echo" } }
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "test.txt");
}
