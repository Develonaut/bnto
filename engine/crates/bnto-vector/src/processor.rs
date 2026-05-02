// Vector Rasterize Node — convert SVG files to raster images (PNG, JPEG, WebP).
//
// Delegates SVG parsing and rendering to rasterize_svg(), then converts the
// pixel buffer to a DynamicImage and encodes via bnto-encode's shared encoder.

use bnto_core::DEFAULT_QUALITY;
use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_encode::ImageFormat;

use crate::common::{format_param_def, quality_param_def, svg_accepts};
use crate::rasterize::{RasterizeOptions, rasterize_svg};

const MIN_QUALITY: u8 = 1;
const MAX_QUALITY: u8 = 100;

/// The vector-rasterize node processor.
pub struct VectorRasterize;

impl Default for VectorRasterize {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorRasterize {
    pub fn new() -> Self {
        Self
    }
}

impl NodeProcessor for VectorRasterize {
    fn name(&self) -> &str {
        "vector-rasterize"
    }

    fn metadata(&self) -> bnto_core::NodeMetadata {
        use bnto_core::metadata::*;
        NodeMetadata {
            node_type: "vector-rasterize".to_string(),
            name: "SVG to Image".to_string(),
            description: "Convert SVG files to raster images (PNG, JPEG, WebP)".to_string(),
            category: NodeCategory::Vector,
            accepts: svg_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: vec![format_param_def(), quality_param_def()],
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
        let target_format = extract_target_format(&input)?;
        let quality = extract_quality(&input.params, target_format);
        let data = input
            .data
            .into_bytes()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read input: {e}")))?;

        progress.report(10, "Rasterizing SVG...");
        let pixmap = rasterize_svg(&data, RasterizeOptions::default())
            .map_err(|e| BntoError::ProcessingFailed(format!("SVG rasterization failed: {e}")))?;

        progress.report(50, "Converting to raster image...");
        let img = pixmap_to_dynamic_image(&pixmap, target_format)?;

        progress.report(
            70,
            &format!("Encoding as {target_format:?} (quality: {quality})..."),
        );
        let encoded = bnto_encode::encode::encode_image(&img, target_format, quality)?;

        let output_name = output_filename(&input.filename, target_format);
        let metadata = build_metadata(&data, &encoded, target_format);

        progress.report(100, "Rasterization complete");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: FileData::Bytes(encoded),
                filename: output_name,
                mime_type: target_format.mime_type().to_string(),
                metadata: serde_json::Map::new(),
            }],
            metadata,
        })
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();
        validate_format_param(params, &mut errors);
        validate_quality_param(params, &mut errors);
        errors
    }
}

// --- Private helpers ---

fn extract_target_format(input: &NodeInput) -> Result<ImageFormat, BntoError> {
    let format_str = input
        .params
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("png");

    parse_target_format(format_str)
}

fn parse_target_format(format_str: &str) -> Result<ImageFormat, BntoError> {
    match format_str.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        "webp" => Ok(ImageFormat::WebP),
        _ => Err(BntoError::InvalidInput(format!(
            "Unsupported target format: '{format_str}'. Supported: png, jpeg, webp"
        ))),
    }
}

fn extract_quality(
    params: &serde_json::Map<String, serde_json::Value>,
    _target_format: ImageFormat,
) -> u8 {
    params
        .get("quality")
        .and_then(|v| v.as_u64())
        .map(|q| q as u8)
        .unwrap_or(DEFAULT_QUALITY)
        .clamp(MIN_QUALITY, MAX_QUALITY)
}

/// Convert a tiny_skia::Pixmap (premultiplied RGBA) to a DynamicImage.
///
/// For formats that don't support transparency (JPEG), composites over a white
/// background so transparent areas appear white instead of black.
fn pixmap_to_dynamic_image(
    pixmap: &tiny_skia::Pixmap,
    target_format: ImageFormat,
) -> Result<image::DynamicImage, BntoError> {
    let mut pixels = pixmap.data().to_vec();
    unpremultiply_alpha(&mut pixels);

    let needs_opaque_bg = matches!(target_format, ImageFormat::Jpeg);
    if needs_opaque_bg {
        composite_over_white(&mut pixels);
    }

    let rgba =
        image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixels).ok_or_else(|| {
            BntoError::ProcessingFailed("Failed to create RgbaImage from pixmap data".to_string())
        })?;

    Ok(image::DynamicImage::ImageRgba8(rgba))
}

/// Blend RGBA pixels over a white background, setting alpha to 255.
/// result_rgb = src_rgb * src_alpha + 255 * (1 - src_alpha)
fn composite_over_white(data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(4) {
        let a = chunk[3] as f32 / 255.0;
        chunk[0] = (chunk[0] as f32 * a + 255.0 * (1.0 - a)) as u8;
        chunk[1] = (chunk[1] as f32 * a + 255.0 * (1.0 - a)) as u8;
        chunk[2] = (chunk[2] as f32 * a + 255.0 * (1.0 - a)) as u8;
        chunk[3] = 255;
    }
}

