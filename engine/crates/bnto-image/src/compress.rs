// Compress Images Node — reduce image file size via lossy/lossless re-encoding.
//
// Compression strategies by format:
//   JPEG: re-encode at lower quality (lossy, DCT-based)
//   PNG: color quantization to 8-bit indexed palette (lossy, via quantize.rs)
//   WebP: lossless re-encoding only (lossy requires libwebp, planned via jSquash)

use std::io::Cursor;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use image::codecs::jpeg::JpegEncoder;
#[cfg(test)]
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::common::image_accepts;
use crate::format::ImageFormat;
use crate::orientation::decode_with_orientation;

// --- Configuration Constants ---

use bnto_core::DEFAULT_COMPRESSION;

/// Minimum compression level. 1 = barely any compression at all.
const MIN_COMPRESSION: u8 = 1;

/// Maximum compression level. 100 = most aggressive compression.
const MAX_COMPRESSION: u8 = 100;

// --- The CompressImages Processor ---

/// The compress-images node processor.
pub struct CompressImages;

impl Default for CompressImages {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressImages {
    pub fn new() -> Self {
        Self
    }

    /// Compress a JPEG image by re-encoding at a specific quality level.
    fn compress_jpeg(
        &self,
        data: &[u8],
        quality: u8,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(10, "Decoding JPEG...");
        let img = decode_with_orientation(data)?;

        progress.report(50, "Compressing JPEG...");
        let mut output = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut output, quality);
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;

        progress.report(100, "JPEG compression complete");
        Ok(output)
    }

    /// Compress a PNG image using lossy color quantization.
    ///
    /// Reduces 24-bit truecolor to an 8-bit indexed palette (256 colors max)
    /// using quantizr (median cut + Floyd-Steinberg dithering).
    fn compress_png(&self, data: &[u8], progress: &ProgressReporter) -> Result<Vec<u8>, BntoError> {
        crate::quantize::compress_png_quantized(data, progress)
    }

    /// Re-encode a WebP image with lossless compression.
    ///
    /// NOTE: The `image` crate's WebP encoder only supports LOSSLESS mode.
    /// Lossy WebP will be added later via jSquash JS fallback.
    fn compress_webp(
        &self,
        data: &[u8],
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(10, "Decoding WebP...");
        let img = decode_with_orientation(data)?;

        progress.report(50, "Compressing WebP (lossless)...");
        let mut output = Vec::new();
        let mut cursor_out = Cursor::new(&mut output);
        img.write_to(&mut cursor_out, image::ImageFormat::WebP)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;

        progress.report(100, "WebP compression complete");
        Ok(output)
    }

    /// Extract the compression level from params and clamp to valid range.
    ///
    /// User-facing semantics are INVERTED from JPEG quality:
    ///   compression=1 -> quality~100, compression=100 -> quality=1
    fn get_compression(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("compression")
            .and_then(|v| v.as_u64())
            .map(|c| c as u8)
            .unwrap_or(DEFAULT_COMPRESSION)
            .clamp(MIN_COMPRESSION, MAX_COMPRESSION)
    }

    /// Convert a compression level (1-100) to JPEG quality (100-1).
    /// Formula: `jpeg_quality = 101 - compression` (clamped to 1..100)
    fn compression_to_quality(compression: u8) -> u8 {
        101u8.saturating_sub(compression).max(1)
    }

    /// Generate output filename: "photo.jpg" -> "photo-compressed.jpg"
    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        if let Some(dot_pos) = input_filename.rfind('.') {
            let stem = &input_filename[..dot_pos];
            format!("{stem}-compressed.{}", format.extension())
        } else {
            format!("{input_filename}-compressed.{}", format.extension())
        }
    }
}

// --- NodeProcessor Trait Implementation ---

