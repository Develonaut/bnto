// =============================================================================
// Node Registry — Maps Node Types to Processors
// =============================================================================
//
// WHAT IS THIS FILE?
// When the executor encounters a node like `{ type: "image", params: { operation: "compress" } }`,
// it needs to find the right Rust processor to handle it. The registry is the
// lookup table that maps compound keys (like "image:compress") to processor
// instances (like `CompressImages`).
//
// WHY A REGISTRY (instead of a big match statement)?
// 1. The executor doesn't need to know about specific node types — it just
//    asks the registry "do you have a processor for this?" Loose coupling.
// 2. Different consumers can register different processors. The browser WASM
//    module registers all 6 processors. A CLI tool might only register image
//    processors. A test can register mock processors.
// 3. New node types can be added without modifying the executor code.
//
// HOW COMPOUND KEYS WORK:
// The registry uses `nodeType:operation` compound keys for dispatch:
//   - "image:compress"        → CompressImages processor
//   - "image:resize"          → ResizeImages processor
//   - "spreadsheet:clean"     → CleanCsv processor
//   - "file-system:rename"    → RenameFiles processor
//
// This matches the pattern already used in the JS-side `wasmLoader.ts`.
// The `resolve()` method extracts the `operation` field from the node's
// params to build the compound key automatically.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

use crate::metadata::NodeMetadata;
use crate::processor::NodeProcessor;

// =============================================================================
// Node Registry
// =============================================================================

/// A registry that maps compound keys to node processors.
///
/// RUST CONCEPT: `HashMap<String, Box<dyn NodeProcessor>>`
/// - `HashMap` is Rust's hash map (like JavaScript's `Map`).
/// - `Box<dyn NodeProcessor>` is a heap-allocated trait object — it can
///   hold ANY type that implements `NodeProcessor`. We don't know the
///   concrete type at compile time (it could be `CompressImages`,
///   `CleanCsv`, etc.), so we use dynamic dispatch via `dyn`.
/// - The `Box` is needed because trait objects are "unsized" — the
///   compiler doesn't know how much memory they need. `Box` puts them
///   on the heap with a fixed-size pointer.
pub struct NodeRegistry {
    /// Maps compound keys (e.g., "image:compress") to processor instances.
    processors: HashMap<String, Box<dyn NodeProcessor>>,
}

impl NodeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// Register a processor under a compound key.
    ///
    /// The key should be in the format "nodeType:operation"
    /// (e.g., "image:compress", "spreadsheet:clean").
    ///
    /// If a processor is already registered under this key, it will
    /// be replaced (last registration wins).
    ///
    /// # Arguments
    /// - `key` — the compound dispatch key (e.g., "image:compress")
    /// - `processor` — the processor instance to register
    pub fn register(&mut self, key: &str, processor: Box<dyn NodeProcessor>) {
        self.processors.insert(key.to_string(), processor);
    }

    /// Look up the processor for a given node type and params.
    ///
    /// Builds the compound key from `node_type` + `params.operation`,
    /// then looks it up in the registry.
    ///
    /// # Arguments
    /// - `node_type` — the node's type field (e.g., "image", "spreadsheet")
    /// - `params` — the node's params map (must contain an "operation" field)
    ///
    /// # Returns
    /// - `Some(&dyn NodeProcessor)` — if a matching processor was found
    /// - `None` — if no processor matches the compound key
    ///
    /// RUST CONCEPT: `Option<&dyn NodeProcessor>`
    /// Returns a reference to the processor (borrowed, not moved).
    /// The registry keeps ownership — the caller can USE the processor
    /// but can't take it out of the registry.
    pub fn resolve(
        &self,
        node_type: &str,
        params: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<&dyn NodeProcessor> {
        // --- Step 1: Extract the operation from params ---
        // The `operation` field tells us which specific processor to use.
        // For example, node_type="image" + operation="compress" → "image:compress".
        let operation = params
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // --- Step 2: Build the compound key on the stack ---
        // PERFORMANCE: `format!()` allocates a new String on the heap every
        // call. Since `resolve()` is called once per node per file, that's
        // thousands of allocations for large batches. Instead, we use a
        // stack-allocated buffer (128 bytes is plenty for "node-type:operation"
        // keys) and only allocate if the key is unusually long.
        let mut key_buf = String::with_capacity(128);
        let _ = write!(key_buf, "{}:{}", node_type, operation);

        // --- Step 3: Look up in the registry ---
        // `.get()` returns `Option<&Box<dyn NodeProcessor>>`.
        // `.map(|b| b.as_ref())` converts `&Box<dyn NodeProcessor>` to
        // `&dyn NodeProcessor` (removes the Box layer for the caller).
        self.processors.get(&key_buf).map(|b| b.as_ref())
    }

    /// Check how many processors are registered (useful for tests).
    pub fn len(&self) -> usize {
        self.processors.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.processors.is_empty()
    }

    /// Collect metadata from ALL registered processors into a catalog.
    ///
    /// This returns a `Vec<NodeMetadata>` — one entry per registered processor.
    /// The order is determined by the HashMap's iteration order (not guaranteed,
    /// but deterministic for a given build). The `node_catalog()` WASM function
    /// sorts the output by compound key for stable snapshots.
    ///
    /// RUST CONCEPT: `.values()` and `.map()`
    /// `.values()` returns an iterator over the HashMap's values (the processors).
    /// `.map(|p| p.metadata())` calls `metadata()` on each processor.
    /// `.collect()` gathers the results into a Vec.
    pub fn catalog(&self) -> Vec<NodeMetadata> {
        self.processors.values().map(|p| p.metadata()).collect()
    }
}

