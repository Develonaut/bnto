// =============================================================================
// WASM Integration Tests — Comprehensive Codec Coverage
// =============================================================================
//
// WHAT ARE THESE TESTS?
// The core wasm.rs tests verify that each format produces valid output bytes.
// These tests go deeper: they verify metadata JSON fields, output filenames,
// quality parameter handling, and size relationships across the WASM boundary
// using the combined function API.
//
// WHY THROUGH WASM?
// The pure Rust unit tests (in compress.rs) already verify compression logic.
// These tests catch WASM-specific issues:
//   - JSON serialization across the boundary (metadata fields survive the trip)
//   - String encoding for filenames (UTF-8 ↔ JS string conversion)
//   - Number precision for compression ratios (f64 ↔ JS number)
//   - Combined result object structure (metadata + data + filename + mimeType)
//
// COVERAGE MATRIX:
//   | Format | Bytes | Metadata | Filename | Quality | Size |
//   |--------|-------|----------|----------|---------|------|
//   | JPEG   |   ✓   |    ✓     |    ✓     |    ✓    |  ✓   |
//   | PNG    |   ✓   |    ✓     |    ✓     |   n/a   |  -   |
//   | WebP   |   ✓   |    ✓     |    ✓     |   n/a   |  -   |
//
// (✓ = tested here, - = tested in wasm_stress.rs, n/a = not applicable)

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_filename, extract_metadata,
    extract_mime_type, init_panic_hook, noop_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// PNG Metadata Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_png_metadata_via_wasm() {
    // --- Test: PNG compression returns valid metadata JSON ---
    //
    // The combined function returns a JS object. We extract the `metadata`
    // property which is a JSON string. Core wasm.rs only checks bytes for
    // PNG. This verifies PNG metadata includes all expected fields after
    // crossing the WASM boundary.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_PNG, "screenshot.png", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for PNG"
    );

    // --- Extract metadata JSON from the combined result ---
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    // --- Verify the metadata JSON contains compression stats ---
    //
    // These fields are built in compress.rs and serialized in wasm_bridge.rs.
    // If any field name gets mangled during WASM serialization, this catches it.
    assert!(
        json_str.contains("originalSize"),
        "Metadata should contain 'originalSize': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressedSize"),
        "Metadata should contain 'compressedSize': got '{json_str}'"
    );

    // Filename and MIME type are separate properties on the result object.
    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("compressed"),
        "PNG filename should contain 'compressed': got '{filename}'"
    );
    let mime = extract_mime_type(&result_obj);
    assert_eq!(mime, "image/png", "PNG MIME type should be 'image/png'");
}

// =============================================================================
// WebP Metadata Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_compress_webp_metadata_via_wasm() {
    // --- Test: WebP compression returns valid metadata JSON ---
    //
    // Verifies that WebP-specific metadata (RIFF container format,
    // lossless encoding) serializes correctly across the WASM boundary
    // via the combined function's metadata property.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_WEBP, "banner.webp", "{}", callback);
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed for WebP"
    );

    // --- Extract metadata JSON from the combined result ---
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);

    // --- Verify the metadata JSON contains compression stats ---
    //
    // These fields are built in compress.rs and serialized in wasm_bridge.rs.
    // If any field name gets mangled during WASM serialization, this catches it.
    assert!(
        json_str.contains("originalSize"),
        "WebP metadata should contain 'originalSize': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressedSize"),
        "WebP metadata should contain 'compressedSize': got '{json_str}'"
    );

    // Filename and MIME type are separate properties on the result object.
    let filename = extract_filename(&result_obj);
    assert!(
        filename.contains("compressed"),
        "WebP filename should contain 'compressed': got '{filename}'"
    );
    let mime = extract_mime_type(&result_obj);
    assert_eq!(mime, "image/webp", "WebP MIME type should be 'image/webp'");
}

// =============================================================================
// Compression Ratio in Metadata
// =============================================================================