impl NodeProcessor for CompressImages {
    fn name(&self) -> &str {
        "compress-images"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image".to_string(),
            operation: "compress".to_string(),
            name: "Compress Images".to_string(),
            description: "Reduce image file size while maintaining quality".to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: vec![compression_param_def()],
        }
    }

    /// Process a single image: detect format, decode, re-encode with compression.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        let format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        let original_size = input.data.len();
        let compressed_data = self.compress_by_format(&input, format, progress)?;
        let compressed_size = compressed_data.len();
        let output_filename = Self::output_filename(&input.filename, format);
        let metadata = build_compression_metadata(original_size, compressed_size, format);

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
    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(compression_val) = params.get("compression") {
            match compression_val.as_u64() {
                Some(c) if c >= MIN_COMPRESSION as u64 && c <= MAX_COMPRESSION as u64 => {}
                Some(c) => {
                    errors.push(format!(
                        "Compression must be between {MIN_COMPRESSION} and {MAX_COMPRESSION}, got {c}"
                    ));
                }
                None => {
                    errors.push(format!(
                        "Compression must be a number, got: {compression_val}"
                    ));
                }
            }
        }

        errors
    }
}

// --- Private helpers ---

impl CompressImages {
    /// Dispatch compression to the format-specific method.
    fn compress_by_format(
        &self,
        input: &NodeInput,
        format: ImageFormat,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        match format {
            ImageFormat::Jpeg => {
                self.compress_jpeg_with_params(&input.data, &input.params, progress)
            }
            ImageFormat::Png => {
                progress.report(5, "Optimizing PNG...");
                self.compress_png(&input.data, progress)
            }
            ImageFormat::WebP => {
                progress.report(5, "Compressing WebP (lossless)...");
                self.compress_webp(&input.data, progress)
            }
        }
    }

    /// Extract compression param, convert to JPEG quality, and compress.
    fn compress_jpeg_with_params(
        &self,
        data: &[u8],
        params: &serde_json::Map<String, serde_json::Value>,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        let compression = Self::get_compression(params);
        let quality = Self::compression_to_quality(compression);
        progress.report(
            5,
            &format!("Compressing JPEG (compression: {compression}, quality: {quality})..."),
        );
        self.compress_jpeg(data, quality, progress)
    }
}

/// Build the metadata map with compression stats for the UI results panel.
fn build_compression_metadata(
    original_size: usize,
    compressed_size: usize,
    format: ImageFormat,
) -> serde_json::Map<String, serde_json::Value> {
    let mut metadata = serde_json::Map::new();
    insert_size_metadata(&mut metadata, original_size, compressed_size);
    insert_compression_ratio(&mut metadata, original_size, compressed_size);
    metadata.insert(
        "format".to_string(),
        serde_json::Value::String(format!("{:?}", format)),
    );
    metadata
}

fn insert_size_metadata(
    metadata: &mut serde_json::Map<String, serde_json::Value>,
    original_size: usize,
    compressed_size: usize,
) {
    metadata.insert(
        "originalSize".to_string(),
        serde_json::Value::Number(serde_json::Number::from(original_size as u64)),
    );
    metadata.insert(
        "compressedSize".to_string(),
        serde_json::Value::Number(serde_json::Number::from(compressed_size as u64)),
    );
}

