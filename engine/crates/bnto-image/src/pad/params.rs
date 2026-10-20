// Pad param reads — canvas sides, background fill, quality.

use image::Rgba;

use bnto_core::DEFAULT_QUALITY;
use bnto_core::errors::BntoError;

use super::color::{TRANSPARENT, parse_background};

type Params = serde_json::Map<String, serde_json::Value>;

/// A canvas side in pixels. Values past `u32` saturate so validation
/// reports them as too large instead of wrapping.
pub(super) fn get_side(params: &Params, name: &str) -> Option<u32> {
    params
        .get(name)
        .and_then(|v| v.as_u64())
        .map(|n| u32::try_from(n).unwrap_or(u32::MAX))
}

pub(super) fn get_background(params: &Params) -> &str {
    params
        .get("background")
        .and_then(|v| v.as_str())
        .unwrap_or(TRANSPARENT)
}

fn get_quality(params: &Params) -> u8 {
    params
        .get("quality")
        .and_then(|v| v.as_u64())
        .map(|q| q.min(100) as u8)
        .unwrap_or(DEFAULT_QUALITY)
        .clamp(1, 100)
}

/// Resolved pad settings for one `process()` call.
pub(super) struct PadParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub background: Rgba<u8>,
    pub quality: u8,
}

impl PadParams {
    pub fn parse(params: &Params) -> Result<Self, BntoError> {
        let raw = get_background(params);
        let background = parse_background(raw).ok_or_else(|| {
            BntoError::InvalidInput(format!("background is not a color: {raw:?}"))
        })?;
        Ok(Self {
            width: get_side(params, "width"),
            height: get_side(params, "height"),
            background,
            quality: get_quality(params),
        })
    }

    /// Canvas size for an image of `(w, h)`: an omitted side keeps the
    /// image's own. Fails when the image doesn't fit, since padding never crops.
    pub fn canvas_size(&self, (w, h): (u32, u32), filename: &str) -> Result<(u32, u32), BntoError> {
        let canvas = (self.width.unwrap_or(w), self.height.unwrap_or(h));
        if w > canvas.0 || h > canvas.1 {
            return Err(BntoError::InvalidInput(format!(
                "{filename} is {w}x{h}, larger than the {}x{} canvas. \
                 Resize it to fit first (image-resize with the same width and height).",
                canvas.0, canvas.1
            )));
        }
        Ok(canvas)
    }
}
