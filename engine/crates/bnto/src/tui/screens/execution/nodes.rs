// Execution node-level transitions — node status, file progress, container
// children and loop iterations.

use super::{ExecutionModel, FileStatus, NodeProgress, NodeStatus};

impl ExecutionModel {
    /// Find a top-level node by id.
    fn node_mut(&mut self, node_id: &str) -> Option<&mut NodeProgress> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    /// Mark a node active, recording its real type.
    pub(super) fn start_node(&mut self, node_id: &str, node_type: String) {
        if let Some(node) = self.node_mut(node_id) {
            node.status = NodeStatus::Active;
            node.node_type = node_type;
        }
    }

    /// Update a file's progress and the owning node's file count.
    pub(super) fn record_file_progress(
        &mut self,
        node_id: &str,
        file_index: usize,
        total_files: usize,
        percent: u32,
        message: String,
    ) {
        // Files run one after another, so one the run has moved past is done.
        for earlier in 0..file_index.min(self.files.len()) {
            if self.files[earlier].status == FileStatus::Processing {
                self.settle_file(earlier, FileStatus::Done);
            }
        }
        if let Some(file) = self.files.get_mut(file_index) {
            file.percent = percent;
            file.status = FileStatus::Processing;
            if !message.is_empty() {
                file.name = message;
            }
        }
        // Track per-node file progress for inline count display.
        if let Some(node) = self.node_mut(node_id) {
            node.total_files = total_files;
            node.files_processed = file_index + 1;
        }
    }

    /// Mark a node completed.
    pub(super) fn complete_node(&mut self, node_id: &str, duration_ms: u64) {
        if let Some(node) = self.node_mut(node_id) {
            node.status = NodeStatus::Completed { duration_ms };
        }
    }

    /// Mark a node failed.
    pub(super) fn fail_node(&mut self, node_id: &str, error: String) {
        if let Some(node) = self.node_mut(node_id) {
            node.status = NodeStatus::Failed(error);
        }
    }

    /// Mark a node skipped. A container's child never started, so it's
    /// added on first sight, like `start_child`.
    pub(super) fn skip_node(
        &mut self,
        parent_id: Option<&str>,
        node_id: String,
        node_type: String,
    ) {
        let Some(parent_id) = parent_id else {
            if let Some(node) = self.node_mut(&node_id) {
                node.status = NodeStatus::Skipped;
            }
            return;
        };
        self.start_child(parent_id, node_id.clone(), node_type);
        if let Some(parent) = self.node_mut(parent_id)
            && let Some(child) = parent.children.iter_mut().find(|c| c.id == node_id)
        {
            child.status = NodeStatus::Skipped;
        }
    }

    /// Mark a container child active, adding it on first sight.
    pub(super) fn start_child(&mut self, parent_id: &str, node_id: String, node_type: String) {
        let Some(parent) = self.node_mut(parent_id) else {
            return;
        };
        // Upsert: reuse existing child entry or create a new one.
        if let Some(child) = parent.children.iter_mut().find(|c| c.id == node_id) {
            child.status = NodeStatus::Active;
            child.node_type = node_type;
        } else {
            let child = NodeProgress::new(node_id, node_type, NodeStatus::Active);
            parent.children.push(child);
        }
    }

    /// Mark a container child completed.
    pub(super) fn complete_child(&mut self, parent_id: &str, node_id: &str, duration_ms: u64) {
        if let Some(parent) = self.node_mut(parent_id)
            && let Some(child) = parent.children.iter_mut().find(|c| c.id == node_id)
        {
            child.status = NodeStatus::Completed { duration_ms };
        }
    }

    /// Start a loop iteration, resetting the stale children of the last one.
    pub(super) fn start_iteration(&mut self, node_id: &str, iteration: usize, total: usize) {
        if let Some(node) = self.node_mut(node_id) {
            node.iteration = Some(iteration);
            node.total_iterations = Some(total);
            for child in &mut node.children {
                child.status = NodeStatus::Pending;
            }
        }
    }

    /// Update a loop's iteration counter. Progressive output files are
    /// written to disk by the bridge, not handled here.
    pub(super) fn set_iteration(&mut self, node_id: &str, iteration: usize) {
        if let Some(node) = self.node_mut(node_id) {
            node.iteration = Some(iteration);
        }
    }

    /// Settle the file a per-file loop iteration ran on. Only when the loop
    /// runs once per selected file; other loops don't map onto file rows.
    pub(super) fn settle_iteration_file(
        &mut self,
        iteration: usize,
        total: usize,
        status: FileStatus,
    ) {
        if total != self.files.len() {
            return;
        }
        if iteration < self.files.len() {
            self.settle_file(iteration, status);
        }
    }

    /// Finish file `index`: its status, and its name back in place of the
    /// last progress message ("Compressing a.jpg..." reads wrong once done).
    fn settle_file(&mut self, index: usize, status: FileStatus) {
        let name = self.waiting_name(index);
        let file = &mut self.files[index];
        file.status = status;
        file.percent = 100;
        file.name = name;
    }

    /// Record a failed iteration. In continue mode the loop keeps going, so
    /// update the counter and surface the error in the output pane.
    pub(super) fn fail_iteration(&mut self, node_id: &str, iteration: usize, error: &str) {
        self.set_iteration(node_id, iteration);
        self.push_output_line(format!("Iteration {iteration} failed: {error}"));
    }
}
