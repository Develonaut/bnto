use super::*;

// =========================================================================
// Metadata Preservation Tests
// =========================================================================

#[test]
fn test_processor_metadata_appears_in_final_result() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test-metadata" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", &[0u8; 100])];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    assert_eq!(metadata["originalSize"], 100);
    assert_eq!(metadata["compressedSize"], 50);
    assert!(
        metadata.contains_key("compressionRatio"),
        "metadata should contain compressionRatio"
    );
}

#[test]
fn test_metadata_preserved_through_loop_container() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "proc", "type": "test-metadata" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![
        make_file("a.jpg", &[0u8; 200]),
        make_file("b.jpg", &[0u8; 400]),
    ];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.files[0].metadata["originalSize"], 200);
    assert_eq!(result.files[0].metadata["compressedSize"], 100);
    assert_eq!(result.files[1].metadata["originalSize"], 400);
    assert_eq!(result.files[1].metadata["compressedSize"], 200);
}

#[test]
fn test_metadata_preserved_through_group_container() {
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group-1", "type": "group",
                "children": [
                    { "id": "proc", "type": "test-metadata" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", &[0u8; 300])];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].metadata["originalSize"], 300);
    assert_eq!(result.files[0].metadata["compressedSize"], 150);
}

#[test]
fn test_metadata_from_last_processor_wins_in_chain() {
    // When two processors are chained, the last processor's metadata wins
    // (overwrites PipelineFile.metadata during chaining).
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "first", "type": "test-metadata" },
            { "id": "second", "type": "test-metadata" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    // 100 bytes -> first outputs 50 -> second outputs 25.
    let files = vec![make_file("photo.jpg", &[0u8; 100])];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    // Second processor sees 50-byte input, outputs 25 bytes.
    assert_eq!(metadata["originalSize"], 50);
    assert_eq!(metadata["compressedSize"], 25);
}

#[test]
fn test_echo_processor_preserves_empty_metadata() {
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
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("test.txt", b"hello")];
    let result =
        execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert!(
        result.files[0].metadata.is_empty(),
        "Echo processor should produce empty metadata"
    );
}
