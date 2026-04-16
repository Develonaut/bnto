// Shared helpers for image processors — accepts list, quality param, and format param.

pub(crate) const MIN_QUALITY: u8 = 1;
pub(crate) const MAX_QUALITY: u8 = 100;

/// Accepted MIME types for all image processors.
pub(crate) fn image_accepts() -> Vec<String> {
    vec![
        "image/jpeg".to_string(),
        "image/png".to_string(),
        "image/webp".to_string(),
    ]
}

/// Accepted MIME types for processors that also handle SVG input.
pub(crate) fn image_accepts_with_svg() -> Vec<String> {
    let mut accepts = image_accepts();
    accepts.push("image/svg+xml".to_string());
    accepts
}

/// Format parameter definition for the convert processor (JPEG/PNG/WebP enum).
pub(crate) fn format_param_def() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "format".to_string(),
        label: "Output Format".to_string(),
        description: "The target image format to convert to".to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                OptionEntry {
                    value: "jpeg".to_string(),
                    label: "JPEG".to_string(),
                },
                OptionEntry {
                    value: "png".to_string(),
                    label: "PNG".to_string(),
                },
                OptionEntry {
                    value: "webp".to_string(),
                    label: "WebP".to_string(),
                },
            ],
        },
        default: Some(serde_json::json!("jpeg")),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

/// Quality parameter definition shared by all image operations.
pub(crate) fn quality_param_def() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "quality".to_string(),
        label: "Quality".to_string(),
        description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::json!(80)),
        constraints: Some(Constraints {
            min: Some(1.0),
            max: Some(100.0),
            required: false,
        }),
        control: Some("slider".to_string()),
        suffix: Some("%".to_string()),
        presets: Some(vec![
            PresetEntry {
                value: serde_json::json!(60),
                label: "Draft".to_string(),
            },
            PresetEntry {
                value: serde_json::json!(80),
                label: "Balanced".to_string(),
            },
            PresetEntry {
                value: serde_json::json!(100),
                label: "Maximum".to_string(),
            },
        ]),
        ..Default::default()
    }
}

/// Validate quality parameter value (shared across image processors).
pub(crate) fn validate_quality(
    params: &serde_json::Map<String, serde_json::Value>,
    errors: &mut Vec<String>,
) {
    if let Some(q_val) = params.get("quality") {
        match q_val.as_u64() {
            Some(q) if q >= MIN_QUALITY as u64 && q <= MAX_QUALITY as u64 => {}
            Some(q) => errors.push(format!(
                "Quality must be between {MIN_QUALITY} and {MAX_QUALITY}, got {q}"
            )),
            None => errors.push(format!("Quality must be a number, got: {q_val}")),
        }
    }
}