/// Unpremultiply RGBA pixels — tiny-skia uses premultiplied alpha,
/// but the image crate expects straight (non-premultiplied) alpha.
fn unpremultiply_alpha(data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(4) {
        let alpha = chunk[3];
        if alpha == 0 || alpha == 255 {
            continue;
        }
        let a = alpha as f32 / 255.0;
        chunk[0] = (chunk[0] as f32 / a).min(255.0) as u8;
        chunk[1] = (chunk[1] as f32 / a).min(255.0) as u8;
        chunk[2] = (chunk[2] as f32 / a).min(255.0) as u8;
    }
}

/// Replace extension: "icon.svg" + PNG -> "icon.png"
fn output_filename(input_filename: &str, target_format: ImageFormat) -> String {
    if let Some(dot_pos) = input_filename.rfind('.') {
        let stem = &input_filename[..dot_pos];
        format!("{stem}.{}", target_format.extension())
    } else {
        format!("{input_filename}.{}", target_format.extension())
    }
}

fn build_metadata(
    input_data: &[u8],
    output_data: &[u8],
    target_format: ImageFormat,
) -> serde_json::Map<String, serde_json::Value> {
    let mut meta = serde_json::Map::new();
    meta.insert("originalFormat".to_string(), serde_json::json!("svg"));
    meta.insert(
        "outputFormat".to_string(),
        serde_json::json!(target_format.extension()),
    );
    meta.insert(
        "originalSize".to_string(),
        serde_json::json!(input_data.len()),
    );
    meta.insert(
        "compressedSize".to_string(),
        serde_json::json!(output_data.len()),
    );
    meta
}

fn validate_format_param(
    params: &serde_json::Map<String, serde_json::Value>,
    errors: &mut Vec<String>,
) {
    if let Some(format_val) = params.get("format")
        && let Some(s) = format_val.as_str()
        && parse_target_format(s).is_err()
    {
        errors.push(format!("Invalid format '{s}'. Supported: png, jpeg, webp"));
    }
}

