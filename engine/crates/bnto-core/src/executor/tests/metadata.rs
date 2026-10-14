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

// =========================================================================
// Per-file metadata accumulates across nodes
// =========================================================================

fn file_carrying(name: &str, data: &[u8], metadata: serde_json::Value) -> PipelineFile {
    PipelineFile {
        metadata: metadata.as_object().unwrap().clone(),
        ..make_file(name, data)
    }
}

fn run_one_file(def_json: &str, file: PipelineFile) -> serde_json::Map<String, serde_json::Value> {
    let def = parse_def(def_json);
    let reporter = PipelineReporter::new_noop();
    let result = execute_pipeline(
        &def,
        vec![file],
        &mock_registry(),
        &reporter,
        &NoopContext,
        fake_now,
    )
    .unwrap();
    result.files.into_iter().next().unwrap().metadata
}

const ECHO: &str = r#"{
    "settings": { "iteration": "auto" },
    "nodes": [
        { "id": "in", "type": "input" },
        { "id": "proc", "type": "test-echo" },
        { "id": "out", "type": "output" }
    ]
}"#;

const COMPRESS: &str = r#"{
    "settings": { "iteration": "auto" },
    "nodes": [
        { "id": "in", "type": "input" },
        { "id": "proc", "type": "test-metadata" },
        { "id": "out", "type": "output" }
    ]
}"#;

#[test]
fn test_a_node_that_attaches_nothing_keeps_the_files_metadata() {
    let meta = run_one_file(
        ECHO,
        file_carrying("a.txt", b"a", serde_json::json!({ "row": "r1" })),
    );
    assert_eq!(meta["row"], "r1");
}

#[test]
fn test_a_node_adds_its_metadata_to_what_the_file_carries() {
    let file = file_carrying("a.jpg", &[0u8; 100], serde_json::json!({ "pack": "Guns" }));
    let meta = run_one_file(COMPRESS, file);
    assert_eq!(meta["pack"], "Guns");
    assert_eq!(meta["compressedSize"], 50);
}

#[test]
fn test_a_nodes_value_wins_a_shared_key() {
    let file = file_carrying(
        "a.jpg",
        &[0u8; 100],
        serde_json::json!({ "originalSize": 7 }),
    );
    assert_eq!(run_one_file(COMPRESS, file)["originalSize"], 100);
}

#[test]
fn test_metadata_accumulates_inside_a_loop() {
    let def = r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [
                    { "id": "proc", "type": "test-metadata" },
                    { "id": "echo", "type": "test-echo" }
                ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#;
    let file = file_carrying("a.jpg", &[0u8; 100], serde_json::json!({ "row": "r1" }));
    let meta = run_one_file(def, file);
    assert_eq!(meta["row"], "r1");
    assert_eq!(meta["compressedSize"], 50);
}

#[test]
fn test_a_batch_node_output_carries_only_what_it_sets() {
    // A batch node receives each file's metadata (see batch_input.rs) and
    // decides what its outputs carry; the executor doesn't copy it for them.
    // The default per-file fallback (batch-echo) sets none.
    let def = r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "batch", "type": "test-batch" },
            { "id": "out", "type": "output" }
        ]
    }"#;
    let meta = run_one_file(
        def,
        file_carrying("a.txt", b"a", serde_json::json!({ "row": "r1" })),
    );
    assert!(!meta.contains_key("row"), "{meta:?}");
}

#[test]
fn test_inside_a_loop_a_childs_value_still_wins_on_the_file() {
    // The loop item wins for {{item.*}} reads; the file's own metadata keeps
    // what the child set.
    let def = r#"{
        "nodes": [
            { "id": "in", "type": "input" },
            {
                "id": "loop-1", "type": "loop",
                "children": [ { "id": "proc", "type": "test-metadata" } ]
            },
            { "id": "out", "type": "output" }
        ]
    }"#;
    let file = file_carrying(
        "a.jpg",
        &[0u8; 100],
        serde_json::json!({ "originalSize": 7 }),
    );
    assert_eq!(run_one_file(def, file)["originalSize"], 100);
}
