// ResizeImages — NodeProcessor wiring for `image-resize`.
//
// Decodes with EXIF orientation, resolves target dimensions, resizes with
// the chosen filter, and re-encodes in the source format.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use bnto_encode::ImageFormat;

use super::param_defs::resize_param_defs;
use super::params::ResizeParams;
use super::validate::validate_resize_params;
use crate::common::image_accepts;
use crate::encode;
use crate::orientation::decode_with_orientation;

/// The resize-images node processor.
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

impl NodeProcessor for ResizeImages {
    fn name(&self) -> &str {
        "image-resize"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image-resize".to_string(),
            name: "Resize Images".to_string(),
            description: "Resize images to new dimensions while maintaining quality".to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: resize_param_defs(),
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    #[expect(clippy::too_many_lines)]
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
        let format = detect_format(&data, &input.filename)?;
        let params = ResizeParams::parse(&input.params);

        progress.report(10, "Decoding image...");
        let img = decode_with_orientation(&data)?;
        let orig = (img.width(), img.height());
        let (final_w, final_h) = params.target_dimensions(orig.0, orig.1)?;
        progress.report(30, &format!("Resizing to {}x{}...", final_w, final_h));

        let resized = resize_image(&img, final_w, final_h, orig.0, orig.1);
        let output_data = encode::encode_image(&resized, format, params.quality)?;
        let metadata =
            build_resize_metadata(orig, (final_w, final_h), data.len(), &output_data, format);

        progress.report(100, "Resize complete");
        Ok(build_resize_output(
            output_data,
            &input.filename,
            format,
            metadata,
        ))
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        validate_resize_params(params)
    }
}

// --- Private helpers for process ---

fn detect_format(data: &[u8], filename: &str) -> Result<ImageFormat, BntoError> {
    ImageFormat::detect(data, filename).ok_or_else(|| {
        BntoError::UnsupportedFormat(format!("Could not determine image format for '{filename}'"))
    })
}

fn resize_image(
    img: &image::DynamicImage,
    final_w: u32,
    final_h: u32,
    orig_w: u32,
    orig_h: u32,
) -> image::DynamicImage {
    let filter = ResizeImages::choose_filter(orig_w * orig_h, final_w * final_h);
    img.resize_exact(final_w, final_h, filter)
}

