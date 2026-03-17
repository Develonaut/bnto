// NodeProcessor trait — the contract every node type must implement.

use crate::errors::BntoError;
use crate::metadata::{NodeCategory, NodeMetadata};
use crate::progress::ProgressReporter;

// =============================================================================
// Input and Output Types
// =============================================================================

/// The input data that a node receives for processing.
pub struct NodeInput {
    /// Raw file data as bytes.
    pub data: Vec<u8>,

    /// Original filename (e.g., "photo.jpg", "data.csv").
    pub filename: String,

    /// MIME type of the input (e.g., "image/jpeg", "text/csv").
    pub mime_type: Option<String>,

    /// Configuration parameters from the node definition.
    pub params: serde_json::Map<String, serde_json::Value>,
}

/// The output from a node after processing.
pub struct NodeOutput {
    /// Processed output files (one or more).
    pub files: Vec<OutputFile>,

    /// Processing metadata (timing, compression ratio, rows removed, etc.).
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// A single output file produced by a node.
pub struct OutputFile {
    pub data: Vec<u8>,
    pub filename: String,
    pub mime_type: String,
}

// =============================================================================
// The NodeProcessor Trait
// =============================================================================

/// The contract that every node type must implement.
///
/// Uses synchronous `process()` because wasm-bindgen doesn't support async
/// trait methods across the WASM boundary. Async is handled at the Web Worker level.
pub trait NodeProcessor {
    /// Unique name of this node type (e.g., "compress-images").
    fn name(&self) -> &str;

    /// Process a single input file and produce output.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError>;

    /// Validate parameters before processing. Returns validation errors (empty = valid).
    /// Default: no validation.
    fn validate(&self, _params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        Vec::new()
    }

    /// Return the processor's self-describing metadata.
    /// Default returns placeholder "unknown" metadata — useful for tests and mocks.
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "unknown".to_string(),
            operation: "default".to_string(),
            name: self.name().to_string(),
            description: String::new(),
            category: NodeCategory::Data,
            accepts: vec![],
            platforms: vec![],
            parameters: vec![],
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock processor that echoes input back as output.
    struct EchoProcessor;

    impl NodeProcessor for EchoProcessor {
        fn name(&self) -> &str {
            "echo"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// Mock processor that always fails.
    struct FailProcessor;

    impl NodeProcessor for FailProcessor {
        fn name(&self) -> &str {
            "fail"
        }

        fn process(
            &self,
            _input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            Err(BntoError::ProcessingFailed(
                "intentional test failure".to_string(),
            ))
        }
    }

    fn make_test_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        }
    }

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

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].data, b"hello world");
        assert_eq!(output.files[0].filename, "test.txt");
    }

    #[test]
    fn test_fail_processor_returns_error() {
        let processor = FailProcessor;
        let progress = ProgressReporter::new_noop();
        let input = make_test_input(b"data", "test.txt");

        let result = processor.process(input, &progress);
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
}
