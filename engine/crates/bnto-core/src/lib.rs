// =============================================================================
// bnto-core — The Foundation WASM Library
// =============================================================================
//
// Shared foundation for all Bnto WASM node crates: error types,
// the NodeProcessor trait, progress reporting, pipeline execution,
// and the node registry. This is an rlib -- it doesn't produce a
// .wasm file itself. That's the job of the bnto-wasm entry point.

// --- Public Modules ---
// These are the building blocks that node crates and the web app will use.

/// Error types for the WASM engine.
/// Every error that can happen during node execution is defined here.
pub mod errors;

/// Structured pipeline events — rich progress reporting for multi-node execution.
/// Powers per-node status highlighting in the editor, progress bars, and error display.
pub mod events;

/// Definition JSON Schema — validates `.bnto.json` files.
/// Generates a JSON Schema (Draft 2020-12) describing the Definition structure
/// so any consumer can validate recipe files without reimplementing TS types.
pub mod definition_schema;

/// Node metadata types — self-describing processor definitions.
/// Each processor declares its name, category, parameters, accepted MIME types,
/// and whether it runs in the browser. Powers the `node_catalog()` WASM export.
pub mod metadata;

/// The pipeline executor — walks nodes, iterates files, chains outputs.
/// This is the engine's brain. See `.claude/strategy/engine-execution.md`.
pub mod executor;

/// Pipeline definition types — what the engine receives to execute.
/// Mirrors the TypeScript `PipelineDefinition` / `PipelineNode` types.
pub mod pipeline;

/// The NodeProcessor trait — the contract every node type must implement.
/// If you're building a new node (like image compression), you implement this.
pub mod processor;

/// Progress reporting — how nodes tell the UI "I'm 50% done".
/// Uses target-agnostic closures (no WASM dependency).
pub mod progress;

/// Node registry — maps node type keys (e.g., "image-compress") to processors.
/// Replaces the JS-side `wasmLoader.ts` registry.
pub mod registry;

// --- Re-exports ---
// These `pub use` statements let users import directly from the crate root.
// Instead of writing `use bnto_core::errors::BntoError`, they can write
// `use bnto_core::BntoError`. Convenience!
pub use definition_schema::definition_json_schema;
pub use errors::BntoError;
pub use events::{PipelineEvent, PipelineReporter};
pub use executor::execute_pipeline;
pub use metadata::{
    Constraints, NodeCategory, NodeMetadata, NodeTypeInfo, ParamCondition, ParamConditionEntry,
    ParameterDef, ParameterType, all_node_types,
};
pub use pipeline::{
    PipelineDefinition, PipelineFile, PipelineFileResult, PipelineNode, PipelineResult,
};
pub use processor::NodeProcessor;
pub use progress::ProgressReporter;
pub use registry::NodeRegistry;

// =============================================================================
// Shared Constants
// =============================================================================
//
// Constants used by multiple node crates live here so there's a single source
// of truth. When compress, resize, and convert all need the same default JPEG
// quality, defining it once in bnto-core prevents the values from drifting
// apart over time.

/// The current `.bnto.json` format version.
///
/// This must stay in sync with `CURRENT_FORMAT_VERSION` in `@bnto/nodes`.
/// The WASM engine uses this to verify that a definition it receives is
/// compatible with the node processors it has compiled in.
///
/// Semver rules: definitions with the same major version are compatible.
/// A definition at "1.3.0" works fine on an engine that supports "1.0.0".
pub const FORMAT_VERSION: &str = "1.0.0";

/// Default quality when not specified by the user (1-100 scale).
/// 80 is the industry sweet spot: significant file size savings with barely
/// noticeable quality loss for most photos. Used by all image operations
/// (compress, resize, convert).
pub const DEFAULT_QUALITY: u8 = 80;

// =============================================================================
// Utility Functions (Pure Rust — no WASM boundary)
// =============================================================================
//
// NOTE: setup(), version(), and greet() used to live here with #[wasm_bindgen]
// attributes. They've moved to the bnto-wasm entry point crate which is the
// single cdylib that produces the .wasm file for the browser. This crate is
// now purely an rlib (Rust library) — no JS exports.
//
// These utility functions remain available as regular Rust functions for use
// by other crates in the workspace and for testing.

/// Returns the version of the bnto-core crate.
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_version_is_valid_semver() {
        // FORMAT_VERSION should be a valid semver string (major.minor.patch)
        let parts: Vec<&str> = FORMAT_VERSION.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "FORMAT_VERSION must have 3 parts (major.minor.patch)"
        );
        for part in &parts {
            part.parse::<u32>()
                .expect("Each semver part must be a valid number");
        }
    }

    #[test]
    fn test_version_returns_cargo_version() {
        let v = version();
        assert!(!v.is_empty(), "Version string should not be empty");
        assert!(
            v.contains('.'),
            "Version should contain dots (semver format)"
        );
    }
}
