// Node Catalog — WASM export returning a JSON string describing every
// registered processor, node type, and the Definition JSON Schema.
// Used by codegen to generate TypeScript `NODE_TYPE_INFO` and schemas.

use wasm_bindgen::prelude::*;

use serde::Serialize;

/// Top-level catalog envelope with version, node types, processors, schema, and recipes.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogEnvelope {
    version: String,
    node_types: Vec<CatalogNodeType>,
    processors: Vec<bnto_core::NodeMetadata>,
    definition_schema: serde_json::Value,
    recipes: Vec<RecipeEntry>,
}

/// A node-type entry in the catalog, joining `NodeTypeInfo` with the param
/// definitions the engine knows for that type. For non-processor types
/// (input, output, loop, group, parallel, transform, edit-fields) params
/// come from `bnto_core::node_type_params`. For processor types they come
/// from the registry's `NodeMetadata::parameters`. Types with no declared
/// params (e.g. `http-request`, `shell-command`) omit the field.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogNodeType {
    #[serde(flatten)]
    info: bnto_core::NodeTypeInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<bnto_core::ParameterDef>,
}

/// A recipe entry in the catalog snapshot.
#[derive(Serialize)]
struct RecipeEntry {
    slug: String,
    name: String,
    description: String,
    category: String,
    tags: Vec<String>,
    definition: serde_json::Value,
}

/// Build the catalog's `nodeTypes` array by joining every `NodeTypeInfo`
/// with its parameter definitions. Keeps the engine as the single source
/// of truth for the UI contract.
fn build_catalog_node_types(processors: &[bnto_core::NodeMetadata]) -> Vec<CatalogNodeType> {
    bnto_core::all_node_types()
        .into_iter()
        .map(|info| {
            let params = bnto_core::node_type_params(&info.name).unwrap_or_else(|| {
                processors
                    .iter()
                    .find(|m| m.node_type == info.name)
                    .map(|m| m.parameters.clone())
                    .unwrap_or_default()
            });
            CatalogNodeType { info, params }
        })
        .collect()
}

