// Canvas compositing — center the image on a filled canvas, flatten for JPEG.

use image::imageops;
use image::{DynamicImage, Rgba, RgbaImage};

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

/// Center `img` on a `width`×`height` canvas filled with `background`.
///
/// The background shows through transparent source pixels, like a CSS
/// `background-color`. An odd leftover pixel goes to the right / bottom.
pub(super) fn pad_onto_canvas(
    img: &DynamicImage,
    (width, height): (u32, u32),
    background: Rgba<u8>,
) -> DynamicImage {
    let mut canvas = RgbaImage::from_pixel(width, height, background);
    let x = width.saturating_sub(img.width()) / 2;
    let y = height.saturating_sub(img.height()) / 2;
    imageops::overlay(&mut canvas, &img.to_rgba8(), i64::from(x), i64::from(y));
    DynamicImage::ImageRgba8(canvas)
}

/// Blend onto opaque white and drop alpha, for formats without transparency.
/// Without this, JPEG encoding would turn transparent padding black.
pub(super) fn flatten_onto_white(img: &DynamicImage) -> DynamicImage {
    let mut base = RgbaImage::from_pixel(img.width(), img.height(), WHITE);
    imageops::overlay(&mut base, &img.to_rgba8(), 0, 0);
    DynamicImage::ImageRgb8(DynamicImage::ImageRgba8(base).to_rgb8())
}
