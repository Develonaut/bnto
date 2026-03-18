// =============================================================================
// bnto-file — File operation nodes for browser execution
// =============================================================================
//
// In the browser there's no filesystem — this is a FILENAME TRANSFORMER.
// File data passes through unchanged; only the name changes.

/// Rename-files node — transforms filenames using patterns, prefixes,
/// suffixes, find/replace, and case transformations.
pub mod rename;

/// WASM bridge — JS-callable functions for file operations.
pub mod wasm_bridge;

pub use rename::RenameFiles;
