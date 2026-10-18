// Parameter schema for file-delete, and typed reads of its values.

use bnto_core::metadata::{Constraints, ParameterDef, ParameterType};
use serde_json::Value;

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct DeleteParams {
    /// The folder every deleted file must be under.
    pub base: String,
}

impl DeleteParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let required =
            || "'base' is required: the folder every deleted file must be under".to_string();
        match params.get("base") {
            None | Some(Value::Null) => Err(required()),
            Some(Value::String(s)) if s.is_empty() => Err(required()),
            Some(Value::String(s)) => Ok(Self { base: s.clone() }),
            Some(_) => Err("'base' must be a folder path (text)".to_string()),
        }
    }
}

/// All file-delete parameters, in the order the editor displays them.
pub(super) fn build_delete_parameters() -> Vec<ParameterDef> {
    vec![ParameterDef {
        name: "base".to_string(),
        label: "Base Folder".to_string(),
        description: "Folder every deleted file must be under. A file outside it (including \
                      through \"..\" or a link) refuses the run, so nothing is deleted."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("./build".to_string()),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }]
}
