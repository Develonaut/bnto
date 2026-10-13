// Static param validation for the resize node.

use super::dimensions::{MAX_DIMENSION, MIN_DIMENSION};
use super::processor::ResizeImages;

/// Collect validation errors: dimensions first, then quality.
pub(super) fn validate_resize_params(
    params: &serde_json::Map<String, serde_json::Value>,
) -> Vec<String> {
    let mut errors = Vec::new();
    validate_dimensions(
        ResizeImages::get_width(params),
        ResizeImages::get_height(params),
        &mut errors,
    );
    validate_quality_param(params, &mut errors);
    errors
}

fn validate_dimensions(width: Option<u32>, height: Option<u32>, errors: &mut Vec<String>) {
    if width.is_none() && height.is_none() {
        errors.push("At least one dimension (width or height) must be specified".to_string());
    }
    if let Some(w) = width {
        validate_dimension_range("Width", w, errors);
    }
    if let Some(h) = height {
        validate_dimension_range("Height", h, errors);
    }
}

fn validate_dimension_range(name: &str, val: u32, errors: &mut Vec<String>) {
    if val < MIN_DIMENSION {
        errors.push(format!(
            "{name} must be at least {MIN_DIMENSION}, got {val}"
        ));
    }
    if val > MAX_DIMENSION {
        errors.push(format!("{name} must be at most {MAX_DIMENSION}, got {val}"));
    }
}

fn validate_quality_param(
    params: &serde_json::Map<String, serde_json::Value>,
    errors: &mut Vec<String>,
) {
    if let Some(quality_val) = params.get("quality") {
        match quality_val.as_u64() {
            Some(q) if (1..=100).contains(&q) => {}
            Some(q) => errors.push(format!("Quality must be between 1 and 100, got {q}")),
            None => errors.push(format!("Quality must be a number, got: {quality_val}")),
        }
    }
}
