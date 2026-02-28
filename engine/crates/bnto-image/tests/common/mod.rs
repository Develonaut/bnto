// =============================================================================
// Shared Test Helpers — Used by All WASM Integration Test Files
// =============================================================================
//
// WHAT IS THIS FILE?
// Common fixtures, helpers, and constants shared across WASM integration
// test files. In Rust, integration tests (files in `tests/`) are each
// compiled as separate crates. This `common/mod.rs` module is imported
// by each test file to avoid duplicating setup code.
//
// RUST CONCEPT: `tests/common/mod.rs`
// Rust's test runner treats each file in `tests/` as an independent
// test crate. To share code, you put it in `tests/common/mod.rs` and
// import it with `mod common;` in each test file. The `common` module
// is NOT itself compiled as a test — it's just a library for tests.
//
// RUST CONCEPT: `#[allow(dead_code)]`
// Each test file imports this module but only uses a subset of the
// fixtures and helpers. Rust's compiler warns about the unused ones
// (it checks per-crate, and each test file is its own crate). We
// silence these warnings because the items ARE used — just by
// different test files.
#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// =============================================================================
// Test Fixtures — Images embedded at compile time
// =============================================================================
//
// These are real photos from the shared test-fixtures directory. They're
// compiled INTO the WASM binary, so no filesystem access is needed at
// runtime. `include_bytes!()` reads a file at compile time and returns
// a `&[u8]` (byte slice) — like a hardcoded array of bytes.

/// A small JPEG image (100x100 pixels, ~3.5 KB)
pub const TEST_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

/// A small PNG image (100x100 pixels, ~16 KB)
pub const TEST_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

/// A small WebP image (100x100 pixels, ~1.8 KB)
pub const TEST_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.webp");

/// A large JPEG image (1200x800 pixels, ~173 KB) — for stress testing
pub const LARGE_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.jpg");

/// A large PNG image (1200x800 pixels, ~1 MB) — for OOM testing
pub const LARGE_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.png");

/// A large WebP image (1200x800 pixels, ~85 KB) — for stress testing
pub const LARGE_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.webp");

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a JavaScript function that does nothing (for progress callbacks).
///
/// Most tests don't care about progress reporting — they just need a
/// valid callback to pass to the WASM functions. This creates a JS
/// `function() {}` that accepts any arguments and does nothing.
pub fn noop_callback() -> js_sys::Function {
    js_sys::eval("(function() {})")
        .expect("Failed to create noop callback")
        .dyn_into::<js_sys::Function>()
        .expect("eval result should be a Function")
}

/// Create a JavaScript callback that records every (percent, message) call.
///
/// Returns a tuple of (callback_function, calls_array). After compression,
/// inspect `calls_array` to verify progress was reported correctly.
///
/// HOW IT WORKS:
/// We use `js_sys::eval()` to create a JS closure that captures an array.
/// Every time the callback is called, it pushes [percent, message] into
/// the array. After the WASM function returns, we read the array to see
/// what progress updates were reported.
///
/// RUST CONCEPT: `js_sys::Reflect::get()`
/// This is Rust's equivalent of JavaScript's `object.property` or
/// `object["property"]`. We use it to extract the `cb` and `calls`
/// properties from the object returned by our eval'd JS code.
pub fn recording_callback() -> (js_sys::Function, js_sys::Array) {
    // --- Step 1: Create a JS object with a callback and an array ---
    //
    // The IIFE (Immediately Invoked Function Expression) creates a
    // closure scope where `calls` lives. The callback captures `calls`
    // by reference (JS closures work like this naturally).
    let obj = js_sys::eval(
        r#"(function() {
            var calls = [];
            var cb = function(percent, message) { calls.push([percent, message]); };
            return { calls: calls, cb: cb };
        })()"#,
    )
    .expect("Failed to create recording callback");

    // --- Step 2: Extract the callback function ---
    let cb = js_sys::Reflect::get(&obj, &JsValue::from_str("cb"))
        .expect("Should have 'cb' property")
        .dyn_into::<js_sys::Function>()
        .expect("'cb' should be a Function");

    // --- Step 3: Extract the calls array ---
    let calls = js_sys::Reflect::get(&obj, &JsValue::from_str("calls"))
        .expect("Should have 'calls' property")
        .dyn_into::<js_sys::Array>()
        .expect("'calls' should be an Array");

    (cb, calls)
}

/// Initialize panic hook for test reliability.
///
/// In production, the bnto-wasm entry point handles this. In these
/// integration tests, the bnto-wasm entry point isn't loaded, so we
/// skip the panic hook — errors will still show in the test output,
/// just without the pretty console.error formatting.
pub fn init_panic_hook() {
    // NOTE: console_error_panic_hook was moved to bnto-wasm.
    // These tests still work fine without it — Rust test runner captures panics.
}

// =============================================================================
// Combined Result Extraction Helpers
// =============================================================================
//
// The combined WASM functions (compress_image_combined, resize_image_combined,
// convert_image_format_combined) return a JS object with four properties:
//   - metadata:  JSON string with operation metadata
//   - data:      Uint8Array of the output image bytes
//   - filename:  Output filename string (e.g., "photo-compressed.jpg")
//   - mimeType:  MIME type string (e.g., "image/jpeg")
//
// These helpers extract each property from the returned JsValue so tests
// don't need to repeat the Reflect::get + type conversion boilerplate.
//
// RUST CONCEPT: `js_sys::Reflect::get()`
// This is Rust's equivalent of JavaScript's property access (obj.key or
// obj["key"]). It returns a JsValue that we then convert to the expected
// Rust type using `.as_string()` (for strings) or `.dyn_into()` (for
// typed JS objects like Uint8Array).

/// Extract metadata JSON string from a combined function result.
///
/// The metadata is a serialized JSON string containing fields like
/// originalSize, compressedSize, compressionRatio, format, etc.
/// Each operation type (compress, resize, convert) has its own
/// metadata shape.
pub fn extract_metadata(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"metadata".into())
        .expect("result should have metadata property")
        .as_string()
        .expect("metadata should be a string")
}

/// Extract raw bytes from a combined function result.
///
/// The data property is a Uint8Array containing the output image bytes.
/// We convert it to a Vec<u8> (Rust's owned byte vector) so tests can
/// inspect individual bytes (e.g., checking magic number headers).
///
/// RUST CONCEPT: `dyn_into::<js_sys::Uint8Array>()`
/// `dyn_into` attempts a runtime type cast from a generic JsValue to
/// a specific JavaScript type. It returns Result<T, JsValue> — Ok if
/// the JS value is actually a Uint8Array, Err if it's something else.
pub fn extract_bytes(result: &JsValue) -> Vec<u8> {
    let data = js_sys::Reflect::get(result, &"data".into())
        .expect("result should have data property");
    let array: js_sys::Uint8Array = data.dyn_into()
        .expect("data should be a Uint8Array");
    array.to_vec()
}

/// Extract output filename from a combined function result.
///
/// The filename includes the operation suffix (e.g., "-compressed",
/// "-resized") and preserves the appropriate file extension.
pub fn extract_filename(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"filename".into())
        .expect("result should have filename property")
        .as_string()
        .expect("filename should be a string")
}

/// Extract MIME type from a combined function result.
///
/// Returns the MIME type string for the output image, e.g.,
/// "image/jpeg", "image/png", or "image/webp".
pub fn extract_mime_type(result: &JsValue) -> String {
    js_sys::Reflect::get(result, &"mimeType".into())
        .expect("result should have mimeType property")
        .as_string()
        .expect("mimeType should be a string")
}
