// PadImages — NodeProcessor wiring for `image-pad`.
//
// Decodes with EXIF orientation, resolves the canvas size, centers the image
// on the filled canvas, and re-encodes in the source format.

use image::DynamicImage;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use bnto_encode::ImageFormat;

use super::canvas::{flatten_onto_white, pad_onto_canvas};
use super::param_defs::pad_param_defs;
use super::params::PadParams;
use super::validate::validate_pad_params;
use crate::common::image_accepts;
use crate::encode;
use crate::orientation::decode_with_orientation;

/// The pad-images node processor.
pub struct PadImages;

impl Default for PadImages {
    fn default() -> Self {
        Self::new()
    }
}

impl PadImages {
    pub fn new() -> Self {
        Self
    }

    /// "logo.png" -> "logo-padded.png"
    fn output_filename(input_filename: &str, format: ImageFormat) -> String {
        let stem = input_filename
            .rsplit_once('.')
            .map_or(input_filename, |(stem, _)| stem);
        format!("{stem}-padded.{}", format.extension())
    }
}

impl NodeProcessor for PadImages {
    fn name(&self) -> &str {
        "image-pad"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "image-pad".to_string(),
            name: "Pad Images".to_string(),
            description: "Pad images onto a larger, centered canvas, filling the added space with \
                          transparency or a color."
                .to_string(),
            category: NodeCategory::Image,
            accepts: image_accepts(),
            platforms: vec!["browser".to_string()],
            parameters: pad_param_defs(),
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
        let data = input
            .data
            .into_bytes()
            .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read input: {e}")))?;
        let format = detect_format(&data, &input.filename)?;
        let params = PadParams::parse(&input.params)?;

        progress.report(10, "Decoding image...");
        let img = decode_with_orientation(&data)?;
        let orig = (img.width(), img.height());
        let canvas = params.canvas_size(orig, &input.filename)?;
        progress.report(40, &format!("Padding to {}x{}...", canvas.0, canvas.1));

        let padded = pad_for_format(&img, canvas, &params, format);
        let output_data = encode::encode_image(&padded, format, params.quality)?;
        let metadata = pad_metadata(orig, canvas, data.len(), output_data.len(), format);

        progress.report(100, "Pad complete");
        let filename = Self::output_filename(&input.filename, format);
        Ok(build_pad_output(output_data, filename, format, metadata))
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        validate_pad_params(params)
    }
}

// --- Private helpers for process ---

fn detect_format(data: &[u8], filename: &str) -> Result<ImageFormat, BntoError> {
    ImageFormat::detect(data, filename).ok_or_else(|| {
        BntoError::UnsupportedFormat(format!("Could not determine image format for '{filename}'"))
    })
}

/// Pad, then flatten for JPEG, which can't hold the transparent fill.
fn pad_for_format(
    img: &DynamicImage,
    canvas: (u32, u32),
    params: &PadParams,
    format: ImageFormat,
) -> DynamicImage {
    let padded = pad_onto_canvas(img, canvas, params.background);
    match format {
        ImageFormat::Jpeg => flatten_onto_white(&padded),
        ImageFormat::Png | ImageFormat::WebP => padded,
    }
}

fn build_pad_output(
    data: Vec<u8>,
    filename: String,
    format: ImageFormat,
    metadata: serde_json::Map<String, serde_json::Value>,
) -> NodeOutput {
    NodeOutput {
        files: vec![OutputFile {
            data: FileData::Bytes(data),
            filename,
            mime_type: format.mime_type().to_string(),
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

fn pad_metadata(
    (orig_w, orig_h): (u32, u32),
    (new_w, new_h): (u32, u32),
    original_size: usize,
    new_size: usize,
    format: ImageFormat,
) -> serde_json::Map<String, serde_json::Value> {
    serde_json::Map::from_iter([
        ("originalWidth".to_string(), orig_w.into()),
        ("originalHeight".to_string(), orig_h.into()),
        ("newWidth".to_string(), new_w.into()),
        ("newHeight".to_string(), new_h.into()),
        ("originalSize".to_string(), (original_size as u64).into()),
        ("newSize".to_string(), (new_size as u64).into()),
        ("format".to_string(), format!("{format:?}").into()),
    ])
}
