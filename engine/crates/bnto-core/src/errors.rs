// Error types for the Bnto WASM engine.

use thiserror::Error;

/// All possible errors from Bnto WASM node operations.
#[derive(Debug, Error)]
pub enum BntoError {
    /// Invalid input data (wrong type, missing required field, etc.).
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// File format not supported by this node.
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Something went wrong during file processing.
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    /// User cancelled the operation.
    #[error("Operation cancelled")]
    Cancelled,

    /// File too large for browser processing (WASM memory is limited).
    #[error("File too large: {0} bytes")]
    FileTooLarge(u64),

    /// Browser ran out of memory while processing.
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
}

// NOTE: The `impl From<BntoError> for JsValue` conversion lives in
// `bnto-wasm/src/lib.rs` to keep bnto-core target-agnostic
// (no wasm-bindgen/js-sys dependency here).

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_error_message() {
        let error = BntoError::InvalidInput("missing file data".to_string());
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
