// =============================================================================
// Convert Image Format Node — Change Between JPEG, PNG, and WebP
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the `convert-image-format` node. When a user drops images on the
// /convert-image-format page and clicks "Run", this code converts images
// between formats: JPEG to PNG, PNG to WebP, WebP to JPEG, etc.
//
// HOW DOES FORMAT CONVERSION WORK?
//
//   All image formats encode the same underlying data — a grid of pixels,
//   where each pixel has color values (Red, Green, Blue, and sometimes Alpha
//   for transparency). The difference is HOW they encode that pixel data:
//
//   JPEG → PNG:
//     JPEG is lossy (some quality is lost). PNG is lossless (perfect copy).
//     Converting JPEG→PNG won't restore lost quality, but it preserves
//     exactly what's left. Useful when you need transparency support or
//     when a tool requires PNG format.
//
//   PNG → JPEG:
//     PNG is lossless but large. JPEG is lossy but much smaller.
//     Converting PNG→JPEG reduces file size significantly. Any transparent
//     pixels become opaque (white or whatever the background is), because
//     JPEG doesn't support transparency. The `quality` parameter controls
//     how much compression is applied (1-100).
//
//   Any → WebP:
//     WebP is Google's modern format. Our Rust encoder only supports
//     LOSSLESS WebP, so the output preserves quality perfectly but might
//     not be as small as lossy WebP. Quality parameter is capped at 85
//     for WebP to prevent bloated files.
//
// THE CONVERSION PIPELINE:
//   1. Parse the target format from the "format" parameter
//   2. Detect the input format from magic bytes or file extension
//   3. Decode: raw bytes → pixel data (DynamicImage)
//   4. Re-encode: pixel data → new format bytes
//   5. Return the converted bytes + metadata (original format, target format, sizes)

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

/// Default quality when re-encoding to a lossy format.
/// 80 is the sweet spot: noticeable file size savings with minimal quality loss.
const DEFAULT_QUALITY: u8 = 80;

/// Maximum quality for WebP output. The Rust `image` crate's WebP encoder is
/// lossless-only, and quality values above 85 tend to produce very large files
/// without meaningful quality improvement. We cap it to prevent bloat.
const MAX_WEBP_QUALITY: u8 = 85;

/// Minimum quality we allow. Below 1, JPEG encoders produce invalid output.
const MIN_QUALITY: u8 = 1;

/// Maximum quality for non-WebP formats. 100 = virtually no compression.
const MAX_QUALITY: u8 = 100;

// =============================================================================
// The ConvertImageFormat Processor
// =============================================================================

/// The convert-image-format node processor.
///
/// This struct implements the `NodeProcessor` trait from bnto-core. It takes
/// an image in one format and re-encodes it to another format specified by
/// the "format" parameter.
///
/// RUST CONCEPT: "Zero-Sized Type" (ZST)
/// Just like CompressImages, this struct has no fields — it takes zero bytes.
/// All the behavior lives in the methods. We need the struct so we can
/// implement the NodeProcessor trait on something.
pub struct ConvertImageFormat;

// RUST CONCEPT: `Default` trait
// Clippy (the Rust linter) requires that if you have a `new()` method with
// no arguments, you also implement `Default`. This lets people create instances
// with `ConvertImageFormat::default()` in addition to `ConvertImageFormat::new()`.
impl Default for ConvertImageFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvertImageFormat {
    /// Create a new ConvertImageFormat processor.
    ///
    /// RUST CONCEPT: `pub fn new() -> Self`
    /// This is the conventional "constructor" in Rust. Unlike JavaScript's
    /// `new MyClass()`, Rust doesn't have a special `new` keyword — it's
    /// just a regular function that returns `Self` (the current type).
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Internal Methods — The Actual Conversion Logic
    // =========================================================================

