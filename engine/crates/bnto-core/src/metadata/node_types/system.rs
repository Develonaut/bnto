// System nodes: running external tools.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "shell-run",
        "Run Shell Command",
        "Run shell commands with stall detection, retry, and streaming output. \
         Still runs under bnto run --dry-run, so a recipe run dry uses it only to read.",
        NodeCategory::System,
        false,
        "server",
        "terminal"
    )]
}
