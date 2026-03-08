use super::*;

// =========================================================================
// Basic Execution Tests
// =========================================================================

#[test]
fn test_empty_definition_no_files() {
    let def = parse_def(r#"{ "nodes": [] }"#);
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();

    assert!(result.files.is_empty());
}

#[test]
fn test_io_only_pipeline_is_passthrough() {
    // A pipeline with ONLY input + output nodes = passthrough.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Files pass through unchanged.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "test.txt");
    assert_eq!(result.files[0].data, b"hello");
}

#[test]
fn test_single_node_single_file() {
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
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello world")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].data, b"hello world");
}

#[test]
fn test_single_node_multiple_files() {
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
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("a.txt", b"aaa"),
        make_file("b.txt", b"bbb"),
        make_file("c.txt", b"ccc"),
    ];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 3);
    assert_eq!(result.files[0].data, b"aaa");
    assert_eq!(result.files[1].data, b"bbb");
    assert_eq!(result.files[2].data, b"ccc");
}

#[test]
fn test_two_sequential_nodes() {
    // uppercase then echo — verify chaining works.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "upper", "type": "test", "params": { "operation": "uppercase" } },
            { "id": "echo", "type": "test", "params": { "operation": "echo" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // Filename should be uppercased by the first node, then echoed by the second.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].name, "TEST.TXT");
}

#[test]
fn test_double_processor_increases_file_count() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "dbl", "type": "test", "params": { "operation": "double" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"data")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    // DoubleProcessor returns 2 files per input.
    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].name, "test.txt-a");
    assert_eq!(result.files[1].name, "test.txt-b");
}

#[test]
fn test_empty_files_array() {
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
    let reporter = PipelineReporter::new_noop();

    let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();
    assert!(result.files.is_empty());
}

#[test]
fn test_100_files_batch() {
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
    let reporter = PipelineReporter::new_noop();

    let files: Vec<PipelineFile> = (0..100)
        .map(|i| make_file(&format!("file-{}.txt", i), b"data"))
        .collect();

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();
    assert_eq!(result.files.len(), 100);
}
