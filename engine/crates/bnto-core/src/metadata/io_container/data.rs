// Parameters for the `transform` data node.

use super::helpers::param;
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
