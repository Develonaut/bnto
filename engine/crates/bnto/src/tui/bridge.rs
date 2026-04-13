// Pipeline execution bridge — runs the engine on a background thread
// and relays PipelineEvent progress back to the TUI event loop via mpsc.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use bnto_core::events::{PipelineEvent, PipelineReporter};
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
    },
    /// Pipeline or setup failed with an error message.
    Error(String),
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
) -> mpsc::Receiver<BridgeEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        run_bridge(tx, &slug, &selected_files, &param_overrides);
    });

    rx
}

/// Inner bridge logic — runs synchronously on the spawned thread.
fn run_bridge(
    tx: mpsc::Sender<BridgeEvent>,
    slug: &str,
    selected_files: &[PathBuf],
    param_overrides: &HashMap<String, String>,
) {
    // Resolve the recipe from the embedded catalog.
    let recipe = match builtin_recipe_by_slug(slug) {
        Some(r) => r,
        None => {
            let _ = tx.send(BridgeEvent::Error(format!("Unknown recipe: {slug}")));
            return;
        }
    };

    // Convert file paths to string args for prepare_inputs.
    let args: Vec<String> = selected_files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    // Convert param overrides from HashMap to "key=value" strings.
    let override_args: Vec<String> = param_overrides
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

    // Create a temp output directory and write results.
    let output_dir = std::env::temp_dir().join(format!("bnto-tui-{slug}"));
    // Clean any previous run's output for this slug.
    let _ = std::fs::remove_dir_all(&output_dir);
    let output_dir_str = output_dir.to_string_lossy().into_owned();

    if let Err(e) = io::write_results(&result, &output_dir_str) {
        let _ = tx.send(BridgeEvent::Error(e));
        return;
    }

    let _ = tx.send(BridgeEvent::Done {
        output_dir: output_dir_str,
        file_count: result.files.len(),
        duration_ms: result.duration_ms,
    });
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
    }
}

/// Read output files from the bridge's output directory to build OutputFile list.
pub fn build_output_files(output_dir: &str, expected_count: usize) -> Vec<OutputFile> {
    let dir = std::path::Path::new(output_dir);
    let mut outputs = Vec::with_capacity(expected_count);
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
            outputs.push(OutputFile {
                name,
                size_bytes,
                original_size: None,
            });
        }
    }
    outputs
}
