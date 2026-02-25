// =============================================================================
// bnto-core — The Foundation WASM Library
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the "entry point" for the bnto-core crate. In Rust, `lib.rs` is
// like `index.ts` in TypeScript — it's the main file that defines what the
// crate exports (makes available to other code).
//
// WHAT DOES THIS CRATE DO?
// It provides the shared foundation that all Bnto WASM node crates use:
//   1. Error types — a standard way to report problems
//   2. The NodeProcessor trait — the interface every node type must implement
//   3. Progress reporting — so the UI can show "50% done..."
//   4. Helper utilities — logging, file conversion, etc.
//
// HOW IS THIS USED?
// The web app loads this WASM module in a Web Worker (a background thread
// in the browser). The worker calls our exported functions, and we call
// back to JavaScript to report progress. Files never leave the browser.
//
// RUST CONCEPT: `mod` AND `pub mod`
// In Rust, code is organized into "modules" (like folders/files in a project).
// - `mod errors;` means "there's a file called errors.rs, include it here"
// - `pub mod errors;` means "include it AND make it visible to outside code"
// - `pub use errors::*;` means "re-export everything from errors so users
//   can write `use bnto_core::BntoError` instead of `use bnto_core::errors::BntoError`"

// --- Public Modules ---
// These are the building blocks that node crates and the web app will use.

/// Error types for the WASM engine.
/// Every error that can happen during node execution is defined here.
pub mod errors;

/// The NodeProcessor trait — the contract every node type must implement.
/// If you're building a new node (like image compression), you implement this.
pub mod processor;

/// Progress reporting — how nodes tell the UI "I'm 50% done".
/// Uses JavaScript callbacks via wasm-bindgen.
pub mod progress;

// --- Re-exports ---
// These `pub use` statements let users import directly from the crate root.
// Instead of writing `use bnto_core::errors::BntoError`, they can write
// `use bnto_core::BntoError`. Convenience!
pub use errors::BntoError;
pub use processor::NodeProcessor;
pub use progress::ProgressReporter;

// =============================================================================
// WASM Setup — One-time initialization when the module loads
// =============================================================================

// `use` statements are like `import` in JavaScript. They bring names from
// other crates (libraries) into scope so we can use them without the full path.
use wasm_bindgen::prelude::*;

// RUST CONCEPT: `#[wasm_bindgen]`
// This is an "attribute macro" — a special annotation that tells wasm-bindgen
// to generate JavaScript wrapper code for the function below it. Without this,
// the function would only be callable from other Rust code, not from JS.

/// Initialize the WASM module. Call this once when the Web Worker starts.
///
/// WHAT IT DOES:
/// Sets up a "panic hook" — when Rust code crashes (panics), instead of
/// showing a cryptic "unreachable" error in the browser console, it shows
/// the actual Rust error message with file and line number. Invaluable
/// for debugging.
///
/// WHY `console_error_panic_hook`?
/// By default, when Rust code panics in WASM, the browser just says
/// "RuntimeError: unreachable" — completely useless for debugging.
/// The panic hook intercepts the crash and prints the real error
/// message to `console.error()`.
///
/// USAGE FROM JAVASCRIPT:
/// ```js
/// import init, { setup } from './pkg/bnto_core.js';
/// await init();  // Load the WASM binary
/// setup();       // Set up the panic hook
/// ```
#[wasm_bindgen]
pub fn setup() {
    // `console_error_panic_hook` redirects Rust panics to console.error().
    // The `set_once()` method ensures it's only installed once, even if
    // `setup()` is called multiple times. Safe to call repeatedly.
    console_error_panic_hook::set_once();
}

/// Returns the version of the WASM engine.
///
/// This is useful for the web app to verify it loaded the right version
/// and for debugging ("which version of the WASM engine is running?").
///
/// RUST CONCEPT: `-> String`
/// The `-> String` part is the return type. In Rust, you must declare
/// what type a function returns. `String` is a heap-allocated text value
/// (like a JS string). The last expression in a function (without a
/// semicolon) is automatically returned — no `return` keyword needed.
///
/// RUST CONCEPT: `.to_string()`
/// `env!("CARGO_PKG_VERSION")` gives us a `&str` (a "string slice" —
/// a reference to text that lives somewhere else, like a pointer).
/// `.to_string()` converts it to an owned `String` that wasm-bindgen
/// can pass back to JavaScript.
#[wasm_bindgen]
pub fn version() -> String {
    // `env!()` is a compile-time macro that reads values from the build
    // environment. `CARGO_PKG_VERSION` is automatically set by Cargo
    // from the `version` field in Cargo.toml. So if Cargo.toml says
    // version = "0.1.0", this returns "0.1.0".
    env!("CARGO_PKG_VERSION").to_string()
}

/// A simple health check to verify the WASM module is working.
///
/// Takes a name and returns a greeting. This proves:
/// 1. String data can cross the Rust ↔ JS boundary
/// 2. The WASM module is loaded and executing correctly
/// 3. wasm-bindgen's string conversion works
///
/// RUST CONCEPT: `&str` vs `String`
/// - `&str` ("string slice") is a REFERENCE to text. It's borrowed —
///   we can read it but we don't own it. Think of it like a read-only
///   view into someone else's string. Cheap to pass around.
/// - `String` is an OWNED piece of text. We allocated it, we're
///   responsible for it. When it goes out of scope, Rust frees the memory.
///
/// RUST CONCEPT: `format!()`
/// `format!()` is like JavaScript's template literals (`Hello, ${name}!`).
/// It creates a new `String` by interpolating values into a template.
/// The `{}` is a placeholder that gets replaced by the argument.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!(
        "Hello from Bnto WASM engine, {}! Version {}",
        name,
        version()
    )
}

// =============================================================================
// Tests
// =============================================================================
//
// RUST CONCEPT: `#[cfg(test)]`
// This attribute says "only compile this code when running tests".
// It's completely removed from the production WASM binary — zero overhead.
// In Rust, tests live right next to the code they test (co-location!).
//
// These are "unit tests" — they test pure Rust logic without needing
// a browser. For WASM-specific tests (that need JS interop), see the
// `tests/` directory which uses wasm-bindgen-test.

#[cfg(test)]
mod tests {
    // `use super::*` imports everything from the parent module (lib.rs).
    // This lets our tests call `version()`, `greet()`, etc. directly.
    use super::*;

    // `#[test]` marks a function as a test case. `cargo test` finds and
    // runs all functions with this attribute.
    #[test]
    fn test_version_returns_cargo_version() {
        // The version should match what's in Cargo.toml.
        // We don't hardcode "0.1.0" here — instead we check it's not empty.
        // This way the test doesn't break when we bump the version.
        let v = version();

        // `assert!()` is like JavaScript's `assert()` — if the condition
        // is false, the test fails. `!v.is_empty()` checks the string
        // isn't empty.
        assert!(!v.is_empty(), "Version string should not be empty");

        // Check it looks like a semver version (has dots).
        // `.contains()` works like JavaScript's `.includes()`.
        assert!(
            v.contains('.'),
            "Version should contain dots (semver format)"
        );
    }

    #[test]
    fn test_greet_includes_name() {
        let greeting = greet("Ryan");

        // `assert!()` with `.contains()` — verify the name appears in output.
        assert!(
            greeting.contains("Ryan"),
            "Greeting should include the name: got '{}'",
            greeting
        );
    }

    #[test]
    fn test_greet_includes_version() {
        let greeting = greet("test");

        // The greeting should include the engine version.
        assert!(
            greeting.contains(&version()),
            "Greeting should include version: got '{}'",
            greeting
        );
    }
}
