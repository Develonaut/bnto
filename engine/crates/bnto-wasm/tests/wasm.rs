// =============================================================================
// WASM Integration Tests — Full Pipeline Tests in Node.js
// =============================================================================
//
// WHAT IS THIS FILE?
// These tests run inside a real JavaScript environment (Node.js) to verify
// the full WASM boundary works. They test:
//   1. WASM binary loads correctly
//   2. setup() / version() / greet() work across JS ↔ Rust
//   3. Image compression combined function works end-to-end via wasm-bindgen
//
// HOW TO RUN:
//   wasm-pack test --node crates/bnto-wasm
//
// This compiles the entry point crate to WASM, starts Node.js, loads
// the WASM binary, and runs these tests inside the JS runtime.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

// Import the entry point crate's public API.
// RUST CONCEPT: Crate names replace hyphens with underscores.
// "bnto-wasm" becomes "bnto_wasm" in Rust code.
use bnto_wasm::*;

// Run tests in Node.js (not a browser).
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Setup Tests
// =============================================================================

/// Verify setup() runs without panicking in Node.js.
/// This proves the WASM binary loaded and the panic hook installed.
#[wasm_bindgen_test]
fn test_setup_does_not_panic() {
    setup();
}

/// Verify setup() is idempotent — safe to call multiple times.
#[wasm_bindgen_test]
fn test_setup_is_idempotent() {
    setup();
    setup();
    setup();
}

// =============================================================================
// Version Tests
// =============================================================================

/// Verify version() returns a semver string across the WASM boundary.
#[wasm_bindgen_test]
fn test_version_across_wasm_boundary() {
    let v = version();
    assert!(!v.is_empty(), "Version should not be empty");
    assert!(v.contains('.'), "Version should be semver: got '{}'", v);
}

// =============================================================================
// Greet Tests — String Round-Trip Verification
// =============================================================================

/// Verify greet() works with ASCII names.
#[wasm_bindgen_test]
fn test_greet_with_ascii_name() {
    let result = greet("Ryan");
    assert!(
        result.contains("Ryan"),
        "Should contain name: got '{}'",
        result
    );
}

/// Verify greet() handles Unicode (multi-byte UTF-8) correctly.
/// WASM string handling can be tricky with non-ASCII characters.
#[wasm_bindgen_test]
fn test_greet_with_unicode_name() {
    let result = greet("Bnto \u{1F371}"); // Bento box emoji!
    assert!(
        result.contains("Bnto \u{1F371}"),
        "Should handle Unicode: got '{}'",
        result
    );
}

/// Verify greet() handles empty strings gracefully.
#[wasm_bindgen_test]
fn test_greet_with_empty_name() {
    let result = greet("");
    assert!(
        result.contains("Bnto WASM engine"),
        "Should still mention engine: got '{}'",
        result
    );
}

// =============================================================================
// Image Compression Tests (via bnto-image combined bridge)
// =============================================================================
//
// These tests verify that bnto-image's compress_image_combined function works
// when called through the unified entry point's WASM binary.
//
// The combined function returns a JS object with four properties:
//   - metadata: JSON string with compression stats
//   - data: Uint8Array of compressed bytes
//   - filename: suggested output filename (string)
//   - mimeType: MIME type of the output (string)
//
// We use js_sys::Reflect to extract properties from the returned JsValue,
// and JsCast to downcast the data property to a Uint8Array.
//
// We embed test fixture images at compile time with include_bytes!() so
// tests work in any environment (no filesystem access needed in WASM).

/// Embed a small test JPEG at compile time.
/// This fixture is shared with the Go engine tests (test-fixtures/images/).
const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

