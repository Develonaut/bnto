// Parameter schema for the pad node — pure data consumed by `metadata()`.

use bnto_core::metadata::{Constraints, ParameterDef, ParameterType};

use super::color::TRANSPARENT;
use crate::common::quality_param_def;
use crate::resize::{MAX_DIMENSION, MIN_DIMENSION};

/// All pad parameters, in the order the editor displays them.
pub(super) fn pad_param_defs() -> Vec<ParameterDef> {
    vec![
        side_param(
            "width",
            "Width",
            "Canvas width in pixels. Leave empty to keep the image's width.",
        ),
        side_param(
            "height",
            "Height",
            "Canvas height in pixels. Leave empty to keep the image's height.",
        ),
        background_param(),
        quality_param_def(),
    ]
}

fn side_param(name: &str, label: &str, desc: &str) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        param_type: ParameterType::Number,
        constraints: Some(Constraints {
            min: Some(f64::from(MIN_DIMENSION)),
            max: Some(f64::from(MAX_DIMENSION)),
            required: false,
        }),
        group: Some("canvas".to_string()),
        suffix: Some("px".to_string()),
        ..Default::default()
    }
}

fn background_param() -> ParameterDef {
    ParameterDef {
        name: "background".to_string(),
        label: "Background".to_string(),
        description: "Fill for the added space: \"transparent\" or a hex color (#fff, #ffffff, \
                      #ffffff80). JPEG has no transparency, so a transparent fill comes out white."
            .to_string(),
        param_type: ParameterType::String,
        default: Some(serde_json::json!(TRANSPARENT)),
        placeholder: Some("#ffffff".to_string()),
        ..Default::default()
    }
}
