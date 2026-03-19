// Lossy PNG compression via color quantization (24-bit -> 8-bit indexed palette).
//
// Uses quantizr (MIT, zero deps): median cut + Floyd-Steinberg dithering.
// imagequant is GPL so we can't use it. quantizr benchmarks favorably:
//   quantizr:  1051 KB -> 447 KB (57.4%), 144 ms
//   quantette: 1051 KB -> 521 KB (50.4%), 319 ms

use bnto_core::errors::BntoError;
use bnto_core::progress::ProgressReporter;

use crate::orientation::decode_with_orientation;

/// Compress a PNG via lossy color quantization (truecolor -> indexed palette).
/// Called by `CompressImages::compress_png()`.
///
/// `compression` (1-100) controls aggressiveness:
///   - max_colors: 256 (compression=1) → 32 (compression=100)
///   - dithering: 0.3 (compression<33) → 0.5 (<66) → 0.8 (>=66)
pub fn compress_png_quantized(
    data: &[u8],
    compression: u8,
    progress: &ProgressReporter,
) -> Result<Vec<u8>, BntoError> {
    progress.report(10, "Decoding PNG...");
    let img = decode_with_orientation(data)?;
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();

    progress.report(30, "Quantizing colors (median cut)...");
    let settings = QuantizeSettings::from_compression(compression);
    let (qz_image, mut result) = run_quantization(rgba.as_raw(), width, height, &settings)?;

    progress.report(50, "Remapping pixels to palette...");
    let indices = remap_pixels(&qz_image, &mut result, width, height)?;
    let palette_colors = extract_palette(&result);

    progress.report(70, "Encoding indexed PNG...");
    let output = encode_indexed_png(width, height, &palette_colors, &indices)?;

    progress.report(100, "PNG quantization complete");
    Ok(output)
}

/// Quantization settings derived from the compression parameter.
struct QuantizeSettings {
    max_colors: u16,
    dithering: f32,
}

impl QuantizeSettings {
    /// Map compression (1-100) to quantization aggressiveness.
    /// Higher compression → fewer colors + more dithering → smaller file.
    fn from_compression(compression: u8) -> Self {
        let c = compression.clamp(1, 100) as u16;
        // Linear interpolation: 256 (compression=1) → 32 (compression=100)
        let max_colors = (256 - ((c - 1) * 224 / 99)).max(32);
        let dithering = if compression < 33 {
            0.3
        } else if compression < 66 {
            0.5
        } else {
            0.8
        };
        Self {
            max_colors,
            dithering,
        }
    }
}

/// Run median cut quantization with configurable settings.
fn run_quantization<'a>(
    raw: &'a [u8],
    width: u32,
    height: u32,
    settings: &QuantizeSettings,
) -> Result<(quantizr::Image<'a>, quantizr::QuantizeResult), BntoError> {
    let qz_image = quantizr::Image::new(raw, width as usize, height as usize)
        .map_err(|e| BntoError::ProcessingFailed(format!("Quantization setup failed: {e}")))?;

    let mut opts = quantizr::Options::default();
    opts.set_max_colors(settings.max_colors as i32)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid color count: {e}")))?;

    let mut result = quantizr::QuantizeResult::quantize(&qz_image, &opts);

    result
        .set_dithering_level(settings.dithering)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid dither level: {e}")))?;

    Ok((qz_image, result))
}

