// WASM Integration Tests -- bnto-core types across the WASM boundary.
//
// bnto-core is an rlib providing foundation types (BntoError, NodeProcessor,
// ProgressReporter). Full pipeline WASM tests live in bnto-wasm.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

#[wasm_bindgen_test]
fn test_core_version_in_wasm_context() {
    let v = bnto_core::version();
    assert!(!v.is_empty(), "Core version should not be empty");
}

#[wasm_bindgen_test]
fn test_error_formatting_in_wasm_context() {
    let error = bnto_core::BntoError::InvalidInput("test error".to_string());
    let msg = error.to_string();
    assert!(
        msg.contains("test error"),
        "Error message should contain the detail"
    );
}

#[wasm_bindgen_test]
fn test_noop_progress_in_wasm_context() {
    let reporter = bnto_core::ProgressReporter::new_noop();
    reporter.report(50, "Half done");
    // No panic = success (noop silently discards progress)
}
