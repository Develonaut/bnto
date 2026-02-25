// =============================================================================
// bnto-csv — CSV Processing for the Browser
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the entry point for the bnto-csv crate. It's like `index.ts` —
// it defines what this crate makes available to the outside world.
//
// WHAT DOES THIS CRATE DO?
// It provides CSV processing "nodes" that run entirely in the browser:
//   1. clean-csv — remove empty rows, trim whitespace, deduplicate
//   2. rename-columns — rename CSV column headers via a mapping
//
// Each node implements the `NodeProcessor` trait from bnto-core, so the
// Web Worker orchestrator can run them all the same way.
//
// HOW FILES FLOW:
//   1. User drops a CSV file in the browser
//   2. Web Worker reads the File object as raw bytes (Vec<u8>)
//   3. We parse the bytes as UTF-8 text, then as CSV records
//   4. We process the records (clean, rename columns, etc.)
//   5. We write the processed records back to CSV bytes
//   6. Web Worker creates a download Blob for the user
//
// Files never leave the user's machine. Zero network. Zero cost.

// --- Modules ---

/// The clean-csv node — removes empty rows, trims whitespace from cells,
/// and removes duplicate rows. The most basic CSV cleanup operation.
pub mod clean;

/// The rename-columns node — renames CSV column headers using a
/// user-provided mapping (e.g., {"First Name": "first_name"}).
pub mod rename_columns;

/// WASM bridge — the JavaScript-callable functions that the Web Worker
/// uses to invoke CSV processing. This is the "door" between JS and Rust.
pub mod wasm_bridge;

// --- Re-exports ---
// So consumers can write `use bnto_csv::CleanCsv` directly.

pub use clean::CleanCsv;
pub use rename_columns::RenameCsvColumns;
