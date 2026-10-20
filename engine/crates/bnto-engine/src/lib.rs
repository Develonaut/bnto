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
    registry.register("report", Box::new(bnto_core::control::Report::new()));
    registry.register("edit-fields", Box::new(bnto_core::data::EditFields::new()));
    registry.register("summarize", Box::new(bnto_core::data::Summarize::new()));
    registry.register("file-decode", Box::new(bnto_file::FileDecode::new()));
    registry.register("file-filter", Box::new(bnto_file::FileFilter::new()));
    registry.register("file-inspect", Box::new(bnto_file::FileInspect::new()));
    registry.register("file-join", Box::new(bnto_file::FileJoin::new()));
    registry.register("file-rename", Box::new(bnto_file::RenameFiles::new()));
    registry.register("file-render", Box::new(bnto_file::FileRender::new()));
    registry.register("file-sort", Box::new(bnto_file::FileSort::new()));
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
    registry.register("image-pad", Box::new(bnto_image::PadImages::new()));
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
        registry.register("file-delete", Box::new(bnto_file::FileDelete::new()));
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
        assert_eq!(registry.len(), 24);

        let expected = [
            "assert",
            "report",
            "edit-fields",
            "summarize",
            "file-decode",
            "file-filter",
            "file-inspect",
            "file-join",
            "file-rename",
            "file-render",
            "file-sort",
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "image-overlay",
            "image-pad",
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
        // Full registry = browser (24) + file-collect + file-copy + file-delete + file-list
        // + file-move + spreadsheet-export + spreadsheet-lookup + shell-command (8) = 32
        assert_eq!(registry.len(), 32);
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
        let findings = bnto_core::check_pipeline(&definition, &create_registry()).findings;
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].node_id, "only-docs");
        assert!(findings[0].message.contains("name_pattern"));
    }

    #[test]
    fn test_check_refuses_a_file_sort_key_with_no_field() {
        let definition: PipelineDefinition = serde_json::from_value(serde_json::json!({
            "nodes": [{ "id": "order", "type": "file-sort", "parameters": {
                "by": "source, -"
            } }]
        }))
        .unwrap();
        let findings = bnto_core::check_pipeline(&definition, &create_registry()).findings;
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].node_id, "order");
        assert!(findings[0].message.contains("empty sort key"));
    }

    #[test]
    #[cfg(feature = "native")]
    fn test_every_builtin_recipe_checks_clean() {
        let registry = create_registry();
        for recipe in recipes::builtin_recipes() {
            let definition: PipelineDefinition =
                serde_json::from_str(recipe.definition_json).unwrap();
            let findings = bnto_core::check_pipeline(&definition, &registry).findings;
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
        // file-inspect attaches `extension`; file-rename reads it per file.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "meta", "type": "file-inspect" },
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

        // Metadata accumulates: file-inspect's keys survive the rename,
        // alongside what file-rename attached.
        let meta = &result.files[0].metadata;
        assert_eq!(meta["extension"], "txt");
        assert_eq!(meta["newFilename"], "txt-notes.txt");
    }

    #[test]
    fn test_a_later_node_reads_a_json_elements_nested_field_through_item() {
        // spreadsheet-read flattens `{"meta": {"id": …}}` to `meta.id`, so
        // file-rename's per-file template reads it as `{{item.meta.id}}`.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "read", "type": "spreadsheet-read" },
                { "id": "rename", "type": "file-rename",
                  "parameters": { "pattern": "{{item.meta.id}}-{{item.name}}.txt" } },
                { "id": "output", "type": "output" }
            ]
        }"#;
        let items = br#"[{"meta": {"id": "x"}, "name": "a"}, {"meta": {"id": "y"}, "name": "b"}]"#;
        let file = PipelineFile {
            name: "items.json".to_string(),
            data: FileData::Bytes(items.to_vec()),
            mime_type: "application/json".to_string(),
            metadata: serde_json::Map::new(),
        };

        let reporter = PipelineReporter::new_noop();
        let result = run_pipeline(json, vec![file], &reporter, &NoopContext)
            .expect("pipeline should succeed");

        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["x-a.txt", "y-b.txt"]);
        assert_eq!(result.files[0].metadata["meta.id"], "x");
    }

    #[test]
    fn test_a_later_node_reads_the_modified_time_file_metadata_attached() {
        // `{{item.modified}}` and `{{ctx.started}}` share one format.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "meta", "type": "file-inspect" },
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
    #[cfg(feature = "native")]
    fn test_file_metadata_gives_each_listed_item_its_folder() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("a").join("b")).unwrap();
        let names = ["top.txt", "a/mid.txt", "a/b/deep.txt"];
        for name in names {
            std::fs::write(tmp.path().join(name), name).unwrap();
        }
        let json = serde_json::json!({
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input", "parameters": { "mode": "none" } },
                { "id": "list", "type": "file-list",
                  "parameters": { "base": tmp.path().to_str().unwrap(), "paths": names } },
                { "id": "meta", "type": "file-inspect" },
                { "id": "output", "type": "output", "parameters": { "mode": "none" } }
            ]
        })
        .to_string();

        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(&json, vec![], &reporter, &NoopContext).expect("pipeline should succeed");

        let folders: Vec<(&str, &serde_json::Value)> = result
            .files
            .iter()
            .map(|f| (f.name.as_str(), &f.metadata["folder"]))
            .collect();
        assert_eq!(
            folders,
            vec![
                ("top.txt", &serde_json::json!("")),
                ("a/mid.txt", &serde_json::json!("a")),
                ("a/b/deep.txt", &serde_json::json!("a/b")),
            ]
        );
    }

    /// A CSV whose rows are people, read into one item per row.
    fn people_csv() -> PipelineFile {
        PipelineFile {
            name: "people.csv".to_string(),
            data: FileData::Bytes(b"first,last\nAda,Lovelace\nGrace,Hopper\n".to_vec()),
            mime_type: "text/csv".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    /// Run read rows -> `edit` (an edit-fields node's JSON, or a loop around
    /// one) -> rename each row file to `{{item.label}}.txt`.
    fn run_edit_and_rename(edit: &str) -> Result<PipelineResult, BntoError> {
        let json = format!(
            r#"{{
            "settings": {{ "iteration": "auto" }},
            "nodes": [
                {{ "id": "input", "type": "input" }},
                {{ "id": "rows", "type": "spreadsheet-read" }},
                {edit},
                {{ "id": "rename", "type": "file-rename",
                  "parameters": {{ "pattern": "{{{{item.label}}}}.txt" }} }},
                {{ "id": "output", "type": "output" }}
            ]
        }}"#
        );
        let reporter = PipelineReporter::new_noop();
        run_pipeline(&json, vec![people_csv()], &reporter, &NoopContext)
    }

    const EDIT_LABEL: &str = r#"{ "id": "edit", "type": "edit-fields",
        "parameters": { "values": { "label": "{{item.first}}-{{item.last}}" } } }"#;

    #[test]
    fn test_a_later_node_reads_a_field_edit_fields_set() {
        let result = run_edit_and_rename(EDIT_LABEL).expect("pipeline should succeed");
        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["Ada-Lovelace.txt", "Grace-Hopper.txt"]);
        assert_eq!(result.files[0].metadata["label"], "Ada-Lovelace");
        assert_eq!(result.files[0].metadata["first"], "Ada");
    }

    #[test]
    fn test_edit_fields_inside_a_loop_sets_each_items_field() {
        let looped = format!(r#"{{ "id": "each", "type": "loop", "children": [{EDIT_LABEL}] }}"#);
        let result = run_edit_and_rename(&looped).expect("pipeline should succeed");
        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["Ada-Lovelace.txt", "Grace-Hopper.txt"]);
    }

    #[test]
    fn test_an_unknown_field_in_edit_fields_fails_the_run_naming_it() {
        let edit = r#"{ "id": "edit", "type": "edit-fields",
            "parameters": { "values": { "label": "{{item.first}} {{item.middle}}" } } }"#;
        let Err(err) = run_edit_and_rename(edit) else {
            panic!("an unknown field should fail the run");
        };
        let msg = err.to_string();
        assert!(msg.contains("\"middle\""), "{msg}");
        assert!(msg.contains("people_row"), "{msg}");
    }

    /// A text file named `name` holding `size` bytes.
    fn sized_file(name: &str, size: usize) -> PipelineFile {
        PipelineFile {
            name: name.to_string(),
            data: FileData::Bytes(vec![b'x'; size]),
            mime_type: "text/plain".to_string(),
            metadata: serde_json::Map::new(),
        }
    }

    #[test]
    fn test_summarize_counts_and_totals_what_file_metadata_attached() {
        // file-inspect gives each file `extension` and `file_size`;
        // summarize turns five files into one item per extension.
        let json = r#"{
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input" },
                { "id": "meta", "type": "file-inspect" },
                { "id": "totals", "type": "summarize",
                  "parameters": { "groupBy": "extension", "sum": "file_size" } },
                { "id": "output", "type": "output" }
            ]
        }"#;
        let files = vec![
            sized_file("a.md", 10),
            sized_file("b.png", 300),
            sized_file("c.md", 20),
            sized_file("d.png", 400),
            sized_file("e.md", 30),
        ];
        let reporter = PipelineReporter::new_noop();
        let result =
            run_pipeline(json, files, &reporter, &NoopContext).expect("pipeline should succeed");

        let rows: Vec<serde_json::Value> = result
            .files
            .iter()
            .map(|f| serde_json::Value::Object(f.metadata.clone()))
            .collect();
        assert_eq!(
            rows,
            vec![
                serde_json::json!({ "extension": "md", "count": 3, "sum_file_size": 60 }),
                serde_json::json!({ "extension": "png", "count": 2, "sum_file_size": 700 }),
            ]
        );
    }

    #[test]
    fn test_check_refuses_a_summarize_field_list_with_an_empty_field() {
        let definition: PipelineDefinition = serde_json::from_value(serde_json::json!({
            "nodes": [{ "id": "totals", "type": "summarize", "parameters": {
                "groupBy": "folder,,kind"
            } }]
        }))
        .unwrap();
        let findings = bnto_core::check_pipeline(&definition, &create_registry()).findings;
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].node_id, "totals");
        assert!(findings[0].message.contains("empty field"), "{findings:?}");
    }

    /// Run file-list (`paths` under `root`) -> file-filter (`.log` only) ->
    /// file-delete (under `root`), with no input files.
    #[cfg(feature = "native")]
    fn run_list_filter_delete(root: &std::path::Path, paths: &[&str]) -> PipelineResult {
        let root = root.to_str().unwrap();
        let json = serde_json::json!({
            "settings": { "iteration": "auto" },
            "nodes": [
                { "id": "input", "type": "input", "parameters": { "mode": "none" } },
                { "id": "list", "type": "file-list",
                  "parameters": { "base": root, "paths": paths } },
                { "id": "logs", "type": "file-filter", "parameters": { "extensions": "log" } },
                { "id": "delete", "type": "file-delete", "parameters": { "base": root } },
                { "id": "output", "type": "output", "parameters": { "mode": "none" } }
            ]
        })
        .to_string();
        let reporter = PipelineReporter::new_noop();
        run_pipeline(&json, vec![], &reporter, &NoopContext).expect("pipeline should succeed")
    }

    #[test]
    #[cfg(feature = "native")]
    fn test_file_delete_removes_what_list_and_filter_chose() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("logs")).unwrap();
        for name in ["a.log", "keep.txt", "logs/b.log"] {
            std::fs::write(tmp.path().join(name), name).unwrap();
        }
        let result =
            run_list_filter_delete(tmp.path(), &["a.log", "keep.txt", "logs/b.log", "gone.log"]);
        assert!(!tmp.path().join("a.log").exists());
        assert!(!tmp.path().join("logs/b.log").exists());
        assert!(
            tmp.path().join("keep.txt").exists(),
            "the filter dropped it"
        );
        assert!(tmp.path().join("logs").is_dir());
        let counts = &result.node_metadata["delete"];
        assert_eq!(counts["deleted"], 2, "{counts:?}");
        assert_eq!(counts["skipped"], 0, "file-list already skipped gone.log");
        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["a.log", "logs/b.log"]);
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
            "file-decode",
            "file-filter",
            "file-inspect",
            "file-join",
            "file-rename",
            "file-sort",
            "image-compress",
            "image-resize",
            "image-convert",
            "image-strip-exif",
            "image-overlay",
            "image-pad",
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
