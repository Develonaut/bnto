// =============================================================================
// Compress Images Node — Reduce Image File Size
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the `compress-images` node. When a user drops images on the
// /compress-images page and clicks "Run", this code does the actual work.
//
// HOW DOES IMAGE COMPRESSION WORK?
//
//   JPEG compression (lossy):
//     JPEG works by dividing the image into 8x8 pixel blocks, converting
//     colors to frequency data (DCT transform), and then throwing away
//     the "least important" frequency information. The quality setting
//     (1-100) controls how much gets thrown away:
//       - 100 = keep almost everything (large file, best quality)
//       - 80 = good balance (noticeably smaller, minor quality loss)
//       - 50 = aggressive (much smaller, visible quality loss)
//
//   PNG compression (lossless):
//     PNG uses the DEFLATE algorithm (same as ZIP files). The image data
//     is first filtered (each row is encoded as the difference from the
//     previous row) and then compressed. The compression LEVEL controls
//     how hard the encoder tries to find the best compression:
//       - Fast = minimal effort (larger file, fast encoding)
//       - Default = balanced
//       - Best = maximum effort (smallest file, slow encoding)
//     IMPORTANT: PNG compression is ALWAYS lossless — no quality loss!
//
//   WebP compression (lossless only, for now):
//     Our Rust WebP encoder only supports lossless mode. It encodes the
//     image using predictive coding (each pixel predicted from neighbors).
//     Lossy WebP support will be added later via a JS fallback (jSquash).
//
// THE COMPRESSION PIPELINE:
//   1. Detect format (JPEG, PNG, WebP) from magic bytes or file extension
//   2. Decode: raw bytes → pixel data (DynamicImage)
//   3. Re-encode: pixel data → compressed bytes with our settings
//   4. Return the compressed bytes + metadata (original size, new size, ratio)

use std::io::Cursor;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::orientation::decode_with_orientation;

use crate::format::ImageFormat;

// =============================================================================
// Configuration Constants
// =============================================================================

/// Default JPEG quality when the user doesn't specify one.
/// 80 is a widely-accepted sweet spot: significant file size reduction
/// with barely noticeable quality loss for most photos.
const DEFAULT_JPEG_QUALITY: u8 = 80;

/// Minimum JPEG quality we allow. Below this, images look terrible.
/// We enforce this as a safety net so users can't accidentally destroy
/// their images.
const MIN_JPEG_QUALITY: u8 = 1;

/// Maximum JPEG quality. 100 = virtually no compression.
const MAX_JPEG_QUALITY: u8 = 100;

// =============================================================================
// The CompressImages Processor
// =============================================================================

/// The compress-images node processor.
///
/// This struct implements the `NodeProcessor` trait from bnto-core,
/// which means it can be plugged into the Web Worker orchestrator
/// alongside any other node type.
///
/// RUST CONCEPT: "Zero-Sized Type" (ZST)
/// This struct has no fields — it's an empty struct. In Rust, this takes
/// up ZERO bytes of memory. We use it just to "carry" the trait
/// implementation. It's like a stateless class in Java/TypeScript —
/// all the behavior comes from the methods, not from stored data.
pub struct CompressImages;

// RUST CONCEPT: `Default` trait
// The `Default` trait provides a way to create a "default" instance of a type.
// It's like having a no-argument constructor. Clippy (the Rust linter) insists
// that if you have a `new()` function that takes no arguments, you should also
// implement `Default`. This lets people write `CompressImages::default()` or
// use it in contexts where `Default` is expected (like `Option::unwrap_or_default()`).
impl Default for CompressImages {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressImages {
    /// Create a new CompressImages processor.
    ///
    /// RUST CONCEPT: `pub fn new() -> Self`
    /// This is a "constructor" by convention (not a language feature).
    /// Rust doesn't have constructors like `new MyClass()` in JS —
    /// instead, we write a function called `new()` that returns `Self`.
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Internal Methods — The Actual Compression Logic
    // =========================================================================

    /// Compress a JPEG image by re-encoding at a specific quality level.
    ///
    /// Arguments:
    ///   - `data` — the raw JPEG bytes from the user's file
    ///   - `quality` — compression quality (1-100, higher = better quality)
    ///   - `progress` — callback to report progress to the UI
    ///
    /// Returns the compressed JPEG bytes, or an error if something went wrong.
    fn compress_jpeg(
        &self,
        data: &[u8],
        quality: u8,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        // --- Step 1: Decode the JPEG into raw pixel data ---
        //
        // `Cursor::new(data)` wraps our byte slice in a "cursor" — a fake
        // file handle that reads from memory instead of disk. The `image`
        // crate needs something that implements `Read` (can read bytes
        // sequentially), and `Cursor` provides that.
        //
        // `ImageReader::new(cursor)` creates an image reader.
        // `.with_guessed_format()` looks at the first bytes to figure out
        //   the format (JPEG, PNG, etc.)
        // `.map_err(...)` converts any IO error into our BntoError type.
        // `?` is the "try operator" — if this returns Err, the whole
        //   function immediately returns that error. It's like throwing
        //   an exception in JS, but explicit.
        progress.report(10, "Decoding JPEG...");

        // Decode with EXIF orientation applied. This ensures smartphone
        // photos (which store pixels sideways + an EXIF tag) come out
        // correctly oriented after compression.
        let img = decode_with_orientation(data)?;

        // --- Step 2: Re-encode with the requested quality ---
        //
        // `Vec::new()` creates an empty byte buffer. The encoder writes
        // the compressed JPEG data into this buffer.
        //
        // `JpegEncoder::new_with_quality(&mut output, quality)` creates
        // a JPEG encoder that writes to `output` with the given quality.
        // The `&mut` means the encoder needs to MODIFY output (write to it).
        //
        // `img.write_with_encoder(encoder)` feeds the decoded pixel data
        // through the encoder, producing compressed JPEG bytes in `output`.
        progress.report(50, "Compressing JPEG...");

        let mut output = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut output, quality);
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;

        progress.report(100, "JPEG compression complete");
        Ok(output)
    }

