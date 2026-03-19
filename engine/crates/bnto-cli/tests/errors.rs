// Error path tests — malformed definitions, missing files, unknown node types.

mod common;

use common::{make_file, run_pipeline};

static SMALL_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

#[test]
fn invalid_json_produces_error() {
    let result = run_pipeline("not valid json {{{", vec![]);
    assert!(result.is_err());
    assert!(
        result.unwrap_err().contains("expected"),
        "Error should mention parse failure"
    );
}

#[test]
fn empty_nodes_array_succeeds() {
    let def = r#"{ "nodes": [] }"#;
    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files);
    // Empty pipeline — no processing, result should be the same input files.
    assert!(result.is_ok());
}

#[test]
fn unknown_node_type_produces_error() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "quantum", "type": "quantum", "parameters": { "operation": "entangle" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files);

    assert!(result.is_err(), "Unknown node type should fail");
    let err = result.unwrap_err();
    assert!(
        err.to_lowercase().contains("quantum")
            || err.to_lowercase().contains("not found")
            || err.to_lowercase().contains("no processor"),
        "Error should reference the unknown type, got: {}",
        err
    );
}

#[test]
fn unknown_operation_produces_error() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "teleport", "type": "image", "parameters": { "operation": "teleport" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files);

    assert!(result.is_err(), "Unknown operation should fail");
}

#[test]
fn io_only_pipeline_passes_files_through() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files).unwrap();

    // I/O nodes are skipped — no processing means the files come back as-is.
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].data.len(), SMALL_JPG.len());
}

#[test]
fn missing_definition_fields_still_deserialize() {
    // Minimal valid definition — just nodes with type and id.
    let def = r#"{ "nodes": [{ "id": "input", "type": "input" }, { "id": "output", "type": "output" }] }"#;
    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files);
    assert!(result.is_ok());
}
