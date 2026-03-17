// bnto-core — Shared foundation for all Bnto WASM node crates.
// Provides error types, NodeProcessor trait, progress reporting,
// pipeline executor, metadata, and registry.

// --- Public Modules ---

/// Error types for the WASM engine.
pub mod errors;

/// Structured pipeline events for multi-node execution progress.
pub mod events;

/// Definition JSON Schema — validates `.bnto.json` files (Draft 2020-12).
pub mod definition_schema;

/// Node metadata types — self-describing processor definitions.
/// Powers the `node_catalog()` WASM export.
pub mod metadata;

/// Pipeline executor — walks nodes, iterates files, chains outputs.
pub mod executor;

/// Pipeline definition types — what the engine receives to execute.
/// Mirrors the TypeScript `PipelineDefinition` / `PipelineNode` types.
pub mod pipeline;

/// The NodeProcessor trait — the contract every node type must implement.
pub mod processor;

/// Per-file progress reporting — target-agnostic closures (no WASM dependency).
pub mod progress;

/// Node registry — maps compound keys (e.g., "image:compress") to processors.
pub mod registry;

// --- Re-exports ---
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

/// The current `.bnto.json` format version.
///
/// Must stay in sync with `CURRENT_FORMAT_VERSION` in `@bnto/nodes`.
/// Semver rules: definitions with the same major version are compatible.
pub const FORMAT_VERSION: &str = "1.0.0";

/// Default JPEG quality (0-100). 80 is the industry sweet spot for
/// file size vs. perceptual quality. Used by resize and convert operations.
pub const DEFAULT_JPEG_QUALITY: u8 = 80;

/// Default compression level for compress-images (1-100).
/// 20 = "Light" compression. User-facing semantics are inverted from JPEG
/// quality: internally `jpeg_quality = 101 - compression`.
pub const DEFAULT_COMPRESSION: u8 = 20;

// =============================================================================
// Utility Functions
// =============================================================================

/// Returns the crate version from Cargo.toml (baked in at compile time).
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
