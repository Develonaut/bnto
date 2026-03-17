// =============================================================================
// Resize Images Node — Change Image Dimensions
// =============================================================================
//
// Resize-images node: decode → calculate target dimensions → resize with
// resampling filter → re-encode in the same format. Supports aspect ratio
// preservation, configurable quality, and automatic EXIF orientation.
//
// Resampling filters:
//   Lanczos3 (downscale) — 6x6 neighborhood sinc, sharp edges, best quality
//   CatmullRom (upscale) — 4x4 cubic spline, smooth interpolation, no ringing

use std::io::Cursor;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType as PngFilterType, PngEncoder};
use image::imageops::FilterType;

use crate::format::ImageFormat;
use crate::orientation::decode_with_orientation;

// =============================================================================
// Configuration Constants
// =============================================================================

use bnto_core::DEFAULT_JPEG_QUALITY;

/// 16384px — practical limit for browsers and the `image` crate in WASM.
const MAX_DIMENSION: u32 = 16384;

const MIN_DIMENSION: u32 = 1;

// =============================================================================
// The ResizeImages Processor
// =============================================================================

/// Resize-images node processor. Supports width-only, height-only, or
/// explicit dimensions with optional aspect ratio preservation.
pub struct ResizeImages;

impl Default for ResizeImages {
    fn default() -> Self {
        Self::new()
    }
}

impl ResizeImages {
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Internal Methods — Parameter Extraction
    // =========================================================================

    /// Extract target width. Returns `None` if unspecified — caller must
    /// calculate from height + aspect ratio, or fail if neither was given.
    fn get_width(params: &serde_json::Map<String, serde_json::Value>) -> Option<u32> {
        params
            .get("width")
            .and_then(|v| v.as_u64())
            .map(|w| w as u32)
    }

    fn get_height(params: &serde_json::Map<String, serde_json::Value>) -> Option<u32> {
        params
            .get("height")
            .and_then(|v| v.as_u64())
            .map(|h| h as u32)
    }

