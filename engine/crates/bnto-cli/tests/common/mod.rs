// Shared test helpers for CLI integration tests.
// Provides pipeline execution with real processors and fixture file helpers.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use bnto_core::{
    execute_pipeline, NodeRegistry, PipelineDefinition, PipelineEvent, PipelineFile,
    PipelineReporter, PipelineResult,
};

/// Create a registry with all 6 real processors (same as CLI and WASM).
pub fn create_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();

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

/// Run a pipeline from a JSON definition string and input files.
pub fn run_pipeline(
    definition_json: &str,
    files: Vec<PipelineFile>,
) -> Result<PipelineResult, String> {
    let definition: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| e.to_string())?;

    let registry = create_registry();
    let reporter = PipelineReporter::new_noop();
    let now_ms = || 0u64;

    execute_pipeline(&definition, files, &registry, &reporter, now_ms).map_err(|e| e.to_string())
}

/// Run a pipeline and capture all progress events.
pub fn run_pipeline_with_events(
    definition_json: &str,
    files: Vec<PipelineFile>,
) -> Result<(PipelineResult, Vec<PipelineEvent>), String> {
    let definition: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| e.to_string())?;

    let registry = create_registry();
    let events: Arc<Mutex<Vec<PipelineEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    let reporter = PipelineReporter::new(move |event| {
        events_clone.lock().unwrap().push(event);
    });
    let now_ms = || 0u64;

    let result = execute_pipeline(&definition, files, &registry, &reporter, now_ms)
        .map_err(|e| e.to_string())?;

    let captured = events.lock().unwrap().clone();
    Ok((result, captured))
}

/// Create a PipelineFile from a name and embedded bytes.
pub fn make_file(name: &str, data: &[u8], mime_type: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime_type.to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// Assert the byte slice starts with the JPEG magic bytes (FF D8 FF).
pub fn assert_jpeg(data: &[u8]) {
    assert!(
        data.len() >= 3 && data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF,
        "Expected JPEG magic bytes (FF D8 FF), got {:02X?}",
        &data[..data.len().min(4)]
    );
}

/// Assert the byte slice starts with the PNG magic bytes (89 50 4E 47).
pub fn assert_png(data: &[u8]) {
    assert!(
        data.len() >= 4 && data[0] == 0x89 && data[1] == 0x50 && data[2] == 0x4E && data[3] == 0x47,
        "Expected PNG magic bytes (89 50 4E 47), got {:02X?}",
        &data[..data.len().min(4)]
    );
}

/// Assert the byte slice starts with the WebP magic bytes (RIFF....WEBP).
pub fn assert_webp(data: &[u8]) {
    assert!(
        data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP",
        "Expected WebP magic bytes (RIFF....WEBP), got {:02X?}",
        &data[..data.len().min(12)]
    );
}

/// Assert the byte slice is valid CSV (contains commas or newlines, valid UTF-8).
pub fn assert_csv(data: &[u8]) {
    let text = std::str::from_utf8(data).expect("CSV data should be valid UTF-8");
    assert!(
        text.contains(',') || text.contains('\n'),
        "Expected CSV content (commas or newlines), got: {}...",
        &text[..text.len().min(100)]
    );
}
