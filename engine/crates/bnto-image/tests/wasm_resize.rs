// =============================================================================
// WASM Integration Tests — Image Resize (via WASM Boundary)
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that the resize_image and resize_image_bytes WASM-exported functions work
// correctly when called from JavaScript across the WASM boundary.
//
// These catch problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array ↔ Vec<u8>)
//   - WASM memory handling during resize operations
//   - JS callback interop (progress reporting across the boundary)
//   - JSON serialization of resize metadata
//
// HOW TO RUN:
//   cd engine-wasm
//   wasm-pack test --headless --node crates/bnto-image
//
// RELATED TEST FILES:
//   - wasm.rs           — core compression tests (JPEG, PNG, WebP)
//   - wasm_codec.rs     — detailed codec coverage
//   - wasm_progress.rs  — progress callback verification
//   - wasm_stress.rs    — large file OOM and size tests

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    LARGE_JPEG, TEST_JPEG, TEST_PNG, TEST_WEBP, init_panic_hook, noop_callback, recording_callback,
};

// Configure tests to run in Node.js.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// JPEG Resize Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_jpeg_metadata_via_wasm() {
    // Test that resize_image() returns correct JSON metadata for JPEG.
    init_panic_hook();
    let callback = noop_callback();

    // Resize the 100×100 test JPEG to 50 pixels wide.
    let result = resize_image(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);

    assert!(result.is_ok(), "resize_image should succeed for JPEG");

    let json_str = result.unwrap();
    assert!(!json_str.is_empty(), "Result JSON should not be empty");

    // Verify the JSON contains expected fields.
    assert!(
        json_str.contains("\"filename\""),
        "Result should contain 'filename': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("resized"),
        "Filename should contain 'resized': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("\"originalWidth\""),
        "Metadata should contain 'originalWidth': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("\"newWidth\""),
        "Metadata should contain 'newWidth': got '{}'",
        json_str
    );
}

#[wasm_bindgen_test]
fn test_resize_jpeg_bytes_via_wasm() {
    // Test that resize_image_bytes() returns valid JPEG bytes.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);

    assert!(result.is_ok(), "resize_image_bytes should succeed for JPEG");

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Resized JPEG bytes should not be empty");

    // Verify valid JPEG magic bytes (FF D8 FF).
    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
    assert_eq!(bytes[2], 0xFF, "Third byte of JPEG should be 0xFF");
}

#[wasm_bindgen_test]
fn test_resize_jpeg_both_dimensions_via_wasm() {
    // Resize with explicit width AND height — ignores aspect ratio.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 60, "height": 40}"#,
        callback,
    );

    assert!(result.is_ok(), "resize with both dimensions should succeed");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
    // Valid JPEG magic bytes
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
}

// =============================================================================
// PNG Resize Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_png_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_PNG, "screenshot.png", r#"{"width": 50}"#, callback);

    assert!(result.is_ok(), "resize_image_bytes should succeed for PNG");

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Resized PNG bytes should not be empty");

    // Verify valid PNG magic bytes (89 50 4E 47).
    assert_eq!(bytes[0], 0x89, "First byte of PNG should be 0x89");
    assert_eq!(bytes[1], 0x50, "Second byte of PNG should be 'P'");
    assert_eq!(bytes[2], 0x4E, "Third byte of PNG should be 'N'");
    assert_eq!(bytes[3], 0x47, "Fourth byte of PNG should be 'G'");
}

// =============================================================================
// WebP Resize Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_webp_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_WEBP, "image.webp", r#"{"width": 50}"#, callback);

    assert!(result.is_ok(), "resize_image_bytes should succeed for WebP");

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Resized WebP bytes should not be empty");

    // Verify valid WebP (RIFF container with WEBP marker).
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
// Height-Only and Aspect Ratio Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_height_only_via_wasm() {
    // Resize using only height — width calculated from aspect ratio.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_JPEG, "photo.jpg", r#"{"height": 50}"#, callback);

    assert!(result.is_ok(), "resize with height-only should succeed");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0xFF); // Valid JPEG
}

#[wasm_bindgen_test]
fn test_resize_with_quality_via_wasm() {
    // Resize JPEG with custom quality parameter.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 50, "quality": 50}"#,
        callback,
    );

    assert!(result.is_ok(), "resize with quality param should succeed");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

#[wasm_bindgen_test]
fn test_resize_maintain_aspect_false_via_wasm() {
    // Resize with maintainAspect=false — width only, height uses original.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 50, "maintainAspect": false}"#,
        callback,
    );

    assert!(
        result.is_ok(),
        "resize with maintainAspect=false should succeed"
    );
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

// =============================================================================
// Upscale Test
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_upscale_via_wasm() {
    // Upscale a 100×100 image to 200×200.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_JPEG, "photo.jpg", r#"{"width": 200}"#, callback);

    assert!(result.is_ok(), "upscale resize should succeed");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
    // Upscaled JPEG should be larger than the original small one
    // (more pixels = more data, even at the same quality)
    assert!(
        bytes.len() > TEST_JPEG.len(),
        "Upscaled image ({} bytes) should be larger than original ({} bytes)",
        bytes.len(),
        TEST_JPEG.len()
    );
}

// =============================================================================
// Large Image Resize Test
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_large_jpeg_via_wasm() {
    // Resize a large image (1200×800) down to 300px wide.
    // This tests WASM memory handling during resize of larger images.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(LARGE_JPEG, "large.jpg", r#"{"width": 300}"#, callback);

    assert!(result.is_ok(), "resize of large JPEG should succeed");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
    // Downscaled image should be smaller than the original
    assert!(
        bytes.len() < LARGE_JPEG.len(),
        "Downscaled image ({} bytes) should be smaller than original ({} bytes)",
        bytes.len(),
        LARGE_JPEG.len()
    );
}

// =============================================================================
// Error Handling Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_no_dimensions_returns_error() {
    // No width or height in params — should fail with a clear error.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(TEST_JPEG, "photo.jpg", "{}", callback);

    assert!(
        result.is_err(),
        "Should return an error when no dimensions specified"
    );
}

#[wasm_bindgen_test]
fn test_resize_unsupported_format_returns_error() {
    // Non-image data should fail.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(
        b"not an image",
        "document.pdf",
        r#"{"width": 50}"#,
        callback,
    );

    assert!(
        result.is_err(),
        "Should return an error for unsupported format"
    );
}

#[wasm_bindgen_test]
fn test_resize_invalid_params_uses_defaults() {
    // Invalid JSON params — should use defaults (but still need width/height).
    // Since invalid JSON becomes empty params, and we need at least one
    // dimension, this should fail.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_bytes(
        TEST_JPEG,
        "photo.jpg",
        "this is not valid json!!!",
        callback,
    );

    assert!(
        result.is_err(),
        "Should fail because invalid JSON means no dimensions"
    );
}

// =============================================================================
// Progress Callback Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_resize_reports_progress() {
    // Verify that resize_image reports progress through the callback.
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = resize_image(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);

    assert!(result.is_ok(), "resize should succeed");

    // Progress should have been reported at least once.
    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}
