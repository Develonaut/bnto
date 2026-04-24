// Pipeline execution bridge — runs the engine on a background thread
// and relays PipelineEvent progress back to the TUI event loop via mpsc.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use bnto_core::events::{PipelineEvent, PipelineReporter};
use bnto_core::{InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::recipes::builtin_recipe_by_slug;
use bnto_engine::run_pipeline;

use crate::context::NativeContext;
use crate::input;
use crate::io;

use super::app::AppMessage;
use super::screens::execution::ExecutionMessage;
use super::screens::results::OutputFile;

/// Message sent from the bridge thread back to the TUI event loop.
#[derive(Debug)]
pub enum BridgeEvent {
    /// A pipeline progress event from the engine.
    Progress(PipelineEvent),
    /// Pipeline finished — output files written to this directory.
    Done {
        output_dir: String,
        file_count: usize,
        duration_ms: u64,
        file_metadata: Vec<FileResultMeta>,
    },
    /// Pipeline or setup failed with an error message.
    Error(String),
}

/// Per-file metadata extracted from engine results before writing to disk.
#[derive(Debug, Clone)]
pub struct FileResultMeta {
    pub name: String,
    pub original_size: Option<u64>,
}

/// Extract per-file metadata (original size) from the engine's PipelineResult.
fn extract_file_metadata(result: &bnto_core::pipeline::PipelineResult) -> Vec<FileResultMeta> {
    result
        .files
        .iter()
        .map(|f| {
            let original_size = f.metadata.get("originalSize").and_then(|v| v.as_u64());
            FileResultMeta {
                name: f.name.clone(),
                original_size,
            }
        })
        .collect()
}

/// Spawn a background thread that runs the pipeline for the given recipe.
///
/// Returns a Receiver that the TUI event loop polls each tick.
/// The thread resolves the recipe, prepares inputs, runs the engine,
/// writes output files, then sends a Done or Error event.
pub fn spawn_pipeline(
    slug: String,
    selected_files: Vec<PathBuf>,
    param_overrides: HashMap<String, String>,
    output_dir_override: Option<String>,
) -> mpsc::Receiver<BridgeEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        run_bridge(
            tx,
            &slug,
            &selected_files,
            &param_overrides,
            output_dir_override.as_deref(),
        );
    });

    rx
}

/// Inner bridge logic — runs synchronously on the spawned thread.
fn run_bridge(
    tx: mpsc::Sender<BridgeEvent>,
    slug: &str,
    selected_files: &[PathBuf],
    param_overrides: &HashMap<String, String>,
    output_dir_override: Option<&str>,
) {
    // Resolve the recipe from the embedded catalog.
    let recipe = match builtin_recipe_by_slug(slug) {
        Some(r) => r,
        None => {
            let _ = tx.send(BridgeEvent::Error(format!("Unknown recipe: {slug}")));
            return;
        }
    };

    // Resolve input mode to handle URL/Text recipes differently.
    let input_mode = serde_json::from_str::<PipelineDefinition>(recipe.definition_json)
        .map(|def| resolve_input_mode(&def))
        .unwrap_or_default();

    // For URL/Text mode, extract the value from param overrides and
    // pass it as a positional arg (prepare_inputs expects it there).
    let mut args: Vec<String> = selected_files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let mut remaining_overrides = param_overrides.clone();

    match input_mode {
        InputMode::Url => {
            if let Some(url) = extract_override_value(&mut remaining_overrides, "url") {
                args.push(url);
            }
        }
        InputMode::Text => {
            if let Some(text) = extract_override_value(&mut remaining_overrides, "text") {
                args.push(text);
            }
        }
        InputMode::FileUpload => {}
    }

    // Convert remaining param overrides to "key=value" strings.
    let override_args: Vec<String> = remaining_overrides
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect();

    // Prepare inputs (validates files, applies param overrides to definition).
    let prepared = match input::prepare_inputs(recipe.definition_json, &args, &override_args) {
        Ok(p) => p,
        Err(e) => {
            let _ = tx.send(BridgeEvent::Error(e));
            return;
        }
    };

    // Create a native context for the current working directory.
    let ctx = match NativeContext::current_dir() {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(BridgeEvent::Error(format!("{e}")));
            return;
        }
    };

    // Create a reporter that sends engine events to the TUI via the channel.
    let progress_tx = tx.clone();
    let reporter = PipelineReporter::new(move |event| {
        let _ = progress_tx.send(BridgeEvent::Progress(event));
    });

    // Run the pipeline (blocking — this is why we're on a background thread).
    let result = match run_pipeline(&prepared.definition_json, prepared.files, &reporter, &ctx) {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.send(BridgeEvent::Error(format!("{e}")));
            return;
        }
    };

    // Use configured output directory or fall back to temp.
    // Always clean previous run's output before writing new results.
    let output_dir = match output_dir_override {
        Some(dir) if !dir.is_empty() => PathBuf::from(dir).join(format!("bnto-{slug}")),
        _ => std::env::temp_dir().join(format!("bnto-tui-{slug}")),
    };
    let _ = std::fs::remove_dir_all(&output_dir);
    let _ = std::fs::create_dir_all(&output_dir);
    let output_dir_str = output_dir.to_string_lossy().into_owned();

    // Extract per-file metadata before writing (writing consumes data).
    let file_metadata: Vec<FileResultMeta> = extract_file_metadata(&result);

    if let Err(e) = io::write_results(&result, &output_dir_str) {
        let _ = tx.send(BridgeEvent::Error(e));
        return;
    }

    let _ = tx.send(BridgeEvent::Done {
        output_dir: output_dir_str,
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
        file_metadata,
    });
}

