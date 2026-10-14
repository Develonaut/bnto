// Parameter schema for spreadsheet-export, and typed reads of its values.

use bnto_core::metadata::{Constraints, ParameterDef, ParameterType};

const DEFAULT_NAME_COLUMN: &str = "file";

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct ExportParams {
    pub path: String,
    pub name_column: String,
    /// Replace a file already at `path` (false keeps it and fails the run).
    pub overwrite: bool,
}

impl ExportParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(
        params: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, String> {
        let text = |key: &str| {
            params
                .get(key)
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
        };
        let path = text("path")
            .ok_or("'path' is required: the CSV file to write")?
            .to_string();
        let name_column = text("name_column")
            .unwrap_or(DEFAULT_NAME_COLUMN)
            .to_string();
        Ok(Self {
            path,
            name_column,
            overwrite: flag(params, "overwrite"),
        })
    }
}

/// A boolean param, given as a JSON boolean or the text "true"/"false".
fn flag(params: &serde_json::Map<String, serde_json::Value>, key: &str) -> bool {
    match params.get(key) {
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::String(s)) => s.eq_ignore_ascii_case("true"),
        _ => false,
    }
}

/// All export parameters, in the order the editor displays them.
pub(super) fn build_export_parameters() -> Vec<ParameterDef> {
    vec![path_param(), name_column_param(), overwrite_param()]
}

fn path_param() -> ParameterDef {
    ParameterDef {
        name: "path".to_string(),
        label: "CSV Path".to_string(),
        description: "File to write, one row per file passing through; its folder must exist"
            .to_string(),
        placeholder: Some("items.csv".to_string()),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn name_column_param() -> ParameterDef {
    ParameterDef {
        name: "name_column".to_string(),
        label: "Name Column".to_string(),
        description: "Header of the first column, which holds each file's name".to_string(),
        default: Some(serde_json::json!(DEFAULT_NAME_COLUMN)),
        ..Default::default()
    }
}

fn overwrite_param() -> ParameterDef {
    ParameterDef {
        name: "overwrite".to_string(),
        label: "Overwrite".to_string(),
        description: "Replace a CSV already at the path; off keeps it (it may be an earlier \
                      run's record) and fails the run"
            .to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(false)),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn read(v: serde_json::Value) -> Result<ExportParams, String> {
        ExportParams::read(v.as_object().unwrap())
    }

    #[test]
    fn test_path_is_required() {
        assert!(read(json!({})).unwrap_err().contains("'path'"));
        assert!(read(json!({ "path": "" })).is_err());
    }

    #[test]
    fn test_name_column_defaults_to_file() {
        assert_eq!(
            read(json!({ "path": "a.csv" })).unwrap().name_column,
            "file"
        );
        let p = read(json!({ "path": "a.csv", "name_column": "new_path" })).unwrap();
        assert_eq!(p.name_column, "new_path");
    }

    #[test]
    fn test_overwrite_is_off_unless_set() {
        assert!(!read(json!({ "path": "a.csv" })).unwrap().overwrite);
        assert!(
            read(json!({ "path": "a.csv", "overwrite": true }))
                .unwrap()
                .overwrite
        );
        assert!(
            read(json!({ "path": "a.csv", "overwrite": "true" }))
                .unwrap()
                .overwrite
        );
    }

    #[test]
    fn test_schema_default_matches_the_reader() {
        let defs = build_export_parameters();
        let default = defs
            .iter()
            .find(|d| d.name == "name_column")
            .unwrap()
            .default
            .clone();
        assert_eq!(default, Some(json!("file")));
    }
}
