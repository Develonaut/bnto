// file-sort — Reorder a run's items by fields of their metadata.
//
// A batch node, pure and browser-capable, like n8n's Sort or a SQL ORDER BY:
// `by` names one or more metadata fields (`source, -title`), each ascending
// unless it starts with `-`, later ones breaking ties in earlier ones. The
// sort is stable, so items whose keys all tie keep their incoming order, and
// it only reorders: data (a disk reference stays one), names and metadata
// pass through untouched. See `key.rs` for how values compare.

mod key;
mod params;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{
    BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use key::Key;
use params::{SortParams, build_sort_parameters};

/// File-sort processor — the same items out, in sorted order.
pub struct FileSort;

impl FileSort {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSort {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileSort {
    fn name(&self) -> &str {
        "file-sort"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-sort".to_string(),
            name: "Sort Files".to_string(),
            description: "Reorder items by one or more metadata fields, each ascending or \
                          descending (`source, -title`). Stable; numbers sort numerically, \
                          missing values last. Data and metadata are untouched."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_sort_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        SortParams::read(params).err().into_iter().collect()
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
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = SortParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let files = sort(input.files, &params)
            .into_iter()
            .map(into_output)
            .collect();
        Ok(NodeOutput {
            files,
            metadata: serde_json::Map::new(),
        })
    }
}

/// `files` in sorted order. Each item's keys are built once, then a stable
/// sort compares them key by key.
fn sort(files: Vec<BatchFile>, params: &SortParams) -> Vec<BatchFile> {
    let mut keyed: Vec<(Vec<Key>, BatchFile)> = files
        .into_iter()
        .map(|file| (keys_of(&file, params), file))
        .collect();
    keyed.sort_by(|(a, _), (b, _)| {
        params
            .keys
            .iter()
            .zip(a.iter().zip(b))
            .map(|(key, (a, b))| a.compare(b, key.order))
            .find(|o| o.is_ne())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    keyed.into_iter().map(|(_, file)| file).collect()
}

/// The file's value for each sort key, in key order.
fn keys_of(file: &BatchFile, params: &SortParams) -> Vec<Key> {
    params
        .keys
        .iter()
        .map(|key| Key::of(file.metadata.get(&key.field), params.case))
        .collect()
}

/// The file as it arrived, now an output.
fn into_output(file: BatchFile) -> OutputFile {
    OutputFile {
        data: file.data,
        filename: file.filename,
        mime_type: file
            .mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string()),
        metadata: file.metadata,
    }
}

#[cfg(test)]
mod tests;
