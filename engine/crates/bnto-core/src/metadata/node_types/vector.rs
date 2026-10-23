// Vector nodes: SVG optimize and rasterize.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "vector-rasterize",
            "Rasterize SVG",
            "Rasterize SVG files into raster images (PNG, JPEG, WebP).",
            NodeCategory::Vector,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "vector-optimize",
            "Optimize SVG",
            "Optimize SVG files by removing editor metadata, comments, and unnecessary elements.",
            NodeCategory::Vector,
            false,
            "browser",
            "file-minus-2"
        ),
    ]
}
