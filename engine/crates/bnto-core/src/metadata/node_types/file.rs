// File nodes: listing, filtering, moving and writing files.

use crate::metadata::{NodeCategory, NodeTypeInfo};

#[expect(clippy::too_many_lines)]
pub(super) fn node_types() -> Vec<NodeTypeInfo> {
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