impl Default for NodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::BntoError;
    use crate::processor::{NodeInput, NodeOutput, OutputFile};
    use crate::progress::ProgressReporter;

    // --- Mock Processor for Testing ---

    /// A simple mock processor that just echoes input back.
    struct MockProcessor {
        /// A name to identify this mock in tests.
        mock_name: String,
    }

    impl MockProcessor {
        fn new(name: &str) -> Self {
            Self {
                mock_name: name.to_string(),
            }
        }
    }

    impl NodeProcessor for MockProcessor {
        fn name(&self) -> &str {
            &self.mock_name
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

    // --- Tests ---

    #[test]
    fn test_new_registry_is_empty() {
        let registry = NodeRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_register_and_resolve() {
        let mut registry = NodeRegistry::new();
        registry.register("image:compress", Box::new(MockProcessor::new("compress")));

        // Build params with the operation field.
        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );

        // Resolve should find the processor.
        let processor = registry.resolve("image", &params);
        assert!(processor.is_some());
        assert_eq!(processor.unwrap().name(), "compress");
    }

    #[test]
    fn test_resolve_unknown_type_returns_none() {
        let registry = NodeRegistry::new();

        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );

        // Empty registry → None.
        let processor = registry.resolve("image", &params);
        assert!(processor.is_none());
    }

    #[test]
    fn test_resolve_unknown_operation_returns_none() {
        let mut registry = NodeRegistry::new();
        registry.register("image:compress", Box::new(MockProcessor::new("compress")));

        // Ask for an operation that wasn't registered.
        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("sharpen".to_string()),
        );

        let processor = registry.resolve("image", &params);
        assert!(processor.is_none());
    }

    #[test]
    fn test_resolve_missing_operation_uses_default() {
        let mut registry = NodeRegistry::new();
        registry.register(
            "custom:default",
            Box::new(MockProcessor::new("custom-default")),
        );

        // Params with no "operation" field → uses "default".
        let params = serde_json::Map::new();

        let processor = registry.resolve("custom", &params);
        assert!(processor.is_some());
        assert_eq!(processor.unwrap().name(), "custom-default");
    }

    #[test]
    fn test_register_multiple_processors() {
        let mut registry = NodeRegistry::new();
        registry.register("image:compress", Box::new(MockProcessor::new("compress")));
        registry.register("image:resize", Box::new(MockProcessor::new("resize")));
        registry.register(
            "spreadsheet:clean",
            Box::new(MockProcessor::new("clean-csv")),
        );

        assert_eq!(registry.len(), 3);

        // Each resolves to the correct processor.
        let mut compress_params = serde_json::Map::new();
        compress_params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );
        assert_eq!(
            registry.resolve("image", &compress_params).unwrap().name(),
            "compress"
        );

        let mut resize_params = serde_json::Map::new();
        resize_params.insert(
            "operation".to_string(),
            serde_json::Value::String("resize".to_string()),
        );
        assert_eq!(
            registry.resolve("image", &resize_params).unwrap().name(),
            "resize"
        );
    }

    #[test]
    fn test_register_overwrites_existing() {
        let mut registry = NodeRegistry::new();
        registry.register("image:compress", Box::new(MockProcessor::new("old")));
        registry.register("image:compress", Box::new(MockProcessor::new("new")));

        // Last registration wins.
        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );
        assert_eq!(registry.resolve("image", &params).unwrap().name(), "new");
    }
}
