// WASM Integration Tests -- full pipeline tests in Node.js.
//
// Verifies the WASM binary loads, setup()/version()/greet() work across
// JS <-> Rust, and compress_image_combined works end-to-end.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use bnto_wasm::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// Setup Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_setup_does_not_panic() {
    setup();
}

#[wasm_bindgen_test]
fn test_setup_is_idempotent() {
    setup();
    setup();
    setup();
}

// =========================================================================
// Version Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_version_across_wasm_boundary() {
    let v = version();
    assert!(!v.is_empty(), "Version should not be empty");
    assert!(v.contains('.'), "Version should be semver: got '{}'", v);
}

// =========================================================================
// Greet Tests -- String Round-Trip Verification
// =========================================================================

#[wasm_bindgen_test]
fn test_greet_with_ascii_name() {
    let result = greet("Ryan");
    assert!(
        result.contains("Ryan"),
        "Should contain name: got '{}'",
        result
    );
}

/// Unicode (multi-byte UTF-8) can be tricky across the WASM string boundary.
#[wasm_bindgen_test]
fn test_greet_with_unicode_name() {
    let result = greet("Bnto \u{1F371}");
    assert!(
        result.contains("Bnto \u{1F371}"),
        "Should handle Unicode: got '{}'",
        result
    );
}

#[wasm_bindgen_test]
fn test_greet_with_empty_name() {
    let result = greet("");
    assert!(
        result.contains("Bnto WASM engine"),
        "Should still mention engine: got '{}'",
        result
    );
}

// =========================================================================
// Image Compression Tests (via bnto-image combined bridge)
// =========================================================================

const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

#[wasm_bindgen_test]
fn test_compress_image_combined_returns_metadata() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result = bnto_image::wasm_bridge::compress_image_combined(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let metadata_val = js_sys::Reflect::get(&result_obj, &"metadata".into())
        .expect("Result should have 'metadata' property");

    let metadata_str = metadata_val
        .as_string()
        .expect("metadata should be a string");

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&metadata_str);
    assert!(
        parsed.is_ok(),
        "metadata should be valid JSON: {}",
        metadata_str
    );

    let value = parsed.unwrap();
    assert!(
        value.get("originalSize").is_some(),
        "JSON should have 'originalSize' field"
    );
    assert!(
        value.get("compressedSize").is_some(),
        "JSON should have 'compressedSize' field"
    );
}

#[wasm_bindgen_test]
fn test_compress_image_combined_returns_data() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");

    let result = bnto_image::wasm_bridge::compress_image_combined(
        TEST_JPEG,
        "small.jpg",
        r#"{"quality": 80}"#,
        noop_cb,
    );
    assert!(
        result.is_ok(),
        "compress_image_combined should succeed: {:?}",
        result.err()
    );

    let result_obj = result.unwrap();
    let data_val = js_sys::Reflect::get(&result_obj, &"data".into())
        .expect("Result should have 'data' property");

    let uint8_array: js_sys::Uint8Array = data_val
        .dyn_into::<js_sys::Uint8Array>()
        .expect("data should be a Uint8Array");

    let bytes = uint8_array.to_vec();
    assert!(!bytes.is_empty(), "Compressed bytes should not be empty");
    assert_eq!(bytes[0], 0xFF, "Should start with JPEG magic byte 1");
    assert_eq!(bytes[1], 0xD8, "Should start with JPEG magic byte 2");
}

#[wasm_bindgen_test]
fn test_compress_combined_with_default_params() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");
    let result =
        bnto_image::wasm_bridge::compress_image_combined(TEST_JPEG, "small.jpg", "{}", noop_cb);
    assert!(
        result.is_ok(),
        "Default params should work: {:?}",
        result.err()
    );
}

#[wasm_bindgen_test]
fn test_compress_combined_invalid_data_returns_error() {
    let noop_cb = js_sys::Function::new_no_args("return undefined");
    let result = bnto_image::wasm_bridge::compress_image_combined(
        b"this is not an image",
        "garbage.jpg",
        "{}",
        noop_cb,
    );
    assert!(result.is_err(), "Invalid data should return an error");
}
