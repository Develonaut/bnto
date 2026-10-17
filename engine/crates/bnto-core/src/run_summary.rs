// Run summary — one serializable verdict for a whole run, so a consumer's
// human summary line and its machine-readable line come from the same values.
// Per-node counts come from the run's pipeline events (`NodeTally`), since
// `PipelineResult` carries only the final files.

use std::fmt::Display;

use serde::Serialize;

use crate::events::{NodeInfo, PipelineEvent};
use crate::pipeline::PipelineResult;

/// Whether a run succeeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Pass,
    Fail,
}

/// One top-level processing node's part in a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NodeSummary {
    /// The node's id from the recipe.
    pub id: String,
    /// The node's type key (e.g. "file-copy").
    pub node_type: String,
    /// Files the node processed, summed over every time it ran.
    pub files_processed: usize,
}

impl NodeSummary {
    fn not_yet_run(info: &NodeInfo) -> Self {
        Self {
            id: info.id.clone(),
            node_type: info.node_type.clone(),
            files_processed: 0,
        }
    }
}

/// The outcome of a whole run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    pub status: RunStatus,
    /// Files the run wrote; the consumer sets it from what its output step did.
    pub files_written: usize,
    pub duration_ms: u64,
    /// Top-level processing nodes in pipeline order.
    pub nodes: Vec<NodeSummary>,
    /// Why the run failed; `None` on a pass.
    pub error: Option<String>,
}

impl RunSummary {
    /// A passing run: one file written per result file until the consumer says otherwise.
    pub fn from_result(result: &PipelineResult, duration_ms: u64) -> Self {
        Self {
            status: RunStatus::Pass,
            files_written: result.files.len(),
            duration_ms,
            nodes: Vec::new(),
            error: None,
        }
    }

    /// A failed run, carrying `error`'s message.
    pub fn from_error(error: impl Display, duration_ms: u64) -> Self {
        Self {
            status: RunStatus::Fail,
            files_written: 0,
            duration_ms,
            nodes: Vec::new(),
            error: Some(error.to_string()),
        }
    }

    /// The same summary with `nodes` in place of its node list.
    pub fn with_nodes(self, nodes: Vec<NodeSummary>) -> Self {
        Self { nodes, ..self }
    }
}

/// Builds the per-node summaries from a run's pipeline events.
#[derive(Debug, Default)]
pub struct NodeTally {
    nodes: Vec<NodeSummary>,
}

impl NodeTally {
    /// Fold one event into the tally.
    pub fn record(&mut self, event: &PipelineEvent) {
        match event {
            PipelineEvent::PipelineStarted { nodes, .. } => {
                self.nodes = nodes.iter().map(NodeSummary::not_yet_run).collect();
            }
            // Nodes inside a container count toward the container, not the run.
            PipelineEvent::NodeCompleted {
                node_id,
                files_processed,
                parent_node_id: None,
                ..
            } => self.add_files(node_id, *files_processed),
            _ => {}
        }
    }

