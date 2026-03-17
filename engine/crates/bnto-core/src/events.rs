// Pipeline events — structured progress reporting for multi-node pipelines.
// Complements `progress.rs` (per-file within one node) with pipeline-level
// events (node started/completed, file progress, pipeline result).

use serde::Serialize;

// =============================================================================
// Pipeline Event Types
// =============================================================================

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
        /// The type of node (e.g., "image:compress", "spreadsheet:clean").
        node_type: String,
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
    },

    /// Emitted when a node fails.
    /// The editor uses this to mark the node as "failed" with an error icon.
    #[serde(rename_all = "camelCase")]
    NodeFailed {
        /// Which node failed.
        node_id: String,
        /// Human-readable error message.
        error: String,
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
}

// =============================================================================
// Pipeline Reporter
// =============================================================================

/// Emits structured pipeline events to a callback.
///
/// This is the pipeline-level equivalent of `ProgressReporter`.
/// The callback receives a `PipelineEvent` which can be serialized
/// to JSON and sent to the UI (via Web Worker postMessage, CLI stdout,
/// or any other transport).
pub struct PipelineReporter {
    /// The callback that receives events. `None` = no-op mode.
    callback: Option<Box<dyn Fn(PipelineEvent)>>,
}

impl PipelineReporter {
    /// Create a new reporter with a callback that receives pipeline events.
    ///
    /// USAGE:
    /// ```rust
    /// use bnto_core::PipelineReporter;
    ///
    /// let reporter = PipelineReporter::new(|event| {
    ///     println!("Pipeline event: {:?}", event);
    /// });
    /// ```
    pub fn new(callback: impl Fn(PipelineEvent) + 'static) -> Self {
        Self {
            callback: Some(Box::new(callback)),
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
        // Create an event and serialize it to JSON.
        let event = PipelineEvent::PipelineStarted {
            total_nodes: 3,
            total_files: 10,
        };
        let json = serde_json::to_value(&event).unwrap();

        // Check the JSON has the right shape:
        // { "type": "PipelineStarted", "totalNodes": 3, "totalFiles": 10 }
        assert_eq!(json["type"], "PipelineStarted");
        assert_eq!(json["totalNodes"], 3);
        assert_eq!(json["totalFiles"], 10);
    }

    #[test]
    fn test_node_started_serializes_correctly() {
        let event = PipelineEvent::NodeStarted {
            node_id: "node-1".to_string(),
            node_index: 0,
            total_nodes: 3,
            node_type: "image:compress".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "NodeStarted");
        assert_eq!(json["nodeId"], "node-1");
        assert_eq!(json["nodeIndex"], 0);
        assert_eq!(json["totalNodes"], 3);
        assert_eq!(json["nodeType"], "image:compress");
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

    // --- Reporter Tests ---

    #[test]
    fn test_noop_reporter_doesnt_panic() {
        // No-op reporter should silently discard all events.
        let reporter = PipelineReporter::new_noop();
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
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
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 2,
            node_type: "image:compress".to_string(),
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
        });

        let events = received.lock().unwrap();
        assert_eq!(events.len(), 1);
        if let PipelineEvent::PipelineStarted {
            total_nodes,
            total_files,
        } = &events[0]
        {
            assert_eq!(*total_nodes, 1);
            assert_eq!(*total_files, 1);
        } else {
            panic!("Expected PipelineStarted event");
        }
    }
}
