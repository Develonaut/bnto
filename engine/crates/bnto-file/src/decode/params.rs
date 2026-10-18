// Parameter schema for file-decode, and typed reads of its values.

use bnto_core::metadata::{OptionEntry, ParameterDef, ParameterType};
use serde_json::Value;

use super::encoding::Encoding;

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct DecodeParams {
    pub encoding: Encoding,
    /// The decoded file's name template; `None` keeps the input's name.
    pub filename: Option<String>,
}

impl DecodeParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let filename = params
            .get("filename")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        Ok(Self {
            encoding: read_encoding(params)?,
            filename,
        })
    }
}

/// What `validate` refuses. An encoding still holding a `{{…}}` placeholder
/// is checked when the run resolves it.
pub(super) fn problems(params: &serde_json::Map<String, Value>) -> Vec<String> {
    let templated = params
        .get("encoding")
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.contains("{{"));
    if templated {
        return Vec::new();
    }
    DecodeParams::read(params).err().into_iter().collect()
}

/// `encoding`, `base64` when unset.
fn read_encoding(params: &serde_json::Map<String, Value>) -> Result<Encoding, String> {
    let value = params
        .get("encoding")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_ENCODING);
    Encoding::parse(value).ok_or_else(|| {
        let known: Vec<&str> = Encoding::OPTIONS.iter().map(|(v, _)| *v).collect();
        format!(
            "Unknown encoding \"{value}\" (expected one of: {})",
            known.join(", ")
        )
    })
}

const DEFAULT_ENCODING: &str = "base64";

/// All file-decode parameters, in the order the editor displays them.
pub(super) fn build_decode_parameters() -> Vec<ParameterDef> {
    vec![encoding_param(), filename_param()]
}

fn encoding_param() -> ParameterDef {
    let options = Encoding::OPTIONS
        .iter()
        .map(|(value, label)| OptionEntry {
            value: value.to_string(),
            label: label.to_string(),
        })
        .collect();
    ParameterDef {
        name: "encoding".to_string(),
        label: "Encoding".to_string(),
        description: "The text encoding each file is in. base64 is the standard alphabet \
                      (+ and /), padded or not; whitespace and line breaks are ignored. \
                      URL-safe base64 (- and _) is not accepted."
            .to_string(),
        param_type: ParameterType::Enum { options },
        default: Some(serde_json::json!(DEFAULT_ENCODING)),
        ..Default::default()
    }
}

fn filename_param() -> ParameterDef {
    ParameterDef {
        name: "filename".to_string(),
        label: "Output Filename".to_string(),
        description: "Name for each decoded file. {{name}} is the input's name without its \
                      last extension and {{ext}} that extension, e.g. {{name}}.json. \
                      Leave empty to keep the input's name."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("{{name}}.json".to_string()),
        ..Default::default()
    }
}