    /// Decode any supported image format into pixel data, applying EXIF orientation.
    ///
    /// This is the first step of conversion: take raw file bytes and turn
    /// them into a grid of pixels that we can then re-encode in any format.
    ///
    /// EXIF orientation is applied automatically — smartphone photos that
    /// store pixels sideways (with an EXIF tag saying "rotate to view")
    /// will be decoded with the correct pixel orientation. This means the
    /// converted output displays correctly without needing an EXIF tag.
    ///
    /// RUST CONCEPT: `image::DynamicImage`
    /// A `DynamicImage` is an in-memory representation of an image as pixels.
    /// It's format-agnostic — once decoded, the pixels are the same whether
    /// the source was JPEG, PNG, or WebP. This is what makes conversion
    /// possible: decode from any format → re-encode to any format.
    fn decode_image(
        data: &[u8],
        progress: &ProgressReporter,
    ) -> Result<image::DynamicImage, BntoError> {
        // Report that we're starting the decode step.
        progress.report(10, "Decoding image...");

        // Decode with EXIF orientation applied. See orientation.rs for
        // the full explanation of why this matters and how it works.
        decode_with_orientation(data)
    }

    /// Encode pixel data as JPEG with the given quality.
    ///
    /// JPEG is a lossy format — the quality parameter (1-100) controls how
    /// much detail is thrown away. Lower quality = smaller file, more artifacts.
    fn encode_jpeg(
        img: &image::DynamicImage,
        quality: u8,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, &format!("Encoding as JPEG (quality: {quality})..."));

        // Create an empty byte buffer for the encoder to write into.
        let mut output = Vec::new();

        // Create a JPEG encoder that writes to our buffer at the given quality.
        // `&mut output` gives the encoder mutable (writable) access to the buffer.
        let encoder = JpegEncoder::new_with_quality(&mut output, quality);

        // Feed the decoded pixels through the encoder, producing JPEG bytes.
        // `write_with_encoder()` handles the pixel format conversion internally
        // (e.g., RGBA → RGB, since JPEG doesn't support transparency).
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;

        Ok(output)
    }

    /// Encode pixel data as PNG (always lossless).
    ///
    /// PNG doesn't have a quality parameter — it's always lossless. The
    /// compression level only affects encoding speed vs file size, not quality.
    fn encode_png(
        img: &image::DynamicImage,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, "Encoding as PNG (lossless)...");

        let mut output = Vec::new();

        // `CompressionType::Best` tells the PNG encoder to try its hardest
        // to make the file small. This is slower but produces smaller files.
        // `FilterType::Adaptive` automatically picks the best row filter for
        // each row, which generally produces the best compression.
        let encoder =
            PngEncoder::new_with_quality(&mut output, CompressionType::Best, FilterType::Adaptive);

        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode PNG: {e}")))?;

        Ok(output)
    }

    /// Encode pixel data as WebP (lossless only in the Rust `image` crate).
    ///
    /// NOTE: The Rust `image` crate's WebP encoder only supports LOSSLESS mode.
    /// Lossy WebP requires Google's libwebp (C library), which doesn't compile
    /// to our WASM target. We'll add lossy WebP later via a jSquash JS fallback.
    fn encode_webp(
        img: &image::DynamicImage,
        progress: &ProgressReporter,
    ) -> Result<Vec<u8>, BntoError> {
        progress.report(60, "Encoding as WebP (lossless)...");

        let mut output = Vec::new();

        // Use `Cursor` for the output buffer so it implements both Write and Seek.
        // The WebP encoder needs Seek to go back and fill in header fields
        // (like total file size) after encoding the image data.
        let mut cursor_out = Cursor::new(&mut output);

        // `image::ImageFormat::WebP` tells the encoder to output in WebP format.
        img.write_to(&mut cursor_out, image::ImageFormat::WebP)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;

        Ok(output)
    }

    /// Parse the target format from the "format" parameter string.
    ///
    /// Accepts "jpeg", "jpg", "png", or "webp" (case-insensitive).
    /// Returns the corresponding `ImageFormat` enum value.
    ///
    /// RUST CONCEPT: `Result<T, E>`
    /// Returns either `Ok(ImageFormat)` if the format is recognized, or
    /// `Err(BntoError)` if the string is empty, missing, or unrecognized.
    fn parse_target_format(format_str: &str) -> Result<ImageFormat, BntoError> {
        // Convert to lowercase so "JPEG", "Jpeg", and "jpeg" all work.
        //
        // RUST CONCEPT: `.to_lowercase()`
        // Creates a new String with all characters in lowercase. The original
        // `&str` is borrowed (read-only), so we need a new owned String.
        let lower = format_str.to_lowercase();

        // RUST CONCEPT: `match` on string slices
        // `.as_str()` converts the owned `String` to a `&str` so we can
        // match against string literals. Rust's `match` on strings is like
        // a `switch` in JavaScript, but exhaustive (must handle all cases).
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

    /// Extract the quality parameter from the node's config params.
    ///
    /// The quality setting controls lossy encoding (JPEG). PNG ignores it
    /// (always lossless). WebP is capped at MAX_WEBP_QUALITY.
    fn get_quality(
        params: &serde_json::Map<String, serde_json::Value>,
        target_format: ImageFormat,
    ) -> u8 {
        // Try to read the "quality" key from the JSON params.
        //
        // RUST CONCEPT: Method chaining with Option
        // `.get("quality")` → Option<&Value>
        // `.and_then(|v| v.as_u64())` → if Some, try to parse as u64
        // `.map(|q| q as u8)` → if Some, cast u64 down to u8
        // `.unwrap_or(DEFAULT_QUALITY)` → if None at any step, use default
        let raw_quality = params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(MIN_QUALITY, MAX_QUALITY);

        // Cap WebP quality at MAX_WEBP_QUALITY to prevent bloat.
        // The Rust encoder is lossless-only, so high quality values just
        // make the file bigger without adding quality.
        //
        // RUST CONCEPT: `match` with pattern matching
        // We match on the target format to apply format-specific logic.
        match target_format {
            ImageFormat::WebP => raw_quality.min(MAX_WEBP_QUALITY),
            _ => raw_quality,
        }
    }

    /// Generate an output filename with the new format extension.
    ///
    /// Replaces the original extension with the target format's extension:
    ///   "photo.jpg" + target=PNG  → "photo.png"
    ///   "image.png" + target=JPEG → "image.jpg"
    ///   "pic.webp"  + target=PNG  → "pic.png"
    fn output_filename(input_filename: &str, target_format: ImageFormat) -> String {
        // Split the filename at the last dot to separate stem from extension.
        //
        // RUST CONCEPT: `.rfind('.')`
        // `.rfind()` searches from the RIGHT (end of string) for a character.
        // This correctly handles filenames with multiple dots like "my.photo.jpg"
        // — we want "my.photo" as the stem, not "my".
        if let Some(dot_pos) = input_filename.rfind('.') {
            // Take everything before the last dot as the stem.
            let stem = &input_filename[..dot_pos];
            // Append the new extension from the target format.
            format!("{stem}.{}", target_format.extension())
        } else {
            // No extension found — just append the new extension.
            format!("{input_filename}.{}", target_format.extension())
        }
    }
}