/// Remap each pixel to its nearest palette index with dithering.
fn remap_pixels(
    qz_image: &quantizr::Image,
    result: &mut quantizr::QuantizeResult,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, BntoError> {
    let pixel_count = (width as usize) * (height as usize);
    let mut indices = vec![0u8; pixel_count];
    result
        .remap_image(qz_image, &mut indices)
        .map_err(|e| BntoError::ProcessingFailed(format!("Pixel remapping failed: {e}")))?;
    Ok(indices)
}

/// Convert quantizr palette to our PaletteColor format.
fn extract_palette(result: &quantizr::QuantizeResult) -> Vec<PaletteColor> {
    let palette = result.get_palette();
    palette.entries[..palette.count as usize]
        .iter()
        .map(|c| PaletteColor {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        })
        .collect()
}

// --- Indexed PNG Encoder ---
// Uses the low-level `png` crate because `image`'s PngEncoder doesn't support
// indexed color mode — it always writes truecolor PNGs.

/// An RGBA color entry for the indexed PNG palette.
#[derive(Clone, Copy)]
struct PaletteColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Encode an indexed PNG from a palette and per-pixel indices.
fn encode_indexed_png(
    width: u32,
    height: u32,
    palette_colors: &[PaletteColor],
    indices: &[u8],
) -> Result<Vec<u8>, BntoError> {
    let mut output = Vec::with_capacity(indices.len());

    {
        // Scoped block so the encoder's mutable borrow of `output` ends
        // before we return it.
        let mut encoder = png::Encoder::new(&mut output, width, height);
        configure_encoder(&mut encoder, palette_colors);

        let mut writer = encoder
            .write_header()
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG header write failed: {e}")))?;
        writer
            .write_image_data(indices)
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG data write failed: {e}")))?;
    }

    Ok(output)
}

/// Set up the PNG encoder: indexed color type, palette (PLTE), transparency
/// (tRNS if needed), and high compression.
fn configure_encoder(encoder: &mut png::Encoder<&mut Vec<u8>>, palette_colors: &[PaletteColor]) {
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Eight);

    // PLTE chunk: RGB triplets [R0, G0, B0, R1, G1, B1, ...]
    let plte: Vec<u8> = palette_colors
        .iter()
        .flat_map(|c| [c.r, c.g, c.b])
        .collect();
    encoder.set_palette(plte);

    // tRNS chunk: only needed if any entry has alpha < 255.
    let trns: Vec<u8> = palette_colors.iter().map(|c| c.a).collect();
    if trns.iter().any(|&a| a < 255) {
        encoder.set_trns(trns);
    }

    // High compression — indexed data (1 byte/pixel) compresses well.
    encoder.set_compression(png::Compression::High);
    encoder.set_filter(png::Filter::Sub);
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // We use the medium PNG fixture for testing — it's 400x400 pixels (~180 KB),
    // large enough to see real compression gains but small enough for fast tests.
    static MEDIUM_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.png");

    // The large PNG fixture — 1.0 MB, used for benchmark-style size verification.
    static LARGE_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/large.png");

    /// Helper: verify that output bytes start with PNG magic bytes.
    fn is_valid_png(data: &[u8]) -> bool {
        data.len() >= 8 && data[..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    }

    #[test]
    fn quantized_png_is_valid() {
        let progress = ProgressReporter::new_noop();
        let result = compress_png_quantized(MEDIUM_PNG, 50, &progress).unwrap();
        assert!(is_valid_png(&result), "Output should be a valid PNG");
    }

    #[test]
    fn quantized_png_reduces_size_significantly() {
        let progress = ProgressReporter::new_noop();
        let result = compress_png_quantized(MEDIUM_PNG, 50, &progress).unwrap();

        let original_kb = MEDIUM_PNG.len() / 1024;
        let output_kb = result.len() / 1024;
        let reduction_pct = (1.0 - result.len() as f64 / MEDIUM_PNG.len() as f64) * 100.0;

        println!(
            "quantizr: {} KB → {} KB ({:.1}% reduction)",
            original_kb, output_kb, reduction_pct
        );

        // We expect at least 30% reduction on a photographic PNG.
        // TinyPNG gets ~65%, quantizr gets ~55-57%.
        assert!(
            reduction_pct > 30.0,
            "Expected >30% reduction, got {reduction_pct:.1}%"
        );
    }

    #[test]
    fn quantized_png_handles_large_image() {
        let progress = ProgressReporter::new_noop();
        let result = compress_png_quantized(LARGE_PNG, 50, &progress).unwrap();

        let original_kb = LARGE_PNG.len() / 1024;
        let output_kb = result.len() / 1024;
        let reduction_pct = (1.0 - result.len() as f64 / LARGE_PNG.len() as f64) * 100.0;

        println!(
            "quantizr (large): {} KB → {} KB ({:.1}% reduction)",
            original_kb, output_kb, reduction_pct
        );

        assert!(is_valid_png(&result));
        assert!(
            reduction_pct > 30.0,
            "Expected >30% reduction on large PNG, got {reduction_pct:.1}%"
        );
    }

    #[test]
    fn higher_compression_produces_smaller_png() {
        let progress = ProgressReporter::new_noop();
        let light = compress_png_quantized(MEDIUM_PNG, 20, &progress).unwrap();
        let heavy = compress_png_quantized(MEDIUM_PNG, 80, &progress).unwrap();

        assert!(
            heavy.len() < light.len(),
            "Compression 80 ({} bytes) should be smaller than compression 20 ({} bytes)",
            heavy.len(),
            light.len()
        );
    }

    #[test]
    fn quantize_settings_boundary_values() {
        let low = QuantizeSettings::from_compression(1);
        assert_eq!(low.max_colors, 256);
        assert!((low.dithering - 0.3).abs() < f32::EPSILON);

        let high = QuantizeSettings::from_compression(100);
        assert_eq!(high.max_colors, 32);
        assert!((high.dithering - 0.8).abs() < f32::EPSILON);

        let mid = QuantizeSettings::from_compression(50);
        assert!(mid.max_colors > 32 && mid.max_colors < 256);
        assert!((mid.dithering - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn encode_indexed_png_produces_valid_output() {
        // Create a tiny 2x2 indexed image with 2 colors
        let palette = vec![
            PaletteColor {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }, // Red
            PaletteColor {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            }, // Blue
        ];
        let indices = vec![0, 1, 1, 0]; // Checkerboard pattern

        let result = encode_indexed_png(2, 2, &palette, &indices).unwrap();
        assert!(is_valid_png(&result));
    }

    #[test]
    fn encode_indexed_png_with_transparency() {
        // Test that alpha < 255 produces a valid PNG with tRNS chunk
        let palette = vec![
            PaletteColor {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }, // Opaque red
            PaletteColor {
                r: 0,
                g: 0,
                b: 255,
                a: 128,
            }, // Semi-transparent blue
        ];
        let indices = vec![0, 1, 1, 0];

        let result = encode_indexed_png(2, 2, &palette, &indices).unwrap();
        assert!(is_valid_png(&result));
    }
}
