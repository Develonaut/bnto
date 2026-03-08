// =============================================================================
// Recipe Integration Tests — Full Pipeline with Real Processors
// =============================================================================
//
// WHAT ARE THESE TESTS?
// These are integration tests that feed real recipe JSON (matching the
// TypeScript composite recipe definitions in `@bnto/nodes`) through the
// full Rust pipeline executor with REAL processors and REAL file data.
//
// This is the "CLI test" layer — if these pass, any layer built on top
// of the engine (browser WASM, desktop Tauri, future CLI binary) is
// guaranteed to work because the engine's internal pipeline is correct.
//
// WHY NOT WASM?
// These are native Rust tests (`cargo test`), not WASM integration tests.
// They test the same code path that the WASM bridge uses (create_default_registry
// + execute_pipeline), without needing a JS runtime. Faster and more reliable.
//
// WHAT DO THEY VALIDATE?
// 1. Recipe JSON deserializes correctly (structural fidelity)
// 2. All processor compound keys resolve from the registry
// 3. Actual output data is correct (image compressed, CSV cleaned, etc.)
// 4. Output file count matches expectations
// 5. Output filenames follow expected patterns

use bnto_core::{PipelineDefinition, PipelineFile, PipelineReporter, execute_pipeline};

// --- Test fixture data ---
// We embed small test files directly so tests run without filesystem access.

/// A tiny valid JPEG (smallest possible — 2x2 pixel, ~600 bytes).
/// We use the small.jpg fixture from the test-fixtures directory.
static SMALL_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

/// A tiny valid PNG from the test-fixtures directory.
static SMALL_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");

/// A simple CSV with data that can be cleaned.
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

/// A simple clean CSV for column rename testing.
static SIMPLE_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/simple.csv");

// --- Helpers ---

/// Build the production registry with all 6 real processors.
/// This is the exact same registry the WASM bridge creates.
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

/// Parse a JSON string into a PipelineDefinition.
fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

/// Create a PipelineFile from name, data, and MIME type.
fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime.to_string(),
    }
}

/// Fake time source — deterministic, returns 1000ms always.
fn fake_now() -> u64 {
    1000
}

// =============================================================================
// Compress Images — full recipe integration
// =============================================================================

#[test]
fn compress_images_recipe_produces_smaller_output() {
    // The composite recipe: Input → Group("Batch Compress") → Loop → image:compress → Output
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

    // Should produce exactly 1 output file.
    assert_eq!(result.files.len(), 1, "should output 1 file");

    // Output should be a valid JPEG (starts with FF D8).
    assert!(result.files[0].data.len() >= 2, "output should have data");
    assert_eq!(
        &result.files[0].data[0..2],
        &[0xFF, 0xD8],
        "output should be a valid JPEG (magic bytes)"
    );

    // At q=50, compressed output should be smaller than the original.
    assert!(
        result.files[0].data.len() < input_size,
        "compressed JPEG at q=50 ({} bytes) should be smaller than input ({} bytes)",
        result.files[0].data.len(),
        input_size
    );
}

// =============================================================================
// Compress Images — batch (multiple files)
// =============================================================================

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

    // Should produce 2 output files — one per input.
    assert_eq!(result.files.len(), 2, "batch should output 2 files");

    // Both should have non-empty data.
    for f in &result.files {
        assert!(
            !f.data.is_empty(),
            "output file '{}' should have data",
            f.name
        );
    }
}

// =============================================================================
// Resize Images — full recipe integration
// =============================================================================

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
    // Output should be a valid JPEG.
    assert!(result.files[0].data.len() >= 2);
    assert_eq!(&result.files[0].data[0..2], &[0xFF, 0xD8]);
}

// =============================================================================
// Convert Image Format — full recipe integration
// =============================================================================

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

    // Output should be a valid PNG (starts with 89 50 4E 47 = \x89PNG).
    assert!(result.files[0].data.len() >= 4);
    assert_eq!(
        &result.files[0].data[0..4],
        &[0x89, 0x50, 0x4E, 0x47],
        "output should be a valid PNG (magic bytes)"
    );
}

// =============================================================================
// Clean CSV — full recipe integration
// =============================================================================

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

    // Output should be valid UTF-8 CSV.
    let output_str =
        std::str::from_utf8(&result.files[0].data).expect("cleaned CSV should be valid UTF-8");

    // Cleaned CSV should be shorter than the messy input
    // (empty rows removed, whitespace trimmed).
    assert!(
        result.files[0].data.len() <= MESSY_CSV.len(),
        "cleaned CSV ({} bytes) should not be larger than input ({} bytes)",
        result.files[0].data.len(),
        MESSY_CSV.len()
    );

    // Should still contain header row.
    assert!(
        output_str.contains(','),
        "cleaned CSV should have comma-separated values"
    );
}

// =============================================================================
// Rename CSV Columns — full recipe integration
// =============================================================================

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

    // Output should be valid UTF-8 CSV.
    let output_str =
        std::str::from_utf8(&result.files[0].data).expect("renamed CSV should be valid UTF-8");

    // The header should contain the new column name.
    let first_line = output_str.lines().next().unwrap_or("");
    assert!(
        first_line.contains("full_name"),
        "header should contain renamed column 'full_name', got: {}",
        first_line
    );
}

// =============================================================================
// Rename Files — full recipe integration
// =============================================================================

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

    // Both files should have the "renamed-" prefix.
    for f in &result.files {
        assert!(
            f.name.starts_with("renamed-"),
            "file '{}' should have 'renamed-' prefix",
            f.name
        );
    }

    // Data should be unchanged (rename only affects filenames, not content).
    assert_eq!(result.files[0].data, b"hello world");
    assert_eq!(result.files[1].data, b"# Title");
}

// =============================================================================
// Progress Events — real recipe emits correct event sequence
// =============================================================================

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

    // Collect events into a shared vector.
    let events: Arc<Mutex<Vec<PipelineEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    let reporter = PipelineReporter::new(move |event: PipelineEvent| {
        events_clone.lock().unwrap().push(event);
    });

    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("compress pipeline should succeed");

    let collected = events.lock().unwrap();

    // First event should be PipelineStarted.
    assert!(
        matches!(
            collected.first(),
            Some(PipelineEvent::PipelineStarted { .. })
        ),
        "first event should be PipelineStarted"
    );

    // Last event should be PipelineCompleted.
    assert!(
        matches!(
            collected.last(),
            Some(PipelineEvent::PipelineCompleted { .. })
        ),
        "last event should be PipelineCompleted"
    );

    // Should have NodeStarted for the batch-compress group.
    let group_started = collected.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress"),
    );
    assert!(
        group_started,
        "should emit NodeStarted for batch-compress group"
    );
}
