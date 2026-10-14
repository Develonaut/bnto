// Parameter schema for file-move, and typed reads of its values.

use bnto_core::metadata::{ParameterDef, ParameterType};

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct MoveParams {
    /// The folder each file's name is a path under.
    pub destination: String,
    /// Create missing folders (false refuses the run when one is missing).
    pub create_dirs: bool,
}

impl MoveParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(
        params: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, String> {
        let destination = params
            .get("destination")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or("'destination' is required: the folder files are moved under")?
            .to_string();
        Ok(Self {
            destination,
            create_dirs: flag(params, "create_dirs", true),
        })
    }
}

/// A boolean param, given as a JSON boolean or the text "true"/"false".
fn flag(params: &serde_json::Map<String, serde_json::Value>, key: &str, default: bool) -> bool {
    match params.get(key) {
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::String(s)) => s.eq_ignore_ascii_case("true"),
        _ => default,
    }
}

/// All file-move parameters, in the order the editor displays them.
pub(super) fn build_move_parameters() -> Vec<ParameterDef> {
    vec![destination_param(), create_dirs_param()]
}

fn destination_param() -> ParameterDef {
    ParameterDef {
        name: "destination".to_string(),
        label: "Destination".to_string(),
        description: "Folder to move files under. Each file's name is its path inside it \
                      (e.g. Pistol/Slide/shot.wav); absolute paths and \"..\" are refused."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("./sorted".to_string()),
        ..Default::default()
    }
}

fn create_dirs_param() -> ParameterDef {
    ParameterDef {
        name: "create_dirs".to_string(),
        label: "Create Folders".to_string(),
        description: "Create missing folders under the destination. Off refuses the run when \
                      a folder is missing."
            .to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(true)),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn read(v: serde_json::Value) -> Result<MoveParams, String> {
        MoveParams::read(v.as_object().unwrap())
    }

    #[test]
    fn test_destination_is_required() {
        assert!(
            read(json!({}))
                .unwrap_err()
                .contains("'destination' is required")
        );
        assert!(read(json!({ "destination": "" })).is_err());
    }

    #[test]
    fn test_create_dirs_defaults_on_and_reads_text() {
        assert!(read(json!({ "destination": "d" })).unwrap().create_dirs);
        let off = read(json!({ "destination": "d", "create_dirs": "false" })).unwrap();
        assert!(!off.create_dirs);
        let off = read(json!({ "destination": "d", "create_dirs": false })).unwrap();
        assert!(!off.create_dirs);
    }
}