    /// Optimize a PNG image by re-encoding with maximum compression.
    ///
    /// Unlike JPEG, PNG compression is LOSSLESS — no quality loss, ever.
    /// We control the compression LEVEL (how hard the encoder tries)
    /// and the filter strategy (how rows are pre-processed before compression).
    fn compress_png(&self, data: &[u8], progress: &ProgressReporter) -> Result<Vec<u8>, BntoError> {
        // --- Step 1: Decode the PNG into raw pixel data ---
        progress.report(10, "Decoding PNG...");

        // Decode with EXIF orientation applied. PNG doesn't have EXIF,
        // so this is effectively a plain decode — but using the shared
        // function ensures consistency across all image nodes.
        let img = decode_with_orientation(data)?;

        // --- Step 2: Re-encode with optimized compression settings ---
        //
        // `CompressionType::Best` — maximum compression effort. The encoder
        //   will try harder to find the best DEFLATE compression, producing
        //   smaller files but taking longer. Worth it because this runs
        //   once and the user downloads a smaller file.
        //
        // `FilterType::Adaptive` — automatically selects the best PNG
        //   row filter for each row. PNG filters pre-process each row
        //   to make it more compressible. "Adaptive" tries all filter
        //   types per row and picks the one that compresses best.
        //   This is the gold standard for PNG optimization.
        progress.report(50, "Optimizing PNG...");

        let mut output = Vec::new();
        let encoder =
            PngEncoder::new_with_quality(&mut output, CompressionType::Best, FilterType::Adaptive);
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode PNG: {e}")))?;

        progress.report(100, "PNG optimization complete");
        Ok(output)
    }

    /// Re-encode a WebP image with lossless compression.
    ///
    /// NOTE: The `image` crate's WebP encoder only supports LOSSLESS mode.
    /// This means WebP files may actually get LARGER if the original was
    /// lossy-compressed. For now, this is a limitation we document.
    /// Lossy WebP will be added later via jSquash JS fallback.
    fn compress_webp(
        &self,
        data: &[u8],
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        // --- Step 1: Decode the WebP into raw pixel data ---
        progress.report(10, "Decoding WebP...");

        // Decode with EXIF orientation applied.
        let img = decode_with_orientation(data)?;

        // --- Step 2: Re-encode as lossless WebP ---
        //
        // The `image` crate's WebP encoder uses lossless mode by default.
        // We write the image to a buffer and let the encoder do its thing.
        //
        // For lossy WebP with quality control, we'd need the Google
        // libwebp library (C code). That doesn't compile to our WASM
        // target. The jSquash JS library wraps a pre-compiled WASM
        // version of libwebp — that's the future fallback path.
        progress.report(50, "Compressing WebP (lossless)...");

        let mut output = Vec::new();
        // Use Cursor for the output buffer so it implements Write + Seek
        let mut cursor_out = Cursor::new(&mut output);
        img.write_to(&mut cursor_out, image::ImageFormat::WebP)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;

        progress.report(100, "WebP compression complete");
        Ok(output)
    }

    /// Extract the JPEG quality parameter from the node's config params.
    ///
    /// The config comes from the `.bnto.json` workflow definition or the
    /// UI's settings panel. If no quality is specified, we use the default.
    ///
    /// RUST CONCEPT: `serde_json::Map<String, serde_json::Value>`
    /// This is a JSON object — keys are strings, values can be anything
    /// (string, number, bool, array, object, null). We need to extract
    /// a specific key ("quality") and convert it to a u8 number.
    fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        // Try to find the "quality" key in the params.
        //
        // RUST CONCEPT: `.get("quality")`
        // Returns `Option<&Value>` — either `Some(&value)` if the key
        // exists, or `None` if it doesn't.
        //
        // RUST CONCEPT: `.and_then(|v| v.as_u64())`
        // `.and_then()` chains Option operations. If `get()` returned
        // `Some(value)`, try to convert it to a u64 (unsigned 64-bit int).
        // `as_u64()` returns `Some(number)` if the JSON value is a number,
        // or `None` if it's a string, bool, etc.
        //
        // RUST CONCEPT: `.map(|q| q as u8)`
        // If we have a number, cast it from u64 to u8. `as` is Rust's
        // type casting operator (like `(u8)value` in C or `value as number`
        // in TypeScript, but only for numeric types).
        //
        // RUST CONCEPT: `.unwrap_or(DEFAULT_JPEG_QUALITY)`
        // If any step in the chain returned None, use the default.
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_JPEG_QUALITY)
            // Clamp the quality to our valid range.
            // `.clamp(min, max)` ensures the value is between min and max.
            .clamp(MIN_JPEG_QUALITY, MAX_JPEG_QUALITY)
    }

    /// Generate an output filename from the input filename.
    ///
    /// Adds "-compressed" before the extension:
    ///   "photo.jpg" → "photo-compressed.jpg"
    ///   "image.png" → "image-compressed.png"
    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        // Split the filename into stem (name without extension) and extension.
        //
        // RUST CONCEPT: `.rfind('.')`
        // `.rfind()` searches from the RIGHT (end of string) for a character.
        // Returns `Some(index)` if found, `None` if not. We search from the
        // right because filenames can contain multiple dots (e.g., "my.file.jpg").
        if let Some(dot_pos) = input_filename.rfind('.') {
            let stem = &input_filename[..dot_pos];
            format!("{stem}-compressed.{}", format.extension())
        } else {
            // No extension found — just append
            format!("{input_filename}-compressed.{}", format.extension())
        }
    }
}

// =============================================================================
// NodeProcessor Trait Implementation
// =============================================================================
//
// This is where CompressImages "plugs into" the Bnto engine framework.
// By implementing `NodeProcessor`, this node can be run by the Web Worker
// orchestrator just like any other node type.

impl NodeProcessor for CompressImages {
    /// The unique identifier for this node type.
    /// Matches the slug in the bnto registry and the Go engine's node name.
    fn name(&self) -> &str {
        "compress-images"
    }

    /// Process a single image: detect format, decode, re-encode with compression.
    ///
    /// This is the main entry point. The Web Worker calls this once per file.
    /// For multiple files, the Worker calls it in a loop.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // --- Step 1: Figure out what format this image is ---
        let format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        // --- Step 2: Compress based on format ---
        //
        // Each format has its own compression strategy:
        //   - JPEG: re-encode at a lower quality level (lossy)
        //   - PNG: re-encode with maximum compression effort (lossless)
        //   - WebP: re-encode as lossless WebP
        let original_size = input.data.len();

