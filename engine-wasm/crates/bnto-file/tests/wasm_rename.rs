// =============================================================================
// WASM Integration Tests — Rename Files via the WASM Bridge
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that our WASM-exported functions work correctly when called from JavaScript.
// This catches problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array <-> Vec<u8>, String <-> &str)
//   - WASM memory allocation issues
//   - JS callback interop (progress reporting across the boundary)
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
    LARGER_TEST_DATA, TEST_FILE_DATA, init_panic_hook, noop_callback, recording_callback,
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
fn test_rename_with_pattern_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // Rename "photo.jpg" using a pattern with {{name}} and {{ext}}.
    let result = rename_file(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"pattern": "{{name}}-compressed.{{ext}}"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file should succeed");

    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "Result JSON should not be empty");

    // Verify the result JSON contains the renamed filename.
    assert!(
        json_str.contains("photo-compressed.jpg"),
        "Result should contain the new filename: got '{}'",
        json_str
    );
}

#[wasm_bindgen_test]
fn test_rename_with_pattern_index_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "document.pdf",
        r#"{"pattern": "file-{{index}}.{{ext}}", "index": "7"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file should succeed with index");

    let json_str = result.unwrap();
    assert!(
        json_str.contains("file-7.pdf"),
        "Result should contain indexed filename: got '{}'",
        json_str
    );
}

// =============================================================================
// Prefix / Suffix Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_with_prefix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "report.csv",
        r#"{"prefix": "final-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file with prefix should succeed");

    let json_str = result.unwrap();
    assert!(
        json_str.contains("final-report.csv"),
        "Result should contain prefixed filename: got '{}'",
        json_str
    );
}

#[wasm_bindgen_test]
fn test_rename_with_suffix_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "data.json",
        r#"{"suffix": "-backup"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file with suffix should succeed");

    let json_str = result.unwrap();
    assert!(
        json_str.contains("data-backup.json"),
        "Result should contain suffixed filename: got '{}'",
        json_str
    );
}

// =============================================================================
// Find / Replace Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_find_replace_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "old-report.txt",
        r#"{"find": "old", "replace": "new"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file with find/replace should succeed"
    );

    let json_str = result.unwrap();
    assert!(
        json_str.contains("new-report.txt"),
        "Result should contain replaced filename: got '{}'",
        json_str
    );
}

// =============================================================================
// Case Transformation Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_case_lower_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "IMPORTANT.TXT",
        r#"{"case": "lower"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file with case=lower should succeed");

    let json_str = result.unwrap();
    // Case transforms the stem only — extension stays as-is from the original.
    assert!(
        json_str.contains("important.TXT"),
        "Result should contain lowercased filename: got '{}'",
        json_str
    );
}

// =============================================================================
// Data Pass-Through Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_rename_bytes_pass_through_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // The returned bytes should be IDENTICAL to the input — rename only
    // changes the filename, not the file data.
    let result = rename_file_bytes(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "renamed-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file_bytes should succeed");

    let bytes = result.unwrap();
    assert_eq!(
        bytes, TEST_FILE_DATA,
        "File data should pass through unchanged"
    );
}

#[wasm_bindgen_test]
fn test_rename_larger_data_pass_through_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // Test with larger data to ensure no truncation or corruption.
    let result = rename_file_bytes(
        LARGER_TEST_DATA,
        "document.txt",
        r#"{"suffix": "-v2"}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "rename_file_bytes should succeed with larger data"
    );

    let bytes = result.unwrap();
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

    // An empty filename should produce an error.
    let result = rename_file_bytes(TEST_FILE_DATA, "", r#"{"prefix": "test-"}"#, callback);

    assert!(result.is_err(), "Should return an error for empty filename");
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();

    // Invalid JSON should not crash — it should use defaults (no transforms).
    let result = rename_file(
        TEST_FILE_DATA,
        "photo.jpg",
        "this is not valid json!!!",
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (uses defaults)"
    );

    let json_str = result.unwrap();
    // With no valid params, filename should pass through unchanged.
    assert!(
        json_str.contains("photo.jpg"),
        "Filename should be unchanged with invalid params: got '{}'",
        json_str
    );
}

// =============================================================================
// Progress Callback Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file should succeed");

    // The progress callback should have been called at least once.
    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}

#[wasm_bindgen_test]
fn test_progress_reaches_100() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file should succeed");

    // The last progress call should have percent = 100.
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
fn test_metadata_includes_original_and_new_filename() {
    init_panic_hook();
    let callback = noop_callback();

    let result = rename_file(
        TEST_FILE_DATA,
        "photo.jpg",
        r#"{"prefix": "new-"}"#,
        callback,
    );

    assert!(result.is_ok(), "rename_file should succeed");

    let json_str = result.unwrap();

    // The metadata should include both the original and new filenames.
    assert!(
        json_str.contains("originalFilename"),
        "Metadata should include originalFilename"
    );
    assert!(
        json_str.contains("newFilename"),
        "Metadata should include newFilename"
    );
    assert!(
        json_str.contains("transformsApplied"),
        "Metadata should include transformsApplied"
    );
}
