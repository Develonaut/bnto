// =============================================================================
// Error Types — Everything That Can Go Wrong
// =============================================================================
//
// WHAT IS THIS FILE?
// This defines all the error types for the Bnto WASM engine. When something
// goes wrong during node execution (bad input, unsupported format, etc.),
// we return one of these errors instead of crashing.
//
// RUST CONCEPT: Error Handling
// Rust doesn't have exceptions (no try/catch like JavaScript). Instead,
// functions that can fail return `Result<T, E>`:
//   - `Ok(value)` = everything worked, here's the result
//   - `Err(error)` = something went wrong, here's what happened
//
// This forces you to handle errors explicitly — you can't accidentally
// ignore them like you can with uncaught exceptions in JS.
//
// WHY A CUSTOM ERROR TYPE?
// We could use `String` for errors, but a custom enum lets us:
//   1. Match on specific error kinds (is it an IO error? A format error?)
//   2. Provide structured information (which file failed? What format?)
//   3. Convert to user-friendly messages for the UI
//
// RUST CONCEPT: `enum`
// An `enum` in Rust is like a TypeScript discriminated union:
//   type BntoError =
//     | { kind: "InvalidInput", message: string }
//     | { kind: "UnsupportedFormat", format: string }
//     | ...
// Each variant can carry different data. The compiler ensures you
// handle every possible variant when matching.

use thiserror::Error;

// RUST CONCEPT: `#[derive(...)]`
// `derive` is an attribute that tells the compiler to automatically
// generate code for standard traits (interfaces). Here:
//   - `Debug` — lets us print the error with `{:?}` format (like console.dir in JS)
//   - `Error` — from thiserror, generates the `std::error::Error` trait implementation
//
// RUST CONCEPT: `#[error("...")]`
// This attribute (from thiserror) defines the human-readable error message
// for each variant. `{0}` is a placeholder for the first field.

/// All possible errors from Bnto WASM node operations.
///
/// Each variant represents a different category of failure.
/// The `#[error("...")]` attribute defines the display message.
#[derive(Debug, Error)]
pub enum BntoError {
    // --- Input Validation Errors ---
    /// The input data is invalid (wrong type, missing required field, etc.).
    /// The string contains a human-readable explanation of what's wrong.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// The file format isn't supported by this node.
    /// For example, trying to compress a .bmp when we only support JPEG/PNG/WebP.
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    // --- Processing Errors ---
    /// Something went wrong while processing the file.
    /// This is a catch-all for errors during the actual work (compression,
    /// CSV parsing, etc.). The string explains what happened.
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    /// The user cancelled the operation (or the Web Worker was terminated).
    /// Not a "real" error — the user chose to stop.
    #[error("Operation cancelled")]
    Cancelled,

    // --- Resource Errors ---
    /// A file is too large for browser processing.
    /// The u64 is the file size in bytes. Browser memory is limited (~2GB),
    /// so we check file sizes before trying to process them.
    ///
    /// RUST CONCEPT: `u64`
    /// `u64` is an unsigned 64-bit integer. "Unsigned" means it can only
    /// be positive (0 or greater). 64 bits means it can hold values up to
    /// about 18 quintillion — plenty for file sizes.
    #[error("File too large: {0} bytes")]
    FileTooLarge(u64),

    /// The browser ran out of memory while processing.
    /// WASM has a limited memory heap. If we try to load a huge image,
    /// we might exhaust it.
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
}

// =============================================================================
// Converting BntoError to JavaScript (MOVED TO bnto-wasm)
// =============================================================================
//
// NOTE: The `impl From<BntoError> for JsValue` conversion used to live here,
// but it coupled bnto-core to WASM dependencies (wasm-bindgen, js-sys).
// That forced every node crate to carry WASM deps even for pure Rust logic.
//
// The conversion now lives in `bnto-wasm/src/lib.rs` — the only crate that
// should know about JavaScript types. This keeps bnto-core target-agnostic:
// it works in WASM, native CLI, desktop (Tauri), or anywhere else.

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_error_message() {
        // Create an error and check its message.
        let error = BntoError::InvalidInput("missing file data".to_string());

        // `.to_string()` triggers the `#[error("...")]` formatting.
        let msg = error.to_string();

        assert_eq!(msg, "Invalid input: missing file data");
    }

    #[test]
    fn test_unsupported_format_error_message() {
        let error = BntoError::UnsupportedFormat("BMP".to_string());
        assert_eq!(error.to_string(), "Unsupported format: BMP");
    }

    #[test]
    fn test_processing_failed_error_message() {
        let error = BntoError::ProcessingFailed("decoder returned invalid data".to_string());
        assert_eq!(
            error.to_string(),
            "Processing failed: decoder returned invalid data"
        );
    }

    #[test]
    fn test_cancelled_error_message() {
        let error = BntoError::Cancelled;
        assert_eq!(error.to_string(), "Operation cancelled");
    }

    #[test]
    fn test_file_too_large_error_message() {
        // 5 GB in bytes
        let error = BntoError::FileTooLarge(5_000_000_000);
        assert_eq!(error.to_string(), "File too large: 5000000000 bytes");
    }

    #[test]
    fn test_out_of_memory_error_message() {
        let error = BntoError::OutOfMemory("failed to allocate 2GB buffer".to_string());
        assert_eq!(
            error.to_string(),
            "Out of memory: failed to allocate 2GB buffer"
        );
    }
}
