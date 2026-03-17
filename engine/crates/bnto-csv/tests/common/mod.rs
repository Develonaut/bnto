// Shared test helpers for all bnto-csv WASM integration tests.
#![allow(dead_code)]

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// =========================================================================
// Test Fixtures -- CSV data embedded at compile time
// =========================================================================

/// Clean CSV with 3 columns and 5 data rows. Baseline "nothing to clean" case.
pub const SIMPLE_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/simple.csv");

/// Messy CSV with whitespace, empty rows, and duplicates. Exercises all cleaning ops.
pub const MESSY_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/messy.csv");

/// Header row only, no data. Edge case: should return header with zero data rows.
pub const HEADERS_ONLY_CSV: &[u8] =
    include_bytes!("../../../../../test-fixtures/csv/headers-only.csv");

/// 8-column CSV for rename-columns testing.
pub const MANY_COLUMNS_CSV: &[u8] =
    include_bytes!("../../../../../test-fixtures/csv/many-columns.csv");

// =========================================================================
// Inline Test Fixtures
// =========================================================================

pub const MINIMAL_CSV: &[u8] = b"name,age,city\nAlice,30,NYC\nBob,25,LA\n";
pub const CSV_WITH_EMPTY_ROWS: &[u8] = b"name,age\nAlice,30\n,,\nBob,25\n,,\n";
pub const CSV_WITH_DUPLICATES: &[u8] = b"name,age\nAlice,30\nBob,25\nAlice,30\n";
pub const CSV_WITH_WHITESPACE: &[u8] = b"name,age\n  Alice  , 30 \n Bob ,25\n";

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

/// Panic hook no-op -- bnto-wasm entry point handles this in production.
pub fn init_panic_hook() {}
