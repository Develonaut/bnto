// bnto-file — File operation nodes for browser execution.
//
// In the browser there's no filesystem — these are FILENAME TRANSFORMERS.
// File data passes through unchanged; only the name changes.

/// Rename-files node — transforms filenames using patterns, prefixes,
/// suffixes, find/replace, case transformations, counter sequencing,
/// and sanitization (slugify, strip, normalize).
pub mod rename;

/// WASM bridge — JS-callable functions for file operations.
pub mod wasm_bridge;

pub use rename::RenameFiles;
