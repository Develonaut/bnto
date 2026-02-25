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

use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

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

        let cursor = Cursor::new(data);
        let img = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read image: {e}")))?
            .decode()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode JPEG: {e}")))?;

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

        let cursor = Cursor::new(data);
        let img = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read image: {e}")))?
            .decode()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode PNG: {e}")))?;

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

        let cursor = Cursor::new(data);
        let img = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read image: {e}")))?
            .decode()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode WebP: {e}")))?;

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
}
