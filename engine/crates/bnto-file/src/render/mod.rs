// file-render — Write one text file from a template over the items.
//
// A browser-capable batch node: a header, one row per incoming item (filled
// from the item's metadata), then a footer, as one in-memory file. It never
// reads its items' content and writes nothing to disk; the output node or
// file-copy does that, so a dry run writes nothing. Runs with zero items too.

mod escape;
mod fill;
mod params;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{
    BatchFile, BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use params::{RenderParams, build_render_parameters};

/// The node-metadata key holding how many rows were written.
const ROWS_KEY: &str = "rows";

/// File-render processor — items in, one rendered text file out.
pub struct FileRender;

impl FileRender {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileRender {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileRender {
    fn name(&self) -> &str {
        "file-render"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-render".to_string(),
            name: "Render Text File".to_string(),
            description: "Render one text file from a template: a header, one row per item \
                          from its metadata, then a footer (a Markdown table, a report)."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_render_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn count_keys(&self) -> &'static [&'static str] {
        &[ROWS_KEY]
    }

    /// `row` reads `{{item.*}}`, filled here once per item.
    fn resolves_items(&self) -> bool {
        true
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        RenderParams::read(params).err().into_iter().collect()
    }

    /// Required by the trait; the executor always runs this node as a batch.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let file = BatchFile {
            data: input.data,
            filename: input.filename,
            mime_type: input.mime_type,
            metadata: serde_json::Map::new(),
        };
        let batch = BatchInput {
            files: vec![file],
            params: input.params,
        };
        self.process_batch(batch, progress, ctx)
    }

    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = RenderParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let text = render(&params, &input.files)?;
        let rows = input.files.len();
        progress.report_output(&format!("Rendered {}: {rows} rows", params.filename));
        Ok(output(&params.filename, text, rows))
    }
}

/// The file's text: header, rows joined by the separator, footer, each part
/// present on its own line and ending with a newline.
fn render(params: &RenderParams, items: &[BatchFile]) -> Result<String, BntoError> {
    let count = items.len();
    let row = fill::fill_count(&params.row, count);
    let rows = items
        .iter()
        .map(|item| fill::fill_row(&row, item, params.escape).map_err(|key| missing(&key, item)))
        .collect::<Result<Vec<_>, _>>()?;
    let parts = [
        fill::fill_count(&params.header, count),
        rows.join(&params.separator),
        fill::fill_count(&params.footer, count),
    ];
    Ok(parts
        .iter()
        .filter(|part| !part.is_empty())
        .map(|part| format!("{part}\n"))
        .collect())
}

/// A row reads `{{item.KEY}}` the item lacks: name the key, the item and
/// the keys it has.
fn missing(key: &str, item: &BatchFile) -> BntoError {
    let has = if item.metadata.is_empty() {
        "it has no metadata".to_string()
    } else {
        let keys: Vec<&str> = item.metadata.keys().map(String::as_str).collect();
        format!("it has: {}", keys.join(", "))
    };
    BntoError::InvalidInput(format!(
        "'row' reads {{{{item.{key}}}}}, which {} doesn't have ({has})",
        item.filename
    ))
}

/// The rendered file, and the row count as node metadata.
fn output(filename: &str, text: String, rows: usize) -> NodeOutput {
    let file = OutputFile {
        data: FileData::Bytes(text.into_bytes()),
        filename: filename.to_string(),
        mime_type: mime_for(filename).to_string(),
        metadata: serde_json::Map::new(),
    };
    let mut metadata = serde_json::Map::new();
    metadata.insert(ROWS_KEY.to_string(), rows.into());
    NodeOutput {
        files: vec![file],
        metadata,
    }
}

/// `text/markdown` for a `.md` or `.markdown` name, else `text/plain`.
fn mime_for(filename: &str) -> &'static str {
    match bnto_core::file_extension(filename)
        .to_ascii_lowercase()
        .as_str()
    {
        "md" | "markdown" => "text/markdown",
        _ => "text/plain",
    }
}

#[cfg(test)]
mod tests;
