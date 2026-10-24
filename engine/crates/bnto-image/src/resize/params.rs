// Resize param accessors — typed reads from the runtime params map.

use bnto_core::DEFAULT_QUALITY;
use bnto_core::errors::BntoError;

use super::processor::ResizeImages;

impl ResizeImages {
    pub(super) fn get_width(params: &serde_json::Map<String, serde_json::Value>) -> Option<u32> {
        params
            .get("width")
            .and_then(|v| v.as_u64())
            .map(|w| w as u32)
    }

    pub(super) fn get_height(params: &serde_json::Map<String, serde_json::Value>) -> Option<u32> {
        params
            .get("height")
            .and_then(|v| v.as_u64())
            .map(|h| h as u32)
    }

    /// Default: true (prevents distortion).
    pub(super) fn get_maintain_aspect(params: &serde_json::Map<String, serde_json::Value>) -> bool {
        params
            .get("maintainAspect")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// Default: true (scale smaller images up to the target).
    pub(super) fn get_enlarge(params: &serde_json::Map<String, serde_json::Value>) -> bool {
        params
            .get("enlarge")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    pub(super) fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(1, 100)
    }
}

/// Resolved resize settings for one `process()` call.
pub(super) struct ResizeParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub maintain_aspect: bool,
    pub enlarge: bool,
    pub quality: u8,
}

impl ResizeParams {
    pub fn parse(params: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            width: ResizeImages::get_width(params),
            height: ResizeImages::get_height(params),
            maintain_aspect: ResizeImages::get_maintain_aspect(params),
            enlarge: ResizeImages::get_enlarge(params),
            quality: ResizeImages::get_quality(params),
        }
    }

    /// Final (width, height) for an image of the given original size.
    pub fn target_dimensions(&self, orig_w: u32, orig_h: u32) -> Result<(u32, u32), BntoError> {
        ResizeImages::calculate_dimensions(
            self.width,
            self.height,
            orig_w,
            orig_h,
            self.maintain_aspect,
            self.enlarge,
        )
    }
}
