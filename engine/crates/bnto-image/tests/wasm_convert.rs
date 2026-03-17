// WASM Integration Tests -- image format conversion via WASM boundary.
//
// Tests convert_image_format_combined(). See also: wasm.rs (compression),
// wasm_codec.rs (detailed codec), wasm_resize.rs, wasm_progress.rs.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// JPEG -> PNG Conversion Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_convert_jpeg_to_png_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);
    assert!(result.is_ok(), "JPEG -> PNG conversion should succeed");

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        !json_str.is_empty(),
        "Result metadata JSON should not be empty"
    );

    assert!(
        json_str.contains("\"originalFormat\""),
        "Metadata should contain 'originalFormat': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("\"targetFormat\""),
        "Metadata should contain 'targetFormat': got '{}'",
        json_str
    );

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains(".png"),
        "Output filename should have .png extension: got '{}'",
        filename
    );
}

#[wasm_bindgen_test]
fn test_convert_jpeg_to_png_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);
    assert!(result.is_ok(), "JPEG -> PNG conversion should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Converted PNG bytes should not be empty");

    // Valid PNG magic bytes.
    assert_eq!(bytes[0], 0x89);
    assert_eq!(bytes[1], 0x50);
    assert_eq!(bytes[2], 0x4E);
    assert_eq!(bytes[3], 0x47);
}

// =========================================================================
// PNG -> WebP Conversion Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_convert_png_to_webp_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = convert_image_format_combined(
        TEST_PNG,
        "screenshot.png",
        r#"{"format": "webp"}"#,
        callback,
    );
    assert!(result.is_ok(), "PNG -> WebP conversion should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Converted WebP bytes should not be empty"
    );

    // Valid WebP (RIFF container with WEBP marker).
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
// WebP -> JPEG Conversion Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_convert_webp_to_jpeg_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_WEBP, "image.webp", r#"{"format": "jpeg"}"#, callback);
    assert!(result.is_ok(), "WebP -> JPEG conversion should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Converted JPEG bytes should not be empty"
    );

    // Valid JPEG magic bytes.
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
    assert_eq!(bytes[2], 0xFF);
}

// =========================================================================
// Progress Callback Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_convert_reports_progress() {
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);
    assert!(result.is_ok(), "Conversion should succeed");

    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}

// =========================================================================
// Error Handling Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_convert_invalid_format_returns_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "bmp"}"#, callback);
    assert!(
        result.is_err(),
        "Should return an error for unsupported target format"
    );
}

#[wasm_bindgen_test]
fn test_convert_missing_format_returns_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = convert_image_format_combined(TEST_JPEG, "photo.jpg", "{}", callback);
    assert!(
        result.is_err(),
        "Should return an error when 'format' parameter is missing"
    );
}

#[wasm_bindgen_test]
fn test_convert_corrupt_data_returns_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = convert_image_format_combined(
        b"this is not an image at all",
        "corrupt.jpg",
        r#"{"format": "png"}"#,
        callback,
    );
    assert!(
        result.is_err(),
        "Should return an error for corrupt image data"
    );
}

#[wasm_bindgen_test]
fn test_convert_with_quality_param_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = convert_image_format_combined(
        TEST_PNG,
        "screenshot.png",
        r#"{"format": "jpeg", "quality": 50}"#,
        callback,
    );
    assert!(
        result.is_ok(),
        "PNG -> JPEG with quality param should succeed"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
}
