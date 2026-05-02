// Compress Images Node — reduce image file size via lossy/lossless re-encoding.
//
// Quality strategies by format:
//   JPEG: re-encode at lower quality (lossy, DCT-based)
//   PNG: color quantization to 8-bit indexed palette (lossy, via quantize.rs)
//   WebP: lossless re-encoding only (lossy requires libwebp, planned via jSquash)

use std::io::Cursor;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use image::codecs::jpeg::JpegEncoder;

use bnto_encode::ImageFormat;

use crate::common::{image_accepts, quality_param_def};
use crate::orientation::decode_with_orientation;

// --- Configuration Constants ---

use bnto_core::DEFAULT_QUALITY;

/// Minimum quality level. 1 = lowest quality, most compression.
const MIN_QUALITY: u8 = 1;

/// Maximum quality level. 100 = highest quality, least compression.
const MAX_QUALITY: u8 = 100;

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
    /// Reduces 24-bit truecolor to an indexed palette using quantizr.
    /// Quality is inverted to a compression level for the quantizer:
    /// quality=100 → compression=1 (minimal), quality=1 → compression=100 (maximum).
    fn compress_png(
        &self,
        data: &[u8],
        quality: u8,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        let compression = Self::quality_to_compression(quality);
        crate::quantize::compress_png_quantized(data, compression, progress)
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

    /// Extract the quality level from params and clamp to valid range.
    fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(MIN_QUALITY, MAX_QUALITY)
    }

    /// Convert a quality level (1-100) to a compression level (100-1) for PNG quantization.
    /// Formula: `compression = 101 - quality` (clamped to 1..100)
    fn quality_to_compression(quality: u8) -> u8 {
        101u8.saturating_sub(quality).max(1)
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
        "image-compress"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image-compress".to_string(),
            name: "Compress Images".to_string(),
            description: "Reduce image file size while maintaining quality".to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: vec![quality_param_def()],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    /// Process a single image: detect format, decode, re-encode with compression.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let data = input
            .data
            .into_bytes()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read input: {e}")))?;
        let format = ImageFormat::detect(&data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        let original_size = data.len();
        let compressed_data = self.compress_by_format(&data, &input.params, format, progress)?;
        let compressed_size = compressed_data.len();
        let output_filename = Self::output_filename(&input.filename, format);
        let metadata = build_compression_metadata(original_size, compressed_size, format);

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: FileData::Bytes(compressed_data),
                filename: output_filename,
                mime_type: format.mime_type().to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata,
        })
    }

    /// Validate quality parameters before processing.
    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(quality_val) = params.get("quality") {
            match quality_val.as_u64() {
                Some(q) if q >= MIN_QUALITY as u64 && q <= MAX_QUALITY as u64 => {}
                Some(q) => {
                    errors.push(format!(
                        "Quality must be between {MIN_QUALITY} and {MAX_QUALITY}, got {q}"
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

// --- Private helpers ---

impl CompressImages {
    /// Dispatch compression to the format-specific method.
    fn compress_by_format(
        &self,
        data: &[u8],
        params: &serde_json::Map<String, serde_json::Value>,
        format: ImageFormat,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        let quality = Self::get_quality(params);
        match format {
            ImageFormat::Jpeg => {
                progress.report(5, &format!("Compressing JPEG (quality: {quality})..."));
                self.compress_jpeg(data, quality, progress)
            }
            ImageFormat::Png => {
                let compression = Self::quality_to_compression(quality);
                progress.report(
                    5,
                    &format!("Optimizing PNG (quality: {quality}, compression: {compression})..."),
                );
                self.compress_png(data, quality, progress)
            }
            ImageFormat::WebP => {
                progress.report(5, "Compressing WebP (lossless)...");
                self.compress_webp(data, progress)
            }
        }
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    /// Extract bytes from FileData. Panics on FileData::Path (never produced by image processors).
    fn file_bytes(data: &FileData) -> &[u8] {
        match data {
            FileData::Bytes(b) => b,
            FileData::Path(_) => panic!("Expected FileData::Bytes"),
        }
    }

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
    fn make_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(data.to_vec()),
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
            data: FileData::Bytes(data.to_vec()),
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
        assert_eq!(processor.name(), "image-compress");
    }

    #[test]
    fn test_validate_passes_with_no_params() {
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
            serde_json::Value::Number(serde_json::Number::from(50u64)),
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
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_JPEG, "photo.jpg");

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);

        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&out_file.data)),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG"
        );

        assert!(
            out_file.filename.contains("-compressed"),
            "Output filename should contain '-compressed': got '{}'",
            out_file.filename
        );

        assert_eq!(out_file.mime_type, "image/jpeg");
    }

    #[test]
    fn test_compress_jpeg_default_quality_produces_smaller_file() {
        // Default quality (80) should produce a file that's the same size
        // or smaller than quality 90 input. The test JPEG was encoded at
        // quality 90, so re-encoding at 80 should be smaller.
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(TEST_MEDIUM_JPEG, "photo.jpg");
        let original_size = TEST_MEDIUM_JPEG.len();

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let compressed_size = file_bytes(&output.files[0].data).len();

        assert!(
            compressed_size < original_size,
            "Compressed JPEG ({} bytes) should be smaller than original ({} bytes)",
            compressed_size,
            original_size
        );
    }

    #[test]
    fn test_compress_jpeg_lower_quality_means_smaller_file() {
        // Quality 20 should produce a smaller file than quality 80.
        let processor = CompressImages::new();

        let input_q20 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 20);
        let input_q80 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 80);

        let output_q20 = processor
            .process(input_q20, &noop_progress(), &NoopContext)
            .unwrap();
        let output_q80 = processor
            .process(input_q80, &noop_progress(), &NoopContext)
            .unwrap();

        assert!(
            file_bytes(&output_q20.files[0].data).len()
                < file_bytes(&output_q80.files[0].data).len(),
            "Quality 20 ({} bytes) should be smaller than quality 80 ({} bytes)",
            file_bytes(&output_q20.files[0].data).len(),
            file_bytes(&output_q80.files[0].data).len()
        );
    }

    #[test]
    fn test_compress_jpeg_quality_clamped_to_range() {
        // Quality 0 should be clamped to 1 (our minimum).
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_JPEG, "photo.jpg", 0);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);

        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&out_file.data)),
            Some(ImageFormat::Png),
            "Output should be a valid PNG"
        );
        assert!(out_file.filename.contains("-compressed"));
        assert_eq!(out_file.mime_type, "image/png");
    }

    #[test]
    fn test_compress_png_with_medium_image() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_MEDIUM_PNG, "screenshot.png");
        let original_size = TEST_MEDIUM_PNG.len();

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let compressed_size = file_bytes(&output.files[0].data).len();

        assert!(
            compressed_size < original_size,
            "Quantized PNG should be smaller than original"
        );

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

    #[test]
    fn test_compress_png_lower_quality_means_smaller_file() {
        // Quality 20 should produce a smaller PNG than quality 80,
        // because lower quality maps to more aggressive quantization.
        let processor = CompressImages::new();

        let input_q80 = make_input_with_quality(TEST_MEDIUM_PNG, "screenshot.png", 80);
        let input_q20 = make_input_with_quality(TEST_MEDIUM_PNG, "screenshot.png", 20);

        let output_q80 = processor
            .process(input_q80, &noop_progress(), &NoopContext)
            .unwrap();
        let output_q20 = processor
            .process(input_q20, &noop_progress(), &NoopContext)
            .unwrap();

        let size_q80 = file_bytes(&output_q80.files[0].data).len();
        let size_q20 = file_bytes(&output_q20.files[0].data).len();

        assert!(
            size_q20 < size_q80,
            "PNG quality 20 ({} bytes) should be smaller than quality 80 ({} bytes)",
            size_q20,
            size_q80
        );

        eprintln!(
            "PNG quality test: q80={} bytes, q20={} bytes",
            size_q80, size_q20
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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);

        let out_file = &output.files[0];
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&out_file.data)),
            Some(ImageFormat::WebP),
            "Output should be a valid WebP"
        );
        assert!(out_file.filename.contains("-compressed"));
        assert_eq!(out_file.mime_type, "image/webp");
    }

    #[test]
    fn test_compress_webp_with_quality_param_produces_valid_output() {
        // WebP is lossless-only, but the quality param should be accepted
        // without error and produce valid output.
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_WEBP, "image.webp", 80);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::WebP),
            "WebP output should be valid even with quality param"
        );
    }

    // =========================================================================
    // Metadata Tests
    // =========================================================================

    #[test]
    fn test_output_includes_compression_metadata() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input(TEST_JPEG, "photo.jpg");

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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
    // Quality Parameter Tests
    // =========================================================================

    #[test]
    fn test_get_quality_default() {
        let params = serde_json::Map::new();
        assert_eq!(CompressImages::get_quality(&params), DEFAULT_QUALITY);
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
        assert_eq!(CompressImages::get_quality(&params), MAX_QUALITY);
    }

    #[test]
    fn test_get_quality_clamped_below_min() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0u64)),
        );
        assert_eq!(CompressImages::get_quality(&params), MIN_QUALITY);
    }

    #[test]
    fn test_get_quality_ignores_non_numeric() {
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::String("high".to_string()),
        );
        assert_eq!(CompressImages::get_quality(&params), DEFAULT_QUALITY);
    }

    // =========================================================================
    // Quality → Compression Inversion Tests (for PNG quantization)
    // =========================================================================

    #[test]
    fn test_quality_to_compression_inversion() {
        assert_eq!(CompressImages::quality_to_compression(100), 1);
        assert_eq!(CompressImages::quality_to_compression(81), 20);
        assert_eq!(CompressImages::quality_to_compression(51), 50);
        assert_eq!(CompressImages::quality_to_compression(21), 80);
        assert_eq!(CompressImages::quality_to_compression(1), 100);
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_unsupported_format_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"not an image at all", "document.pdf");
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

        assert!(result.is_err(), "Should return an error for corrupt JPEG");
    }

    #[test]
    fn test_corrupt_png_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let mut corrupt_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        corrupt_data.extend_from_slice(b"this is not real PNG data!!!!");

        let input = make_input(&corrupt_data, "corrupt.png");
        let result = processor.process(input, &progress, &NoopContext);

        assert!(result.is_err(), "Should return an error for corrupt PNG");
    }

    #[test]
    fn test_empty_data_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(b"", "empty.jpg");
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

        assert!(result.is_err(), "Single-byte file should return an error");
    }

    #[test]
    fn test_single_byte_file_with_jpg_extension_returns_error() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let input = make_input(&[0x42], "tiny.jpg");
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

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
        let result = processor.process(input, &progress, &NoopContext);

        assert!(
            result.is_err(),
            "All-0xFF bytes with .jpg extension should fail at decode"
        );
    }

    // =========================================================================
    // Quality Parameter Boundary Tests
    // =========================================================================

    #[test]
    fn test_get_quality_at_exact_min_boundary() {
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
        let mut params = serde_json::Map::new();
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(101u64)),
        );
        assert_eq!(
            CompressImages::get_quality(&params),
            MAX_QUALITY,
            "Quality 101 should be clamped to 100 (just above max)"
        );
    }

    #[test]
    fn test_validate_quality_0_fails() {
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

    #[test]
    fn test_compress_jpeg_at_quality_1_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);

        let output = processor
            .process(input, &progress, &NoopContext)
            .expect("Quality 1 should produce valid output, not an error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Jpeg),
            "Quality 1 output should still be a valid JPEG"
        );

        assert!(
            file_bytes(&output.files[0].data).len() < TEST_MEDIUM_JPEG.len(),
            "Quality 1 ({} bytes) should be smaller than original ({} bytes)",
            file_bytes(&output.files[0].data).len(),
            TEST_MEDIUM_JPEG.len()
        );
    }

    #[test]
    fn test_compress_jpeg_at_quality_100_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();
        let input = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let output = processor
            .process(input, &progress, &NoopContext)
            .expect("Quality 100 should produce valid output, not an error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Jpeg),
            "Quality 100 output should be a valid JPEG"
        );

        assert!(
            !file_bytes(&output.files[0].data).is_empty(),
            "Quality 100 output should not be empty"
        );
    }

    #[test]
    fn test_quality_1_smaller_than_quality_100() {
        let processor = CompressImages::new();

        let input_q1 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);
        let input_q100 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let output_q1 = processor
            .process(input_q1, &noop_progress(), &NoopContext)
            .unwrap();
        let output_q100 = processor
            .process(input_q100, &noop_progress(), &NoopContext)
            .unwrap();

        let size_q1 = file_bytes(&output_q1.files[0].data).len();
        let size_q100 = file_bytes(&output_q100.files[0].data).len();

        assert!(
            size_q1 < size_q100,
            "Quality 1 ({} bytes) should be smaller than quality 100 ({} bytes)",
            size_q1,
            size_q100
        );
    }

    #[test]
    fn test_quality_monotonically_affects_file_size() {
        let processor = CompressImages::new();

        let input_q1 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 1);
        let input_q50 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 50);
        let input_q100 = make_input_with_quality(TEST_MEDIUM_JPEG, "photo.jpg", 100);

        let out_q1 = processor
            .process(input_q1, &noop_progress(), &NoopContext)
            .unwrap();
        let size_q1 = file_bytes(&out_q1.files[0].data).len();
        let out_q50 = processor
            .process(input_q50, &noop_progress(), &NoopContext)
            .unwrap();
        let size_q50 = file_bytes(&out_q50.files[0].data).len();
        let out_q100 = processor
            .process(input_q100, &noop_progress(), &NoopContext)
            .unwrap();
        let size_q100 = file_bytes(&out_q100.files[0].data).len();

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

        eprintln!(
            "Quality ordering: q1={} bytes, q50={} bytes, q100={} bytes",
            size_q1, size_q50, size_q100
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
        use image::codecs::png::{CompressionType, FilterType, PngEncoder};
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
            .process(input, &progress, &NoopContext)
            .expect("1x1 JPEG should compress without error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
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
            .process(input, &progress, &NoopContext)
            .expect("1x1 PNG should compress without error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
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
            .process(input, &progress, &NoopContext)
            .expect("1x1 WebP should compress without error");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
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
    fn test_1x1_jpeg_with_quality_1_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_quality(&jpeg_bytes, "tiny_q1.jpg", 1);

        let output = processor
            .process(input, &progress, &NoopContext)
            .expect("1x1 JPEG at quality 1 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / quality 1"
        );
    }

    #[test]
    fn test_1x1_jpeg_with_quality_100_produces_valid_output() {
        let processor = CompressImages::new();
        let progress = noop_progress();

        let jpeg_bytes = create_1x1_jpeg();
        let input = make_input_with_quality(&jpeg_bytes, "tiny_q100.jpg", 100);

        let output = processor
            .process(input, &progress, &NoopContext)
            .expect("1x1 JPEG at quality 100 should produce valid output");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Jpeg),
            "Output should be a valid JPEG even at 1x1 / quality 100"
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_data = file_bytes(&result.files[0].data);

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_data = file_bytes(&result.files[0].data);

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_data = file_bytes(&result.files[0].data);

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_img = decode_with_orientation(file_bytes(&result.files[0].data)).unwrap();
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_img = decode_with_orientation(file_bytes(&result.files[0].data)).unwrap();
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_img = decode_with_orientation(file_bytes(&result.files[0].data)).unwrap();
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_img = decode_with_orientation(file_bytes(&result.files[0].data)).unwrap();
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let output_img = decode_with_orientation(file_bytes(&result.files[0].data)).unwrap();
        assert_eq!(output_img.width(), 40, "Rot270 swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot270 swaps to 60 tall");
    }
}
