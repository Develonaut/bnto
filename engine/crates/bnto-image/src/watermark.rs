// Watermark Images Node — overlay a watermark image onto source images.
//
// Decodes a base64-encoded watermark from params, resizes it relative to
// the source image width, applies opacity, and composites at the chosen
// position with configurable pixel offsets.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use image::imageops::{self, FilterType};
use image::{DynamicImage, GenericImageView, RgbaImage};

use bnto_core::DEFAULT_QUALITY;
use bnto_core::errors::BntoError;
use bnto_core::metadata::{
    Constraints, InputCardinality, NodeCategory, NodeMetadata, ParameterDef, ParameterType,
};
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use crate::common::image_accepts;
use crate::encode;
use crate::format::ImageFormat;
use crate::orientation::decode_with_orientation;

const DEFAULT_SIZE: u64 = 25;
const DEFAULT_OPACITY: u64 = 80;
const DEFAULT_OFFSET_X: u64 = 10;
const DEFAULT_OFFSET_Y: u64 = 10;
const DEFAULT_POSITION: &str = "bottom-right";

const POSITIONS: &[&str] = &[
    "top-left",
    "top-center",
    "top-right",
    "middle-left",
    "center",
    "middle-right",
    "bottom-left",
    "bottom-center",
    "bottom-right",
];

pub struct WatermarkImages;

impl Default for WatermarkImages {
    fn default() -> Self {
        Self::new()
    }
}

impl WatermarkImages {
    pub fn new() -> Self {
        Self
    }

    fn get_watermark_bytes(
        params: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Vec<u8>, BntoError> {
        let b64 = params
            .get("watermark")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BntoError::InvalidInput("watermark parameter is required".to_string())
            })?;

        // Strip optional data URI prefix (e.g., "data:image/png;base64,")
        let raw = if let Some(idx) = b64.find(",") {
            &b64[idx + 1..]
        } else {
            b64
        };

        BASE64.decode(raw).map_err(|e| {
            BntoError::InvalidInput(format!("Invalid base64 in watermark parameter: {e}"))
        })
    }

    fn get_position(params: &serde_json::Map<String, serde_json::Value>) -> &str {
        params
            .get("position")
            .and_then(|v| v.as_str())
            .unwrap_or(DEFAULT_POSITION)
    }

    fn get_size(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
        params
            .get("size")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_SIZE)
            .clamp(1, 100)
    }

    fn get_opacity(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
        params
            .get("opacity")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_OPACITY)
            .clamp(0, 100)
    }

    fn get_offset_x(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
        params
            .get("offsetX")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_OFFSET_X)
            .clamp(0, 500)
    }

    fn get_offset_y(params: &serde_json::Map<String, serde_json::Value>) -> u64 {
        params
            .get("offsetY")
            .and_then(|v| v.as_u64())
            .unwrap_or(DEFAULT_OFFSET_Y)
            .clamp(0, 500)
    }

    fn get_quality(params: &serde_json::Map<String, serde_json::Value>) -> u8 {
        params
            .get("quality")
            .and_then(|v| v.as_u64())
            .map(|q| q as u8)
            .unwrap_or(DEFAULT_QUALITY)
            .clamp(1, 100)
    }

    /// Resize watermark to target width (% of source), maintaining aspect ratio.
    fn resize_watermark(watermark: &DynamicImage, target_width: u32) -> DynamicImage {
        let (wm_w, wm_h) = watermark.dimensions();
        if wm_w == 0 {
            return watermark.clone();
        }
        let aspect = wm_h as f64 / wm_w as f64;
        let target_height = (target_width as f64 * aspect).round().max(1.0) as u32;
        watermark.resize_exact(target_width, target_height, FilterType::Lanczos3)
    }

    /// Apply opacity by multiplying each pixel's alpha channel.
    fn apply_opacity(img: &DynamicImage, opacity: u64) -> RgbaImage {
        let rgba = img.to_rgba8();
        let factor = opacity as f64 / 100.0;
        let mut result = rgba.clone();
        for pixel in result.pixels_mut() {
            pixel[3] = (pixel[3] as f64 * factor).round().clamp(0.0, 255.0) as u8;
        }
        result
    }

    /// Calculate top-left (x, y) for placing the watermark on the source image.
    fn calculate_position(
        position: &str,
        src_w: u32,
        src_h: u32,
        wm_w: u32,
        wm_h: u32,
        offset_x: u64,
        offset_y: u64,
    ) -> (i64, i64) {
        let ox = offset_x as i64;
        let oy = offset_y as i64;
        let sw = src_w as i64;
        let sh = src_h as i64;
        let ww = wm_w as i64;
        let wh = wm_h as i64;

        match position {
            "top-left" => (ox, oy),
            "top-center" => ((sw - ww) / 2, oy),
            "top-right" => (sw - ww - ox, oy),
            "middle-left" => (ox, (sh - wh) / 2),
            "center" => ((sw - ww) / 2, (sh - wh) / 2),
            "middle-right" => (sw - ww - ox, (sh - wh) / 2),
            "bottom-left" => (ox, sh - wh - oy),
            "bottom-center" => ((sw - ww) / 2, sh - wh - oy),
            _ => (sw - ww - ox, sh - wh - oy), // bottom-right default
        }
    }

    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        let ext = format.extension();
        if let Some(stem) = input_filename.rsplit_once('.') {
            format!("{}-watermarked.{}", stem.0, ext)
        } else {
            format!("{}-watermarked.{}", input_filename, ext)
        }
    }
}

