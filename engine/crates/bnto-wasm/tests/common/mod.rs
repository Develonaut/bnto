// Shared helpers for recipe integration tests.
#![allow(dead_code)]

use bnto_core::{PipelineDefinition, PipelineFile};

/// A tiny valid JPEG (smallest possible -- 2x2 pixel, ~600 bytes).
pub static SMALL_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

/// A tiny valid PNG from the test-fixtures directory.
pub static SMALL_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

/// A simple CSV with data that can be cleaned.
pub static MESSY_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/messy.csv");

/// A simple clean CSV for column rename testing.
pub static SIMPLE_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/simple.csv");

/// Build the production registry with all 6 real processors.
pub fn real_registry() -> bnto_core::NodeRegistry {
    let mut registry = bnto_core::NodeRegistry::new();
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
    registry
}

/// Parse a JSON string into a PipelineDefinition.
pub fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

/// Create a PipelineFile from name, data, and MIME type.
pub fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// Fake time source -- deterministic, returns 1000ms always.
pub fn fake_now() -> u64 {
    1000
}
