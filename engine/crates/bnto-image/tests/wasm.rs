// WASM Integration Tests -- core image compression and error handling.
//
// Tests the JS <-> Rust boundary for compress_image_combined(). Pure Rust
// compression logic is tested in compress.rs. See also: wasm_codec.rs
// (metadata/filenames/quality), wasm_progress.rs, wasm_stress.rs.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// JPEG Compression Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_compress_jpeg_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for JPEG"
    );

    let result_obj = result.unwrap();

    let json_str = extract_metadata(&result_obj);
    assert!(!json_str.is_empty(), "Metadata JSON should not be empty");
    assert!(
        json_str.contains("originalSize"),
        "Metadata should contain 'originalSize': got '{}'",
        json_str
    );
    assert!(
        json_str.contains("compressedSize"),
        "Metadata should contain 'compressedSize': got '{}'",
        json_str
    );

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("compressed"),
        "Output filename should contain 'compressed': got '{}'",
        filename
    );
}

#[wasm_bindgen_test]
fn test_compress_jpeg_combined_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for JPEG"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Compressed JPEG bytes should not be empty"
    );

    // Valid JPEG magic bytes.
    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
}

// =========================================================================
// PNG Compression Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_compress_png_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_PNG, "screenshot.png", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for PNG"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Compressed PNG bytes should not be empty"
    );

    // Valid PNG magic bytes.
    assert_eq!(bytes[0], 0x89);
    assert_eq!(bytes[1], 0x50);
    assert_eq!(bytes[2], 0x4E);
    assert_eq!(bytes[3], 0x47);
}

// =========================================================================
// WebP Compression Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_compress_webp_combined_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_WEBP, "image.webp", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for WebP"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(
        !bytes.is_empty(),
        "Compressed WebP bytes should not be empty"
    );

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
// Error Handling Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_unsupported_format_returns_js_error() {
    init_panic_hook();
    let callback = noop_callback();
    let result = compress_image_combined(b"not an image", "document.pdf", "{}", callback);
    assert!(
        result.is_err(),
        "Should return an error for unsupported format"
    );
}

#[wasm_bindgen_test]
fn test_invalid_params_json_uses_defaults() {
    init_panic_hook();
    let callback = noop_callback();
    let result = compress_image_combined(
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
