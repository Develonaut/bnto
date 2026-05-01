// =============================================================================
// NodeProcessor Trait — The Contract Every Node Type Must Implement
// =============================================================================

use crate::context::ProcessContext;
use crate::errors::BntoError;
use crate::metadata::{NodeCategory, NodeMetadata};
use crate::progress::ProgressReporter;

// =============================================================================
// Input and Output Types
// =============================================================================

/// The input data that a node receives for processing.
pub struct NodeInput {
    /// The raw file data (bytes). For an image, this is the JPEG/PNG/WebP
    /// binary data. For a CSV, this is the UTF-8 text content as bytes.
    pub data: Vec<u8>,

    /// The original filename (e.g., "photo.jpg", "data.csv").
    /// Used to determine the file format and to generate output filenames.
    pub filename: String,

    /// The MIME type of the input (e.g., "image/jpeg", "text/csv").
    /// `None` when the MIME type wasn't provided by the caller.
    pub mime_type: Option<String>,

    /// Configuration parameters for the node (e.g., quality level, target
    /// format, dimensions). A JSON-compatible map where keys are parameter
    /// names from @bnto/nodes schemas.
    pub params: serde_json::Map<String, serde_json::Value>,
}

/// The output from a node after processing.
///
/// A node can produce one or more output files. For example, the
/// compress-images node takes one image in and produces one compressed
/// image out. A future "split PDF" node might produce many pages.
pub struct NodeOutput {
    /// The processed file data. Each entry is one output file.
    pub files: Vec<OutputFile>,

    /// Optional metadata about the processing (timing, compression ratio,
    /// rows removed, etc.). Displayed in the UI's results panel.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// A single output file produced by a node.
pub struct OutputFile {
    /// The raw file data (bytes) of the processed output.
    pub data: Vec<u8>,

    /// The filename for this output (e.g., "photo-compressed.jpg").
    pub filename: String,

    /// The MIME type of the output (e.g., "image/jpeg").
    pub mime_type: String,

    /// Per-file metadata (e.g., CSV row column values for loop iteration).
    /// When non-empty, takes priority over `NodeOutput.metadata` in collect_output.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Input for batch processors that need all files at once (e.g., merge, zip).
///
/// Unlike `NodeInput` (one file), this carries the full set of pipeline files
/// plus the shared configuration parameters.
pub struct BatchInput {
    /// All files to process as a group.
    pub files: Vec<BatchFile>,

    /// Configuration parameters for the node (same as `NodeInput.params`).
    pub params: serde_json::Map<String, serde_json::Value>,
}

/// A single file within a batch input.
pub struct BatchFile {
    /// The raw file data (bytes).
    pub data: Vec<u8>,

    /// The original filename.
    pub filename: String,

    /// The MIME type, if known.
    pub mime_type: Option<String>,
}

// =============================================================================
// The NodeProcessor Trait
// =============================================================================

/// The contract that every node type must implement.
///
/// Currently synchronous -- async is handled at the Web Worker level.
/// wasm-bindgen doesn't support async trait methods across the WASM boundary.
pub trait NodeProcessor {
    /// The processor's type key (e.g., "image-compress").
    /// Must match the key used in `registry.register()`.
    /// Convention: category-operation, kebab-case.
    fn name(&self) -> &str;

    /// Process a single input file and produce output.
    ///
    /// Arguments:
    ///   - `&self` — reference to the node processor instance
    ///   - `input` — the file data, filename, MIME type, and config params
    ///   - `progress` — callback to report progress to the UI (0-100%)
    ///   - `ctx` — system access boundary (commands, temp files, env vars)
    ///
    /// Returns:
    ///   - `Ok(NodeOutput)` — processing succeeded, here are the results
    ///   - `Err(BntoError)` — processing failed, here's what went wrong
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError>;

    /// Validate the input parameters before processing.
    ///
    /// This is called BEFORE `process()` to catch configuration errors
    /// early (missing required params, invalid values, etc.) without
    /// doing any expensive file processing.
    ///
    /// Returns a list of validation errors (empty = valid).
    ///
    /// Default implementation passes validation. Override in specific
    /// node types to add parameter validation.
    fn validate(&self, _params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        Vec::new()
    }

    /// Process a batch of files together, producing combined output.
    ///
    /// Override this for processors with `InputCardinality::Batch` (merge, zip,
    /// concat). The default falls back to calling `process()` per file and
    /// concatenating results — suitable for `PerFile` processors.
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let total = input.files.len();
        let mut all_files = Vec::new();
        let mut combined_metadata = serde_json::Map::new();

