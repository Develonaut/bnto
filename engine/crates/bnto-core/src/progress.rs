// Per-file progress reporting — how nodes tell the UI "I'm 50% done".
//
// Target-agnostic: uses plain Rust closures instead of js_sys::Function.
// The WASM-specific wrapping (JS type conversion) lives in each node crate's
// wasm_bridge.rs, where it belongs.

// =============================================================================
// ProgressReporter
// =============================================================================

/// Reports per-file processing progress from a node to the caller.
///
/// The callback receives (percent: u32, message: &str). `None` callback
/// means no-op mode (for tests).
pub struct ProgressReporter {
    #[allow(clippy::type_complexity)]
    callback: Option<Box<dyn Fn(u32, &str)>>,
}

impl ProgressReporter {
    /// Create a reporter with a callback that receives progress updates.
    pub fn new(callback: impl Fn(u32, &str) + 'static) -> Self {
        Self {
            callback: Some(Box::new(callback)),
        }
    }

    /// No-op reporter that discards all updates. Used in tests.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    /// Report progress. Silently discarded in no-op mode.
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
        let reporter = ProgressReporter::new_noop();
        reporter.report(0, "Starting...");
        reporter.report(50, "Halfway there...");
        reporter.report(100, "Done!");
    }

    #[test]
    fn test_noop_reporter_callback_is_none() {
        let reporter = ProgressReporter::new_noop();
        assert!(reporter.callback.is_none());
    }

    #[test]
    fn test_reporter_calls_callback() {
        let calls: Arc<Mutex<Vec<(u32, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let calls_clone = Arc::clone(&calls);

        let reporter = ProgressReporter::new(move |percent, message| {
            calls_clone
                .lock()
                .unwrap()
                .push((percent, message.to_string()));
        });

        reporter.report(0, "Starting...");
        reporter.report(50, "Halfway there...");
        reporter.report(100, "Done!");

        let recorded = calls.lock().unwrap();
        assert_eq!(recorded.len(), 3, "Should have recorded 3 calls");
        assert_eq!(recorded[0], (0, "Starting...".to_string()));
        assert_eq!(recorded[1], (50, "Halfway there...".to_string()));
        assert_eq!(recorded[2], (100, "Done!".to_string()));
    }

    #[test]
    fn test_reporter_with_callback_has_some() {
        let reporter = ProgressReporter::new(|_percent, _message| {});
        assert!(reporter.callback.is_some());
    }
}
