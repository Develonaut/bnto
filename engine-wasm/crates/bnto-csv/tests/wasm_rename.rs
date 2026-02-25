// =============================================================================
// WASM Integration Tests — Rename CSV Columns via WASM Boundary
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that our WASM-exported functions work correctly when called from JavaScript.
// This catches problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array ↔ Vec<u8>, String ↔ &str)
//   - WASM memory allocation issues
//   - JS callback interop (progress reporting across the boundary)
//
// HOW TO RUN:
//   These tests are included in `task wasm:test` and run via wasm-pack
//   in a Node.js environment.
//
// NATIVE UNIT TESTS:
//   The thorough unit tests are in `src/rename_columns.rs`. This file
//   focuses on verifying the WASM boundary works correctly — that data
//   crosses from JS to Rust and back without corruption.

mod common;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use bnto_csv::wasm_bridge::*;
use common::{
    MANY_COLUMNS_CSV, MINIMAL_CSV, SIMPLE_CSV, init_panic_hook, noop_callback, recording_callback,
};

// Configure tests to run in Node.js (not a browser).
// This is faster and doesn't require a headless browser setup.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Basic Rename Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_columns_via_wasm_metadata() {
    // Test the metadata-returning variant of the WASM function.
    // We rename one column and verify the JSON result contains the expected metadata.
    init_panic_hook();
    let callback = noop_callback();

    // Rename "name" to "full_name" in our minimal CSV.
    let result = rename_csv_columns(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );

    // The function should succeed.
    assert!(
        result.is_ok(),
        "rename_csv_columns should succeed: {:?}",
        result.err()
    );

    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "Result JSON should not be empty");

    // The result should contain the renamed filename.
    assert!(
        json_str.contains("test-renamed.csv"),
        "Result should contain renamed filename: got '{json_str}'"
    );

    // The result should contain MIME type.
    assert!(
        json_str.contains("text/csv"),
        "Result should contain text/csv MIME type: got '{json_str}'"
    );

    // The result should report 1 column renamed.
    assert!(
        json_str.contains("\"columnsRenamed\":1") || json_str.contains("\"columnsRenamed\": 1"),
        "Result should report 1 column renamed: got '{json_str}'"
    );
}

#[wasm_bindgen_test]
fn test_rename_columns_via_wasm_bytes() {
    // Test the bytes-returning variant — the efficient path for downloads.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_bytes(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_csv_columns_bytes should succeed: {:?}",
        result.err()
    );

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Output bytes should not be empty");

    // Parse the output as a string to verify content.
    let output_str = String::from_utf8(bytes).expect("Output should be valid UTF-8");

    // Header should be renamed.
    assert!(
        output_str.starts_with("full_name,age,city\n"),
        "Headers should be renamed: got '{}'",
        output_str.lines().next().unwrap_or("")
    );

    // Data rows should be preserved.
    assert!(
        output_str.contains("Alice,30,NYC"),
        "Data rows should be preserved: {output_str}"
    );
    assert!(
        output_str.contains("Bob,25,LA"),
        "Data rows should be preserved: {output_str}"
    );
}

// =============================================================================
// Data Preservation Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_data_preserved_after_rename_via_wasm() {
    // Verify that data rows survive the WASM boundary intact.
    // Use the larger SIMPLE_CSV fixture to test with real file data.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_bytes(
        SIMPLE_CSV,
        "simple.csv",
        r#"{"columns": {"name": "full_name", "email": "email_address"}}"#,
        callback,
    );

    assert!(result.is_ok(), "Should succeed with SIMPLE_CSV fixture");

    let output_str = String::from_utf8(result.unwrap()).expect("Should be valid UTF-8");

    // Header should be renamed.
    assert!(
        output_str.contains("full_name"),
        "Should contain renamed header 'full_name'"
    );
    assert!(
        output_str.contains("email_address"),
        "Should contain renamed header 'email_address'"
    );

    // Data should be preserved.
    assert!(output_str.contains("Alice"), "Should preserve 'Alice' data");
    assert!(
        output_str.contains("alice@example.com"),
        "Should preserve email data"
    );
}

#[wasm_bindgen_test]
fn test_many_columns_rename_via_wasm() {
    // Test with the many-columns fixture (8 columns).
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_bytes(
        MANY_COLUMNS_CSV,
        "many-columns.csv",
        r#"{"columns": {"first_name": "given_name", "last_name": "surname", "department": "team"}}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed with MANY_COLUMNS_CSV fixture"
    );

    let output_str = String::from_utf8(result.unwrap()).expect("Should be valid UTF-8");

    // Renamed columns should appear.
    assert!(
        output_str.contains("given_name"),
        "Should have renamed 'first_name' to 'given_name'"
    );
    assert!(
        output_str.contains("surname"),
        "Should have renamed 'last_name' to 'surname'"
    );
    assert!(
        output_str.contains("team"),
        "Should have renamed 'department' to 'team'"
    );

    // Unrenamed columns should be preserved.
    assert!(
        output_str.contains("email"),
        "Should preserve 'email' column"
    );
    assert!(
        output_str.contains("phone"),
        "Should preserve 'phone' column"
    );
    assert!(output_str.contains("city"), "Should preserve 'city' column");
}

// =============================================================================
// Edge Case Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_missing_columns_ignored_via_wasm() {
    // Mapping references a column that doesn't exist in the CSV.
    // Should succeed without error — the nonexistent column is just ignored.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_bytes(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"nonexistent_column": "something_else"}}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed even when mapped column doesn't exist"
    );

    let output_str = String::from_utf8(result.unwrap()).expect("Should be valid UTF-8");

    // Headers should be unchanged since "nonexistent_column" isn't in the CSV.
    assert!(
        output_str.starts_with("name,age,city\n"),
        "Headers should be unchanged when mapped column doesn't exist"
    );
}

#[wasm_bindgen_test]
fn test_invalid_params_json_passthrough_via_wasm() {
    // If the params JSON is invalid, the function should still succeed
    // (no renames applied — passthrough mode).
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_bytes(
        MINIMAL_CSV,
        "test.csv",
        "this is not valid json at all!!!",
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (passthrough)"
    );

    let output_str = String::from_utf8(result.unwrap()).expect("Should be valid UTF-8");

    // Headers unchanged — invalid JSON means no renames.
    assert!(
        output_str.starts_with("name,age,city\n"),
        "Should pass through unchanged with invalid JSON"
    );
}

#[wasm_bindgen_test]
fn test_non_utf8_returns_js_error_via_wasm() {
    // Non-UTF8 input should return a JavaScript error, not a panic.
    init_panic_hook();
    let callback = noop_callback();

    // Create invalid UTF-8 bytes.
    let bad_bytes: &[u8] = &[0xFF, 0xFE, 0x00, 0x61];

    let result = rename_csv_columns_bytes(bad_bytes, "bad.csv", "{}", callback);

    assert!(result.is_err(), "Should return an error for non-UTF8 input");
}

// =============================================================================
// Progress Callback Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_via_wasm() {
    // Verify that the progress callback is actually called during processing.
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_csv_columns_bytes(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );

    assert!(result.is_ok(), "Should succeed");

    // The progress callback should have been called at least once.
    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );

    // The last call should be at 100% (done).
    // Each call is an array [percent, message].
    let last_call = calls.get(calls.length() - 1);
    let last_call_arr: js_sys::Array = last_call.dyn_into().expect("Call should be an array");
    let last_percent = last_call_arr
        .get(0)
        .as_f64()
        .expect("Percent should be a number");
    assert_eq!(last_percent, 100.0, "Last progress call should be 100%");
}
