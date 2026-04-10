// Strip EXIF Metadata — remove all EXIF data from images.
//
// Strategy: decode → re-encode. The `image` crate strips all EXIF metadata
// during re-encoding, so the decode/re-encode cycle is sufficient.
// Orientation is applied before stripping so the output is visually correct.

use bnto_core::DEFAULT_QUALITY;
use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use bnto_encode::ImageFormat;

use crate::common::{image_accepts, quality_param_def};
use crate::encode;
use crate::orientation::decode_with_orientation;

const MIN_QUALITY: u8 = 1;
const MAX_QUALITY: u8 = 100;

/// The image-strip-exif node processor.
pub struct StripExif;

impl Default for StripExif {
    fn default() -> Self {
        Self::new()
    }
}

impl StripExif {
    pub fn new() -> Self {
        Self
    }

    fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(MIN_QUALITY, MAX_QUALITY)
    }

    /// Generate output filename: "photo.jpg" → "photo-stripped.jpg"
    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        if let Some(dot_pos) = input_filename.rfind('.') {
            let stem = &input_filename[..dot_pos];
            format!("{stem}-stripped.{}", format.extension())
        } else {
            format!("{input_filename}-stripped.{}", format.extension())
        }
    }
}

// --- NodeProcessor Trait Implementation ---