        let compressed_data = match format {
            ImageFormat::Jpeg => {
                let quality = Self::get_quality(&input.params);
                progress.report(5, &format!("Compressing JPEG (quality: {quality})..."));
                self.compress_jpeg(&input.data, quality, progress)?
            }
            ImageFormat::Png => {
                progress.report(5, "Optimizing PNG...");
                self.compress_png(&input.data, progress)?
            }
            ImageFormat::WebP => {
                progress.report(5, "Compressing WebP (lossless)...");
                self.compress_webp(&input.data, progress)?
            }
        };

        // --- Step 3: Build the output ---
        let compressed_size = compressed_data.len();
        let output_filename = Self::output_filename(&input.filename, format);

        // Calculate compression ratio for the metadata.
        // This shows up in the UI's results panel so the user can see
        // how much space they saved.
        //
        // RUST CONCEPT: `as f64`
        // We cast to f64 (64-bit float, like JS `number`) for the division
        // because integer division in Rust truncates (5 / 3 = 1, not 1.67).
        let ratio = if original_size > 0 {
            (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
        } else {
            0.0
        };

        // Build the metadata map (JSON object).
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(original_size as u64)),
        );
        metadata.insert(
            "compressedSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(compressed_size as u64)),
        );
        // serde_json::Number::from_f64 returns Option because NaN/Infinity
        // aren't valid JSON. We unwrap safely because our ratio is always
        // a finite number (we checked original_size > 0).
        if let Some(ratio_num) = serde_json::Number::from_f64(ratio) {
            metadata.insert(
                "compressionRatio".to_string(),
                serde_json::Value::Number(ratio_num),
            );
        }
        metadata.insert(
            "format".to_string(),
            serde_json::Value::String(format!("{:?}", format)),
        );

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: compressed_data,
                filename: output_filename,
                mime_type: format.mime_type().to_string(),
            }],
            metadata,
        })
    }

    /// Validate compression parameters before processing.
    ///
    /// Catches configuration errors early — e.g., quality out of range.
    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        // Check quality parameter if provided
        if let Some(quality_val) = params.get("quality") {
            // RUST CONCEPT: `match` on multiple patterns
            // We check if the value is a valid number in range.
            match quality_val.as_u64() {
                Some(q) if q >= MIN_JPEG_QUALITY as u64 && q <= MAX_JPEG_QUALITY as u64 => {
                    // Valid quality value — nothing to report
                }
                Some(q) => {
                    errors.push(format!(
                        "Quality must be between {MIN_JPEG_QUALITY} and {MAX_JPEG_QUALITY}, got {q}"
                    ));
                }
                None => {
                    errors.push(format!("Quality must be a number, got: {quality_val}"));
                }
            }
        }

        errors
    }
}

