// =============================================================================
// Progress Reporting — How Nodes Talk to the UI
// =============================================================================
//
// WHAT IS THIS FILE?
// When a node is processing a file (compressing an image, cleaning a CSV),
// the user wants to see progress: "Processing... 50% done". This module
// provides the ProgressReporter that nodes use to report their progress.
//
// HOW IT WORKS:
// 1. The Web Worker creates a ProgressReporter with a JavaScript callback
// 2. The callback uses `postMessage()` to send progress to the main thread
// 3. The node calls `reporter.report(50, "Compressing...")` during processing
// 4. The main thread updates the UI (progress bar, status text)
//
// WHY A STRUCT (not just a function)?
// A struct lets us:
//   - Store the callback and reuse it without passing it everywhere
//   - Add a "no-op" mode for testing (no JS callback needed)
//   - Track state (like preventing backwards progress)
//   - Add rate limiting later (don't flood the UI with updates)

use wasm_bindgen::prelude::*;

// =============================================================================
// ProgressReporter
// =============================================================================

/// Reports processing progress from a node back to the JavaScript UI.
///
/// RUST CONCEPT: `Option<js_sys::Function>`
/// `Option<T>` means "maybe a T, maybe nothing". Here, the callback
/// might be `Some(function)` (real reporting) or `None` (no-op mode,
/// used in tests). This avoids needing a separate "mock" type for testing.
///
/// RUST CONCEPT: `js_sys::Function`
/// This is a Rust type that represents a JavaScript function. We can
/// call it from Rust using `.call1()`, `.call2()`, etc. The function
/// was created in JavaScript and passed to Rust via wasm-bindgen.
pub struct ProgressReporter {
    /// The JavaScript callback function. When we call it, it sends
    /// a progress update to the UI thread.
    /// `None` = no-op mode (for tests or when progress isn't needed).
    callback: Option<js_sys::Function>,
}

impl ProgressReporter {
    /// Create a new ProgressReporter with a JavaScript callback.
    ///
    /// The callback receives two arguments:
    ///   1. progress (number, 0-100) — percentage complete
    ///   2. message (string) — human-readable status text
    ///
    /// USAGE FROM JAVASCRIPT:
    /// ```js
    /// const reporter = new ProgressReporter((progress, message) => {
    ///     postMessage({ type: 'progress', progress, message });
    /// });
    /// ```
    pub fn new(callback: js_sys::Function) -> Self {
        // RUST CONCEPT: `Self`
        // `Self` is a shorthand for the type we're implementing (ProgressReporter).
        // `Self { callback: Some(callback) }` creates a new ProgressReporter
        // with the callback wrapped in `Some()` (meaning "we have a value").
        Self {
            callback: Some(callback),
        }
    }

    /// Create a no-op reporter that discards all progress updates.
    /// Used in tests where we don't have a JavaScript runtime.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    /// Report progress to the UI.
    ///
    /// Arguments:
    ///   - `percent` — how far along we are (0 to 100)
    ///   - `message` — what we're currently doing ("Compressing image 3/10...")
    ///
    /// RUST CONCEPT: `&self`
    /// `&self` means we're borrowing the reporter (read-only access).
    /// The caller keeps ownership. This is fine because reporting progress
    /// doesn't modify the reporter itself.
    pub fn report(&self, percent: u32, message: &str) {
        // RUST CONCEPT: `if let Some(cb) = &self.callback`
        // This is pattern matching on an Option. If the callback exists
        // (Some), we bind it to `cb` and run the block. If it's None
        // (no-op mode), we skip silently.
        if let Some(cb) = &self.callback {
            // Create JavaScript values from our Rust data.
            // `JsValue::from(percent)` converts a Rust u32 to a JS number.
            // `JsValue::from(message)` converts a Rust &str to a JS string.
            let js_percent = JsValue::from(percent);
            let js_message = JsValue::from(message);

            // Call the JavaScript callback with our two arguments.
            //
            // `.call2()` calls a JS function with 2 arguments.
            // The first argument to call2 is `this` (we use JsValue::NULL
            // because we don't need a `this` context — it's a plain function).
            //
            // `.ok()` converts the Result to an Option, discarding any error.
            // We intentionally ignore callback errors — if the UI thread is
            // busy or the callback throws, we don't want to crash the processing.
            // Progress is "best effort", not critical.
            let _ = cb.call2(&JsValue::NULL, &js_percent, &js_message);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_reporter_doesnt_panic() {
        // The no-op reporter should silently accept progress updates
        // without crashing, even though there's no JS callback.
        let reporter = ProgressReporter::new_noop();

        // These should all succeed silently.
        reporter.report(0, "Starting...");
        reporter.report(50, "Halfway there...");
        reporter.report(100, "Done!");
    }

    #[test]
    fn test_noop_reporter_callback_is_none() {
        let reporter = ProgressReporter::new_noop();

        // In no-op mode, the callback should be None.
        assert!(reporter.callback.is_none());
    }
}
