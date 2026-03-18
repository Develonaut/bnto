// bnto-wasm — the single cdylib entry point for all browser-side WASM.
// One .wasm binary, one JS heap. Re-exports all node crate wasm_bridge
// functions and provides the pipeline executor + catalog for the Web Worker.

use wasm_bindgen::prelude::*;

// =============================================================================
// Pipeline Executor — WASM bridge for the core executor
// =============================================================================
//
// The execute module provides `execute_pipeline()` — a single WASM function
// that takes a pipeline definition (JSON), input files, and a progress callback,
// then runs the entire pipeline in Rust and returns all results. This replaces
// the JS-side `executePipeline.ts` orchestration.
mod execute;

// Re-export the execute_pipeline function so it's available as a WASM export.
// The #[wasm_bindgen] attribute on the function in execute.rs makes it callable
// from JavaScript automatically.
pub use execute::execute_pipeline;

// =============================================================================
// Node Catalog — Self-describing processor metadata
// =============================================================================
//
// The catalog module provides `node_catalog()` — a WASM function that returns
// a JSON string describing every registered processor (name, params, MIME types,
// platforms). Used to validate TS `@bnto/nodes` definitions against the engine.
mod catalog;

// Re-export so it's available as a WASM export.
pub use catalog::node_catalog;

// =============================================================================
// BntoError → JsValue Conversion
// =============================================================================
//
// NOTE: The `impl From<BntoError> for JsValue` was removed from bnto-core to
// keep it target-agnostic. However, it CANNOT live here either — Rust's orphan
// rule prevents implementing foreign traits for foreign types even when one is
// from a workspace crate. Instead, each node crate's wasm_bridge.rs has a
// `bnto_err_to_js()` helper function that does the conversion locally.

// =============================================================================
// Re-export Node Crates
// =============================================================================
//
// IMPORTANT: By re-exporting bnto-image, we ensure the linker includes its
// code in our .wasm binary. Without this, bnto-image's #[wasm_bindgen]
// functions would be dead-stripped and the Web Worker couldn't call them.

// Re-export bnto-core's public types (BntoError, NodeProcessor, ProgressReporter)
// so consumers can access everything through one crate.
pub use bnto_core;

// Re-export all node crates to ensure their #[wasm_bindgen] functions are
// linked into our .wasm binary. Without these lines, the bridge functions
// would be stripped by the linker since nothing in THIS crate calls them
// directly — only the Web Worker does (from JS).
pub use bnto_csv;
pub use bnto_file;
pub use bnto_image;

// =============================================================================
// Setup — One-time initialization
// =============================================================================

/// Initialize the WASM module. Call this ONCE when the Web Worker starts,
/// before calling any processing functions.
///
/// WHAT IT DOES:
/// Installs a "panic hook" so when Rust code crashes, the browser console
/// shows the real error message instead of the useless "unreachable" error.
///
/// SAFE TO CALL MULTIPLE TIMES — set_once() is idempotent.
///
/// USAGE FROM WEB WORKER:
/// ```js
/// import init, { setup } from './bnto_wasm.js';
/// await init('/wasm/bnto_wasm_bg.wasm');
/// setup();  // Call once, then process files
/// ```
#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();
}

/// Returns the version of the Bnto WASM engine.
///
/// Useful for the web app to verify the correct WASM version is loaded
/// and for debugging ("which engine version is this?").
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Health check — proves the WASM module is loaded and working.
///
/// Takes a name and returns a greeting. The Web Worker can call this
/// after init() to verify:
///   1. WASM binary loaded correctly
///   2. String data crosses the Rust ↔ JS boundary properly
///   3. wasm-bindgen's type conversion works
///
/// EXAMPLE:
/// ```js
/// const msg = greet("Ryan");  // "Hello from Bnto WASM engine, Ryan! v0.1.0"
/// ```
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Bnto WASM engine, {}! v{}", name, version())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_not_empty() {
        let v = version();
        assert!(!v.is_empty(), "Version should not be empty");
        assert!(v.contains('.'), "Version should be semver format");
    }

    #[test]
    fn test_greet_includes_name_and_version() {
        let greeting = greet("Ryan");
        assert!(greeting.contains("Ryan"), "Should include the name");
        assert!(greeting.contains(&version()), "Should include version");
    }
}
