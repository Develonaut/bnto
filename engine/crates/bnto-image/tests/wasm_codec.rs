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
    TEST_JPEG, TEST_PNG, TEST_WEBP, extract_bytes, extract_metadata, init_panic_hook, noop_callback,
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

    // --- Verify the metadata JSON contains all expected fields ---
    //
    // These fields are built in compress.rs and serialized in wasm_bridge.rs.
    // If any field name gets mangled during WASM serialization, this catches it.
    assert!(
        json_str.contains("filename"),
        "PNG metadata should contain 'filename': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressed"),
        "PNG output filename should contain 'compressed': got '{json_str}'"
    );
    assert!(
        json_str.contains("mimeType"),
        "PNG metadata should contain 'mimeType': got '{json_str}'"
    );
    assert!(
        json_str.contains("image/png"),
        "PNG MIME type should be 'image/png': got '{json_str}'"
    );
    assert!(
        json_str.contains("originalSize"),
        "Metadata should contain 'originalSize': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressedSize"),
        "Metadata should contain 'compressedSize': got '{json_str}'"
    );
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

    assert!(
        json_str.contains("filename"),
        "WebP metadata should contain 'filename': got '{json_str}'"
    );
    assert!(
        json_str.contains("compressed"),
        "WebP output filename should contain 'compressed': got '{json_str}'"
    );
    assert!(
        json_str.contains("mimeType"),
        "WebP metadata should contain 'mimeType': got '{json_str}'"
    );
    assert!(
        json_str.contains("image/webp"),
        "WebP MIME type should be 'image/webp': got '{json_str}'"
    );
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

    let result = compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"quality": 60}"#, callback);
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
    // The filename is now available both in the metadata JSON string AND
    // as a top-level property on the combined result object. We check the
    // metadata JSON string for backward-compatible assertions.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(TEST_JPEG, "my-photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(result.is_ok(), "compress_image_combined should succeed");

    // --- Extract metadata JSON and check filename ---
    let result_obj = result.unwrap();
    let json_str = extract_metadata(&result_obj);
    assert!(
        json_str.contains("my-photo-compressed.jpg"),
        "Output filename should be 'my-photo-compressed.jpg': got '{json_str}'"
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
    let json_str = extract_metadata(&result_obj);
    assert!(
        json_str.contains("chart-compressed.png"),
        "Output filename should be 'chart-compressed.png': got '{json_str}'"
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
    let json_str = extract_metadata(&result_obj);
    assert!(
        json_str.contains("hero-image-compressed.webp"),
        "Output filename should be 'hero-image-compressed.webp': got '{json_str}'"
    );
}

// =============================================================================
// Quality Parameter — Affects Output Size Across WASM
// =============================================================================

#[wasm_bindgen_test]
fn test_jpeg_lower_quality_produces_smaller_output() {
    // --- Test: JPEG quality parameter affects output size through WASM ---
    //
    // Lower quality = more aggressive compression = smaller file.
    // This verifies that the quality parameter actually makes it through
    // the JSON → Rust parsing → JPEG encoder pipeline across WASM.
    //
    // If the params_json parsing was broken, both would use the default
    // quality and produce identical sizes — this test would catch that.
    //
    // We extract bytes from the combined result's `data` property.
    init_panic_hook();

    // --- Compress at quality 50 (aggressive) ---
    let result_q50 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"quality": 50}"#,
        noop_callback(),
    );
    assert!(result_q50.is_ok(), "Quality 50 should succeed");
    let bytes_q50 = extract_bytes(&result_q50.unwrap());

    // --- Compress at quality 95 (minimal compression) ---
    let result_q95 = compress_image_combined(
        TEST_JPEG,
        "photo.jpg",
        r#"{"quality": 95}"#,
        noop_callback(),
    );
    assert!(result_q95.is_ok(), "Quality 95 should succeed");
    let bytes_q95 = extract_bytes(&result_q95.unwrap());

    // Quality 50 output should be smaller than quality 95 output.
    // Both are valid JPEGs (checked by magic bytes tests in wasm.rs).
    assert!(
        bytes_q50.len() < bytes_q95.len(),
        "Quality 50 ({} bytes) should produce smaller output than quality 95 ({} bytes)",
        bytes_q50.len(),
        bytes_q95.len()
    );
}
