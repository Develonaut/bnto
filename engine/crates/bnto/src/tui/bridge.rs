// Pipeline execution bridge — runs the engine on a background thread
// and relays PipelineEvent progress back to the TUI event loop via mpsc.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use bnto_core::events::{PipelineEvent, PipelineReporter};
use bnto_core::{InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::run_pipeline;

use crate::context::NativeContext;
use crate::dry_run::files::FilePreview;
use crate::input;
use crate::io;

mod events;
mod output;

pub use crate::progressive_output::write_progressive_files;
pub use events::map_pipeline_event;
use output::extract_file_metadata;
pub use output::{FileResultMeta, build_output_files};

/// Message sent from the bridge thread back to the TUI event loop.
#[derive(Debug)]
pub enum BridgeEvent {
    /// A pipeline progress event from the engine.
    Progress(PipelineEvent),
    /// Output directory resolved — sent before pipeline execution starts.
    /// Used by progressive output to write files as iterations complete.
    OutputDir(String),
    /// Pipeline finished — output files written to this directory.
    Done {
        output_dir: String,
        file_count: usize,
        duration_ms: u64,
        file_metadata: Vec<FileResultMeta>,
        warnings: Vec<String>,
        /// File transformation previews (populated in preview mode only).
        preview_data: Vec<FilePreview>,
    },
    /// Pipeline or setup failed with an error message.
    Error(String),
}

/// Everything one bridge run needs, owned by the worker thread.
struct BridgeJob {
    slug: String,
    definition_json: String,
    selected_files: Vec<PathBuf>,
    param_overrides: HashMap<String, String>,
    preview_mode: bool,
}

/// Spawn a background thread that runs the pipeline for the given recipe.
///
/// Returns a Receiver that the TUI event loop polls each tick.
/// The thread runs the engine with the given definition JSON,
/// writes output files, then sends a Done or Error event.
pub fn spawn_pipeline(
    slug: String,
    definition_json: String,
    selected_files: Vec<PathBuf>,
    param_overrides: HashMap<String, String>,
    preview_mode: bool,
) -> mpsc::Receiver<BridgeEvent> {
    let (tx, rx) = mpsc::channel();
    let job = BridgeJob {
        slug,
        definition_json,
        selected_files,
        param_overrides,
        preview_mode,
    };
    thread::spawn(move || run_bridge(tx, &job));
    rx
}

/// Inner bridge logic — runs synchronously on the spawned thread.
/// Any failure is reported to the TUI as a single `Error` event.
fn run_bridge(tx: mpsc::Sender<BridgeEvent>, job: &BridgeJob) {
    if let Err(e) = try_run_bridge(&tx, job) {
        let _ = tx.send(BridgeEvent::Error(e));
    }
}

/// Prepare inputs, run the pipeline, then write (or preview) the results.
fn try_run_bridge(tx: &mpsc::Sender<BridgeEvent>, job: &BridgeJob) -> Result<(), String> {
    let (args, override_args) = input_args(job);
    // Prepare inputs (validates files, applies param overrides to definition).
    let prepared = input::prepare_inputs(&job.definition_json, &args, &override_args)?;
    let ctx = NativeContext::current_dir()
        .map_err(|e| format!("{e}"))?
        .with_dry_run(job.preview_mode);
    let mode = resolve_mode(&job.definition_json);
    let output_dir = resolve_output_dir(tx, job, &mode, &ctx, &prepared.files);
    let reporter = channel_reporter(tx);
    // Run the pipeline (blocking — this is why we're on a background thread).
    let result = run_pipeline(&prepared.definition_json, prepared.files, &reporter, &ctx)
        .map_err(|e| format!("{e}"))?;
    if job.preview_mode {
        send_preview_done(tx, output_dir, result);
        return Ok(());
    }
    // Normal mode: write output files using mode-aware dispatch.
    let file_metadata = extract_file_metadata(&result);
    io::write_results_with_mode(&result, &mode, &output_dir, false)?;
    send_done(tx, output_dir, result, file_metadata);
    Ok(())
}

/// Positional args and `key=value` overrides for `prepare_inputs`.
///
/// For URL/Text mode, the value moves from the param overrides into the
/// positional args (prepare_inputs expects it there).
fn input_args(job: &BridgeJob) -> (Vec<String>, Vec<String>) {
    let mut args: Vec<String> = job
        .selected_files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let mut remaining_overrides = job.param_overrides.clone();
    let param = match recipe_input_mode(&job.definition_json) {
        InputMode::Url => Some("url"),
        InputMode::Text => Some("text"),
        InputMode::FileUpload | InputMode::None => None,
    };
    if let Some(value) = param.and_then(|p| extract_override_value(&mut remaining_overrides, p)) {
        args.push(value);
    }
    let override_args = remaining_overrides
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    (args, override_args)
}

/// The recipe's input mode (FileUpload when the JSON doesn't parse).
fn recipe_input_mode(definition_json: &str) -> InputMode {
    serde_json::from_str::<PipelineDefinition>(definition_json)
        .map(|def| resolve_input_mode(&def))
        .unwrap_or_default()
}

/// Resolve the output mode from the recipe definition.
fn resolve_mode(definition_json: &str) -> String {
    serde_json::from_str::<PipelineDefinition>(definition_json)
        .map(|def| bnto_core::resolve_output_mode(&def))
        .unwrap_or_else(|_| "write".to_string())
}

/// Resolve the output directory and announce it to the TUI.
///
/// Preview mode writes nothing (empty string, no event). Overwrite mode
/// reports the source directory, since files are renamed in place.
fn resolve_output_dir(
    tx: &mpsc::Sender<BridgeEvent>,
    job: &BridgeJob,
    mode: &str,
    ctx: &NativeContext,
    files: &[bnto_core::PipelineFile],
) -> String {
    if job.preview_mode {
        return String::new();
    }
    let dir_str = if mode == "overwrite" {
        source_dir(&job.selected_files)
    } else {
        fresh_output_dir(job, ctx, files)
    };
    let _ = tx.send(BridgeEvent::OutputDir(dir_str.clone()));
    dir_str
}

/// Directory of the first selected file (or ".").
fn source_dir(selected_files: &[PathBuf]) -> String {
    selected_files
        .first()
        .and_then(|p| p.parent())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string())
}

