// =============================================================================
// PNG Color Quantization — Lossy Palette Reduction for Smaller PNGs
// =============================================================================
//
// WHAT IS THIS FILE?
// This module implements lossy PNG compression via color quantization using
// quantizr (MIT licensed, zero dependencies). Instead of just optimizing
// the DEFLATE compression (lossless), we reduce the number of colors in the
// image from millions (24-bit truecolor) down to 256 (8-bit indexed palette)
// BEFORE compressing. This is exactly what TinyPNG does — and it's why we
// get ~57% reduction instead of ~5% with lossless alone.
//
// HOW DOES COLOR QUANTIZATION WORK?
//
//   A typical photo uses thousands of unique colors. The human eye can't
//   distinguish many of these — similar greens, slightly different browns.
//   Quantization finds the "best" 256 colors to represent the whole image:
//
//   1. ANALYZE — Look at all pixels, find clusters of similar colors
//   2. PALETTE — Pick 256 representative colors (the "palette")
//   3. REMAP — Replace each pixel's color with the nearest palette entry
//   4. DITHER — Add subtle noise to hide color banding artifacts
//   5. ENCODE — Write as an indexed PNG (palette + 1 byte per pixel)
//
//   The result: each pixel is 1 byte (palette index) instead of 3-4 bytes
//   (RGB or RGBA), AND the indexed data compresses much better with DEFLATE
//   because there are only 256 possible values per byte.
//
// WHY quantizr?
// We can't use imagequant (the gold standard) because it's GPL v3 and we're
// MIT. quantizr is an MIT-licensed alternative using median cut quantization
// + Floyd-Steinberg dithering. Zero dependencies, simple API. Benchmarked
// against quantette (Oklab + k-means):
//
//   quantizr:  1051 KB → 447 KB (57.4%), 144 ms  ← WINNER
//   quantette: 1051 KB → 521 KB (50.4%), 319 ms
//
// quantizr wins on both output size AND speed.

use bnto_core::errors::BntoError;
use bnto_core::progress::ProgressReporter;

use crate::orientation::decode_with_orientation;

// =============================================================================
// Public API — Compress a PNG Using Color Quantization
// =============================================================================

