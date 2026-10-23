// Network nodes.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "http-request",
        "HTTP Request",
        "Request URLs over HTTP to call APIs (GET, POST, PUT, DELETE, etc.).",
        NodeCategory::Network,
        false,
        "server",
        "globe"
    )]
}
