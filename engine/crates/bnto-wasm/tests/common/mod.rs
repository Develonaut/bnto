// Shared helpers for recipe integration tests.
#![allow(dead_code)]

use bnto_core::processor::FileData;
use bnto_core::{PipelineDefinition, PipelineFile};

/// A tiny valid JPEG (smallest possible -- 2x2 pixel, ~600 bytes).
pub static SMALL_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

/// A tiny valid PNG from the test-fixtures directory.
pub static SMALL_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

/// A simple CSV with data that can be cleaned.
pub static MESSY_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/messy.csv");

/// A simple clean CSV for column rename testing.
pub static SIMPLE_CSV: &[u8] = include_bytes!("../../../../../test-fixtures/csv/simple.csv");

/// Build the browser-safe registry with all WASM-capable processors.
pub fn real_registry() -> bnto_core::NodeRegistry {
    bnto_engine::create_browser_registry()
}

/// Parse a JSON string into a PipelineDefinition.
pub fn parse(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).expect("recipe JSON should parse")
}

/// Create a PipelineFile from name, data, and MIME type.
pub fn file(name: &str, data: &[u8], mime: &str) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: FileData::Bytes(data.to_vec()),
        mime_type: mime.to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// Extract bytes from FileData (panics if Path variant, which tests never produce).
pub fn file_bytes(data: &FileData) -> Vec<u8> {
    data.clone().into_bytes().expect("test: should have bytes")
}

/// Fake time source -- deterministic, returns 1000ms always.
pub fn fake_now() -> u64 {
    1000
}