fn build_resize_output(
    data: Vec<u8>,
    input_filename: &str,
    format: ImageFormat,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data: FileData::Bytes(data),
            filename: ResizeImages::output_filename(input_filename, format),
            mime_type: format.mime_type().to_string(),
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

fn build_resize_metadata(
    (orig_w, orig_h): (u32, u32),
    (new_w, new_h): (u32, u32),
    original_size: usize,
    output_data: &[u8],
    format: ImageFormat,
) -> serde_json::Map<String, serde_json::Value> {
    let mut m = serde_json::Map::new();
    m.insert("originalWidth".to_string(), orig_w.into());
    m.insert("originalHeight".to_string(), orig_h.into());
    m.insert("newWidth".to_string(), new_w.into());
    m.insert("newHeight".to_string(), new_h.into());
    m.insert(
        "originalSize".to_string(),
        serde_json::Value::Number((original_size as u64).into()),
    );
    m.insert(
        "newSize".to_string(),
        serde_json::Value::Number((output_data.len() as u64).into()),
    );
    m.insert(
        "format".to_string(),
        serde_json::Value::String(format!("{:?}", format)),
    );
    m
}

// --- Tests ---

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
    // Test Fixtures — Real images from the shared test-fixtures directory
    // =========================================================================

    /// A small JPEG image (100x100, ~2.7 KB)
    const TEST_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.jpg");

    /// A small PNG image (100x100, ~11 KB)
    const TEST_PNG: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.png");

    /// A small WebP image (100x100, ~966 bytes)
    const TEST_WEBP: &[u8] = include_bytes!("../../../../../test-fixtures/images/small.webp");

    /// A medium JPEG image (400x400, ~23 KB) — for testing visible dimension changes
    const TEST_MEDIUM_JPEG: &[u8] =
        include_bytes!("../../../../../test-fixtures/images/medium.jpg");

    /// Helper: create a NodeInput with given data, filename, and params.
    fn make_input(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(data.to_vec()),
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
        assert_eq!(processor.name(), "image-resize");
    }

    #[test]
    fn test_resize_processor_default() {
        // Default trait works the same as new()
        let processor = ResizeImages;
        assert_eq!(processor.name(), "image-resize");
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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Verify output structure
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "test-resized.jpg");
        assert_eq!(output.files[0].mime_type, "image/jpeg");

        // Verify the output is a valid JPEG with correct dimensions
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 50);
        assert_eq!(h, 50); // 1:1 aspect ratio preserved
    }

    #[test]
    fn test_resize_jpeg_both_dimensions_fits_inside() {
        // 100×100 into a 60×40 box with maintainAspect (default) → 40×40.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(60, 40));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 40);
        assert_eq!(h, 40);
    }

    #[test]
    fn test_resize_jpeg_both_dimensions_no_aspect_stretches() {
        // maintainAspect=false resizes to the exact 60×40 box.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let mut params = dims_params(60, 40);
        params.insert("maintainAspect".to_string(), serde_json::Value::Bool(false));
        let input = make_input(TEST_JPEG, "test.jpg", params);

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 60);
        assert_eq!(h, 40);
    }

    #[test]
    fn test_resize_png_width_only() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_PNG, "test.png", width_params(50));

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "test-resized.png");
        assert_eq!(output.files[0].mime_type, "image/png");

        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_webp_width_only() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_WEBP, "test.webp", width_params(50));

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "test-resized.webp");
        assert_eq!(output.files[0].mime_type, "image/webp");

        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_height_only() {
        // Resize using height only — width calculated from aspect ratio.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", height_params(50));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(h, 50);
        assert_eq!(w, 50); // 1:1 aspect ratio
    }

    #[test]
    fn test_resize_upscale() {
        // Upscale from 100×100 to 200×200 (2x enlargement).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", width_params(200));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 200);
        assert_eq!(h, 200);
    }

    #[test]
    fn test_resize_medium_jpeg_non_square() {
        // Our medium JPEG is 400×400. Resize to width=200, should get 200×200.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_MEDIUM_JPEG, "photo.jpg", width_params(200));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 200);
        assert_eq!(h, 200);
    }

    #[test]
    fn test_resize_metadata_has_dimensions() {
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", width_params(50));

        let output = processor.process(input, &progress, &NoopContext).unwrap();

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

        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err());
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

        let result = processor.process(input, &progress, &NoopContext);
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

        let result = processor.process(input, &progress, &NoopContext);
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

        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Should produce valid output (quality affects file size, not dimensions)
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }

    #[test]
    fn test_resize_same_size_is_valid() {
        // Resize to the same dimensions — should work (identity resize).
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(100, 100));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 100);
        assert_eq!(h, 100);
    }

    #[test]
    fn test_resize_1x1_minimum() {
        // Resize to 1×1 — the smallest valid image.
        let processor = ResizeImages::new();
        let progress = ProgressReporter::new_noop();
        let input = make_input(TEST_JPEG, "test.jpg", dims_params(1, 1));

        let output = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&output.files[0].data));
        assert_eq!(w, 1);
        assert_eq!(h, 1);
    }

    // =========================================================================
    // Quality Parameter Verification — Different values produce different sizes
    // =========================================================================

    #[test]
    fn test_resize_jpeg_quality_affects_output_size() {
        // Resizing with quality 30 vs 90 should produce measurably different
        // JPEG file sizes. This verifies quality is actually applied.
        let processor = ResizeImages::new();

        let mut params_q30 = width_params(200);
        params_q30.insert("quality".to_string(), serde_json::Value::from(30));
        let input_q30 = make_input(TEST_MEDIUM_JPEG, "photo.jpg", params_q30);

        let mut params_q90 = width_params(200);
        params_q90.insert("quality".to_string(), serde_json::Value::from(90));
        let input_q90 = make_input(TEST_MEDIUM_JPEG, "photo.jpg", params_q90);

        let output_q30 = processor
            .process(input_q30, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        let output_q90 = processor
            .process(input_q90, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();

        let size_q30 = file_bytes(&output_q30.files[0].data).len();
        let size_q90 = file_bytes(&output_q90.files[0].data).len();

        assert!(
            size_q30 < size_q90,
            "Quality 30 ({} bytes) should be smaller than quality 90 ({} bytes)",
            size_q30,
            size_q90
        );
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();

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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
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

        let result = processor.process(input, &progress, &NoopContext).unwrap();
        let (w, h) = get_image_dimensions(file_bytes(&result.files[0].data));
        assert_eq!(w, 30, "Rot180: width=30");
        assert_eq!(h, 20, "Rot180: aspect 3:2 preserved → height=20");
    }

    // =========================================================================
    // enlarge — leave images smaller than the target alone
    // =========================================================================

    /// A large JPEG image (1200×800).
    const TEST_LARGE_JPEG: &[u8] = include_bytes!("../../../../../test-fixtures/images/large.jpg");

    fn with_enlarge(
        mut params: serde_json::Map<String, serde_json::Value>,
        enlarge: bool,
    ) -> serde_json::Map<String, serde_json::Value> {
        params.insert("enlarge".to_string(), serde_json::Value::Bool(enlarge));
        params
    }

    fn resized_dimensions(
        data: &[u8],
        params: serde_json::Map<String, serde_json::Value>,
    ) -> (u32, u32) {
        let input = make_input(data, "photo.jpg", params);
        let output = ResizeImages::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        get_image_dimensions(file_bytes(&output.files[0].data))
    }

    #[test]
    fn test_get_enlarge_default_true() {
        assert!(ResizeImages::get_enlarge(&serde_json::Map::new()));
    }

    #[test]
    fn test_get_enlarge_explicit_false() {
        assert!(!ResizeImages::get_enlarge(&with_enlarge(
            width_params(800),
            false
        )));
    }

    #[test]
    fn test_metadata_declares_enlarge() {
        let meta = ResizeImages::new().metadata();
        let enlarge = meta
            .parameters
            .iter()
            .find(|p| p.name == "enlarge")
            .unwrap();
        assert_eq!(enlarge.label, "Enlarge Smaller Images");
        assert!(matches!(
            enlarge.param_type,
            bnto_core::metadata::ParameterType::Boolean
        ));
        assert_eq!(enlarge.default, Some(serde_json::json!(true)));
        assert_eq!(enlarge.group.as_deref(), Some("dimensions"));
    }

    #[test]
    fn test_resize_no_enlarge_keeps_400px_image() {
        let params = with_enlarge(width_params(800), false);
        assert_eq!(resized_dimensions(TEST_MEDIUM_JPEG, params), (400, 400));
    }

    #[test]
    fn test_resize_enlarge_scales_400px_image_up() {
        let params = with_enlarge(width_params(800), true);
        assert_eq!(resized_dimensions(TEST_MEDIUM_JPEG, params), (800, 800));
    }

    #[test]
    fn test_resize_no_enlarge_still_shrinks_large_image() {
        // 1200×800 at width 800 → 800×533, whatever enlarge says.
        for enlarge in [true, false] {
            let params = with_enlarge(width_params(800), enlarge);
            assert_eq!(resized_dimensions(TEST_LARGE_JPEG, params), (800, 533));
        }
    }

    #[test]
    fn test_resize_no_enlarge_height_only() {
        let small = with_enlarge(height_params(600), false);
        assert_eq!(resized_dimensions(TEST_MEDIUM_JPEG, small), (400, 400));
        let large = with_enlarge(height_params(400), false);
        assert_eq!(resized_dimensions(TEST_LARGE_JPEG, large), (600, 400));
    }

    #[test]
    fn test_resize_no_enlarge_metadata_reports_kept_size() {
        let input = make_input(
            TEST_MEDIUM_JPEG,
            "photo.jpg",
            with_enlarge(width_params(800), false),
        );
        let output = ResizeImages::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();
        assert_eq!(output.metadata["newWidth"], 400);
        assert_eq!(output.metadata["newHeight"], 400);
    }

    #[test]
    fn test_resize_no_enlarge_uses_oriented_dimensions() {
        // Stored 60×40, orientation 6 (rotate 90°) → 40×60 as shown. Width 100 keeps 40×60.
        let exif_jpeg = inject_exif_orientation(&create_test_jpeg(60, 40), 6);
        let params = with_enlarge(width_params(100), false);
        assert_eq!(resized_dimensions(&exif_jpeg, params), (40, 60));
    }
}
