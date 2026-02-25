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
// RUST CONCEPT: `#![allow(dead_code)]`
// Each test file imports this module but only uses a subset of the
// fixtures and helpers. Rust's compiler warns about the unused ones
// (it checks per-crate, and each test file is its own crate). We
// silence these warnings because the items ARE used — just by
// different test files.
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// =============================================================================
// Test Fixtures — CSV data embedded at compile time
// =============================================================================
//
// These are CSV test files from the shared test-fixtures directory. They're
// compiled INTO the test binary, so no filesystem access is needed at
// runtime. `include_bytes!()` reads a file at compile time and returns
// a `&[u8]` (byte slice) — like a hardcoded array of bytes.

/// A simple, clean CSV with 3 columns and 5 data rows.
/// No whitespace issues, no empty rows, no duplicates.
/// Used as a baseline "nothing to clean" test case.
pub const SIMPLE_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/simple.csv");

/// A messy CSV with whitespace, empty rows, and duplicates.
/// This is the "kitchen sink" test case that exercises all
/// cleaning operations at once.
pub const MESSY_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/messy.csv");

/// A CSV with only a header row and no data.
/// Edge case: should return the header with zero data rows.
pub const HEADERS_ONLY_CSV: &[u8] =
    include_bytes!("../../../../../test-fixtures/csv/headers-only.csv");

/// A CSV with many columns (8) for rename-columns testing.
/// Has first_name, last_name, email, phone, city, state, zip, department.
pub const MANY_COLUMNS_CSV: &[u8] =
    include_bytes!("../../../../../test-fixtures/csv/many-columns.csv");

// =============================================================================
// Inline Test Fixtures — For quick, specific test cases
// =============================================================================

/// Minimal 3-column CSV for rename-columns tests.
/// "name", "age", "city" — easy to verify column renames on.
pub const MINIMAL_CSV: &[u8] = b"name,age,city\nAlice,30,NYC\nBob,25,LA\n";

/// Simple CSV as raw bytes (for tests that don't need file fixtures).
pub const CSV_WITH_EMPTY_ROWS: &[u8] = b"name,age\nAlice,30\n,,\nBob,25\n,,\n";

/// CSV with exact duplicate rows.
pub const CSV_WITH_DUPLICATES: &[u8] = b"name,age\nAlice,30\nBob,25\nAlice,30\n";

/// CSV with whitespace around cell values.
pub const CSV_WITH_WHITESPACE: &[u8] = b"name,age\n  Alice  , 30 \n Bob ,25\n";

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
/// Returns a tuple of (callback_function, calls_array). After processing,
/// inspect `calls_array` to verify progress was reported correctly.
///
/// HOW IT WORKS:
/// We use `js_sys::eval()` to create a JS closure that captures an array.
/// Every time the callback is called, it pushes [percent, message] into
/// the array. After the WASM function returns, we read the array to see
/// what progress updates were reported.
pub fn recording_callback() -> (js_sys::Function, js_sys::Array) {
    // --- Step 1: Create a JS object with a callback and an array ---
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