/// Return a pretty-printed JSON string of the engine's full catalog.
#[wasm_bindgen]
pub fn node_catalog() -> Result<String, JsValue> {
    let registry = bnto_engine::create_browser_registry();
    let mut catalog = registry.catalog();

    // Sort by node type for deterministic output across builds.
    catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

    let recipes: Vec<RecipeEntry> = bnto_engine::recipes::builtin_recipes()
        .into_iter()
        .map(|r| RecipeEntry {
            slug: r.slug,
            name: r.name,
            description: r.description,
            category: r.category,
            tags: r.tags,
            definition: serde_json::from_str(r.definition_json)
                .expect("built-in recipe JSON must be valid"),
        })
        .collect();

    let envelope = CatalogEnvelope {
        version: bnto_core::FORMAT_VERSION.to_string(),
        node_types: build_catalog_node_types(&catalog),
        processors: catalog,
        definition_schema: bnto_core::definition_json_schema(),
        recipes,
    };

    serde_json::to_string_pretty(&envelope)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize catalog: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build recipe entries from engine's built-in recipes (shared by tests).
    fn build_test_recipes() -> Vec<RecipeEntry> {
        bnto_engine::recipes::builtin_recipes()
            .into_iter()
            .map(|r| RecipeEntry {
                slug: r.slug,
                name: r.name,
                description: r.description,
                category: r.category,
                tags: r.tags,
                definition: serde_json::from_str(r.definition_json)
                    .expect("built-in recipe JSON must be valid"),
            })
            .collect()
    }

    fn build_test_envelope() -> CatalogEnvelope {
        let registry = bnto_engine::create_registry();
        let mut catalog = registry.catalog();
        catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        CatalogEnvelope {
            version: bnto_core::FORMAT_VERSION.to_string(),
            node_types: build_catalog_node_types(&catalog),
            processors: catalog,
            definition_schema: bnto_core::definition_json_schema(),
            recipes: build_test_recipes(),
        }
    }

    #[test]
    fn test_catalog_envelope_has_correct_version() {
        let envelope = build_test_envelope();
        assert_eq!(envelope.version, bnto_core::FORMAT_VERSION);
    }

    #[test]
    fn test_catalog_has_all_thirteen_processors() {
        // The native registry has 13 processors (12 browser + video-download).
        let registry = bnto_engine::create_registry();
        let catalog = registry.catalog();

        assert_eq!(
            catalog.len(),
            13,
            "Catalog should have exactly 13 processors"
        );
    }

    #[test]
    fn test_catalog_contains_expected_node_types() {
        // Verify all 13 expected processor type keys are present.
        let registry = bnto_engine::create_registry();
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
            "image-overlay",
            "vector-rasterize",
            "vector-optimize",
            "video-download",
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
    fn test_browser_processors_support_browser_platform() {
        // All processors except video-download should include "browser".
        // video-download is server/CLI-only (needs yt-dlp + filesystem).
        let registry = bnto_engine::create_registry();
        let catalog = registry.catalog();

        let non_browser = ["video-download"];

        for entry in &catalog {
            if non_browser.contains(&entry.node_type.as_str()) {
                assert!(
                    !entry.platforms.contains(&"browser".to_string()),
                    "{} should NOT include 'browser' platform",
                    entry.node_type
                );
            } else {
                assert!(
                    entry.platforms.contains(&"browser".to_string()),
                    "{} should include 'browser' platform",
                    entry.node_type
                );
            }
        }
    }

    #[test]
    fn test_catalog_serializes_to_valid_json() {
        let envelope = build_test_envelope();

        let json = serde_json::to_string_pretty(&envelope).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify top-level structure.
        assert!(parsed["version"].is_string());
        assert!(parsed["nodeTypes"].is_array());
        assert_eq!(parsed["nodeTypes"].as_array().unwrap().len(), 22);
        assert!(parsed["processors"].is_array());
        assert_eq!(parsed["processors"].as_array().unwrap().len(), 13);
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
        // Verify recipes section.
        assert!(parsed["recipes"].is_array());
        assert_eq!(
            parsed["recipes"].as_array().unwrap().len(),
            18,
            "Catalog should include all 18 built-in recipes"
        );
    }

    #[test]
    fn test_catalog_node_types_include_params_for_io_and_containers() {
        // The 7 non-processor engine-defined node types must carry their
        // parameter definitions directly on each `nodeTypes` entry — that's
        // the point of PR 2 (engine as the single source of truth).
        let envelope = build_test_envelope();
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let node_types = parsed["nodeTypes"].as_array().unwrap();
        let expected_param_counts = [
            ("input", 8),
            ("output", 5),
            ("loop", 5),
            ("group", 1),
            ("parallel", 3),
            ("transform", 2),
            ("edit-fields", 2),
        ];

        for (name, expected_count) in expected_param_counts {
            let entry = node_types
                .iter()
                .find(|e| e["name"] == name)
                .unwrap_or_else(|| panic!("node type `{}` missing from catalog", name));
            let params = entry["params"]
                .as_array()
                .unwrap_or_else(|| panic!("node type `{}` must have params array", name));
            assert_eq!(
                params.len(),
                expected_count,
                "`{}` should have {} params, got {}",
                name,
                expected_count,
                params.len()
            );
        }
    }

    #[test]
    fn test_catalog_node_types_include_params_for_processors() {
        // Processor node types (13 of them) should also carry their params
        // on the `nodeTypes` entry — looked up from registry metadata.
        let envelope = build_test_envelope();
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let node_types = parsed["nodeTypes"].as_array().unwrap();
        // At least one known processor type should have non-empty params.
        let compress = node_types
            .iter()
            .find(|e| e["name"] == "image-compress")
            .expect("image-compress must be in catalog");
        let params = compress["params"].as_array();
        assert!(
            params.is_some() && !params.unwrap().is_empty(),
            "image-compress should have params inlined on its nodeTypes entry"
        );
    }

    #[test]
    fn test_catalog_input_mode_has_option_labels() {
        // Spot-check that OptionEntry labels reach the catalog JSON for the
        // input node type — proves the IO/container params round-trip.
        let envelope = build_test_envelope();
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let input = parsed["nodeTypes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["name"] == "input")
            .expect("input type must be in catalog");
        let mode = input["params"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["name"] == "mode")
            .expect("input.mode param must exist");
        let options = mode["paramType"]["options"].as_array().unwrap();
        let labels: Vec<&str> = options
            .iter()
            .map(|o| o["label"].as_str().unwrap())
            .collect();
        assert_eq!(labels, vec!["File Upload", "Text", "URL"]);
    }

    /// Generate the catalog snapshot file at `engine/catalog.snapshot.json`.
    ///
    /// Run with: `cargo test --package bnto-wasm generate_catalog_snapshot -- --nocapture`
    /// Or via: `task wasm:snapshot`
    ///
    /// Uses the native registry so ALL processors (including CLI-only ones
    /// like video-download) appear in the snapshot. TypeScript codegen gets
    /// the full schema/params even for processors that can't run in-browser
    /// yet — ready for when they can.
    ///
    /// This is an "ignored" test — it only runs when explicitly requested.
    /// It writes to a file, which isn't something normal tests should do.
    #[test]
    #[ignore]
    fn generate_catalog_snapshot() {
        let envelope = build_test_envelope();
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
        let registry = bnto_engine::create_registry();
        let mut catalog1 = registry.catalog();
        catalog1.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        let registry2 = bnto_engine::create_registry();
        let mut catalog2 = registry2.catalog();
        catalog2.sort_by(|a, b| a.node_type.cmp(&b.node_type));

        // Both should produce the same node type keys in the same order.
        let keys1: Vec<&str> = catalog1.iter().map(|m| m.node_type.as_str()).collect();
        let keys2: Vec<&str> = catalog2.iter().map(|m| m.node_type.as_str()).collect();

        assert_eq!(keys1, keys2);
    }
}
