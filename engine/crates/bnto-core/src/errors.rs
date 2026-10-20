// =============================================================================
// Error Types
// =============================================================================
//
// All error types for the Bnto WASM engine. When something goes wrong during
// node execution (bad input, unsupported format, etc.), we return one of
// these errors instead of crashing.

use thiserror::Error;

use crate::processor::NodeOutput;

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

    /// The run passed its time limit (`settings.timeout`, or the consumer's
    /// override), so nothing more of it started.
    #[error("Run timed out after {secs}s")]
    TimedOut { secs: u64 },

    // --- Resource Errors ---
    /// A file is too large for browser processing.
    /// Browser memory is limited (~2GB), so we check file sizes before
    /// trying to process them.
    #[error("File too large: {0} bytes")]
    FileTooLarge(u64),

    /// The browser ran out of memory while processing.
    /// WASM has a limited memory heap. If we try to load a huge image,
    /// we might exhaust it.
    #[error("Out of memory: {0}")]
    OutOfMemory(String),

    // --- Failures That Carry More ---
    /// An external command ran and exited non-zero. `stdout` is what it
    /// printed, so a caller can still use it. Displays like `ProcessingFailed`.
    #[error("Processing failed: {message}")]
    CommandFailed { message: String, stdout: Vec<u8> },

    /// A step failed but still handed over output (e.g. a failed command's
    /// stdout), which the run's closing steps receive. Displays as the
    /// underlying error. Build with [`BntoError::with_output`].
    #[error("{}", .0.error)]
    FailedWithOutput(Box<FailedOutput>),

    /// A closing step (a node marked `always`) failed after an earlier step
    /// had already failed the run. `first` stays the run's verdict.
    #[error("{first}; closing step '{node_id}' also failed: {error}")]
    ClosingStepFailed {
        first: Box<BntoError>,
        node_id: String,
        error: Box<BntoError>,
    },
}

/// The error and output of a step that failed after making files.
pub struct FailedOutput {
    pub error: BntoError,
    pub output: NodeOutput,
}

// File bytes would flood a debug print, so show the file names only.
impl std::fmt::Debug for FailedOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&str> = self
            .output
            .files
            .iter()
            .map(|o| o.filename.as_str())
            .collect();
        f.debug_struct("FailedOutput")
            .field("error", &self.error)
            .field("files", &names)
            .finish()
    }
}

impl BntoError {
    /// `error`, handing over `output` along with the failure. Replaces any
    /// output `error` already carried.
    pub fn with_output(error: BntoError, output: NodeOutput) -> Self {
        let (error, _) = error.into_parts();
        Self::FailedWithOutput(Box::new(FailedOutput { error, output }))
    }

    /// Whether this failure is, or began with, the run passing its time
    /// limit. A timeout ends the run: no closing step or skipped loop item
    /// runs past it.
    pub fn is_timeout(&self) -> bool {
        match self {
            Self::TimedOut { .. } => true,
            Self::FailedWithOutput(failed) => failed.error.is_timeout(),
            Self::ClosingStepFailed { first, error, .. } => {
                first.is_timeout() || error.is_timeout()
            }
            _ => false,
        }
    }

    /// The error itself and the output it handed over, if any.
    pub fn into_parts(self) -> (BntoError, Option<NodeOutput>) {
        match self {
            Self::FailedWithOutput(failed) => (failed.error, Some(failed.output)),
            error => (error, None),
        }
    }
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
    fn test_timed_out_error_names_the_limit() {
        let error = BntoError::TimedOut { secs: 30 };
        assert_eq!(error.to_string(), "Run timed out after 30s");
    }

    #[test]
    fn test_a_timeout_is_seen_through_what_wraps_it() {
        let output = NodeOutput {
            files: vec![],
            metadata: serde_json::Map::new(),
        };
        let wrapped = BntoError::with_output(BntoError::TimedOut { secs: 1 }, output);
        assert!(wrapped.is_timeout());
        let closing = BntoError::ClosingStepFailed {
            first: Box::new(BntoError::ProcessingFailed("killed".into())),
            node_id: "cleanup".into(),
            error: Box::new(BntoError::TimedOut { secs: 1 }),
        };
        assert!(closing.is_timeout());
        assert!(!BntoError::Cancelled.is_timeout());
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

    fn log_output() -> NodeOutput {
        NodeOutput {
            files: vec![crate::processor::OutputFile {
                data: crate::FileData::Bytes(b"log".to_vec()),
                filename: "tool.log".to_string(),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata: serde_json::Map::new(),
        }
    }

    #[test]
    fn test_command_failed_displays_like_processing_failed() {
        let error = BntoError::CommandFailed {
            message: "Command 'tool' failed (exit 2): boom".to_string(),
            stdout: b"partial".to_vec(),
        };
        assert_eq!(
            error.to_string(),
            "Processing failed: Command 'tool' failed (exit 2): boom"
        );
    }

    #[test]
    fn test_failed_with_output_displays_as_its_error_and_splits_back() {
        let inner = BntoError::ProcessingFailed("tool exited 2".to_string());
        let error = BntoError::with_output(inner, log_output());
        assert_eq!(error.to_string(), "Processing failed: tool exited 2");
        assert!(format!("{error:?}").contains("tool.log"), "{error:?}");
        let (error, output) = error.into_parts();
        assert!(matches!(error, BntoError::ProcessingFailed(_)));
        assert_eq!(output.unwrap().files[0].filename, "tool.log");
    }

    #[test]
    fn test_into_parts_of_a_plain_error_has_no_output() {
        let (error, output) = BntoError::Cancelled.into_parts();
        assert!(matches!(error, BntoError::Cancelled));
        assert!(output.is_none());
    }

    #[test]
    fn test_closing_step_failed_names_both_first_failure_first() {
        let error = BntoError::ClosingStepFailed {
            first: Box::new(BntoError::ProcessingFailed("build broke".to_string())),
            node_id: "cleanup".to_string(),
            error: Box::new(BntoError::InvalidInput("no file".to_string())),
        };
        assert_eq!(
            error.to_string(),
            "Processing failed: build broke; closing step 'cleanup' also failed: \
             Invalid input: no file"
        );
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
