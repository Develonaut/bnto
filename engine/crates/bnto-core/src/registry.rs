// Node registry — maps node type keys (e.g., "image-compress") to NodeProcessor
// instances. Decouples the executor from specific node types; consumers
// register only the processors they need (all 6 for WASM, subset for CLI/tests).

use crate::metadata::NodeMetadata;
use crate::processor::NodeProcessor;
use std::collections::HashMap;

// =============================================================================
// Node Registry
// =============================================================================

/// A registry that maps node type keys (e.g., "image-compress") to node processors.
///
/// Uses `Box<dyn NodeProcessor>` for dynamic dispatch -- the registry
/// holds heterogeneous processor types behind a single trait object.
pub struct NodeRegistry {
    /// Maps node type keys (e.g., "image-compress") to processor instances.
    processors: HashMap<String, Box<dyn NodeProcessor>>,
}

impl NodeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// Register a processor under a node type key.
    ///
    /// The key should be the node type name
    /// (e.g., "image-compress", "spreadsheet-clean").
    ///
    /// If a processor is already registered under this key, it will
    /// be replaced (last registration wins).
    ///
    /// # Arguments
    /// - `key` — the node type key (e.g., "image-compress")
    /// - `processor` — the processor instance to register
    pub fn register(&mut self, key: &str, processor: Box<dyn NodeProcessor>) {
        self.processors.insert(key.to_string(), processor);
    }

    /// Look up the processor for a given node type.
    ///
    /// Does a direct lookup by `node_type` key (e.g., "image-compress").
    ///
    /// # Arguments
    /// - `node_type` — the node's type field (e.g., "image-compress", "spreadsheet-clean")
    /// - `_params` — the node's params map (reserved for future use)
    ///
    /// # Returns
    /// - `Some(&dyn NodeProcessor)` — if a matching processor was found
    /// - `None` — if no processor matches the key
    pub fn resolve(
        &self,
        node_type: &str,
        _params: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<&dyn NodeProcessor> {
        self.processors.get(node_type).map(|b| b.as_ref())
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
    /// sorts the output by node type for stable snapshots.
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
    use crate::context::ProcessContext;
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
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
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
        registry.register("image-compress", Box::new(MockProcessor::new("compress")));

        let params = serde_json::Map::new();

        // Resolve should find the processor by node type key.
        let processor = registry.resolve("image-compress", &params);
        assert!(processor.is_some());
        assert_eq!(processor.unwrap().name(), "compress");
    }

    #[test]
    fn test_resolve_unknown_type_returns_none() {
        let registry = NodeRegistry::new();
        let params = serde_json::Map::new();

        // Empty registry → None.
        let processor = registry.resolve("unknown-type", &params);
        assert!(processor.is_none());
    }

    #[test]
    fn test_register_multiple_processors() {
        let mut registry = NodeRegistry::new();
        registry.register("image-compress", Box::new(MockProcessor::new("compress")));
        registry.register("image-resize", Box::new(MockProcessor::new("resize")));
        registry.register(
            "spreadsheet-clean",
            Box::new(MockProcessor::new("clean-csv")),
        );

        assert_eq!(registry.len(), 3);

        let params = serde_json::Map::new();

        // Each resolves to the correct processor by type key.
        assert_eq!(
            registry.resolve("image-compress", &params).unwrap().name(),
            "compress"
        );
        assert_eq!(
            registry.resolve("image-resize", &params).unwrap().name(),
            "resize"
        );
        assert_eq!(
            registry
                .resolve("spreadsheet-clean", &params)
                .unwrap()
                .name(),
            "clean-csv"
        );
    }

    #[test]
    fn test_register_overwrites_existing() {
        let mut registry = NodeRegistry::new();
        registry.register("image-compress", Box::new(MockProcessor::new("old")));
        registry.register("image-compress", Box::new(MockProcessor::new("new")));

        let params = serde_json::Map::new();

        // Last registration wins.
        assert_eq!(
            registry.resolve("image-compress", &params).unwrap().name(),
            "new"
        );
    }
}
