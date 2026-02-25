// =============================================================================
// WASM Integration Tests — Image Processing Across the JS ↔ Rust Boundary
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that our WASM-exported functions work correctly when called from JavaScript.
// This catches problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array ↔ Vec<u8>, String ↔ &str)
//   - WASM memory allocation issues (large images might exceed WASM memory)
//   - JS callback interop (progress reporting across the boundary)
//
// HOW TO RUN:
//   wasm-pack test --node crates/bnto-image
//
// This compiles to WASM, starts Node.js, loads the WASM binary, and
// runs these tests inside the JS runtime. It's slower than native tests
// but catches an entire category of bugs.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

// Import our crate's WASM-exported functions.
// The crate name uses underscores (Rust convention for crate names with hyphens).
use bnto_image::wasm_bridge::*;

// Configure tests to run in Node.js (not in a browser).
// Browser tests would need a headless Chrome/Firefox setup.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Test Fixtures — Small images embedded at compile time
// =============================================================================
//
// We embed test images directly in the test binary using `include_bytes!`.
// These are the same shared fixtures used by the native Rust tests.

/// A small JPEG image (100x100 pixels, ~2.7 KB)
const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

/// A small PNG image (100x100 pixels, ~11 KB)
const TEST_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");

/// A small WebP image (100x100 pixels, ~966 bytes)
const TEST_WEBP: &[u8] = include_bytes!("../../../../test-fixtures/images/small.webp");

// =============================================================================
// Helper: Create a no-op progress callback
// =============================================================================
//
// In Node.js tests, we need a real JavaScript function for the progress
// callback, but we don't need it to do anything. This creates a JS
// function that accepts any arguments and does nothing.
//
// WASM CONCEPT: `js_sys::Function`
// This is a Rust representation of a JavaScript function. We create it
// using `js_sys::eval()` which evaluates a string of JavaScript code
// and returns the result.

/// Create a JavaScript function that does nothing (for progress callbacks).
fn noop_callback() -> js_sys::Function {
    // `eval` runs JavaScript code and returns the result as a JsValue.
    // We create an anonymous arrow function that ignores its arguments.
    // The `.into()` converts JsValue to js_sys::Function.
    //
    // SAFETY: This is safe because the JS code is a static string literal
    // that we control. Never use `eval` with user-provided strings!
    js_sys::eval("(function() {})")
        .expect("Failed to create noop callback")
        .dyn_into::<js_sys::Function>()
        .expect("eval result should be a Function")
}

// =============================================================================
// Setup Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_setup_does_not_panic() {
    // Verify the WASM module initializes correctly.
    // If this fails, nothing else will work.
    setup();
}

#[wasm_bindgen_test]
fn test_version_returns_semver() {
    let v = version();
    assert!(!v.is_empty(), "Version should not be empty");
    assert!(v.contains('.'), "Version should be semver: got '{}'", v);
}

// =============================================================================
// JPEG Compression Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_jpeg_via_wasm() {
    setup();
    let callback = noop_callback();

    // Call the WASM-exported function with a real JPEG image.
    // This tests the entire pipeline: JS → Rust → image decode →
    // re-encode → Rust → JS.
    let result = compress_image(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);

    // The result should be Ok (not an error).
    assert!(result.is_ok(), "compress_image should succeed for JPEG");

    // The result is a JSON string with metadata about the compressed file.
    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "Result JSON should not be empty");
    // Verify it contains expected fields
    assert!(
        json_str.contains("filename"),
        "Result JSON should contain 'filename': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("compressed"),
        "Filename in result should contain 'compressed': got '{}'",
        json_str
    );
}

#[wasm_bindgen_test]
fn test_compress_jpeg_bytes_via_wasm() {
    setup();
    let callback = noop_callback();

    // Use the bytes variant — returns raw compressed bytes.
    let result = compress_image_bytes(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);

    assert!(
        result.is_ok(),
        "compress_image_bytes should succeed for JPEG"
    );

    let bytes = result.unwrap();
    // Compressed output should be non-empty.
    assert!(
        !bytes.is_empty(),
        "Compressed JPEG bytes should not be empty"
    );

    // Verify the output is a valid JPEG (check magic bytes).
    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
}

// =============================================================================
// PNG Compression Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_png_via_wasm() {
    setup();
    let callback = noop_callback();

    let result = compress_image_bytes(
        TEST_PNG,
        "screenshot.png",
        "{}", // Empty params = use defaults
        callback,
    );

    assert!(
        result.is_ok(),
        "compress_image_bytes should succeed for PNG"
    );

    let bytes = result.unwrap();
    assert!(
        !bytes.is_empty(),
        "Compressed PNG bytes should not be empty"
    );

    // Verify the output is a valid PNG (check magic bytes).
    assert_eq!(bytes[0], 0x89, "First byte of PNG should be 0x89");
    assert_eq!(bytes[1], 0x50, "Second byte of PNG should be 'P'");
    assert_eq!(bytes[2], 0x4E, "Third byte of PNG should be 'N'");
    assert_eq!(bytes[3], 0x47, "Fourth byte of PNG should be 'G'");
}

// =============================================================================
// WebP Compression Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_webp_via_wasm() {
    setup();
    let callback = noop_callback();

    let result = compress_image_bytes(TEST_WEBP, "image.webp", "{}", callback);

    assert!(
        result.is_ok(),
        "compress_image_bytes should succeed for WebP"
    );

    let bytes = result.unwrap();
    assert!(
        !bytes.is_empty(),
        "Compressed WebP bytes should not be empty"
    );

    // Verify the output is a valid WebP (RIFF container with WEBP).
    assert_eq!(bytes[0], b'R', "WebP should start with RIFF");
    assert_eq!(bytes[1], b'I');
    assert_eq!(bytes[2], b'F');
    assert_eq!(bytes[3], b'F');
    assert_eq!(bytes[8], b'W', "WebP RIFF should contain WEBP");
    assert_eq!(bytes[9], b'E');
    assert_eq!(bytes[10], b'B');
    assert_eq!(bytes[11], b'P');
}

// =============================================================================
// Error Handling Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_unsupported_format_returns_js_error() {
    setup();
    let callback = noop_callback();

    // Send garbage data with an unsupported extension.
    let result = compress_image_bytes(b"not an image", "document.pdf", "{}", callback);

    // Should return an error, not Ok.
    assert!(
        result.is_err(),
        "Should return an error for unsupported format"
    );
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    setup();
    let callback = noop_callback();

    // Send invalid JSON for params — should fall back to defaults,
    // not crash.
    let result = compress_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        "this is not valid json!!!",
        callback,
    );

    // Should still succeed — invalid params = use defaults.
    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (uses defaults)"
    );
}
