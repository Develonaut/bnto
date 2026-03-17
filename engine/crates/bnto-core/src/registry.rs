// Node registry — maps compound keys (e.g., "image:compress") to processor instances.
// Decouples the executor from specific node types.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

use crate::metadata::NodeMetadata;
use crate::processor::NodeProcessor;

// =============================================================================
// Node Registry
// =============================================================================

/// Maps compound keys ("nodeType:operation") to processor instances via
/// dynamic dispatch (`Box<dyn NodeProcessor>`).
pub struct NodeRegistry {
    processors: HashMap<String, Box<dyn NodeProcessor>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// Register a processor under a compound key (e.g., "image:compress").
    /// Last registration wins if the key already exists.
    pub fn register(&mut self, key: &str, processor: Box<dyn NodeProcessor>) {
        self.processors.insert(key.to_string(), processor);
    }

    /// Look up the processor for a given node type and params.
    /// Builds the compound key from `node_type` + `params.operation`.
    pub fn resolve(
        &self,
        node_type: &str,
        params: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<&dyn NodeProcessor> {
        let operation = params
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // Stack-allocated buffer avoids heap allocation per resolve() call.
        let mut key_buf = String::with_capacity(128);
        let _ = write!(key_buf, "{}:{}", node_type, operation);

        self.processors.get(&key_buf).map(|b| b.as_ref())
    }

    pub fn len(&self) -> usize {
        self.processors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processors.is_empty()
    }

    /// Collect metadata from all registered processors into a catalog.
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

    struct MockProcessor {
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

        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );

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

        let processor = registry.resolve("image", &params);
        assert!(processor.is_none());
    }

    #[test]
    fn test_resolve_unknown_operation_returns_none() {
        let mut registry = NodeRegistry::new();
        registry.register("image:compress", Box::new(MockProcessor::new("compress")));

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

        let mut params = serde_json::Map::new();
        params.insert(
            "operation".to_string(),
            serde_json::Value::String("compress".to_string()),
        );
        assert_eq!(registry.resolve("image", &params).unwrap().name(), "new");
    }
}