    /// Default: true (prevents distortion when only one dimension given).
    fn get_maintain_aspect(params: &serde_json::Map<String, serde_json::Value>) -> bool {
        params
            .get("maintainAspect")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_JPEG_QUALITY)
            .clamp(1, 100)
    }

    // =========================================================================
    // Internal Methods — Dimension Calculation
    // =========================================================================

    /// Calculate final (width, height) from params + original dimensions.
    ///
    /// Rules:
    ///   - Both specified: use directly
    ///   - One specified + maintainAspect: calculate the other from ratio
    ///   - Neither specified: error
    ///   - maintainAspect=false + one dimension: use original for the other
    fn calculate_dimensions(
        target_width: Option<u32>,
        target_height: Option<u32>,
        original_width: u32,
        original_height: u32,
        maintain_aspect: bool,
    ) -> Result<(u32, u32), BntoError> {
        if target_width.is_none() && target_height.is_none() {
            return Err(BntoError::InvalidInput(
                "At least one dimension (width or height) must be specified".to_string(),
            ));
        }

        if original_width == 0 || original_height == 0 {
            return Err(BntoError::InvalidInput(
                "Original image has zero dimensions".to_string(),
            ));
        }

        let (final_width, final_height) = match (target_width, target_height) {
            // Both given — use directly (user knows what they want).
            (Some(w), Some(h)) => (w, h),

            // Width only — calculate height from aspect ratio.
            (Some(w), None) => {
                if maintain_aspect {
                    // Use f64 to avoid integer rounding errors.
                    let ratio = original_height as f64 / original_width as f64;
                    let h = (w as f64 * ratio).round() as u32;
                    (w, h.max(MIN_DIMENSION))
                } else {
                    (w, original_height)
                }
            }

            // Height only — calculate width from aspect ratio.
            (None, Some(h)) => {
                if maintain_aspect {
                    let ratio = original_width as f64 / original_height as f64;
                    let w = (h as f64 * ratio).round() as u32;
                    (w.max(MIN_DIMENSION), h)
                } else {
                    (original_width, h)
                }
            }

            // Guarded above — exhaustive match requires this branch.
            (None, None) => unreachable!(),
        };

        if final_width > MAX_DIMENSION || final_height > MAX_DIMENSION {
            return Err(BntoError::InvalidInput(format!(
                "Target dimensions {}×{} exceed maximum allowed {}×{}",
                final_width, final_height, MAX_DIMENSION, MAX_DIMENSION
            )));
        }

        if final_width < MIN_DIMENSION || final_height < MIN_DIMENSION {
            return Err(BntoError::InvalidInput(format!(
                "Target dimensions {}×{} are below minimum {}×{}",
                final_width, final_height, MIN_DIMENSION, MIN_DIMENSION
            )));
        }

        Ok((final_width, final_height))
    }

    // =========================================================================
    // Internal Methods — Resize + Re-encode
    // =========================================================================

    /// Lanczos3 for downscaling (sharp, detail-preserving), CatmullRom for
    /// upscaling (smooth, avoids ringing artifacts Lanczos can produce).
    fn choose_filter(original_pixels: u32, target_pixels: u32) -> FilterType {
        if target_pixels < original_pixels {
            FilterType::Lanczos3
        } else {
            FilterType::CatmullRom
        }
    }

    fn encode_jpeg(img: &image::DynamicImage, quality: u8) -> Result<Vec<u8>, BntoError> {
        let mut output = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut output, quality);
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode JPEG: {e}")))?;
        Ok(output)
    }

    fn encode_png(img: &image::DynamicImage) -> Result<Vec<u8>, BntoError> {
        let mut output = Vec::new();
        let encoder = PngEncoder::new_with_quality(
            &mut output,
            CompressionType::Best,
            PngFilterType::Adaptive,
        );
        img.write_with_encoder(encoder)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode PNG: {e}")))?;
        Ok(output)
    }

    fn encode_webp(img: &image::DynamicImage) -> Result<Vec<u8>, BntoError> {
        let mut output = Vec::new();
        let mut cursor_out = Cursor::new(&mut output);
        img.write_to(&mut cursor_out, image::ImageFormat::WebP)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to encode WebP: {e}")))?;
        Ok(output)
    }

    /// "photo.jpg" -> "photo-resized.jpg"
    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        if let Some(dot_pos) = input_filename.rfind('.') {
            let stem = &input_filename[..dot_pos];
            format!("{stem}-resized.{}", format.extension())
        } else {
            format!("{input_filename}-resized.{}", format.extension())
        }
    }
}

// =============================================================================
// NodeProcessor Trait Implementation
// =============================================================================

