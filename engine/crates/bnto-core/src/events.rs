// Pipeline events — structured progress reporting for multi-node pipelines.
// Complements `progress.rs` (per-file within one node) with pipeline-level
// events (node started/completed, file progress, pipeline result).

use std::sync::Arc;

use serde::Serialize;

// =============================================================================
// Pipeline Event Types
// =============================================================================

/// Lightweight node metadata included in `PipelineStarted` so the UI
/// can show real node ids and types immediately (instead of "node-0").
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub id: String,
    pub node_type: String,
}

/// A single output file attached to an `IterationCompleted` event
/// when progressive output is enabled. Lets consumers write files
/// to disk immediately instead of waiting for the full pipeline.
///
/// The `data` field is `#[serde(skip)]` so WASM JSON serialization
/// doesn't try to serialize multi-GB video files.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressOutputFile {
    pub name: String,
    #[serde(skip)]
    pub data: crate::processor::FileData,
    pub mime_type: String,
}

/// Every event the pipeline executor can emit during execution.
///
/// Serialized as a tagged union with `"type"` discriminant and camelCase
/// field names, matching the TypeScript `PipelineEvent` discriminated union.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum PipelineEvent {
    /// Emitted once at the very start of pipeline execution.
    /// Tells the UI how many nodes and files to expect.
    #[serde(rename_all = "camelCase")]
    PipelineStarted {
        /// How many processing nodes will run (excludes I/O markers).
        total_nodes: usize,
        /// How many input files were provided.
        total_files: usize,
        /// Metadata for each processing node — lets the UI populate
        /// node types immediately instead of waiting for NodeStarted.
        nodes: Vec<NodeInfo>,
    },

    /// Emitted when a processing node begins execution.
    /// The editor uses this to highlight the active node on the canvas.
    #[serde(rename_all = "camelCase")]
    NodeStarted {
        /// The unique ID of the node (from the recipe definition).
        node_id: String,
        /// Zero-based index of this node in the processing sequence.
        node_index: usize,
        /// Total number of processing nodes in the pipeline.
        total_nodes: usize,
        /// The type of node (e.g., "image-compress", "spreadsheet-clean").
        node_type: String,
        /// If this node is inside a container (loop/group), the container's ID.
        /// `None` for top-level nodes.
        parent_node_id: Option<String>,
    },

    /// Emitted at the start of each loop iteration.
    /// Powers iteration count display (e.g., "3/15").
    #[serde(rename_all = "camelCase")]
    IterationStarted {
        /// The loop container node that owns this iteration.
        node_id: String,
        /// Zero-based iteration index.
        iteration: usize,
        /// Total number of iterations in this loop.
        total_iterations: usize,
    },

    /// Emitted after a loop iteration completes successfully.
    /// Powers per-iteration progress (e.g., "3/15 done") and progressive output.
    #[serde(rename_all = "camelCase")]
    IterationCompleted {
        /// The loop container node that owns this iteration.
        node_id: String,
        /// Zero-based iteration index.
        iteration: usize,
        /// Total number of iterations in this loop.
        total_iterations: usize,
        /// How long this iteration took, in milliseconds.
        duration_ms: u64,
        /// How many output files this iteration produced.
        files_produced: usize,
        /// Output files from this iteration (progressive mode only).
        /// Empty when `outputPersistence` is `deferred` (default).
        /// Serde-skipped so WASM doesn't serialize multi-GB file data.
        #[serde(skip)]
        output_files: Vec<ProgressOutputFile>,
    },

    /// Emitted when a loop iteration fails.
    /// In `continue` mode, the loop keeps going after emitting this.
    #[serde(rename_all = "camelCase")]
    IterationFailed {
        /// The loop container node that owns this iteration.
        node_id: String,
        /// Zero-based iteration index.
        iteration: usize,
        /// Total number of iterations in this loop.
        total_iterations: usize,
        /// Human-readable error message.
        error: String,
    },

    /// Emitted during file processing within a node.
    /// Powers progress bars and per-file status indicators.
    #[serde(rename_all = "camelCase")]
    FileProgress {
        /// Which node is processing this file.
        node_id: String,
        /// Zero-based index of the current file within this node's batch.
        file_index: usize,
        /// Total files this node will process.
        total_files: usize,
        /// Processing progress for this specific file (0-100).
        percent: u32,
        /// Human-readable status message (e.g., "Compressing photo.jpg...").
        message: String,
    },

    /// Emitted when a node finishes successfully.
    /// The editor uses this to mark the node as "completed" with a checkmark.
    #[serde(rename_all = "camelCase")]
    NodeCompleted {
        /// Which node completed.
        node_id: String,
        /// How long this node took, in milliseconds.
        duration_ms: u64,
        /// How many files this node processed.
        files_processed: usize,
        /// If this node is inside a container, the container's ID.
        parent_node_id: Option<String>,
    },

    /// Emitted when a node fails.
    /// The editor uses this to mark the node as "failed" with an error icon.
    #[serde(rename_all = "camelCase")]
    NodeFailed {
        /// Which node failed.
        node_id: String,
        /// Human-readable error message.
        error: String,
        /// If this node is inside a container, the container's ID.
        parent_node_id: Option<String>,
    },

    /// Emitted once when the entire pipeline finishes successfully.
    #[serde(rename_all = "camelCase")]
    PipelineCompleted {
        /// Total wall-clock time for the entire pipeline, in milliseconds.
        duration_ms: u64,
        /// Total number of files processed across all nodes.
        total_files_processed: usize,
    },

    /// Emitted when the pipeline fails (a node error stops execution).
    #[serde(rename_all = "camelCase")]
    PipelineFailed {
        /// Which node caused the pipeline to fail.
        node_id: String,
        /// Human-readable error message.
        error: String,
    },

    /// Emitted when a child process writes a line to stderr during execution.
    /// Used for live progress from tools like yt-dlp and ffmpeg.
    #[serde(rename_all = "camelCase")]
    CommandOutput {
        /// Which node's command produced this output.
        node_id: String,
        /// One line of stderr output from the child process.
        line: String,
    },
}

