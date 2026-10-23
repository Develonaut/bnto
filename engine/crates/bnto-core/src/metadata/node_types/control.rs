// Control-flow nodes: containers and run checks.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "assert",
            "Assert",
            "Assert a comparison on each item; fail the run when it doesn't hold, otherwise pass files through.",
            NodeCategory::Control,
            false,
            "browser",
            "circle-off"
        ),
        node_type!(
            "group",
            "Group",
            "Container for child nodes. Orchestrates sequential or parallel execution.",
            NodeCategory::Control,
            true,
            "browser",
            "box"
        ),
        node_type!(
            "loop",
            "Loop",
            "Iterate over arrays (forEach), repeat N times, or loop while condition.",
            NodeCategory::Control,
            true,
            "browser",
            "repeat"
        ),
        node_type!(
            "parallel",
            "Parallel",
            "Execute tasks concurrently with configurable worker pool and error strategy.",
            NodeCategory::Control,
            true,
            "browser",
            "git-fork"
        ),
        node_type!(
            "report",
            "Report",
            "Report a templated line in the run's output; files pass through unchanged.",
            NodeCategory::Control,
            false,
            "browser",
            "message-square"
        ),
    ]
}