    /// Auto-iteration completes a per-file node once per file, so counts add up.
    fn add_files(&mut self, node_id: &str, files: usize) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.files_processed += files;
        }
    }

    /// The tallied nodes, in pipeline order.
    pub fn nodes(&self) -> Vec<NodeSummary> {
        self.nodes.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::BntoError;
    use crate::pipeline::PipelineFileResult;
    use crate::processor::FileData;

    fn result_with(names: &[&str], duration_ms: u64) -> PipelineResult {
        let files = names
            .iter()
            .map(|n| PipelineFileResult {
                name: n.to_string(),
                data: FileData::Bytes(vec![]),
                mime_type: "text/plain".into(),
                metadata: serde_json::Map::new(),
            })
            .collect();
        PipelineResult {
            files,
            duration_ms,
            warnings: vec![],
        }
    }

    fn started(nodes: &[(&str, &str)]) -> PipelineEvent {
        PipelineEvent::PipelineStarted {
            total_nodes: nodes.len(),
            total_files: 1,
            nodes: nodes
                .iter()
                .map(|(id, t)| NodeInfo {
                    id: id.to_string(),
                    node_type: t.to_string(),
                })
                .collect(),
        }
    }

    fn completed(id: &str, files: usize, parent: Option<&str>) -> PipelineEvent {
        PipelineEvent::NodeCompleted {
            node_id: id.into(),
            duration_ms: 1,
            files_processed: files,
            parent_node_id: parent.map(String::from),
        }
    }

    fn node(id: &str, node_type: &str, files_processed: usize) -> NodeSummary {
        NodeSummary {
            id: id.into(),
            node_type: node_type.into(),
            files_processed,
        }
    }

    // --- RunSummary ---

    #[test]
    fn test_from_result_passes_with_the_result_files_and_duration() {
        let summary = RunSummary::from_result(&result_with(&["a.txt", "b.txt"], 9), 42);
        assert_eq!(summary.status, RunStatus::Pass);
        assert_eq!(summary.files_written, 2);
        assert_eq!(summary.duration_ms, 42);
        assert_eq!(summary.error, None);
    }

    #[test]
    fn test_from_result_with_two_tallied_nodes_carries_each_node() {
        let mut tally = NodeTally::default();
        tally.record(&started(&[
            ("collect", "file-collect"),
            ("copy", "file-copy"),
        ]));
        tally.record(&completed("collect", 1, None));
        tally.record(&completed("copy", 3, None));
        let summary =
            RunSummary::from_result(&result_with(&["a", "b", "c"], 5), 5).with_nodes(tally.nodes());
        assert_eq!(
            summary.nodes,
            vec![
                node("collect", "file-collect", 1),
                node("copy", "file-copy", 3)
            ]
        );
    }

    #[test]
    fn test_from_error_fails_with_the_message() {
        let err = BntoError::ProcessingFailed("disk full".into());
        let summary = RunSummary::from_error(&err, 7);
        assert_eq!(summary.status, RunStatus::Fail);
        assert_eq!(summary.error.as_deref(), Some(err.to_string().as_str()));
        assert_eq!(summary.files_written, 0);
        assert_eq!(summary.duration_ms, 7);
        assert!(summary.nodes.is_empty());
    }

    #[test]
    fn test_from_error_takes_a_plain_message() {
        let summary = RunSummary::from_error("unresolved {{vars.Out}}", 0);
        assert_eq!(summary.error.as_deref(), Some("unresolved {{vars.Out}}"));
    }

    #[test]
    fn test_summary_serializes_to_the_documented_shape() {
        let summary = RunSummary::from_result(&result_with(&["a"], 3), 3).with_nodes(vec![node(
            "copy",
            "file-copy",
            1,
        )]);
        let json = serde_json::to_value(&summary).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "status": "pass",
                "files_written": 1,
                "duration_ms": 3,
                "nodes": [{ "id": "copy", "node_type": "file-copy", "files_processed": 1 }],
                "error": null
            })
        );
    }

    #[test]
    fn test_failed_summary_serializes_status_fail() {
        let json = serde_json::to_value(RunSummary::from_error("boom", 1)).unwrap();
        assert_eq!(json["status"], "fail");
        assert_eq!(json["error"], "boom");
    }

    // --- NodeTally ---

    #[test]
    fn test_tally_sums_a_node_that_runs_once_per_file() {
        let mut tally = NodeTally::default();
        tally.record(&started(&[("rename", "file-rename")]));
        tally.record(&completed("rename", 1, None));
        tally.record(&completed("rename", 1, None));
        assert_eq!(tally.nodes(), vec![node("rename", "file-rename", 2)]);
    }

    #[test]
    fn test_tally_ignores_nodes_inside_a_container() {
        let mut tally = NodeTally::default();
        tally.record(&started(&[("loop", "loop")]));
        tally.record(&completed("inner", 4, Some("loop")));
        tally.record(&completed("loop", 2, None));
        assert_eq!(tally.nodes(), vec![node("loop", "loop", 2)]);
    }

    #[test]
    fn test_tally_keeps_a_node_that_never_completed_at_zero() {
        let mut tally = NodeTally::default();
        tally.record(&started(&[("a", "file-collect"), ("b", "file-copy")]));
        tally.record(&completed("a", 1, None));
        assert_eq!(
            tally.nodes(),
            vec![node("a", "file-collect", 1), node("b", "file-copy", 0)]
        );
    }
}