/// Recreate the output directory: the recipe's output node directory param
/// (with {{ctx.*}} + {{node.*}} resolved), else a per-recipe temp dir.
fn fresh_output_dir(
    job: &BridgeJob,
    ctx: &NativeContext,
    files: &[bnto_core::PipelineFile],
) -> String {
    let recipe_dir = serde_json::from_str::<PipelineDefinition>(&job.definition_json)
        .ok()
        .and_then(|def| {
            let dir = bnto_core::resolve_output_directory(&def)?;
            let resolved = bnto_core::resolve_ctx_templates(&dir, ctx);
            let node_outputs = bnto_core::build_node_outputs_for_input(&def.nodes, files);
            Some(bnto_core::resolve_node_templates(&resolved, &node_outputs))
        });
    let output_dir = match recipe_dir {
        Some(dir) => PathBuf::from(dir),
        None => std::env::temp_dir().join(format!("bnto-tui-{}", job.slug)),
    };
    let _ = std::fs::remove_dir_all(&output_dir);
    let _ = std::fs::create_dir_all(&output_dir);
    output_dir.to_string_lossy().into_owned()
}

/// A reporter that forwards engine events to the TUI via the channel.
fn channel_reporter(tx: &mpsc::Sender<BridgeEvent>) -> PipelineReporter {
    let progress_tx = tx.clone();
    PipelineReporter::new(move |event| {
        let _ = progress_tx.send(BridgeEvent::Progress(event));
    })
}

/// Preview mode: extract file previews, skip writing.
fn send_preview_done(
    tx: &mpsc::Sender<BridgeEvent>,
    output_dir: String,
    result: bnto_core::pipeline::PipelineResult,
) {
    let previews = crate::dry_run::files::extract_previews(&result);
    let _ = tx.send(BridgeEvent::Done {
        output_dir,
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
        file_metadata: vec![],
        warnings: result.warnings,
        preview_data: previews,
    });
}

