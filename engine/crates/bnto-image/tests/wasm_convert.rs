// =============================================================================
// WASM Integration Tests — Image Format Conversion (via WASM Boundary)
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These tests run inside a Node.js process (not native Rust). They prove
// that the convert_image_format_combined WASM-exported function works
// correctly when called from JavaScript across the WASM boundary.
//
// These catch problems that native Rust tests can't find:
//   - wasm-bindgen type conversions (Uint8Array <-> Vec<u8>)
//   - WASM memory handling during format conversion
//   - JS callback interop (progress reporting across the boundary)
//   - Combined result object structure (metadata + data + filename + mimeType)
//
// HOW TO RUN:
//   cd engine
//   wasm-pack test --headless --node crates/bnto-image
//
// RELATED TEST FILES:
//   - wasm.rs           — core compression tests (JPEG, PNG, WebP)
//   - wasm_codec.rs     — detailed codec coverage
//   - wasm_resize.rs    — image resize tests
//   - wasm_progress.rs  — progress callback verification
//   - wasm_stress.rs    — large file OOM and size tests

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback, recording_callback,
};

// Configure tests to run in Node.js.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// JPEG -> PNG Conversion Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_convert_jpeg_to_png_metadata_via_wasm() {
    // Test that convert_image_format_combined() returns a JS object with
    // correct metadata JSON for a JPEG → PNG conversion.
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);

    assert!(result.is_ok(), "JPEG -> PNG conversion should succeed");

    // --- Extract metadata from the combined result object ---
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        !json_str.is_empty(),
        "Result metadata JSON should not be empty"
    );

    // Verify the metadata JSON contains conversion stats.
    // Filename is a separate property on the result object, not in metadata.
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

    // Filename with correct extension is a top-level property on the result.
    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains(".png"),
        "Output filename should have .png extension: got '{}'",
        filename
    );
}

#[wasm_bindgen_test]
fn test_convert_jpeg_to_png_bytes_via_wasm() {
    // Test that convert_image_format_combined() returns valid PNG bytes
    // in the `data` property of the combined result.
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);

    assert!(result.is_ok(), "JPEG -> PNG conversion should succeed");

    // --- Extract raw bytes from the combined result's data property ---
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Converted PNG bytes should not be empty");

    // Verify valid PNG magic bytes: 89 50 4E 47
    assert_eq!(bytes[0], 0x89, "First byte of PNG should be 0x89");
    assert_eq!(bytes[1], 0x50, "Second byte of PNG should be 'P'");
    assert_eq!(bytes[2], 0x4E, "Third byte of PNG should be 'N'");
    assert_eq!(bytes[3], 0x47, "Fourth byte of PNG should be 'G'");
}

// =============================================================================
// PNG -> WebP Conversion Tests (via WASM boundary)
// =============================================================================

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

    // --- Extract raw bytes from the combined result ---
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Converted WebP bytes should not be empty"
    );

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
// WebP -> JPEG Conversion Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_convert_webp_to_jpeg_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result =
        convert_image_format_combined(TEST_WEBP, "image.webp", r#"{"format": "jpeg"}"#, callback);

    assert!(result.is_ok(), "WebP -> JPEG conversion should succeed");

    // --- Extract raw bytes from the combined result ---
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Converted JPEG bytes should not be empty"
    );

    // Verify valid JPEG magic bytes: FF D8 FF
    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
    assert_eq!(bytes[2], 0xFF, "Third byte of JPEG should be 0xFF");
}

// =============================================================================
// Progress Callback Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_convert_reports_progress() {
    // Verify that convert_image_format_combined reports progress through the callback.
    init_panic_hook();
    let (callback, calls) = recording_callback();

    let result =
        convert_image_format_combined(TEST_JPEG, "photo.jpg", r#"{"format": "png"}"#, callback);

    assert!(result.is_ok(), "Conversion should succeed");

    // Progress should have been reported at least once.
    assert!(
        calls.length() > 0,
        "Progress callback should have been called at least once, got {} calls",
        calls.length()
    );
}

// =============================================================================
// Error Handling Tests (via WASM boundary)
// =============================================================================

#[wasm_bindgen_test]
fn test_convert_invalid_format_returns_error() {
    // An unsupported target format should return a JS error.
    // The combined function returns Result<JsValue, JsValue>, so .is_err()
    // still works the same way for error checking.
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
    // No "format" parameter at all — should return a JS error.
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
    // Random bytes that aren't a valid image — should fail gracefully.
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
    // Convert PNG -> JPEG with explicit quality parameter.
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

    // --- Extract and verify JPEG bytes from the combined result ---
    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty());
    // Verify valid JPEG.
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
}
