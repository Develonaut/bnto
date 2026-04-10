// Shared image encoding — single entrypoint for all image processors.
//
// All processors that produce raster output route through this module
// for final encoding. This prevents duplicated encode functions and ensures
// quality/compression params are consistently applied across all formats.

use std::io::Cursor;

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
/// - **WebP**: Lossless only (image crate limitation). Quality param is accepted
///   but has no effect until lossy WebP support is added via jSquash.
pub fn encode_image(
    img: &DynamicImage,
    format: ImageFormat,
    quality: u8,
) -> Result<Vec<u8>, BntoError> {
    match format {
        ImageFormat::Jpeg => encode_jpeg(img, quality),
        ImageFormat::Png => encode_png(img, quality),
        ImageFormat::WebP => encode_webp(img),
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

fn encode_webp(img: &DynamicImage) -> Result<Vec<u8>, BntoError> {
    let mut output = Vec::new();
    let mut cursor_out = Cursor::new(&mut output);
    img.write_to(&mut cursor_out, image::ImageFormat::WebP)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;
    Ok(output)
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
    use image::{DynamicImage, Rgb, RgbImage};

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
