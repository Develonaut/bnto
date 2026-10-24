// Parameter schema for edit-fields, and typed reads of its values.

use serde_json::{Map, Value};

use crate::metadata::{Constraints, ParameterDef, ParameterType};

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct EditFieldsParams {
    /// Field name to value; strings may read `{{item.KEY}}` and `{{file}}`.
    pub values: Map<String, Value>,
    /// Drop every field the item carried that `values` doesn't set.
    pub keep_only_set: bool,
}

impl EditFieldsParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &Map<String, Value>) -> Result<Self, String> {
        Ok(Self {
            values: read_values(params)?,
            keep_only_set: read_keep_only_set(params)?,
        })
    }
}

fn read_values(params: &Map<String, Value>) -> Result<Map<String, Value>, String> {
    let values = match params.get("values") {
        None | Some(Value::Null) => {
            return Err("'values' is required: a map of field name to value".into());
        }
        Some(Value::Object(values)) => values,
        Some(other) => {
            return Err(format!(
                "'values' must be a map of field name to value, not {other}"
            ));
        }
    };
    if values.keys().any(|k| k.trim().is_empty()) {
        return Err("'values' has a field with no name".into());
    }
    Ok(values.clone())
}

fn read_keep_only_set(params: &Map<String, Value>) -> Result<bool, String> {
    match params.get("keepOnlySet") {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(keep)) => Ok(*keep),
        Some(other) => Err(format!("'keepOnlySet' must be true or false, not {other}")),
    }
}

/// All edit-fields parameters, in the order the editor displays them.
pub(super) fn build_edit_fields_parameters() -> Vec<ParameterDef> {
    vec![values_param(), keep_only_set_param()]
}

fn values_param() -> ParameterDef {
    ParameterDef {
        name: "values".to_string(),
        label: "Values".to_string(),
        description: "Field name to value. A value can read the item's other fields, \
                      e.g. {{item.first}} {{item.last}}, and {{file}} (the file's name); \
                      one that is exactly \
                      {{item.<key>}} keeps that field's type. Templates read the fields \
                      the item arrived with, and one it lacks fails the run."
            .to_string(),
        param_type: ParameterType::Object,
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        control: Some("keyValue".to_string()),
        ..Default::default()
    }
}

fn keep_only_set_param() -> ParameterDef {
    ParameterDef {
        name: "keepOnlySet".to_string(),
        label: "Keep Only Set Fields".to_string(),
        description: "When on, each item keeps only the fields set in values; \
                      every other field it carried is dropped."
            .to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(false)),
        control: Some("switch".to_string()),
        ..Default::default()
    }
}
