// Data nodes: editing and summarizing items.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "edit-fields",
            "Edit Fields",
            "Edit fields on each item, setting them from templates over its other fields; files pass through.",
            NodeCategory::Data,
            false,
            "browser",
            "pen-line"
        ),
        node_type!(
            "summarize",
            "Summarize",
            "Summarize items into one per group of fields, with its count and field totals.",
            NodeCategory::Data,
            false,
            "browser",
            "sigma"
        ),
        node_type!(
            "transform",
            "Transform",
            "Transform data using expressions (single value) or field mappings.",
            NodeCategory::Data,
            false,
            "browser",
            "arrow-left-right"
        ),
    ]
}
