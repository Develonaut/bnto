// =============================================================================
// bnto-wasm — The Unified WASM Entry Point
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the ONE file that the browser's Web Worker loads. It's the front door
// to ALL Bnto's browser-side processing. Every function the Web Worker can call
// is either defined here or re-exported from a node crate.
//
// WHY DOES THIS EXIST?
// In WASM, each "cdylib" crate produces its own .wasm file with a separate
// JavaScript heap. If bnto-core and bnto-image were both cdylib, they'd be
// two separate WASM modules that can't share data. That's a problem!
//
// By making this the ONLY cdylib crate and having all others as rlib (Rust
// libraries), we get:
//   1. ONE .wasm file — one download, one init() call
//   2. Shared heap — objects can be passed between node crates freely
//   3. Smaller total size — shared dependencies are included once
//
// HOW THE WEB WORKER USES THIS:
//
//   JavaScript (Web Worker):
//   ```js
//   import init, { setup, version, compress_image_bytes } from './bnto_wasm.js';
//
//   // Step 1: Load the WASM binary (one-time)
//   await init('/wasm/bnto_wasm_bg.wasm');
//
//   // Step 2: Initialize panic hooks (one-time)
//   setup();
//
//   // Step 3: Process files!
//   const result = compress_image_bytes(bytes, 'photo.jpg', '{"quality": 80}', progressCb);
//   ```

use wasm_bindgen::prelude::*;

// =============================================================================
// Re-export Node Crates
// =============================================================================
//
// IMPORTANT: By re-exporting bnto-image, we ensure the linker includes its
// code in our .wasm binary. Without this, bnto-image's #[wasm_bindgen]
// functions would be dead-stripped and the Web Worker couldn't call them.
//
// RUST CONCEPT: `pub use`
// `pub use some_crate;` re-exports the entire crate under our namespace.
// This is like `export * from './some-module'` in JavaScript. It makes all
// of bnto-image's public items available to anyone using bnto-wasm.

// Re-export bnto-core's public types (BntoError, NodeProcessor, ProgressReporter)
// so consumers can access everything through one crate.
pub use bnto_core;

// Re-export bnto-image to ensure its #[wasm_bindgen] functions are linked
// into our .wasm binary. Without this line, compress_image and
// compress_image_bytes would be stripped by the linker since nothing in
// THIS crate calls them directly — only the Web Worker does (from JS).
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
///
/// RUST CONCEPT: `env!("CARGO_PKG_VERSION")`
/// This macro reads the `version` field from THIS crate's Cargo.toml
/// at compile time. So if Cargo.toml says version = "0.1.0", this
/// function returns "0.1.0". It's baked into the binary — no runtime
/// config file needed.
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
