// WASM integration tests -- image resize per format (JPEG, PNG, WebP).

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    init_panic_hook, noop_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// --- JPEG Resize Tests ---

#[wasm_bindgen_test]
fn test_resize_jpeg_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_JPEG, "photo.jpg", r#"{"width": 50}"#, callback);

    assert!(
        result.is_ok(),
        "resize_image_combined should succeed for JPEG"
    );

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        !json_str.is_empty(),
        "Result metadata JSON should not be empty"
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

    assert!(
        result.is_ok(),
        "resize_image_combined should succeed for JPEG"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Resized JPEG bytes should not be empty");

    assert_eq!(bytes[0], 0xFF, "First byte of JPEG should be 0xFF");
    assert_eq!(bytes[1], 0xD8, "Second byte of JPEG should be 0xD8");
    assert_eq!(bytes[2], 0xFF, "Third byte of JPEG should be 0xFF");
}

#[wasm_bindgen_test]
fn test_resize_jpeg_both_dimensions_via_wasm() {
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

// --- PNG Resize Tests ---

#[wasm_bindgen_test]
fn test_resize_png_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_PNG, "screenshot.png", r#"{"width": 50}"#, callback);

    assert!(
        result.is_ok(),
        "resize_image_combined should succeed for PNG"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Resized PNG bytes should not be empty");

    assert_eq!(bytes[0], 0x89, "First byte of PNG should be 0x89");
    assert_eq!(bytes[1], 0x50, "Second byte of PNG should be 'P'");
    assert_eq!(bytes[2], 0x4E, "Third byte of PNG should be 'N'");
    assert_eq!(bytes[3], 0x47, "Fourth byte of PNG should be 'G'");
}

// --- WebP Resize Tests ---

#[wasm_bindgen_test]
fn test_resize_webp_bytes_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = resize_image_combined(TEST_WEBP, "image.webp", r#"{"width": 50}"#, callback);

    assert!(
        result.is_ok(),
        "resize_image_combined should succeed for WebP"
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Resized WebP bytes should not be empty");

    assert_eq!(bytes[0], b'R', "WebP should start with RIFF");
    assert_eq!(bytes[1], b'I');
    assert_eq!(bytes[2], b'F');
    assert_eq!(bytes[3], b'F');
    assert_eq!(bytes[8], b'W', "WebP RIFF should contain WEBP");
    assert_eq!(bytes[9], b'E');
    assert_eq!(bytes[10], b'B');
    assert_eq!(bytes[11], b'P');
}
