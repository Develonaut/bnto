// Pipeline events — structured progress reporting for multi-node pipelines.
//
// Relationship to ProgressReporter:
// - ProgressReporter (progress.rs) = per-file, within ONE node
// - PipelineReporter (this file) = pipeline-level, across ALL nodes
//
// The executor wraps PipelineReporter around ProgressReporter — converting
// per-file progress into structured FileProgress events.

use serde::Serialize;

// =============================================================================
// Pipeline Event Types
// =============================================================================

/// Every event the pipeline executor can emit during execution.
/// Serialized with `type` tag and camelCase fields to match TS discriminated unions.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum PipelineEvent {
    /// Pipeline execution starting. Reports total nodes and files.
    #[serde(rename_all = "camelCase")]
    PipelineStarted {
        total_nodes: usize,
        total_files: usize,
    },

    /// A processing node began execution.
    #[serde(rename_all = "camelCase")]
    NodeStarted {
        node_id: String,
        node_index: usize,
        total_nodes: usize,
        node_type: String,
    },

    /// Per-file progress within a node.
    #[serde(rename_all = "camelCase")]
    FileProgress {
        node_id: String,
        file_index: usize,
        total_files: usize,
        percent: u32,
        message: String,
    },

    /// A node finished successfully.
    #[serde(rename_all = "camelCase")]
    NodeCompleted {
        node_id: String,
        duration_ms: u64,
        files_processed: usize,
    },

    /// A node failed.
    #[serde(rename_all = "camelCase")]
    NodeFailed { node_id: String, error: String },

    /// Entire pipeline finished successfully.
    #[serde(rename_all = "camelCase")]
    PipelineCompleted {
        duration_ms: u64,
        total_files_processed: usize,
    },

    /// Pipeline failed (a node error stopped execution).
    #[serde(rename_all = "camelCase")]
    PipelineFailed { node_id: String, error: String },
}

// =============================================================================
// Pipeline Reporter
// =============================================================================

/// Emits structured pipeline events to a callback.
/// The callback can serialize events to JSON and send them via
/// Web Worker postMessage, CLI stdout, or any other transport.
pub struct PipelineReporter {
    callback: Option<Box<dyn Fn(PipelineEvent)>>,
}

impl PipelineReporter {
    pub fn new(callback: impl Fn(PipelineEvent) + 'static) -> Self {
        Self {
            callback: Some(Box::new(callback)),
        }
    }

    /// No-op reporter that discards all events. Used in tests.
    pub fn new_noop() -> Self {
        Self { callback: None }
    }

    pub fn emit(&self, event: PipelineEvent) {
        if let Some(cb) = &self.callback {
            cb(event);
        }
    }
}

// =============================================================================
// Recording Reporter (test-only)
// =============================================================================

/// Records all events in a thread-safe Vec for test assertions.
#[cfg(test)]
pub struct RecordingReporter {
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
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn reporter(&self) -> PipelineReporter {
        let events = std::sync::Arc::clone(&self.events);
        PipelineReporter::new(move |event| {
            events.lock().unwrap().push(event);
        })
    }

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
    // Verify JSON shape matches what the TypeScript side expects.

    #[test]
    fn test_pipeline_started_serializes_correctly() {
        let event = PipelineEvent::PipelineStarted {
            total_nodes: 3,
            total_files: 10,
        };
        let json = serde_json::to_value(&event).unwrap();

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
        let reporter = PipelineReporter::new_noop();
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 100,
            total_files_processed: 1,
        });
    }

    #[test]
    fn test_recording_reporter_captures_events() {
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

        let events = recorder.events();
        assert_eq!(events.len(), 3);

        assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));
        assert!(matches!(events[1], PipelineEvent::NodeStarted { .. }));
        assert!(matches!(events[2], PipelineEvent::PipelineCompleted { .. }));
    }

    #[test]
    fn test_reporter_calls_callback() {
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
