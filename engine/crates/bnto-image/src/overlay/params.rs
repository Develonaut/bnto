// Overlay param defaults and clamped accessors for the runtime params map.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

use bnto_core::DEFAULT_QUALITY;
use bnto_core::errors::BntoError;

pub(super) const DEFAULT_SIZE: u64 = 25;
pub(super) const DEFAULT_OPACITY: u64 = 80;
pub(super) const DEFAULT_OFFSET_X: i64 = 0;
pub(super) const DEFAULT_OFFSET_Y: i64 = 0;
pub(super) const DEFAULT_POSITION: &str = "bottom-right";

/// Gravity regions accepted by the `position` param.
pub(super) const POSITIONS: &[&str] = &[
    "top-left",
    "top-center",
    "top-right",
    "middle-left",
    "center",
    "middle-right",
    "bottom-left",
    "bottom-center",
    "bottom-right",
];

/// Resolved overlay settings for one `process()` call.
pub(super) struct OverlaySettings<'a> {
    pub position: &'a str,
    pub size: u64,
    pub opacity: u64,
    pub offset_x: i64,
    pub offset_y: i64,
    pub quality: u8,
}

impl<'a> OverlaySettings<'a> {
    pub fn from_params(params: &'a serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            position: get_position(params),
            size: get_size(params),
            opacity: get_opacity(params),
            offset_x: get_offset_x(params),
            offset_y: get_offset_y(params),
            quality: get_quality(params),
        }
    }
}

/// Decode the required base64 `overlay` param, tolerating a data URI prefix.
pub(super) fn get_overlay_bytes(
    params: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<u8>, BntoError> {
    let b64 = params
        .get("overlay")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BntoError::InvalidInput("overlay parameter is required".to_string()))?;

    // Strip optional data URI prefix (e.g., "data:image/png;base64,")
    let raw = if let Some(idx) = b64.find(",") {
        &b64[idx + 1..]
    } else {
        b64
    };

    BASE64
        .decode(raw)
        .map_err(|e| BntoError::InvalidInput(format!("Invalid base64 in overlay parameter: {e}")))
}

fn get_position(params: &serde_json::Map<String, serde_json::Value>) -> &str {
    params
        .get("position")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_POSITION)
}

fn get_size(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
    params
        .get("size")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_SIZE)
        .clamp(1, 500)
}

fn get_opacity(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
    params
        .get("opacity")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_OPACITY)
        .clamp(0, 100)
}

fn get_offset_x(params: &serde_json::Map<String, serde_json::Value>) -> i64 {
    params
        .get("offsetX")
        .and_then(|v| v.as_i64())
        .unwrap_or(DEFAULT_OFFSET_X)
        .clamp(-500, 500)
}

fn get_offset_y(params: &serde_json::Map<String, serde_json::Value>) -> i64 {
    params
        .get("offsetY")
        .and_then(|v| v.as_i64())
        .unwrap_or(DEFAULT_OFFSET_Y)
        .clamp(-500, 500)
}

fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
    params
        .get("quality")
        .and_then(|v| v.as_u64())
        .map(|q| q as u8)
        .unwrap_or(DEFAULT_QUALITY)
        .clamp(1, 100)
}
