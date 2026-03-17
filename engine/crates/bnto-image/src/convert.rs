// =============================================================================
// Convert Image Format Node — Change Between JPEG, PNG, and WebP
// =============================================================================
//
// Convert-image-format node: decode any supported format → re-encode to the
// target format. All formats share the same intermediate pixel representation,
// so conversion is: decode(source) → encode(target).
//
// Format semantics:
//   JPEG→PNG: lossless capture of remaining quality (won't restore lost detail)
//   PNG→JPEG: significant size reduction, transparency becomes opaque
//   Any→WebP: lossless only (Rust `image` crate limitation, lossy needs libwebp)

use std::io::Cursor;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::format::ImageFormat;
use crate::orientation::decode_with_orientation;

// =============================================================================
// Configuration Constants
// =============================================================================

use bnto_core::DEFAULT_JPEG_QUALITY as DEFAULT_QUALITY;

/// Rust `image` crate WebP encoder is lossless-only; values above 85 produce
/// much larger files without meaningful quality improvement.
const MAX_WEBP_QUALITY: u8 = 85;

const MIN_QUALITY: u8 = 1;
const MAX_QUALITY: u8 = 100;

// =============================================================================
// The ConvertImageFormat Processor
// =============================================================================

/// Convert-image-format node processor.
pub struct ConvertImageFormat;

impl Default for ConvertImageFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvertImageFormat {
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Internal Methods
    // =========================================================================

    /// Decode any supported format into pixels, applying EXIF orientation.
    /// Smartphone photos with EXIF rotation are physically rotated so the
    /// converted output displays correctly without needing an EXIF tag.
    fn decode_image(
        data: &[u8],
        progress: &ProgressReporter,
    ) -> Result<image::DynamicImage, BntoError> {
        progress.report(10, "Decoding image...");
        decode_with_orientation(data)
    }

    fn encode_jpeg(
        img: &image::DynamicImage,
        quality: u8,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, &format!("Encoding as JPEG (quality: {quality})..."));
        let mut output = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut output, quality);
        // write_with_encoder handles RGBA→RGB conversion internally
        // (JPEG doesn't support transparency).
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;
        Ok(output)
    }

    /// PNG is always lossless — compression level only affects speed vs size.
    fn encode_png(
        img: &image::DynamicImage,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, "Encoding as PNG (lossless)...");
        let mut output = Vec::new();
        let encoder =
            PngEncoder::new_with_quality(&mut output, CompressionType::Best, FilterType::Adaptive);
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode PNG: {e}")))?;
        Ok(output)
    }

    /// Lossless-only in the Rust `image` crate. Lossy WebP requires
    /// Google's libwebp (C library) which doesn't compile to our WASM target.
    fn encode_webp(
        img: &image::DynamicImage,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, "Encoding as WebP (lossless)...");
        let mut output = Vec::new();
        // Cursor needed because WebP encoder requires Seek to backfill
        // header fields (like total file size) after encoding.
        let mut cursor_out = Cursor::new(&mut output);
        img.write_to(&mut cursor_out, image::ImageFormat::WebP)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;
        Ok(output)
    }

    /// Parse target format from string. Accepts "jpeg", "jpg", "png", "webp"
    /// (case-insensitive).
    fn parse_target_format(format_str: &str) -> Result<ImageFormat, BntoError> {
        let lower = format_str.to_lowercase();
        match lower.as_str() {
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(BntoError::InvalidInput(format!(
                "Unsupported target format: '{}'. Supported: jpeg, png, webp",
                format_str
            ))),
        }
    }

    /// Extract quality, capping WebP at MAX_WEBP_QUALITY to prevent bloat
    /// (lossless encoder + high quality = large files with no quality gain).
    fn get_quality(
        params: &serde_json::Map<String, serde_json::Value>,
        target_format: ImageFormat,
    ) -> u8 {
        let raw_quality = params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(MIN_QUALITY, MAX_QUALITY);

        match target_format {
            ImageFormat::WebP => raw_quality.min(MAX_WEBP_QUALITY),
            _ => raw_quality,
        }
    }

    /// Replace the original extension with the target format's extension.
    /// "photo.jpg" + target=PNG -> "photo.png"
    fn output_filename(input_filename: &str, target_format: ImageFormat) -> String {
        // rfind('.') handles filenames with multiple dots like "my.photo.jpg"
        if let Some(dot_pos) = input_filename.rfind('.') {
            let stem = &input_filename[..dot_pos];
            format!("{stem}.{}", target_format.extension())
        } else {
            format!("{input_filename}.{}", target_format.extension())
        }
    }
}

