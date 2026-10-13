// Parameter schema for the overlay node — pure data consumed by `metadata()`.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

use super::params::{
    DEFAULT_OFFSET_X, DEFAULT_OFFSET_Y, DEFAULT_OPACITY, DEFAULT_POSITION, DEFAULT_SIZE, POSITIONS,
};

/// All overlay parameters, in the order the editor displays them.
pub(super) fn build_overlay_parameters() -> Vec<ParameterDef> {
    vec![
        overlay_param(),
        position_param(),
        percent_param(
            "size",
            "Size",
            "Overlay width as a percentage of the source image width.",
            DEFAULT_SIZE,
            (1.0, 500.0),
        ),
        percent_param(
            "opacity",
            "Opacity",
            "Overlay transparency (0 = invisible, 100 = fully opaque).",
            DEFAULT_OPACITY,
            (0.0, 100.0),
        ),
        offset_param(
            "offsetX",
            "Offset X",
            "Horizontal pixel offset from the position. Positive = right, negative = left.",
            DEFAULT_OFFSET_X,
        ),
        offset_param(
            "offsetY",
            "Offset Y",
            "Vertical pixel offset from the position. Positive = down, negative = up.",
            DEFAULT_OFFSET_Y,
        ),
        crate::common::quality_param_def(),
    ]
}

fn overlay_param() -> ParameterDef {
    ParameterDef {
        name: "overlay".to_string(),
        label: "Overlay Image".to_string(),
        description: "The image to overlay (base64-encoded).".to_string(),
        param_type: ParameterType::File {
            accept: vec![
                "image/png".to_string(),
                "image/jpeg".to_string(),
                "image/webp".to_string(),
            ],
        },
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn position_param() -> ParameterDef {
    let options = POSITIONS
        .iter()
        .map(|s| OptionEntry {
            value: s.to_string(),
            label: position_label(s),
        })
        .collect();
    ParameterDef {
        name: "position".to_string(),
        label: "Position".to_string(),
        description: "Where to place the overlay on the image.".to_string(),
        param_type: ParameterType::Enum { options },
        default: Some(serde_json::Value::String(DEFAULT_POSITION.to_string())),
        control: Some("watermarkPreview".to_string()),
        ..Default::default()
    }
}

/// A `%`-suffixed number parameter with `(min, max)` bounds.
fn percent_param(
    name: &str,
    label: &str,
    desc: &str,
    default: u64,
    (min, max): (f64, f64),
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(default.into())),
        constraints: Some(Constraints {
            min: Some(min),
            max: Some(max),
            required: false,
        }),
        suffix: Some("%".to_string()),
        ..Default::default()
    }
}

/// A pixel offset parameter in the "offset" group, bounded to ±500px.
fn offset_param(name: &str, label: &str, desc: &str, default: i64) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(default.into())),
        constraints: Some(Constraints {
            min: Some(-500.0),
            max: Some(500.0),
            required: false,
        }),
        group: Some("offset".to_string()),
        suffix: Some("px".to_string()),
        ..Default::default()
    }
}

/// "bottom-right" -> "Bottom Right".
fn position_label(value: &str) -> String {
    value
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