/// Test that compress_image_combined() returns valid metadata in the result object.
///
/// This tests the full pipeline: JS bytes → Rust decode → compress → combined
/// JS object with metadata JSON string, compressed bytes, filename, and mimeType.
/// We use js_sys::Function::new_no_args to create a dummy progress callback.
#[wasm_bindgen_test]
fn test_compress_image_combined_returns_metadata() {
    // Create a no-op progress callback (function that does nothing).
    // In the real Web Worker, this would call postMessage().
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    // Call the combined compress function. This goes through bnto-image's wasm_bridge.
    let result = bnto_image::wasm_bridge::compress_image_combined(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );

    // Verify it succeeded (not an error).
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed: {:?}",
        result.err()
    );

    // --- Extract the metadata property from the returned JS object ---
    //
    // RUST CONCEPT: js_sys::Reflect::get()
    // This is the Rust equivalent of JavaScript's Reflect.get(obj, key).
    // It lets us read properties from a JsValue (a generic JS object)
    // without knowing its concrete type at compile time.
    let result_obj = result.unwrap();
    let metadata_val = js_sys::Reflect::get(&result_obj, &"metadata".into())
        .expect("Result should have 'metadata' property");

    // The metadata property is a JSON string — convert it from JsValue to Rust String.
    let metadata_str = metadata_val
        .as_string()
        .expect("metadata should be a string");

    // Verify the metadata is valid JSON.
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&metadata_str);
    assert!(
        parsed.is_ok(),
        "metadata should be valid JSON: {}",
        metadata_str
    );

    // Verify the JSON has expected fields.
    let value = parsed.unwrap();
    assert!(value.get("file").is_some(), "JSON should have 'file' field");
    assert!(
        value.get("metadata").is_some(),
        "JSON should have 'metadata' field"
    );
}

/// Test that compress_image_combined() returns raw compressed bytes in the data property.
///
/// This verifies the efficient path: the data property is a Uint8Array that the
/// Web Worker can turn directly into a Blob for download. No JSON parsing needed
/// for the file data itself.
#[wasm_bindgen_test]
fn test_compress_image_combined_returns_data() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result = bnto_image::wasm_bridge::compress_image_combined(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );

    assert!(
        result.is_ok(),
        "compress_image_combined should succeed: {:?}",
        result.err()
    );

    // --- Extract the data property from the returned JS object ---
    //
    // RUST CONCEPT: JsCast::dyn_into()
    // The data property is a JsValue, but we know it's actually a Uint8Array.
    // dyn_into() is Rust's runtime type cast for JS types — like JavaScript's
    // `value instanceof Uint8Array`. If the cast fails, it returns Err.
    let result_obj = result.unwrap();
    let data_val = js_sys::Reflect::get(&result_obj, &"data".into())
        .expect("Result should have 'data' property");

    // Cast the JsValue to a Uint8Array so we can read the bytes.
    let uint8_array: js_sys::Uint8Array = data_val
        .dyn_into::<js_sys::Uint8Array>()
        .expect("data should be a Uint8Array");

    // Convert the Uint8Array to a Rust Vec<u8> for inspection.
    let bytes = uint8_array.to_vec();

    // Compressed output should be non-empty.
    assert!(!bytes.is_empty(), "Compressed bytes should not be empty");
    // Verify the output starts with JPEG magic bytes (FF D8 FF).
    assert_eq!(bytes[0], 0xFF, "Should start with JPEG magic byte 1");
    assert_eq!(bytes[1], 0xD8, "Should start with JPEG magic byte 2");
}

/// Test that compressing with default params (empty JSON) works via combined function.
/// The compression engine should use sensible defaults when no
/// quality is specified.
#[wasm_bindgen_test]
fn test_compress_combined_with_default_params() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result =
        bnto_image::wasm_bridge::compress_image_combined(TEST_JPEG, "small.jpg", "{}", noop_cb);

    assert!(
        result.is_ok(),
        "Default params should work: {:?}",
        result.err()
    );
}

/// Test that compressing an invalid file returns an error (not a panic).
/// The WASM module should handle errors gracefully and return them
/// as JavaScript Error objects, not crash the entire Web Worker.
#[wasm_bindgen_test]
fn test_compress_combined_invalid_data_returns_error() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    // Send garbage data that isn't a valid image.
    let result = bnto_image::wasm_bridge::compress_image_combined(
        b"this is not an image",
        "garbage.jpg",
        "{}",
        noop_cb,
    );

    // Should be an error, not a panic.
    assert!(result.is_err(), "Invalid data should return an error");
}
