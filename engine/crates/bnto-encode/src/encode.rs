// Shared image encoding — single entrypoint for all image processors.
//
// All processors that produce raster output route through this module
// for final encoding. This prevents duplicated encode functions and ensures
// quality/compression params are consistently applied across all formats.

use bnto_core::errors::BntoError;
use image::DynamicImage;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::format::ImageFormat;

/// Encode a DynamicImage into the target format with quality control.
///
/// - **JPEG**: `quality` controls lossy encoding (1 = smallest, 100 = best).
/// - **PNG**: `quality` maps to compression effort (lossless format, so quality
///   affects speed/size tradeoff, not visual fidelity). <33 = Fast, <66 = Default, >=66 = Best.
/// - **WebP**: `quality` controls lossy (VP8) encoding, same scale as JPEG.
///   Transparency is kept in a separate alpha plane.
pub fn encode_image(
    img: &DynamicImage,
    format: ImageFormat,
    quality: u8,
) -> Result<Vec<u8>, BntoError> {
    match format {
        ImageFormat::Jpeg => encode_jpeg(img, quality),
        ImageFormat::Png => encode_png(img, quality),
        ImageFormat::WebP => encode_webp(img, quality),
    }
}

fn encode_jpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, BntoError> {
    let mut output = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut output, quality);
    img.write_with_encoder(encoder)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;
    Ok(output)
}

fn encode_png(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, BntoError> {
    let compression = quality_to_png_compression(quality);
    let mut output = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut output, compression, FilterType::Adaptive);
    img.write_with_encoder(encoder)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode PNG: {e}")))?;
    Ok(output)
}

/// Lossy WebP via oxideav-webp. Opaque images go in as RGB so the encoder
/// doesn't write an alpha plane that is all 255.
fn encode_webp(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, BntoError> {
    let options = oxideav_webp::EncodeOptions::default().with_quality(f32::from(quality));
    let (width, height) = (img.width(), img.height());
    let encoded = if img.color().has_alpha() {
        oxideav_webp::encode_rgba8(width, height, img.to_rgba8().as_raw(), &options)
    } else {
        oxideav_webp::encode_rgb8(width, height, img.to_rgb8().as_raw(), &options)
    };
    encoded.map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))
}

