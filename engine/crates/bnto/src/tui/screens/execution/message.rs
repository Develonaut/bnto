// Execution screen messages — pipeline, node, iteration and UI events.

use super::super::results::OutputFile;

/// Messages the execution screen can handle.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ExecutionMessage {
    /// The user allowed an untrusted recipe's commands (handled by the app,
    /// which records the approval).
    AllowCommands,
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
        warnings: Vec<String>,
    },
    /// What a message-mode recipe reported, shown on the results screen.
    Reported {
        message: crate::run_message::RunMessage,
    },
    /// A line of stderr output from a running command.
    CommandOutput { node_id: String, line: String },
    /// A running node's live status ("Found 1204 files · photos").
    NodeProgress { node_id: String, message: String },
    /// Loop iteration started — tracks iteration count.
    IterationStarted {
        node_id: String,
        iteration: usize,
        total_iterations: usize,
    },
    /// Loop iteration completed successfully.
    IterationCompleted {
        node_id: String,
        iteration: usize,
        total_iterations: usize,
        duration_ms: u64,
        files_produced: usize,
        /// Output files from progressive mode (written to disk by the bridge).
        output_files: Vec<bnto_core::events::ProgressOutputFile>,
    },
    /// Loop iteration failed (in continue-on-error mode, loop continues).
    IterationFailed {
        node_id: String,
        iteration: usize,
        total_iterations: usize,
        error: String,
    },
    /// A child node inside a container started.
    ChildNodeStarted {
        parent_node_id: String,
        node_id: String,
        node_type: String,
    },
    /// A child node inside a container completed.
    ChildNodeCompleted {
        parent_node_id: String,
        node_id: String,
        duration_ms: u64,
    },
    /// User pressed Esc to cancel.
    Cancel,
    /// Timer tick with current elapsed time.
    Tick { elapsed_ms: u64 },
}
