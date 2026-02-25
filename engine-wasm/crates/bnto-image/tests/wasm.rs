// =============================================================================
// WASM Integration Tests — Core Image Compression & Error Handling
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
//   These tests are included in `cargo test --workspace` (native) and
//   run as WASM integration tests via the bnto-wasm entry point crate.
//
// RELATED TEST FILES:
//   - wasm_codec.rs   — comprehensive codec coverage (metadata, filenames, quality)
//   - wasm_progress.rs — progress callback verification
//   - wasm_stress.rs  — large file OOM and size tests

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{TEST_JPEG, TEST_PNG, TEST_WEBP, init_panic_hook, noop_callback};

// Configure tests to run in Node.js.
wasm_bindgen_test_configure!(run_in_node_experimental);

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

    let result = compress_image_bytes(TEST_PNG, "screenshot.png", "{}", callback);

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
