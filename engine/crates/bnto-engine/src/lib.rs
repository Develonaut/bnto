// bnto-engine — shared engine layer for CLI + WASM consumers.
//
// Provides the default processor registry and a convenience `run_pipeline()`
// so consumers don't duplicate registration logic. Both bnto-wasm (browser)
// and bnto-cli (native binary) depend on this crate.

use bnto_core::{
    BntoError, NodeRegistry, PipelineDefinition, PipelineFile, PipelineReporter, PipelineResult,
    execute_pipeline,
};

/// Create a registry pre-loaded with all browser-capable node processors.
///
/// Maps flat node type keys (e.g., "image-compress") to Rust processor instances.
/// This is the single source of truth for which processors are available —
/// both WASM and CLI consumers call this instead of duplicating registrations.
pub fn create_default_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();

    registry.register(
        "image-compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register("image-resize", Box::new(bnto_image::ResizeImages::new()));
    registry.register(
        "image-convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );
    registry.register("spreadsheet-clean", Box::new(bnto_csv::CleanCsv::new()));
    registry.register(
        "spreadsheet-rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );
    registry.register("file-rename", Box::new(bnto_file::RenameFiles::new()));
    registry.register("image-strip-exif", Box::new(bnto_image::StripExif::new()));
    registry.register("spreadsheet-convert", Box::new(bnto_csv::CsvToJson::new()));
    registry.register("spreadsheet-merge", Box::new(bnto_csv::MergeCsv::new()));
    registry.register("image-overlay", Box::new(bnto_image::OverlayImage::new()));

    registry
}

/// Run a pipeline from a JSON definition string and a list of files.
///
/// Convenience wrapper that parses JSON, creates the default registry,
/// and executes the pipeline. Suited for CLI and integration tests
/// where the full WASM bridge isn't needed.
pub fn run_pipeline(
    definition_json: &str,
    files: Vec<PipelineFile>,
    reporter: &PipelineReporter,
) -> Result<PipelineResult, BntoError> {
    let definition: PipelineDefinition = serde_json::from_str(definition_json)
        .map_err(|e| BntoError::InvalidInput(format!("Failed to parse definition: {e}")))?;

    let registry = create_default_registry();

    let now_ms = || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    };

    execute_pipeline(&definition, files, &registry, reporter, now_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_registry_has_all_processors() {
        let registry = create_default_registry();
        assert_eq!(registry.len(), 10);

        let expected = [
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "image-overlay",
            "spreadsheet-clean",
            "spreadsheet-rename",
            "spreadsheet-convert",
            "spreadsheet-merge",
            "file-rename",
        ];

        let params = serde_json::Map::new();
        for node_type in &expected {
            assert!(
                registry.resolve(node_type, &params).is_some(),
                "Missing processor for {node_type}",
            );
        }
    }

    #[test]
    fn test_run_pipeline_rejects_invalid_json() {
        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline("not valid json", vec![], &reporter);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_pipeline_with_simple_definition() {
        let json = r#"{
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "compress", "type": "image-compress", "parameters": { "quality": 80 } },
                { "id": "output", "type": "output" }
            ]
        }"#;

        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "test.jpg".to_string(),
            data: test_image.to_vec(),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("Pipeline should succeed");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty());
    }

    #[test]
    fn test_generated_compress_images_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/compress-images.bnto.json"
        );
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: test_image.to_vec(),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("compress-images recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty());
    }

    #[test]
    fn test_generated_resize_images_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/resize-images.bnto.json"
        );
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: test_image.to_vec(),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("resize-images recipe");

        assert_eq!(result.files.len(), 1);
    }

    #[test]
    fn test_generated_clean_csv_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/clean-csv.bnto.json"
        );
        let csv_data = include_bytes!("../../../../test-fixtures/csv/messy.csv");
        let files = vec![PipelineFile {
            name: "data.csv".to_string(),
            data: csv_data.to_vec(),
            mime_type: "text/csv".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("clean-csv recipe");

        assert_eq!(result.files.len(), 1);
    }

    #[test]
    fn test_generated_rename_files_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/rename-files.bnto.json"
        );
        let files = vec![PipelineFile {
            name: "document.txt".to_string(),
            data: b"hello world".to_vec(),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("rename-files recipe");

        assert_eq!(result.files.len(), 1);
        assert!(
            result.files[0].name.starts_with("renamed-"),
            "Expected 'renamed-' prefix, got: {}",
            result.files[0].name,
        );
    }

    #[test]
    fn test_generated_csv_to_json_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/csv-to-json.bnto.json"
        );
        let csv_data = include_bytes!("../../../../test-fixtures/csv/simple.csv");
        let files = vec![PipelineFile {
            name: "data.csv".to_string(),
            data: csv_data.to_vec(),
            mime_type: "text/csv".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("csv-to-json recipe");

        assert_eq!(result.files.len(), 1);
        assert!(result.files[0].name.ends_with(".json"));
    }

    #[test]
    fn test_generated_merge_csv_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/merge-csv.bnto.json"
        );
        let csv_a = b"name,age\nAlice,30\n";
        let csv_b = b"name,age\nBob,25\n";
        let files = vec![
            PipelineFile {
                name: "a.csv".to_string(),
                data: csv_a.to_vec(),
                mime_type: "text/csv".to_string(),
                metadata: serde_json::Map::new(),
            },
            PipelineFile {
                name: "b.csv".to_string(),
                data: csv_b.to_vec(),
                mime_type: "text/csv".to_string(),
                metadata: serde_json::Map::new(),
            },
        ];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("merge-csv recipe");

        assert_eq!(result.files.len(), 1);
        let output = String::from_utf8_lossy(&result.files[0].data);
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn test_generated_strip_exif_recipe() {
        let json = include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/strip-exif.bnto.json"
        );
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: test_image.to_vec(),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter).expect("strip-exif recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty());
    }

    #[test]
    fn test_generated_watermark_images_recipe() {
        // The generated recipe has an empty overlay param — inject a real one.
        let mut def: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../packages/@bnto/registry/src/recipes/generated/watermark-images.bnto.json"
        ))
        .unwrap();

        // Base64-encode the test overlay image
        let overlay_bytes =
            include_bytes!("../../../../test-fixtures/images/overlays/overlay-logo.png");
        let overlay_b64 = format!(
            "data:image/png;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, overlay_bytes)
        );

        // Set the overlay param on the overlay node
        let nodes = def["nodes"].as_array_mut().unwrap();
        for node in nodes.iter_mut() {
            if node["type"] == "image-overlay" {
                node["parameters"]["overlay"] = serde_json::Value::String(overlay_b64.clone());
            }
        }

        let json = serde_json::to_string(&def).unwrap();
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: test_image.to_vec(),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(&json, files, &reporter).expect("watermark-images recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty());
    }

    #[test]
    fn test_all_generated_recipes_parse() {
        let recipes = [
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/compress-images.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/resize-images.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/convert-image-format.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/rename-files.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/clean-csv.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/rename-csv-columns.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/optimize-images-for-web.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/generate-thumbnails.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/compress-and-rename.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/standardize-csv.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/csv-to-json.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/strip-exif.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/watermark-images.bnto.json"
            ),
            include_str!(
                "../../../../packages/@bnto/registry/src/recipes/generated/merge-csv.bnto.json"
            ),
        ];

        for (i, json) in recipes.iter().enumerate() {
            let def: PipelineDefinition = serde_json::from_str(json)
                .unwrap_or_else(|e| panic!("Recipe {i} failed to parse: {e}"));
            assert!(
                !def.nodes.is_empty(),
                "Recipe {i} should have at least one node",
            );
        }
    }
}
