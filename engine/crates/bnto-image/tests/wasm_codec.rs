// WASM Integration Tests -- comprehensive codec coverage.
//
// Goes deeper than wasm.rs: verifies metadata JSON fields, output filenames,
// quality parameter handling, and size relationships across WASM. Catches
// JSON serialization, string encoding, and number precision issues.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    extract_mime_type, init_panic_hook, noop_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// PNG Metadata Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_compress_png_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_PNG, "screenshot.png", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for PNG"
    );

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    assert!(
        json_str.contains("originalSize"),
        "Metadata should contain 'originalSize': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressedSize"),
        "Metadata should contain 'compressedSize': got '{json_str}'"
    );

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("compressed"),
        "PNG filename should contain 'compressed': got '{filename}'"
    );
    let mime = extract_mime_type(&result_obj);
    assert_eq!(mime, "image/png", "PNG MIME type should be 'image/png'");
}

// =========================================================================
// WebP Metadata Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_compress_webp_metadata_via_wasm() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_WEBP, "banner.webp", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for WebP"
    );

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    assert!(
        json_str.contains("originalSize"),
        "WebP metadata should contain 'originalSize': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressedSize"),
        "WebP metadata should contain 'compressedSize': got '{json_str}'"
    );

    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("compressed"),
        "WebP filename should contain 'compressed': got '{filename}'"
    );
    let mime = extract_mime_type(&result_obj);
    assert_eq!(mime, "image/webp", "WebP MIME type should be 'image/webp'");
}

// =========================================================================
// Compression Ratio in Metadata
// =========================================================================

#[wasm_bindgen_test]
fn test_jpeg_metadata_has_compression_ratio() {
    // compressionRatio is an f64 that crosses WASM inside the metadata JSON string.
    init_panic_hook();
    let callback = noop_callback();

    let result =
        compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"compression": 40}"#, callback);
    assert!(result.is_ok(), "compress_image_combined should succeed");

    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    assert!(
        json_str.contains("compressionRatio"),
        "JPEG metadata should contain 'compressionRatio': got '{json_str}'"
    );
    assert!(
        json_str.contains("format"),
        "Metadata should contain 'format': got '{json_str}'"
    );
    assert!(
        json_str.contains("Jpeg"),
        "Format should be 'Jpeg': got '{json_str}'"
    );
}

// =========================================================================
// Output Filename Tests -- "-compressed" Suffix Across WASM
// =========================================================================

#[wasm_bindgen_test]
fn test_jpeg_output_filename_has_compressed_suffix() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(
        TEST_JPEG,
        "my-photo.jpg",
        r#"{"compression": 20}"#,
        callback,
    );
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(filename, "my-photo-compressed.jpg");
}

#[wasm_bindgen_test]
fn test_png_output_filename_has_compressed_suffix() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_PNG, "chart.png", "{}", callback);
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(filename, "chart-compressed.png");
}

#[wasm_bindgen_test]
fn test_webp_output_filename_has_compressed_suffix() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_WEBP, "hero-image.webp", "{}", callback);
    assert!(result.is_ok());

    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(filename, "hero-image-compressed.webp");
}

// =========================================================================
// Compression Parameter -- Affects Output Size Across WASM
// =========================================================================

#[wasm_bindgen_test]
fn test_jpeg_higher_compression_produces_smaller_output() {
    // Verifies compression param actually makes it through JSON -> Rust -> JPEG encoder.
    // compression 80 -> quality 21 (aggressive), compression 5 -> quality 96 (minimal).
    // If params_json parsing broke, both would use defaults and produce identical sizes.
    init_panic_hook();

    let result_c80 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"compression": 80}"#,
        noop_callback(),
    );
    assert!(result_c80.is_ok(), "Compression 80 should succeed");
    let bytes_c80 = extract_bytes(&result_c80.unwrap());

    let result_c5 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"compression": 5}"#,
        noop_callback(),
    );
    assert!(result_c5.is_ok(), "Compression 5 should succeed");
    let bytes_c5 = extract_bytes(&result_c5.unwrap());

    assert!(
        bytes_c80.len() < bytes_c5.len(),
        "Compression 80 ({} bytes) should produce smaller output than compression 5 ({} bytes)",
        bytes_c80.len(),
        bytes_c5.len()
    );
}