#[wasm_bindgen_test]
fn test_jpeg_metadata_has_compression_ratio() {
    // --- Test: JPEG metadata includes compressionRatio field ---
    //
    // The compression ratio is calculated as:
    //   (1.0 - compressedSize / originalSize) * 100.0
    //
    // This is a floating-point number that crosses the WASM boundary inside
    // the metadata JSON string. We verify it survives serialization
    // (f64 → JSON → String → extract via Reflect::get).
    init_panic_hook();
    let callback = noop_callback();

    let result =
        compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"compression": 40}"#, callback);
    assert!(result.is_ok(), "compress_image_combined should succeed");

    // --- Extract metadata JSON from the combined result ---
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

// =============================================================================
// Output Filename Tests — "-compressed" Suffix Across WASM
// =============================================================================

#[wasm_bindgen_test]
fn test_jpeg_output_filename_has_compressed_suffix() {
    // --- Test: Output filename gets "-compressed" suffix ---
    //
    // "my-photo.jpg" → "my-photo-compressed.jpg"
    // The filename is a top-level property on the combined result object,
    // extracted via extract_filename(). It is NOT in the metadata JSON.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(
        TEST_JPEG,
        "my-photo.jpg",
        r#"{"compression": 20}"#,
        callback,
    );
    assert!(result.is_ok(), "compress_image_combined should succeed");

    // --- Extract filename from the combined result object ---
    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(
        filename, "my-photo-compressed.jpg",
        "Output filename should be 'my-photo-compressed.jpg': got '{filename}'"
    );
}

#[wasm_bindgen_test]
fn test_png_output_filename_has_compressed_suffix() {
    // --- Test: PNG output filename preserves the .png extension ---
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_PNG, "chart.png", "{}", callback);
    assert!(result.is_ok(), "compress_image_combined should succeed");

    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(
        filename, "chart-compressed.png",
        "Output filename should be 'chart-compressed.png': got '{filename}'"
    );
}

#[wasm_bindgen_test]
fn test_webp_output_filename_has_compressed_suffix() {
    // --- Test: WebP output filename preserves the .webp extension ---
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_WEBP, "hero-image.webp", "{}", callback);
    assert!(result.is_ok(), "compress_image_combined should succeed");

    let result_obj = result.unwrap();
    let filename = extract_filename(&result_obj);
    assert_eq!(
        filename, "hero-image-compressed.webp",
        "Output filename should be 'hero-image-compressed.webp': got '{filename}'"
    );
}

// =============================================================================
// Compression Parameter — Affects Output Size Across WASM
// =============================================================================

#[wasm_bindgen_test]
fn test_jpeg_higher_compression_produces_smaller_output() {
    // --- Test: JPEG compression parameter affects output size through WASM ---
    //
    // Higher compression = more aggressive = smaller file.
    // This verifies that the compression parameter actually makes it through
    // the JSON → Rust parsing → JPEG encoder pipeline across WASM.
    //
    // Internally: compression → quality via `101 - compression`.
    //   compression 80 → quality 21 (aggressive)
    //   compression 5  → quality 96 (minimal)
    //
    // If the params_json parsing was broken, both would use the default
    // compression and produce identical sizes — this test would catch that.
    //
    // We extract bytes from the combined result's `data` property.
    init_panic_hook();

    // --- Compress at compression 80 (aggressive) ---
    let result_c80 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"compression": 80}"#,
        noop_callback(),
    );
    assert!(result_c80.is_ok(), "Compression 80 should succeed");
    let bytes_c80 = extract_bytes(&result_c80.unwrap());

    // --- Compress at compression 5 (minimal) ---
    let result_c5 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"compression": 5}"#,
        noop_callback(),
    );
    assert!(result_c5.is_ok(), "Compression 5 should succeed");
    let bytes_c5 = extract_bytes(&result_c5.unwrap());

    // Compression 80 output should be smaller than compression 5 output.
    // Both are valid JPEGs (checked by magic bytes tests in wasm.rs).
    assert!(
        bytes_c80.len() < bytes_c5.len(),
        "Compression 80 ({} bytes) should produce smaller output than compression 5 ({} bytes)",
        bytes_c80.len(),
        bytes_c5.len()
    );
}