/// Report a finished, written run.
fn send_done(
    tx: &mpsc::Sender<BridgeEvent>,
    output_dir: String,
    result: bnto_core::pipeline::PipelineResult,
    file_metadata: Vec<FileResultMeta>,
) {
    let _ = tx.send(BridgeEvent::Done {
        output_dir,
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
        file_metadata,
        warnings: result.warnings,
        preview_data: vec![],
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

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::FileData;
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
                data: FileData::Bytes(vec![]),
                mime_type: "image/jpeg".into(),
                metadata,
            }],
            duration_ms: 100,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let meta = extract_file_metadata(&result);
        assert_eq!(meta.len(), 1);
        assert_eq!(meta[0].name, "photo.jpg");
        assert_eq!(meta[0].original_size, Some(780_000));
    }

    fn result_with(metadata: serde_json::Value) -> bnto_core::pipeline::PipelineResult {
        bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "thumb.webp".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "image/webp".into(),
                metadata: metadata.as_object().unwrap().clone(),
            }],
            duration_ms: 100,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        }
    }

    #[test]
    fn extract_metadata_prefers_the_input_size() {
        // resize → convert: convert's originalSize is the resized image.
        let result = result_with(serde_json::json!({ "originalSize": 300, "inputSize": 900 }));
        assert_eq!(extract_file_metadata(&result)[0].original_size, Some(900));
    }

    #[test]
    fn extract_metadata_ignores_an_input_size_alone() {
        // No node reported sizes (a rename), so no comparison is shown.
        let result = result_with(serde_json::json!({ "inputSize": 900 }));
        assert_eq!(extract_file_metadata(&result)[0].original_size, None);
    }

    #[test]
    fn extract_metadata_handles_missing() {
        let result = bnto_core::pipeline::PipelineResult {
            files: vec![bnto_core::pipeline::PipelineFileResult {
                name: "file.txt".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "text/plain".into(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 50,
            warnings: Vec::new(),
            node_metadata: Default::default(),
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
                data: FileData::Bytes(vec![]),
                mime_type: "text/plain".into(),
                metadata,
            }],
            duration_ms: 50,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let meta = extract_file_metadata(&result);
        assert_eq!(meta[0].original_size, None);
    }

    #[test]
    fn build_output_files_merges_metadata() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        fs::write(dir.join("photo.jpg"), vec![0u8; 290]).unwrap();

        let meta = vec![FileResultMeta {
            name: "photo.jpg".into(),
            original_size: Some(780),
        }];
        let outputs = build_output_files(&dir.to_string_lossy(), &meta);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].original_size, Some(780));
        assert_eq!(outputs[0].size_bytes, 290);
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
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        fs::write(dir.join("other.txt"), vec![0u8; 100]).unwrap();

        let meta = vec![FileResultMeta {
            name: "different.txt".into(),
            original_size: Some(500),
        }];
        let outputs = build_output_files(&dir.to_string_lossy(), &meta);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].original_size, None);
    }

    #[test]
    fn write_progressive_files_copies_a_path_and_keeps_the_source() {
        // Guards: an argument file passes through as a path to the user's
        // own file, so writing it out must copy, never move it.
        let temp = tempfile::tempdir().unwrap();
        let src = temp.path().join("original.mp4");
        fs::write(&src, b"video").unwrap();
        let out = temp.path().join("out");
        fs::create_dir(&out).unwrap();

        let files = [bnto_core::events::ProgressOutputFile {
            name: "renamed.mp4".into(),
            data: FileData::Path(src.clone()),
            mime_type: "video/mp4".into(),
        }];
        write_progressive_files(&out.to_string_lossy(), &files);

        assert_eq!(fs::read(out.join("renamed.mp4")).unwrap(), b"video");
        assert!(src.exists(), "source must survive a progressive write");
    }
}
