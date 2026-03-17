// WASM Integration Tests -- rename files via the WASM bridge (combined API).
//
// Tests the JS <-> Rust boundary for rename_file_combined(). Catches
// wasm-bindgen type conversion, memory, and callback interop issues.

mod common;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use bnto_file::wasm_bridge::*;
use common::{
    LARGER_TEST_DATA, TEST_FILE_DATA, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// Pattern Rename Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_combined_with_pattern_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"pattern": "{{name}}-compressed.{{ext}}"}"#,
        callback,
    );
    assert!(result.is_ok(), "rename_file_combined should succeed");

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("photo-compressed.jpg"),
        "Metadata should contain the new filename: got '{}'",
        metadata
    );

    let filename = extract_filename(&result_obj);
    assert_eq!(filename, "photo-compressed.jpg");
}

#[wasm_bindgen_test]
fn test_rename_combined_with_pattern_index_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "document.pdf",
        r#"{"pattern": "file-{{index}}.{{ext}}", "index": "7"}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "rename_file_combined should succeed with index"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("file-7.pdf"),
        "Metadata should contain indexed filename: got '{}'",
        metadata
    );
}

// =========================================================================
// Prefix / Suffix Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_combined_with_prefix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "report.csv",
        r#"{"prefix": "final-"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("final-report.csv"),
        "Metadata should contain prefixed filename: got '{}'",
        metadata
    );
}

#[wasm_bindgen_test]
fn test_rename_combined_with_suffix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "data.json",
        r#"{"suffix": "-backup"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("data-backup.json"),
        "Metadata should contain suffixed filename: got '{}'",
        metadata
    );
}

// =========================================================================
// Find / Replace Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_combined_find_replace_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "old-report.txt",
        r#"{"find": "old", "replace": "new"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("new-report.txt"),
        "Metadata should contain replaced filename: got '{}'",
        metadata
    );
}

// =========================================================================
// Case Transformation Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_combined_case_lower_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "IMPORTANT.TXT",
        r#"{"case": "lower"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    // Case transforms the stem only -- extension stays as-is from the original.
    assert!(
        metadata.contains("important.TXT"),
        "Metadata should contain lowercased filename: got '{}'",
        metadata
    );
}

// =========================================================================
// Data Pass-Through Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_rename_combined_bytes_pass_through_via_wasm() {
    // Rename only changes the filename, not the file data.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "renamed-"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert_eq!(
        bytes, TEST_FILE_DATA,
        "File data should pass through unchanged"
    );
}

#[wasm_bindgen_test]
fn test_rename_combined_larger_data_pass_through_via_wasm() {
    // Verify no truncation or corruption with bigger payloads.
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        LARGER_TEST_DATA,
        "document.txt",
        r#"{"suffix": "-v2"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert_eq!(
        bytes, LARGER_TEST_DATA,
        "Larger file data should pass through unchanged"
    );
}

// =========================================================================
// Error Handling Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_empty_filename_returns_js_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = rename_file_combined(TEST_FILE_DATA, "", r#"{"prefix": "test-"}"#, callback);
    assert!(result.is_err(), "Should return an error for empty filename");
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        "this is not valid json!!!",
        callback,
    );
    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (uses defaults)"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("photo.jpg"),
        "Filename should be unchanged with invalid params: got '{}'",
        metadata
    );
}

// =========================================================================
// Progress Callback Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_combined() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );
    assert!(result.is_ok());

    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}

#[wasm_bindgen_test]
fn test_progress_reaches_100_combined() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );
    assert!(result.is_ok());

    let last_call = calls.get(calls.length() - 1);
    let last_arr = last_call
        .dyn_into::<js_sys::Array>()
        .expect("Each call should be an array");
    let last_percent = last_arr
        .get(0)
        .as_f64()
        .expect("Percent should be a number");

    assert_eq!(
        last_percent, 100.0,
        "Last progress call should be 100%, got {}",
        last_percent
    );
}

// =========================================================================
// Metadata Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_metadata_includes_original_and_new_filename_combined() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);

    assert!(
        metadata.contains("originalFilename"),
        "Metadata should include originalFilename"
    );
    assert!(
        metadata.contains("newFilename"),
        "Metadata should include newFilename"
    );
    assert!(
        metadata.contains("transformsApplied"),
        "Metadata should include transformsApplied"
    );
}