// =============================================================================
// Tests — Written TDD-style: tests defined BEFORE implementation
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    // =========================================================================
    // Test Fixtures — Real images from our shared test-fixtures directory
    // =========================================================================
    //
    // `include_bytes!()` embeds these files at compile time. If any file is
    // missing, the build fails with a clear error message pointing to the
    // exact path. This is better than runtime errors.

    /// A small JPEG image (100x100, ~2.7 KB)
    const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

    /// A small PNG image (100x100, ~11 KB)
    const TEST_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");

    /// A small WebP image (100x100, ~966 bytes)
    const TEST_WEBP: &[u8] = include_bytes!("../../../../test-fixtures/images/small.webp");

    /// A medium PNG image (400x400, ~180 KB) — better for compression testing
    /// because there's more room to compress.
    const TEST_MEDIUM_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.png");

    /// A medium JPEG image (400x400, ~23 KB)
    const TEST_MEDIUM_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.jpg");

    // =========================================================================
    // Helper Functions
    // =========================================================================

    /// Create a test input with the given data, filename, and optional params.
    ///
    /// Used throughout the tests to avoid repeating the NodeInput construction.
    fn make_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        }
    }

    /// Create a test input with quality parameter set.
    fn make_input_with_quality(data: &[u8], filename: &str, quality: u64) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(quality)),
        );
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

    /// A no-op progress reporter for tests (no JS runtime needed).
    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

    // =========================================================================
    // Basic Trait Implementation Tests
    // =========================================================================

    #[test]
    fn test_name_is_compress_images() {
        let processor = CompressImages::new();
        assert_eq!(processor.name(), "compress-images");
    }

    #[test]
    fn test_validate_passes_with_no_params() {
        // No params = use defaults. Should validate fine.
        let processor = CompressImages::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_passes_with_valid_quality() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(80u64)),
        );
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_fails_with_quality_out_of_range() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(150u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("between"));
    }

    #[test]
    fn test_validate_fails_with_non_numeric_quality() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::String("high".to_string()),
        );
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("must be a number"));
    }

    // =========================================================================
    // JPEG Compression Tests
    // =========================================================================

    #[test]
    fn test_compress_jpeg_produces_valid_output() {
        // The most basic test: give it a JPEG, get back a JPEG.
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_JPEG, "photo.jpg");

        let output = processor.process(input, &progress).unwrap();

        // Should produce exactly one output file
        assert_eq!(output.files.len(), 1);

        // Output should be a JPEG (check magic bytes)
        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(&out_file.data),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG"
        );

        // Filename should include "-compressed"
        assert!(
            out_file.filename.contains("-compressed"),
            "Output filename should contain '-compressed': got '{}'",
            out_file.filename
        );

        // MIME type should be correct
        assert_eq!(out_file.mime_type, "image/jpeg");
    }

    #[test]
    fn test_compress_jpeg_default_quality_produces_smaller_file() {
        // Default quality (80) should produce a file that's the same size
        // or smaller than quality 90 input. The test JPEG was encoded at
        // quality 90, so re-encoding at 80 should be smaller.
        let processor = CompressImages::new();
        let progress = noop_progress();

        // Use the medium JPEG for a more meaningful compression test.
        let input = make_input(TEST_MEDIUM_JPEG, "photo.jpg");
        let original_size = TEST_MEDIUM_JPEG.len();

        let output = processor.process(input, &progress).unwrap();
        let compressed_size = output.files[0].data.len();

        // With default quality (80) vs the original (quality 90),
        // the compressed version should be smaller.
        assert!(
            compressed_size < original_size,
            "Compressed JPEG ({} bytes) should be smaller than original ({} bytes)",
            compressed_size,
            original_size
        );
    }

    #[test]
    fn test_compress_jpeg_lower_quality_means_smaller_file() {
        // Quality 50 should produce a smaller file than quality 90.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input_q50 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 50);
        let input_q90 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 90);

        let output_q50 = processor.process(input_q50, &noop_progress()).unwrap();
        let output_q90 = processor.process(input_q90, &progress).unwrap();

        assert!(
            output_q50.files[0].data.len() < output_q90.files[0].data.len(),
            "Quality 50 ({} bytes) should be smaller than quality 90 ({} bytes)",
            output_q50.files[0].data.len(),
            output_q90.files[0].data.len()
        );
    }

    #[test]
    fn test_compress_jpeg_quality_clamped_to_range() {
        // Quality 0 should be clamped to 1 (our minimum).
        // This shouldn't crash — it should just use the minimum quality.
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_JPEG, "photo.jpg", 0);

        // Should not panic or error
        let output = processor.process(input, &progress).unwrap();
        assert_eq!(output.files.len(), 1);
    }

    // =========================================================================
    // PNG Compression Tests
    // =========================================================================

    #[test]
    fn test_compress_png_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_PNG, "screenshot.png");

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files.len(), 1);

        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(&out_file.data),
            Some(ImageFormat::Png),
            "Output should be a valid PNG"
        );
        assert!(out_file.filename.contains("-compressed"));
        assert_eq!(out_file.mime_type, "image/png");
    }

    #[test]
    fn test_compress_png_with_medium_image() {
        // The medium PNG (180 KB) should compress noticeably because
        // the original was generated by ImageMagick (not heavily optimized).
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_MEDIUM_PNG, "screenshot.png");
        let original_size = TEST_MEDIUM_PNG.len();

        let output = processor.process(input, &progress).unwrap();
        let compressed_size = output.files[0].data.len();

        // With Best compression + Adaptive filter, we should see some savings.
        // We don't assert "smaller" because PNG is lossless — an already-
        // optimized PNG might not shrink further. But it should at least
        // produce a valid output.
        assert!(compressed_size > 0, "Compressed PNG should not be empty");

        // Log the sizes for manual inspection during development
        eprintln!(
            "PNG compression: {} bytes → {} bytes ({:.1}% {})",
            original_size,
            compressed_size,
            ((compressed_size as f64 / original_size as f64) - 1.0).abs() * 100.0,
            if compressed_size < original_size {
                "smaller"
            } else {
                "larger"
            }
        );
    }

    // =========================================================================
    // WebP Compression Tests
    // =========================================================================

    #[test]
    fn test_compress_webp_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_WEBP, "image.webp");

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files.len(), 1);

        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(&out_file.data),
            Some(ImageFormat::WebP),
            "Output should be a valid WebP"
        );
        assert!(out_file.filename.contains("-compressed"));
        assert_eq!(out_file.mime_type, "image/webp");
    }

    // =========================================================================
    // Metadata Tests
    // =========================================================================

    #[test]
    fn test_output_includes_compression_metadata() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_JPEG, "photo.jpg");

        let output = processor.process(input, &progress).unwrap();

        // Metadata should include originalSize, compressedSize, compressionRatio, format
        assert!(
            output.metadata.contains_key("originalSize"),
            "Metadata should include originalSize"
        );
        assert!(
            output.metadata.contains_key("compressedSize"),
            "Metadata should include compressedSize"
        );
        assert!(
            output.metadata.contains_key("compressionRatio"),
            "Metadata should include compressionRatio"
        );
        assert!(
            output.metadata.contains_key("format"),
            "Metadata should include format"
        );

        // Original size should match what we passed in
        let original = output.metadata["originalSize"].as_u64().unwrap();
        assert_eq!(
            original,
            TEST_JPEG.len() as u64,
            "originalSize should match input size"
        );
    }

    // =========================================================================
    // Output Filename Tests
    // =========================================================================

    #[test]
    fn test_output_filename_with_extension() {
        let name = CompressImages::output_filename("photo.jpg", ImageFormat::Jpeg);
        assert_eq!(name, "photo-compressed.jpg");
    }

    #[test]
    fn test_output_filename_preserves_format_extension() {
        // If input is "image.png", output should use the correct extension.
        let name = CompressImages::output_filename("image.png", ImageFormat::Png);
        assert_eq!(name, "image-compressed.png");
    }

    #[test]
    fn test_output_filename_without_extension() {
        let name = CompressImages::output_filename("noext", ImageFormat::Jpeg);
        assert_eq!(name, "noext-compressed.jpg");
    }

    #[test]
    fn test_output_filename_with_multiple_dots() {
        // "my.vacation.photo.jpg" → "my.vacation.photo-compressed.jpg"
        let name = CompressImages::output_filename("my.vacation.photo.jpg", ImageFormat::Jpeg);
        assert_eq!(name, "my.vacation.photo-compressed.jpg");
    }

    // =========================================================================
    // Quality Parameter Tests
    // =========================================================================

    #[test]
    fn test_get_quality_default() {
        let params = serde_json::Map::new();
        assert_eq!(CompressImages::get_quality(&params), DEFAULT_JPEG_QUALITY);
    }

    #[test]
    fn test_get_quality_from_params() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(60u64)),
        );
        assert_eq!(CompressImages::get_quality(&params), 60);
    }

    #[test]
    fn test_get_quality_clamped_above_max() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(200u64)),
        );
        assert_eq!(CompressImages::get_quality(&params), MAX_JPEG_QUALITY);
    }

    #[test]
    fn test_get_quality_clamped_below_min() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0u64)),
        );
        assert_eq!(CompressImages::get_quality(&params), MIN_JPEG_QUALITY);
    }

    #[test]
    fn test_get_quality_ignores_non_numeric() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::String("high".to_string()),
        );
        assert_eq!(CompressImages::get_quality(&params), DEFAULT_JPEG_QUALITY);
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_unsupported_format_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        // Random bytes that aren't a valid image, with an unsupported extension
        let input = make_input(b"not an image at all", "document.pdf");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Should return an error for unsupported format"
        );

        // RUST CONCEPT: `if let Err(e) = result`
        // Pattern match on the error to inspect it.
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Could not determine image format"),
                "Error should mention format detection: got '{}'",
                msg
            );
        }
    }

    #[test]
    fn test_corrupt_jpeg_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        // Start with valid JPEG magic bytes but garbage data after.
        // The decoder should fail gracefully, not panic.
        let mut corrupt_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
        corrupt_data.extend_from_slice(b"this is not real JPEG data!!!!");

        let input = make_input(&corrupt_data, "corrupt.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Should return an error for corrupt JPEG");
    }

    #[test]
    fn test_corrupt_png_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        // Valid PNG header but garbage data after
        let mut corrupt_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        corrupt_data.extend_from_slice(b"this is not real PNG data!!!!");

        let input = make_input(&corrupt_data, "corrupt.png");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Should return an error for corrupt PNG");
    }

    #[test]
    fn test_empty_data_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        // Empty file — should fail at format detection
        let input = make_input(b"", "empty.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Should return an error for empty data");
    }

    // =========================================================================
    // Edge Case Tests — Truncated, Corrupt, and Zero-Byte Inputs
    // =========================================================================
    //
    // These tests verify that the compression pipeline handles degenerate
    // inputs gracefully. In the browser, users can drop ANY file — partially
    // downloaded images, renamed text files, zero-byte placeholders, etc.
    // We need clean error messages, not panics or hangs.
    //
    // The pipeline has two failure points:
    //   1. Format detection (ImageFormat::detect) — returns None for
    //      unrecognizable data, which becomes UnsupportedFormat error.
    //   2. Image decoding (ImageReader + .decode()) — fails for data
    //      that has valid magic bytes but invalid/truncated image content,
    //      which becomes ProcessingFailed error.

    // --- Zero-Byte File ---

    #[test]
    fn test_zero_byte_file_with_no_extension_returns_unsupported_format() {
        // Zero bytes AND no extension — both detection strategies fail.
        // This is the most degenerate input possible.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"", "noextension");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Zero-byte file should return an error");

        // RUST CONCEPT: `if let Err(e) = result`
        // This pattern destructures the Result — if it's an Err, we
        // get the inner error value as `e` and can inspect its message.
        // It's like: "if result is an error, bind it to `e` and run this block."
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Could not determine image format"),
                "Should fail at format detection, got: '{}'",
                e
            );
        }
    }

    #[test]
    fn test_zero_byte_file_with_jpg_extension_returns_processing_error() {
        // Zero bytes with a .jpg extension — format detection succeeds
        // (via extension fallback), but decoding fails because there's
        // no actual image data to decode.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"", "empty.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Zero-byte JPEG should fail at decoding");

        // The error could be from format detection (empty bytes) OR
        // from the decoder. Either way, it should be a clean error.
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Could not determine")
                    || msg.contains("Failed to read")
                    || msg.contains("Failed to decode"),
                "Should fail with a clear error message, got: '{}'",
                msg
            );
        }
    }

    // --- Single-Byte File ---

    #[test]
    fn test_single_byte_file_returns_error() {
        // One byte of random data. Not enough for any format signature,
        // and the .dat extension isn't recognized either.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x42], "one_byte.dat");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Single-byte file should return an error");
    }

    #[test]
    fn test_single_byte_file_with_jpg_extension_returns_error() {
        // One byte of data with a .jpg extension. Format detection
        // succeeds via extension fallback, but the decoder will fail
        // because one byte isn't a valid JPEG.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x42], "tiny.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Single-byte file with .jpg extension should fail at decode"
        );
    }

    // --- Truncated JPEG (valid header, missing image data) ---

    #[test]
    fn test_truncated_jpeg_4_bytes_header_only() {
        // Exactly the JPEG magic bytes (FF D8 FF E0) and nothing else.
        // Format detection succeeds (it's a valid JPEG header), but
        // the image decoder will fail because there's no actual image
        // data after the SOI marker.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let input = make_input(&data, "truncated.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "JPEG with only 4-byte header should fail at decode"
        );

        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Failed to decode") || msg.contains("Failed to read"),
                "Should get a decode error, got: '{}'",
                msg
            );
        }
    }

    #[test]
    fn test_truncated_jpeg_10_bytes_returns_error() {
        // JPEG header (4 bytes) + 6 bytes of garbage. Still not a valid
        // JPEG — the decoder needs at minimum a Start of Frame (SOF)
        // marker followed by actual image data.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        data.extend_from_slice(&[0x00, 0x10, 0x4A, 0x46, 0x49, 0x46]); // partial APP0 header
        let input = make_input(&data, "truncated10.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "10-byte truncated JPEG should fail at decode"
        );
    }

    // --- Truncated PNG (valid header, missing chunks) ---

    #[test]
    fn test_truncated_png_8_bytes_header_only() {
        // Exactly the PNG 8-byte signature and nothing else. Format
        // detection succeeds (valid PNG magic), but the decoder will
        // fail because PNG requires at least an IHDR chunk after the
        // signature.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let input = make_input(&data, "truncated.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "PNG with only 8-byte header should fail at decode"
        );

        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Failed to decode") || msg.contains("Failed to read"),
                "Should get a decode error, got: '{}'",
                msg
            );
        }
    }

    #[test]
    fn test_truncated_png_20_bytes_returns_error() {
        // PNG signature (8 bytes) + partial IHDR chunk header (12 bytes).
        // The IHDR chunk needs at least 25 bytes (4 length + 4 type + 13 data + 4 CRC),
        // so 20 total bytes leaves the chunk incomplete.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG sig
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D]); // IHDR length (13)
        data.extend_from_slice(b"IHDR"); // chunk type
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]); // partial: width = 1

        let input = make_input(&data, "truncated20.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "20-byte truncated PNG should fail at decode"
        );
    }

    // --- Truncated WebP (valid RIFF container, missing image data) ---

    #[test]
    fn test_truncated_webp_12_bytes_header_only() {
        // Exactly the WebP RIFF container header (12 bytes) and nothing else.
        // Format detection sees "RIFF....WEBP" and reports WebP, but the
        // decoder will fail because there's no VP8/VP8L bitstream after.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = vec![
            b'R', b'I', b'F', b'F', // RIFF marker
            0x04, 0x00, 0x00, 0x00, // file size (4 = just "WEBP" after this)
            b'W', b'E', b'B', b'P', // WEBP marker
        ];
        let input = make_input(&data, "truncated.webp");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "WebP with only 12-byte RIFF header should fail at decode"
        );

        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Failed to decode") || msg.contains("Failed to read"),
                "Should get a decode error, got: '{}'",
                msg
            );
        }
    }

    #[test]
    fn test_corrupt_webp_returns_error() {
        // Valid WebP RIFF header (12 bytes) + garbage data that isn't
        // a valid VP8 or VP8L bitstream.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![
            b'R', b'I', b'F', b'F', 0x20, 0x00, 0x00, 0x00, // file size (32 bytes after this)
            b'W', b'E', b'B', b'P',
        ];
        data.extend_from_slice(b"this is not a VP8 bitstream!!!!");

        let input = make_input(&data, "corrupt.webp");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Corrupt WebP should return an error");
    }

    // --- Corrupt Magic Bytes (look like one format but aren't) ---

    #[test]
    fn test_corrupt_jpeg_valid_header_garbage_body() {
        // JPEG magic (FF D8 FF E0) followed by random bytes that
        // don't form valid JPEG segments. The format detector reports
        // JPEG, but the decoder should fail gracefully.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        // Add 100 bytes of structured-looking but invalid JPEG data
        for i in 0..100u8 {
            data.push(i);
        }

        let input = make_input(&data, "corrupt_body.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "JPEG with valid header but garbage body should fail at decode"
        );
    }

    #[test]
    fn test_corrupt_png_valid_header_garbage_chunks() {
        // PNG signature (8 bytes) followed by bytes that don't form
        // valid PNG chunks. The format detector reports PNG, but the
        // chunk parser in the decoder should fail.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        // Invalid chunk: wrong length, wrong type, wrong CRC
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]); // absurd chunk length
        data.extend_from_slice(b"FAKE"); // fake chunk type
        data.extend_from_slice(&[0x00; 20]); // some zero bytes

        let input = make_input(&data, "corrupt_chunks.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "PNG with valid header but garbage chunks should fail at decode"
        );
    }

    // --- Extension-Only Detection with Undecodable Data ---

    #[test]
    fn test_random_bytes_with_jpg_extension_returns_error() {
        // Random bytes (no valid magic) with a .jpg extension. Format
        // detection falls back to the extension and reports JPEG, but
        // the decoder should fail because the data isn't a JPEG.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = b"This is just a plain text file, not a JPEG image at all";
        let input = make_input(data, "not_really.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Random data with .jpg extension should fail at decode"
        );

        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Failed to decode") || msg.contains("Failed to read"),
                "Should fail at image decoding, got: '{}'",
                msg
            );
        }
    }

    #[test]
    fn test_random_bytes_with_png_extension_returns_error() {
        // Same as above but with .png extension.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = b"Not a PNG, just text pretending to be one.";
        let input = make_input(data, "fake.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Random data with .png extension should fail at decode"
        );
    }

    #[test]
    fn test_random_bytes_with_webp_extension_returns_error() {
        // Same as above but with .webp extension.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = b"Not a WebP file, just a pretender.";
        let input = make_input(data, "fake.webp");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Random data with .webp extension should fail at decode"
        );
    }

    // --- Mixed Corruption Scenarios ---

    #[test]
    fn test_5_byte_file_no_known_extension_returns_error() {
        // Five bytes of random data with an unknown extension.
        // Too short for any magic bytes (except JPEG at 4), but the
        // data doesn't start with FF D8 FF. Both detection methods fail.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x01, 0x02, 0x03, 0x04, 0x05], "mystery.dat");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "5-byte unknown file should return UnsupportedFormat error"
        );

        if let Err(e) = result {
            assert!(
                e.to_string().contains("Could not determine image format"),
                "Should fail at format detection, got: '{}'",
                e
            );
        }
    }

    #[test]
    fn test_all_zeros_file_returns_error() {
        // 50 bytes of all zeros. This has enough length for any magic byte
        // check, but none of the signatures match (JPEG=FF D8 FF, PNG=89 50...).
        // Extension ".bin" is also unknown.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x00; 50], "zeros.bin");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "All-zeros file should return an error");

        if let Err(e) = result {
            assert!(
                e.to_string().contains("Could not determine image format"),
                "Should fail at format detection, got: '{}'",
                e
            );
        }
    }

    #[test]
    fn test_all_ff_bytes_with_jpg_extension_returns_error() {
        // 20 bytes of 0xFF. The first 3 bytes are FF FF FF which is NOT
        // a valid JPEG signature (JPEG needs FF D8 FF, not FF FF FF).
        // But the .jpg extension triggers extension-based detection.
        // The decoder should then fail because the data isn't valid JPEG.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0xFF; 20], "allones.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "All-0xFF bytes with .jpg extension should fail at decode"
        );
    }

    // =========================================================================
    // Quality Parameter Boundary Tests
    // =========================================================================
    //
    // The quality parameter controls JPEG compression aggressiveness (1-100).
    // These tests hit every boundary:
    //   - 0: below minimum → clamped to 1 by get_quality(), rejected by validate()
    //   - 1: exact minimum → valid, should produce the most compressed output
    //   - 100: exact maximum → valid, should produce the least compressed output
    //   - 101: above maximum → clamped to 100 by get_quality(), rejected by validate()
    //
    // Two paths test quality handling:
    //   1. `get_quality()` — silent clamping (used during processing)
    //   2. `validate()` — explicit error messages (used before processing)
    //
    // We test both paths because they serve different purposes:
    //   - get_quality() is defensive: "make it work no matter what"
    //   - validate() is informative: "tell the user what's wrong"

    // --- get_quality() boundary tests ---

    #[test]
    fn test_get_quality_at_exact_min_boundary() {
        // Quality = 1 (MIN_JPEG_QUALITY) should pass through unchanged.
        // This is the exact boundary — no clamping needed.
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1u64)),
        );
        assert_eq!(
            CompressImages::get_quality(&params),
            1,
            "Quality 1 should pass through as-is (exact minimum boundary)"
        );
    }

    #[test]
    fn test_get_quality_at_exact_max_boundary() {
        // Quality = 100 (MAX_JPEG_QUALITY) should pass through unchanged.
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100u64)),
        );
        assert_eq!(
            CompressImages::get_quality(&params),
            100,
            "Quality 100 should pass through as-is (exact maximum boundary)"
        );
    }

    #[test]
    fn test_get_quality_101_clamped_to_max() {
        // Quality = 101 is just above the max. Should clamp to 100.
        //
        // RUST CONCEPT: `.clamp(min, max)`
        // Returns max if the value exceeds max. So 101.clamp(1, 100) = 100.
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(101u64)),
        );
        assert_eq!(
            CompressImages::get_quality(&params),
            MAX_JPEG_QUALITY,
            "Quality 101 should be clamped to 100 (just above max)"
        );
    }

    // --- validate() boundary tests ---

    #[test]
    fn test_validate_quality_0_fails() {
        // Quality = 0 is below our minimum (1). The validate() method
        // should report this as an out-of-range error so the UI can show
        // the user a helpful message before processing starts.
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(
            errors.len(),
            1,
            "Quality 0 should produce exactly one validation error"
        );
        assert!(
            errors[0].contains("between"),
            "Error should mention the valid range: got '{}'",
            errors[0]
        );
    }

    #[test]
    fn test_validate_quality_1_passes() {
        // Quality = 1 is the exact minimum boundary. Should pass validation.
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1u64)),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Quality 1 should pass validation (exact minimum): got {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_quality_100_passes() {
        // Quality = 100 is the exact maximum boundary. Should pass validation.
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100u64)),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Quality 100 should pass validation (exact maximum): got {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_quality_101_fails() {
        // Quality = 101 is just above the max. Should fail validation.
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(101u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(
            errors.len(),
            1,
            "Quality 101 should produce exactly one validation error"
        );
        assert!(
            errors[0].contains("between"),
            "Error should mention the valid range: got '{}'",
            errors[0]
        );
    }

    // --- Actual compression at quality boundaries ---

    #[test]
    fn test_compress_jpeg_at_quality_1_produces_valid_output() {
        // Quality 1 = maximum compression, worst visual quality.
        // Should still produce a valid, decodable JPEG — just very small.
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);

        let output = processor
            .process(input, &progress)
            .expect("Quality 1 should produce valid output, not an error");

        // Output should be a valid JPEG (check magic bytes)
        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Quality 1 output should still be a valid JPEG"
        );

        // Quality 1 should be noticeably smaller than the original
        assert!(
            output.files[0].data.len() < TEST_MEDIUM_JPEG.len(),
            "Quality 1 ({} bytes) should be smaller than original ({} bytes)",
            output.files[0].data.len(),
            TEST_MEDIUM_JPEG.len()
        );
    }

    #[test]
    fn test_compress_jpeg_at_quality_100_produces_valid_output() {
        // Quality 100 = minimal compression, best visual quality.
        // Should produce a valid JPEG, possibly larger than the original
        // (re-encoding at max quality can inflate files).
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let output = processor
            .process(input, &progress)
            .expect("Quality 100 should produce valid output, not an error");

        // Output should be a valid JPEG
        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Quality 100 output should be a valid JPEG"
        );

        // Output should not be empty
        assert!(
            !output.files[0].data.is_empty(),
            "Quality 100 output should not be empty"
        );
    }

    #[test]
    fn test_quality_1_smaller_than_quality_100() {
        // The core contract of lossy compression: lower quality = smaller file.
        // Quality 1 (most aggressive) should always produce a smaller file
        // than quality 100 (least aggressive) for the same input image.
        //
        // This is a stronger test than just "it doesn't crash" — it verifies
        // the quality parameter actually controls the output size.
        let processor = CompressImages::new();

        let input_q1 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);
        let input_q100 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let output_q1 = processor.process(input_q1, &noop_progress()).unwrap();
        let output_q100 = processor.process(input_q100, &noop_progress()).unwrap();

        let size_q1 = output_q1.files[0].data.len();
        let size_q100 = output_q100.files[0].data.len();

        assert!(
            size_q1 < size_q100,
            "Quality 1 ({} bytes) should be smaller than quality 100 ({} bytes)",
            size_q1,
            size_q100
        );
    }

    #[test]
    fn test_quality_monotonically_affects_file_size() {
        // Verify the full ordering: q1 <= q50 <= q100.
        // JPEG quality should be monotonically non-decreasing with file size.
        //
        // WHY THIS MATTERS: If quality 50 produces a LARGER file than
        // quality 100, something is deeply wrong with our quality pipeline
        // (e.g., the param isn't being passed through, or it's being
        // silently replaced with a default).
        let processor = CompressImages::new();

        let input_q1 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);
        let input_q50 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 50);
        let input_q100 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let size_q1 = processor.process(input_q1, &noop_progress()).unwrap().files[0]
            .data
            .len();
        let size_q50 = processor
            .process(input_q50, &noop_progress())
            .unwrap()
            .files[0]
            .data
            .len();
        let size_q100 = processor
            .process(input_q100, &noop_progress())
            .unwrap()
            .files[0]
            .data
            .len();

        assert!(
            size_q1 <= size_q50,
            "q1 ({} bytes) should be <= q50 ({} bytes)",
            size_q1,
            size_q50
        );
        assert!(
            size_q50 <= size_q100,
            "q50 ({} bytes) should be <= q100 ({} bytes)",
            size_q50,
            size_q100
        );

        // Log sizes for manual inspection during development
        eprintln!(
            "Quality ordering: q1={} bytes, q50={} bytes, q100={} bytes",
            size_q1, size_q50, size_q100
        );
    }

    // =========================================================================
    // 1x1 Pixel Image Tests — Minimum Viable Image
    // =========================================================================
    //
    // A 1x1 pixel image is the smallest possible image. It exercises the
    // compression pipeline with minimal data, testing that:
    //   1. The decoder handles tiny images without crashing
    //   2. The encoder produces valid output (not empty, has correct magic bytes)
    //   3. Metadata is correct (original/compressed sizes, format)
    //
    // WHY THIS MATTERS: Image codecs sometimes have edge cases with very small
    // images — JPEG's 8x8 block DCT needs padding for images smaller than 8x8,
    // PNG row filters need at least one pixel, etc. If our pipeline handles 1x1,
    // it handles anything larger.
    //
    // We create these tiny images programmatically using the `image` crate
    // rather than loading fixtures, because 1x1 encoded images are so small
    // that generating them in-test is cleaner than maintaining fixture files.

    /// Helper: create a 1x1 pixel JPEG image (a single red pixel).
    ///
    /// RUST CONCEPT: `image::RgbImage`
    /// This is a 2D array of RGB pixels. `new(width, height)` creates a
    /// black image. `.put_pixel(x, y, pixel)` sets one pixel's color.
    /// We then encode it to JPEG bytes in memory.
    fn create_1x1_jpeg() -> Vec<u8> {
        use image::{DynamicImage, Rgb, RgbImage};

        // Create a 1x1 image with a red pixel (R=255, G=0, B=0)
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([255, 0, 0]));

        // Encode to JPEG bytes in memory
        let dynamic = DynamicImage::ImageRgb8(img);
        let mut buf = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut buf, 80);
        dynamic
            .write_with_encoder(encoder)
            .expect("Failed to encode 1x1 JPEG");
        buf
    }

    /// Helper: create a 1x1 pixel PNG image (a single green pixel).
    fn create_1x1_png() -> Vec<u8> {
        use image::{DynamicImage, Rgb, RgbImage};

        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([0, 255, 0]));

        let dynamic = DynamicImage::ImageRgb8(img);
        let mut buf = Vec::new();
        let encoder =
            PngEncoder::new_with_quality(&mut buf, CompressionType::Default, FilterType::NoFilter);
        dynamic
            .write_with_encoder(encoder)
            .expect("Failed to encode 1x1 PNG");
        buf
    }

    /// Helper: create a 1x1 pixel WebP image (a single blue pixel).
    fn create_1x1_webp() -> Vec<u8> {
        use image::{DynamicImage, Rgb, RgbImage};

        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([0, 0, 255]));

        // Encode to WebP bytes using the `image` crate's built-in encoder
        let dynamic = DynamicImage::ImageRgb8(img);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        dynamic
            .write_to(&mut cursor, image::ImageFormat::WebP)
            .expect("Failed to encode 1x1 WebP");
        buf
    }

    #[test]
    fn test_1x1_jpeg_compresses_successfully() {
        // A 1x1 pixel JPEG is the smallest valid JPEG image.
        // The JPEG codec internally works on 8x8 blocks — a 1x1 image
        // gets padded to fill one block. This tests that the padding
        // doesn't cause any issues in our decode → re-encode pipeline.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        assert!(
            !jpeg_bytes.is_empty(),
            "Helper should produce non-empty JPEG bytes"
        );

        let input = make_input(&jpeg_bytes, "tiny.jpg");
        let output = processor
            .process(input, &progress)
            .expect("1x1 JPEG should compress without error");

        // Verify we got a valid JPEG back
        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Compressed 1x1 should still be a valid JPEG"
        );
        assert_eq!(output.files[0].mime_type, "image/jpeg");
        assert!(
            output.files[0].filename.contains("-compressed"),
            "Output filename should contain '-compressed'"
        );

        // Metadata should have valid sizes
        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_png_compresses_successfully() {
        // A 1x1 pixel PNG is the smallest valid PNG image.
        // PNG row filters operate per-row — with just one pixel per row,
        // the filter has minimal data to work with. This tests that our
        // Adaptive filter strategy handles the edge case.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let png_bytes = create_1x1_png();
        assert!(
            !png_bytes.is_empty(),
            "Helper should produce non-empty PNG bytes"
        );

        let input = make_input(&png_bytes, "tiny.png");
        let output = processor
            .process(input, &progress)
            .expect("1x1 PNG should compress without error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Png),
            "Compressed 1x1 should still be a valid PNG"
        );
        assert_eq!(output.files[0].mime_type, "image/png");

        // Metadata should have valid sizes
        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_webp_compresses_successfully() {
        // A 1x1 pixel WebP is the smallest valid WebP image.
        // The WebP lossless encoder uses predictive coding (predicting
        // each pixel from its neighbors). With just one pixel, there
        // are no neighbors to predict from — this is the degenerate case.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let webp_bytes = create_1x1_webp();
        assert!(
            !webp_bytes.is_empty(),
            "Helper should produce non-empty WebP bytes"
        );

        let input = make_input(&webp_bytes, "tiny.webp");
        let output = processor
            .process(input, &progress)
            .expect("1x1 WebP should compress without error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::WebP),
            "Compressed 1x1 should still be a valid WebP"
        );
        assert_eq!(output.files[0].mime_type, "image/webp");

        // Metadata should have valid sizes
        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_jpeg_with_quality_1_produces_valid_output() {
        // The extreme combo: smallest possible image + lowest quality.
        // If this doesn't crash, our pipeline handles the most degenerate
        // valid JPEG case.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_quality(&jpeg_bytes, "tiny_q1.jpg", 1);

        let output = processor
            .process(input, &progress)
            .expect("1x1 JPEG at quality 1 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / quality 1"
        );
    }

    #[test]
    fn test_1x1_jpeg_with_quality_100_produces_valid_output() {
        // Smallest possible image + highest quality.
        // For a 1x1 image, quality 100 vs quality 1 may not differ much
        // in file size (very little data to compress), but both should work.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_quality(&jpeg_bytes, "tiny_q100.jpg", 100);

        let output = processor
            .process(input, &progress)
            .expect("1x1 JPEG at quality 100 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / quality 100"
        );
    }

    // =========================================================================
    // EXIF Orientation Tests — Full Pipeline
    // =========================================================================
    //
    // These tests verify that the compress pipeline correctly applies EXIF
    // orientation. A smartphone portrait photo (60×40 stored pixels with
    // EXIF orientation=6) should produce a 40×60 compressed output.

    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    #[test]
    fn test_compress_jpeg_applies_exif_orientation_rotate90() {
        // Create a 60×40 JPEG with EXIF orientation=6 (rotate 90° CW).
        // After compression, the output should be 40×60 (dimensions swapped).
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "portrait.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        // Decode the output (plain decode — no EXIF in the output since
        // the image crate strips metadata during encoding).
        let output_img = decode_with_orientation(output_data).unwrap();

        assert_eq!(
            output_img.width(),
            40,
            "Compressed portrait should be 40 wide"
        );
        assert_eq!(
            output_img.height(),
            60,
            "Compressed portrait should be 60 tall"
        );
    }

    #[test]
    fn test_compress_jpeg_applies_exif_orientation_rotate180() {
        // Create a 60×40 JPEG with EXIF orientation=3 (rotate 180°).
        // After compression, dimensions stay 60×40 (180° doesn't swap).
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "flipped.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 60);
        assert_eq!(output_img.height(), 40);
    }

    #[test]
    fn test_compress_jpeg_no_exif_preserves_dimensions() {
        // A normal JPEG (no EXIF orientation) should pass through
        // with dimensions unchanged — backward compatibility check.
        let jpeg = create_test_jpeg(60, 40);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&jpeg, "landscape.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 60);
        assert_eq!(output_img.height(), 40);
    }
}
