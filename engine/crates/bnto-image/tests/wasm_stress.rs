// WASM Integration Tests -- large file stress and memory tests.
//
// WASM linear memory model means large images need: input bytes + decoded
// pixels (W x H x 4 RGBA) + encoded output. A 1200x800 image needs ~3.8 MB
// for pixels alone. These tests verify no OOM or crash with 1MB+ inputs.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{LARGE_JPEG, LARGE_PNG, LARGE_WEBP, extract_bytes, init_panic_hook, noop_callback};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// OOM Safety -- Can WASM Handle Real-World File Sizes?
// =========================================================================

#[wasm_bindgen_test]
fn test_large_jpeg_no_oom() {
    // 173 KB JPEG -> ~4-5 MB WASM memory (decoded pixels + buffers).
    init_panic_hook();
    let callback = noop_callback();

    let result =
        compress_image_combined(LARGE_JPEG, "big-photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(
        result.is_ok(),
        "Large JPEG compression should not OOM: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Large JPEG output should not be empty");
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xD8);
}

#[wasm_bindgen_test]
fn test_large_png_no_oom() {
    // 1 MB+ PNG -> ~6+ MB peak WASM memory. Biggest test fixture.
    init_panic_hook();
    let callback = noop_callback();

    // Guard: ensure fixture is actually >1 MB (catches accidental replacement).
    assert!(
        LARGE_PNG.len() > 1_000_000,
        "Test fixture should be >1 MB, got {} bytes",
        LARGE_PNG.len()
    );

    let result = compress_image_combined(LARGE_PNG, "big-screenshot.png", "{}", callback);
    assert!(
        result.is_ok(),
        "Large PNG compression should not OOM: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Large PNG output should not be empty");
    assert_eq!(bytes[0], 0x89);
    assert_eq!(bytes[1], 0x50);
    assert_eq!(bytes[2], 0x4E);
    assert_eq!(bytes[3], 0x47);
}

#[wasm_bindgen_test]
fn test_large_webp_no_oom() {
    init_panic_hook();
    let callback = noop_callback();

    let result = compress_image_combined(LARGE_WEBP, "big-banner.webp", "{}", callback);
    assert!(
        result.is_ok(),
        "Large WebP compression should not OOM: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    assert!(!bytes.is_empty(), "Large WebP output should not be empty");
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
// Output Size -- Lossy Compression Actually Compresses
// =========================================================================

#[wasm_bindgen_test]
fn test_large_jpeg_output_smaller_than_input() {
    // Core value prop: "your files get smaller." Large images show the
    // most dramatic compression gains.
    init_panic_hook();
    let callback = noop_callback();

    let input_size = LARGE_JPEG.len();

    let result =
        compress_image_combined(LARGE_JPEG, "big-photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    let result_obj = result.unwrap();
    let bytes = extract_bytes(&result_obj);
    let output_size = bytes.len();

    assert!(
        output_size <= input_size,
        "Compressed JPEG ({output_size} bytes) should be <= original ({input_size} bytes) at quality 80"
    );
}
