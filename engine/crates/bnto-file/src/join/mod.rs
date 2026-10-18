// file-join — Join the files flowing in into one file, in arrival order.
//
// A batch node like n8n's Aggregate or NiFi's MergeContent: upstream steps
// choose the files and their order; this writes their bytes one after the
// other, a separator between them, as one file named by `filename`. Disk
// files stream into a temp file; nothing else is written, so a dry run (whose
// temp files are removed with the run) writes nothing. Zero files give an
// empty file. A disk file that's gone fails the run, naming it.

mod params;
mod write;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{
    BatchFile, BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use crate::metadata::guess_mime_type;
use params::{JoinParams, build_join_parameters};

/// The node-metadata key holding how many files were joined.
const JOINED_KEY: &str = "joined";

/// File-join processor — files in, one joined file out.
pub struct FileJoin;

impl FileJoin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileJoin {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileJoin {
    fn name(&self) -> &str {
        "file-join"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-join".to_string(),
            name: "Join Files".to_string(),
            description: "Join the files flowing in into one file, in the order they arrive, \
                          with an optional separator between them."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_join_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn count_keys(&self) -> &'static [&'static str] {
        &[JOINED_KEY]
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        JoinParams::read(params).err().into_iter().collect()
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
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = JoinParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let data = write::joined(&input.files, params.separator.as_bytes(), ctx)?;
        let joined = input.files.len();
        let files = if joined == 1 { "file" } else { "files" };
        progress.report_output(&format!("Joined {joined} {files} into {}", params.filename));
        Ok(output(&params.filename, data, joined))
    }
}

/// The joined file, its MIME type from its name, and the count as node metadata.
fn output(filename: &str, data: FileData, joined: usize) -> NodeOutput {
    let file = OutputFile {
        data,
        filename: filename.to_string(),
        mime_type: guess_mime_type(filename),
        metadata: serde_json::Map::new(),
    };
    let mut metadata = serde_json::Map::new();
    metadata.insert(JOINED_KEY.to_string(), joined.into());
    NodeOutput {
        files: vec![file],
        metadata,
    }
}

#[cfg(test)]
mod tests;
