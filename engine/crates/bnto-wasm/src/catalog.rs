// Node Catalog — WASM export returning a JSON string describing every
// registered processor, node type, and the Definition JSON Schema.
// Used by codegen to generate TypeScript `NODE_TYPE_INFO` and schemas.

use wasm_bindgen::prelude::*;

use serde::Serialize;

/// Top-level catalog envelope with version, node types, processors, and schema.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogEnvelope {
    version: String,
    node_types: Vec<bnto_core::NodeTypeInfo>,
    processors: Vec<bnto_core::NodeMetadata>,
    definition_schema: serde_json::Value,
}

/// Return a pretty-printed JSON string of the engine's full catalog.
#[wasm_bindgen]
pub fn node_catalog() -> Result<String, JsValue> {
    let registry = bnto_engine::create_default_registry();
    let mut catalog = registry.catalog();

    // Sort by node type for deterministic output across builds.
    catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

    let envelope = CatalogEnvelope {
        version: bnto_core::FORMAT_VERSION.to_string(),
        node_types: bnto_core::all_node_types(),
        processors: catalog,
        definition_schema: bnto_core::definition_json_schema(),
    };

    serde_json::to_string_pretty(&envelope)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize catalog: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_envelope_has_correct_version() {
        // The catalog version should match bnto-core's FORMAT_VERSION.
        let registry = bnto_engine::create_default_registry();
        let catalog = registry.catalog();

        let envelope = CatalogEnvelope {
            version: bnto_core::FORMAT_VERSION.to_string(),
            node_types: bnto_core::all_node_types(),
            processors: catalog,
            definition_schema: bnto_core::definition_json_schema(),
        };

        assert_eq!(envelope.version, bnto_core::FORMAT_VERSION);
    }

    #[test]
    fn test_catalog_has_all_nine_processors() {
        // The default registry has 9 processors, so the catalog should too.
        let registry = bnto_engine::create_default_registry();
        let catalog = registry.catalog();

        assert_eq!(catalog.len(), 9, "Catalog should have exactly 9 processors");
    }

    #[test]
    fn test_catalog_contains_expected_node_types() {
        // Verify all 7 expected node type keys are present.
        let registry = bnto_engine::create_default_registry();
        let catalog = registry.catalog();

        let keys: Vec<&str> = catalog.iter().map(|m| m.node_type.as_str()).collect();

        let expected = [
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "spreadsheet-clean",
            "spreadsheet-rename",
            "spreadsheet-convert",
            "spreadsheet-merge",
            "file-rename",
        ];

        for key in &expected {
            assert!(
                keys.contains(key),
                "Catalog should contain '{}', got: {:?}",
                key,
                keys
            );
        }
    }

    #[test]
    fn test_all_processors_support_browser_platform() {
        // Every processor in the default registry should include "browser"
        // in its platforms list (all 6 current processors run via WASM).
        let registry = bnto_engine::create_default_registry();
        let catalog = registry.catalog();

        for entry in &catalog {
            assert!(
                entry.platforms.contains(&"browser".to_string()),
                "{} should include 'browser' platform",
                entry.node_type
            );
        }
    }

    #[test]
    fn test_catalog_serializes_to_valid_json() {
        // The full catalog should serialize to valid, parseable JSON.
        let registry = bnto_engine::create_default_registry();
        let mut catalog = registry.catalog();
        catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        let envelope = CatalogEnvelope {
            version: bnto_core::FORMAT_VERSION.to_string(),
            node_types: bnto_core::all_node_types(),
            processors: catalog,
            definition_schema: bnto_core::definition_json_schema(),
        };

        let json = serde_json::to_string_pretty(&envelope).unwrap();

        // Parse back to verify it's valid JSON.
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify top-level structure.
        assert!(parsed["version"].is_string());
        assert!(parsed["nodeTypes"].is_array());
        assert_eq!(parsed["nodeTypes"].as_array().unwrap().len(), 18);
        assert!(parsed["processors"].is_array());
        assert_eq!(parsed["processors"].as_array().unwrap().len(), 9);
        // The definitionSchema should be present as a JSON object.
        assert!(
            parsed["definitionSchema"].is_object(),
            "Catalog should include definitionSchema as a JSON object"
        );
        // Verify it has the expected $ref to the Definition type.
        assert_eq!(
            parsed["definitionSchema"]["$ref"], "#/$defs/Definition",
            "definitionSchema should reference $defs/Definition"
        );
    }

    /// Generate the catalog snapshot file at `engine/catalog.snapshot.json`.
    ///
    /// Run with: `cargo test --package bnto-wasm generate_catalog_snapshot -- --nocapture`
    /// Or via: `task wasm:snapshot`
    ///
    /// This is an "ignored" test — it only runs when explicitly requested.
    /// It writes to a file, which isn't something normal tests should do.
    #[test]
    #[ignore]
    fn generate_catalog_snapshot() {
        let registry = bnto_engine::create_default_registry();
        let mut catalog = registry.catalog();
        catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        let envelope = CatalogEnvelope {
            version: bnto_core::FORMAT_VERSION.to_string(),
            node_types: bnto_core::all_node_types(),
            processors: catalog,
            definition_schema: bnto_core::definition_json_schema(),
        };

        let json = serde_json::to_string_pretty(&envelope).unwrap();

        // Write to engine/catalog.snapshot.json (two levels up from crates/bnto-wasm/).
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("catalog.snapshot.json");

        std::fs::write(&snapshot_path, &json).unwrap();
        println!("Wrote catalog snapshot to: {}", snapshot_path.display());
        println!("{}", json);
    }

    #[test]
    fn test_catalog_sort_order_is_deterministic() {
        // Running catalog() twice should produce the same sorted order.
        let registry = bnto_engine::create_default_registry();
        let mut catalog1 = registry.catalog();
        catalog1.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        let registry2 = bnto_engine::create_default_registry();
        let mut catalog2 = registry2.catalog();
        catalog2.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        // Both should produce the same node type keys in the same order.
        let keys1: Vec<&str> = catalog1.iter().map(|m| m.node_type.as_str()).collect();
        let keys2: Vec<&str> = catalog2.iter().map(|m| m.node_type.as_str()).collect();

        assert_eq!(keys1, keys2);
    }
}
