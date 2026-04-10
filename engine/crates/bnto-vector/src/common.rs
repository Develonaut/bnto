// Shared helpers for vector processors — accepts list and parameter definitions.

/// Accepted MIME types for SVG input.
pub(crate) fn svg_accepts() -> Vec<String> {
    vec!["image/svg+xml".to_string()]
}

/// Format parameter — target raster format for SVG output.
pub(crate) fn format_param_def() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "format".to_string(),
        label: "Output Format".to_string(),
        description: "The target raster format to convert SVG to".to_string(),
        param_type: ParameterType::Enum {
            options: vec!["png".to_string(), "jpeg".to_string(), "webp".to_string()],
        },
        default: Some(serde_json::json!("png")),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

/// Quality parameter shared by vector rasterization operations.
pub(crate) fn quality_param_def() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "quality".to_string(),
        label: "Quality".to_string(),
        description: "Output quality (1 = lowest, 100 = highest). Applies to JPEG; PNG is lossless; WebP is lossless-only.".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::json!(80)),
        constraints: Some(Constraints {
            min: Some(1.0),
            max: Some(100.0),
            required: false,
        }),
        ..Default::default()
    }
}
