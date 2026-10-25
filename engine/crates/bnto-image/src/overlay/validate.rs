// Static param validation for the overlay node — one check per rule.

use super::params::POSITIONS;

type Params = serde_json::Map<String, serde_json::Value>;

/// Collect validation errors, in the order the checks are declared.
pub(super) fn validate_params(params: &Params) -> Vec<String> {
    [
        check_overlay(params),
        check_position(params),
        check_size(params),
        check_opacity(params),
        check_quality(params),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Overlay image is required: blank text has no image to decode.
fn check_overlay(params: &Params) -> Option<String> {
    params
        .get("overlay")
        .and_then(|v| v.as_str())
        .is_none_or(|overlay| overlay.trim().is_empty())
        .then(|| "overlay is required".to_string())
}

fn check_position(params: &Params) -> Option<String> {
    let pos = params.get("position").and_then(|v| v.as_str())?;
    (!POSITIONS.contains(&pos))
        .then(|| format!("position must be one of: {}", POSITIONS.join(", ")))
}

fn check_size(params: &Params) -> Option<String> {
    let size = params.get("size").and_then(|v| v.as_u64())?;
    (!(1..=500).contains(&size)).then(|| "size must be between 1 and 500".to_string())
}

fn check_opacity(params: &Params) -> Option<String> {
    let opacity = params.get("opacity").and_then(|v| v.as_u64())?;
    (opacity > 100).then(|| "opacity must be between 0 and 100".to_string())
}

fn check_quality(params: &Params) -> Option<String> {
    let quality = params.get("quality").and_then(|v| v.as_u64())?;
    (!(1..=100).contains(&quality)).then(|| "quality must be between 1 and 100".to_string())
}