/// Compress a PNG by reducing its color palette from millions of colors
/// to 256 (8-bit indexed), then encoding as an indexed PNG with DEFLATE.
///
/// This is the "lossy PNG" path — the same technique TinyPNG uses.
/// Called by `CompressImages::compress_png()` for all PNG compression.
///
/// # How It Works
/// 1. Decode the PNG into raw RGBA pixel data
/// 2. Feed pixels to quantizr → get a 256-color palette
/// 3. Apply Floyd-Steinberg dithering to hide banding artifacts
/// 4. Remap each pixel to its nearest palette index
/// 5. Encode as an indexed PNG (palette + 1 byte per pixel)
///
/// # Performance
/// On a 1.0 MB PNG: ~144 ms, output ~447 KB (57% reduction)
pub fn compress_png_quantized(
    data: &[u8],
    progress: &ProgressReporter,
) -> Result<Vec<u8>, BntoError> {
    // --- Step 1: Decode the PNG into raw pixel data ---
    progress.report(10, "Decoding PNG...");
    let img = decode_with_orientation(data)?;

    // Convert to RGBA8 — quantizr expects raw RGBA bytes.
    // RUST CONCEPT: `.into_rgba8()` consumes the DynamicImage and returns
    // an RgbaImage (Vec<u8> of RGBA pixel data, 4 bytes per pixel).
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();

    // --- Step 2: Quantize — find the best 256 colors ---
    progress.report(30, "Quantizing colors (median cut)...");

    // Create a quantizr Image from the raw RGBA pixel buffer.
    // `quantizr::Image::new()` takes a &[u8] of RGBA bytes + dimensions.
    // It returns a Result because it validates that len == width * height * 4.
    let qz_image = quantizr::Image::new(rgba.as_raw(), width as usize, height as usize)
        .map_err(|e| BntoError::ProcessingFailed(format!("Quantization setup failed: {e}")))?;

    // Configure: max 256 colors (the most an 8-bit palette can hold)
    let mut opts = quantizr::Options::default();
    opts.set_max_colors(256)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid color count: {e}")))?;

    // Run the quantization — this is the expensive step.
    // It analyzes all pixels, builds a color histogram, and finds the
    // optimal 256-color palette using median cut partitioning.
    let mut result = quantizr::QuantizeResult::quantize(&qz_image, &opts);

    // Enable dithering at 50% strength.
    // 0.0 = no dithering (visible color banding on gradients)
    // 1.0 = full dithering (can look noisy on flat areas)
    // 0.5 = good balance for photographic content
    result
        .set_dithering_level(0.5)
        .map_err(|e| BntoError::ProcessingFailed(format!("Invalid dither level: {e}")))?;

    // --- Step 3: Remap — replace each pixel with its palette index ---
    progress.report(50, "Remapping pixels to palette...");

    // Allocate the index buffer — one byte per pixel. Each byte is a
    // palette index (0-255) pointing to the color in the palette.
    let pixel_count = (width as usize) * (height as usize);
    let mut indices = vec![0u8; pixel_count];

    // Remap applies dithering while mapping pixels to palette indices.
    // It walks each pixel, finds the closest palette color, records
    // the index, and diffuses the color error to neighbors.
    result
        .remap_image(&qz_image, &mut indices)
        .map_err(|e| BntoError::ProcessingFailed(format!("Pixel remapping failed: {e}")))?;

    // Get the final palette — up to 256 RGBA colors.
    let palette = result.get_palette();

    // --- Step 4: Encode as indexed PNG ---
    progress.report(70, "Encoding indexed PNG...");

    // Convert quantizr's palette entries to RGBA tuples for the encoder.
    // `palette.entries` is a fixed-size array; `palette.count` tells us
    // how many entries are actually used (could be fewer than 256).
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
// Indexed PNG Encoder — Writes Palette-Based PNGs
// =============================================================================
//
// Quantization produces:
//   - A palette of up to 256 RGBA colors
//   - An array of indices (one byte per pixel) into that palette
//
// This function takes those and writes a valid indexed PNG file using the
// low-level `png` crate. We can't use the `image` crate's PngEncoder here
// because it always writes truecolor PNGs — it doesn't support indexed mode.
//
// AN INDEXED PNG STRUCTURE:
//   IHDR — image dimensions + color type 3 (indexed)
//   PLTE — palette chunk: N * 3 bytes (RGB triplets)
//   tRNS — transparency chunk: N bytes (alpha per palette entry)
//   IDAT — image data: one byte per pixel (palette indices)
//   IEND — end marker

/// An RGBA color entry for the indexed PNG palette.
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
    // Allocate a buffer for the PNG output. We estimate the output will be
    // smaller than the raw indices (DEFLATE compression), but we start with
    // a generous buffer to avoid reallocations.
    let mut output = Vec::with_capacity(indices.len());

    {
        // Create a PNG encoder targeting our output buffer.
        //
        // RUST CONCEPT: The `{}` block creates a scope. The encoder borrows
        // `output` mutably. By putting it in a block, the borrow ends when
        // the block ends, and we can return `output` afterward.
        let mut encoder = png::Encoder::new(&mut output, width, height);

        // Set color type to Indexed (palette-based).
        // `BitDepth::Eight` means each pixel is one byte (index 0-255).
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);

        // Set the PLTE chunk — the color palette.
        // The png crate expects RGB triplets: [R0, G0, B0, R1, G1, B1, ...]
        let plte: Vec<u8> = palette_colors
            .iter()
            .flat_map(|c| [c.r, c.g, c.b])
            .collect();
        encoder.set_palette(plte);

        // Set the tRNS chunk — alpha transparency for each palette entry.
        // Only needed if any palette entry has alpha < 255.
        let trns: Vec<u8> = palette_colors.iter().map(|c| c.a).collect();
        let has_transparency = trns.iter().any(|&a| a < 255);
        if has_transparency {
            encoder.set_trns(trns);
        }

        // Set compression to maximum effort — same as our lossless path.
        // The indexed data (1 byte per pixel, only 256 possible values)
        // compresses MUCH better than truecolor data (3-4 bytes per pixel).
        encoder.set_compression(png::Compression::High);
        encoder.set_filter(png::Filter::Sub);

        // Write the header and then the image data.
        let mut writer = encoder
            .write_header()
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG header write failed: {e}")))?;

        writer
            .write_image_data(indices)
            .map_err(|e| BntoError::ProcessingFailed(format!("PNG data write failed: {e}")))?;

        // RUST CONCEPT: `writer` is dropped here when the block ends.
        // The `Drop` impl flushes any remaining data and writes IEND.
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