// =============================================================================
// Pipeline Reporter
// =============================================================================

/// Emits structured pipeline events to a callback.
///
/// Uses `Arc` internally so it can be cloned and shared — the executor
/// clones the reporter to move into closures that relay command output.
pub struct PipelineReporter {
    /// The callback that receives events. `None` = no-op mode.
    callback: Option<Arc<dyn Fn(PipelineEvent)>>,
}

impl Clone for PipelineReporter {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

impl PipelineReporter {
    /// Create a new reporter with a callback that receives pipeline events.
    pub fn new(callback: impl Fn(PipelineEvent) + 'static) -> Self {
        Self {
            callback: Some(Arc::new(callback)),
        }
    }

    /// Create a no-op reporter that discards all events.
    /// Used in tests where we don't need event tracking.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    /// Emit a pipeline event. If no callback is set (no-op mode),
    /// the event is silently discarded.
    pub fn emit(&self, event: PipelineEvent) {
        if let Some(cb) = &self.callback {
            cb(event);
        }
    }
}

// =============================================================================
// Recording Reporter (for tests)
// =============================================================================

/// A reporter that records all events in a thread-safe Vec.
/// Used in tests to verify the executor emits the right events
/// in the right order.
#[cfg(test)]
pub struct RecordingReporter {
    /// The shared, thread-safe event log.
    events: std::sync::Arc<std::sync::Mutex<Vec<PipelineEvent>>>,
}

#[cfg(test)]
impl Default for RecordingReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl RecordingReporter {
    /// Create a new recording reporter.
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Build a `PipelineReporter` that records events into this recorder.
    pub fn reporter(&self) -> PipelineReporter {
        let events = std::sync::Arc::clone(&self.events);
        PipelineReporter::new(move |event| {
            events.lock().unwrap().push(event);
        })
    }

