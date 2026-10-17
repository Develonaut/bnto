// Parameter schema for file-list, and typed reads of its values.

use bnto_core::metadata::{ParameterDef, ParameterType};
use serde_json::Value;

/// The node's params, read and checked.
#[derive(Debug, Default)]
pub(super) struct ListParams {
    /// Paths listed by the recipe, added to any read from input lists.
    pub paths: Vec<String>,
    /// The folder paths are relative to; `None` means the work dir.
    pub base: Option<String>,
}

impl ListParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let base = params
            .get("base")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        Ok(Self {
            paths: read_paths(params.get("paths"))?,
            base,
        })
    }
}

/// `paths` as a list of text (a single string is a one-item list).
fn read_paths(value: Option<&Value>) -> Result<Vec<String>, String> {
    let not_text = || "'paths' must be a list of text paths".to_string();
    match value {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::String(s)) => Ok(vec![s.clone()]),
        Some(Value::Array(items)) => items
            .iter()
            .map(|v| v.as_str().map(str::to_string).ok_or_else(not_text))
            .collect(),
        Some(_) => Err(not_text()),
    }
}

/// All file-list parameters, in the order the editor displays them.
pub(super) fn build_list_parameters() -> Vec<ParameterDef> {
    vec![paths_param(), base_param()]
}

fn paths_param() -> ParameterDef {
    ParameterDef {
        name: "paths".to_string(),
        label: "Paths".to_string(),
        description: "Paths to list, relative to the base folder, added to any read from \
                      input files (one per line, or NUL-separated)."
            .to_string(),
        param_type: ParameterType::Object,
        ..Default::default()
    }
}

fn base_param() -> ParameterDef {
    ParameterDef {
        name: "base".to_string(),
        label: "Base Folder".to_string(),
        description: "Folder the listed paths are relative to. Defaults to the working \
                      directory; absolute paths and \"..\" are refused."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some(".".to_string()),
        ..Default::default()
    }
}
