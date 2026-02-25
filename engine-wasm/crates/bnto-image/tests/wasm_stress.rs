// =============================================================================
// WASM Integration Tests — Large File Stress & Memory Tests
// =============================================================================
//
// WHAT ARE THESE TESTS?
// WASM has a linear memory model — all heap allocations come from a single
// contiguous buffer that grows on demand. Large images need:
//   1. Input bytes loaded into WASM memory
//   2. Decoded pixel data (width × height × 4 bytes for RGBA)
//   3. Encoded output bytes
//
// A 1200×800 image needs ~3.8 MB for pixels alone, plus input + output.
// These tests verify the WASM runtime can handle 1MB+ inputs without
// running out of memory or crashing.
//
// WHY "OUTPUT <= INPUT" FOR JPEG?
// For JPEG with quality < 100, re-encoding at a lower quality should
// produce smaller output. PNG and WebP use lossless compression, so
// output size depends on the image content and original compression —
// we only assert success (no crash), not smaller output for those.
//
// MEMORY BUDGET:
//   Input:  ~1 MB (large.png)
//   Pixels: ~3.8 MB (1200 × 800 × 4 bytes RGBA)
//   Output: ~variable
//   Total:  ~6+ MB peak WASM memory usage
//   Limit:  ~256 MB (default WASM memory maximum)

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{LARGE_JPEG, LARGE_PNG, LARGE_WEBP, init_panic_hook, noop_callback};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// OOM Safety — Can WASM Handle Real-World File Sizes?
// =============================================================================

#[wasm_bindgen_test]
fn test_large_jpeg_no_oom() {
    // --- Test: Process a 173 KB JPEG without WASM memory issues ---
    //
    // The decoded pixel data for a 1200×800 image is ~3.8 MB (RGBA).
    // Plus the input buffer (~173 KB) and output buffer (variable).
    // Total WASM memory needed: ~4-5 MB. The default WASM memory
    // allows growth to ~256 MB, so this should be fine — but it's
    // worth verifying that the memory management works correctly
    // across the WASM boundary.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(LARGE_JPEG, "big-photo.jpg", r#"{"quality": 80}"#, callback);

    assert!(
        result.is_ok(),
        "Large JPEG compression should not OOM: {:?}",
        result.err()
    );

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Large JPEG output should not be empty");

    // --- Verify output is valid JPEG ---
    assert_eq!(bytes[0], 0xFF, "Large JPEG output should start with FF");
    assert_eq!(bytes[1], 0xD8, "Large JPEG output should have D8 marker");
}

#[wasm_bindgen_test]
fn test_large_png_no_oom() {
    // --- Test: Process a 1 MB+ PNG without crashing ---
    //
    // This is the biggest test fixture: 1,076,419 bytes (>1 MB).
    // The decoded pixel data is ~3.8 MB. With input + output buffers,
    // we need ~6+ MB of WASM memory. This test proves the WASM
    // runtime can handle real-world file sizes.
    //
    // NOTE: We don't assert output <= input for PNG because:
    // PNG compression is lossless — the output size depends on the
    // original compression level. If the original was already optimally
    // compressed, re-encoding with Best/Adaptive might produce a
    // similar or slightly larger file.
    init_panic_hook();
    let callback = noop_callback();

    // --- Verify this is actually a 1 MB+ file ---
    //
    // This assertion guards the test itself. If someone replaces the
    // fixture with a smaller image, this test would no longer verify
    // the "1 MB+" requirement from the plan.
    assert!(
        LARGE_PNG.len() > 1_000_000,
        "Test fixture should be >1 MB, got {} bytes",
        LARGE_PNG.len()
    );

    let result = compress_image_bytes(LARGE_PNG, "big-screenshot.png", "{}", callback);

    assert!(
        result.is_ok(),
        "Large PNG compression should not OOM: {:?}",
        result.err()
    );

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Large PNG output should not be empty");

    // --- Verify output is valid PNG ---
    assert_eq!(bytes[0], 0x89, "Large PNG output should start with 0x89");
    assert_eq!(bytes[1], 0x50, "Large PNG output should have 'P'");
    assert_eq!(bytes[2], 0x4E, "Large PNG output should have 'N'");
    assert_eq!(bytes[3], 0x47, "Large PNG output should have 'G'");
}

#[wasm_bindgen_test]
fn test_large_webp_no_oom() {
    // --- Test: Process a large WebP without crashing ---
    //
    // WebP lossless re-encoding also needs decoded pixel data in memory.
    // This verifies the WebP codec path handles large images correctly.
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_bytes(LARGE_WEBP, "big-banner.webp", "{}", callback);

    assert!(
        result.is_ok(),
        "Large WebP compression should not OOM: {:?}",
        result.err()
    );

    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Large WebP output should not be empty");

    // --- Verify output is valid WebP (RIFF container) ---
    assert_eq!(bytes[0], b'R', "Large WebP output should start with RIFF");
    assert_eq!(bytes[1], b'I');
    assert_eq!(bytes[2], b'F');
    assert_eq!(bytes[3], b'F');
    assert_eq!(bytes[8], b'W', "Large WebP RIFF should contain WEBP");
    assert_eq!(bytes[9], b'E');
    assert_eq!(bytes[10], b'B');
    assert_eq!(bytes[11], b'P');
}

// =============================================================================
// Output Size — Lossy Compression Actually Compresses
// =============================================================================

#[wasm_bindgen_test]
fn test_large_jpeg_output_smaller_than_input() {
    // --- Test: Lossy JPEG compression produces smaller output ---
    //
    // At quality 80 (default), re-encoding a JPEG should almost always
    // produce a smaller file. This is the core value proposition of the
    // compress-images bnto: "your files get smaller."
    //
    // We test this with the large JPEG because larger images show more
    // dramatic compression gains — there's more redundant data to remove.
    init_panic_hook();
    let callback = noop_callback();

    let input_size = LARGE_JPEG.len();

    let result = compress_image_bytes(LARGE_JPEG, "big-photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    let bytes = result.unwrap();
    let output_size = bytes.len();

    assert!(
        output_size <= input_size,
        "Compressed JPEG ({output_size} bytes) should be <= original ({input_size} bytes) at quality 80"
    );
}
