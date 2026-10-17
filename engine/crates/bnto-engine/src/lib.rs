// bnto-engine — shared engine layer for CLI + WASM consumers.
//
// Provides the processor registry and a convenience `run_pipeline()`
// so consumers don't duplicate registration logic. Both bnto-wasm (browser)
// and bnto (native CLI binary) depend on this crate.

pub mod deps;
pub mod recipes;

use bnto_core::{
    BntoError, NodeRegistry, PipelineDefinition, PipelineFile, PipelineReporter, PipelineResult,
    ProcessContext, execute_pipeline,
};

/// Create a registry with only browser-capable (WASM-safe) processors.
///
/// Used by the `node_catalog()` WASM export and browser execution path.
/// For the full registry (including CLI-only processors), use `create_registry()`.
pub fn create_browser_registry() -> NodeRegistry {
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
    registry.register(
        "spreadsheet-clean",
        Box::new(bnto_spreadsheet::CleanSpreadsheet::new()),
    );
    registry.register(
        "spreadsheet-rename",
        Box::new(bnto_spreadsheet::RenameColumns::new()),
    );
    registry.register("assert", Box::new(bnto_core::control::Assert::new()));
    registry.register("file-filter", Box::new(bnto_file::FileFilter::new()));
    registry.register("file-metadata", Box::new(bnto_file::FileMetadata::new()));
    registry.register("file-rename", Box::new(bnto_file::RenameFiles::new()));
    registry.register("image-strip-exif", Box::new(bnto_image::StripExif::new()));
    registry.register(
        "spreadsheet-convert",
        Box::new(bnto_spreadsheet::ConvertFormat::new()),
    );
    registry.register(
        "spreadsheet-merge",
        Box::new(bnto_spreadsheet::MergeSpreadsheets::new()),
    );
    registry.register("image-overlay", Box::new(bnto_image::OverlayImage::new()));
    registry.register(
        "vector-rasterize",
        Box::new(bnto_vector::VectorRasterize::new()),
    );
    registry.register("vector-optimize", Box::new(bnto_vector::OptimizeSvg));
    registry.register(
        "spreadsheet-read",
        Box::new(bnto_spreadsheet::ReadSpreadsheet::new()),
    );

    registry
}

/// Create the full processor registry — all node types across all targets.
///
/// Starts from the browser-safe registry and adds CLI/server/desktop
/// processors (like shell-command) when compiled with the `native` feature.
/// This is the canonical registry — CLI, tests, and codegen use this.
pub fn create_registry() -> NodeRegistry {
    #[allow(unused_mut)]
    let mut registry = create_browser_registry();

    #[cfg(feature = "native")]
    {
        registry.register("file-collect", Box::new(bnto_file::FileCollect::new()));
        registry.register("file-copy", Box::new(bnto_file::FileCopy::new()));
        registry.register("file-list", Box::new(bnto_file::FileList::new()));
        registry.register("file-move", Box::new(bnto_file::FileMove::new()));
        registry.register(
            "spreadsheet-export",
            Box::new(bnto_spreadsheet::SpreadsheetExport::new()),
        );
        registry.register(
            "spreadsheet-lookup",
            Box::new(bnto_spreadsheet::SpreadsheetLookup::new()),
        );
        registry.register("shell-command", Box::new(bnto_shell::ShellCommand::new()));
    }

    registry
}

