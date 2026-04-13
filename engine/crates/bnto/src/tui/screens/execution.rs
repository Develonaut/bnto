// Execution screen — shows live pipeline progress with per-file and per-node status.
//
// TEA pattern: ExecutionModel (state) + ExecutionMessage (events) + update() (pure transitions).

use std::collections::HashMap;
use std::path::PathBuf;

use super::results::OutputFile;

/// Pipeline execution status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Idle,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Per-file processing status.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FileStatus {
    Waiting,
    Processing,
    Done,
    Failed(String),
}

/// Progress tracking for a single file.
#[derive(Debug, Clone)]
pub struct FileProgress {
    pub name: String,
    pub percent: u32,
    pub status: FileStatus,
}

/// Per-node processing status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    Pending,
    Active,
    Completed { duration_ms: u64 },
    Failed(String),
}

/// Progress tracking for a single node.
#[derive(Debug, Clone)]
pub struct NodeProgress {
    pub id: String,
    pub node_type: String,
    pub status: NodeStatus,
}

/// Execution screen state.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ExecutionModel {
    pub slug: String,
    pub status: ExecutionStatus,
    pub files: Vec<FileProgress>,
    pub nodes: Vec<NodeProgress>,
    pub elapsed_ms: u64,
    pub error: Option<String>,
    /// Input files selected by the user in the picker.
    pub selected_files: Vec<PathBuf>,
    /// Param overrides from the detail screen configuration.
    pub param_overrides: HashMap<String, String>,
    /// Output files populated after pipeline completion.
    pub output_files: Vec<OutputFile>,
    /// Directory where output files were written.
    pub output_dir: Option<String>,
}

/// Messages the execution screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExecutionMessage {
    /// Pipeline started — initialize file and node tracking.
    PipelineStarted {
        total_nodes: usize,
        total_files: usize,
    },
    /// A processing node began execution.
    NodeStarted { node_id: String, node_type: String },
    /// Progress update for a file within a node.
    FileProgress {
        node_id: String,
        file_index: usize,
        total_files: usize,
        percent: u32,
        message: String,
    },
    /// A node completed successfully.
    NodeCompleted { node_id: String, duration_ms: u64 },
    /// A node failed.
    NodeFailed { node_id: String, error: String },
    /// Entire pipeline completed.
    PipelineCompleted {
        duration_ms: u64,
        total_files_processed: usize,
    },
    /// Pipeline failed.
    PipelineFailed { node_id: String, error: String },
    /// Output files ready after pipeline writes results to disk.
    OutputsReady {
        files: Vec<OutputFile>,
        output_dir: Option<String>,
    },
    /// User pressed Esc to cancel.
    Cancel,
    /// Timer tick with current elapsed time.
    Tick { elapsed_ms: u64 },
}

impl ExecutionModel {
    /// Create a new idle execution model for the given recipe slug.
    pub fn new(slug: &str) -> Self {
        Self {
            slug: slug.to_string(),
            status: ExecutionStatus::Idle,
            files: Vec::new(),
            nodes: Vec::new(),
            elapsed_ms: 0,
            error: None,
            selected_files: Vec::new(),
            param_overrides: HashMap::new(),
            output_files: Vec::new(),
            output_dir: None,
        }
    }

    /// Create an execution model pre-loaded with input files and param overrides.
    pub fn with_inputs(
        slug: &str,
        selected_files: Vec<PathBuf>,
        param_overrides: HashMap<String, String>,
    ) -> Self {
        Self {
            selected_files,
            param_overrides,
            ..Self::new(slug)
        }
    }
}

