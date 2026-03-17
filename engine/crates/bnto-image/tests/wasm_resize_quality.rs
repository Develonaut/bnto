// WASM integration tests -- resize aspect ratio, upscale, large image, errors, progress.

mod common;

use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{
    LARGE_JPEG, TEST_JPEG, extract_bytes, init_panic_hook, noop_callback, recording_callback,
};

wasm_bindgen_test_configure!(run_in_node_experimental);

// --- Height-Only and Aspect Ratio Tests ---

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

// --- Upscale Test ---

#[wasm_bindgen_test]
fn test_resize_upscale_via_wasm() {
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

// --- Large Image Resize Test ---

#[wasm_bindgen_test]
fn test_resize_large_jpeg_via_wasm() {
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

// --- Error Handling Tests ---

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

// --- Progress Callback Tests ---

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
