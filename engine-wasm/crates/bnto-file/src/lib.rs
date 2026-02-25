// =============================================================================
// bnto-file — File Operations for the Browser
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the entry point for the bnto-file crate. It's like `index.ts` —
// it defines what this crate makes available to the outside world.
//
// WHAT DOES THIS CRATE DO?
// It provides file operation "nodes" that run entirely in the browser:
//   1. rename-files — transform filenames using patterns, prefixes,
//      suffixes, find/replace, and case transformations
//   2. (future) organize-files — sort files into folders by type/date
//   3. (future) filter-files — include/exclude files by pattern
//
// Each node implements the `NodeProcessor` trait from bnto-core, so the
// Web Worker orchestrator can run them all the same way.
//
// KEY INSIGHT: In the browser, there's no filesystem. This node is a
// FILENAME TRANSFORMER, not a filesystem operation. The user drops files,
// this node renames them according to rules, and the user downloads them
// with new names. The actual file DATA passes through unchanged — only
// the name changes.
//
// HOW FILES FLOW:
//   1. User drops files in the browser
//   2. Web Worker reads File objects as raw bytes (Vec<u8>) with a filename
//   3. We apply naming rules to transform the filename
//   4. We return the SAME bytes with the NEW filename
//   5. Web Worker creates a download Blob with the new name
//
// Files never leave the user's machine. Zero network. Zero cost.

// --- Modules ---

/// The rename-files node — transforms filenames using patterns, prefixes,
/// suffixes, find/replace, and case transformations. The file data passes
/// through unchanged; only the filename is modified.
pub mod rename;

/// WASM bridge — the JavaScript-callable functions that the Web Worker
/// uses to invoke file operations. This is the "door" between JS and Rust.
pub mod wasm_bridge;

// --- Re-exports ---
// So consumers can write `use bnto_file::RenameFiles` directly.

pub use rename::RenameFiles;
