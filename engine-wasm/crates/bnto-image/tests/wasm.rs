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
// NOTE: setup() and version() used to be exported from this crate. They now
// live in the bnto-wasm entry point crate. Panic hook setup is done inline
// here for test reliability.
//
// HOW TO RUN:
//   These tests are included in `cargo test --workspace` (native) and
//   run as WASM integration tests via the bnto-wasm entry point crate.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

// Import the crate's WASM-exported functions.
use bnto_image::wasm_bridge::*;

// Configure tests to run in Node.js.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Test Fixtures — Small images embedded at compile time
// =============================================================================

/// A small JPEG image (100x100 pixels, ~2.7 KB)
const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

/// A small PNG image (100x100 pixels, ~11 KB)
const TEST_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");

/// A small WebP image (100x100 pixels, ~966 bytes)
const TEST_WEBP: &[u8] = include_bytes!("../../../../test-fixtures/images/small.webp");

// =============================================================================
// Helper: Create a no-op progress callback
// =============================================================================

/// Create a JavaScript function that does nothing (for progress callbacks).
fn noop_callback() -> js_sys::Function {
    js_sys::eval("(function() {})")
        .expect("Failed to create noop callback")
        .dyn_into::<js_sys::Function>()
        .expect("eval result should be a Function")
}

/// Initialize panic hook for test reliability. In production, the bnto-wasm
/// entry point handles this. In these integration tests, the bnto-wasm entry
/// point isn't loaded, so we skip the panic hook — errors will still show
/// in the test output, just without the pretty console.error formatting.
fn init_panic_hook() {
    // NOTE: console_error_panic_hook was moved to bnto-wasm.
    // These tests still work fine without it — Rust test runner captures panics.
}

// =============================================================================
// JPEG Compression Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_jpeg_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);

    assert!(result.is_ok(), "compress_image should succeed for JPEG");

    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "Result JSON should not be empty");
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
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);

    assert!(
        result.is_ok(),
        "compress_image_bytes should succeed for JPEG"
    );

    let bytes = result.unwrap();
    assert!(
        !bytes.is_empty(),
        "Compressed JPEG bytes should not be empty"
    );

    // Verify valid JPEG magic bytes.
    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
}

// =============================================================================
// PNG Compression Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_png_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(
        TEST_PNG,
        "screenshot.png",
        "{}",
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

    // Verify valid PNG magic bytes.
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
    init_panic_hook();
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

    // Verify valid WebP (RIFF container).
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
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(b"not an image", "document.pdf", "{}", callback);

    assert!(
        result.is_err(),
        "Should return an error for unsupported format"
    );
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        "this is not valid json!!!",
        callback,
    );

    assert!(
        result.is_ok(),
        "Should succeed with invalid params JSON (uses defaults)"
    );
}
