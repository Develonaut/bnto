// Recipe Integration Tests -- full pipeline with real processors.
//
// Feeds real recipe JSON through the Rust pipeline executor with real
// processors and real file data. If these pass, any layer built on top
// (browser WASM, desktop Tauri, CLI) is guaranteed to work.
//
// Native Rust tests (cargo test), not WASM -- same code path as the
// WASM bridge but faster and more reliable without a JS runtime.

use bnto_core::{PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline};

// Test fixture data embedded at compile time.
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static SMALL_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");
static SIMPLE_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/simple.csv");

fn real_registry() -> bnto_core::NodeRegistry {
    let mut registry = bnto_core::NodeRegistry::new();
    registry.register(
        "image:compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register("image:resize", Box::new(bnto_image::ResizeImages::new()));
    registry.register(
        "image:convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );
    registry.register("spreadsheet:clean", Box::new(bnto_csv::CleanCsv::new()));
    registry.register(
        "spreadsheet:rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );
    registry.register(
        "file-system:rename",
        Box::new(bnto_file::RenameFiles::new()),
    );
    registry
}

fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }
}

fn fake_now() -> u64 {
    1000
}

// =========================================================================
// Compress Images
// =========================================================================

#[test]
fn compress_images_recipe_produces_smaller_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 50 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let input_size = SMALL_JPEG.len();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("compress pipeline should succeed");

    assert_eq!(result.files.len(), 1, "should output 1 file");
    assert!(result.files[0].data.len() >= 2);
    assert_eq!(
        &result.files[0].data[0..2],
        &[0xFF, 0xD8],
        "output should be valid JPEG"
    );

    assert!(
        result.files[0].data.len() < input_size,
        "compressed JPEG at q=50 ({} bytes) should be smaller than input ({} bytes)",
        result.files[0].data.len(),
        input_size
    );
}

#[test]
fn compress_images_recipe_handles_batch() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![
        file("a.jpg", SMALL_JPEG, "image/jpeg"),
        file("b.png", SMALL_PNG, "image/png"),
    ];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("batch compress should succeed");

    assert_eq!(result.files.len(), 2, "batch should output 2 files");
    for f in &result.files {
        assert!(
            !f.data.is_empty(),
            "output file '{}' should have data",
            f.name
        );
    }
}

// =========================================================================
// Compress Images -- metadata verification
// =========================================================================

