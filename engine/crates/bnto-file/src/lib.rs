// bnto-file — File operation nodes.
//
// Browser-capable: filter, rename, metadata (no filesystem access).
// CLI-only: collect, copy, move (require filesystem access).

/// File-filter node — drop files that don't match extension, pattern,
/// or size criteria. Non-matching files produce empty output.
pub mod filter;

/// Rename-files node — transforms filenames using patterns, prefixes,
/// suffixes, find/replace, case transformations, counter sequencing,
/// and sanitization (slugify, strip, normalize).
pub mod rename;

/// File-collect node — traverse directories and glob-match files into
/// the pipeline. CLI-only (requires filesystem access).
pub mod collect;

/// File-copy node — write output files to a destination directory with
/// conflict resolution. CLI-only (requires filesystem access).
pub mod copy;

/// The `newer` conflict mode of file-copy (time-compared, in-place copy).
mod copy_newer;

/// File-move node — move files into folders under a destination by the path
/// in each name, refusing the whole run if a move would replace a file.
/// CLI-only (requires filesystem access).
pub mod r#move;

/// Checks and runs a batch of file moves so none replaces a file. Shared by
/// file-move and the CLI's `output` overwrite mode.
pub mod relocate;

/// File-metadata node — extract file metadata (size, extension, MIME
/// type, hash) and attach to pipeline output. Browser-compatible.
pub mod metadata;

/// WASM bridge — JS-callable functions for file operations.
pub mod wasm_bridge;

pub use collect::FileCollect;
pub use copy::FileCopy;
pub use filter::FileFilter;
pub use metadata::FileMetadata;
pub use r#move::FileMove;
pub use rename::RenameFiles;
