// =============================================================================
// PNG Color Quantization — Lossy Palette Reduction for Smaller PNGs
// =============================================================================
//
// Lossy PNG compression via color quantization using quantizr (MIT, zero deps).
// Reduces colors from millions (24-bit truecolor) to 256 (8-bit indexed palette)
// BEFORE DEFLATE compression — the same technique TinyPNG uses.
//
// Algorithm: median cut quantization + Floyd-Steinberg dithering
//   1. Analyze pixels, find color clusters
//   2. Pick 256 representative colors (the palette)
//   3. Remap each pixel to nearest palette entry
//   4. Dither to hide color banding artifacts
//   5. Encode as indexed PNG (1 byte per pixel instead of 3-4)
//
// Why quantizr over imagequant? imagequant is GPL v3, we're MIT.
// Benchmarked against quantette (Oklab + k-means):
//   quantizr:  1051 KB -> 447 KB (57.4%), 144 ms  <- WINNER
//   quantette: 1051 KB -> 521 KB (50.4%), 319 ms

use bnto_core::errors::BntoError;
use bnto_core::progress::ProgressReporter;

use crate::orientation::decode_with_orientation;

// =============================================================================
// Public API
// =============================================================================

/// Compress a PNG by reducing its color palette to 256 (8-bit indexed).
/// Called by `CompressImages::compress_png()` for all PNG compression.
///
/// Performance: ~144 ms on a 1.0 MB PNG, output ~447 KB (57% reduction)
pub fn compress_png_quantized(
    data: &[u8],
    progress: &ProgressReporter,
) -> Result<Vec<u8>, BntoError> {
    progress.report(10, "Decoding PNG...");
    let img = decode_with_orientation(data)?;

    // quantizr expects raw RGBA bytes (4 bytes per pixel).
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();

    // --- Quantize: find the best 256 colors ---
    progress.report(30, "Quantizing colors (median cut)...");

    let qz_image = quantizr::Image::new(rgba.as_raw(), width as usize, height as usize)
        .map_err(|e| BntoError::ProcessingFailed(format!("Quantization setup failed: {e}")))?;

    let mut opts = quantizr::Options::default();
    opts.set_max_colors(256)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid color count: {e}")))?;

    let mut result = quantizr::QuantizeResult::quantize(&qz_image, &opts);

    // Dithering at 50%: 0.0=banding on gradients, 1.0=noisy on flat areas
    result
        .set_dithering_level(0.5)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid dither level: {e}")))?;

    // --- Remap: replace each pixel with its palette index ---
    progress.report(50, "Remapping pixels to palette...");

    let pixel_count = (width as usize) * (height as usize);
    let mut indices = vec![0u8; pixel_count];

    // Remap applies dithering while mapping pixels to palette indices —
    // finds closest palette color, records index, diffuses error to neighbors.
    result
        .remap_image(&qz_image, &mut indices)
        .map_err(|e| BntoError::ProcessingFailed(format!("Pixel remapping failed: {e}")))?;

    let palette = result.get_palette();

    // --- Encode as indexed PNG ---
    progress.report(70, "Encoding indexed PNG...");

    let palette_colors: Vec<PaletteColor> = palette.entries[..palette.count as usize]
        .iter()
        .map(|c| PaletteColor {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        })
        .collect();

    let output = encode_indexed_png(width, height, &palette_colors, &indices)?;

    progress.report(100, "PNG quantization complete");
    Ok(output)
}

// =============================================================================
// Indexed PNG Encoder
// =============================================================================
//
// We use the low-level `png` crate because the `image` crate's PngEncoder
// always writes truecolor PNGs — it doesn't support indexed mode.
//
// Indexed PNG structure:
//   IHDR — dimensions + color type 3 (indexed)
//   PLTE — palette: N * 3 bytes (RGB triplets)
//   tRNS — transparency: N bytes (alpha per palette entry)
//   IDAT — image data: one byte per pixel (palette indices)
//   IEND — end marker

#[derive(Clone, Copy)]
struct PaletteColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn encode_indexed_png(
    width: u32,
    height: u32,
    palette_colors: &[PaletteColor],
    indices: &[u8],
) -> Result<Vec<u8>, BntoError> {
    let mut output = Vec::with_capacity(indices.len());

    {
        // Scoped borrow: encoder borrows `output` mutably. The borrow
        // ends when the block ends, letting us return `output` afterward.
        let mut encoder = png::Encoder::new(&mut output, width, height);

        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);

        // PLTE chunk: RGB triplets
        let plte: Vec<u8> = palette_colors
            .iter()
            .flat_map(|c| [c.r, c.g, c.b])
            .collect();
        encoder.set_palette(plte);

        // tRNS chunk: only needed if any entry has alpha < 255
        let trns: Vec<u8> = palette_colors.iter().map(|c| c.a).collect();
        let has_transparency = trns.iter().any(|&a| a < 255);
        if has_transparency {
            encoder.set_trns(trns);
        }

        // Indexed data (256 possible values per byte) compresses much
        // better than truecolor (3-4 bytes per pixel).
        encoder.set_compression(png::Compression::High);
        encoder.set_filter(png::Filter::Sub);

        let mut writer = encoder
            .write_header()
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG header write failed: {e}")))?;

        writer
            .write_image_data(indices)
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG data write failed: {e}")))?;

        // Writer drop flushes remaining data and writes IEND.
    }

    Ok(output)
}

// =============================================================================
// Tests
// =============================================================================

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
        let result = compress_png_quantized(MEDIUM_PNG, &progress).unwrap();
        assert!(is_valid_png(&result), "Output should be a valid PNG");
    }

    #[test]
    fn quantized_png_reduces_size_significantly() {
        let progress = ProgressReporter::new_noop();
        let result = compress_png_quantized(MEDIUM_PNG, &progress).unwrap();

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
        let result = compress_png_quantized(LARGE_PNG, &progress).unwrap();

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
