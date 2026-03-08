// =============================================================================
// NodeProcessor Trait — The Contract Every Node Type Must Implement
// =============================================================================
//
// WHAT IS THIS FILE?
// This defines the "interface" (called a "trait" in Rust) that every node
// type must implement. If you want to create a new node type (like image
// compression or CSV cleaning), you implement this trait.
//
// RUST CONCEPT: Traits
// A trait is like a TypeScript `interface` — it defines a set of methods
// that a type must provide. Unlike interfaces, traits can also provide
// default implementations for some methods.
//
//   trait NodeProcessor {
//       fn process(...) -> Result<...>;  // You MUST implement this
//       fn name() -> &str;               // You MUST implement this
//   }
//
// Then a specific node type "implements" the trait:
//
//   impl NodeProcessor for ImageCompressor {
//       fn process(...) -> Result<...> { /* compression logic */ }
//       fn name() -> &str { "compress-images" }
//   }
//
// WHY A TRAIT (instead of just functions)?
// 1. It guarantees every node type has the same interface
// 2. We can write generic code that works with ANY node type
// 3. We can test with mock implementations
// 4. The compiler catches missing methods at build time (not runtime!)

use crate::errors::BntoError;
use crate::metadata::{NodeCategory, NodeMetadata};
use crate::progress::ProgressReporter;

// =============================================================================
// Input and Output Types
// =============================================================================

/// The input data that a node receives for processing.
///
/// RUST CONCEPT: `pub struct` with `pub` fields
/// A `struct` is like a TypeScript `interface` or a JavaScript plain object.
/// Each field has a name and a type. `pub` makes the field accessible
/// from outside this module.
///
/// RUST CONCEPT: `Vec<u8>`
/// `Vec` is Rust's dynamic array (like JavaScript's `Array`).
/// `u8` is an unsigned 8-bit integer (0-255) — one byte.
/// So `Vec<u8>` is a "vector of bytes" — raw file data.
/// This is how we represent file contents in Rust.
pub struct NodeInput {
    /// The raw file data (bytes). For an image, this is the JPEG/PNG/WebP
    /// binary data. For a CSV, this is the UTF-8 text content as bytes.
    pub data: Vec<u8>,

    /// The original filename (e.g., "photo.jpg", "data.csv").
    /// Used to determine the file format and to generate output filenames.
    pub filename: String,

    /// The MIME type of the input (e.g., "image/jpeg", "text/csv").
    /// This helps us quickly determine how to process the file without
    /// having to inspect the file contents ("magic bytes").
    ///
    /// RUST CONCEPT: `Option<T>`
    /// `Option<String>` means "there might or might not be a String here".
    /// It's like `string | null` in TypeScript. It has two variants:
    ///   - `Some("image/jpeg")` = we have a value
    ///   - `None` = no value (the MIME type wasn't provided)
    ///
    /// This is Rust's way of handling nullable values safely — the compiler
    /// forces you to check for `None` before using the value.
    pub mime_type: Option<String>,

    /// Configuration parameters for the node (e.g., quality level, target
    /// format, dimensions). This is a JSON-compatible map of key-value pairs.
    /// The keys are strings (parameter names from @bnto/nodes schemas),
    /// and the values are JSON values (strings, numbers, booleans, etc.).
    ///
    /// RUST CONCEPT: `serde_json::Value`
    /// This is Rust's equivalent of JavaScript's `any` — it can hold any
    /// JSON-compatible value. We use it here because different node types
    /// have different parameters, and we don't want to define a struct
    /// for every possible combination.
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
}

// =============================================================================
// The NodeProcessor Trait
// =============================================================================

/// The contract that every node type must implement.
///
/// RUST CONCEPT: `async_trait` vs `async fn in traits`
/// As of Rust 1.75+, traits can have `async fn` methods directly.
/// However, wasm-bindgen doesn't natively support async trait methods
/// across the WASM boundary. So for now, we use synchronous `process()`
/// and handle async at the Web Worker level (the JS wrapper calls
/// process() and yields to the event loop between files).
///
/// RUST CONCEPT: `&self`
/// `&self` means "a reference to the current instance" — like `this` in JS,
/// but immutable (read-only). The `&` means we're borrowing, not taking
/// ownership. The caller still owns the processor after calling the method.
///
/// RUST CONCEPT: `Result<NodeOutput, BntoError>`
/// This return type means "either a successful NodeOutput OR a BntoError".
/// The caller MUST handle both cases. There's no way to accidentally
/// ignore the error — the compiler enforces it.
pub trait NodeProcessor {
    /// The unique name of this node type (e.g., "compress-images").
    /// Used for logging and progress reporting.
    fn name(&self) -> &str;

    /// Process a single input file and produce output.
    ///
    /// Arguments:
    ///   - `&self` — reference to the node processor instance
    ///   - `input` — the file data, filename, MIME type, and config params
    ///   - `progress` — callback to report progress to the UI (0-100%)
    ///
    /// Returns:
    ///   - `Ok(NodeOutput)` — processing succeeded, here are the results
    ///   - `Err(BntoError)` — processing failed, here's what went wrong
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError>;

    /// Validate the input parameters before processing.
    ///
    /// This is called BEFORE `process()` to catch configuration errors
    /// early (missing required params, invalid values, etc.) without
    /// doing any expensive file processing.
    ///
    /// Returns a list of validation errors (empty = valid).
    ///
    /// RUST CONCEPT: Default method implementation
    /// This method has a body (`{ Vec::new() }`) — it's a "default
    /// implementation". Types that implement NodeProcessor get this
    /// method for free. They CAN override it with custom validation,
    /// but they don't HAVE to. By default, validation passes (empty vec).
    fn validate(&self, _params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        // Default: no validation errors. Override in specific node types
        // to add parameter validation.
        //
        // RUST CONCEPT: `Vec::new()`
        // Creates a new empty vector (dynamic array). Like `[]` in JavaScript.
        Vec::new()
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

    // --- Test helpers ---
    // We create a simple mock processor to test the trait contract.

    /// A mock node processor for testing. Does nothing — just echoes
    /// the input back as output.
    struct EchoProcessor;

    // RUST CONCEPT: `impl Trait for Type`
    // This is how you implement a trait (interface) for a specific type.
    // The compiler checks that you've implemented ALL required methods.
    impl NodeProcessor for EchoProcessor {
        fn name(&self) -> &str {
            "echo"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            // Just echo the input data back as output.
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
        ) -> Result<NodeOutput, BntoError> {
            // Always return an error.
            // RUST CONCEPT: `Err(...)` is the error variant of Result.
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

        // RUST CONCEPT: `.unwrap()`
        // `.unwrap()` extracts the value from `Ok(...)` or panics if
        // it's `Err(...)`. In tests, panicking is fine — it means the
        // test fails. In production code, NEVER use unwrap() — always
        // handle errors properly.
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

        // RUST CONCEPT: `.is_err()`
        // Checks if a Result is the `Err` variant without unwrapping.
        let result = processor.process(input, &progress);
        assert!(result.is_err());

        // RUST CONCEPT: `if let` pattern matching
        // `if let Err(e) = result` means "if result is Err, bind the
        // error to `e` and run this block". It's a concise way to
        // match on one specific pattern.
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