        for (i, file) in input.files.into_iter().enumerate() {
            let pct = ((i as u32) * 100) / (total as u32).max(1);
            progress.report(pct, &format!("Processing file {} of {total}...", i + 1));

            let single_input = NodeInput {
                data: file.data,
                filename: file.filename,
                mime_type: file.mime_type,
                params: input.params.clone(),
            };
            let output = self.process(single_input, progress, ctx)?;
            all_files.extend(output.files);
            // Merge metadata from the last file processed.
            combined_metadata = output.metadata;
        }

        Ok(NodeOutput {
            files: all_files,
            metadata: combined_metadata,
        })
    }

    /// Return the processor's self-describing metadata.
    ///
    /// This tells consumers everything about this processor: what it's called,
    /// what category it belongs to, what parameters it accepts, what files it
    /// handles, and whether it runs in the browser.
    ///
    /// Every concrete processor SHOULD override this with its real metadata.
    /// The default returns a placeholder "unknown" metadata — useful for tests
    /// and mocks that don't need real metadata.
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "unknown".to_string(),
            name: self.name().to_string(),
            description: String::new(),
            category: NodeCategory::Data,
            accepts: vec![],
            platforms: vec![],
            parameters: vec![],
            input_cardinality: Default::default(),
            requires: vec![],
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NoopContext;

    // --- Test helpers ---
    // We create a simple mock processor to test the trait contract.

    /// A mock node processor for testing. Does nothing — just echoes
    /// the input back as output.
    struct EchoProcessor;

    impl NodeProcessor for EchoProcessor {
        fn name(&self) -> &str {
            "echo"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            // Just echo the input data back as output.
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                    metadata: serde_json::Map::new(),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// A mock processor that always fails — for testing error handling.
    struct FailProcessor;

    impl NodeProcessor for FailProcessor {
        fn name(&self) -> &str {
            "fail"
        }

        fn process(
            &self,
            _input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            Err(BntoError::ProcessingFailed(
                "intentional test failure".to_string(),
            ))
        }
    }

    /// Helper to create a simple test input.
    fn make_test_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        }
    }

    // --- Tests ---

    #[test]
    fn test_echo_processor_name() {
        let processor = EchoProcessor;
        assert_eq!(processor.name(), "echo");
    }

    #[test]
    fn test_echo_processor_echoes_data() {
        let processor = EchoProcessor;
        let progress = ProgressReporter::new_noop();
        let input = make_test_input(b"hello world", "test.txt");

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].data, b"hello world");
        assert_eq!(output.files[0].filename, "test.txt");
    }

    #[test]
    fn test_fail_processor_returns_error() {
        let processor = FailProcessor;
        let progress = ProgressReporter::new_noop();
        let input = make_test_input(b"data", "test.txt");

        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("intentional test failure"));
        }
    }

    #[test]
    fn test_default_validate_returns_empty() {
        let processor = EchoProcessor;
        let params = serde_json::Map::new();

        // The default validate() should return no errors.
        let errors = processor.validate(&params);
        assert!(errors.is_empty());
    }

    // --- Batch Processing Tests ---

    #[test]
    fn test_default_process_batch_falls_back_to_per_file() {
        let processor = EchoProcessor;
        let progress = ProgressReporter::new_noop();
        let input = BatchInput {
            files: vec![
                BatchFile {
                    data: b"file1".to_vec(),
                    filename: "a.txt".to_string(),
                    mime_type: None,
                },
                BatchFile {
                    data: b"file2".to_vec(),
                    filename: "b.txt".to_string(),
                    mime_type: None,
                },
            ],
            params: serde_json::Map::new(),
        };

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();

        // Default batch falls back to per-file: 2 inputs → 2 outputs.
        assert_eq!(output.files.len(), 2);
        assert_eq!(output.files[0].filename, "a.txt");
        assert_eq!(output.files[0].data, b"file1");
        assert_eq!(output.files[1].filename, "b.txt");
        assert_eq!(output.files[1].data, b"file2");
    }

    #[test]
    fn test_default_process_batch_empty_input() {
        let processor = EchoProcessor;
        let progress = ProgressReporter::new_noop();
        let input = BatchInput {
            files: vec![],
            params: serde_json::Map::new(),
        };

        let output = processor
            .process_batch(input, &progress, &NoopContext)
            .unwrap();
        assert_eq!(output.files.len(), 0);
    }

    #[test]
    fn test_default_process_batch_propagates_errors() {
        let processor = FailProcessor;
        let progress = ProgressReporter::new_noop();
        let input = BatchInput {
            files: vec![BatchFile {
                data: b"data".to_vec(),
                filename: "test.txt".to_string(),
                mime_type: None,
            }],
            params: serde_json::Map::new(),
        };

        let result = processor.process_batch(input, &progress, &NoopContext);
        assert!(result.is_err());
    }
}
