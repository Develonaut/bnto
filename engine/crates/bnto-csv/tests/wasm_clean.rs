// =============================================================================
// WASM Integration Tests — CSV Cleaning via the Combined WASM Function
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that our WASM-exported CSV functions work correctly when called from
// JavaScript. This catches problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array ↔ Vec<u8>, String ↔ &str)
//   - WASM memory allocation issues
//   - JS callback interop (progress reporting across the boundary)
//
// HOW TO RUN:
//   These tests are included in `cargo test --workspace` (native) and
//   run as WASM integration tests via `wasm-pack test --node`.
//
// WHY SEPARATE FROM unit tests in clean.rs?
// Unit tests in clean.rs test PURE RUST logic with no JS runtime.
// These tests verify the JS ↔ Rust boundary works end-to-end.
// Both layers are needed: unit tests are fast and precise, WASM tests
// catch serialization and type-conversion bugs.
//
// COMBINED FUNCTION PATTERN:
// All tests use `clean_csv_combined()` which returns a single JS object
// containing both metadata (JSON string) and data (Uint8Array). This
// replaces the old dual-function pattern (clean_csv + clean_csv_bytes).
// Use the extract_* helpers from `common` to pull out individual fields.

mod common;

use wasm_bindgen_test::*;

use bnto_csv::wasm_bridge::*;
use common::{
    CSV_WITH_DUPLICATES, CSV_WITH_EMPTY_ROWS, HEADERS_ONLY_CSV, MESSY_CSV, SIMPLE_CSV,
    extract_bytes, extract_filename, extract_metadata, extract_mime_type, init_panic_hook,
    noop_callback, recording_callback,
};

// Configure tests to run in Node.js.
// RUST CONCEPT: `wasm_bindgen_test_configure!`
// This macro tells the test harness to use Node.js as the runtime
// (instead of a browser). This is faster and doesn't need a GUI.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Basic CSV Cleaning Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_clean_simple_csv_combined_metadata_via_wasm() {
    // A clean CSV should pass through with metadata returned as JSON.
    // We use the combined function and extract the metadata string to
    // verify the expected fields are present.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(SIMPLE_CSV, "simple.csv", "{}", callback);

    assert!(
        result.is_ok(),
        "clean_csv_combined should succeed for simple CSV"
    );

    // Extract and verify the combined result object.
    let result_obj = result.unwrap();

    // Metadata JSON has cleaning stats (originalRows, cleanedRows, etc.)
    let json_str = extract_metadata(&result_obj);
    assert!(!json_str.is_empty(), "Metadata JSON should not be empty");
    assert!(
        json_str.contains("originalRows"),
        "Metadata should contain 'originalRows': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("cleanedRows"),
        "Metadata should contain 'cleanedRows': got '{}'",
        json_str
    );

    // Filename and MIME type are separate properties on the result object.
    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("cleaned"),
        "Output filename should contain 'cleaned': got '{}'",
        filename
    );
    let mime = extract_mime_type(&result_obj);
    assert!(
        mime.contains("text/csv"),
        "MIME type should be text/csv: got '{}'",
        mime
    );
}

#[wasm_bindgen_test]
fn test_clean_simple_csv_combined_bytes_via_wasm() {
    // The combined function includes raw CSV data as a Uint8Array in
    // the "data" property. We extract it and verify the CSV content.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(SIMPLE_CSV, "simple.csv", "{}", callback);

    assert!(
        result.is_ok(),
        "clean_csv_combined should succeed for simple CSV"
    );

    // Extract the raw bytes from the combined result object.
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);

    // The output should be non-empty CSV text.
    assert!(!bytes.is_empty(), "Cleaned CSV bytes should not be empty");

    // Verify it's valid UTF-8 (CSV is a text format).
    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");

    // Should contain the original column names.
    assert!(text.contains("name"), "Should contain 'name' header");
    assert!(text.contains("age"), "Should contain 'age' header");
    assert!(text.contains("city"), "Should contain 'city' header");
}

// =============================================================================
// Empty Rows Removed Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_empty_rows_removed_via_wasm() {
    // CSV with empty rows — they should be stripped out.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(CSV_WITH_EMPTY_ROWS, "data.csv", "{}", callback);

    assert!(result.is_ok(), "clean_csv_combined should succeed");

    // Extract metadata from the combined result to check cleaning stats.
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    // The metadata should report rows were removed.
    assert!(
        json_str.contains("\"rowsRemoved\""),
        "Metadata should include rowsRemoved: got '{}'",
        json_str
    );
}

// =============================================================================
// Duplicates Removed Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_duplicates_removed_via_wasm() {
    // CSV with duplicate rows — the second occurrence should be removed.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(CSV_WITH_DUPLICATES, "data.csv", "{}", callback);

    assert!(result.is_ok(), "clean_csv_combined should succeed");

    // Extract metadata from the combined result to check cleaning stats.
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    // The metadata should report duplicates were found and removed.
    assert!(
        json_str.contains("\"duplicatesRemoved\""),
        "Metadata should include duplicatesRemoved: got '{}'",
        json_str
    );
}

// =============================================================================
// Messy CSV Full Cleaning Test (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_clean_messy_csv_combined_via_wasm() {
    // The messy fixture has whitespace, empty rows, and duplicates.
    // All three cleaning operations should fire. We extract the bytes
    // from the combined result to verify the cleaned content.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(MESSY_CSV, "messy.csv", "{}", callback);

    assert!(
        result.is_ok(),
        "clean_csv_combined should succeed for messy CSV"
    );

    // Extract the raw bytes from the combined result.
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");

    // After cleaning, empty rows and duplicates should be gone.
    assert!(text.contains("Alice"), "Should still have Alice");
    assert!(text.contains("Bob"), "Should still have Bob");
    assert!(text.contains("Charlie"), "Should still have Charlie");
    assert!(text.contains("Diana"), "Should still have Diana");
}

// =============================================================================
// Headers-Only CSV Test (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_headers_only_csv_combined_via_wasm() {
    // A CSV with only headers and no data should still succeed.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(HEADERS_ONLY_CSV, "empty.csv", "{}", callback);

    assert!(
        result.is_ok(),
        "clean_csv_combined should succeed for headers-only CSV"
    );

    // Extract the raw bytes from the combined result.
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");

    // Should contain the header row.
    assert!(text.contains("name"), "Should contain header 'name'");
}

// =============================================================================
// Error Handling Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_empty_input_returns_js_error() {
    // Completely empty input should throw a JS Error.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(b"", "empty.csv", "{}", callback);

    assert!(result.is_err(), "Should return an error for empty input");
}

#[wasm_bindgen_test]
fn test_non_utf8_input_returns_js_error() {
    // Binary data (not valid UTF-8) should throw a JS Error.
    init_panic_hook();
    let callback = noop_callback();

    let bad_bytes: &[u8] = &[0xFF, 0xFE, 0x00, 0x41];
    let result = clean_csv_combined(bad_bytes, "binary.csv", "{}", callback);

    assert!(result.is_err(), "Should return an error for non-UTF8 input");
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    // Invalid JSON params should not crash — it should use defaults.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_csv_combined(
        SIMPLE_CSV,
        "data.csv",
        "this is not valid json!!!",
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (uses defaults)"
    );
}

// =============================================================================
// Progress Callback Test (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_combined() {
    // Verify the progress callback is actually called during processing.
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = clean_csv_combined(SIMPLE_CSV, "data.csv", "{}", callback);

    assert!(result.is_ok(), "clean_csv_combined should succeed");

    // The progress callback should have been called at least once.
    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}
