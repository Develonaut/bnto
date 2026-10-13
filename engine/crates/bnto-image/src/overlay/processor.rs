// OverlayImage — NodeProcessor wiring for `image-overlay`.
//
// Decodes source + overlay, delegates compositing, and encodes the result
// in the source format.

use image::DynamicImage;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use bnto_encode::ImageFormat;

use super::composite::composite_overlay;
use super::param_defs::build_overlay_parameters;
use super::params::{OverlaySettings, get_overlay_bytes};
use super::validate::validate_params;
use crate::common::image_accepts;
use crate::encode;
use crate::orientation::decode_with_orientation;

pub struct OverlayImage;

impl Default for OverlayImage {
    fn default() -> Self {
        Self::new()
    }
}

impl OverlayImage {
    pub fn new() -> Self {
        Self
    }

    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        let ext = format.extension();
        if let Some(stem) = input_filename.rsplit_once('.') {
            format!("{}-overlay.{}", stem.0, ext)
        } else {
            format!("{}-overlay.{}", input_filename, ext)
        }
    }
}

impl NodeProcessor for OverlayImage {
    fn name(&self) -> &str {
        "image-overlay"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "image-overlay".to_string(),
            name: "Overlay Image".to_string(),
            description:
                "Overlay an image onto source images at a configurable position, size, and opacity."
                    .to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            input_cardinality: InputCardinality::PerFile,
            parameters: build_overlay_parameters(),
            requires: vec![],
        }
    }

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
        progress.report(0, "Decoding source image...");
        let (format, source) = decode_source(&data, &input.filename)?;

        progress.report(20, "Decoding overlay...");
        let overlay = decode_overlay(&input.params)?;
        let settings = OverlaySettings::from_params(&input.params);
        let (composited, wm_dims) = composite_overlay(&source, &overlay, &settings, progress);

        progress.report(80, "Encoding output...");
        let output_data = encode::encode_image(&composited, format, settings.quality)?;
        let metadata = overlay_metadata(data.len(), output_data.len(), settings.position, wm_dims);
        let output_filename = Self::output_filename(&input.filename, format);

        progress.report(100, "Done");
        Ok(build_overlay_output(
            output_data,
            output_filename,
            format,
            metadata,
        ))
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        validate_params(params)
    }
}

/// Detect the source format and decode it with EXIF orientation applied.
fn decode_source(data: &[u8], filename: &str) -> Result<(ImageFormat, DynamicImage), BntoError> {
    let format = ImageFormat::detect(data, filename)
        .ok_or_else(|| BntoError::UnsupportedFormat(filename.to_string()))?;
    let source = decode_with_orientation(data)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode image: {e}")))?;
    Ok((format, source))
}

fn decode_overlay(
    params: &serde_json::Map<String, serde_json::Value>,
) -> Result<DynamicImage, BntoError> {
    let wm_bytes = get_overlay_bytes(params)?;
    image::load_from_memory(&wm_bytes)
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode overlay image: {e}")))
}

