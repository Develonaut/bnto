// WASM Integration Tests -- rename CSV columns via combined WASM function.
//
// Tests the JS <-> Rust boundary for rename_csv_columns_combined().
// Native unit tests in rename_columns.rs verify pure Rust logic; these
// catch serialization and type-conversion bugs across WASM.

mod common;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use bnto_csv::wasm_bridge::*;
use common::{
    MANY_COLUMNS_CSV, MINIMAL_CSV, SIMPLE_CSV, extract_bytes, extract_filename, extract_metadata,
    extract_mime_type, init_panic_hook, noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// Basic Rename Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_columns_combined_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "rename_csv_columns_combined should succeed: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(!json_str.is_empty(), "Result JSON should not be empty");

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("test-renamed.csv"),
        "Output filename should be 'test-renamed.csv': got '{filename}'"
    );
    let mime = extract_mime_type(&result_obj);
    assert!(
        mime.contains("text/csv"),
        "MIME type should be text/csv: got '{mime}'"
    );

    assert!(
        json_str.contains("\"columnsRenamed\":1") || json_str.contains("\"columnsRenamed\": 1"),
        "Result should report 1 column renamed: got '{json_str}'"
    );
}

#[wasm_bindgen_test]
fn test_rename_columns_combined_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "rename_csv_columns_combined should succeed: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Output bytes should not be empty");

    let output_str = String::from_utf8(bytes).expect("Output should be valid UTF-8");
    assert!(
        output_str.starts_with("full_name,age,city\n"),
        "Headers should be renamed: got '{}'",
        output_str.lines().next().unwrap_or("")
    );
    assert!(
        output_str.contains("Alice,30,NYC"),
        "Data rows should be preserved: {output_str}"
    );
    assert!(
        output_str.contains("Bob,25,LA"),
        "Data rows should be preserved: {output_str}"
    );
}

// =========================================================================
// Data Preservation Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_data_preserved_after_rename_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        SIMPLE_CSV,
        "simple.csv",
        r#"{"columns": {"name": "full_name", "email": "email_address"}}"#,
        callback,
    );
    assert!(result.is_ok(), "Should succeed with SIMPLE_CSV fixture");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let output_str = String::from_utf8(bytes).expect("Should be valid UTF-8");

    assert!(
        output_str.contains("full_name"),
        "Should contain renamed header 'full_name'"
    );
    assert!(
        output_str.contains("email_address"),
        "Should contain renamed header 'email_address'"
    );
    assert!(output_str.contains("Alice"), "Should preserve 'Alice' data");
    assert!(
        output_str.contains("alice@example.com"),
        "Should preserve email data"
    );
}

#[wasm_bindgen_test]
fn test_many_columns_rename_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        MANY_COLUMNS_CSV,
        "many-columns.csv",
        r#"{"columns": {"first_name": "given_name", "last_name": "surname", "department": "team"}}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "Should succeed with MANY_COLUMNS_CSV fixture"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let output_str = String::from_utf8(bytes).expect("Should be valid UTF-8");

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

// =========================================================================
// Edge Case Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_missing_columns_ignored_combined_via_wasm() {
    // Mapping references a column that doesn't exist -- should succeed, column ignored.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"nonexistent_column": "something_else"}}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "Should succeed even when mapped column doesn't exist"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let output_str = String::from_utf8(bytes).expect("Should be valid UTF-8");
    assert!(
        output_str.starts_with("name,age,city\n"),
        "Headers should be unchanged when mapped column doesn't exist"
    );
}

#[wasm_bindgen_test]
fn test_invalid_params_json_passthrough_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_csv_columns_combined(
        MINIMAL_CSV,
        "test.csv",
        "this is not valid json at all!!!",
        callback,
    );
    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (passthrough)"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let output_str = String::from_utf8(bytes).expect("Should be valid UTF-8");
    assert!(
        output_str.starts_with("name,age,city\n"),
        "Should pass through unchanged with invalid JSON"
    );
}

#[wasm_bindgen_test]
fn test_non_utf8_returns_js_error_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();
    let bad_bytes: &[u8] = &[0xFF, 0xFE, 0x00, 0x61];
    let result = rename_csv_columns_combined(bad_bytes, "bad.csv", "{}", callback);
    assert!(result.is_err(), "Should return an error for non-UTF8 input");
}

// =========================================================================
// Progress Callback Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_combined_via_wasm() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_csv_columns_combined(
        MINIMAL_CSV,
        "test.csv",
        r#"{"columns": {"name": "full_name"}}"#,
        callback,
    );
    assert!(result.is_ok(), "Should succeed");

    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );

    // Last call should be at 100%.
    let last_call = calls.get(calls.length() - 1);
    let last_call_arr: js_sys::Array = last_call.dyn_into().expect("Call should be an array");
    let last_percent = last_call_arr
        .get(0)
        .as_f64()
        .expect("Percent should be a number");
    assert_eq!(last_percent, 100.0, "Last progress call should be 100%");
}