    /// Get a snapshot of all recorded events.
    pub fn events(&self) -> Vec<PipelineEvent> {
        self.events.lock().unwrap().clone()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Serialization Tests ---
    // Verify the JSON shape matches what the TypeScript side expects.

    #[test]
    fn test_pipeline_started_serializes_correctly() {
        let event = PipelineEvent::PipelineStarted {
            total_nodes: 3,
            total_files: 10,
            nodes: vec![
                NodeInfo {
                    id: "n1".into(),
                    node_type: "image-compress".into(),
                },
                NodeInfo {
                    id: "n2".into(),
                    node_type: "file-rename".into(),
                },
                NodeInfo {
                    id: "n3".into(),
                    node_type: "image-resize".into(),
                },
            ],
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "PipelineStarted");
        assert_eq!(json["totalNodes"], 3);
        assert_eq!(json["totalFiles"], 10);
        let nodes = json["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0]["id"], "n1");
        assert_eq!(nodes[0]["nodeType"], "image-compress");
    }

    #[test]
    fn test_node_started_serializes_correctly() {
        let event = PipelineEvent::NodeStarted {
            node_id: "node-1".to_string(),
            node_index: 0,
            total_nodes: 3,
            node_type: "image-compress".to_string(),
            parent_node_id: None,
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "NodeStarted");
        assert_eq!(json["nodeId"], "node-1");
        assert_eq!(json["nodeIndex"], 0);
        assert_eq!(json["totalNodes"], 3);
        assert_eq!(json["nodeType"], "image-compress");
        assert_eq!(json["parentNodeId"], serde_json::Value::Null);
    }

    #[test]
    fn test_node_started_with_parent_serializes_correctly() {
        let event = PipelineEvent::NodeStarted {
            node_id: "child-1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "shell-command".to_string(),
            parent_node_id: Some("loop-1".to_string()),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "NodeStarted");
        assert_eq!(json["parentNodeId"], "loop-1");
    }

    #[test]
    fn test_iteration_started_serializes_correctly() {
        let event = PipelineEvent::IterationStarted {
            node_id: "loop-1".to_string(),
            iteration: 2,
            total_iterations: 15,
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "IterationStarted");
        assert_eq!(json["nodeId"], "loop-1");
        assert_eq!(json["iteration"], 2);
        assert_eq!(json["totalIterations"], 15);
    }

    #[test]
    fn test_iteration_completed_serializes_correctly() {
        let event = PipelineEvent::IterationCompleted {
            node_id: "loop-1".to_string(),
            iteration: 2,
            total_iterations: 7,
            duration_ms: 1500,
            files_produced: 1,
            output_files: Vec::new(),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "IterationCompleted");
        assert_eq!(json["nodeId"], "loop-1");
        assert_eq!(json["iteration"], 2);
        assert_eq!(json["totalIterations"], 7);
        assert_eq!(json["durationMs"], 1500);
        assert_eq!(json["filesProduced"], 1);
    }

    #[test]
    fn test_iteration_completed_skips_output_files_in_json() {
        let event = PipelineEvent::IterationCompleted {
            node_id: "loop-1".to_string(),
            iteration: 0,
            total_iterations: 1,
            duration_ms: 100,
            files_produced: 1,
            output_files: vec![ProgressOutputFile {
                name: "video.mp4".to_string(),
                data: crate::processor::FileData::Bytes(vec![0u8; 100]),
                mime_type: "video/mp4".to_string(),
            }],
        };
        let json = serde_json::to_value(&event).unwrap();

        // output_files is serde(skip) — should NOT appear in JSON.
        assert!(json.get("outputFiles").is_none());
        assert_eq!(json["filesProduced"], 1);
    }

    #[test]
    fn test_progress_output_file_serializes_without_data() {
        let file = ProgressOutputFile {
            name: "video.mp4".to_string(),
            data: crate::processor::FileData::Bytes(vec![0u8; 1024]),
            mime_type: "video/mp4".to_string(),
        };
        let json = serde_json::to_value(&file).unwrap();

        assert_eq!(json["name"], "video.mp4");
        assert_eq!(json["mimeType"], "video/mp4");
        // data is serde(skip) — should NOT appear in JSON.
        assert!(json.get("data").is_none());
    }

    #[test]
    fn test_iteration_failed_serializes_correctly() {
        let event = PipelineEvent::IterationFailed {
            node_id: "loop-1".to_string(),
            iteration: 3,
            total_iterations: 10,
            error: "Download timed out".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "IterationFailed");
        assert_eq!(json["nodeId"], "loop-1");
        assert_eq!(json["iteration"], 3);
        assert_eq!(json["totalIterations"], 10);
        assert_eq!(json["error"], "Download timed out");
    }

    #[test]
    fn test_file_progress_serializes_correctly() {
        let event = PipelineEvent::FileProgress {
            node_id: "node-2".to_string(),
            file_index: 2,
            total_files: 5,
            percent: 75,
            message: "Compressing photo.jpg...".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "FileProgress");
        assert_eq!(json["nodeId"], "node-2");
        assert_eq!(json["fileIndex"], 2);
        assert_eq!(json["totalFiles"], 5);
        assert_eq!(json["percent"], 75);
        assert_eq!(json["message"], "Compressing photo.jpg...");
    }

    #[test]
    fn test_node_completed_serializes_correctly() {
        let event = PipelineEvent::NodeCompleted {
            node_id: "node-1".to_string(),
            duration_ms: 1234,
            files_processed: 5,
            parent_node_id: None,
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "NodeCompleted");
        assert_eq!(json["nodeId"], "node-1");
        assert_eq!(json["durationMs"], 1234);
        assert_eq!(json["filesProcessed"], 5);
    }

    #[test]
    fn test_node_failed_serializes_correctly() {
        let event = PipelineEvent::NodeFailed {
            node_id: "node-3".to_string(),
            error: "Unsupported format: BMP".to_string(),
            parent_node_id: None,
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "NodeFailed");
        assert_eq!(json["nodeId"], "node-3");
        assert_eq!(json["error"], "Unsupported format: BMP");
    }

    #[test]
    fn test_pipeline_completed_serializes_correctly() {
        let event = PipelineEvent::PipelineCompleted {
            duration_ms: 5678,
            total_files_processed: 10,
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "PipelineCompleted");
        assert_eq!(json["durationMs"], 5678);
        assert_eq!(json["totalFilesProcessed"], 10);
    }

    #[test]
    fn test_pipeline_failed_serializes_correctly() {
        let event = PipelineEvent::PipelineFailed {
            node_id: "node-2".to_string(),
            error: "Processing failed: out of memory".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "PipelineFailed");
        assert_eq!(json["nodeId"], "node-2");
        assert_eq!(json["error"], "Processing failed: out of memory");
    }

    #[test]
    fn test_command_output_serializes_correctly() {
        let event = PipelineEvent::CommandOutput {
            node_id: "node-0".to_string(),
            line: "[download]  34.2% of ~150MiB".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "CommandOutput");
        assert_eq!(json["nodeId"], "node-0");
        assert_eq!(json["line"], "[download]  34.2% of ~150MiB");
    }

    #[test]
    fn test_pipeline_reporter_is_cloneable() {
        let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = std::sync::Arc::clone(&received);

        let reporter = PipelineReporter::new(move |event| {
            received_clone.lock().unwrap().push(event);
        });
        let reporter2 = reporter.clone();

        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
            nodes: vec![],
        });
        reporter2.emit(PipelineEvent::CommandOutput {
            node_id: "n".to_string(),
            line: "hello".to_string(),
        });

        let events = received.lock().unwrap();
        assert_eq!(events.len(), 2);
    }

