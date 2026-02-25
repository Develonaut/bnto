// =============================================================================
// WASM Integration Tests — Full Pipeline Tests in Node.js
// =============================================================================
//
// WHAT IS THIS FILE?
// These tests run inside a real JavaScript environment (Node.js) to verify
// the full WASM boundary works. They test:
//   1. WASM binary loads correctly
//   2. setup() / version() / greet() work across JS ↔ Rust
//   3. Image compression functions work end-to-end via wasm-bindgen
//
// HOW TO RUN:
//   wasm-pack test --node crates/bnto-wasm
//
// This compiles the entry point crate to WASM, starts Node.js, loads
// the WASM binary, and runs these tests inside the JS runtime.

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
// Image Compression Tests (via bnto-image bridge)
// =============================================================================
//
// These tests verify that bnto-image's compress_image and compress_image_bytes
// functions work when called through the unified entry point's WASM binary.
//
// We embed test fixture images at compile time with include_bytes!() so
// tests work in any environment (no filesystem access needed in WASM).

/// Embed a small test JPEG at compile time.
/// This fixture is shared with the Go engine tests (test-fixtures/images/).
const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

/// Test that compress_image() returns valid JSON metadata.
///
/// This tests the full pipeline: JS bytes → Rust decode → compress → JSON metadata.
/// We use js_sys::Function::new_no_args to create a dummy progress callback.
#[wasm_bindgen_test]
fn test_compress_image_returns_json() {
    // Create a no-op progress callback (function that does nothing).
    // In the real Web Worker, this would call postMessage().
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    // Call the compress function. This goes through bnto-image's wasm_bridge.
    let result = bnto_image::wasm_bridge::compress_image(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );

    // Verify it succeeded (not an error).
    assert!(
        result.is_ok(),
        "compress_image should succeed: {:?}",
        result.err()
    );

    // Verify the result is valid JSON.
    let json_str = result.unwrap();
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json_str);
    assert!(parsed.is_ok(), "Result should be valid JSON: {}", json_str);

    // Verify the JSON has expected fields.
    let value = parsed.unwrap();
    assert!(value.get("file").is_some(), "JSON should have 'file' field");
    assert!(
        value.get("metadata").is_some(),
        "JSON should have 'metadata' field"
    );
}

/// Test that compress_image_bytes() returns raw compressed bytes.
///
/// This is the efficient path the Web Worker uses — raw bytes that become
/// a Blob for download. No JSON parsing needed for the file data.
#[wasm_bindgen_test]
fn test_compress_image_bytes_returns_data() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result = bnto_image::wasm_bridge::compress_image_bytes(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );

    assert!(
        result.is_ok(),
        "compress_image_bytes should succeed: {:?}",
        result.err()
    );

    let bytes = result.unwrap();
    // Compressed output should be non-empty.
    assert!(!bytes.is_empty(), "Compressed bytes should not be empty");
    // Verify the output starts with JPEG magic bytes (FF D8 FF).
    assert_eq!(bytes[0], 0xFF, "Should start with JPEG magic byte 1");
    assert_eq!(bytes[1], 0xD8, "Should start with JPEG magic byte 2");
}

/// Test that compressing with default params (empty JSON) works.
/// The compression engine should use sensible defaults when no
/// quality is specified.
#[wasm_bindgen_test]
fn test_compress_with_default_params() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result =
        bnto_image::wasm_bridge::compress_image_bytes(TEST_JPEG, "small.jpg", "{}", noop_cb);

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
fn test_compress_invalid_data_returns_error() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    // Send garbage data that isn't a valid image.
    let result = bnto_image::wasm_bridge::compress_image_bytes(
        b"this is not an image",
        "garbage.jpg",
        "{}",
        noop_cb,
    );

    // Should be an error, not a panic.
    assert!(result.is_err(), "Invalid data should return an error");
}
