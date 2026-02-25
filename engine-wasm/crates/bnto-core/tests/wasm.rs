// =============================================================================
// WASM Integration Tests — Tests That Run in a Browser/Node.js
// =============================================================================
//
// WHAT IS THIS FILE?
// These tests run inside a real JavaScript environment (browser or Node.js),
// not just native Rust. This is important because:
//   1. It proves our WASM binary loads correctly
//   2. It proves wasm-bindgen's type conversions work (Rust ↔ JS)
//   3. It catches WASM-specific bugs that native tests can't find
//
// RUST CONCEPT: Integration Tests
// In Rust, files in the `tests/` directory are "integration tests".
// They test the crate from the outside (like a consumer would use it),
// not from inside (like unit tests in `src/` do). They can only access
// the crate's public API.
//
// HOW TO RUN THESE TESTS:
//   wasm-pack test --node crates/bnto-core
//
// This compiles the crate to WASM, starts a Node.js process, loads
// the WASM binary, and runs these test functions inside Node.js.
// You can also use `--chrome` or `--firefox` to test in a real browser.

// This `use` brings in the wasm-bindgen-test macros.
// `wasm_bindgen_test` is the attribute we put on test functions.
// `wasm_bindgen_test_configure!` lets us configure the test runner.
use wasm_bindgen_test::*;

// Import our crate's public API. In integration tests, we use the
// crate name (with underscores — Rust replaces hyphens with underscores
// in crate names).
use bnto_core::*;

// Configure wasm-bindgen-test to run in Node.js by default.
// If we wanted browser tests, we'd remove this line.
wasm_bindgen_test_configure!(run_in_node_experimental);

// =============================================================================
// Setup Tests — Verify the WASM module loads and initializes
// =============================================================================

/// Test that the setup function runs without panicking.
/// This proves:
///   1. The WASM binary loaded successfully in Node.js
///   2. console_error_panic_hook installed correctly
///   3. Basic WASM initialization works
#[wasm_bindgen_test]
fn test_setup_does_not_panic() {
    // If this panics, the test fails. The fact that it runs at all
    // proves the WASM module loaded and initialized correctly.
    setup();
}

/// Test that setup can be called multiple times safely.
/// The panic hook uses `set_once()` internally, so this should
/// be idempotent (safe to call repeatedly).
#[wasm_bindgen_test]
fn test_setup_is_idempotent() {
    setup();
    setup();
    setup(); // No panic = success
}

// =============================================================================
// Version Tests — Verify version reporting works across the WASM boundary
// =============================================================================

/// Test that version() returns a non-empty semver string.
/// This proves string data crosses the Rust → JS boundary correctly.
#[wasm_bindgen_test]
fn test_version_across_wasm_boundary() {
    let v = version();
    assert!(!v.is_empty(), "Version should not be empty");
    assert!(
        v.contains('.'),
        "Version should be semver format: got '{}'",
        v
    );
}

// =============================================================================
// Greet Tests — Verify string round-trip across the WASM boundary
// =============================================================================

/// Test that greet() works with a simple ASCII name.
/// This proves:
///   1. Strings pass correctly from Rust to JS (return value)
///   2. The function executes in the WASM environment
#[wasm_bindgen_test]
fn test_greet_with_ascii_name() {
    let result = greet("Ryan");
    assert!(
        result.contains("Ryan"),
        "Greeting should contain the name: got '{}'",
        result
    );
}

/// Test that greet() works with Unicode characters.
/// WASM string handling can be tricky with non-ASCII characters.
/// This ensures we handle multi-byte UTF-8 correctly.
#[wasm_bindgen_test]
fn test_greet_with_unicode_name() {
    let result = greet("Bnto \u{1F371}"); // Bento box emoji!
    assert!(
        result.contains("Bnto \u{1F371}"),
        "Greeting should handle Unicode: got '{}'",
        result
    );
}

/// Test that greet() works with an empty string.
/// Edge case: what happens when the name is empty?
#[wasm_bindgen_test]
fn test_greet_with_empty_name() {
    let result = greet("");
    // Should still return a valid greeting, just without a name.
    assert!(
        result.contains("Bnto WASM engine"),
        "Greeting should still mention the engine: got '{}'",
        result
    );
}
