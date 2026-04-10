// WASM Integration Tests -- spreadsheet cleaning via the combined WASM function.
//
// Tests the JS <-> Rust boundary for clean_spreadsheet_combined(). Native unit
// tests in clean.rs verify pure Rust logic; these catch serialization
// and type-conversion bugs across WASM.

mod common;

use wasm_bindgen_test::*;

use bnto_spreadsheet::wasm_bridge::*;
use common::{
    CSV_WITH_DUPLICATES, CSV_WITH_EMPTY_ROWS, HEADERS_ONLY_CSV, MESSY_CSV, SIMPLE_CSV,
    extract_bytes, extract_filename, extract_metadata, extract_mime_type, init_panic_hook,
    noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// Basic CSV Cleaning Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_clean_simple_csv_combined_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(SIMPLE_CSV, "simple.csv", "{}", callback);
    assert!(
        result.is_ok(),
        "clean_spreadsheet_combined should succeed for simple CSV"
    );

    let result_obj = result.unwrap();

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
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(SIMPLE_CSV, "simple.csv", "{}", callback);
    assert!(
        result.is_ok(),
        "clean_spreadsheet_combined should succeed for simple CSV"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Cleaned CSV bytes should not be empty");

    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");
    assert!(text.contains("name"), "Should contain 'name' header");
    assert!(text.contains("age"), "Should contain 'age' header");
    assert!(text.contains("city"), "Should contain 'city' header");
}

// =========================================================================
// Empty Rows Removed Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_empty_rows_removed_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(CSV_WITH_EMPTY_ROWS, "data.csv", "{}", callback);
    assert!(result.is_ok(), "clean_spreadsheet_combined should succeed");

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        json_str.contains("\"rowsRemoved\""),
        "Metadata should include rowsRemoved: got '{}'",
        json_str
    );
}

// =========================================================================
// Duplicates Removed Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_duplicates_removed_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(CSV_WITH_DUPLICATES, "data.csv", "{}", callback);
    assert!(result.is_ok(), "clean_spreadsheet_combined should succeed");

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        json_str.contains("\"duplicatesRemoved\""),
        "Metadata should include duplicatesRemoved: got '{}'",
        json_str
    );
}

// =========================================================================
// Messy CSV Full Cleaning Test
// =========================================================================

#[wasm_bindgen_test]
fn test_clean_messy_csv_combined_via_wasm() {
    // Messy fixture has whitespace, empty rows, and duplicates.
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(MESSY_CSV, "messy.csv", "{}", callback);
    assert!(
        result.is_ok(),
        "clean_spreadsheet_combined should succeed for messy CSV"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");

    assert!(text.contains("Alice"), "Should still have Alice");
    assert!(text.contains("Bob"), "Should still have Bob");
    assert!(text.contains("Charlie"), "Should still have Charlie");
    assert!(text.contains("Diana"), "Should still have Diana");
}

// =========================================================================
// Headers-Only CSV Test
// =========================================================================

#[wasm_bindgen_test]
fn test_headers_only_csv_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = clean_spreadsheet_combined(HEADERS_ONLY_CSV, "empty.csv", "{}", callback);
    assert!(
        result.is_ok(),
        "clean_spreadsheet_combined should succeed for headers-only CSV"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let text = String::from_utf8(bytes).expect("Output should be valid UTF-8");
    assert!(text.contains("name"), "Should contain header 'name'");
}

// =========================================================================
// Error Handling Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_empty_input_returns_js_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = clean_spreadsheet_combined(b"", "empty.csv", "{}", callback);
    assert!(result.is_err(), "Should return an error for empty input");
}

#[wasm_bindgen_test]
fn test_non_utf8_input_returns_js_error() {
    init_panic_hook();
    let callback = noop_callback();
    let bad_bytes: &[u8] = &[0xFF, 0xFE, 0x00, 0x41];
    let result = clean_spreadsheet_combined(bad_bytes, "binary.csv", "{}", callback);
    assert!(result.is_err(), "Should return an error for non-UTF8 input");
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();
    let result = clean_spreadsheet_combined(
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

// =========================================================================
// Progress Callback Test
// =========================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_combined() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = clean_spreadsheet_combined(SIMPLE_CSV, "data.csv", "{}", callback);
    assert!(result.is_ok(), "clean_spreadsheet_combined should succeed");

    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}
