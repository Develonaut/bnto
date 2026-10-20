// Static param validation for the pad node.

use super::color::parse_background;
use super::params::{get_background, get_side};
use crate::common::validate_quality;
use crate::resize::{MAX_DIMENSION, MIN_DIMENSION};

type Params = serde_json::Map<String, serde_json::Value>;

/// Collect validation errors: canvas sides, then background, then quality.
pub(super) fn validate_pad_params(params: &Params) -> Vec<String> {
    let mut errors = Vec::new();
    validate_sides(params, &mut errors);
    validate_background(params, &mut errors);
    validate_quality(params, &mut errors);
    errors
}

fn validate_sides(params: &Params, errors: &mut Vec<String>) {
    let (width, height) = (get_side(params, "width"), get_side(params, "height"));
    if width.is_none() && height.is_none() {
        errors.push("At least one canvas side (width or height) must be specified".to_string());
    }
    for (name, side) in [("Width", width), ("Height", height)] {
        if let Some(val) = side.filter(|v| !(MIN_DIMENSION..=MAX_DIMENSION).contains(v)) {
            errors.push(format!(
                "{name} must be between {MIN_DIMENSION} and {MAX_DIMENSION}, got {val}"
            ));
        }
    }
}

fn validate_background(params: &Params, errors: &mut Vec<String>) {
    let raw = get_background(params);
    if parse_background(raw).is_none() {
        errors.push(format!(
            "background must be \"transparent\" or a hex color like #ffffff, got {raw:?}"
        ));
    }
}
