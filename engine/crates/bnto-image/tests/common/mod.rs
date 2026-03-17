// Shared test helpers for all bnto-image WASM integration tests.
#![allow(dead_code)]

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// =========================================================================
// Test Fixtures -- images embedded at compile time
// =========================================================================

/// Small JPEG (100x100, ~3.5 KB)
pub const TEST_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

/// Small PNG (100x100, ~16 KB)
pub const TEST_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

/// Small WebP (100x100, ~1.8 KB)
pub const TEST_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.webp");

/// Large JPEG (1200x800, ~173 KB) -- for stress testing
pub const LARGE_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.jpg");

/// Large PNG (1200x800, ~1 MB) -- for OOM testing
pub const LARGE_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.png");

/// Large WebP (1200x800, ~85 KB) -- for stress testing
pub const LARGE_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.webp");

// =========================================================================
// Helper Functions
// =========================================================================

/// No-op JS callback for tests that don't need progress reporting.
pub fn noop_callback() -> js_sys::Function {
    js_sys::eval("(function() {})")
        .expect("Failed to create noop callback")
        .dyn_into::<js_sys::Function>()
        .expect("eval result should be a Function")
}

/// JS callback that records every (percent, message) call into an array.
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

/// Panic hook no-op -- bnto-wasm entry point handles this in production.
pub fn init_panic_hook() {}

// =========================================================================
// Combined Result Extraction Helpers
// =========================================================================
//
// Combined WASM functions return a JS object with:
//   { metadata: JSON string, data: Uint8Array, filename: string, mimeType: string }

pub fn extract_metadata(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"metadata".into())
        .expect("result should have metadata property")
        .as_string()
        .expect("metadata should be a string")
}

pub fn extract_bytes(result: &JsValue) -> Vec<u8> {
    let data =
        js_sys::Reflect::get(result, &"data".into()).expect("result should have data property");
    let array: js_sys::Uint8Array = data.dyn_into().expect("data should be a Uint8Array");
    array.to_vec()
}

pub fn extract_filename(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"filename".into())
        .expect("result should have filename property")
        .as_string()
        .expect("filename should be a string")
}

pub fn extract_mime_type(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"mimeType".into())
        .expect("result should have mimeType property")
        .as_string()
        .expect("mimeType should be a string")
}
