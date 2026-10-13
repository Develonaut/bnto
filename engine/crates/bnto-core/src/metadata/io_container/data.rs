// Parameters for the `transform` and `edit-fields` data nodes.

use super::helpers::{param, required};
use crate::metadata::{ParameterDef, ParameterType};

/// Parameters for the `transform` node.
pub(super) fn transform_params() -> Vec<ParameterDef> {
    let expression_desc =
        "Expr expression for a single transformation. Mutually exclusive with mappings.";
    let mappings_desc = "Map of field names to expr expressions for multi-field transformations. Mutually exclusive with expression.";
    vec![
        ParameterDef {
            placeholder: Some("firstName + \" \" + lastName".to_string()),
            control: Some("textarea".to_string()),
            ..param(
                "expression",
                "Expression",
                expression_desc,
                ParameterType::String,
            )
        },
        ParameterDef {
            control: Some("keyValue".to_string()),
            ..param("mappings", "Mappings", mappings_desc, ParameterType::Object)
        },
    ]
}

/// Parameters for the `edit-fields` node.
pub(super) fn edit_fields_params() -> Vec<ParameterDef> {
    let values_desc = "Map of field names to values. Values can be static or template expressions (e.g., {{.record.name}}).";
    let keep_desc = "When true, only output fields that are explicitly set in values.";
    vec![
        ParameterDef {
            constraints: Some(required(true)),
            control: Some("keyValue".to_string()),
            ..param("values", "Values", values_desc, ParameterType::Object)
        },
        ParameterDef {
            default: Some(serde_json::json!(false)),
            control: Some("switch".to_string()),
            ..param(
                "keepOnlySet",
                "Keep Only Set Fields",
                keep_desc,
                ParameterType::Boolean,
            )
        },
    ]
}