/// Map a quality value (1-100) to PNG compression level.
/// PNG is lossless — quality affects encode speed vs file size, not visual fidelity.
fn quality_to_png_compression(quality: u8) -> CompressionType {
    if quality < 33 {
        CompressionType::Fast
    } else if quality < 66 {
        CompressionType::Default
    } else {
        CompressionType::Best
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgb, RgbImage, Rgba, RgbaImage};
    use std::io::Cursor;

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let mut img = RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgb([(x * 3) as u8, (y * 5) as u8, 128]));
            }
        }
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_encode_jpeg_produces_valid_output() {
        let img = create_test_image(50, 50);
        let result = encode_image(&img, ImageFormat::Jpeg, 80).unwrap();
        assert_eq!(result[0], 0xFF);
        assert_eq!(result[1], 0xD8);
    }

    #[test]
    fn test_encode_png_produces_valid_output() {
        let img = create_test_image(50, 50);
        let result = encode_image(&img, ImageFormat::Png, 80).unwrap();
        assert_eq!(&result[..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_encode_webp_produces_valid_output() {
        let img = create_test_image(50, 50);
        let result = encode_image(&img, ImageFormat::WebP, 80).unwrap();
        assert_eq!(&result[..4], b"RIFF");
        assert_eq!(&result[8..12], b"WEBP");
    }

    /// A photo-like image: smooth gradients plus per-pixel noise, which is
    /// where lossy VP8 beats lossless VP8L by a wide margin.
    fn create_photo_like_image(width: u32, height: u32) -> DynamicImage {
        let mut img = RgbImage::new(width, height);
        let mut seed = 0x2545_F491_u32;
        for y in 0..height {
            for x in 0..width {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                let noise = (seed % 24) as u8;
                img.put_pixel(
                    x,
                    y,
                    Rgb([(x * 2) as u8 ^ noise, (y * 3) as u8 ^ noise, 96 + noise]),
                );
            }
        }
        DynamicImage::ImageRgb8(img)
    }

    /// Return the FourCC of the first image-data chunk (`VP8 ` or `VP8L`).
    fn webp_bitstream(data: &[u8]) -> &[u8] {
        let mut pos = 12;
        while pos + 8 <= data.len() {
            let fourcc = &data[pos..pos + 4];
            if fourcc == b"VP8 " || fourcc == b"VP8L" {
                return fourcc;
            }
            let size = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
            pos += 8 + size + (size & 1);
        }
        panic!("no VP8/VP8L chunk in WebP output");
    }

    #[test]
    fn test_encode_webp_is_lossy() {
        let img = create_photo_like_image(64, 64);
        let result = encode_image(&img, ImageFormat::WebP, 80).unwrap();
        assert_eq!(webp_bitstream(&result), b"VP8 ");
    }

    #[test]
    fn test_webp_quality_affects_size() {
        let img = create_photo_like_image(128, 128);
        let low = encode_image(&img, ImageFormat::WebP, 20).unwrap();
        let high = encode_image(&img, ImageFormat::WebP, 90).unwrap();
        assert!(
            low.len() < high.len(),
            "Low quality ({}) should be smaller than high quality ({})",
            low.len(),
            high.len()
        );
    }

    #[test]
    fn test_webp_lossy_beats_lossless_on_photos() {
        let img = create_photo_like_image(128, 128);
        let lossy = encode_image(&img, ImageFormat::WebP, 80).unwrap();
        let mut lossless = Vec::new();
        img.write_to(&mut Cursor::new(&mut lossless), image::ImageFormat::WebP)
            .unwrap();
        assert!(
            lossy.len() < lossless.len(),
            "Lossy ({}) should be smaller than lossless ({})",
            lossy.len(),
            lossless.len()
        );
    }

    #[test]
    fn test_webp_round_trips_dimensions() {
        // Odd sizes exercise the partial macroblocks at the right and bottom edges.
        let img = create_photo_like_image(37, 21);
        let result = encode_image(&img, ImageFormat::WebP, 80).unwrap();
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (37, 21));
    }

    #[test]
    fn test_webp_keeps_transparency() {
        let mut img = RgbaImage::new(32, 32);
        for (x, _, px) in img.enumerate_pixels_mut() {
            *px = Rgba([200, 40, 40, if x < 16 { 0 } else { 255 }]);
        }
        let result = encode_image(&DynamicImage::ImageRgba8(img), ImageFormat::WebP, 80).unwrap();
        let decoded = image::load_from_memory(&result).unwrap().to_rgba8();
        assert_eq!(decoded.get_pixel(2, 2)[3], 0, "left half stays transparent");
        assert_eq!(decoded.get_pixel(30, 2)[3], 255, "right half stays opaque");
    }

    #[test]
    fn test_jpeg_quality_affects_size() {
        let img = create_test_image(100, 100);
        let low = encode_image(&img, ImageFormat::Jpeg, 20).unwrap();
        let high = encode_image(&img, ImageFormat::Jpeg, 90).unwrap();
        assert!(
            low.len() < high.len(),
            "Low quality ({}) should be smaller than high quality ({})",
            low.len(),
            high.len()
        );
    }

    #[test]
    fn test_quality_to_png_compression_mapping() {
        assert!(matches!(
            quality_to_png_compression(1),
            CompressionType::Fast
        ));
        assert!(matches!(
            quality_to_png_compression(32),
            CompressionType::Fast
        ));
        assert!(matches!(
            quality_to_png_compression(33),
            CompressionType::Default
        ));
        assert!(matches!(
            quality_to_png_compression(65),
            CompressionType::Default
        ));
        assert!(matches!(
            quality_to_png_compression(66),
            CompressionType::Best
        ));
        assert!(matches!(
            quality_to_png_compression(100),
            CompressionType::Best
        ));
    }
}
