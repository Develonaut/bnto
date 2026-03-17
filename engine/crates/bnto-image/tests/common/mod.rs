// Shared test helpers for bnto-image WASM integration tests.
#![allow(dead_code)]

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// --- Test Fixtures ---

/// A small JPEG image (100x100 pixels, ~3.5 KB).
pub const TEST_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

/// A small PNG image (100x100 pixels, ~16 KB).
pub const TEST_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

/// A small WebP image (100x100 pixels, ~1.8 KB).
pub const TEST_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.webp");

/// A large JPEG image (1200x800 pixels, ~173 KB) -- for stress testing.
pub const LARGE_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.jpg");

/// A large PNG image (1200x800 pixels, ~1 MB) -- for OOM testing.
pub const LARGE_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.png");

/// A large WebP image (1200x800 pixels, ~85 KB) -- for stress testing.
pub const LARGE_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.webp");

// --- Helper Functions ---

/// Create a no-op JS function for progress callbacks.
pub fn noop_callback() -> js_sys::Function {
    js_sys::eval("(function() {})")
        .expect("Failed to create noop callback")
        .dyn_into::<js_sys::Function>()
        .expect("eval result should be a Function")
}

/// Create a JS callback that records every (percent, message) call.
/// Returns (callback_function, calls_array).
pub fn recording_callback() -> (js_sys::Function, js_sys::Array) {
    let obj = js_sys::eval(
        r#"(function() {
            var calls = [];
            var cb = function(percent, message) { calls.push([percent, message]); };
            return { calls: calls, cb: cb };
        })()"#,
    )
    .expect("Failed to create recording callback");

    let cb = js_sys::Reflect::get(&obj, &JsValue::from_str("cb"))
        .expect("Should have 'cb' property")
        .dyn_into::<js_sys::Function>()
        .expect("'cb' should be a Function");

    let calls = js_sys::Reflect::get(&obj, &JsValue::from_str("calls"))
        .expect("Should have 'calls' property")
        .dyn_into::<js_sys::Array>()
        .expect("'calls' should be an Array");

    (cb, calls)
}

/// No-op panic hook -- bnto-wasm handles this in production.
pub fn init_panic_hook() {}

// --- Combined Result Extraction Helpers ---

/// Extract metadata JSON string from a combined function result.
pub fn extract_metadata(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"metadata".into())
        .expect("result should have metadata property")
        .as_string()
        .expect("metadata should be a string")
}

/// Extract raw bytes from a combined function result.
pub fn extract_bytes(result: &JsValue) -> Vec<u8> {
    let data =
        js_sys::Reflect::get(result, &"data".into()).expect("result should have data property");
    let array: js_sys::Uint8Array = data.dyn_into().expect("data should be a Uint8Array");
    array.to_vec()
}

/// Extract output filename from a combined function result.
pub fn extract_filename(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"filename".into())
        .expect("result should have filename property")
        .as_string()
        .expect("filename should be a string")
}

/// Extract MIME type from a combined function result.
pub fn extract_mime_type(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"mimeType".into())
        .expect("result should have mimeType property")
        .as_string()
        .expect("mimeType should be a string")
}
