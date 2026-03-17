// Shared test helpers for bnto-file WASM integration tests.
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// --- Test Fixtures ---

/// Simple test data -- content doesn't matter for rename operations.
pub const TEST_FILE_DATA: &[u8] = b"Hello, this is test file content.";

/// Larger test data -- verifies data integrity with bigger payloads.
pub const LARGER_TEST_DATA: &[u8] = b"This is a larger test file with more content.\n\
    It has multiple lines and enough data to verify that\n\
    the rename operation passes all bytes through unchanged,\n\
    regardless of the file size or content.";

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
pub fn extract_metadata(result: &wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(result, &"metadata".into())
        .expect("result should have metadata property")
        .as_string()
        .expect("metadata should be a string")
}

/// Extract raw bytes from a combined function result.
pub fn extract_bytes(result: &wasm_bindgen::JsValue) -> Vec<u8> {
    use wasm_bindgen::JsCast;
    let data =
        js_sys::Reflect::get(result, &"data".into()).expect("result should have data property");
    let array: js_sys::Uint8Array = data.dyn_into().expect("data should be a Uint8Array");
    array.to_vec()
}

/// Extract output filename from a combined function result.
pub fn extract_filename(result: &wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(result, &"filename".into())
        .expect("result should have filename property")
        .as_string()
        .expect("filename should be a string")
}
