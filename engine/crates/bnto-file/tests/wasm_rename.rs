// =============================================================================
// WASM Integration Tests — Rename Files via the WASM Bridge (Combined API)
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that our WASM-exported `rename_file_combined` function works correctly
// when called from JavaScript. The combined function returns a single JS
// object with { metadata, data, filename, mimeType } — replacing the old
// dual-function pattern (rename_file + rename_file_bytes).
//
// This catches problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array <-> Vec<u8>, String <-> &str)
//   - WASM memory allocation issues
//   - JS callback interop (progress reporting across the boundary)
//   - Combined result object construction (Reflect.set across the boundary)
//
// HOW TO RUN:
//   These tests are included in `cargo test --workspace` (native) and
//   run as WASM integration tests via the bnto-wasm entry point crate.
//   Use `task wasm:test` to run them in the Node.js WASM environment.

mod common;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use bnto_file::wasm_bridge::*;
use common::{
    LARGER_TEST_DATA, TEST_FILE_DATA, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback, recording_callback,
};

// Configure tests to run in Node.js.
// RUST CONCEPT: `wasm_bindgen_test_configure!`
// This macro tells the test harness to run in Node.js (not the browser).
// Node.js is faster for CI/CD because it doesn't need to spin up a
// headless browser.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Pattern Rename Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_combined_with_pattern_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // Rename "photo.jpg" using a pattern with {{name}} and {{ext}}.
    // The combined function returns a JS object, not a plain string.
    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"pattern": "{{name}}-compressed.{{ext}}"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file_combined should succeed");

    // Extract the metadata JSON string from the combined result object.
    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(!metadata.is_empty(), "Metadata JSON should not be empty");

    // Verify the metadata contains the renamed filename.
    assert!(
        metadata.contains("photo-compressed.jpg"),
        "Metadata should contain the new filename: got '{}'",
        metadata
    );

    // Also verify the top-level filename property matches.
    let filename = extract_filename(&result_obj);
    assert_eq!(
        filename, "photo-compressed.jpg",
        "Top-level filename should be the renamed file"
    );
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

// =============================================================================
// Prefix / Suffix Tests (via WASM boundary)
// =============================================================================

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

    assert!(
        result.is_ok(),
        "rename_file_combined with prefix should succeed"
    );

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

    assert!(
        result.is_ok(),
        "rename_file_combined with suffix should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("data-backup.json"),
        "Metadata should contain suffixed filename: got '{}'",
        metadata
    );
}

// =============================================================================
// Find / Replace Tests (via WASM boundary)
// =============================================================================

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

    assert!(
        result.is_ok(),
        "rename_file_combined with find/replace should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    assert!(
        metadata.contains("new-report.txt"),
        "Metadata should contain replaced filename: got '{}'",
        metadata
    );
}

// =============================================================================
// Case Transformation Tests (via WASM boundary)
// =============================================================================

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

    assert!(
        result.is_ok(),
        "rename_file_combined with case=lower should succeed"
    );

    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);
    // Case transforms the stem only — extension stays as-is from the original.
    assert!(
        metadata.contains("important.TXT"),
        "Metadata should contain lowercased filename: got '{}'",
        metadata
    );
}

// =============================================================================
// Data Pass-Through Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_combined_bytes_pass_through_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // The returned bytes should be IDENTICAL to the input — rename only
    // changes the filename, not the file data. With the combined function,
    // we extract bytes from the result object's `data` property.
    let result = rename_file_combined(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "renamed-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file_combined should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert_eq!(
        bytes, TEST_FILE_DATA,
        "File data should pass through unchanged"
    );
}

#[wasm_bindgen_test]
fn test_rename_combined_larger_data_pass_through_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // Test with larger data to ensure no truncation or corruption
    // across the WASM boundary.
    let result = rename_file_combined(
        LARGER_TEST_DATA,
        "document.txt",
        r#"{"suffix": "-v2"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_combined should succeed with larger data"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert_eq!(
        bytes, LARGER_TEST_DATA,
        "Larger file data should pass through unchanged"
    );
}

// =============================================================================
// Error Handling Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_empty_filename_returns_js_error() {
    init_panic_hook();
    let callback = noop_callback();

    // An empty filename should produce an error from the combined function.
    let result = rename_file_combined(TEST_FILE_DATA, "", r#"{"prefix": "test-"}"#, callback);

    assert!(result.is_err(), "Should return an error for empty filename");
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();

    // Invalid JSON should not crash — it should use defaults (no transforms).
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
    // With no valid params, filename should pass through unchanged.
    assert!(
        metadata.contains("photo.jpg"),
        "Filename should be unchanged with invalid params: got '{}'",
        metadata
    );
}

// =============================================================================
// Progress Callback Tests (via WASM boundary)
// =============================================================================

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

    assert!(result.is_ok(), "rename_file_combined should succeed");

    // The progress callback should have been called at least once.
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

    assert!(result.is_ok(), "rename_file_combined should succeed");

    // The last progress call should have percent = 100.
    // WASM CONCEPT: Progress callbacks cross the JS boundary — each call
    // from Rust invokes the JS function with (percent, message). The
    // recording callback stores these as [percent, message] arrays.
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

// =============================================================================
// Metadata Tests (via WASM boundary)
// =============================================================================

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

    assert!(result.is_ok(), "rename_file_combined should succeed");

    // Extract metadata from the combined result's metadata property.
    let result_obj = result.unwrap();
    let metadata = extract_metadata(&result_obj);

    // The metadata should include both the original and new filenames.
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
