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
// 1. The caller creates a ProgressReporter with a callback function
// 2. The callback can do ANYTHING — post a message to a Web Worker,
//    print to the console, update a progress bar, etc.
// 3. The node calls `reporter.report(50, "Compressing...")` during processing
// 4. The callback receives the percent and message and does its thing
//
// WHY IS THIS TARGET-AGNOSTIC?
// Previously, this struct wrapped a `js_sys::Function` directly, which
// forced every crate that depends on bnto-core to pull in WASM dependencies
// (wasm-bindgen, js-sys, web-sys). That's bad because:
//   - It couples the core engine to a specific platform (browser/WASM)
//   - A future desktop app (Tauri) or CLI wouldn't need WASM deps at all
//   - It makes unit tests harder (need to mock JS types)
//
// Now we use a plain Rust closure — `Box<dyn Fn(u32, &str)>` — which is
// just "a function that takes a number and a string". The WASM-specific
// wrapping (converting to JS types, calling js_sys::Function) lives in
// the wasm_bridge.rs files in each node crate, where it belongs.
//
// WHY A STRUCT (not just a function)?
// A struct lets us:
//   - Store the callback and reuse it without passing it everywhere
//   - Add a "no-op" mode for testing (no callback needed)
//   - Track state (like preventing backwards progress)
//   - Add rate limiting later (don't flood the UI with updates)

// =============================================================================
// ProgressReporter
// =============================================================================

/// Reports processing progress from a node back to the caller (UI, CLI, etc.).
///
/// RUST CONCEPT: `Box<dyn Fn(u32, &str)>`
/// Let's break this down piece by piece:
///   - `Fn(u32, &str)` — a trait (interface) for any function that takes
///     a u32 (percentage) and a &str (message). In JavaScript terms, it's
///     like `(percent: number, message: string) => void`.
///   - `dyn` — means "dynamic dispatch". We don't know the EXACT type of
///     the function at compile time (it could be a closure, a function
///     pointer, etc.). Rust will use a "vtable" (lookup table) at runtime
///     to figure out which function to call. Small performance cost, but
///     gives us flexibility — any closure works.
///   - `Box<...>` — puts the closure on the heap (dynamically allocated
///     memory). We need this because closures can be different sizes
///     depending on what they capture, and Rust needs to know the size
///     of struct fields at compile time. Boxing gives us a fixed-size
///     pointer regardless of the closure's actual size.
///
/// RUST CONCEPT: `Option<T>`
/// `Option<T>` means "maybe a T, maybe nothing". Here, the callback
/// might be `Some(function)` (real reporting) or `None` (no-op mode,
/// used in tests). This avoids needing a separate "mock" type for testing.
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
    /// RUST CONCEPT: `impl Fn(u32, &str) + 'static`
    /// This is a generic parameter that means "any function or closure that:
    ///   - Takes a u32 and a &str as arguments (`Fn(u32, &str)`)
    ///   - Lives for the entire program duration (`'static` lifetime)"
    ///
    /// The `'static` bound is needed because we're storing the closure in
    /// a Box. Without it, the closure might reference temporary data that
    /// gets freed while we're still holding the closure — a dangling pointer.
    /// `'static` means the closure either owns all its data or only references
    /// things that live forever (like string literals).
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
        // RUST CONCEPT: `Self`
        // `Self` is a shorthand for the type we're implementing (ProgressReporter).
        //
        // `Box::new(callback)` puts the closure on the heap and gives us a
        // fixed-size pointer. `Some(...)` wraps it in an Option to indicate
        // "we have a callback".
        Self {
            callback: Some(Box::new(callback)),
        }
    }

    /// Create a no-op reporter that discards all progress updates.
    /// Used in tests where we don't need progress reporting.
    ///
    /// RUST CONCEPT: `None`
    /// `None` is the "nothing" variant of `Option`. When `report()` is called,
    /// it checks for `Some(callback)` and skips silently if it finds `None`.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    /// Report progress to the caller.
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
            // Call the callback with our two arguments.
            // `cb` is a `Box<dyn Fn(u32, &str)>`, which we can call like
            // a regular function. The `Box` is transparent — Rust auto-
            // dereferences it for us.
            //
            // We intentionally don't handle panics from the callback here.
            // If the callback panics (e.g., the JS side threw), the panic
            // will propagate up naturally. Progress is "best effort", but
            // a panic means something is seriously wrong.
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

    // RUST CONCEPT: `Arc` and `Mutex`
    // These are thread-safe wrappers for shared data:
    //   - `Arc` (Atomic Reference Count) — lets multiple owners share the same data
    //   - `Mutex` (Mutual Exclusion) — ensures only one owner accesses the data at a time
    //
    // We need them here because the closure captures the Vec, but the test
    // also needs to read the Vec after calling report(). `Arc<Mutex<...>>`
    // lets both the closure and the test body access the same Vec safely.
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
