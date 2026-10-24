// bnto-file — File operation nodes.
//
// Browser-capable: decode, filter, inspect, join, rename, render, sort (no filesystem access).
// CLI-only: collect, copy, delete, list, move (require filesystem access).

/// A compiled file-name glob, shared by file-collect and file-filter.
mod glob;

/// Stem/extension split of an item name, within its last path segment.
/// Shared by file-decode and file-rename.
mod split_extension;

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

/// File-delete node — delete the files flowing into it, each under a base
/// folder; missing files are skipped and counted, folders refused.
/// CLI-only (requires filesystem access).
pub mod delete;

/// File-list node — turn lists of paths (from input files or a param) into
/// path references under a base folder; missing paths are skipped and counted.
/// CLI-only (requires filesystem access).
pub mod list;

/// The `newer` conflict mode of file-copy (time-compared, in-place copy).
mod copy_newer;

/// File-move node — move files into folders under a destination by the path
/// in each name, refusing the whole run if a move would replace a file.
/// CLI-only (requires filesystem access).
pub mod r#move;

/// Checks and runs a batch of file moves so none replaces a file. Shared by
/// file-move and the CLI's `output` overwrite mode.
pub mod relocate;

/// File-decode node — decode each file's content from a text encoding
/// (base64) into its bytes. Browser-compatible.
pub mod decode;

/// File-sort node — reorder items by fields of their metadata, each key
/// ascending or descending; stable, data and metadata untouched.
/// Browser-compatible.
pub mod sort;

/// File-inspect node — extract file metadata (size, extension, MIME
/// type, hash) and attach to pipeline output. Browser-compatible.
pub mod inspect;

/// File-render node — write one text file from a template over the items
/// (a header, one block per item, a footer). Browser-compatible.
pub mod render;

/// File-join node — join the files flowing in into one file, in arrival
/// order, a separator between them; disk files stream, never loaded whole.
/// Browser-compatible.
pub mod join;

/// WASM bridge — JS-callable functions for file operations.
pub mod wasm_bridge;

pub use collect::FileCollect;
pub use copy::FileCopy;
pub use decode::FileDecode;
pub use delete::FileDelete;
pub use filter::FileFilter;
pub use inspect::FileInspect;
pub use join::FileJoin;
pub use list::FileList;
pub use r#move::FileMove;
pub use rename::RenameFiles;
pub use render::FileRender;
pub use sort::FileSort;
