// Spreadsheet nodes: CSV and JSON rows.

use crate::metadata::{NodeCategory, NodeTypeInfo};

pub(super) fn node_types() -> Vec<NodeTypeInfo> {
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
