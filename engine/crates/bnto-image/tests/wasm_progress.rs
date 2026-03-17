// WASM Integration Tests -- progress callback verification.
//
// Verifies that progress callbacks fire correctly across the WASM boundary:
// callbacks actually fire, percentages are non-decreasing, messages are real
// strings, and the final progress is always 100%.

mod common;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use bnto_image::wasm_bridge::*;
use common::{TEST_JPEG, TEST_PNG, TEST_WEBP, init_panic_hook, recording_callback};

wasm_bindgen_test_configure!(run_in_node_experimental);

// =========================================================================
// Per-Format Progress Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_progress_callback_fires_for_jpeg() {
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result =
        compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"compression": 20}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "Progress should fire at least 3 times (got {call_count} calls)"
    );

    // Verify percentages are non-decreasing.
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

    assert!(
        (last_percent - 100.0).abs() < f64::EPSILON,
        "Final progress should be 100%, got {last_percent}"
    );
}

#[wasm_bindgen_test]
fn test_progress_callback_fires_for_png() {
    // PNG uses a different code path (lossless, no quality param).
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_combined(TEST_PNG, "screenshot.png", "{}", callback);
    assert!(result.is_ok(), "PNG compression should succeed");

    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "PNG progress should fire at least 3 times (got {call_count} calls)"
    );

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
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result = compress_image_combined(TEST_WEBP, "banner.webp", "{}", callback);
    assert!(result.is_ok(), "WebP compression should succeed");

    let call_count = calls.length();
    assert!(
        call_count >= 3,
        "WebP progress should fire at least 3 times (got {call_count} calls)"
    );

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

// =========================================================================
// Message Content Tests
// =========================================================================

#[wasm_bindgen_test]
fn test_progress_messages_are_nonempty_strings() {
    // Every progress message must be a real string -- the UI shows these
    // in the status bar. Catches broken string conversion across WASM.
    init_panic_hook();

    let (callback, calls) = recording_callback();

    let result =
        compress_image_combined(TEST_JPEG, "photo.jpg", r#"{"compression": 20}"#, callback);
    assert!(result.is_ok(), "Compression should succeed");

    for i in 0..calls.length() {
        let pair = calls
            .get(i)
            .dyn_into::<js_sys::Array>()
            .expect("Each call should be an array");

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
