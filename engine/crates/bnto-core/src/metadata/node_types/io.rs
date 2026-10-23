// Input and output markers.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "input",
            "Input",
            "Declares how data enters the recipe.",
            NodeCategory::Io,
            false,
            "browser",
            "file-up"
        ),
        node_type!(
            "output",
            "Output",
            "Declares how results are delivered.",
            NodeCategory::Io,
            false,
            "browser",
            "download"
        ),
    ]
}