impl NodeProcessor for StripExif {
    fn name(&self) -> &str {
        "image-strip-exif"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "image-strip-exif".to_string(),
            name: "Strip EXIF".to_string(),
            description: "Remove all EXIF metadata from images (GPS, camera info, timestamps)"
                .to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: vec![quality_param_def()],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let format = ImageFormat::detect(&input.data, &input.filename).ok_or_else(|| {
            BntoError::UnsupportedFormat(format!(
                "Could not determine image format for '{}'",
                input.filename
            ))
        })?;

        let original_size = input.data.len();
        let quality = Self::get_quality(&input.params);

        progress.report(10, "Decoding image...");
        let img = decode_with_orientation(&input.data)?;

        progress.report(50, "Re-encoding without EXIF...");
        let stripped_data = encode::encode_image(&img, format, quality)?;

        let stripped_size = stripped_data.len();
        let output_filename = Self::output_filename(&input.filename, format);

        progress.report(100, "EXIF stripping complete");

        let mut metadata = serde_json::Map::new();
        metadata.insert("originalSize".to_string(), serde_json::json!(original_size));
        metadata.insert("strippedSize".to_string(), serde_json::json!(stripped_size));
        metadata.insert("format".to_string(), serde_json::json!(format.extension()));

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: stripped_data,
                filename: output_filename,
                mime_type: format.mime_type().to_string(),
            }],
            metadata,
        })
    }

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

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};
    use bnto_core::NoopContext;

    fn make_input(data: Vec<u8>, filename: &str) -> NodeInput {
        NodeInput {
            data,
            filename: filename.to_string(),
            mime_type: Some("image/jpeg".to_string()),
            params: serde_json::Map::new(),
        }
    }

    fn make_input_with_quality(data: Vec<u8>, filename: &str, quality: u8) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::json!(quality));
        NodeInput {
            data,
            filename: filename.to_string(),
            mime_type: Some("image/jpeg".to_string()),
            params,
        }
    }

    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

    // --- Basic functionality ---

    #[test]
    fn test_strip_exif_name() {
        let processor = StripExif::new();
        assert_eq!(processor.name(), "image-strip-exif");
    }

    #[test]
    fn test_strip_exif_metadata() {
        let processor = StripExif::new();
        let meta = processor.metadata();
        assert_eq!(meta.node_type, "image-strip-exif");
        assert_eq!(meta.category, bnto_core::metadata::NodeCategory::Image);
        assert!(!meta.accepts.is_empty());
        assert_eq!(meta.parameters.len(), 1);
        assert_eq!(meta.parameters[0].name, "quality");
    }

    // --- JPEG processing ---

    #[test]
    fn test_strip_exif_from_jpeg() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = StripExif::new();
        let input = make_input(exif_jpeg, "photo.jpg");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "photo-stripped.jpg");
        assert_eq!(output.files[0].mime_type, "image/jpeg");
        // Output should be a valid JPEG (magic bytes FF D8)
        assert_eq!(output.files[0].data[0], 0xFF);
        assert_eq!(output.files[0].data[1], 0xD8);
    }

    #[test]
    fn test_stripped_jpeg_has_no_exif() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = StripExif::new();
        let input = make_input(exif_jpeg, "photo.jpg");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        // The re-encoded JPEG should NOT contain an APP1 (EXIF) marker.
        // APP1 marker = FF E1. Scan the output for it after the SOI (FF D8).
        let data = &output.files[0].data;
        let has_exif = data
            .windows(2)
            .skip(1) // skip SOI
            .any(|w| w[0] == 0xFF && w[1] == 0xE1);
        assert!(
            !has_exif,
            "Stripped JPEG should not contain APP1 (EXIF) marker"
        );
    }

    #[test]
    fn test_orientation_applied_before_strip() {
        // A 60×40 image with orientation=6 (90° CW) should become 40×60 after strip.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let processor = StripExif::new();
        let input = make_input(exif_jpeg, "portrait.jpg");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        // Decode the output to verify orientation was applied
        let result_img = image::load_from_memory(&output.files[0].data).unwrap();
        assert_eq!(
            result_img.width(),
            40,
            "Orientation should have been applied"
        );
        assert_eq!(result_img.height(), 60);
    }

    // --- PNG processing ---

    #[test]
    fn test_strip_exif_from_png() {
        let png_data = include_bytes!("../../../../test-fixtures/images/small.png");

        let processor = StripExif::new();
        let input = make_input(png_data.to_vec(), "screenshot.png");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "screenshot-stripped.png");
        assert_eq!(output.files[0].mime_type, "image/png");
        // Valid PNG magic bytes
        assert_eq!(&output.files[0].data[..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    // --- WebP processing ---

    #[test]
    fn test_strip_exif_from_webp() {
        let webp_data = include_bytes!("../../../../test-fixtures/images/small.webp");

        let processor = StripExif::new();
        let input = make_input(webp_data.to_vec(), "image.webp");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "image-stripped.webp");
        assert_eq!(output.files[0].mime_type, "image/webp");
        assert_eq!(&output.files[0].data[..4], b"RIFF");
    }

    // --- Quality parameter ---

    #[test]
    fn test_quality_affects_output_size() {
        let jpeg = create_test_jpeg(100, 100);
        let processor = StripExif::new();

        let low = processor
            .process(
                make_input_with_quality(jpeg.clone(), "test.jpg", 20),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();
        let high = processor
            .process(
                make_input_with_quality(jpeg, "test.jpg", 95),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();

        assert!(
            low.files[0].data.len() < high.files[0].data.len(),
            "Low quality ({}) should produce smaller output than high quality ({})",
            low.files[0].data.len(),
            high.files[0].data.len()
        );
    }

    // --- Metadata output ---

    #[test]
    fn test_metadata_contains_sizes() {
        let jpeg = create_test_jpeg(60, 40);
        let processor = StripExif::new();
        let input = make_input(jpeg, "photo.jpg");
        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert!(output.metadata.contains_key("originalSize"));
        assert!(output.metadata.contains_key("strippedSize"));
        assert!(output.metadata.contains_key("format"));
        assert_eq!(output.metadata["format"], "jpg");
    }

    // --- Output filename ---

    #[test]
    fn test_output_filename_with_extension() {
        assert_eq!(
            StripExif::output_filename("photo.jpg", ImageFormat::Jpeg),
            "photo-stripped.jpg"
        );
        assert_eq!(
            StripExif::output_filename("image.png", ImageFormat::Png),
            "image-stripped.png"
        );
    }

    #[test]
    fn test_output_filename_without_extension() {
        assert_eq!(
            StripExif::output_filename("photo", ImageFormat::Jpeg),
            "photo-stripped.jpg"
        );
    }

    // --- Validation ---

    #[test]
    fn test_validate_accepts_valid_quality() {
        let processor = StripExif::new();
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::json!(80));
        assert!(processor.validate(&params).is_empty());
    }

    #[test]
    fn test_validate_rejects_out_of_range_quality() {
        let processor = StripExif::new();
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::json!(200));
        assert!(!processor.validate(&params).is_empty());
    }

    #[test]
    fn test_validate_rejects_non_numeric_quality() {
        let processor = StripExif::new();
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::json!("high"));
        assert!(!processor.validate(&params).is_empty());
    }

    #[test]
    fn test_validate_accepts_empty_params() {
        let processor = StripExif::new();
        let params = serde_json::Map::new();
        assert!(processor.validate(&params).is_empty());
    }

    // --- Error handling ---

    #[test]
    fn test_rejects_unsupported_format() {
        let processor = StripExif::new();
        let input = make_input(b"not an image".to_vec(), "file.bmp");
        let result = processor.process(input, &noop_progress(), &NoopContext);
        assert!(result.is_err());
    }
}
