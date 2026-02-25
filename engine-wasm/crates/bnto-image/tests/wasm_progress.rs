// =============================================================================
// WASM Integration Tests — Progress Callback Verification
// =============================================================================
//
// WHAT ARE THESE TESTS?
// When the WASM code processes an image, it calls a JavaScript callback
// function to report progress (e.g., "10% — Decoding JPEG..."). These tests
// verify that:
//   1. The callback actually fires (not silently dropped)
//   2. Percentages are non-decreasing (0% → 50% → 100%, never backwards)
//   3. Messages are real strings (not empty, not undefined)
//   4. The final progress is always 100%
//
// WHY THIS MATTERS:
// The progress callback crosses the WASM boundary via `js_sys::Function`.
// The Rust code calls `.call2(&JsValue::NULL, &percent, &message)` and
// the JS function receives two arguments. If the type conversion breaks
// (e.g., percent arrives as a string instead of a number), the UI would
// show garbage. These tests catch that.
//
// THE PROGRESS PIPELINE:
//   Rust code calls `progress.report(50, "Compressing...")`
//     → ProgressReporter wraps args as JsValue (number + string)
//     → Calls js_sys::Function.call2() across the WASM boundary
//     → JavaScript callback receives (percent: number, message: string)
//     → In production, the Web Worker forwards this via postMessage()
//     → Main thread updates the UI progress bar

mod common;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{TEST_JPEG, TEST_PNG, TEST_WEBP, init_panic_hook, recording_callback};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Per-Format Progress Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_for_jpeg() {
    // --- Test: JPEG compression fires progress updates ---
    //
    // The compress pipeline reports progress at these points:
    //   5%  — "Compressing JPEG (quality: N)..."
    //   10% — "Decoding JPEG..."
    //   50% — "Compressing JPEG..."
    //   100% — "JPEG compression complete"
    //
    // We verify at least 3 calls fire and percentages never go backwards.
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_bytes(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    // --- Verify the callback was called multiple times ---
    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "Progress should fire at least 3 times (got {call_count} calls)"
    );

    // --- Verify percentages are non-decreasing ---
    //
    // Each call is a JS array [percent, message]. We iterate through
    // them and check that percent never goes backwards.
    //
    // RUST CONCEPT: `dyn_into::<js_sys::Array>()`
    // `dyn_into` tries to cast a JsValue to a specific JS type.
    // It returns `Result<T, JsValue>` — Ok if the cast succeeds,
    // Err if the JS value isn't the expected type. `.unwrap()` panics
    // if the cast fails, which is fine in tests.
    let mut last_percent: f64 = -1.0;
    for i in 0..call_count {
        let pair = calls
            .get(i)
            .dyn_into::<js_sys::Array>()
            .expect("Each call should be an array [percent, message]");

        let percent = pair.get(0).as_f64().expect("Percent should be a number");

        assert!(
            percent >= last_percent,
            "Progress should be non-decreasing: got {percent} after {last_percent}"
        );
        last_percent = percent;
    }

    // --- Verify the final progress is 100% ---
    assert!(
        (last_percent - 100.0).abs() < f64::EPSILON,
        "Final progress should be 100%, got {last_percent}"
    );
}

#[wasm_bindgen_test]
fn test_progress_callback_fires_for_png() {
    // --- Test: PNG compression also fires progress updates ---
    //
    // PNG uses a different code path (lossless, no quality param).
    // Verify the progress callback still works for this format.
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_bytes(TEST_PNG, "screenshot.png", "{}", callback);
    assert!(result.is_ok(), "PNG compression should succeed");

    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "PNG progress should fire at least 3 times (got {call_count} calls)"
    );

    // --- Verify non-decreasing percentages ---
    let mut last_percent: f64 = -1.0;
    for i in 0..call_count {
        let pair = calls
            .get(i)
            .dyn_into::<js_sys::Array>()
            .expect("Each call should be an array");
        let percent = pair.get(0).as_f64().expect("Percent should be a number");
        assert!(
            percent >= last_percent,
            "PNG progress should be non-decreasing: got {percent} after {last_percent}"
        );
        last_percent = percent;
    }

    assert!(
        (last_percent - 100.0).abs() < f64::EPSILON,
        "PNG final progress should be 100%, got {last_percent}"
    );
}

#[wasm_bindgen_test]
fn test_progress_callback_fires_for_webp() {
    // --- Test: WebP compression fires progress updates ---
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_bytes(TEST_WEBP, "banner.webp", "{}", callback);
    assert!(result.is_ok(), "WebP compression should succeed");

    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "WebP progress should fire at least 3 times (got {call_count} calls)"
    );

    // --- Verify non-decreasing and ends at 100% ---
    let mut last_percent: f64 = -1.0;
    for i in 0..call_count {
        let pair = calls
            .get(i)
            .dyn_into::<js_sys::Array>()
            .expect("Each call should be an array");
        let percent = pair.get(0).as_f64().expect("Percent should be a number");
        assert!(
            percent >= last_percent,
            "WebP progress should be non-decreasing: got {percent} after {last_percent}"
        );
        last_percent = percent;
    }

    assert!(
        (last_percent - 100.0).abs() < f64::EPSILON,
        "WebP final progress should be 100%, got {last_percent}"
    );
}

// =============================================================================
// Message Content Tests
// =============================================================================

#[wasm_bindgen_test]
fn test_progress_messages_are_nonempty_strings() {
    // --- Test: Every progress message is a real, non-empty string ---
    //
    // The progress messages are displayed in the UI's status text.
    // If they arrived as `undefined` or empty string, the UI would
    // show a blank status bar. This catches broken string conversion
    // across the WASM boundary.
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_bytes(TEST_JPEG, "photo.jpg", r#"{"quality": 80}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    for i in 0..calls.length() {
        let pair = calls
            .get(i)
            .dyn_into::<js_sys::Array>()
            .expect("Each call should be an array");

        // --- Check that the message (second element) is a non-empty string ---
        let message = pair.get(1);
        assert!(
            message.is_string(),
            "Progress message at index {i} should be a string, got: {message:?}"
        );

        let message_str = message.as_string().unwrap();
        assert!(
            !message_str.is_empty(),
            "Progress message at index {i} should not be empty"
        );
    }
}
