// Parameter schema for the resize node — pure data consumed by `metadata()`.

use bnto_core::metadata::{Constraints, ParameterDef, ParameterType};

use crate::common::quality_param_def;

/// All resize parameters, in the order the editor displays them.
pub(super) fn resize_param_defs() -> Vec<ParameterDef> {
    vec![
        dimension_param("width", "Width", "Target width in pixels"),
        dimension_param("height", "Height", "Target height in pixels"),
        maintain_aspect_param(),
        quality_param_def(),
    ]
}

fn dimension_param(name: &str, label: &str, desc: &str) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        param_type: ParameterType::Number,
        constraints: Some(Constraints {
            min: Some(1.0),
            max: None,
            required: false,
        }),
        group: Some("dimensions".to_string()),
        suffix: Some("px".to_string()),
        ..Default::default()
    }
}

fn maintain_aspect_param() -> ParameterDef {
    ParameterDef {
        name: "maintainAspect".to_string(),
        label: "Maintain Aspect Ratio".to_string(),
        description: "Keep the original width-to-height ratio when resizing".to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(true)),
        group: Some("dimensions".to_string()),
        ..Default::default()
    }
}
