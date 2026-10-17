// The rows a table file reads as, and the items they become.

use bnto_core::processor::{FileData, OutputFile};

/// One row's values, keyed by column name.
pub(super) type Row = serde_json::Map<String, serde_json::Value>;

/// A parsed table file: its rows and how many distinct columns they have.
pub(super) struct Table {
    pub column_count: usize,
    pub rows: Vec<Row>,
}

/// One empty-data item per row, its values as metadata, named `{stem}_row{i}.{ext}`.
pub(super) fn row_files(stem: &str, ext: &str, mime_type: &str, rows: Vec<Row>) -> Vec<OutputFile> {
    rows.into_iter()
        .enumerate()
        .map(|(i, metadata)| OutputFile {
            data: FileData::Bytes(Vec::new()),
            filename: format!("{stem}_row{i}.{ext}"),
            mime_type: mime_type.to_string(),
            metadata,
        })
        .collect()
}

/// The node-level metadata: `columnCount` and `rowCount`.
pub(super) fn read_metadata(table: &Table) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    metadata.insert("columnCount".to_string(), table.column_count.into());
    metadata.insert("rowCount".to_string(), table.rows.len().into());
    metadata
}
