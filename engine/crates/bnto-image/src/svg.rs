// SVG detection and rasterization — used by the convert processor to accept SVG input.
//
// SVG is text-based XML with no magic bytes like raster formats. Detection uses
// content inspection (XML/SVG markers) and file extension as a fallback.

use bnto_core::errors::BntoError;
use bnto_vector::RasterizeOptions;

/// Check if data is SVG by looking for XML/SVG markers or file extension.
pub(crate) fn is_svg(data: &[u8], filename: &str) -> bool {
    if filename.to_lowercase().ends_with(".svg") {
        return true;
    }
    let trimmed = strip_bom_and_whitespace(data);
    trimmed.starts_with(b"<svg") || trimmed.starts_with(b"<?xml")
}

/// Strip UTF-8 BOM and leading ASCII whitespace from raw bytes.
fn strip_bom_and_whitespace(data: &[u8]) -> &[u8] {
    let d = data.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(data);
    let skip = d.iter().take_while(|b| b.is_ascii_whitespace()).count();
    &d[skip..]
}

/// Rasterize SVG bytes to a DynamicImage at 96 DPI (SVG spec default).
pub(crate) fn rasterize_svg_to_image(data: &[u8]) -> Result<image::DynamicImage, BntoError> {
    let pixmap = bnto_vector::rasterize_svg(data, RasterizeOptions::default())
        .map_err(|e| BntoError::InvalidInput(format!("SVG rasterization failed: {e}")))?;
    let width = pixmap.width();
    let height = pixmap.height();
    image::RgbaImage::from_raw(width, height, pixmap.take())
        .map(image::DynamicImage::ImageRgba8)
        .ok_or_else(|| BntoError::InvalidInput("Failed to convert SVG pixels to image".into()))
}
