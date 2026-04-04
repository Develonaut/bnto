// Mock processors and shared test helpers for executor tests.
use super::*;
use crate::context::{NoopContext, ProcessContext};
use crate::events::RecordingReporter;
use crate::processor::{NodeOutput, OutputFile};

// =========================================================================
// Mock Processors for Testing
// =========================================================================

/// Echoes input files back unchanged.
struct EchoProcessor;

impl crate::processor::NodeProcessor for EchoProcessor {
    fn name(&self) -> &str {
        "echo"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename,
                mime_type: input
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Converts filename to uppercase.
struct UpperCaseProcessor;

impl crate::processor::NodeProcessor for UpperCaseProcessor {
    fn name(&self) -> &str {
        "uppercase"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename.to_uppercase(),
                mime_type: input
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Always fails. For testing error handling.
struct FailProcessor;

impl crate::processor::NodeProcessor for FailProcessor {
    fn name(&self) -> &str {
        "fail"
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        Err(BntoError::ProcessingFailed(
            "intentional test failure".to_string(),
        ))
    }
}

/// Reports progress at 25/50/75/100%.
struct SlowProcessor;

impl crate::processor::NodeProcessor for SlowProcessor {
    fn name(&self) -> &str {
        "slow"
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(25, "Quarter done");
        progress.report(50, "Half done");
        progress.report(75, "Three quarters");
        progress.report(100, "Complete");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: input.data,
                filename: input.filename,
                mime_type: input
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Returns two files per input.
struct DoubleProcessor;

impl crate::processor::NodeProcessor for DoubleProcessor {
    fn name(&self) -> &str {
        "double"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let mime = input
            .mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string());
        Ok(NodeOutput {
            files: vec![
                OutputFile {
                    data: input.data.clone(),
                    filename: format!("{}-a", input.filename),
                    mime_type: mime.clone(),
                },
                OutputFile {
                    data: input.data,
                    filename: format!("{}-b", input.filename),
                    mime_type: mime,
                },
            ],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Source processor — generates output from params, ignores input data.
/// Returns a single file whose name comes from the "url" param (or "generated.bin").
struct SourceProcessor;

impl crate::processor::NodeProcessor for SourceProcessor {
    fn name(&self) -> &str {
        "source"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            node_type: "test-source".to_string(),
            name: "Source".to_string(),
            description: "Generates output from params".to_string(),
            category: crate::metadata::NodeCategory::Network,
            accepts: vec![],
            platforms: vec!["server".to_string()],
            parameters: vec![],
            input_cardinality: crate::metadata::InputCardinality::Source,
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let url = input
            .params
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("generated.bin");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: format!("downloaded-from:{}", url).into_bytes(),
                filename: format!("{}.mp4", url.rsplit('/').next().unwrap_or("output")),
                mime_type: "video/mp4".to_string(),
            }],
            metadata: serde_json::Map::new(),
        })
    }
}

/// Simulates compression: halves data size and attaches size metadata.
/// Used to verify metadata survives through the pipeline.
struct MetadataProcessor;

impl crate::processor::NodeProcessor for MetadataProcessor {
    fn name(&self) -> &str {
        "metadata"
    }

    fn process(
        &self,
        input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let original_size = input.data.len() as u64;
        let compressed_data = vec![0u8; input.data.len() / 2];
        let compressed_size = compressed_data.len() as u64;
        let ratio = compressed_size as f64 / original_size as f64;

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(original_size)),
        );
        metadata.insert(
            "compressedSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(compressed_size)),
        );
        if let Some(ratio_num) = serde_json::Number::from_f64(ratio) {
            metadata.insert(
                "compressionRatio".to_string(),
                serde_json::Value::Number(ratio_num),
            );
        }

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: compressed_data,
                filename: input.filename,
                mime_type: input
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string()),
            }],
            metadata,
        })
    }
}

// =========================================================================
// Test Helpers
// =========================================================================

fn make_file(name: &str, data: &[u8]) -> PipelineFile {
    PipelineFile {
        name: name.to_string(),
        data: data.to_vec(),
        mime_type: "application/octet-stream".to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// Build a registry with mock processors under test type keys.
fn mock_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry.register("test-echo", Box::new(EchoProcessor));
    registry.register("test-uppercase", Box::new(UpperCaseProcessor));
    registry.register("test-fail", Box::new(FailProcessor));
    registry.register("test-slow", Box::new(SlowProcessor));
    registry.register("test-double", Box::new(DoubleProcessor));
    registry.register("test-metadata", Box::new(MetadataProcessor));
    registry.register("test-source", Box::new(SourceProcessor));
    registry
}

/// Maps real recipe node type keys to mock processors so we can test
/// recipe orchestration without needing actual image/CSV/file processors.
fn recipe_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry.register("image-compress", Box::new(EchoProcessor));
    registry.register("image-resize", Box::new(EchoProcessor));
    registry.register("image-convert", Box::new(EchoProcessor));
    registry.register("spreadsheet-clean", Box::new(EchoProcessor));
    registry.register("spreadsheet-rename", Box::new(EchoProcessor));
    // UpperCase verifies transformation happened.
    registry.register("file-rename", Box::new(UpperCaseProcessor));
    registry
}

fn parse_def(json: &str) -> PipelineDefinition {
    serde_json::from_str(json).unwrap()
}

/// Always returns 1000ms — keeps tests deterministic.
fn fake_now() -> u64 {
    1000
}

// =========================================================================
// Test Submodules
// =========================================================================

mod auto_iteration;
mod basic;
mod containers;
mod errors;
mod metadata;
mod progress;
mod recipes;
mod recipes_containers;
mod recipes_errors;
mod recipes_events;
mod sub_pipelines;
mod sub_pipelines_containers;
mod sub_pipelines_edge_cases;