/// Pure state transition for the execution screen.
pub fn update(mut model: ExecutionModel, msg: ExecutionMessage) -> ExecutionModel {
    match msg {
        ExecutionMessage::PipelineStarted {
            total_nodes,
            total_files,
        } => {
            model.status = ExecutionStatus::Running;
            model.files = (0..total_files)
                .map(|i| FileProgress {
                    name: format!("file {}", i + 1),
                    percent: 0,
                    status: FileStatus::Waiting,
                })
                .collect();
            model.nodes = (0..total_nodes)
                .map(|i| NodeProgress {
                    id: format!("node-{}", i),
                    node_type: String::new(),
                    status: NodeStatus::Pending,
                })
                .collect();
        }
        ExecutionMessage::NodeStarted { node_id, node_type } => {
            if let Some(node) = model.nodes.iter_mut().find(|n| n.id == node_id) {
                node.status = NodeStatus::Active;
                node.node_type = node_type;
            }
        }
        ExecutionMessage::FileProgress {
            file_index,
            percent,
            message,
            ..
        } => {
            if let Some(file) = model.files.get_mut(file_index) {
                file.percent = percent;
                file.status = FileStatus::Processing;
                if !message.is_empty() {
                    file.name = message;
                }
            }
        }
        ExecutionMessage::NodeCompleted {
            node_id,
            duration_ms,
        } => {
            if let Some(node) = model.nodes.iter_mut().find(|n| n.id == node_id) {
                node.status = NodeStatus::Completed { duration_ms };
            }
        }
        ExecutionMessage::NodeFailed { node_id, error } => {
            if let Some(node) = model.nodes.iter_mut().find(|n| n.id == node_id) {
                node.status = NodeStatus::Failed(error);
            }
        }
        ExecutionMessage::PipelineCompleted { duration_ms, .. } => {
            model.status = ExecutionStatus::Completed;
            model.elapsed_ms = duration_ms;
            // Mark all remaining files as done.
            for file in &mut model.files {
                if file.status == FileStatus::Processing || file.status == FileStatus::Waiting {
                    file.status = FileStatus::Done;
                    file.percent = 100;
                }
            }
        }
        ExecutionMessage::PipelineFailed { error, .. } => {
            model.status = ExecutionStatus::Failed;
            model.error = Some(error);
        }
        ExecutionMessage::OutputsReady { files, output_dir } => {
            model.output_files = files;
            model.output_dir = output_dir;
        }
        ExecutionMessage::Cancel => {
            model.status = ExecutionStatus::Cancelled;
        }
        ExecutionMessage::Tick { elapsed_ms } => {
            model.elapsed_ms = elapsed_ms;
        }
    }
    model
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_idle_model() {
        let m = ExecutionModel::new("compress-images");
        assert_eq!(m.slug, "compress-images");
        assert_eq!(m.status, ExecutionStatus::Idle);
        assert!(m.files.is_empty());
        assert!(m.nodes.is_empty());
        assert_eq!(m.elapsed_ms, 0);
        assert!(m.error.is_none());
    }

    #[test]
    fn pipeline_started_sets_running() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 2,
                total_files: 3,
            },
        );
        assert_eq!(m.status, ExecutionStatus::Running);
        assert_eq!(m.files.len(), 3);
        assert_eq!(m.nodes.len(), 2);
        assert!(m.files.iter().all(|f| f.status == FileStatus::Waiting));
        assert!(m.nodes.iter().all(|n| n.status == NodeStatus::Pending));
    }

    #[test]
    fn node_started_activates_node() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 2,
                total_files: 1,
            },
        );
        let m = update(
            m,
            ExecutionMessage::NodeStarted {
                node_id: "node-0".into(),
                node_type: "image-compress".into(),
            },
        );
        assert_eq!(m.nodes[0].status, NodeStatus::Active);
        assert_eq!(m.nodes[0].node_type, "image-compress");
        assert_eq!(m.nodes[1].status, NodeStatus::Pending);
    }

    #[test]
    fn file_progress_updates_percent() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 2,
            },
        );
        let m = update(
            m,
            ExecutionMessage::FileProgress {
                node_id: "n".into(),
                file_index: 0,
                total_files: 2,
                percent: 50,
                message: "photo.jpg".into(),
            },
        );
        assert_eq!(m.files[0].percent, 50);
        assert_eq!(m.files[0].status, FileStatus::Processing);
        assert_eq!(m.files[0].name, "photo.jpg");
        // Second file unchanged.
        assert_eq!(m.files[1].status, FileStatus::Waiting);
    }

    #[test]
    fn node_completed_marks_done() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
            },
        );
        let m = update(
            m,
            ExecutionMessage::NodeStarted {
                node_id: "node-0".into(),
                node_type: "image-compress".into(),
            },
        );
        let m = update(
            m,
            ExecutionMessage::NodeCompleted {
                node_id: "node-0".into(),
                duration_ms: 500,
            },
        );
        assert_eq!(
            m.nodes[0].status,
            NodeStatus::Completed { duration_ms: 500 }
        );
    }

    #[test]
    fn node_failed_captures_error() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
            },
        );
        let m = update(
            m,
            ExecutionMessage::NodeFailed {
                node_id: "node-0".into(),
                error: "unsupported format".into(),
            },
        );
        assert_eq!(
            m.nodes[0].status,
            NodeStatus::Failed("unsupported format".into())
        );
    }

    #[test]
    fn pipeline_completed_sets_status() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 2,
            },
        );
        let m = update(
            m,
            ExecutionMessage::PipelineCompleted {
                duration_ms: 1234,
                total_files_processed: 2,
            },
        );
        assert_eq!(m.status, ExecutionStatus::Completed);
        assert_eq!(m.elapsed_ms, 1234);
        // All files marked done.
        assert!(m.files.iter().all(|f| f.status == FileStatus::Done));
        assert!(m.files.iter().all(|f| f.percent == 100));
    }

    #[test]
    fn pipeline_failed_captures_error() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineFailed {
                node_id: "n".into(),
                error: "out of memory".into(),
            },
        );
        assert_eq!(m.status, ExecutionStatus::Failed);
        assert_eq!(m.error, Some("out of memory".into()));
    }

    #[test]
    fn cancel_sets_cancelled() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
            },
        );
        let m = update(m, ExecutionMessage::Cancel);
        assert_eq!(m.status, ExecutionStatus::Cancelled);
    }

    #[test]
    fn with_inputs_stores_files_and_overrides() {
        let files = vec![PathBuf::from("/a.jpg"), PathBuf::from("/b.png")];
        let mut overrides = HashMap::new();
        overrides.insert("compress.quality".into(), "60".into());
        let m = ExecutionModel::with_inputs("s", files.clone(), overrides.clone());
        assert_eq!(m.slug, "s");
        assert_eq!(m.status, ExecutionStatus::Idle);
        assert_eq!(m.selected_files, files);
        assert_eq!(m.param_overrides, overrides);
        assert!(m.output_files.is_empty());
        assert!(m.output_dir.is_none());
    }

    #[test]
    fn outputs_ready_stores_files_and_dir() {
        let m = ExecutionModel::new("s");
        let outputs = vec![OutputFile {
            name: "photo.jpg".into(),
            size_bytes: 290_000,
            original_size: Some(780_000),
        }];
        let m = update(
            m,
            ExecutionMessage::OutputsReady {
                files: outputs.clone(),
                output_dir: Some("/tmp/out".into()),
            },
        );
        assert_eq!(m.output_files, outputs);
        assert_eq!(m.output_dir, Some("/tmp/out".into()));
    }

    #[test]
    fn tick_updates_elapsed() {
        let m = ExecutionModel::new("s");
        let m = update(m, ExecutionMessage::Tick { elapsed_ms: 500 });
        assert_eq!(m.elapsed_ms, 500);
        let m = update(m, ExecutionMessage::Tick { elapsed_ms: 1000 });
        assert_eq!(m.elapsed_ms, 1000);
    }
}
