// =============================================================================
// node_catalog — WASM Export for the Node Catalog
// =============================================================================
//
// WHAT IS THIS FILE?
// This provides a single WASM function that returns a JSON string describing
// every registered node processor. The web app (or a build script) can call
// this to get the engine's self-describing catalog — what nodes exist, what
// parameters they accept, what files they handle, etc.
//
// WHY IS THIS USEFUL?
// Currently, node metadata lives in `@bnto/nodes` (TypeScript). By also
// exporting it from the engine, we can:
//   1. Validate that the TS and Rust definitions stay in sync
//   2. Eventually use the engine as the single source of truth
//   3. Generate JSON Schema for `.bnto.json` files from the engine
//
// HOW THE WEB APP USES THIS:
//   ```js
//   import { node_catalog } from './bnto_wasm.js';
//   const catalogJson = node_catalog();
//   const catalog = JSON.parse(catalogJson);
//   // catalog = { version: "1.0.0", processors: [...] }
//   ```

use wasm_bindgen::prelude::*;

use serde::Serialize;

// =============================================================================
// Catalog Envelope — Wraps the processor list with version info
// =============================================================================

/// The top-level catalog structure returned by `node_catalog()`.
///
/// Includes the format version so consumers can verify compatibility.
/// The `version` field matches `FORMAT_VERSION` in bnto-core, which must
/// stay in sync with `CURRENT_FORMAT_VERSION` in `@bnto/nodes`.
///
/// RUST CONCEPT: `#[serde(rename_all = "camelCase")]`
/// Ensures JSON output uses camelCase keys (JavaScript convention).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogEnvelope {
    /// The format version of the catalog (matches bnto-core FORMAT_VERSION).
    /// Consumers can use this to verify they understand the catalog schema.
    version: String,

    /// The list of all registered processor metadata entries.
    /// Sorted by compound key (nodeType:operation) for stable output.
    processors: Vec<bnto_core::NodeMetadata>,
}

// =============================================================================
// WASM Export: node_catalog
// =============================================================================

/// Return a JSON string describing every registered node processor.
///
/// This is the engine's self-describing catalog — the single source of truth
/// for what nodes the engine supports, what parameters they accept, what file
/// types they handle, and whether they run in the browser.
///
/// The output is a JSON object with two fields:
///   - `version` (string) — the format version (e.g., "1.0.0")
///   - `processors` (array) — one entry per registered processor
///
/// Each processor entry includes: nodeType, operation, name, description,
/// category, accepts (MIME types), browserCapable, and parameters.
///
/// # Returns
/// A JSON string (pretty-printed for readability).
///
/// # Errors
/// Returns a JsValue error if JSON serialization fails (shouldn't happen
/// with well-formed metadata, but we handle it gracefully).
#[wasm_bindgen]
pub fn node_catalog() -> Result<String, JsValue> {
    // --- Step 1: Create the default registry with all 6 processors ---
    let registry = super::execute::create_default_registry();

    // --- Step 2: Collect metadata from all processors ---
    let mut catalog = registry.catalog();

    // --- Step 3: Sort by compound key for deterministic output ---
    // HashMap iteration order is not guaranteed, so we sort by
    // nodeType:operation to ensure the snapshot is stable across builds.
    catalog.sort_by(|a, b| {
        let key_a = format!("{}:{}", a.node_type, a.operation);
        let key_b = format!("{}:{}", b.node_type, b.operation);
        key_a.cmp(&key_b)
    });

    // --- Step 4: Wrap in an envelope with the format version ---
    let envelope = CatalogEnvelope {
        version: bnto_core::FORMAT_VERSION.to_string(),
        processors: catalog,
    };

    // --- Step 5: Serialize to pretty JSON ---
    serde_json::to_string_pretty(&envelope)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize catalog: {}", e)))
}

// =============================================================================
// Tests (native — no WASM boundary needed)
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
            processors: catalog,
        };

        assert_eq!(envelope.version, bnto_core::FORMAT_VERSION);
    }

    #[test]
    fn test_catalog_has_all_six_processors() {
        // The default registry has 6 processors, so the catalog should too.
        let registry = crate::execute::create_default_registry();
        let catalog = registry.catalog();

        assert_eq!(
            catalog.len(),
            6,
            "Catalog should have exactly 6 processors"
        );
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
                entry.node_type, entry.operation
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
            processors: catalog,
        };

        let json = serde_json::to_string_pretty(&envelope).unwrap();

        // Parse back to verify it's valid JSON.
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify top-level structure.
        assert!(parsed["version"].is_string());
        assert!(parsed["processors"].is_array());
        assert_eq!(parsed["processors"].as_array().unwrap().len(), 6);
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
            processors: catalog,
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
