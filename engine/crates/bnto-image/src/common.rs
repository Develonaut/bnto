// Shared helpers for image processors — accepts list and quality parameter.

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
        ..Default::default()
    }
}