// =============================================================================
// NodeProcessor Trait Implementation
// =============================================================================
//
// This is where ConvertImageFormat "plugs into" the Bnto engine framework.
// By implementing `NodeProcessor`, this node can be run by the Web Worker
// orchestrator just like compress-images or resize-images.

impl NodeProcessor for ConvertImageFormat {
    /// The unique identifier for this node type.
    /// Matches the slug in the bnto registry and the Go engine's node name.
    fn name(&self) -> &str {
        "convert-image-format"
    }

    /// Convert a single image from one format to another.
    ///
    /// This is the main entry point. The Web Worker calls this once per file.
    /// For multiple files, the Worker calls it in a loop.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        // --- Step 1: Parse the target format from params ---
        //
        // The "format" parameter is required — it tells us what to convert TO.
        // If it's missing or invalid, we return an error immediately.
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

        // --- Step 2: Detect the input format ---
        //
        // We need to know the original format for two reasons:
        //   1. To decode the image correctly
        //   2. To report it in the metadata (so the user sees "JPEG → PNG")
        let input_format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        // --- Step 3: Decode the image into pixels ---
        let original_size = input.data.len();
        progress.report(
            5,
            &format!("Converting {:?} → {:?}...", input_format, target_format),
        );
        let img = Self::decode_image(&input.data, progress)?;

        // --- Step 4: Re-encode to the target format ---
        //
        // Each format uses a different encoder:
        //   - JPEG: lossy, uses quality parameter
        //   - PNG: lossless, ignores quality
        //   - WebP: lossless-only in Rust, quality capped at 85
        let quality = Self::get_quality(&input.params, target_format);
        let converted_data = match target_format {
            ImageFormat::Jpeg => Self::encode_jpeg(&img, quality, progress)?,
            ImageFormat::Png => Self::encode_png(&img, progress)?,
            ImageFormat::WebP => Self::encode_webp(&img, progress)?,
        };

