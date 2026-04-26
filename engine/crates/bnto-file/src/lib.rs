// bnto-file — File operation nodes for browser execution.
//
// In the browser there's no filesystem — these are FILENAME TRANSFORMERS.
// File data passes through unchanged; only the name changes.

/// Rename-files node — transforms filenames using patterns, prefixes,
/// suffixes, find/replace, case transformations, and counter sequencing.
pub mod rename;

/// Sanitize-files node — cleans filenames for web-safe or cross-platform use.
pub mod sanitize;

/// WASM bridge — JS-callable functions for file operations.
pub mod wasm_bridge;

pub use rename::RenameFiles;
pub use sanitize::SanitizeFiles;