impl NodeProcessor for ResizeImages {
    fn name(&self) -> &str {
        "resize-images"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image".to_string(),
            operation: "resize".to_string(),
            name: "Resize Images".to_string(),
            description: "Change image dimensions while maintaining quality".to_string(),
            category: NodeCategory::Image,
            accepts: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/webp".to_string(),
            ],
            platforms: vec!["browser".to_string()],
            parameters: vec![
                ParameterDef {
                    name: "width".to_string(),
                    label: "Width".to_string(),
                    description: "Target width in pixels".to_string(),
                    param_type: ParameterType::Number,
                    constraints: Some(Constraints {
                        min: Some(1.0),
                        max: None,
                        required: false,
                    }),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "resize".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "height".to_string(),
                    label: "Height".to_string(),
                    description: "Target height in pixels".to_string(),
                    param_type: ParameterType::Number,
                    constraints: Some(Constraints {
                        min: Some(1.0),
                        max: None,
                        required: false,
                    }),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "resize".to_string(),
                    })),
                    ..Default::default()
                },
                ParameterDef {
                    name: "maintainAspect".to_string(),
                    label: "Maintain Aspect Ratio".to_string(),
                    description: "Keep the original width-to-height ratio when resizing"
                        .to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::json!(true)),
                    visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                        param: "operation".to_string(),
                        equals: "resize".to_string(),
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
        let format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        let target_width = Self::get_width(&input.params);
        let target_height = Self::get_height(&input.params);
        let maintain_aspect = Self::get_maintain_aspect(&input.params);
        let quality = Self::get_quality(&input.params);

        // EXIF orientation matters for resize: a 4000x3000 stored image with
        // orientation=6 is really a 3000x4000 portrait. Resize must work
        // against the ORIENTED dimensions, not the stored pixel dimensions.
        progress.report(10, "Decoding image...");
        let img = decode_with_orientation(&input.data)?;

        let original_width = img.width();
        let original_height = img.height();

        let (final_width, final_height) = Self::calculate_dimensions(
            target_width,
            target_height,
            original_width,
            original_height,
            maintain_aspect,
        )?;

        progress.report(
            30,
            &format!("Resizing to {}×{}...", final_width, final_height),
        );

        let original_pixels = original_width * original_height;
        let target_pixels = final_width * final_height;
        let filter = Self::choose_filter(original_pixels, target_pixels);

        // `resize_exact` uses our calculated dimensions directly, unlike
        // `resize` which fits within a bounding box.
        let resized = img.resize_exact(final_width, final_height, filter);

        progress.report(70, "Encoding resized image...");

        // Preserve the original format (JPEG in -> JPEG out, etc.)
        let output_data = match format {
            ImageFormat::Jpeg => Self::encode_jpeg(&resized, quality)?,
            ImageFormat::Png => Self::encode_png(&resized)?,
            ImageFormat::WebP => Self::encode_webp(&resized)?,
        };

        let output_filename = Self::output_filename(&input.filename, format);

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalWidth".to_string(),
            serde_json::Value::Number(original_width.into()),
        );
        metadata.insert(
            "originalHeight".to_string(),
            serde_json::Value::Number(original_height.into()),
        );
        metadata.insert(
            "newWidth".to_string(),
            serde_json::Value::Number(final_width.into()),
        );
        metadata.insert(
            "newHeight".to_string(),
            serde_json::Value::Number(final_height.into()),
        );
        metadata.insert(
            "originalSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(input.data.len() as u64)),
        );
        metadata.insert(
            "newSize".to_string(),
            serde_json::Value::Number(serde_json::Number::from(output_data.len() as u64)),
        );
        metadata.insert(
            "format".to_string(),
            serde_json::Value::String(format!("{:?}", format)),
        );

        progress.report(100, "Resize complete");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: output_data,
                filename: output_filename,
                mime_type: format.mime_type().to_string(),
            }],
            metadata,
        })
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        let width = Self::get_width(params);
        let height = Self::get_height(params);

        if width.is_none() && height.is_none() {
            errors.push("At least one dimension (width or height) must be specified".to_string());
        }

        if let Some(w) = width {
            if w < MIN_DIMENSION {
                errors.push(format!("Width must be at least {MIN_DIMENSION}, got {w}"));
            }
            if w > MAX_DIMENSION {
                errors.push(format!("Width must be at most {MAX_DIMENSION}, got {w}"));
            }
        }

        if let Some(h) = height {
            if h < MIN_DIMENSION {
                errors.push(format!("Height must be at least {MIN_DIMENSION}, got {h}"));
            }
            if h > MAX_DIMENSION {
                errors.push(format!("Height must be at most {MAX_DIMENSION}, got {h}"));
            }
        }

        if let Some(quality_val) = params.get("quality") {
            match quality_val.as_u64() {
                Some(q) if (1..=100).contains(&q) => {}
                Some(q) => {
                    errors.push(format!("Quality must be between 1 and 100, got {q}"));
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
    use bnto_core::processor::NodeInput;
    use bnto_core::progress::ProgressReporter;

    // =========================================================================
    // Test Fixtures — Real images from the shared test-fixtures directory
    // =========================================================================

    /// A small JPEG image (100x100, ~2.7 KB)
    const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");

    /// A small PNG image (100x100, ~11 KB)
    const TEST_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");

    /// A small WebP image (100x100, ~966 bytes)
    const TEST_WEBP: &[u8] = include_bytes!("../../../../test-fixtures/images/small.webp");

    /// A medium JPEG image (400x400, ~23 KB) — for testing visible dimension changes
    const TEST_MEDIUM_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.jpg");

    /// Helper: create a NodeInput with given data, filename, and params.
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

    /// Helper: create a params map with the given width.
    fn width_params(width: u32) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert("width".to_string(), serde_json::Value::from(width));
        params
    }

    /// Helper: create a params map with width and height.
    fn dims_params(width: u32, height: u32) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert("width".to_string(), serde_json::Value::from(width));
        params.insert("height".to_string(), serde_json::Value::from(height));
        params
    }

    /// Helper: create a params map with height only.
    fn height_params(height: u32) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert("height".to_string(), serde_json::Value::from(height));
        params
    }

    /// Helper: decode an image from bytes and return its dimensions.
    /// Uses decode_with_orientation so test assertions match what the
    /// processor would see (orientation-corrected dimensions).
    fn get_image_dimensions(data: &[u8]) -> (u32, u32) {
        let img = decode_with_orientation(data).unwrap();
        (img.width(), img.height())
    }

    // =========================================================================
    // Processor Identity Tests
    // =========================================================================

    #[test]
    fn test_resize_processor_name() {
        let processor = ResizeImages::new();
        assert_eq!(processor.name(), "resize-images");
    }

    #[test]
    fn test_resize_processor_default() {
        // Default trait works the same as new()
        let processor = ResizeImages;
        assert_eq!(processor.name(), "resize-images");
    }

    // =========================================================================
    // Dimension Calculation Tests — Pure Logic, No Image Processing
    // =========================================================================

    #[test]
    fn test_calculate_dimensions_both_specified() {
        // When both width and height are given, use them directly.
        let result = ResizeImages::calculate_dimensions(Some(300), Some(200), 1200, 800, true);
        assert_eq!(result.unwrap(), (300, 200));
    }

    #[test]
    fn test_calculate_dimensions_width_only_with_aspect() {
        // Width=600, original=1200×800 → height = 600 × (800/1200) = 400
        let result = ResizeImages::calculate_dimensions(Some(600), None, 1200, 800, true);
        assert_eq!(result.unwrap(), (600, 400));
    }

    #[test]
    fn test_calculate_dimensions_height_only_with_aspect() {
        // Height=400, original=1200×800 → width = 400 × (1200/800) = 600
        let result = ResizeImages::calculate_dimensions(None, Some(400), 1200, 800, true);
        assert_eq!(result.unwrap(), (600, 400));
    }

    #[test]
    fn test_calculate_dimensions_width_only_no_aspect() {
        // Width=600, maintainAspect=false → height stays at original (800)
        let result = ResizeImages::calculate_dimensions(Some(600), None, 1200, 800, false);
        assert_eq!(result.unwrap(), (600, 800));
    }

    #[test]
    fn test_calculate_dimensions_height_only_no_aspect() {
        // Height=400, maintainAspect=false → width stays at original (1200)
        let result = ResizeImages::calculate_dimensions(None, Some(400), 1200, 800, false);
        assert_eq!(result.unwrap(), (1200, 400));
    }

    #[test]
    fn test_calculate_dimensions_neither_specified_is_error() {
        // No width or height — that's an error.
        let result = ResizeImages::calculate_dimensions(None, None, 1200, 800, true);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("At least one dimension"));
    }

    #[test]
    fn test_calculate_dimensions_zero_original_is_error() {
        // Original image has zero dimensions — can't compute aspect ratio.
        let result = ResizeImages::calculate_dimensions(Some(100), None, 0, 800, true);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("zero dimensions"));
    }

    #[test]
    fn test_calculate_dimensions_exceeds_max_is_error() {
        // Target exceeds MAX_DIMENSION (16384).
        let result = ResizeImages::calculate_dimensions(Some(20000), Some(20000), 100, 100, false);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("exceed maximum"));
    }

    #[test]
    fn test_calculate_dimensions_aspect_ratio_rounds_correctly() {
        // 1920×1080 resized to width=1280
        // height = 1280 × (1080/1920) = 1280 × 0.5625 = 720.0 (exact)
        let result = ResizeImages::calculate_dimensions(Some(1280), None, 1920, 1080, true);
        assert_eq!(result.unwrap(), (1280, 720));
    }

    #[test]
    fn test_calculate_dimensions_aspect_ratio_rounds_up() {
        // 1000×333 resized to width=500
        // height = 500 × (333/1000) = 500 × 0.333 = 166.5 → rounds to 167
        let result = ResizeImages::calculate_dimensions(Some(500), None, 1000, 333, true);
        let (_, h) = result.unwrap();
        assert_eq!(h, 167); // rounded from 166.5
    }

    #[test]
    fn test_calculate_dimensions_tiny_aspect_minimum_1px() {
        // 1000×1 resized to width=2
        // height = 2 × (1/1000) = 0.002 → rounds to 0 → clamped to 1
        let result = ResizeImages::calculate_dimensions(Some(2), None, 1000, 1, true);
        let (w, h) = result.unwrap();
        assert_eq!(w, 2);
        assert_eq!(h, 1); // minimum 1px
    }

    // =========================================================================
    // Filter Selection Tests
    // =========================================================================

    #[test]
    fn test_choose_filter_downscale_uses_lanczos3() {
        // Shrinking: use Lanczos3 for best quality
        let filter = ResizeImages::choose_filter(1000 * 1000, 500 * 500);
        assert_eq!(filter, FilterType::Lanczos3);
    }

    #[test]
    fn test_choose_filter_upscale_uses_catmullrom() {
        // Enlarging: use CatmullRom for smooth interpolation
        let filter = ResizeImages::choose_filter(500 * 500, 1000 * 1000);
        assert_eq!(filter, FilterType::CatmullRom);
    }

    #[test]
    fn test_choose_filter_same_size_uses_catmullrom() {
        // Same size: CatmullRom (upscale path — cheaper, same result)
        let filter = ResizeImages::choose_filter(500 * 500, 500 * 500);
        assert_eq!(filter, FilterType::CatmullRom);
    }

    // =========================================================================
    // Parameter Extraction Tests
    // =========================================================================

    #[test]
    fn test_get_width_present() {
        let params = width_params(800);
        assert_eq!(ResizeImages::get_width(&params), Some(800));
    }

    #[test]
    fn test_get_width_missing() {
        let params = serde_json::Map::new();
        assert_eq!(ResizeImages::get_width(&params), None);
    }

    #[test]
    fn test_get_width_not_a_number() {
        let mut params = serde_json::Map::new();
        params.insert(
            "width".to_string(),
            serde_json::Value::String("abc".to_string()),
        );
        assert_eq!(ResizeImages::get_width(&params), None);
    }

    #[test]
    fn test_get_height_present() {
        let params = height_params(600);
        assert_eq!(ResizeImages::get_height(&params), Some(600));
    }

    #[test]
    fn test_get_maintain_aspect_default_true() {
        let params = serde_json::Map::new();
        assert!(ResizeImages::get_maintain_aspect(&params));
    }

    #[test]
    fn test_get_maintain_aspect_explicit_false() {
        let mut params = serde_json::Map::new();
        params.insert("maintainAspect".to_string(), serde_json::Value::Bool(false));
        assert!(!ResizeImages::get_maintain_aspect(&params));
    }

    #[test]
    fn test_get_quality_default() {
        let params = serde_json::Map::new();
        assert_eq!(ResizeImages::get_quality(&params), 80);
    }

    #[test]
    fn test_get_quality_custom() {
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::Value::from(50));
        assert_eq!(ResizeImages::get_quality(&params), 50);
    }

    #[test]
    fn test_get_quality_clamped_to_range() {
        // Quality 0 clamps to 1
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::Value::from(0));
        assert_eq!(ResizeImages::get_quality(&params), 1);

        // Quality 200 clamps to 100
        params.insert("quality".to_string(), serde_json::Value::from(200));
        assert_eq!(ResizeImages::get_quality(&params), 100);
    }

    // =========================================================================
    // Output Filename Tests
    // =========================================================================

    #[test]
    fn test_output_filename_jpeg() {
        assert_eq!(
            ResizeImages::output_filename("photo.jpg", ImageFormat::Jpeg),
            "photo-resized.jpg"
        );
    }

    #[test]
    fn test_output_filename_png() {
        assert_eq!(
            ResizeImages::output_filename("screenshot.png", ImageFormat::Png),
            "screenshot-resized.png"
        );
    }

    #[test]
    fn test_output_filename_webp() {
        assert_eq!(
            ResizeImages::output_filename("image.webp", ImageFormat::WebP),
            "image-resized.webp"
        );
    }

    #[test]
    fn test_output_filename_no_extension() {
        assert_eq!(
            ResizeImages::output_filename("noext", ImageFormat::Jpeg),
            "noext-resized.jpg"
        );
    }

    #[test]
    fn test_output_filename_multiple_dots() {
        // "my.file.name.png" → stem is "my.file.name", extension is "png"
        assert_eq!(
            ResizeImages::output_filename("my.file.name.png", ImageFormat::Png),
            "my.file.name-resized.png"
        );
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_valid_width_only() {
        let processor = ResizeImages::new();
        let params = width_params(800);
        let errors = processor.validate(&params);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_valid_height_only() {
        let processor = ResizeImages::new();
        let params = height_params(600);
        let errors = processor.validate(&params);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_valid_both_dimensions() {
        let processor = ResizeImages::new();
        let params = dims_params(800, 600);
        let errors = processor.validate(&params);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_no_dimensions_is_error() {
        let processor = ResizeImages::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("At least one dimension"));
    }

    #[test]
    fn test_validate_width_too_large_is_error() {
        let processor = ResizeImages::new();
        let params = width_params(20000);
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("at most"));
    }

    #[test]
    fn test_validate_invalid_quality_is_error() {
        let processor = ResizeImages::new();
        let mut params = width_params(800);
        params.insert("quality".to_string(), serde_json::Value::from(200));
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Quality"));
    }

    #[test]
    fn test_validate_quality_not_a_number_is_error() {
        let processor = ResizeImages::new();
        let mut params = width_params(800);
        params.insert(
            "quality".to_string(),
            serde_json::Value::String("high".to_string()),
        );
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("must be a number"));
    }

    // =========================================================================
    // Full Processing Tests — Real Image Resize
    // =========================================================================

    #[test]
    fn test_resize_jpeg_width_only() {
        // Resize a 100×100 JPEG to width=50. With maintainAspect=true
        // (default), height should also be 50 (1:1 aspect ratio).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", width_params(50));

        let output = processor.process(input, &progress).unwrap();

        // Verify output structure
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "test-resized.jpg");
        assert_eq!(output.files[0].mime_type, "image/jpeg");

        // Verify the output is a valid JPEG with correct dimensions
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 50);
        assert_eq!(h, 50); // 1:1 aspect ratio preserved
    }

    #[test]
    fn test_resize_jpeg_both_dimensions() {
        // Resize to explicit 60×40 (ignoring aspect ratio).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(60, 40));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 60);
        assert_eq!(h, 40);
    }

    #[test]
    fn test_resize_png_width_only() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_PNG, "test.png", width_params(50));

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "test-resized.png");
        assert_eq!(output.files[0].mime_type, "image/png");

        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_webp_width_only() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_WEBP, "test.webp", width_params(50));

        let output = processor.process(input, &progress).unwrap();

        assert_eq!(output.files[0].filename, "test-resized.webp");
        assert_eq!(output.files[0].mime_type, "image/webp");

        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_height_only() {
        // Resize using height only — width calculated from aspect ratio.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", height_params(50));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(h, 50);
        assert_eq!(w, 50); // 1:1 aspect ratio
    }

    #[test]
    fn test_resize_upscale() {
        // Upscale from 100×100 to 200×200 (2x enlargement).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", width_params(200));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 200);
        assert_eq!(h, 200);
    }

    #[test]
    fn test_resize_medium_jpeg_non_square() {
        // Our medium JPEG is 400×400. Resize to width=200, should get 200×200.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_MEDIUM_JPEG, "photo.jpg", width_params(200));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 200);
        assert_eq!(h, 200);
    }

    #[test]
    fn test_resize_metadata_has_dimensions() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", width_params(50));

        let output = processor.process(input, &progress).unwrap();

        // Verify metadata contains all expected fields
        assert_eq!(output.metadata["originalWidth"], 100);
        assert_eq!(output.metadata["originalHeight"], 100);
        assert_eq!(output.metadata["newWidth"], 50);
        assert_eq!(output.metadata["newHeight"], 50);
        assert!(output.metadata.contains_key("originalSize"));
        assert!(output.metadata.contains_key("newSize"));
        assert_eq!(output.metadata["format"], "Jpeg");
    }

    #[test]
    fn test_resize_no_dimensions_is_error() {
        // No width or height params — should fail with a clear error.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", serde_json::Map::new());

        let result = processor.process(input, &progress);
        assert!(result.is_err());
        // RUST CONCEPT: `if let Err(e) = result`
        // We can't use `unwrap_err()` because `NodeOutput` doesn't
        // implement `Debug`. `if let` pattern matching works without it.
        if let Err(e) = result {
            assert!(e.to_string().contains("At least one dimension"));
        }
    }

    #[test]
    fn test_resize_unsupported_format_is_error() {
        // Random bytes that aren't a valid image
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(b"not an image", "test.txt", width_params(50));

        let result = processor.process(input, &progress);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Could not determine image format"));
        }
    }

    #[test]
    fn test_resize_corrupt_image_is_error() {
        // Valid JPEG header but truncated content
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let corrupt = &TEST_JPEG[..20]; // Only first 20 bytes — truncated
        let input = make_input(corrupt, "corrupt.jpg", width_params(50));

        let result = processor.process(input, &progress);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to decode"));
        }
    }

    #[test]
    fn test_resize_with_quality_param() {
        // JPEG resize with custom quality
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let mut params = width_params(50);
        params.insert("quality".to_string(), serde_json::Value::from(50));
        let input = make_input(TEST_JPEG, "test.jpg", params);

        let output = processor.process(input, &progress).unwrap();

        // Should produce valid output (quality affects file size, not dimensions)
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_same_size_is_valid() {
        // Resize to the same dimensions — should work (identity resize).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(100, 100));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 100);
        assert_eq!(h, 100);
    }

    #[test]
    fn test_resize_1x1_minimum() {
        // Resize to 1×1 — the smallest valid image.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(1, 1));

        let output = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&output.files[0].data);
        assert_eq!(w, 1);
        assert_eq!(h, 1);
    }

    // =========================================================================
    // EXIF Orientation Tests — Full Pipeline
    // =========================================================================
    //
    // These tests verify that resize uses ORIENTED dimensions, not raw
    // stored pixel dimensions. A 60×40 stored image with orientation=6
    // (rotate 90° CW) is a 40×60 portrait. Resizing "width=20" with
    // aspect ratio should calculate height from the oriented 40×60 ratio.

    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    #[test]
    fn test_resize_exif_rotated_image_uses_oriented_dimensions() {
        // Create a 60×40 JPEG with EXIF orientation=6 (rotate 90° CW).
        // After orientation, the image is 40×60 (portrait).
        // Resize to width=20, maintain aspect ratio.
        // Expected: 20×30 (aspect ratio 40:60 = 2:3).
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "portrait.jpg", width_params(20));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);

        assert_eq!(w, 20, "Resized width should be 20");
        assert_eq!(
            h, 30,
            "Resized height should preserve 2:3 portrait aspect ratio"
        );
    }

    #[test]
    fn test_resize_exif_rotated_image_height_only() {
        // Same 60×40 image with orientation=6 → oriented 40×60.
        // Resize to height=30, maintain aspect ratio.
        // Expected: 20×30 (aspect ratio 40:60 = 2:3).
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "portrait.jpg", height_params(30));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);

        assert_eq!(w, 20, "Width should maintain 2:3 aspect ratio");
        assert_eq!(h, 30, "Height should be 30 as requested");
    }

    #[test]
    fn test_resize_no_exif_preserves_normal_behavior() {
        // A normal JPEG without EXIF orientation — backward compatibility.
        // 60×40 image, resize to width=30, maintain aspect.
        // Expected: 30×20 (aspect ratio 60:40 = 3:2).
        let jpeg = create_test_jpeg(60, 40);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&jpeg, "landscape.jpg", width_params(30));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);

        assert_eq!(w, 30);
        assert_eq!(h, 20, "Height should preserve 3:2 landscape aspect ratio");
    }

    #[test]
    fn test_resize_metadata_reports_oriented_dimensions() {
        // Verify that the metadata (originalWidth, originalHeight) in the
        // output reflects the ORIENTED dimensions, not the raw stored ones.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "portrait.jpg", width_params(20));

        let result = processor.process(input, &progress).unwrap();

        // After orientation=6, the image is 40×60 (portrait).
        // The metadata should report these oriented dimensions.
        let orig_w = result
            .metadata
            .get("originalWidth")
            .and_then(|v| v.as_u64())
            .unwrap();
        let orig_h = result
            .metadata
            .get("originalHeight")
            .and_then(|v| v.as_u64())
            .unwrap();

        assert_eq!(orig_w, 40, "Metadata should report oriented width");
        assert_eq!(orig_h, 60, "Metadata should report oriented height");
    }

    // =========================================================================
    // EXIF Orientation — Extended Coverage
    // =========================================================================
    //
    // Orientations 2, 4, 5, 7, 8 were only tested in orientation.rs (unit).
    // These tests verify they also work through the full resize pipeline,
    // particularly that aspect ratio calculations use oriented dimensions.

    #[test]
    fn test_resize_exif_flip_horizontal() {
        // Orientation=2: flip horizontal. Oriented dimensions are 60×40
        // (same as stored — flips don't swap). Resize width=30 → 30×20.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 2);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "flipped-h.jpg", width_params(30));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 30, "Flip H: width=30");
        assert_eq!(h, 20, "Flip H: aspect 3:2 preserved → height=20");
    }

    #[test]
    fn test_resize_exif_flip_vertical() {
        // Orientation=4: flip vertical. Oriented dimensions are 60×40.
        // Resize width=30 → 30×20.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 4);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "flipped-v.jpg", width_params(30));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 30, "Flip V: width=30");
        assert_eq!(h, 20, "Flip V: aspect 3:2 preserved → height=20");
    }

    #[test]
    fn test_resize_exif_rotate270_flip_h() {
        // Orientation=5: rotate 270° CW + flip horizontal.
        // Oriented dimensions are 40×60 (portrait). Resize width=20 → 20×30.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 5);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot270-flip-h.jpg", width_params(20));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 20, "Rot270+FlipH: width=20");
        assert_eq!(h, 30, "Rot270+FlipH: aspect 2:3 → height=30");
    }

    #[test]
    fn test_resize_exif_rotate90_flip_h() {
        // Orientation=7: rotate 90° CW + flip horizontal.
        // Oriented dimensions are 40×60 (portrait). Resize width=20 → 20×30.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 7);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot90-flip-h.jpg", width_params(20));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 20, "Rot90+FlipH: width=20");
        assert_eq!(h, 30, "Rot90+FlipH: aspect 2:3 → height=30");
    }

    #[test]
    fn test_resize_exif_rotate270() {
        // Orientation=8: rotate 270° CW.
        // Oriented dimensions are 40×60 (portrait). Resize width=20 → 20×30.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot270.jpg", width_params(20));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 20, "Rot270: width=20");
        assert_eq!(h, 30, "Rot270: aspect 2:3 → height=30");
    }

    #[test]
    fn test_resize_exif_rotate180() {
        // Orientation=3: rotate 180°. Oriented dimensions are 60×40
        // (same as stored — 180° doesn't swap). Resize width=30 → 30×20.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(&exif_jpeg, "rot180.jpg", width_params(30));

        let result = processor.process(input, &progress).unwrap();
        let (w, h) = get_image_dimensions(&result.files[0].data);
        assert_eq!(w, 30, "Rot180: width=30");
        assert_eq!(h, 20, "Rot180: aspect 3:2 preserved → height=20");
    }
}
