// file-decode — Decode each file's content from a text encoding into its bytes.
//
// Per-file and pure, so it runs in the browser too. base64 is the one
// encoding today (standard alphabet, padded or not, whitespace ignored);
// `encoding` leaves room for more. The decoded file keeps the input's name
// unless `filename` gives one, and its MIME type follows that name.
// A file that isn't valid in the encoding fails the run, naming the file.

mod encoding;
mod output_name;
mod params;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use crate::metadata::guess_mime_type;
use params::{DecodeParams, build_decode_parameters};

/// The decoded byte count each file records; the node's record adds it up.
const DECODED_SIZE: &str = "decoded_size";

/// File-decode processor — encoded text in, decoded bytes out.
pub struct FileDecode;

impl FileDecode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileDecode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileDecode {
    fn name(&self) -> &str {
        "file-decode"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-decode".to_string(),
            name: "Decode Files".to_string(),
            description: "Decode each file's content from a text encoding (base64) into its \
                          bytes, so later nodes read the decoded content."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_decode_parameters(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        params::problems(params)
    }

    fn count_keys(&self) -> &'static [&'static str] {
        &[DECODED_SIZE]
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = DecodeParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        progress.report(0, "Decoding...");
        let text = input.data.into_bytes().map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read {}: {e}", input.filename))
        })?;
        let data = decode_text(&params, &text, &input.filename)?;
        let filename = output_name::output_name(&input.filename, params.filename.as_deref());
        progress.report(100, "Done");
        Ok(decoded(data, filename))
    }
}

/// `text` decoded, or an error naming `file` and why it isn't valid.
fn decode_text(params: &DecodeParams, text: &[u8], file: &str) -> Result<Vec<u8>, BntoError> {
    params.encoding.decode(text).map_err(|reason| {
        let encoding = params.encoding.key();
        BntoError::InvalidInput(format!("{file} is not valid {encoding}: {reason}"))
    })
}

/// One decoded file, its MIME type from its name, and its size in the
/// node's metadata (which the executor lays over the input's metadata).
fn decoded(data: Vec<u8>, filename: String) -> NodeOutput {
    let mut metadata = serde_json::Map::new();
    metadata.insert(DECODED_SIZE.to_string(), data.len().into());
    NodeOutput {
        files: vec![OutputFile {
            data: FileData::Bytes(data),
            mime_type: guess_mime_type(&filename),
            filename,
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

#[cfg(test)]
mod tests;
