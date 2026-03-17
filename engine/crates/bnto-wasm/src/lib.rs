// =============================================================================
// bnto-wasm — Unified WASM entry point (single cdylib for all node crates)
// =============================================================================
//
// The ONLY cdylib crate — all node crates are rlib. This gives us one .wasm
// file, one init() call, a shared heap, and smaller total size.

use wasm_bindgen::prelude::*;

// =============================================================================
// Pipeline Executor
// =============================================================================

mod execute;
pub use execute::execute_pipeline;

// =============================================================================
// Node Catalog
// =============================================================================

mod catalog;
pub use catalog::node_catalog;

// =============================================================================
// Re-exports
// =============================================================================
//
// Re-exporting node crates ensures the linker includes their #[wasm_bindgen]
// functions in the .wasm binary (otherwise they'd be dead-stripped since
// nothing in this crate calls them directly — only the Web Worker does from JS).

pub use bnto_core;
pub use bnto_csv;
pub use bnto_file;
pub use bnto_image;

// =============================================================================
// Setup
// =============================================================================

/// Initialize the WASM module. Call once when the Web Worker starts.
/// Installs the panic hook for readable error messages in the browser console.
/// Safe to call multiple times (idempotent).
#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();
}

/// Returns the WASM engine version (from Cargo.toml at compile time).
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Health check — proves the WASM module loaded and string data crosses
/// the Rust/JS boundary correctly.
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
