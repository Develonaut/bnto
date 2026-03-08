use super::*;

// =========================================================================
// Metadata Preservation Tests
// =========================================================================

#[test]
fn test_processor_metadata_appears_in_final_result() {
    // A processor that sets originalSize/compressedSize/compressionRatio
    // should have those values in the final PipelineFileResult.metadata.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "proc", "type": "test", "params": { "operation": "metadata" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", &[0u8; 100])];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    // Metadata must contain the stats the processor set.
    assert_eq!(metadata["originalSize"], 100);
    assert_eq!(metadata["compressedSize"], 50);
    assert!(
        metadata.contains_key("compressionRatio"),
        "metadata should contain compressionRatio"
    );
}

#[test]
fn test_metadata_preserved_through_loop_container() {
    // Metadata should survive when a processor runs inside a loop container.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "proc", "type": "test", "params": { "operation": "metadata" } }
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
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 2);

    // First file: 200 bytes input → 100 bytes output.
    assert_eq!(result.files[0].metadata["originalSize"], 200);
    assert_eq!(result.files[0].metadata["compressedSize"], 100);

    // Second file: 400 bytes input → 200 bytes output.
    assert_eq!(result.files[1].metadata["originalSize"], 400);
    assert_eq!(result.files[1].metadata["compressedSize"], 200);
}

#[test]
fn test_metadata_preserved_through_group_container() {
    // Metadata should survive when a processor runs inside a group container.
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "group-1", "type": "group",
                "children": [
                    { "id": "proc", "type": "test", "params": { "operation": "metadata" } }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    let files = vec![make_file("photo.jpg", &[0u8; 300])];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].metadata["originalSize"], 300);
    assert_eq!(result.files[0].metadata["compressedSize"], 150);
}

#[test]
fn test_metadata_from_last_processor_wins_in_chain() {
    // When two processors are chained, the metadata from the LAST
    // processor should be on the final result (since it overwrites
    // the PipelineFile.metadata during chaining).
    let def = parse_def(
        r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "first", "type": "test", "params": { "operation": "metadata" } },
            { "id": "second", "type": "test", "params": { "operation": "metadata" } },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let registry = mock_registry();
    let reporter = PipelineReporter::new_noop();

    // 100 bytes → first processor outputs 50 bytes → second outputs 25.
    let files = vec![make_file("photo.jpg", &[0u8; 100])];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    // The second processor sees 50-byte input, outputs 25 bytes.
    assert_eq!(metadata["originalSize"], 50);
    assert_eq!(metadata["compressedSize"], 25);
}

#[test]
fn test_echo_processor_preserves_empty_metadata() {
    // An echo processor that sets no metadata should result in
    // empty metadata on the final result (not crash or contain stale data).
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

    let files = vec![make_file("test.txt", b"hello")];
    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

    assert_eq!(result.files.len(), 1);
    assert!(
        result.files[0].metadata.is_empty(),
        "Echo processor should produce empty metadata"
    );
}