/// Run a pipeline from a JSON definition string and a list of files.
///
/// Convenience wrapper that parses JSON, creates the full registry,
/// and executes the pipeline. Suited for CLI and integration tests
/// where the full WASM bridge isn't needed.
pub fn run_pipeline(
    definition_json: &str,
    files: Vec<PipelineFile>,
    reporter: &PipelineReporter,
    ctx: &dyn ProcessContext,
) -> Result<PipelineResult, BntoError> {
    let definition: PipelineDefinition = serde_json::from_str(definition_json)
        .map_err(|e| BntoError::InvalidInput(format!("Failed to parse definition: {e}")))?;

    let registry = create_registry();

    // Pre-flight: fail fast if required external tools are missing.
    deps::check_pipeline_dependencies(&definition, &registry, ctx)?;

    // Pre-flight: fail fast if required secrets (env vars) are missing.
    deps::check_pipeline_secrets(&definition, ctx)?;

    let now_ms = || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    };

    execute_pipeline(&definition, files, &registry, reporter, ctx, now_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;
    use bnto_core::processor::FileData;

    #[test]
    fn test_browser_registry_has_all_processors() {
        let registry = create_browser_registry();
        assert_eq!(registry.len(), 16);

        let expected = [
            "assert",
            "file-filter",
            "file-metadata",
            "file-rename",
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "image-overlay",
            "spreadsheet-clean",
            "spreadsheet-read",
            "spreadsheet-rename",
            "spreadsheet-convert",
            "spreadsheet-merge",
            "vector-rasterize",
            "vector-optimize",
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
    #[cfg(feature = "native")]
    fn test_full_registry_has_native_processors() {
        let registry = create_registry();
        // Full registry = browser (16) + file-collect + file-copy + file-list
        // + file-move + spreadsheet-export + spreadsheet-lookup + shell-command (7) = 23
        assert_eq!(registry.len(), 23);
        let params = serde_json::Map::new();
        assert!(
            registry.resolve("shell-command", &params).is_some(),
            "Native registry should include shell-command",
        );
    }

    #[test]
    fn test_check_refuses_a_file_filter_regex_that_does_not_compile() {
        let definition: PipelineDefinition = serde_json::from_value(serde_json::json!({
            "nodes": [{ "id": "only-docs", "type": "file-filter", "parameters": {
                "name_pattern": "(", "pattern_mode": "regex"
            } }]
        }))
        .unwrap();
        let findings = bnto_core::check_pipeline(&definition, &create_registry());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].node_id, "only-docs");
        assert!(findings[0].message.contains("name_pattern"));
    }

    #[test]
    #[cfg(feature = "native")]
    fn test_every_builtin_recipe_checks_clean() {
        let registry = create_registry();
        for recipe in recipes::builtin_recipes() {
            let definition: PipelineDefinition =
                serde_json::from_str(recipe.definition_json).unwrap();
            let findings = bnto_core::check_pipeline(&definition, &registry);
            assert!(findings.is_empty(), "{}: {findings:?}", recipe.slug);
        }
    }

    #[test]
    fn test_run_pipeline_rejects_invalid_json() {
        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline("not valid json", vec![], &reporter, &NoopContext);
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
            data: FileData::Bytes(test_image.to_vec()),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("Pipeline should succeed");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty().expect("should read length"));
    }

    #[test]
    fn test_generated_compress_images_recipe() {
        let json = include_str!("../recipes/compress-images.bnto.json");
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: FileData::Bytes(test_image.to_vec()),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("compress-images recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty().expect("should read length"));
    }

    #[test]
    fn test_generated_resize_images_recipe() {
        let json = include_str!("../recipes/resize-images.bnto.json");
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: FileData::Bytes(test_image.to_vec()),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("resize-images recipe");

        assert_eq!(result.files.len(), 1);
    }

    #[test]
    fn test_generated_clean_csv_recipe() {
        let json = include_str!("../recipes/clean-csv.bnto.json");
        let csv_data = include_bytes!("../../../../test-fixtures/csv/messy.csv");
        let files = vec![PipelineFile {
            name: "data.csv".to_string(),
            data: FileData::Bytes(csv_data.to_vec()),
            mime_type: "text/csv".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter, &NoopContext).expect("clean-csv recipe");

        assert_eq!(result.files.len(), 1);
    }

    #[test]
    fn test_flat_recipe_renames_from_metadata_an_earlier_node_attached() {
        // file-metadata attaches `extension`; file-rename reads it per file.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "meta", "type": "file-metadata" },
                { "id": "rename", "type": "file-rename",
                  "parameters": { "pattern": "{{item.extension}}-{{name}}.{{ext}}" } },
                { "id": "output", "type": "output" }
            ]
        }"#;
        let file = |name: &str| PipelineFile {
            name: name.to_string(),
            data: FileData::Bytes(b"x".to_vec()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        };

        let reporter = PipelineReporter::new_noop();
        let files = vec![file("notes.txt"), file("data.csv")];
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("pipeline should succeed");

        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["txt-notes.txt", "csv-data.csv"]);

        // Metadata accumulates: file-metadata's keys survive the rename,
        // alongside what file-rename attached.
        let meta = &result.files[0].metadata;
        assert_eq!(meta["extension"], "txt");
        assert_eq!(meta["newFilename"], "txt-notes.txt");
    }

    #[test]
    fn test_a_later_node_reads_the_modified_time_file_metadata_attached() {
        // `{{item.modified}}` and `{{ctx.started}}` share one format.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "meta", "type": "file-metadata" },
                { "id": "rename", "type": "file-rename",
                  "parameters": { "pattern": "{{item.modified}} {{ctx.started}}" } },
                { "id": "output", "type": "output" }
            ]
        }"#;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("a.txt");
        std::fs::write(&path, b"x").unwrap();
        let mtime = filetime::FileTime::from_unix_time(1_700_000_000, 123_000_000);
        filetime::set_file_mtime(&path, mtime).unwrap();
        let file = PipelineFile {
            name: "a.txt".to_string(),
            data: FileData::Path(path),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        };

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, vec![file], &reporter, &NoopContext)
            .expect("pipeline should succeed");

        let name = &result.files[0].name;
        let (modified, started) = name.split_once(' ').expect("two times");
        assert_eq!(modified, "2023-11-14T22:13:20.123Z");
        assert_eq!(result.files[0].metadata["modified"], modified);
        let shape = |s: &str| s.chars().map(|c| c.is_ascii_digit()).collect::<Vec<_>>();
        assert_eq!(shape(modified), shape(started), "{name}");
        assert!(
            modified < started,
            "an old file sorts before this run's start"
        );
    }

    #[test]
    fn test_generated_rename_files_recipe() {
        let json = include_str!("../recipes/rename-files.bnto.json");
        let files = vec![PipelineFile {
            name: "document.txt".to_string(),
            data: FileData::Bytes(b"hello world".to_vec()),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("rename-files recipe");

        assert_eq!(result.files.len(), 1);
        assert!(
            result.files[0].name.starts_with("renamed-"),
            "Expected 'renamed-' prefix, got: {}",
            result.files[0].name,
        );
    }

    #[test]
    fn test_generated_csv_to_json_recipe() {
        let json = include_str!("../recipes/csv-to-json.bnto.json");
        let csv_data = include_bytes!("../../../../test-fixtures/csv/simple.csv");
        let files = vec![PipelineFile {
            name: "data.csv".to_string(),
            data: FileData::Bytes(csv_data.to_vec()),
            mime_type: "text/csv".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("csv-to-json recipe");

        assert_eq!(result.files.len(), 1);
        assert!(result.files[0].name.ends_with(".json"));
    }

    #[test]
    fn test_generated_merge_csv_recipe() {
        let json = include_str!("../recipes/merge-csv.bnto.json");
        let csv_a = b"name,age\nAlice,30\n";
        let csv_b = b"name,age\nBob,25\n";
        let files = vec![
            PipelineFile {
                name: "a.csv".to_string(),
                data: FileData::Bytes(csv_a.to_vec()),
                mime_type: "text/csv".to_string(),
                metadata: serde_json::Map::new(),
            },
            PipelineFile {
                name: "b.csv".to_string(),
                data: FileData::Bytes(csv_b.to_vec()),
                mime_type: "text/csv".to_string(),
                metadata: serde_json::Map::new(),
            },
        ];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter, &NoopContext).expect("merge-csv recipe");

        assert_eq!(result.files.len(), 1);
        let bytes = result.files[0]
            .data
            .clone()
            .into_bytes()
            .expect("should have bytes");
        let output = String::from_utf8_lossy(&bytes);
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn test_generated_strip_exif_recipe() {
        let json = include_str!("../recipes/strip-exif.bnto.json");
        let test_image = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let files = vec![PipelineFile {
            name: "photo.jpg".to_string(),
            data: FileData::Bytes(test_image.to_vec()),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter, &NoopContext).expect("strip-exif recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty().expect("should read length"));
    }

    #[test]
    fn test_generated_watermark_images_recipe() {
        // The generated recipe has an empty overlay param — inject a real one.
        let mut def: serde_json::Value =
            serde_json::from_str(include_str!("../recipes/watermark-images.bnto.json")).unwrap();

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
            data: FileData::Bytes(test_image.to_vec()),
            mime_type: "image/jpeg".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(&json, files, &reporter, &NoopContext).expect("watermark-images recipe");

        assert_eq!(result.files.len(), 1);
        assert!(!result.files[0].data.is_empty().expect("should read length"));
    }

    #[test]
    fn test_generated_optimize_svg_recipe() {
        let json = include_str!("../recipes/optimize-svg.bnto.json");
        let test_svg = include_bytes!("../../../../test-fixtures/vector/verbose.svg");
        let files = vec![PipelineFile {
            name: "icon.svg".to_string(),
            data: FileData::Bytes(test_svg.to_vec()),
            mime_type: "image/svg+xml".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("optimize-svg recipe");

        assert_eq!(result.files.len(), 1);
        let bytes = result.files[0]
            .data
            .clone()
            .into_bytes()
            .expect("should have bytes");
        // Output should be smaller than input (editor cruft removed)
        assert!(bytes.len() < test_svg.len());
        // Output should still be valid SVG (starts with <svg)
        let output = String::from_utf8_lossy(&bytes);
        assert!(
            output.contains("<svg"),
            "Output should contain <svg element"
        );
    }

    #[test]
    fn test_all_processor_names_match_registry_keys() {
        // Convention: name() must return the same string used in registry.register().
        // Pattern: category-operation (e.g., "image-compress", not "compress-images").
        let registry = create_browser_registry();
        let expected_keys = [
            "file-filter",
            "file-metadata",
            "file-rename",
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "image-overlay",
            "spreadsheet-clean",
            "spreadsheet-read",
            "spreadsheet-rename",
            "spreadsheet-convert",
            "spreadsheet-merge",
            "vector-rasterize",
            "vector-optimize",
        ];
        let params = serde_json::Map::new();
        for key in &expected_keys {
            let processor = registry
                .resolve(key, &params)
                .unwrap_or_else(|| panic!("Missing processor for {key}"));
            assert_eq!(
                processor.name(),
                *key,
                "processor.name() must match registry key. Got '{}' for key '{}'",
                processor.name(),
                key
            );
        }
    }

    #[test]
    fn test_generated_svg_to_png_recipe() {
        let json = include_str!("../recipes/svg-to-png.bnto.json");
        let test_svg = include_bytes!("../../../../test-fixtures/images/small.svg");
        let files = vec![PipelineFile {
            name: "icon.svg".to_string(),
            data: FileData::Bytes(test_svg.to_vec()),
            mime_type: "image/svg+xml".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, files, &reporter, &NoopContext).expect("svg-to-png recipe");

        assert_eq!(result.files.len(), 1);
        assert!(result.files[0].name.ends_with(".png"));
        // PNG magic bytes
        let bytes = result.files[0]
            .data
            .clone()
            .into_bytes()
            .expect("should have bytes");
        assert!(bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]));
    }

    #[test]
    fn test_generated_svg_to_jpeg_recipe() {
        let json = include_str!("../recipes/svg-to-jpeg.bnto.json");
        let test_svg = include_bytes!("../../../../test-fixtures/images/small.svg");
        let files = vec![PipelineFile {
            name: "icon.svg".to_string(),
            data: FileData::Bytes(test_svg.to_vec()),
            mime_type: "image/svg+xml".to_string(),
            metadata: serde_json::Map::new(),
        }];

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("svg-to-jpeg recipe");

        assert_eq!(result.files.len(), 1);
        assert!(result.files[0].name.ends_with(".jpg"));
        // JPEG magic bytes
        let bytes = result.files[0]
            .data
            .clone()
            .into_bytes()
            .expect("should have bytes");
        assert!(bytes.starts_with(&[0xFF, 0xD8, 0xFF]));
    }

    #[test]
    fn test_all_generated_recipes_parse() {
        let recipes = [
            include_str!("../recipes/compress-images.bnto.json"),
            include_str!("../recipes/resize-images.bnto.json"),
            include_str!("../recipes/convert-image-format.bnto.json"),
            include_str!("../recipes/rename-files.bnto.json"),
            include_str!("../recipes/clean-csv.bnto.json"),
            include_str!("../recipes/rename-csv-columns.bnto.json"),
            include_str!("../recipes/optimize-images-for-web.bnto.json"),
            include_str!("../recipes/generate-thumbnails.bnto.json"),
            include_str!("../recipes/compress-and-rename.bnto.json"),
            include_str!("../recipes/standardize-csv.bnto.json"),
            include_str!("../recipes/csv-to-json.bnto.json"),
            include_str!("../recipes/strip-exif.bnto.json"),
            include_str!("../recipes/watermark-images.bnto.json"),
            include_str!("../recipes/merge-csv.bnto.json"),
            include_str!("../recipes/download-video.bnto.json"),
            include_str!("../recipes/svg-to-png.bnto.json"),
            include_str!("../recipes/svg-to-jpeg.bnto.json"),
            include_str!("../recipes/optimize-svg.bnto.json"),
            include_str!("../recipes/number-files.bnto.json"),
            include_str!("../recipes/sanitize-filenames.bnto.json"),
            include_str!("../recipes/flatten-folders.bnto.json"),
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
