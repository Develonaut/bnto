// Overlay compositing — scale the overlay, apply opacity, and blend onto the source.

use image::imageops::{self, FilterType};
use image::{DynamicImage, GenericImageView, RgbaImage};

use bnto_core::progress::ProgressReporter;

use super::params::OverlaySettings;
use super::positioning::calculate_position;

/// Scale, fade, and place the overlay onto `source`.
/// Returns the composited image and the final overlay dimensions.
pub(super) fn composite_overlay(
    source: &DynamicImage,
    overlay: &DynamicImage,
    settings: &OverlaySettings,
    progress: &ProgressReporter,
) -> (DynamicImage, (u32, u32)) {
    progress.report(40, "Resizing overlay...");
    let resized_wm = scale_overlay(overlay, source.width(), settings.size);
    let wm_dims = resized_wm.dimensions();

    progress.report(50, "Applying opacity...");
    let wm_rgba = apply_opacity(&resized_wm, settings.opacity);

    progress.report(60, "Compositing overlay...");
    (place_overlay(source, &wm_rgba, settings), wm_dims)
}

/// Scale the overlay to `size`% of the source width (at least 1px).
fn scale_overlay(overlay: &DynamicImage, src_w: u32, size: u64) -> DynamicImage {
    let target_wm_width = ((src_w as f64 * size as f64) / 100.0).round().max(1.0) as u32;
    resize_overlay(overlay, target_wm_width)
}

/// Blend the prepared overlay onto a copy of `source` at its gravity position.
fn place_overlay(
    source: &DynamicImage,
    wm: &RgbaImage,
    settings: &OverlaySettings,
) -> DynamicImage {
    let (src_w, src_h) = source.dimensions();
    let (pos_x, pos_y) = calculate_position(
        settings.position,
        src_w,
        src_h,
        wm.width(),
        wm.height(),
        settings.offset_x,
        settings.offset_y,
    );
    let mut canvas = source.to_rgba8();
    imageops::overlay(&mut canvas, wm, pos_x, pos_y);
    DynamicImage::ImageRgba8(canvas)
}

/// Resize overlay to target width (% of source), maintaining aspect ratio.
fn resize_overlay(overlay: &DynamicImage, target_width: u32) -> DynamicImage {
    let (ov_w, ov_h) = overlay.dimensions();
    if ov_w == 0 {
        return overlay.clone();
    }
    let aspect = ov_h as f64 / ov_w as f64;
    let target_height = (target_width as f64 * aspect).round().max(1.0) as u32;
    overlay.resize_exact(target_width, target_height, FilterType::Lanczos3)
}

/// Apply opacity by multiplying each pixel's alpha channel.
fn apply_opacity(img: &DynamicImage, opacity: u64) -> RgbaImage {
    let rgba = img.to_rgba8();
    let factor = opacity as f64 / 100.0;
    let mut result = rgba.clone();
    for pixel in result.pixels_mut() {
        pixel[3] = (pixel[3] as f64 * factor).round().clamp(0.0, 255.0) as u8;
    }
    result
}
