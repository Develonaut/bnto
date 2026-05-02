// Execution screen — shows live pipeline progress with per-file and per-node status.
//
// TEA pattern: ExecutionModel (state) + ExecutionMessage (events) + update() (pure transitions).

use std::collections::{HashMap, VecDeque};
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
    /// Number of files processed so far (updated by FileProgress events).
    pub files_processed: usize,
    /// Total files this node will process (set on first FileProgress).
    pub total_files: usize,
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
    /// Raw definition JSON for the pipeline bridge.
    pub definition_json: String,
    /// Output files populated after pipeline completion.
    pub output_files: Vec<OutputFile>,
    /// Directory where output files were written.
    pub output_dir: Option<String>,
    /// Rolling window of recent command output lines (stderr from child processes).
    pub output_lines: VecDeque<String>,
}

/// Maximum number of command output lines to retain in the rolling window.
const MAX_OUTPUT_LINES: usize = 50;

/// Messages the execution screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExecutionMessage {
    /// Pipeline started — initialize file and node tracking.
    PipelineStarted {
        total_nodes: usize,
        total_files: usize,
        /// Pre-populated node metadata (id, type) so the UI shows real
        /// node types immediately instead of placeholder "node-0".
        node_info: Vec<(String, String)>,
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
    /// A line of stderr output from a running command.
    CommandOutput { node_id: String, line: String },
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
            definition_json: String::new(),
            output_files: Vec::new(),
            output_dir: None,
            output_lines: VecDeque::new(),
        }
    }

    /// Create an execution model pre-loaded with input files, overrides, and definition.
    pub fn with_inputs(
        slug: &str,
        selected_files: Vec<PathBuf>,
        param_overrides: HashMap<String, String>,
        definition_json: String,
    ) -> Self {
        Self {
            selected_files,
            param_overrides,
            definition_json,
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
            node_info,
        } => {
            model.status = ExecutionStatus::Running;
            model.files = (0..total_files)
                .map(|i| FileProgress {
                    name: format!("file {}", i + 1),
                    percent: 0,
                    status: FileStatus::Waiting,
                })
                .collect();
            // Use real node metadata when available, fall back to placeholders.
            model.nodes = if node_info.len() == total_nodes {
                node_info
                    .into_iter()
                    .map(|(id, node_type)| NodeProgress {
                        id,
                        node_type,
                        status: NodeStatus::Pending,
                        files_processed: 0,
                        total_files: 0,
                    })
                    .collect()
            } else {
                (0..total_nodes)
                    .map(|i| NodeProgress {
                        id: format!("node-{}", i),
                        node_type: String::new(),
                        status: NodeStatus::Pending,
                        files_processed: 0,
                        total_files: 0,
                    })
                    .collect()
            };
        }
        ExecutionMessage::NodeStarted { node_id, node_type } => {
            if let Some(node) = model.nodes.iter_mut().find(|n| n.id == node_id) {
                node.status = NodeStatus::Active;
                node.node_type = node_type;
            }
        }
        ExecutionMessage::FileProgress {
            node_id,
            file_index,
            total_files,
            percent,
            message,
        } => {
            if let Some(file) = model.files.get_mut(file_index) {
                file.percent = percent;
                file.status = FileStatus::Processing;
                if !message.is_empty() {
                    file.name = message;
                }
            }
            // Track per-node file progress for inline count display.
            if let Some(node) = model.nodes.iter_mut().find(|n| n.id == node_id) {
                node.total_files = total_files;
                node.files_processed = file_index + 1;
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
        ExecutionMessage::CommandOutput { line, .. } => {
            model.output_lines.push_back(line);
            while model.output_lines.len() > MAX_OUTPUT_LINES {
                model.output_lines.pop_front();
            }
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
                node_info: vec![],
            },
        );
        assert_eq!(m.status, ExecutionStatus::Running);
        assert_eq!(m.files.len(), 3);
        assert_eq!(m.nodes.len(), 2);
        assert!(m.files.iter().all(|f| f.status == FileStatus::Waiting));
        assert!(m.nodes.iter().all(|n| n.status == NodeStatus::Pending));
    }

    #[test]
    fn pipeline_started_uses_node_info_for_real_ids() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 2,
                total_files: 1,
                node_info: vec![
                    ("compress".into(), "image-compress".into()),
                    ("rename".into(), "file-rename".into()),
                ],
            },
        );
        assert_eq!(m.nodes.len(), 2);
        assert_eq!(m.nodes[0].id, "compress");
        assert_eq!(m.nodes[0].node_type, "image-compress");
        assert_eq!(m.nodes[1].id, "rename");
        assert_eq!(m.nodes[1].node_type, "file-rename");
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
                node_info: vec![],
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
                node_info: vec![],
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
                node_info: vec![],
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
                node_info: vec![],
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
                node_info: vec![],
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
                node_info: vec![],
            },
        );
        let m = update(m, ExecutionMessage::Cancel);
        assert_eq!(m.status, ExecutionStatus::Cancelled);
    }

    #[test]
    fn with_inputs_stores_files_and_overrides() {
        let files = vec![PathBuf::from("/a.jpg"), PathBuf::from("/b.png")];
        let mut overrides = HashMap::new();
        overrides.insert("compress:quality".into(), "60".into());
        let def = r#"{"nodes":[]}"#.to_string();
        let m = ExecutionModel::with_inputs("s", files.clone(), overrides.clone(), def.clone());
        assert_eq!(m.slug, "s");
        assert_eq!(m.status, ExecutionStatus::Idle);
        assert_eq!(m.selected_files, files);
        assert_eq!(m.param_overrides, overrides);
        assert_eq!(m.definition_json, def);
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
    fn command_output_populates_output_lines() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::CommandOutput {
                node_id: "n".into(),
                line: "[download] 50%".into(),
            },
        );
        assert_eq!(m.output_lines.len(), 1);
        assert_eq!(m.output_lines[0], "[download] 50%");
    }

    #[test]
    fn command_output_bounded_at_max() {
        let mut m = ExecutionModel::new("s");
        for i in 0..55 {
            m = update(
                m,
                ExecutionMessage::CommandOutput {
                    node_id: "n".into(),
                    line: format!("line {i}"),
                },
            );
        }
        assert_eq!(m.output_lines.len(), MAX_OUTPUT_LINES);
        // Oldest lines evicted — should start at line 5.
        assert_eq!(m.output_lines[0], "line 5");
        assert_eq!(m.output_lines[49], "line 54");
    }

    #[test]
    fn tick_updates_elapsed() {
        let m = ExecutionModel::new("s");
        let m = update(m, ExecutionMessage::Tick { elapsed_ms: 500 });
        assert_eq!(m.elapsed_ms, 500);
        let m = update(m, ExecutionMessage::Tick { elapsed_ms: 1000 });
        assert_eq!(m.elapsed_ms, 1000);
    }

    // --- Per-node file count tracking ---

    #[test]
    fn file_progress_tracks_per_node_counts() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 3,
                node_info: vec![("compress".into(), "image-compress".into())],
            },
        );
        let m = update(
            m,
            ExecutionMessage::NodeStarted {
                node_id: "compress".into(),
                node_type: "image-compress".into(),
            },
        );
        let m = update(
            m,
            ExecutionMessage::FileProgress {
                node_id: "compress".into(),
                file_index: 0,
                total_files: 3,
                percent: 100,
                message: "photo1.jpg".into(),
            },
        );
        assert_eq!(m.nodes[0].files_processed, 1);
        assert_eq!(m.nodes[0].total_files, 3);

        let m = update(
            m,
            ExecutionMessage::FileProgress {
                node_id: "compress".into(),
                file_index: 1,
                total_files: 3,
                percent: 50,
                message: "photo2.jpg".into(),
            },
        );
        assert_eq!(m.nodes[0].files_processed, 2);
        assert_eq!(m.nodes[0].total_files, 3);
    }

    #[test]
    fn node_progress_initializes_with_zero_counts() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 2,
                total_files: 1,
                node_info: vec![
                    ("a".into(), "image-compress".into()),
                    ("b".into(), "file-rename".into()),
                ],
            },
        );
        assert_eq!(m.nodes[0].files_processed, 0);
        assert_eq!(m.nodes[0].total_files, 0);
        assert_eq!(m.nodes[1].files_processed, 0);
        assert_eq!(m.nodes[1].total_files, 0);
    }
}