#[test]
fn compress_images_metadata_includes_size_stats() {
    // Metadata flows through the pipeline to PipelineFileResult.metadata,
    // which the UI reads to show "X% smaller" on each result card.
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 50 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let input_size = SMALL_JPEG.len() as u64;
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("compress pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    let metadata = &result.files[0].metadata;

    assert_eq!(
        metadata["originalSize"].as_u64().unwrap(),
        input_size,
        "originalSize should match the input file size"
    );

    let compressed_size = metadata["compressedSize"].as_u64().unwrap();
    assert_eq!(
        compressed_size,
        result.files[0].data.len() as u64,
        "compressedSize should match the output data length"
    );

    let ratio = metadata["compressionRatio"].as_f64().unwrap();
    assert!(
        ratio > 0.0 && ratio < 100.0,
        "compressionRatio {} should be between 0 and 100",
        ratio
    );
}

// =========================================================================
// Resize Images
// =========================================================================

#[test]
fn resize_images_recipe_produces_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-resize", "type": "group",
                "nodes": [{
                    "id": "resize-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "resize-image", "type": "image",
                        "parameters": { "operation": "resize", "width": 100 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("resize pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].data.len() >= 2);
    assert_eq!(&result.files[0].data[0..2], &[0xFF, 0xD8]);
}

// =========================================================================
// Convert Image Format
// =========================================================================

#[test]
fn convert_image_format_recipe_produces_png() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-convert", "type": "group",
                "nodes": [{
                    "id": "convert-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "convert-image", "type": "image",
                        "parameters": { "operation": "convert", "format": "png", "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("convert pipeline should succeed");

    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].data.len() >= 4);
    assert_eq!(
        &result.files[0].data[0..4],
        &[0x89, 0x50, 0x4E, 0x47],
        "output should be valid PNG (magic bytes)"
    );
}

// =========================================================================
// Clean CSV
// =========================================================================

#[test]
fn clean_csv_recipe_produces_cleaned_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "csv-cleaner", "type": "group",
                "nodes": [{
                    "id": "clean", "type": "spreadsheet",
                    "parameters": {
                        "operation": "clean",
                        "trimWhitespace": true,
                        "removeEmptyRows": true,
                        "removeDuplicates": true
                    }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("data.csv", MESSY_CSV, "text/csv")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("clean CSV pipeline should succeed");

    assert_eq!(result.files.len(), 1);

    let output_str =
        std::str::from_utf8(&result.files[0].data).expect("cleaned CSV should be valid UTF-8");

    // Cleaned CSV should be shorter (empty rows removed, whitespace trimmed).
    assert!(
        result.files[0].data.len() <= MESSY_CSV.len(),
        "cleaned CSV ({} bytes) should not be larger than input ({} bytes)",
        result.files[0].data.len(),
        MESSY_CSV.len()
    );

    assert!(
        output_str.contains(','),
        "cleaned CSV should have comma-separated values"
    );
}

// =========================================================================
// Rename CSV Columns
// =========================================================================

#[test]
fn rename_csv_columns_recipe_produces_output() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "column-renamer", "type": "group",
                "nodes": [{
                    "id": "rename-columns", "type": "spreadsheet",
                    "parameters": {
                        "operation": "rename",
                        "columns": { "name": "full_name" }
                    }
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![file("data.csv", SIMPLE_CSV, "text/csv")];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("rename columns pipeline should succeed");

    assert_eq!(result.files.len(), 1);

    let output_str =
        std::str::from_utf8(&result.files[0].data).expect("renamed CSV should be valid UTF-8");

    let first_line = output_str.lines().next().unwrap_or("");
    assert!(
        first_line.contains("full_name"),
        "header should contain renamed column 'full_name', got: {}",
        first_line
    );
}

// =========================================================================
// Rename Files
// =========================================================================

#[test]
fn rename_files_recipe_applies_prefix() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-rename", "type": "group",
                "nodes": [{
                    "id": "rename-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "rename-file", "type": "file-system",
                        "parameters": { "operation": "rename", "prefix": "renamed-" }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();
    let reporter = PipelineReporter::new_noop();
    let files = vec![
        file("document.txt", b"hello world", "text/plain"),
        file("readme.md", b"# Title", "text/markdown"),
    ];

    let result = execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("rename files pipeline should succeed");

    assert_eq!(result.files.len(), 2, "should output 2 renamed files");

    for f in &result.files {
        assert!(
            f.name.starts_with("renamed-"),
            "file '{}' should have 'renamed-' prefix",
            f.name
        );
    }

    // Data should be unchanged (rename only affects filenames).
    assert_eq!(result.files[0].data, b"hello world");
    assert_eq!(result.files[1].data, b"# Title");
}

// =========================================================================
// Progress Events -- real recipe emits correct event sequence
// =========================================================================

#[test]
fn compress_recipe_emits_expected_events() {
    use bnto_core::PipelineEvent;
    use std::sync::{Arc, Mutex};

    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();

    let events: Arc<Mutex<Vec<PipelineEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    let reporter = PipelineReporter::new(move |event: PipelineEvent| {
        events_clone.lock().unwrap().push(event);
    });

    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("compress pipeline should succeed");

    let collected = events.lock().unwrap();

    assert!(
        matches!(
            collected.first(),
            Some(PipelineEvent::PipelineStarted { .. })
        ),
        "first event should be PipelineStarted"
    );

    assert!(
        matches!(
            collected.last(),
            Some(PipelineEvent::PipelineCompleted { .. })
        ),
        "last event should be PipelineCompleted"
    );

    let group_started = collected.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress"),
    );
    assert!(
        group_started,
        "should emit NodeStarted for batch-compress group"
    );
}
