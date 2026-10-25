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
/// conflict resolution, checking every copy before any runs. CLI-only
/// (requires filesystem access).
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

/// file-copy's batch: per-file `{{item.*}}` params, and the check that no
/// copy replaces a file another file of the batch is read from.
mod copy_batch;

/// File-move node — move files into folders under a destination by the path
/// in each name, refusing the whole run if a move would replace a file.
/// CLI-only (requires filesystem access).
pub mod r#move;

/// Checks and runs a batch of file moves so none replaces a file. Shared by
/// file-move and the CLI's `output` overwrite mode.
pub mod relocate;

/// Which files of a batch are read from a file a write would replace. Shared
/// by file-copy and the CLI's output write.
pub mod readers;

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

#[cfg(test)]
mod time_runs_out;

/// Name prefix of the folder the CLI and TUI output step sets files aside in
/// to run a swap of writes (hidden on Unix, not on Windows). A finished write
/// removes it; one a failed or interrupted write left must not become a later
/// run's input, so `file-collect` skips any folder with this prefix.
pub const STAGED_COPIES_PREFIX: &str = ".bnto-staged-";

/// Name prefix of a file (or a folder `file-move` moves) that `relocate`
/// parked to break a swap of renames (hidden on Unix, not on Windows):
/// `.bnto-rename-<n>-<original name>`, in its own folder until its final
/// rename. One a stopped run left must not become a later run's input, so
/// `file-collect` skips any file or folder with it.
pub const PARKED_NAME_PREFIX: &str = ".bnto-rename-";

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
