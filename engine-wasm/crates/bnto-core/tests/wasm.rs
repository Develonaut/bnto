// =============================================================================
// WASM Integration Tests — bnto-core Types Across the WASM Boundary
// =============================================================================
//
// NOTE: The setup(), version(), and greet() tests moved to the bnto-wasm
// entry point crate, which is now the single cdylib that exports those
// functions to JavaScript.
//
// bnto-core is now an rlib (Rust library) that provides foundation types:
//   - BntoError (error types)
//   - NodeProcessor (trait for node implementations)
//   - ProgressReporter (progress callback wrapper)
//
// These types are tested natively via unit tests in src/*.rs. WASM integration
// tests for the full pipeline (JS → WASM → process → JS) live in bnto-wasm.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

/// Verify that bnto-core's version() function works when called from
/// WASM context (even though it's not a #[wasm_bindgen] export anymore,
/// it's still callable from Rust code running inside WASM).
#[wasm_bindgen_test]
fn test_core_version_in_wasm_context() {
    let v = bnto_core::version();
    assert!(!v.is_empty(), "Core version should not be empty");
}

/// Verify that BntoError display formatting works in WASM context.
/// Error messages cross the boundary as strings — this ensures they
/// format correctly even when running inside the WASM runtime.
#[wasm_bindgen_test]
fn test_error_formatting_in_wasm_context() {
    let error = bnto_core::BntoError::InvalidInput("test error".to_string());
    let msg = error.to_string();
    assert!(msg.contains("test error"), "Error message should contain the detail");
}

/// Verify that the noop ProgressReporter works in WASM context.
/// In tests, we use new_noop() instead of a real JS callback.
#[wasm_bindgen_test]
fn test_noop_progress_in_wasm_context() {
    let reporter = bnto_core::ProgressReporter::new_noop();
    reporter.report(50, "Half done");
    // No panic = success (noop silently discards progress)
}
