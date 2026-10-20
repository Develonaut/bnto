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

/// Return metadata for all 39 registered node types.
///
/// Single source of truth for the engine's node type registry.
/// Composed from per-category helpers, then sorted alphabetically for stable output.
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    let mut types = Vec::with_capacity(26);
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
/// (input, output, loop, group, parallel, transform) declare
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

fn data_node_types() -> Vec<NodeTypeInfo> {
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

fn file_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "file-collect",
            "Collect Files",
            "Collect files from a folder that match a glob pattern into the pipeline.",
            NodeCategory::File,
            false,
            "cli",
            "folder-search"
        ),
        node_type!(
            "file-copy",
            "Copy Files",
            "Copy output files into a destination folder, with conflict handling.",
            NodeCategory::File,
            false,
            "cli",
            "copy"
        ),
        node_type!(
            "file-delete",
            "Delete Files",
            "Delete the files flowing in, each under a base folder; missing files are skipped and counted.",
            NodeCategory::File,
            false,
            "cli",
            "file-minus-2"
        ),
        node_type!(
            "file-list",
            "List Files",
            "List files from lists of paths under a base folder; every path is listed, found or missing.",
            NodeCategory::File,
            false,
            "cli",
            "file-text"
        ),
        node_type!(
            "file-move",
            "Move Files",
            "Move files into folders under a destination, by the path in each file's name.",
            NodeCategory::File,
            false,
            "cli",
            "folder-open"
        ),
        node_type!(
            "file-decode",
            "Decode Files",
            "Decode each file's content from a text encoding (base64) into its bytes.",
            NodeCategory::File,
            false,
            "browser",
            "arrow-left-right"
        ),
        node_type!(
            "file-filter",
            "Filter Files",
            "Filter files by extension, name or path pattern, and size, keeping or dropping the matches.",
            NodeCategory::File,
            false,
            "browser",
            "filter"
        ),
        node_type!(
            "file-inspect",
            "Inspect Files",
            "Inspect files and attach their metadata (folder, size, extension, MIME type, modified time, hash) to the output.",
            NodeCategory::File,
            false,
            "browser",
            "file-text"
        ),
        node_type!(
            "file-join",
            "Join Files",
            "Join the files flowing in into one file, in arrival order (edge order when several steps feed it), a separator between them.",
            NodeCategory::File,
            false,
            "browser",
            "merge"
        ),
        node_type!(
            "file-rename",
            "Rename Files",
            "Rename files using patterns, find/replace, case rules, counters, and sanitization.",
            NodeCategory::File,
            false,
            "browser",
            "folder-open"
        ),
        node_type!(
            "file-render",
            "Render Text File",
            "Render one text file from a template: a header, one row per item, then a footer.",
            NodeCategory::File,
            false,
            "browser",
            "file-text"
        ),
        node_type!(
            "file-sort",
            "Sort Files",
            "Sort items by one or more metadata fields, each ascending or descending.",
            NodeCategory::File,
            false,
            "browser",
            "arrow-up-down"
        ),
    ]
}

fn image_node_types() -> Vec<NodeTypeInfo> {
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
        "Request URLs over HTTP to call APIs (GET, POST, PUT, DELETE, etc.).",
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
            "Clean CSV data by removing empty rows, trimming whitespace, and deduplicating.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-read",
            "Read Rows",
            "Read a CSV or a JSON array into one item per row for loop iteration. Nested JSON fields read as dotted keys (meta.id).",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "table"
        ),
        node_type!(
            "spreadsheet-convert",
            "Convert CSV to JSON",
            "Convert CSV data to JSON format with configurable delimiters.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-export",
            "Export Items as CSV",
            "Export the files passing through, with their metadata, as one CSV row each.",
            NodeCategory::Spreadsheet,
            false,
            "cli",
            "table"
        ),
        node_type!(
            "spreadsheet-lookup",
            "Look Up Rows",
            "Look up the CSV row whose key matches each file's name and attach it to that file.",
            NodeCategory::Spreadsheet,
            false,
            "cli",
            "table"
        ),
        node_type!(
            "spreadsheet-merge",
            "Merge CSV",
            "Merge multiple CSV files into one with header reconciliation and deduplication.",
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

fn vector_node_types() -> Vec<NodeTypeInfo> {
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