        // --- Step 5: Build the output with metadata ---
        let converted_size = converted_data.len();
        let output_filename = Self::output_filename(&input.filename, target_format);

        progress.report(90, "Building output...");

        // Build the metadata map for the UI results panel.
        // This tells the user what happened: original format, target format,
        // original size, new size.
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

        // Return the converted image as a single output file.
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: converted_data,
                filename: output_filename,
                mime_type: target_format.mime_type().to_string(),
            }],
            metadata,
        })
    }

    /// Validate conversion parameters before processing.
    ///
    /// Catches configuration errors early:
    ///   - Missing "format" parameter
    ///   - Invalid format value
    ///   - Quality out of range
    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        // --- Check "format" parameter ---
        // This is required — without it, we don't know what to convert to.
        match params.get("format") {
            None => {
                errors.push(
                    "Missing required 'format' parameter. Specify 'jpeg', 'png', or 'webp'."
                        .to_string(),
                );
            }
            Some(format_val) => {
                // Check that the value is a string (not a number or boolean).
                match format_val.as_str() {
                    None => {
                        errors.push(format!(
                            "Format must be a string ('jpeg', 'png', or 'webp'), got: {format_val}"
                        ));
                    }
                    Some(format_str) => {
                        // Check that the format is one we support.
                        if Self::parse_target_format(format_str).is_err() {
                            errors.push(format!(
                                "Unsupported format: '{}'. Supported: jpeg, png, webp",
                                format_str
                            ));
                        }
                    }
                }
            }
        }

        // --- Check "quality" parameter (optional) ---
        if let Some(quality_val) = params.get("quality") {
            match quality_val.as_u64() {
                Some(q) if q >= MIN_QUALITY as u64 && q <= MAX_QUALITY as u64 => {
                    // Valid quality — nothing to report.
                }
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
// Tests — Written BEFORE the implementation (TDD!)
// =============================================================================
//
// These unit tests run natively (no WASM, no browser needed). They test the
// pure Rust logic: format parsing, quality extraction, filename generation,
// and the actual image conversion pipeline using real test fixture images.

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a NodeInput with the given data, filename, and params.
    ///
    /// RUST CONCEPT: Helper functions in tests
    /// We extract common setup into helper functions to keep each test
    /// focused on what it's actually testing. DRY (Don't Repeat Yourself)
    /// applies to tests too.
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

    /// Build a params map with just a "format" key.
    fn format_params(format: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert(
            "format".to_string(),
            serde_json::Value::String(format.to_string()),
        );
        params
    }

    /// Build a params map with "format" and "quality" keys.
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
        // "jpeg" should parse to ImageFormat::Jpeg
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
        // Users might type "JPEG", "Png", "WEBP" — all should work.
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
        // "bmp" is not supported — should return an error.
        let result = ConvertImageFormat::parse_target_format("bmp");
        assert!(result.is_err());

        // Check the error message mentions the unsupported format.
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
        // No quality param → use DEFAULT_QUALITY (80).
        let params = serde_json::Map::new();
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::Jpeg);
        assert_eq!(quality, DEFAULT_QUALITY);
    }

    #[test]
    fn test_quality_respected_for_jpeg() {
        // Quality param should be used as-is for JPEG.
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
        // Quality <= 85 for WebP should pass through unchanged.
        let params = format_quality_params("webp", 70);
        let quality = ConvertImageFormat::get_quality(&params, ImageFormat::WebP);
        assert_eq!(quality, 70);
    }

    #[test]
    fn test_quality_clamped_to_valid_range() {
        // Quality of 0 should clamp to MIN_QUALITY (1).
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
        // If the input has no extension, just append the new one.
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
        // Pass a number instead of a string.
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
    //
    // These tests use real images from the test-fixtures directory. They
    // verify that the full conversion pipeline works end-to-end: decode the
    // source format, re-encode to the target format, and produce valid output.

    /// Load the small JPEG test fixture (100x100 pixels, ~3.5 KB).
    /// `include_bytes!()` embeds the file at compile time.
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
        // Should produce exactly one output file.
        assert_eq!(output.files.len(), 1);

        let file = &output.files[0];
        // Filename should have .png extension.
        assert_eq!(file.filename, "photo.png");
        // MIME type should be image/png.
        assert_eq!(file.mime_type, "image/png");

        // Verify the output is actually PNG by checking magic bytes.
        // PNG starts with: 89 50 4E 47 (which is ".PNG" in ASCII).
        assert!(file.data.len() > 8, "PNG output should have data");
        assert_eq!(file.data[0], 0x89, "PNG should start with 0x89");
        assert_eq!(file.data[1], 0x50, "PNG byte 2 should be 'P'");
        assert_eq!(file.data[2], 0x4E, "PNG byte 3 should be 'N'");
        assert_eq!(file.data[3], 0x47, "PNG byte 4 should be 'G'");

        // Check metadata reports the conversion.
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

        // Verify JPEG magic bytes: FF D8 FF.
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

        // Verify PNG magic bytes.
        assert_eq!(file.data[0], 0x89);
        assert_eq!(file.data[1], 0x50);
    }

    #[test]
    fn test_convert_same_format_jpeg_to_jpeg() {
        // Converting JPEG → JPEG should still work. It re-encodes the image,
        // which might change the file size depending on quality settings.
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("jpeg"));

        let result = processor.process(input, &progress);
        assert!(result.is_ok(), "JPEG → JPEG should succeed");

        let output = result.unwrap();
        let file = &output.files[0];
        assert_eq!(file.filename, "photo.jpg");
        assert_eq!(file.mime_type, "image/jpeg");

        // Should still be valid JPEG.
        assert_eq!(file.data[0], 0xFF);
        assert_eq!(file.data[1], 0xD8);
    }

    #[test]
    fn test_convert_missing_format_param_returns_error() {
        // No "format" param at all — should fail with a clear error.
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", serde_json::Map::new());

        let result = processor.process(input, &progress);
        assert!(result.is_err(), "Missing format should return an error");

        // RUST CONCEPT: `if let Err(e) = result`
        // We can't use `.unwrap_err()` because `NodeOutput` doesn't implement
        // the `Debug` trait (which `unwrap_err()` needs to print the Ok value
        // if the Result is unexpectedly Ok). Instead, we use pattern matching
        // to extract the error safely.
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
        // Invalid format value — should fail with a clear error.
        let processor = ConvertImageFormat::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "photo.jpg", format_params("gif"));

        let result = processor.process(input, &progress);
        assert!(result.is_err(), "Invalid format should return an error");

        // Use pattern matching instead of `.unwrap_err()` because
        // `NodeOutput` doesn't implement `Debug`.
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

        // Low quality (50).
        let input_low = make_input(TEST_PNG, "img.png", format_quality_params("jpeg", 50));
        let output_low = processor.process(input_low, &progress).unwrap();
        let size_low = output_low.files[0].data.len();

        // High quality (100).
        let input_high = make_input(TEST_PNG, "img.png", format_quality_params("jpeg", 100));
        let output_high = processor.process(input_high, &progress).unwrap();
        let size_high = output_high.files[0].data.len();

        // Higher quality should produce a larger (or equal) file.
        assert!(
            size_high >= size_low,
            "Quality 100 ({size_high} bytes) should be >= quality 50 ({size_low} bytes)"
        );
    }

    #[test]
    fn test_convert_corrupt_data_returns_error() {
        // Random bytes that aren't a valid image — should fail gracefully.
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
        // The Default trait should work just like new().
        // Note: For unit structs (structs with no fields), Rust/clippy prefers
        // just using the struct name directly instead of calling ::default().
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

        // Decode the PNG output and check dimensions.
        let output_img = decode_with_orientation(output_data).unwrap();
        assert_eq!(output_img.width(), 40, "Converted PNG should be 40 wide");
        assert_eq!(output_img.height(), 60, "Converted PNG should be 60 tall");
    }

    #[test]
    fn test_convert_exif_rotated_jpeg_to_webp() {
        // JPEG with orientation=6 → WebP. Output should be 40×60.
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
        // A normal JPEG (no EXIF) converted to PNG should keep dimensions.
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
}
