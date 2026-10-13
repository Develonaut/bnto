// Step layout for CLI progress: top-level steps are numbered against the
// pipeline; container children are indented one level per nesting depth
// and keep the container-relative numbers the engine reports.

use std::collections::HashMap;

/// Base indent for every step line.
const BASE_INDENT: &str = "  ";
/// Extra indent per container nesting level.
const DEPTH_INDENT: &str = "    ";
/// Gap between a step's indent and its status lines (`done ...`).
const STATUS_INDENT: &str = "      ";

/// Tracks each started node's nesting depth from its parent's id.
#[derive(Debug, Default)]
pub struct StepDepths {
    depths: HashMap<String, usize>,
}

impl StepDepths {
    /// Record a started node and return its depth (0 for top level).
    /// Unknown parents count as top level, so the child sits one level in.
    pub fn start(&mut self, node_id: &str, parent_node_id: Option<&str>) -> usize {
        let depth = parent_node_id.map_or(0, |parent| self.depth(parent) + 1);
        self.depths.insert(node_id.to_string(), depth);
        depth
    }

    /// Depth of a previously started node (0 if never seen).
    pub fn depth(&self, node_id: &str) -> usize {
        self.depths.get(node_id).copied().unwrap_or(0)
    }
}

/// Indent for a step line at `depth`.
pub fn step_indent(depth: usize) -> String {
    format!("{BASE_INDENT}{}", DEPTH_INDENT.repeat(depth))
}

/// Indent for the status lines under a step at `depth`.
pub fn status_indent(depth: usize) -> String {
    format!("{}{STATUS_INDENT}", step_indent(depth))
}

/// The `[i/n]` counter for a zero-based `node_index`.
pub fn step_counter(node_index: usize, total_nodes: usize) -> String {
    format!("[{}/{total_nodes}]", node_index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plain (uncolored) step line, as the reporter lays it out.
    fn line(depths: &mut StepDepths, id: &str, parent: Option<&str>, i: usize, n: usize) -> String {
        let depth = depths.start(id, parent);
        format!("{}{} {id}", step_indent(depth), step_counter(i, n))
    }

    #[test]
    fn top_level_step_is_not_indented_beyond_base() {
        let mut depths = StepDepths::default();
        assert_eq!(line(&mut depths, "resize", None, 0, 3), "  [1/3] resize");
    }

    #[test]
    fn container_children_indent_with_container_relative_numbers() {
        let mut depths = StepDepths::default();
        let lines = [
            line(&mut depths, "loop", None, 1, 3),
            line(&mut depths, "echo", Some("loop"), 0, 1),
            line(&mut depths, "next", None, 2, 3),
        ];
        assert_eq!(lines, ["  [2/3] loop", "      [1/1] echo", "  [3/3] next"]);
    }

    #[test]
    fn nested_containers_indent_one_level_per_depth() {
        let mut depths = StepDepths::default();
        line(&mut depths, "outer", None, 0, 1);
        line(&mut depths, "inner", Some("outer"), 0, 2);
        assert_eq!(
            line(&mut depths, "leaf", Some("inner"), 0, 1),
            "          [1/1] leaf"
        );
        assert_eq!(depths.depth("inner"), 1);
        assert_eq!(depths.depth("leaf"), 2);
    }

    #[test]
    fn unknown_parent_counts_as_one_level_in() {
        let mut depths = StepDepths::default();
        assert_eq!(depths.start("child", Some("missing")), 1);
    }

    #[test]
    fn unseen_node_has_depth_zero() {
        assert_eq!(StepDepths::default().depth("nope"), 0);
    }

    #[test]
    fn status_indent_aligns_under_the_step_name() {
        assert_eq!(status_indent(0), "        ");
        assert_eq!(status_indent(1), "            ");
    }
}