impl NodeProcessor for WatermarkImages {
    fn name(&self) -> &str {
        "image-watermark"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "image-watermark".to_string(),
            name: "Watermark Images".to_string(),
            description: "Overlay a watermark image onto source images.".to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            input_cardinality: InputCardinality::PerFile,
            parameters: vec![
                ParameterDef {
                    name: "watermark".to_string(),
                    label: "Watermark Image".to_string(),
                    description: "The watermark image to overlay (base64-encoded).".to_string(),
                    param_type: ParameterType::File {
                        accept: vec![
                            "image/png".to_string(),
                            "image/jpeg".to_string(),
                            "image/webp".to_string(),
                        ],
                    },
                    constraints: Some(Constraints {
                        min: None,
                        max: None,
                        required: true,
                    }),
                    ..Default::default()
                },
                ParameterDef {
                    name: "position".to_string(),
                    label: "Position".to_string(),
                    description: "Where to place the watermark on the image.".to_string(),
                    param_type: ParameterType::Enum {
                        options: POSITIONS.iter().map(|s| s.to_string()).collect(),
                    },
                    default: Some(serde_json::Value::String(DEFAULT_POSITION.to_string())),
                    ..Default::default()
                },
                ParameterDef {
                    name: "size".to_string(),
                    label: "Size".to_string(),
                    description: "Watermark width as a percentage of the source image width."
                        .to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::Value::Number(DEFAULT_SIZE.into())),
                    constraints: Some(Constraints {
                        min: Some(1.0),
                        max: Some(100.0),
                        required: false,
                    }),
                    ..Default::default()
                },
                ParameterDef {
                    name: "opacity".to_string(),
                    label: "Opacity".to_string(),
                    description: "Watermark transparency (0 = invisible, 100 = fully opaque)."
                        .to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::Value::Number(DEFAULT_OPACITY.into())),
                    constraints: Some(Constraints {
                        min: Some(0.0),
                        max: Some(100.0),
                        required: false,
                    }),
                    ..Default::default()
                },
                ParameterDef {
                    name: "offsetX".to_string(),
                    label: "Offset X".to_string(),
                    description: "Horizontal offset from the edge in pixels.".to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::Value::Number(DEFAULT_OFFSET_X.into())),
                    constraints: Some(Constraints {
                        min: Some(0.0),
                        max: Some(500.0),
                        required: false,
                    }),
                    ..Default::default()
                },
                ParameterDef {
                    name: "offsetY".to_string(),
                    label: "Offset Y".to_string(),
                    description: "Vertical offset from the edge in pixels.".to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::Value::Number(DEFAULT_OFFSET_Y.into())),
                    constraints: Some(Constraints {
                        min: Some(0.0),
                        max: Some(500.0),
                        required: false,
                    }),
                    ..Default::default()
                },
                crate::common::quality_param_def(),
            ],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Decoding source image...");

        let format = ImageFormat::detect(&input.data, &input.filename)
            .ok_or_else(|| BntoError::UnsupportedFormat(input.filename.clone()))?;

        let source = decode_with_orientation(&input.data)
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode image: {e}")))?;

        let (src_w, src_h) = source.dimensions();

        progress.report(20, "Decoding watermark...");

        let wm_bytes = Self::get_watermark_bytes(&input.params)?;
        let watermark = image::load_from_memory(&wm_bytes).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to decode watermark image: {e}"))
        })?;

        let size = Self::get_size(&input.params);
        let opacity = Self::get_opacity(&input.params);
        let position = Self::get_position(&input.params);
        let offset_x = Self::get_offset_x(&input.params);
        let offset_y = Self::get_offset_y(&input.params);
        let quality = Self::get_quality(&input.params);

        progress.report(40, "Resizing watermark...");

        let target_wm_width = ((src_w as f64 * size as f64) / 100.0).round().max(1.0) as u32;
        let resized_wm = Self::resize_watermark(&watermark, target_wm_width);
        let (wm_w, wm_h) = resized_wm.dimensions();

        progress.report(50, "Applying opacity...");

        let wm_rgba = Self::apply_opacity(&resized_wm, opacity);

        progress.report(60, "Compositing watermark...");

        let (pos_x, pos_y) =
            Self::calculate_position(position, src_w, src_h, wm_w, wm_h, offset_x, offset_y);

        let mut canvas = source.to_rgba8();
        imageops::overlay(&mut canvas, &wm_rgba, pos_x, pos_y);
        let composited = DynamicImage::ImageRgba8(canvas);

        progress.report(80, "Encoding output...");

        let output_data = encode::encode_image(&composited, format, quality)?;
        let original_size = input.data.len();
        let output_size = output_data.len();

        let output_filename = Self::output_filename(&input.filename, format);

        progress.report(100, "Done");

        Ok(NodeOutput {
            files: vec![OutputFile {
                filename: output_filename,
                data: output_data,
                mime_type: format.mime_type().to_string(),
            }],
            metadata: serde_json::Map::from_iter([
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
                    "watermarkSize".to_string(),
                    serde_json::Value::String(format!("{}x{}", wm_w, wm_h)),
                ),
            ]),
        })
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        // Watermark is required
        if params.get("watermark").and_then(|v| v.as_str()).is_none() {
            errors.push("watermark is required".to_string());
        }

        // Validate position enum
        if let Some(pos) = params.get("position").and_then(|v| v.as_str())
            && !POSITIONS.contains(&pos)
        {
            errors.push(format!("position must be one of: {}", POSITIONS.join(", ")));
        }

        // Validate numeric ranges
        if let Some(size) = params.get("size").and_then(|v| v.as_u64())
            && !(1..=100).contains(&size)
        {
            errors.push("size must be between 1 and 100".to_string());
        }

        if let Some(opacity) = params.get("opacity").and_then(|v| v.as_u64())
            && opacity > 100
        {
            errors.push("opacity must be between 0 and 100".to_string());
        }

        if let Some(quality) = params.get("quality").and_then(|v| v.as_u64())
            && !(1..=100).contains(&quality)
        {
            errors.push("quality must be between 1 and 100".to_string());
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;

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

    /// Create a test PNG watermark image (transparent background, white circle).
    fn create_test_watermark(width: u32, height: u32) -> Vec<u8> {
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

    fn watermark_base64() -> String {
        let bytes = create_test_watermark(50, 50);
        BASE64.encode(&bytes)
    }

    fn make_input(data: &[u8], filename: &str) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: Some("image/jpeg".to_string()),
            params: serde_json::Map::from_iter([(
                "watermark".to_string(),
                serde_json::Value::String(watermark_base64()),
            )]),
        }
    }

    fn make_input_with_params(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
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
        let processor = WatermarkImages::new();
        assert_eq!(processor.name(), "image-watermark");
    }

    #[test]
    fn test_validate_with_watermark() {
        let processor = WatermarkImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "watermark".to_string(),
            serde_json::Value::String(watermark_base64()),
        );
        let errors = processor.validate(&params);
        assert!(
            errors.is_empty(),
            "Should validate with watermark: {errors:?}"
        );
    }

    #[test]
    fn test_validate_missing_watermark() {
        let processor = WatermarkImages::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("watermark"));
    }

    // =========================================================================
    // Happy path
    // =========================================================================

    #[test]
    fn test_watermark_happy_path() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let input = make_input(&source, "photo.jpg");

        let output = processor
            .process(input, &noop_progress())
            .expect("Should process successfully");

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Jpeg),
        );
        assert!(output.files[0].filename.contains("-watermarked"));
    }

    // =========================================================================
    // Position affects output
    // =========================================================================

    #[test]
    fn test_position_affects_output() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let wm = watermark_base64();

        let mut params_tl = serde_json::Map::new();
        params_tl.insert(
            "watermark".to_string(),
            serde_json::Value::String(wm.clone()),
        );
        params_tl.insert(
            "position".to_string(),
            serde_json::Value::String("top-left".to_string()),
        );

        let mut params_br = serde_json::Map::new();
        params_br.insert("watermark".to_string(), serde_json::Value::String(wm));
        params_br.insert(
            "position".to_string(),
            serde_json::Value::String("bottom-right".to_string()),
        );

        let out_tl = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_tl),
                &noop_progress(),
            )
            .unwrap();
        let out_br = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_br),
                &noop_progress(),
            )
            .unwrap();

        assert_ne!(
            out_tl.files[0].data, out_br.files[0].data,
            "Different positions should produce different outputs"
        );
    }

    // =========================================================================
    // Opacity affects output
    // =========================================================================

    #[test]
    fn test_opacity_affects_output() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let wm = watermark_base64();

        let mut params_full = serde_json::Map::new();
        params_full.insert(
            "watermark".to_string(),
            serde_json::Value::String(wm.clone()),
        );
        params_full.insert(
            "opacity".to_string(),
            serde_json::Value::Number(100u64.into()),
        );

        let mut params_half = serde_json::Map::new();
        params_half.insert("watermark".to_string(), serde_json::Value::String(wm));
        params_half.insert(
            "opacity".to_string(),
            serde_json::Value::Number(50u64.into()),
        );

        let out_full = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_full),
                &noop_progress(),
            )
            .unwrap();
        let out_half = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_half),
                &noop_progress(),
            )
            .unwrap();

        assert_ne!(
            out_full.files[0].data, out_half.files[0].data,
            "Different opacity should produce different outputs"
        );
    }

    // =========================================================================
    // Size affects output
    // =========================================================================

    #[test]
    fn test_size_affects_output() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let wm = watermark_base64();

        let mut params_small = serde_json::Map::new();
        params_small.insert(
            "watermark".to_string(),
            serde_json::Value::String(wm.clone()),
        );
        params_small.insert("size".to_string(), serde_json::Value::Number(10u64.into()));

        let mut params_large = serde_json::Map::new();
        params_large.insert("watermark".to_string(), serde_json::Value::String(wm));
        params_large.insert("size".to_string(), serde_json::Value::Number(50u64.into()));

        let out_small = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_small),
                &noop_progress(),
            )
            .unwrap();
        let out_large = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_large),
                &noop_progress(),
            )
            .unwrap();

        assert_ne!(
            out_small.files[0].data, out_large.files[0].data,
            "Different sizes should produce different outputs"
        );
    }

    // =========================================================================
    // Offset affects output
    // =========================================================================

    #[test]
    fn test_offset_affects_output() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let wm = watermark_base64();

        let mut params_a = serde_json::Map::new();
        params_a.insert(
            "watermark".to_string(),
            serde_json::Value::String(wm.clone()),
        );
        params_a.insert(
            "offsetX".to_string(),
            serde_json::Value::Number(5u64.into()),
        );

        let mut params_b = serde_json::Map::new();
        params_b.insert("watermark".to_string(), serde_json::Value::String(wm));
        params_b.insert(
            "offsetX".to_string(),
            serde_json::Value::Number(50u64.into()),
        );

        let out_a = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_a),
                &noop_progress(),
            )
            .unwrap();
        let out_b = processor
            .process(
                make_input_with_params(&source, "photo.jpg", params_b),
                &noop_progress(),
            )
            .unwrap();

        assert_ne!(
            out_a.files[0].data, out_b.files[0].data,
            "Different offsets should produce different outputs"
        );
    }

    // =========================================================================
    // Invalid watermark base64
    // =========================================================================

    #[test]
    fn test_invalid_watermark_base64() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let mut params = serde_json::Map::new();
        params.insert(
            "watermark".to_string(),
            serde_json::Value::String("not-valid-base64!!!".to_string()),
        );

        let result = processor.process(
            make_input_with_params(&source, "photo.jpg", params),
            &noop_progress(),
        );

        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("Should fail with invalid base64"),
        };
        let msg = err.to_string();
        assert!(msg.contains("base64") || msg.contains("watermark"));
    }

    // =========================================================================
    // Output filename
    // =========================================================================

    #[test]
    fn test_output_filename() {
        assert_eq!(
            WatermarkImages::output_filename("photo.jpg", ImageFormat::Jpeg),
            "photo-watermarked.jpg"
        );
        assert_eq!(
            WatermarkImages::output_filename("image.png", ImageFormat::Png),
            "image-watermarked.png"
        );
    }

    // =========================================================================
    // Output metadata
    // =========================================================================

    #[test]
    fn test_output_metadata() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let input = make_input(&source, "photo.jpg");

        let output = processor.process(input, &noop_progress()).unwrap();

        assert!(output.metadata.contains_key("originalSize"));
        assert!(output.metadata.contains_key("outputSize"));
        assert!(output.metadata.contains_key("position"));
        assert!(output.metadata.contains_key("watermarkSize"));
    }

    // =========================================================================
    // All formats work
    // =========================================================================

    #[test]
    fn test_png_source() {
        let processor = WatermarkImages::new();
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

        let output = processor.process(input, &noop_progress()).unwrap();
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::Png),
        );
    }

    #[test]
    fn test_webp_source() {
        let processor = WatermarkImages::new();
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

        let output = processor.process(input, &noop_progress()).unwrap();
        assert_eq!(
            ImageFormat::from_magic_bytes(&output.files[0].data),
            Some(ImageFormat::WebP),
        );
    }

    // =========================================================================
    // Data URI prefix handling
    // =========================================================================

    #[test]
    fn test_data_uri_prefix() {
        let processor = WatermarkImages::new();
        let source = create_test_jpeg(200, 150);
        let b64 = watermark_base64();
        let data_uri = format!("data:image/png;base64,{b64}");

        let mut params = serde_json::Map::new();
        params.insert("watermark".to_string(), serde_json::Value::String(data_uri));

        let input = make_input_with_params(&source, "photo.jpg", params);
        let output = processor.process(input, &noop_progress());
        assert!(output.is_ok(), "Should handle data URI prefix");
    }

    // =========================================================================
    // Validate position enum
    // =========================================================================

    #[test]
    fn test_validate_invalid_position() {
        let processor = WatermarkImages::new();
        let mut params = serde_json::Map::new();
        params.insert(
            "watermark".to_string(),
            serde_json::Value::String(watermark_base64()),
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