// =============================================================================
// NodeProcessor Trait Implementation
// =============================================================================

impl NodeProcessor for ConvertImageFormat {
    fn name(&self) -> &str {
        "convert-image-format"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image".to_string(),
            operation: "convert".to_string(),
            name: "Convert Image Format".to_string(),
            description: "Convert images between JPEG, PNG, and WebP formats".to_string(),
            category: NodeCategory::Image,
            accepts: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/webp".to_string(),
            ],
            platforms: vec!["browser".to_string()],
            parameters: vec![
                ParameterDef {
                    name: "format".to_string(),
                    label: "Output Format".to_string(),
                    description: "The target image format to convert to".to_string(),
                    param_type: ParameterType::Enum {
                        options: vec!["jpeg".to_string(), "png".to_string(), "webp".to_string()],
                    },
                    constraints: Some(Constraints {
                        min: None,
                        max: None,
                        required: true,
                    }),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "convert".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "quality".to_string(),
                    label: "Quality".to_string(),
                    description: "Output quality for lossy formats (1-100)".to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::json!(80)),
                    constraints: Some(Constraints {
                        min: Some(1.0),
                        max: Some(100.0),
                        required: false,
                    }),
                    // Quality applies to resize and convert — compress has its
                    // own "compression" param with inverted semantics.
                    visible_when: Some(ParamCondition::Any(vec![
                        ParamConditionEntry {
                            param: "operation".to_string(),
                            equals: "resize".to_string(),
                        },
                        ParamConditionEntry {
                            param: "operation".to_string(),
                            equals: "convert".to_string(),
                        },
                    ])),
                    ..Default::default()
                },
            ],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // Parse required "format" param.
        let format_str = input
            .params
            .get("format")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BntoError::InvalidInput(
                    "Missing required 'format' parameter. Specify 'jpeg', 'png', or 'webp'."
                        .to_string(),
                )
            })?;

        let target_format = Self::parse_target_format(format_str)?;

        let input_format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        let original_size = input.data.len();
        progress.report(
            5,
            &format!("Converting {:?} → {:?}...", input_format, target_format),
        );
        let img = Self::decode_image(&input.data, progress)?;

        let quality = Self::get_quality(&input.params, target_format);
        let converted_data = match target_format {
            ImageFormat::Jpeg => Self::encode_jpeg(&img, quality, progress)?,
            ImageFormat::Png => Self::encode_png(&img, progress)?,
            ImageFormat::WebP => Self::encode_webp(&img, progress)?,
        };

        let converted_size = converted_data.len();
        let output_filename = Self::output_filename(&input.filename, target_format);

        progress.report(90, "Building output...");

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalFormat".to_string(),
            serde_json::Value::String(format!("{:?}", input_format)),
        );
        metadata.insert(
            "targetFormat".to_string(),
            serde_json::Value::String(format!("{:?}", target_format)),
        );
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(original_size as u64)),
        );
        metadata.insert(
            "newSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(converted_size as u64)),
        );

        progress.report(100, "Conversion complete");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: converted_data,
                filename: output_filename,
                mime_type: target_format.mime_type().to_string(),
            }],
            metadata,
        })
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        match params.get("format") {
            None => {
                errors.push(
                    "Missing required 'format' parameter. Specify 'jpeg', 'png', or 'webp'."
                        .to_string(),
                );
            }
            Some(format_val) => match format_val.as_str() {
                None => {
                    errors.push(format!(
                        "Format must be a string ('jpeg', 'png', or 'webp'), got: {format_val}"
                    ));
                }
                Some(format_str) => {
                    if Self::parse_target_format(format_str).is_err() {
                        errors.push(format!(
                            "Unsupported format: '{}'. Supported: jpeg, png, webp",
                            format_str
                        ));
                    }
                }
            },
        }

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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    fn make_input(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

    fn format_params(format: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert(
            "format".to_string(),
            serde_json::Value::String(format.to_string()),
        );
        params
    }

    fn format_quality_params(
        format: &str,
        quality: u64,
    ) -> serde_json::Map<String, serde_json::Value> {
        let mut params = format_params(format);
        params.insert(
            "quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from(quality)),
        );
        params
    }

    // =========================================================================
    // Format Parsing Tests
    // =========================================================================

    #[test]
    fn test_parse_jpeg_format() {
        let result = ConvertImageFormat::parse_target_format("jpeg");
        assert_eq!(result.unwrap(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_parse_jpg_format() {
        // "jpg" is an alias for "jpeg" — both should work.
        let result = ConvertImageFormat::parse_target_format("jpg");
        assert_eq!(result.unwrap(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_parse_png_format() {
        let result = ConvertImageFormat::parse_target_format("png");
        assert_eq!(result.unwrap(), ImageFormat::Png);
    }

    #[test]
    fn test_parse_webp_format() {
        let result = ConvertImageFormat::parse_target_format("webp");
        assert_eq!(result.unwrap(), ImageFormat::WebP);
    }

    #[test]
    fn test_parse_format_case_insensitive() {
        assert_eq!(
            ConvertImageFormat::parse_target_format("JPEG").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            ConvertImageFormat::parse_target_format("Png").unwrap(),
            ImageFormat::Png
        );
        assert_eq!(
            ConvertImageFormat::parse_target_format("WEBP").unwrap(),
            ImageFormat::WebP
        );
    }

    #[test]
    fn test_parse_invalid_format_returns_error() {
        let result = ConvertImageFormat::parse_target_format("bmp");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("bmp"),
            "Error should mention the format: {error_msg}"
        );
    }

    #[test]
    fn test_parse_empty_format_returns_error() {
        let result = ConvertImageFormat::parse_target_format("");
        assert!(result.is_err());
    }

    // =========================================================================
    // Quality Extraction Tests
    // =========================================================================

    #[test]
    fn test_quality_default_when_not_specified() {
        let params = serde_json::Map::new();
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::Jpeg);
        assert_eq!(quality, DEFAULT_QUALITY);
    }

    #[test]
    fn test_quality_respected_for_jpeg() {
        let params = format_quality_params("jpeg", 50);
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::Jpeg);
        assert_eq!(quality, 50);
    }

    #[test]
    fn test_quality_capped_for_webp() {
        // Quality > 85 for WebP should be capped at 85.
        let params = format_quality_params("webp", 100);
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::WebP);
        assert_eq!(quality, MAX_WEBP_QUALITY);
    }

    #[test]
    fn test_quality_not_capped_for_webp_when_below_max() {
        let params = format_quality_params("webp", 70);
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::WebP);
        assert_eq!(quality, 70);
    }

    #[test]
    fn test_quality_clamped_to_valid_range() {
        let params = format_quality_params("jpeg", 0);
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::Jpeg);
        assert_eq!(quality, MIN_QUALITY);
    }

    // =========================================================================
    // Filename Generation Tests
    // =========================================================================

    #[test]
    fn test_output_filename_jpeg_to_png() {
        let name = ConvertImageFormat::output_filename("photo.jpg", ImageFormat::Png);
        assert_eq!(name, "photo.png");
    }

    #[test]
    fn test_output_filename_png_to_jpeg() {
        let name = ConvertImageFormat::output_filename("screenshot.png", ImageFormat::Jpeg);
        assert_eq!(name, "screenshot.jpg");
    }

    #[test]
    fn test_output_filename_jpeg_to_webp() {
        let name = ConvertImageFormat::output_filename("photo.jpeg", ImageFormat::WebP);
        assert_eq!(name, "photo.webp");
    }

    #[test]
    fn test_output_filename_webp_to_png() {
        let name = ConvertImageFormat::output_filename("image.webp", ImageFormat::Png);
        assert_eq!(name, "image.png");
    }

    #[test]
    fn test_output_filename_no_extension() {
        let name = ConvertImageFormat::output_filename("myfile", ImageFormat::Jpeg);
        assert_eq!(name, "myfile.jpg");
    }

    #[test]
    fn test_output_filename_multiple_dots() {
        // Only the last extension should be replaced.
        let name = ConvertImageFormat::output_filename("my.photo.jpg", ImageFormat::Png);
        assert_eq!(name, "my.photo.png");
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_missing_format_returns_error() {
        let processor = ConvertImageFormat::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert!(!errors.is_empty(), "Missing format should produce an error");
        assert!(
            errors[0].contains("format"),
            "Error should mention 'format'"
        );
    }

    #[test]
    fn test_validate_invalid_format_returns_error() {
        let processor = ConvertImageFormat::new();
        let params = format_params("bmp");
        let errors = processor.validate(&params);
        assert!(!errors.is_empty(), "Invalid format should produce an error");
        assert!(
            errors[0].contains("bmp"),
            "Error should mention the bad format"
        );
    }

    #[test]
    fn test_validate_valid_format_returns_empty() {
        let processor = ConvertImageFormat::new();
        let params = format_params("png");
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Valid format should produce no errors");
    }

    #[test]
    fn test_validate_format_not_a_string_returns_error() {
        let processor = ConvertImageFormat::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "format".to_string(),
            serde_json::Value::Number(serde_json::Number::from(42)),
        );
        let errors = processor.validate(&params);
        assert!(
            !errors.is_empty(),
            "Non-string format should produce an error"
        );
    }

    #[test]
    fn test_validate_quality_out_of_range_returns_error() {
        let processor = ConvertImageFormat::new();
        let params = format_quality_params("jpeg", 200);
        let errors = processor.validate(&params);
        assert!(!errors.is_empty(), "Quality > 100 should produce an error");
    }

    #[test]
    fn test_validate_quality_valid_returns_empty() {
        let processor = ConvertImageFormat::new();
        let params = format_quality_params("jpeg", 80);
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Valid quality should produce no errors");
    }

    // =========================================================================
    // Conversion Tests — Using Real Test Fixture Images
    // =========================================================================

    const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
    const TEST_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");
    const TEST_WEBP: &[u8] = include_bytes!("../../../../test-fixtures/images/small.webp");

    #[test]
    fn test_convert_jpeg_to_png() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("png"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "JPEG → PNG should succeed");

        let output = result.unwrap();
        assert_eq!(output.files.len(), 1);

        let file = &output.files[0];
        assert_eq!(file.filename, "photo.png");
        assert_eq!(file.mime_type, "image/png");

        // Verify PNG magic bytes: 89 50 4E 47 (".PNG")
        assert!(file.data.len() > 8, "PNG output should have data");
        assert_eq!(file.data[0], 0x89, "PNG should start with 0x89");
        assert_eq!(file.data[1], 0x50, "PNG byte 2 should be 'P'");
        assert_eq!(file.data[2], 0x4E, "PNG byte 3 should be 'N'");
        assert_eq!(file.data[3], 0x47, "PNG byte 4 should be 'G'");

        assert_eq!(
            output
                .metadata
                .get("originalFormat")
                .and_then(|v| v.as_str()),
            Some("Jpeg")
        );
        assert_eq!(
            output.metadata.get("targetFormat").and_then(|v| v.as_str()),
            Some("Png")
        );
    }

    #[test]
    fn test_convert_png_to_jpeg() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_PNG, "screenshot.png", format_params("jpeg"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "PNG → JPEG should succeed");

        let output = result.unwrap();
        assert_eq!(output.files.len(), 1);

        let file = &output.files[0];
        assert_eq!(file.filename, "screenshot.jpg");
        assert_eq!(file.mime_type, "image/jpeg");

        // Verify JPEG magic bytes: FF D8 FF
        assert!(file.data.len() > 3, "JPEG output should have data");
        assert_eq!(file.data[0], 0xFF);
        assert_eq!(file.data[1], 0xD8);
        assert_eq!(file.data[2], 0xFF);
    }

    #[test]
    fn test_convert_jpeg_to_webp() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("webp"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "JPEG → WebP should succeed");

        let output = result.unwrap();
        assert_eq!(output.files.len(), 1);

        let file = &output.files[0];
        assert_eq!(file.filename, "photo.webp");
        assert_eq!(file.mime_type, "image/webp");

        // Verify WebP magic bytes: RIFF....WEBP
        assert!(file.data.len() > 12, "WebP output should have data");
        assert_eq!(file.data[0], b'R');
        assert_eq!(file.data[1], b'I');
        assert_eq!(file.data[2], b'F');
        assert_eq!(file.data[3], b'F');
        assert_eq!(file.data[8], b'W');
        assert_eq!(file.data[9], b'E');
        assert_eq!(file.data[10], b'B');
        assert_eq!(file.data[11], b'P');
    }

    #[test]
    fn test_convert_webp_to_png() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_WEBP, "image.webp", format_params("png"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "WebP → PNG should succeed");

        let output = result.unwrap();
        let file = &output.files[0];
        assert_eq!(file.filename, "image.png");
        assert_eq!(file.mime_type, "image/png");

        assert_eq!(file.data[0], 0x89);
        assert_eq!(file.data[1], 0x50);
    }

    #[test]
    fn test_convert_same_format_jpeg_to_jpeg() {
        // Re-encoding JPEG→JPEG is valid (may change file size based on quality).
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("jpeg"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "JPEG → JPEG should succeed");

        let output = result.unwrap();
        let file = &output.files[0];
        assert_eq!(file.filename, "photo.jpg");
        assert_eq!(file.mime_type, "image/jpeg");

        assert_eq!(file.data[0], 0xFF);
        assert_eq!(file.data[1], 0xD8);
    }

    #[test]
    fn test_convert_missing_format_param_returns_error() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", serde_json::Map::new());

        let result = processor.process(input, &progress);
        assert!(result.is_err(), "Missing format should return an error");

        // Can't use unwrap_err() because NodeOutput doesn't implement Debug.
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("format"),
                "Error should mention 'format': {error_msg}"
            );
        }
    }

    #[test]
    fn test_convert_invalid_format_param_returns_error() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("gif"));

        let result = processor.process(input, &progress);
        assert!(result.is_err(), "Invalid format should return an error");

        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("gif"),
                "Error should mention the bad format: {error_msg}"
            );
        }
    }

    #[test]
    fn test_convert_quality_param_respected_jpeg() {
        // Convert PNG → JPEG with quality 50 vs quality 100.
        // Quality 50 should produce a smaller file than quality 100.
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();

        let input_low = make_input(TEST_PNG, "img.png", format_quality_params("jpeg", 50));
        let output_low = processor.process(input_low, &progress).unwrap();
        let size_low = output_low.files[0].data.len();

        let input_high = make_input(TEST_PNG, "img.png", format_quality_params("jpeg", 100));
        let output_high = processor.process(input_high, &progress).unwrap();
        let size_high = output_high.files[0].data.len();

        assert!(
            size_high >= size_low,
            "Quality 100 ({size_high} bytes) should be >= quality 50 ({size_low} bytes)"
        );
    }

    #[test]
    fn test_convert_corrupt_data_returns_error() {
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(b"this is not an image", "corrupt.jpg", format_params("png"));

        let result = processor.process(input, &progress);
        assert!(result.is_err(), "Corrupt image data should return an error");
    }

    // =========================================================================
    // Node Name and Default Tests
    // =========================================================================

    #[test]
    fn test_node_name() {
        let processor = ConvertImageFormat::new();
        assert_eq!(processor.name(), "convert-image-format");
    }

    #[test]
    fn test_default_creates_instance() {
        let processor = ConvertImageFormat;
        assert_eq!(processor.name(), "convert-image-format");
    }

    // =========================================================================
    // EXIF Orientation Tests — Full Pipeline
    // =========================================================================
    //
    // These tests verify that format conversion correctly applies EXIF
    // orientation. A JPEG with orientation=6 converted to PNG should
    // produce a PNG with correctly rotated pixels.

    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    #[test]
    fn test_convert_exif_rotated_jpeg_to_png() {
        // Create a 60×40 JPEG with EXIF orientation=6 (rotate 90° CW).
        // Convert to PNG. The output PNG should be 40×60 (rotated).
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "portrait.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 40, "Converted PNG should be 40 wide");
        assert_eq!(output_img.height(), 60, "Converted PNG should be 60 tall");
    }

    #[test]
    fn test_convert_exif_rotated_jpeg_to_webp() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "portrait.jpg", format_params("webp"));

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 40);
        assert_eq!(output_img.height(), 60);
    }

    #[test]
    fn test_convert_no_exif_jpeg_to_png_preserves_dimensions() {
        let jpeg = create_test_jpeg(60, 40);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&jpeg, "landscape.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_data = &result.files[0].data;

        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 60);
        assert_eq!(output_img.height(), 40);
    }

    // =========================================================================
    // EXIF Orientation — Extended Coverage
    // =========================================================================
    //
    // Orientations 2, 4, 5, 7, 8 were only tested in orientation.rs (unit).
    // These tests verify they also work through the full convert pipeline.

    #[test]
    fn test_convert_exif_flip_horizontal_jpeg_to_png() {
        // Orientation=2: flip horizontal. Dimensions stay 60×40.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 2);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "flipped-h.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 60, "Flip H preserves width");
        assert_eq!(output_img.height(), 40, "Flip H preserves height");
    }

    #[test]
    fn test_convert_exif_flip_vertical_jpeg_to_png() {
        // Orientation=4: flip vertical. Dimensions stay 60×40.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 4);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "flipped-v.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 60, "Flip V preserves width");
        assert_eq!(output_img.height(), 40, "Flip V preserves height");
    }

    #[test]
    fn test_convert_exif_rotate270_flip_h_jpeg_to_png() {
        // Orientation=5: rotate 270° CW + flip horizontal.
        // 90°-family rotation swaps dimensions: 60×40 → 40×60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 5);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot270-flip-h.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot270+FlipH swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot270+FlipH swaps to 60 tall");
    }

    #[test]
    fn test_convert_exif_rotate90_flip_h_jpeg_to_png() {
        // Orientation=7: rotate 90° CW + flip horizontal.
        // 90°-family rotation swaps dimensions: 60×40 → 40×60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 7);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot90-flip-h.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot90+FlipH swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot90+FlipH swaps to 60 tall");
    }

    #[test]
    fn test_convert_exif_rotate270_jpeg_to_png() {
        // Orientation=8: rotate 270° CW. Dimensions swap: 60×40 → 40×60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot270.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 40, "Rot270 swaps to 40 wide");
        assert_eq!(output_img.height(), 60, "Rot270 swaps to 60 tall");
    }

    #[test]
    fn test_convert_exif_rotate180_jpeg_to_png() {
        // Orientation=3: rotate 180°. Dimensions stay 60×40.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot180.jpg", format_params("png"));

        let result = processor.process(input, &progress).unwrap();
        let output_img = decode_with_orientation(&result.files[0].data).unwrap();
        assert_eq!(output_img.width(), 60, "Rot180 preserves width");
        assert_eq!(output_img.height(), 40, "Rot180 preserves height");
    }
}
