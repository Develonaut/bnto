// WASM Integration Tests -- image resize via WASM boundary.
//
// Tests resize_image_combined(). See also: wasm.rs (compression),
// wasm_codec.rs (detailed codec), wasm_convert.rs, wasm_progress.rs.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    LARGE_JPEG, TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// JPEG Resize Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_jpeg_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    // Resize 100x100 test JPEG to 50px wide.
    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);
    assert!(
        result.is_ok(),
        "resize_image_combined should succeed for JPEG"
    );

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(!json_str.is_empty());

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

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("resized"),
        "Output filename should contain 'resized': got '{}'",
        filename
    );
}

#[wasm_bindgen_test]
fn test_resize_jpeg_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Resized JPEG bytes should not be empty");

    // Valid JPEG magic bytes.
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
    assert_eq!(bytes[2], 0xFF);
}

#[wasm_bindgen_test]
fn test_resize_jpeg_both_dimensions_via_wasm() {
    // Explicit width AND height -- ignores aspect ratio.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 60, "height": 40}"#,
        callback,
    );
    assert!(result.is_ok(), "resize with both dimensions should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
}

// =========================================================================
// PNG Resize Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_png_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_PNG, "screenshot.png", r#"{"width": 50}"#, callback);
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());

    // Valid PNG magic bytes.
    assert_eq!(bytes[0], 0x89);
    assert_eq!(bytes[1], 0x50);
    assert_eq!(bytes[2], 0x4E);
    assert_eq!(bytes[3], 0x47);
}

// =========================================================================
// WebP Resize Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_webp_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_WEBP, "image.webp", r#"{"width": 50}"#, callback);
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());

    // Valid WebP (RIFF container).
    assert_eq!(bytes[0], b'R');
    assert_eq!(bytes[1], b'I');
    assert_eq!(bytes[2], b'F');
    assert_eq!(bytes[3], b'F');
    assert_eq!(bytes[8], b'W');
    assert_eq!(bytes[9], b'E');
    assert_eq!(bytes[10], b'B');
    assert_eq!(bytes[11], b'P');
}

// =========================================================================
// Height-Only and Aspect Ratio Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_height_only_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"height": 50}"#, callback);
    assert!(result.is_ok(), "resize with height-only should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0xFF);
}

#[wasm_bindgen_test]
fn test_resize_with_quality_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 50, "quality": 50}"#,
        callback,
    );
    assert!(result.is_ok(), "resize with quality param should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
}

#[wasm_bindgen_test]
fn test_resize_maintain_aspect_false_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"width": 50, "maintainAspect": false}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "resize with maintainAspect=false should succeed"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
}

// =========================================================================
// Upscale Test
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_upscale_via_wasm() {
    // Upscale 100x100 to 200x200 -- more pixels = larger output.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"width": 200}"#, callback);
    assert!(result.is_ok(), "upscale resize should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    assert!(
        bytes.len() > TEST_JPEG.len(),
        "Upscaled image ({} bytes) should be larger than original ({} bytes)",
        bytes.len(),
        TEST_JPEG.len()
    );
}

// =========================================================================
// Large Image Resize Test
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_large_jpeg_via_wasm() {
    // 1200x800 down to 300px. Tests WASM memory handling with larger images.
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(LARGE_JPEG, "large.jpg", r#"{"width": 300}"#, callback);
    assert!(result.is_ok(), "resize of large JPEG should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    assert!(
        bytes.len() < LARGE_JPEG.len(),
        "Downscaled image ({} bytes) should be smaller than original ({} bytes)",
        bytes.len(),
        LARGE_JPEG.len()
    );
}

// =========================================================================
// Error Handling Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_no_dimensions_returns_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = resize_image_combined(TEST_JPEG, "photo.jpg", "{}", callback);
    assert!(
        result.is_err(),
        "Should return an error when no dimensions specified"
    );
}

#[wasm_bindgen_test]
fn test_resize_unsupported_format_returns_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = resize_image_combined(
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
    // Invalid JSON -> empty params -> no dimensions -> should fail.
    init_panic_hook();
    let callback = noop_callback();
    let result = resize_image_combined(
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

// =========================================================================
// Progress Callback Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_resize_reports_progress() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);
    assert!(result.is_ok(), "resize should succeed");

    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}