    // --- Reporter Tests ---

    #[test]
    fn test_noop_reporter_doesnt_panic() {
        // No-op reporter should silently discard all events.
        let reporter = PipelineReporter::new_noop();
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
            nodes: vec![],
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 100,
            total_files_processed: 1,
        });
        // No panic = success.
    }

    #[test]
    fn test_recording_reporter_captures_events() {
        // Create a recording reporter and emit some events.
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 2,
            total_files: 3,
            nodes: vec![],
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 2,
            node_type: "image-compress".to_string(),
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 500,
            total_files_processed: 3,
        });

        // Verify all 3 events were captured in order.
        let events = recorder.events();
        assert_eq!(events.len(), 3);

        // Check types using pattern matching.
        assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));
        assert!(matches!(events[1], PipelineEvent::NodeStarted { .. }));
        assert!(matches!(events[2], PipelineEvent::PipelineCompleted { .. }));
    }

    #[test]
    fn test_reporter_calls_callback() {
        // Verify the callback receives the exact event we emit.
        let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = std::sync::Arc::clone(&received);

        let reporter = PipelineReporter::new(move |event| {
            received_clone.lock().unwrap().push(event);
        });

        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
            nodes: vec![],
        });

        let events = received.lock().unwrap();
        assert_eq!(events.len(), 1);
        if let PipelineEvent::PipelineStarted {
            total_nodes,
            total_files,
            ..
        } = &events[0]
        {
            assert_eq!(*total_nodes, 1);
            assert_eq!(*total_files, 1);
        } else {
            panic!("Expected PipelineStarted event");
        }
    }
}
