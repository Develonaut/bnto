// Per-category node type definitions.
//
// Each function returns `NodeTypeInfo` entries for one category.
// `all_node_types()` aggregates them into the engine's authoritative registry.

use super::{NodeCategory, NodeTypeInfo, ParameterDef};

/// Constructs a `NodeTypeInfo` from positional fields. Reduces per-entry
/// boilerplate — keeps the table scannable.
macro_rules! node_type {
    ($name:expr, $label:expr, $desc:expr, $cat:expr, $container:expr, $platform:expr, $icon:expr) => {
        NodeTypeInfo {
            name: $name.to_string(),
            label: $label.to_string(),
            description: $desc.to_string(),
            category: $cat,
            is_container: $container,
            platforms: vec![$platform.to_string()],
            icon: $icon.to_string(),
        }
    };
}

/// Return metadata for all 27 registered node types.
///
/// Single source of truth for the engine's node type registry.
/// Composed from per-category helpers, then sorted alphabetically for stable output.
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    let mut types = Vec::with_capacity(27);
    types.extend(control_node_types());
    types.extend(data_node_types());
    types.extend(file_node_types());
    types.extend(image_node_types());
    types.extend(io_node_types());
    types.extend(network_node_types());
    types.extend(spreadsheet_node_types());
    types.extend(system_node_types());
    types.extend(vector_node_types());
    types.sort_by(|a, b| a.name.cmp(&b.name));
    types
}

/// Parameter definitions for a node type, when the engine knows them
/// independently of a `NodeProcessor` registration.
///
/// Processor node types carry their params on `NodeMetadata::parameters`
/// (collected from the registry). Structural / declarative node types
/// (input, output, loop, group, parallel, transform, edit-fields) declare
/// their params in `metadata::io_container`, which this accessor exposes
/// through a single lookup by type name.
///
/// Returns `None` for unknown types and for processor types (use the
/// registry's `NodeMetadata::parameters` for those).
pub fn node_type_params(type_name: &str) -> Option<Vec<ParameterDef>> {
    super::io_container::io_container_param_defs()
        .get(type_name)
        .cloned()
}

fn control_node_types() -> Vec<NodeTypeInfo> {
    vec![
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
    ]
}

fn data_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "edit-fields",
            "Edit Fields",
            "Set field values from static values or template expressions.",
            NodeCategory::Data,
            false,
            "browser",
            "pen-line"
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

fn file_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "file-collect",
            "Collect Files",
            "Traverse a directory and collect files matching a glob pattern into the pipeline.",
            NodeCategory::File,
            false,
            "cli",
            "folder-search"
        ),
        node_type!(
            "file-copy",
            "Copy Files",
            "Place output files in a destination directory with conflict handling.",
            NodeCategory::File,
            false,
            "cli",
            "copy"
        ),
        node_type!(
            "file-move",
            "Move Files",
            "Move output files to a destination directory with conflict handling.",
            NodeCategory::File,
            false,
            "cli",
            "move"
        ),
        node_type!(
            "file-filter",
            "Filter Files",
            "Drop files that don't match extension, pattern, or size criteria.",
            NodeCategory::File,
            false,
            "browser",
            "filter"
        ),
        node_type!(
            "file-metadata",
            "File Metadata",
            "Extract file metadata (size, extension, MIME type, hash) and attach to output.",
            NodeCategory::File,
            false,
            "browser",
            "file-text"
        ),
        node_type!(
            "file-rename",
            "Rename Files",
            "Transform filenames using patterns, find/replace, case rules, counters, and sanitization.",
            NodeCategory::File,
            false,
            "browser",
            "folder-open"
        ),
    ]
}

fn image_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "image-compress",
            "Compress Images",
            "Reduce image file size while maintaining quality.",
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
            "image-resize",
            "Resize Images",
            "Change image dimensions while maintaining quality.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-strip-exif",
            "Strip EXIF",
            "Remove all EXIF metadata from images (GPS, camera info, timestamps).",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-overlay",
            "Overlay Image",
            "Overlay an image onto source images at a configurable position, size, and opacity.",
            NodeCategory::Image,
            false,
            "browser",
            "stamp"
        ),
    ]
}

fn io_node_types() -> Vec<NodeTypeInfo> {
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

fn network_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "http-request",
        "HTTP Request",
        "Make HTTP requests to APIs (GET, POST, PUT, DELETE, etc.).",
        NodeCategory::Network,
        false,
        "server",
        "globe"
    )]
}

fn spreadsheet_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "spreadsheet-clean",
            "Clean CSV",
            "Remove empty rows, trim whitespace, and deduplicate CSV data.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-read",
            "Read CSV Rows",
            "Explode a CSV into one item per row for loop iteration.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "table"
        ),
        node_type!(
            "spreadsheet-convert",
            "CSV to JSON",
            "Convert CSV data to JSON format with configurable delimiters.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-merge",
            "Merge CSV",
            "Combine multiple CSV files into one with header reconciliation and deduplication.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-rename",
            "Rename CSV Columns",
            "Rename column headers in a CSV file.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
    ]
}

fn system_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "shell-command",
        "Shell Command",
        "Execute shell commands with stall detection, retry, and streaming output.",
        NodeCategory::System,
        false,
        "server",
        "terminal"
    )]
}

fn vector_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "vector-rasterize",
            "SVG to Image",
            "Convert SVG files to raster images (PNG, JPEG, WebP).",
            NodeCategory::Vector,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "vector-optimize",
            "Optimize SVG",
            "Remove editor metadata, comments, and unnecessary elements from SVG files.",
            NodeCategory::Vector,
            false,
            "browser",
            "file-minus-2"
        ),
    ]
}
