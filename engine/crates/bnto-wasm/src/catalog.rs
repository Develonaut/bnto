// =============================================================================
// node_catalog — WASM export for the engine's self-describing node catalog
// =============================================================================
//
// Returns JSON describing every registered processor (params, MIME types,
// platforms). Used to validate TS @bnto/nodes definitions against the engine
// and to generate JSON Schema for .bnto.json files.

use wasm_bindgen::prelude::*;

use serde::Serialize;

// =============================================================================
// Catalog Envelope
// =============================================================================

/// Top-level catalog structure. Version must stay in sync with
/// `FORMAT_VERSION` in bnto-core and `CURRENT_FORMAT_VERSION` in @bnto/nodes.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogEnvelope {
    version: String,

    /// All registered node types (sorted alphabetically).
    node_types: Vec<bnto_core::NodeTypeInfo>,

    /// All implemented processor operations (sorted by nodeType:operation).
    processors: Vec<bnto_core::NodeMetadata>,

    /// JSON Schema (Draft 2020-12) for the `.bnto.json` Definition format.
    definition_schema: serde_json::Value,
}

// =============================================================================
// WASM Export: node_catalog
// =============================================================================

/// Return a pretty-printed JSON string describing every registered processor.
#[wasm_bindgen]
pub fn node_catalog() -> Result<String, JsValue> {
    let registry = super::execute::create_default_registry();

    let mut catalog = registry.catalog();

    // Sort by compound key for deterministic output across builds.
    catalog.sort_by(|a, b| {
        let key_a = format!("{}:{}", a.node_type, a.operation);
        let key_b = format!("{}:{}", b.node_type, b.operation);
        key_a.cmp(&key_b)
    });

    let node_types = bnto_core::all_node_types();
    let definition_schema = bnto_core::definition_json_schema();

    let envelope = CatalogEnvelope {
        version: bnto_core::FORMAT_VERSION.to_string(),
        node_types,
        processors: catalog,
        definition_schema,
    };

    serde_json::to_string_pretty(&envelope)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize catalog: {}", e)))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_envelope_has_correct_version() {
        // The catalog version should match bnto-core's FORMAT_VERSION.
        let registry = crate::execute::create_default_registry();
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
    fn test_catalog_has_all_six_processors() {
        // The default registry has 6 processors, so the catalog should too.
        let registry = crate::execute::create_default_registry();
        let catalog = registry.catalog();

        assert_eq!(catalog.len(), 6, "Catalog should have exactly 6 processors");
    }

    #[test]
    fn test_catalog_contains_expected_compound_keys() {
        // Verify all 6 expected compound keys are present.
        let registry = crate::execute::create_default_registry();
        let catalog = registry.catalog();

        let keys: Vec<String> = catalog
            .iter()
            .map(|m| format!("{}:{}", m.node_type, m.operation))
            .collect();

        let expected = [
            "image:compress",
            "image:resize",
            "image:convert",
            "spreadsheet:clean",
            "spreadsheet:rename",
            "file-system:rename",
        ];

        for key in &expected {
            assert!(
                keys.contains(&key.to_string()),
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
        let registry = crate::execute::create_default_registry();
        let catalog = registry.catalog();

        for entry in &catalog {
            assert!(
                entry.platforms.contains(&"browser".to_string()),
                "{}:{} should include 'browser' platform",
                entry.node_type,
                entry.operation
            );
        }
    }

    #[test]
    fn test_catalog_serializes_to_valid_json() {
        // The full catalog should serialize to valid, parseable JSON.
        let registry = crate::execute::create_default_registry();
        let mut catalog = registry.catalog();
        catalog.sort_by(|a, b| {
            let key_a = format!("{}:{}", a.node_type, a.operation);
            let key_b = format!("{}:{}", b.node_type, b.operation);
            key_a.cmp(&key_b)
        });

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
        assert_eq!(parsed["nodeTypes"].as_array().unwrap().len(), 12);
        assert!(parsed["processors"].is_array());
        assert_eq!(parsed["processors"].as_array().unwrap().len(), 6);
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
        let registry = crate::execute::create_default_registry();
        let mut catalog = registry.catalog();
        catalog.sort_by(|a, b| {
            let key_a = format!("{}:{}", a.node_type, a.operation);
            let key_b = format!("{}:{}", b.node_type, b.operation);
            key_a.cmp(&key_b)
        });

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
        let registry = crate::execute::create_default_registry();
        let mut catalog1 = registry.catalog();
        catalog1.sort_by(|a, b| {
            let key_a = format!("{}:{}", a.node_type, a.operation);
            let key_b = format!("{}:{}", b.node_type, b.operation);
            key_a.cmp(&key_b)
        });

        let registry2 = crate::execute::create_default_registry();
        let mut catalog2 = registry2.catalog();
        catalog2.sort_by(|a, b| {
            let key_a = format!("{}:{}", a.node_type, a.operation);
            let key_b = format!("{}:{}", b.node_type, b.operation);
            key_a.cmp(&key_b)
        });

        // Both should produce the same compound keys in the same order.
        let keys1: Vec<String> = catalog1
            .iter()
            .map(|m| format!("{}:{}", m.node_type, m.operation))
            .collect();
        let keys2: Vec<String> = catalog2
            .iter()
            .map(|m| format!("{}:{}", m.node_type, m.operation))
            .collect();

        assert_eq!(keys1, keys2);
    }
}
