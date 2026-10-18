// Parameter schema for file-join, and typed reads of its values.

use bnto_core::metadata::{Constraints, ParameterDef};
use serde_json::Value;

const DEFAULT_SEPARATOR: &str = "";

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct JoinParams {
    pub filename: String,
    pub separator: String,
}

type Params = serde_json::Map<String, Value>;

impl JoinParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &Params) -> Result<Self, String> {
        let filename = params
            .get("filename")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .ok_or("'filename' is required: the name of the joined file")?
            .to_string();
        Ok(Self {
            filename,
            separator: separator(params)?,
        })
    }
}

/// `separator` as written; nothing when not set.
fn separator(params: &Params) -> Result<String, String> {
    match params.get("separator") {
        None | Some(Value::Null) => Ok(DEFAULT_SEPARATOR.to_string()),
        Some(Value::String(s)) => Ok(s.clone()),
        Some(other) => Err(format!("'separator' must be text, not {other}")),
    }
}

/// All file-join parameters, in the order the editor displays them.
pub(super) fn build_join_parameters() -> Vec<ParameterDef> {
    vec![filename_param(), separator_param()]
}

fn filename_param() -> ParameterDef {
    ParameterDef {
        name: "filename".to_string(),
        label: "File Name".to_string(),
        description: "Name of the joined file, e.g. README.md; its MIME type follows the \
                      extension. Reads run templates such as {{vars.X}}."
            .to_string(),
        placeholder: Some("joined.txt".to_string()),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn separator_param() -> ParameterDef {
    ParameterDef {
        name: "separator".to_string(),
        label: "Separator".to_string(),
        description: "Put between files, as written (a newline, a blank line, ---). Not \
                      before the first or after the last. Empty joins them directly."
            .to_string(),
        default: Some(serde_json::json!(DEFAULT_SEPARATOR)),
        ..Default::default()
    }
}
