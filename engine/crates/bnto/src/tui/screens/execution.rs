// Execution screen — shows live pipeline progress with per-file and per-node status.
//
// TEA pattern: ExecutionModel (state) + ExecutionMessage (events) + update() (pure transitions).

mod message;
mod nodes;
mod run;
mod update;

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use super::results::OutputFile;

pub use message::ExecutionMessage;
pub use update::update;

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
    /// Current iteration (0-based) for loop containers.
    pub iteration: Option<usize>,
    /// Total iterations for loop containers.
    pub total_iterations: Option<usize>,
    /// Child nodes inside this container (loop/group).
    pub children: Vec<NodeProgress>,
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
    /// Non-fatal warnings from the pipeline (e.g. skipped loop iterations).
    pub warnings: Vec<String>,
    /// When true, pipeline runs without writing output — results go to Preview screen.
    pub preview_mode: bool,
}

/// Maximum number of command output lines to retain in the rolling window.
const MAX_OUTPUT_LINES: usize = 50;

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
            warnings: Vec::new(),
            preview_mode: false,
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

    /// Create a preview-mode execution model — pipeline runs without writing output.
    pub fn with_preview(
        slug: &str,
        selected_files: Vec<PathBuf>,
        param_overrides: HashMap<String, String>,
        definition_json: String,
    ) -> Self {
        Self {
            preview_mode: true,
            ..Self::with_inputs(slug, selected_files, param_overrides, definition_json)
        }
    }
}

impl NodeProgress {
    /// A fresh node entry with no file or iteration progress yet.
    fn new(id: String, node_type: String, status: NodeStatus) -> Self {
        Self {
            id,
            node_type,
            status,
            files_processed: 0,
            total_files: 0,
            iteration: None,
            total_iterations: None,
            children: Vec::new(),
        }
    }
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
                warnings: Vec::new(),
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
    fn command_output_strips_ansi_codes() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::CommandOutput {
                node_id: "n".into(),
                line: "\u{1b}[32minfo:\u{1b}[39m PostDownloader: done".into(),
            },
        );
        assert_eq!(m.output_lines[0], "info: PostDownloader: done");
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

    // --- Iteration + child node tracking ---

    #[test]
    fn iteration_started_sets_iteration_count() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("loop-1".into(), "loop".into())],
            },
        );
        let m = update(
            m,
            ExecutionMessage::IterationStarted {
                node_id: "loop-1".into(),
                iteration: 2,
                total_iterations: 15,
            },
        );
        assert_eq!(m.nodes[0].iteration, Some(2));
        assert_eq!(m.nodes[0].total_iterations, Some(15));
    }

    #[test]
    fn iteration_started_resets_children() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("loop-1".into(), "loop".into())],
            },
        );
        // First iteration — child starts and completes.
        let m = update(
            m,
            ExecutionMessage::IterationStarted {
                node_id: "loop-1".into(),
                iteration: 0,
                total_iterations: 3,
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                node_type: "shell-command".into(),
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeCompleted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                duration_ms: 100,
            },
        );
        assert_eq!(
            m.nodes[0].children[0].status,
            NodeStatus::Completed { duration_ms: 100 }
        );

        // Second iteration — children should reset to Pending.
        let m = update(
            m,
            ExecutionMessage::IterationStarted {
                node_id: "loop-1".into(),
                iteration: 1,
                total_iterations: 3,
            },
        );
        assert_eq!(m.nodes[0].children[0].status, NodeStatus::Pending);
    }

    #[test]
    fn child_node_started_adds_to_parent() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("loop-1".into(), "loop".into())],
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "loop-1".into(),
                node_id: "child-a".into(),
                node_type: "shell-command".into(),
            },
        );
        assert_eq!(m.nodes[0].children.len(), 1);
        assert_eq!(m.nodes[0].children[0].id, "child-a");
        assert_eq!(m.nodes[0].children[0].node_type, "shell-command");
        assert_eq!(m.nodes[0].children[0].status, NodeStatus::Active);
    }

    #[test]
    fn child_node_started_upserts_existing() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("loop-1".into(), "loop".into())],
            },
        );
        // First time — creates child entry.
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                node_type: "shell-command".into(),
            },
        );
        // Second time — upserts, doesn't duplicate.
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                node_type: "shell-command".into(),
            },
        );
        assert_eq!(m.nodes[0].children.len(), 1);
        assert_eq!(m.nodes[0].children[0].status, NodeStatus::Active);
    }

    #[test]
    fn child_node_completed_marks_child_done() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("loop-1".into(), "loop".into())],
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                node_type: "transform".into(),
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeCompleted {
                parent_node_id: "loop-1".into(),
                node_id: "child".into(),
                duration_ms: 42,
            },
        );
        assert_eq!(
            m.nodes[0].children[0].status,
            NodeStatus::Completed { duration_ms: 42 }
        );
    }

    #[test]
    fn multiple_children_tracked_independently() {
        let m = ExecutionModel::new("s");
        let m = update(
            m,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
                node_info: vec![("group-1".into(), "group".into())],
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "group-1".into(),
                node_id: "a".into(),
                node_type: "edit-fields".into(),
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeCompleted {
                parent_node_id: "group-1".into(),
                node_id: "a".into(),
                duration_ms: 5,
            },
        );
        let m = update(
            m,
            ExecutionMessage::ChildNodeStarted {
                parent_node_id: "group-1".into(),
                node_id: "b".into(),
                node_type: "transform".into(),
            },
        );
        assert_eq!(m.nodes[0].children.len(), 2);
        assert_eq!(
            m.nodes[0].children[0].status,
            NodeStatus::Completed { duration_ms: 5 }
        );
        assert_eq!(m.nodes[0].children[1].status, NodeStatus::Active);
    }
}
