// Image nodes: compress, convert, resize and friends.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "image-compress",
            "Compress Images",
            "Compress images to a smaller file size while maintaining quality.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-convert",
            "Convert Image Format",
            "Convert images between JPEG, PNG, and WebP formats.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-pad",
            "Pad Images",
            "Pad images onto a larger, centered canvas, filling the added space with transparency or a color.",
            NodeCategory::Image,
            false,
            "browser",
            "expand"
        ),
        node_type!(
            "image-resize",
            "Resize Images",
            "Resize images to new dimensions while maintaining quality.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-strip-exif",
            "Strip EXIF",
            "Strip all EXIF metadata from images (GPS, camera info, timestamps).",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-overlay",
            "Overlay Images",
            "Overlay an image onto source images at a configurable position, size, and opacity.",
            NodeCategory::Image,
            false,
            "browser",
            "stamp"
        ),
    ]
}