fn insert_compression_ratio(
    metadata: &mut serde_json::Map<String, serde_json::Value>,
    original_size: usize,
    compressed_size: usize,
) {
    let ratio = if original_size > 0 {
        (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
    } else {
        0.0
    };
    if let Some(ratio_num) = serde_json::Number::from_f64(ratio) {
        metadata.insert(
            "compressionRatio".to_string(),
            serde_json::Value::Number(ratio_num),
        );
    }
}

/// Compression parameter definition (1-100, default 20).
fn compression_param_def() -> bnto_core::metadata::ParameterDef {
    use bnto_core::metadata::*;
    ParameterDef {
        name: "compression".to_string(),
        label: "Compression".to_string(),
        description: "How much to compress (1 = minimal, 100 = maximum)".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::json!(20)),
        constraints: Some(Constraints {
            min: Some(1.0),
            max: Some(100.0),
            required: false,
        }),
        visible_when: Some(ParamCondition::Single(ParamConditionEntry {
            param: "operation".to_string(),
            equals: "compress".to_string(),
        })),
        ..Default::default()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    // =========================================================================
    // Test Fixtures — Real images from our shared test-fixtures directory
    // =========================================================================

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

    /// Create a test input with compression parameter set.
    fn make_input_with_compression(data: &[u8], filename: &str, compression: u64) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(compression)),
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
    fn test_validate_passes_with_valid_compression() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(50u64)),
        );
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_fails_with_compression_out_of_range() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(150u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("between"));
    }

    #[test]
    fn test_validate_fails_with_non_numeric_compression() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
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
    fn test_compress_jpeg_higher_compression_means_smaller_file() {
        // Compression 80 should produce a smaller file than compression 20.
        // (compression 80 → quality 21, compression 20 → quality 81)
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input_c80 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 80);
        let input_c20 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 20);

        let output_c80 = processor.process(input_c80, &noop_progress()).unwrap();
        let output_c20 = processor.process(input_c20, &progress).unwrap();

        assert!(
            output_c80.files[0].data.len() < output_c20.files[0].data.len(),
            "Compression 80 ({} bytes) should be smaller than compression 20 ({} bytes)",
            output_c80.files[0].data.len(),
            output_c20.files[0].data.len()
        );
    }

    #[test]
    fn test_compress_jpeg_compression_clamped_to_range() {
        // Compression 0 should be clamped to 1 (our minimum).
        // This shouldn't crash — it should just use the minimum compression.
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_compression(TEST_JPEG, "photo.jpg", 0);

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

        // With quantizr (lossy color quantization), we expect significant
        // reduction on photographic PNGs — typically 50%+ smaller.
        assert!(
            compressed_size < original_size,
            "Quantized PNG should be smaller than original"
        );

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
        let name = CompressImages::output_filename("my.vacation.photo.jpg", ImageFormat::Jpeg);
        assert_eq!(name, "my.vacation.photo-compressed.jpg");
    }

    // =========================================================================
    // Compression Parameter Tests
    // =========================================================================

    #[test]
    fn test_get_compression_default() {
        let params = serde_json::Map::new();
        assert_eq!(
            CompressImages::get_compression(&params),
            DEFAULT_COMPRESSION
        );
    }

    #[test]
    fn test_get_compression_from_params() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(60u64)),
        );
        assert_eq!(CompressImages::get_compression(&params), 60);
    }

    #[test]
    fn test_get_compression_clamped_above_max() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(200u64)),
        );
        assert_eq!(CompressImages::get_compression(&params), MAX_COMPRESSION);
    }

    #[test]
    fn test_get_compression_clamped_below_min() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0u64)),
        );
        assert_eq!(CompressImages::get_compression(&params), MIN_COMPRESSION);
    }

    #[test]
    fn test_get_compression_ignores_non_numeric() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::String("high".to_string()),
        );
        assert_eq!(
            CompressImages::get_compression(&params),
            DEFAULT_COMPRESSION
        );
    }

    // =========================================================================
    // Compression → Quality Inversion Tests
    // =========================================================================

    #[test]
    fn test_compression_to_quality_inversion() {
        assert_eq!(CompressImages::compression_to_quality(1), 100);
        assert_eq!(CompressImages::compression_to_quality(20), 81);
        assert_eq!(CompressImages::compression_to_quality(50), 51);
        assert_eq!(CompressImages::compression_to_quality(80), 21);
        assert_eq!(CompressImages::compression_to_quality(100), 1);
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_unsupported_format_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"not an image at all", "document.pdf");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Should return an error for unsupported format"
        );

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

        let mut corrupt_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        corrupt_data.extend_from_slice(b"this is not real JPEG data!!!!");

        let input = make_input(&corrupt_data, "corrupt.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Should return an error for corrupt JPEG");
    }

    #[test]
    fn test_corrupt_png_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

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

        let input = make_input(b"", "empty.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Should return an error for empty data");
    }

    // =========================================================================
    // Edge Case Tests — Truncated, Corrupt, and Zero-Byte Inputs
    // =========================================================================

    #[test]
    fn test_zero_byte_file_with_no_extension_returns_unsupported_format() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"", "noextension");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Zero-byte file should return an error");

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
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"", "empty.jpg");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Zero-byte JPEG should fail at decoding");

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

    #[test]
    fn test_single_byte_file_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x42], "one_byte.dat");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Single-byte file should return an error");
    }

    #[test]
    fn test_single_byte_file_with_jpg_extension_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x42], "tiny.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "Single-byte file with .jpg extension should fail at decode"
        );
    }

    #[test]
    fn test_truncated_jpeg_4_bytes_header_only() {
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
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        data.extend_from_slice(&[0x00, 0x10, 0x4A, 0x46, 0x49, 0x46]);
        let input = make_input(&data, "truncated10.jpg");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "10-byte truncated JPEG should fail at decode"
        );
    }

    #[test]
    fn test_truncated_png_8_bytes_header_only() {
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
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D]);
        data.extend_from_slice(b"IHDR");
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);

        let input = make_input(&data, "truncated20.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "20-byte truncated PNG should fail at decode"
        );
    }

    #[test]
    fn test_truncated_webp_12_bytes_header_only() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let data = vec![
            b'R', b'I', b'F', b'F', 0x04, 0x00, 0x00, 0x00, b'W', b'E', b'B', b'P',
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
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![
            b'R', b'I', b'F', b'F', 0x20, 0x00, 0x00, 0x00, b'W', b'E', b'B', b'P',
        ];
        data.extend_from_slice(b"this is not a VP8 bitstream!!!!");

        let input = make_input(&data, "corrupt.webp");
        let result = processor.process(input, &progress);

        assert!(result.is_err(), "Corrupt WebP should return an error");
    }

    #[test]
    fn test_corrupt_jpeg_valid_header_garbage_body() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0];
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
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
        data.extend_from_slice(b"FAKE");
        data.extend_from_slice(&[0x00; 20]);

        let input = make_input(&data, "corrupt_chunks.png");
        let result = processor.process(input, &progress);

        assert!(
            result.is_err(),
            "PNG with valid header but garbage chunks should fail at decode"
        );
    }

    #[test]
    fn test_random_bytes_with_jpg_extension_returns_error() {
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

    #[test]
    fn test_5_byte_file_no_known_extension_returns_error() {
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
    // Compression Parameter Boundary Tests
    // =========================================================================

    #[test]
    fn test_get_compression_at_exact_min_boundary() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1u64)),
        );
        assert_eq!(
            CompressImages::get_compression(&params),
            1,
            "Compression 1 should pass through as-is (exact minimum boundary)"
        );
    }

    #[test]
    fn test_get_compression_at_exact_max_boundary() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100u64)),
        );
        assert_eq!(
            CompressImages::get_compression(&params),
            100,
            "Compression 100 should pass through as-is (exact maximum boundary)"
        );
    }

    #[test]
    fn test_get_compression_101_clamped_to_max() {
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(101u64)),
        );
        assert_eq!(
            CompressImages::get_compression(&params),
            MAX_COMPRESSION,
            "Compression 101 should be clamped to 100 (just above max)"
        );
    }

    #[test]
    fn test_validate_compression_0_fails() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(
            errors.len(),
            1,
            "Compression 0 should produce exactly one validation error"
        );
        assert!(
            errors[0].contains("between"),
            "Error should mention the valid range: got '{}'",
            errors[0]
        );
    }

    #[test]
    fn test_validate_compression_1_passes() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1u64)),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Compression 1 should pass validation (exact minimum): got {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_compression_100_passes() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100u64)),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Compression 100 should pass validation (exact maximum): got {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_compression_101_fails() {
        let processor = CompressImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "compression".to_string(),
            serde_json::Value::Number(serde_json::Number::from(101u64)),
        );
        let errors = processor.validate(&params);
        assert_eq!(
            errors.len(),
            1,
            "Compression 101 should produce exactly one validation error"
        );
        assert!(
            errors[0].contains("between"),
            "Error should mention the valid range: got '{}'",
            errors[0]
        );
    }

    #[test]
    fn test_compress_jpeg_at_compression_100_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let output = processor
            .process(input, &progress)
            .expect("Compression 100 should produce valid output, not an error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Compression 100 output should still be a valid JPEG"
        );

        assert!(
            output.files[0].data.len() < TEST_MEDIUM_JPEG.len(),
            "Compression 100 ({} bytes) should be smaller than original ({} bytes)",
            output.files[0].data.len(),
            TEST_MEDIUM_JPEG.len()
        );
    }

    #[test]
    fn test_compress_jpeg_at_compression_1_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 1);

        let output = processor
            .process(input, &progress)
            .expect("Compression 1 should produce valid output, not an error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Compression 1 output should be a valid JPEG"
        );

        assert!(
            !output.files[0].data.is_empty(),
            "Compression 1 output should not be empty"
        );
    }

    #[test]
    fn test_compression_100_smaller_than_compression_1() {
        let processor = CompressImages::new();

        let input_c100 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 100);
        let input_c1 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 1);

        let output_c100 = processor.process(input_c100, &noop_progress()).unwrap();
        let output_c1 = processor.process(input_c1, &noop_progress()).unwrap();

        let size_c100 = output_c100.files[0].data.len();
        let size_c1 = output_c1.files[0].data.len();

        assert!(
            size_c100 < size_c1,
            "Compression 100 ({} bytes) should be smaller than compression 1 ({} bytes)",
            size_c100,
            size_c1
        );
    }

    #[test]
    fn test_compression_monotonically_affects_file_size() {
        let processor = CompressImages::new();

        let input_c100 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 100);
        let input_c50 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 50);
        let input_c1 = make_input_with_compression(TEST_MEDIUM_JPEG, "photo.jpg", 1);

        let size_c100 = processor
            .process(input_c100, &noop_progress())
            .unwrap()
            .files[0]
            .data
            .len();
        let size_c50 = processor
            .process(input_c50, &noop_progress())
            .unwrap()
            .files[0]
            .data
            .len();
        let size_c1 = processor.process(input_c1, &noop_progress()).unwrap().files[0]
            .data
            .len();

        assert!(
            size_c100 <= size_c50,
            "c100 ({} bytes) should be <= c50 ({} bytes)",
            size_c100,
            size_c50
        );
        assert!(
            size_c50 <= size_c1,
            "c50 ({} bytes) should be <= c1 ({} bytes)",
            size_c50,
            size_c1
        );

        eprintln!(
            "Compression ordering: c100={} bytes, c50={} bytes, c1={} bytes",
            size_c100, size_c50, size_c1
        );
    }

    // =========================================================================
    // 1x1 Pixel Image Tests — Minimum Viable Image
    // =========================================================================

    /// Helper: create a 1x1 pixel JPEG image (a single red pixel).
    fn create_1x1_jpeg() -> Vec<u8> {
        use image::{DynamicImage, Rgb, RgbImage};

        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([255, 0, 0]));

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

        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_png_compresses_successfully() {
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

        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_webp_compresses_successfully() {
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

        let original_size = output.metadata["originalSize"].as_u64().unwrap();
        let compressed_size = output.metadata["compressedSize"].as_u64().unwrap();
        assert!(original_size > 0, "Original size should be > 0");
        assert!(compressed_size > 0, "Compressed size should be > 0");
    }

    #[test]
    fn test_1x1_jpeg_with_compression_100_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_compression(&jpeg_bytes, "tiny_c100.jpg", 100);

        let output = processor
            .process(input, &progress)
            .expect("1x1 JPEG at compression 100 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / compression 100"
        );
    }

    #[test]
    fn test_1x1_jpeg_with_compression_1_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_compression(&jpeg_bytes, "tiny_c1.jpg", 1);

        let output = processor
            .process(input, &progress)
            .expect("1x1 JPEG at compression 1 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / compression 1"
        );
    }

    // =========================================================================
    // EXIF Orientation Tests — Full Pipeline
    // =========================================================================

    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    #[test]
    fn test_compress_jpeg_applies_exif_orientation_rotate90() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "portrait.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

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

    // =========================================================================
    // EXIF Orientation — Extended Coverage
    // =========================================================================

    #[test]
    fn test_compress_jpeg_exif_flip_horizontal() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 2);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "flipped-h.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 60, "Flip H preserves width");
        assert_eq!(output_img.height(), 40, "Flip H preserves height");
    }

    #[test]
    fn test_compress_jpeg_exif_flip_vertical() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 4);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "flipped-v.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 60, "Flip V preserves width");
        assert_eq!(output_img.height(), 40, "Flip V preserves height");
    }

    #[test]
    fn test_compress_jpeg_exif_rotate270_flip_h() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 5);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "rot270-flip-h.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot270+FlipH swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot270+FlipH swaps to 60 tall");
    }

    #[test]
    fn test_compress_jpeg_exif_rotate90_flip_h() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 7);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "rot90-flip-h.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot90+FlipH swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot90+FlipH swaps to 60 tall");
    }

    #[test]
    fn test_compress_jpeg_exif_rotate270() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(&exif_jpeg, "rot270.jpg");

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot270 swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot270 swaps to 60 tall");
    }
}