/// Find and remove an override value by param name (e.g. "url").
///
/// Override keys use "nodeId:paramName" format from the detail screen.
/// Returns the value and removes the entry so it isn't double-applied.
fn extract_override_value(
    overrides: &mut HashMap<String, String>,
    param_name: &str,
) -> Option<String> {
    let key = overrides
        .keys()
        .find(|k| k.ends_with(&format!(":{param_name}")))
        .cloned();
    key.and_then(|k| overrides.remove(&k))
}

/// Map an engine PipelineEvent to a TUI AppMessage.
pub fn map_pipeline_event(event: PipelineEvent) -> AppMessage {
    match event {
        PipelineEvent::PipelineStarted {
            total_nodes,
            total_files,
        } => AppMessage::Execution(ExecutionMessage::PipelineStarted {
            total_nodes,
            total_files,
        }),
        PipelineEvent::NodeStarted {
            node_id, node_type, ..
        } => AppMessage::Execution(ExecutionMessage::NodeStarted { node_id, node_type }),
        PipelineEvent::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        } => AppMessage::Execution(ExecutionMessage::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        }),
        PipelineEvent::NodeCompleted {
            node_id,
            duration_ms,
            ..
        } => AppMessage::Execution(ExecutionMessage::NodeCompleted {
            node_id,
            duration_ms,
        }),
        PipelineEvent::NodeFailed { node_id, error } => {
            AppMessage::Execution(ExecutionMessage::NodeFailed { node_id, error })
        }
        PipelineEvent::PipelineCompleted {
            duration_ms,
            total_files_processed,
        } => AppMessage::Execution(ExecutionMessage::PipelineCompleted {
            duration_ms,
            total_files_processed,
        }),
        PipelineEvent::PipelineFailed { node_id, error } => {
            AppMessage::Execution(ExecutionMessage::PipelineFailed { node_id, error })
        }
        PipelineEvent::CommandOutput { node_id, line } => {
            AppMessage::Execution(ExecutionMessage::CommandOutput { node_id, line })
        }
    }
}

/// Read output files from the bridge's output directory, merging engine metadata.
pub fn build_output_files(output_dir: &str, file_metadata: &[FileResultMeta]) -> Vec<OutputFile> {
    let dir = std::path::Path::new(output_dir);
    let mut outputs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
            // Match by filename to get original_size from engine metadata.
            let original_size = file_metadata
                .iter()
                .find(|m| m.name == name)
                .and_then(|m| m.original_size);
            outputs.push(OutputFile {
                name,
                size_bytes,
                original_size,
            });
        }
    }
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn extract_metadata_reads_original_size() {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(780_000.into()),
        );
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "photo.jpg".into(),
                data: vec![],
                mime_type: "image/jpeg".into(),
                metadata,
            }],
            duration_ms: 100,
        };
        let meta = extract_file_metadata(&result);
        assert_eq!(meta.len(), 1);
        assert_eq!(meta[0].name, "photo.jpg");
        assert_eq!(meta[0].original_size, Some(780_000));
    }

    #[test]
    fn extract_metadata_handles_missing() {
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "file.txt".into(),
                data: vec![],
                mime_type: "text/plain".into(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 50,
        };
        let meta = extract_file_metadata(&result);
        assert_eq!(meta[0].original_size, None);
    }

    #[test]
    fn extract_metadata_handles_non_numeric() {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::String("not a number".into()),
        );
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "file.txt".into(),
                data: vec![],
                mime_type: "text/plain".into(),
                metadata,
            }],
            duration_ms: 50,
        };
        let meta = extract_file_metadata(&result);
        assert_eq!(meta[0].original_size, None);
    }

    #[test]
    fn build_output_files_merges_metadata() {
        let dir = std::env::temp_dir().join("bnto-test-bridge-merge");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("photo.jpg"), vec![0u8; 290]).unwrap();

        let meta = vec![FileResultMeta {
            name: "photo.jpg".into(),
            original_size: Some(780),
        }];
        let outputs = build_output_files(&dir.to_string_lossy(), &meta);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].original_size, Some(780));
        assert_eq!(outputs[0].size_bytes, 290);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn extract_override_value_finds_by_param_name() {
        let mut overrides = HashMap::new();
        overrides.insert("download:url".into(), "https://example.com".into());
        overrides.insert("download:format".into(), "mp4".into());

        let url = extract_override_value(&mut overrides, "url");
        assert_eq!(url, Some("https://example.com".into()));
        // The key should be removed from overrides.
        assert!(!overrides.contains_key("download:url"));
        // Other keys remain.
        assert!(overrides.contains_key("download:format"));
    }

    #[test]
    fn extract_override_value_returns_none_when_missing() {
        let mut overrides = HashMap::new();
        overrides.insert("download:format".into(), "mp4".into());

        let text = extract_override_value(&mut overrides, "text");
        assert_eq!(text, None);
        assert_eq!(overrides.len(), 1);
    }

    #[test]
    fn build_output_files_handles_unmatched() {
        let dir = std::env::temp_dir().join("bnto-test-bridge-unmatched");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("other.txt"), vec![0u8; 100]).unwrap();

        let meta = vec![FileResultMeta {
            name: "different.txt".into(),
            original_size: Some(500),
        }];
        let outputs = build_output_files(&dir.to_string_lossy(), &meta);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].original_size, None);

        let _ = fs::remove_dir_all(&dir);
    }
}