fn build_overlay_output(
    data: Vec<u8>,
    filename: String,
    format: ImageFormat,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            filename,
            data: FileData::Bytes(data),
            mime_type: format.mime_type().to_string(),
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

fn overlay_metadata(
    original_size: usize,
    output_size: usize,
    position: &str,
    (wm_w, wm_h): (u32, u32),
) -> serde_json::Map<String, serde_json::Value> {
    serde_json::Map::from_iter([
        (
            "originalSize".to_string(),
            serde_json::Value::Number((original_size as u64).into()),
        ),
        (
            "outputSize".to_string(),
            serde_json::Value::Number((output_size as u64).into()),
        ),
        (
            "position".to_string(),
            serde_json::Value::String(position.to_string()),
        ),
        (
            "overlaySize".to_string(),
            serde_json::Value::String(format!("{}x{}", wm_w, wm_h)),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use bnto_core::NoopContext;
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;

    /// Extract bytes from FileData. Panics on FileData::Path (never produced by image processors).
    fn file_bytes(data: &FileData) -> &[u8] {
        match data {
            FileData::Bytes(b) => b,
            FileData::Path(_) => panic!("Expected FileData::Bytes"),
        }
    }

    fn noop_progress() -> ProgressReporter {
        ProgressReporter::new_noop()
    }

    /// Create a test JPEG image with the given dimensions.
    fn create_test_jpeg(width: u32, height: u32) -> Vec<u8> {
        let mut img = RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba([(x * 3) as u8, (y * 5) as u8, 128, 255]));
            }
        }
        let dynamic = DynamicImage::ImageRgba8(img);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        dynamic
            .write_to(&mut cursor, image::ImageFormat::Jpeg)
            .unwrap();
        buf
    }

    /// Create a test PNG overlay image (transparent background, white circle).
    fn create_test_overlay(width: u32, height: u32) -> Vec<u8> {
        let mut img = RgbaImage::new(width, height);
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let r = (width.min(height) as f64 / 2.0) - 2.0;
        for y in 0..height {
            for x in 0..width {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                if dx * dx + dy * dy <= r * r {
                    img.put_pixel(x, y, Rgba([255, 255, 255, 200]));
                } else {
                    img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
        }
        let dynamic = DynamicImage::ImageRgba8(img);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        dynamic
            .write_to(&mut cursor, image::ImageFormat::Png)
            .unwrap();
        buf
    }

    fn overlay_base64() -> String {
        let bytes = create_test_overlay(50, 50);
        BASE64.encode(&bytes)
    }

    fn make_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(data.to_vec()),
            filename: filename.to_string(),
            mime_type: Some("image/jpeg".to_string()),
            params: serde_json::Map::from_iter([(
                "overlay".to_string(),
                serde_json::Value::String(overlay_base64()),
            )]),
        }
    }

    fn make_input_with_params(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(data.to_vec()),
            filename: filename.to_string(),
            mime_type: Some("image/jpeg".to_string()),
            params,
        }
    }

    // =========================================================================
    // Trait basics
    // =========================================================================

    #[test]
    fn test_name() {
        let processor = OverlayImage::new();
        assert_eq!(processor.name(), "image-overlay");
    }

    #[test]
    fn test_validate_with_overlay() {
        let processor = OverlayImage::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "overlay".to_string(),
            serde_json::Value::String(overlay_base64()),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Should validate with overlay: {errors:?}"
        );
    }

    #[test]
    fn test_validate_missing_overlay() {
        let processor = OverlayImage::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("overlay"));
    }

    // =========================================================================
    // Happy path
    // =========================================================================

    #[test]
    fn test_overlay_happy_path() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let input = make_input(&source, "photo.jpg");

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .expect("Should process successfully");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Jpeg),
        );
        assert!(output.files[0].filename.contains("-overlay"));
    }

    // =========================================================================
    // Position affects output
    // =========================================================================

    #[test]
    fn test_position_affects_output() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let wm = overlay_base64();

        let mut params_tl = serde_json::Map::new();
        params_tl.insert("overlay".to_string(), serde_json::Value::String(wm.clone()));
        params_tl.insert(
            "position".to_string(),
            serde_json::Value::String("top-left".to_string()),
        );

        let mut params_br = serde_json::Map::new();
        params_br.insert("overlay".to_string(), serde_json::Value::String(wm));
        params_br.insert(
            "position".to_string(),
            serde_json::Value::String("bottom-right".to_string()),
        );

        let out_tl = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_tl),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();
        let out_br = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_br),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();

        assert_ne!(
            file_bytes(&out_tl.files[0].data),
            file_bytes(&out_br.files[0].data),
            "Different positions should produce different outputs"
        );
    }

    // =========================================================================
    // Opacity affects output
    // =========================================================================

    #[test]
    fn test_opacity_affects_output() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let wm = overlay_base64();

        let mut params_full = serde_json::Map::new();
        params_full.insert("overlay".to_string(), serde_json::Value::String(wm.clone()));
        params_full.insert(
            "opacity".to_string(),
            serde_json::Value::Number(100u64.into()),
        );

        let mut params_half = serde_json::Map::new();
        params_half.insert("overlay".to_string(), serde_json::Value::String(wm));
        params_half.insert(
            "opacity".to_string(),
            serde_json::Value::Number(50u64.into()),
        );

        let out_full = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_full),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();
        let out_half = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_half),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();

        assert_ne!(
            file_bytes(&out_full.files[0].data),
            file_bytes(&out_half.files[0].data),
            "Different opacity should produce different outputs"
        );
    }

    // =========================================================================
    // Size affects output
    // =========================================================================

    #[test]
    fn test_size_affects_output() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let wm = overlay_base64();

        let mut params_small = serde_json::Map::new();
        params_small.insert("overlay".to_string(), serde_json::Value::String(wm.clone()));
        params_small.insert("size".to_string(), serde_json::Value::Number(10u64.into()));

        let mut params_large = serde_json::Map::new();
        params_large.insert("overlay".to_string(), serde_json::Value::String(wm));
        params_large.insert("size".to_string(), serde_json::Value::Number(50u64.into()));

        let out_small = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_small),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();
        let out_large = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_large),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();

        assert_ne!(
            file_bytes(&out_small.files[0].data),
            file_bytes(&out_large.files[0].data),
            "Different sizes should produce different outputs"
        );
    }

    // =========================================================================
    // Offset affects output
    // =========================================================================

    #[test]
    fn test_offset_affects_output() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let wm = overlay_base64();

        let mut params_a = serde_json::Map::new();
        params_a.insert("overlay".to_string(), serde_json::Value::String(wm.clone()));
        params_a.insert(
            "offsetX".to_string(),
            serde_json::Value::Number(5u64.into()),
        );

        let mut params_b = serde_json::Map::new();
        params_b.insert("overlay".to_string(), serde_json::Value::String(wm));
        params_b.insert(
            "offsetX".to_string(),
            serde_json::Value::Number(50u64.into()),
        );

        let out_a = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_a),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();
        let out_b = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_b),
                &noop_progress(),
                &NoopContext,
            )
            .unwrap();

        assert_ne!(
            file_bytes(&out_a.files[0].data),
            file_bytes(&out_b.files[0].data),
            "Different offsets should produce different outputs"
        );
    }

    // =========================================================================
    // Invalid overlay base64
    // =========================================================================

    #[test]
    fn test_invalid_overlay_base64() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let mut params = serde_json::Map::new();
        params.insert(
            "overlay".to_string(),
            serde_json::Value::String("not-valid-base64!!!".to_string()),
        );

        let result = processor.process(
            make_input_with_params(&source, "photo.jpg", params),
            &noop_progress(),
            &NoopContext,
        );

        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("Should fail with invalid base64"),
        };
        let msg = err.to_string();
        assert!(msg.contains("base64") || msg.contains("overlay"));
    }

    // =========================================================================
    // Output filename
    // =========================================================================

    #[test]
    fn test_output_filename() {
        assert_eq!(
            OverlayImage::output_filename("photo.jpg", ImageFormat::Jpeg),
            "photo-overlay.jpg"
        );
        assert_eq!(
            OverlayImage::output_filename("image.png", ImageFormat::Png),
            "image-overlay.png"
        );
    }

    // =========================================================================
    // Output metadata
    // =========================================================================

    #[test]
    fn test_output_metadata() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let input = make_input(&source, "photo.jpg");

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();

        assert!(output.metadata.contains_key("originalSize"));
        assert!(output.metadata.contains_key("outputSize"));
        assert!(output.metadata.contains_key("position"));
        assert!(output.metadata.contains_key("overlaySize"));
    }

    // =========================================================================
    // All formats work
    // =========================================================================

    #[test]
    fn test_png_source() {
        let processor = OverlayImage::new();
        let mut img = RgbaImage::new(100, 100);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 150, 200, 255]);
        }
        let dynamic = DynamicImage::ImageRgba8(img);
        let mut buf = Vec::new();
        dynamic
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();

        let mut input = make_input(&buf, "test.png");
        input.mime_type = Some("image/png".to_string());

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::Png),
        );
    }

    #[test]
    fn test_webp_source() {
        let processor = OverlayImage::new();
        let mut img = RgbaImage::new(100, 100);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 150, 200, 255]);
        }
        let dynamic = DynamicImage::ImageRgba8(img);
        let mut buf = Vec::new();
        dynamic
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::WebP)
            .unwrap();

        let mut input = make_input(&buf, "test.webp");
        input.mime_type = Some("image/webp".to_string());

        let output = processor
            .process(input, &noop_progress(), &NoopContext)
            .unwrap();
        assert_eq!(
            ImageFormat::from_magic_bytes(file_bytes(&output.files[0].data)),
            Some(ImageFormat::WebP),
        );
    }

    // =========================================================================
    // Data URI prefix handling
    // =========================================================================

    #[test]
    fn test_data_uri_prefix() {
        let processor = OverlayImage::new();
        let source = create_test_jpeg(200, 150);
        let b64 = overlay_base64();
        let data_uri = format!("data:image/png;base64,{b64}");

        let mut params = serde_json::Map::new();
        params.insert("overlay".to_string(), serde_json::Value::String(data_uri));

        let input = make_input_with_params(&source, "photo.jpg", params);
        let output = processor.process(input, &noop_progress(), &NoopContext);
        assert!(output.is_ok(), "Should handle data URI prefix");
    }

    // =========================================================================
    // Validate position enum
    // =========================================================================

    #[test]
    fn test_validate_invalid_position() {
        let processor = OverlayImage::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "overlay".to_string(),
            serde_json::Value::String(overlay_base64()),
        );
        params.insert(
            "position".to_string(),
            serde_json::Value::String("invalid-pos".to_string()),
        );

        let errors = processor.validate(&params);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("position"));
    }
}