fn validate_quality_param(
    params: &serde_json::Map<String, serde_json::Value>,
    errors: &mut Vec<String>,
) {
    if let Some(q) = params.get("quality") {
        if let Some(n) = q.as_u64()
            && (n < MIN_QUALITY as u64 || n > MAX_QUALITY as u64)
        {
            errors.push(format!(
                "Quality must be between {MIN_QUALITY} and {MAX_QUALITY}, got {n}"
            ));
        } else if let Some(n) = q.as_f64()
            && (n < MIN_QUALITY as f64 || n > MAX_QUALITY as f64)
        {
            errors.push(format!(
                "Quality must be between {MIN_QUALITY} and {MAX_QUALITY}, got {n}"
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;

    fn fixture_svg() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../test-fixtures/images/small.svg"
        ))
        .expect("small.svg fixture must exist")
    }

    fn make_input(params: serde_json::Map<String, serde_json::Value>) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(fixture_svg()),
            filename: "icon.svg".to_string(),
            mime_type: Some("image/svg+xml".to_string()),
            params,
        }
    }

    fn make_params(format: &str, quality: u64) -> serde_json::Map<String, serde_json::Value> {
        let mut params = serde_json::Map::new();
        params.insert("format".to_string(), serde_json::json!(format));
        params.insert("quality".to_string(), serde_json::json!(quality));
        params
    }

    // --- Trait basics ---

    #[test]
    fn test_name_returns_vector_rasterize() {
        assert_eq!(VectorRasterize::new().name(), "vector-rasterize");
    }

    #[test]
    fn test_metadata_has_svg_accepts() {
        let meta = VectorRasterize::new().metadata();
        assert!(meta.accepts.contains(&"image/svg+xml".to_string()));
    }

    #[test]
    fn test_metadata_category_is_vector() {
        let meta = VectorRasterize::new().metadata();
        assert!(matches!(
            meta.category,
            bnto_core::metadata::NodeCategory::Vector
        ));
    }

    #[test]
    fn test_validate_passes_with_defaults() {
        let errors = VectorRasterize::new().validate(&serde_json::Map::new());
        assert!(errors.is_empty(), "Default params should pass validation");
    }

    #[test]
    fn test_validate_rejects_invalid_quality() {
        let mut params = serde_json::Map::new();
        params.insert("quality".to_string(), serde_json::json!(200));
        let errors = VectorRasterize::new().validate(&params);
        assert!(!errors.is_empty(), "quality=200 should fail validation");
    }

    #[test]
    fn test_validate_rejects_invalid_format() {
        let mut params = serde_json::Map::new();
        params.insert("format".to_string(), serde_json::json!("bmp"));
        let errors = VectorRasterize::new().validate(&params);
        assert!(!errors.is_empty(), "format=bmp should fail validation");
    }

    // --- SVG to PNG ---

    #[test]
    fn test_svg_to_png_produces_valid_png() {
        let input = make_input(make_params("png", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .expect("SVG to PNG should succeed");

        assert_eq!(output.files.len(), 1);
        let FileData::Bytes(ref data) = output.files[0].data else {
            panic!("expected FileData::Bytes");
        };
        // PNG magic bytes: 0x89 P N G
        assert!(data.len() > 8);
        assert_eq!(&data[..4], &[0x89, b'P', b'N', b'G']);
    }

    // --- SVG to JPEG ---

    #[test]
    fn test_svg_to_jpeg_produces_valid_jpeg() {
        let input = make_input(make_params("jpeg", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .expect("SVG to JPEG should succeed");

        assert_eq!(output.files.len(), 1);
        let FileData::Bytes(ref data) = output.files[0].data else {
            panic!("expected FileData::Bytes");
        };
        // JPEG magic: 0xFF 0xD8
        assert!(data.len() > 2);
        assert_eq!(&data[..2], &[0xFF, 0xD8]);
    }

    // --- SVG to WebP ---

    #[test]
    fn test_svg_to_webp_produces_valid_webp() {
        let input = make_input(make_params("webp", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .expect("SVG to WebP should succeed");

        assert_eq!(output.files.len(), 1);
        let FileData::Bytes(ref data) = output.files[0].data else {
            panic!("expected FileData::Bytes");
        };
        // RIFF....WEBP header
        assert!(data.len() > 12);
        assert_eq!(&data[..4], b"RIFF");
        assert_eq!(&data[8..12], b"WEBP");
    }

    // --- Quality sensitivity ---

    #[test]
    fn test_quality_affects_jpeg_size() {
        let reporter = ProgressReporter::new_noop();

        let input_low = make_input(make_params("jpeg", 20));
        let output_low = VectorRasterize::new()
            .process(input_low, &reporter, &NoopContext)
            .unwrap();

        let input_high = make_input(make_params("jpeg", 90));
        let output_high = VectorRasterize::new()
            .process(input_high, &reporter, &NoopContext)
            .unwrap();

        let FileData::Bytes(ref low_data) = output_low.files[0].data else {
            panic!("expected FileData::Bytes");
        };
        let FileData::Bytes(ref high_data) = output_high.files[0].data else {
            panic!("expected FileData::Bytes");
        };
        assert_ne!(
            low_data.len(),
            high_data.len(),
            "Different quality levels should produce different file sizes"
        );
    }

    // --- Output filename ---

    #[test]
    fn test_output_filename_has_target_extension() {
        let input = make_input(make_params("png", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "icon.png");
    }

    #[test]
    fn test_output_filename_jpeg_extension() {
        let input = make_input(make_params("jpeg", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .unwrap();
        assert_eq!(output.files[0].filename, "icon.jpg");
    }

    // --- Metadata ---

    #[test]
    fn test_metadata_reports_original_format() {
        let input = make_input(make_params("png", 80));
        let reporter = ProgressReporter::new_noop();
        let output = VectorRasterize::new()
            .process(input, &reporter, &NoopContext)
            .unwrap();
        assert_eq!(output.metadata["originalFormat"], "svg");
    }

    // --- Error handling ---

    #[test]
    fn test_invalid_svg_returns_error() {
        let input = NodeInput {
            data: FileData::Bytes(b"not valid svg data".to_vec()),
            filename: "bad.svg".to_string(),
            mime_type: Some("image/svg+xml".to_string()),
            params: make_params("png", 80),
        };
        let reporter = ProgressReporter::new_noop();
        let result = VectorRasterize::new().process(input, &reporter, &NoopContext);
        assert!(result.is_err(), "Invalid SVG should return an error");
    }

    #[test]
    fn test_empty_data_returns_error() {
        let input = NodeInput {
            data: FileData::Bytes(vec![]),
            filename: "empty.svg".to_string(),
            mime_type: Some("image/svg+xml".to_string()),
            params: make_params("png", 80),
        };
        let reporter = ProgressReporter::new_noop();
        let result = VectorRasterize::new().process(input, &reporter, &NoopContext);
        assert!(result.is_err(), "Empty data should return an error");
    }

    // --- Output MIME type ---

    #[test]
    fn test_output_mime_type_matches_format() {
        let cases = [
            ("png", "image/png"),
            ("jpeg", "image/jpeg"),
            ("webp", "image/webp"),
        ];
        let reporter = ProgressReporter::new_noop();

        for (format, expected_mime) in &cases {
            let input = make_input(make_params(format, 80));
            let output = VectorRasterize::new()
                .process(input, &reporter, &NoopContext)
                .unwrap();
            assert_eq!(
                output.files[0].mime_type, *expected_mime,
                "Format {format} should produce MIME {expected_mime}"
            );
        }
    }
}
