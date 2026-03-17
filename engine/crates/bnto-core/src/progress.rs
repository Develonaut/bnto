// =============================================================================
// Progress Reporting — How Nodes Talk to the UI
// =============================================================================
//
// Target-agnostic progress reporting. Uses a plain Rust closure instead of
// `js_sys::Function` so bnto-core stays platform-independent. The WASM-specific
// wrapping lives in each node crate's `wasm_bridge.rs`.

// =============================================================================
// ProgressReporter
// =============================================================================

/// Reports processing progress from a node back to the caller (UI, CLI, etc.).
///
/// Wraps an optional boxed closure. `Some(callback)` for real reporting,
/// `None` for no-op mode (used in tests).
pub struct ProgressReporter {
    /// The callback function. When we call it, it sends a progress update
    /// to wherever the caller wants (UI thread, console, etc.).
    /// `None` = no-op mode (for tests or when progress isn't needed).
    #[allow(clippy::type_complexity)]
    callback: Option<Box<dyn Fn(u32, &str)>>,
}

impl ProgressReporter {
    /// Create a new ProgressReporter with a callback function.
    ///
    /// The callback receives two arguments:
    ///   1. progress (u32, 0-100) — percentage complete
    ///   2. message (&str) — human-readable status text
    ///
    /// USAGE:
    /// ```rust
    /// use bnto_core::ProgressReporter;
    ///
    /// // Simple logger
    /// let reporter = ProgressReporter::new(|percent, message| {
    ///     println!("{}% — {}", percent, message);
    /// });
    ///
    /// // In a WASM bridge, wrap a js_sys::Function:
    /// // let reporter = ProgressReporter::new(move |percent, message| {
    /// //     let _ = js_callback.call2(&JsValue::NULL, &percent.into(), &message.into());
    /// // });
    /// ```
    pub fn new(callback: impl Fn(u32, &str) + 'static) -> Self {
        Self {
            callback: Some(Box::new(callback)),
        }
    }

    /// Create a no-op reporter that discards all progress updates.
    /// Used in tests where we don't need progress reporting.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    /// Report progress to the caller.
    ///
    /// Arguments:
    ///   - `percent` — how far along we are (0 to 100)
    ///   - `message` — what we're currently doing ("Compressing image 3/10...")
    pub fn report(&self, percent: u32, message: &str) {
        if let Some(cb) = &self.callback {
            cb(percent, message);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    #[test]
    fn test_noop_reporter_doesnt_panic() {
        // The no-op reporter should silently accept progress updates
        // without crashing, even though there's no callback.
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

    #[test]
    fn test_reporter_calls_callback() {
        // Create a shared Vec to record calls. We use Arc<Mutex<Vec>>
        // so both the closure and the test body can access the data.
        let calls: Arc<Mutex<Vec<(u32, String)>>> = Arc::new(Mutex::new(Vec::new()));

        // Clone the Arc for the closure. This gives the closure its own
        // "handle" to the shared Vec. The closure and the test body now
        // both hold a reference to the SAME underlying Vec.
        let calls_clone = Arc::clone(&calls);

        // Create a reporter with a closure that records each call.
        let reporter = ProgressReporter::new(move |percent, message| {
            // `.lock().unwrap()` acquires the mutex lock. If another thread
            // held it, we'd wait. `.unwrap()` panics if the mutex is poisoned
            // (another thread panicked while holding the lock).
            calls_clone
                .lock()
                .unwrap()
                .push((percent, message.to_string()));
        });

        // Report some progress.
        reporter.report(0, "Starting...");
        reporter.report(50, "Halfway there...");
        reporter.report(100, "Done!");

        // Verify the callback was called with the right arguments.
        let recorded = calls.lock().unwrap();
        assert_eq!(recorded.len(), 3, "Should have recorded 3 calls");
        assert_eq!(recorded[0], (0, "Starting...".to_string()));
        assert_eq!(recorded[1], (50, "Halfway there...".to_string()));
        assert_eq!(recorded[2], (100, "Done!".to_string()));
    }

    #[test]
    fn test_reporter_with_callback_has_some() {
        // A reporter created with `new()` should have Some(callback).
        let reporter = ProgressReporter::new(|_percent, _message| {
            // No-op for this test — we just want to check it's Some.
        });
        assert!(reporter.callback.is_some());
    }
}
