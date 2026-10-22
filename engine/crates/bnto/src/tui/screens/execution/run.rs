// Execution pipeline-level transitions — start, finish, outputs, command output.

use super::super::results::OutputFile;
use super::{
    ExecutionModel, ExecutionStatus, FileProgress, FileStatus, MAX_OUTPUT_LINES, NodeProgress,
    NodeStatus,
};

impl ExecutionModel {
    /// Initialize file and node tracking for a freshly started pipeline.
    pub(super) fn start_pipeline(
        &mut self,
        total_nodes: usize,
        total_files: usize,
        node_info: Vec<(String, String)>,
    ) {
        self.status = ExecutionStatus::Running;
        self.files = (0..total_files)
            .map(|i| FileProgress {
                name: self.waiting_name(i),
                percent: 0,
                status: FileStatus::Waiting,
            })
            .collect();
        self.nodes = initial_nodes(total_nodes, node_info);
        for node in &mut self.nodes {
            node.children = super::plan::container_steps(&self.definition_json, &node.id)
                .into_iter()
                .map(|(id, node_type)| NodeProgress::new(id, node_type, NodeStatus::Pending))
                .collect();
        }
    }

    /// The `i`th selected file's name, or "file N" when it isn't known.
    pub(super) fn waiting_name(&self, i: usize) -> String {
        self.selected_files
            .get(i)
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("file {}", i + 1))
    }

    /// Mark the pipeline complete and every unfinished file as done.
    pub(super) fn complete_pipeline(&mut self, duration_ms: u64) {
        self.status = ExecutionStatus::Completed;
        self.elapsed_ms = duration_ms;
        for file in &mut self.files {
            if file.status == FileStatus::Processing || file.status == FileStatus::Waiting {
                file.status = FileStatus::Done;
                file.percent = 100;
            }
        }
    }

    /// Mark the pipeline failed with `error`.
    pub(super) fn fail_pipeline(&mut self, error: String) {
        self.status = ExecutionStatus::Failed;
        self.error = Some(error);
    }

    /// Store the files the pipeline wrote, plus any non-fatal warnings.
    pub(super) fn set_outputs(
        &mut self,
        files: Vec<OutputFile>,
        output_dir: Option<String>,
        warnings: Vec<String>,
    ) {
        self.output_files = files;
        self.output_dir = output_dir;
        self.warnings = warnings;
    }

    /// Append a line of command stderr to the output pane.
    pub(super) fn push_command_output(&mut self, line: &str) {
        // Tools like patreon-dl colorize unconditionally — strip escape
        // codes so the output pane shows text, not literal `[32m…`.
        self.push_output_line(crate::tui::strip_ansi::strip_ansi(line));
    }

    /// Show a node's live status in the output pane. Updates from the same
    /// node replace each other, so a long walk is one moving line.
    pub(super) fn show_live_line(&mut self, node_id: String, message: String) {
        if self.live_line_node.as_ref() == Some(&node_id) {
            self.output_lines.pop_back();
        }
        self.push_output_line(message);
        self.live_line_node = Some(node_id);
    }

    /// Append to the rolling output window, dropping the oldest lines.
    pub(super) fn push_output_line(&mut self, line: String) {
        self.live_line_node = None;
        self.output_lines.push_back(line);
        while self.output_lines.len() > MAX_OUTPUT_LINES {
            self.output_lines.pop_front();
        }
    }
}

/// Use real node metadata when it covers every node, else placeholders.
fn initial_nodes(total_nodes: usize, node_info: Vec<(String, String)>) -> Vec<NodeProgress> {
    if node_info.len() == total_nodes {
        node_info
            .into_iter()
            .map(|(id, node_type)| NodeProgress::new(id, node_type, NodeStatus::Pending))
            .collect()
    } else {
        (0..total_nodes)
            .map(|i| NodeProgress::new(format!("node-{}", i), String::new(), NodeStatus::Pending))
            .collect()
    }
}
