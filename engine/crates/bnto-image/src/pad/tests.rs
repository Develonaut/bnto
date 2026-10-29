// Processor-level tests for `image-pad`: metadata, validation, canvas size,
// placement, background fill, format handling, and errors.

use bnto_core::NoopContext;
use bnto_core::errors::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

use super::PadImages;
use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const CLEAR: Rgba<u8> = Rgba([0, 0, 0, 0]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);

// --- Helpers ---

fn params(json: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    json.as_object().cloned().unwrap_or_default()
}

/// A `w`×`h` image of one color.
fn solid(w: u32, h: u32, color: Rgba<u8>) -> DynamicImage {
    DynamicImage::ImageRgba8(RgbaImage::from_pixel(w, h, color))
}

fn encode(img: &DynamicImage, format: image::ImageFormat) -> Vec<u8> {
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, format).unwrap();
    out.into_inner()
}

fn png(w: u32, h: u32, color: Rgba<u8>) -> Vec<u8> {
    encode(&solid(w, h, color), image::ImageFormat::Png)
}

fn run(data: Vec<u8>, filename: &str, p: serde_json::Value) -> Result<NodeOutput, BntoError> {
    let input = NodeInput {
        data: FileData::Bytes(data),
        filename: filename.to_string(),
        mime_type: None,
        params: params(p),
    };
    PadImages::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
}

fn bytes(output: &NodeOutput) -> &[u8] {
    match &output.files[0].data {
        FileData::Bytes(b) => b,
        FileData::Path(_) => panic!("image-pad should emit bytes"),
    }
}

fn decoded(output: &NodeOutput) -> DynamicImage {
    image::load_from_memory(bytes(output)).unwrap()
}

// --- Trait basics ---

#[test]
fn test_name_matches_registry_key() {
    assert_eq!(PadImages::new().name(), "image-pad");
}

#[test]
fn test_metadata_describes_a_per_file_browser_image_node() {
    let meta = PadImages::new().metadata();
    assert_eq!(meta.node_type, "image-pad");
    assert_eq!(meta.platforms, vec!["browser"]);
    assert_eq!(
        meta.input_cardinality,
        bnto_core::metadata::InputCardinality::PerFile
    );
    let names: Vec<_> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["width", "height", "background", "quality"]);
}

#[test]
fn test_background_defaults_to_transparent() {
    let meta = PadImages::new().metadata();
    let bg = meta.parameters.iter().find(|p| p.name == "background");
    assert_eq!(bg.unwrap().default, Some(serde_json::json!("transparent")));
}

// --- Validation ---

#[test]
fn test_validate_needs_a_width_or_height() {
    let errors = PadImages::new().validate(&params(serde_json::json!({})));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("width or height"), "{errors:?}");
}

#[test]
fn test_validate_accepts_one_side() {
    let pad = PadImages::new();
    assert!(
        pad.validate(&params(serde_json::json!({"width": 512})))
            .is_empty()
    );
    assert!(
        pad.validate(&params(serde_json::json!({"height": 512})))
            .is_empty()
    );
}

#[test]
fn test_validate_rejects_out_of_range_sizes() {
    let pad = PadImages::new();
    let zero = pad.validate(&params(serde_json::json!({"width": 0})));
    assert!(zero.iter().any(|e| e.contains("Width")), "{zero:?}");
    let huge = pad.validate(&params(serde_json::json!({"height": 20000})));
    assert!(huge.iter().any(|e| e.contains("Height")), "{huge:?}");
}

#[test]
fn test_validate_rejects_a_background_that_is_not_a_color() {
    let errors = PadImages::new().validate(&params(serde_json::json!({
        "width": 10, "background": "red"
    })));
    assert!(
        errors.iter().any(|e| e.contains("background")),
        "{errors:?}"
    );
}

#[test]
fn test_validate_rejects_bad_quality() {
    let errors = PadImages::new().validate(&params(serde_json::json!({
        "width": 10, "quality": 0
    })));
    assert!(errors.iter().any(|e| e.contains("Quality")), "{errors:?}");
}

// --- Canvas size and placement ---

#[test]
fn test_pads_a_wide_image_to_a_square_with_transparency() {
    let out = run(
        png(40, 20, RED),
        "logo.png",
        serde_json::json!({
            "width": 40, "height": 40
        }),
    )
    .unwrap();
    let img = decoded(&out).to_rgba8();
    assert_eq!(img.dimensions(), (40, 40));
    assert_eq!(*img.get_pixel(20, 0), CLEAR, "band above is transparent");
    assert_eq!(*img.get_pixel(20, 9), CLEAR);
    assert_eq!(*img.get_pixel(20, 10), RED, "image starts 10px down");
    assert_eq!(*img.get_pixel(20, 29), RED);
    assert_eq!(*img.get_pixel(20, 30), CLEAR, "band below is transparent");
}

#[test]
fn test_an_omitted_side_keeps_the_image_size() {
    let out = run(png(20, 10, RED), "a.png", serde_json::json!({"width": 30})).unwrap();
    assert_eq!(decoded(&out).dimensions(), (30, 10));
    let out = run(png(20, 10, RED), "a.png", serde_json::json!({"height": 16})).unwrap();
    assert_eq!(decoded(&out).dimensions(), (20, 16));
}

#[test]
fn test_an_odd_leftover_goes_right_and_down() {
    let out = run(png(3, 1, RED), "a.png", serde_json::json!({"width": 6})).unwrap();
    let img = decoded(&out).to_rgba8();
    let row: Vec<_> = (0..6).map(|x| *img.get_pixel(x, 0)).collect();
    assert_eq!(row, [CLEAR, RED, RED, RED, CLEAR, CLEAR]);
}

#[test]
fn test_a_canvas_the_image_size_keeps_every_pixel() {
    let out = run(
        png(8, 5, RED),
        "a.png",
        serde_json::json!({
            "width": 8, "height": 5
        }),
    )
    .unwrap();
    let img = decoded(&out).to_rgba8();
    assert_eq!(img.dimensions(), (8, 5));
    assert!(img.pixels().all(|p| *p == RED));
}

#[test]
fn test_a_one_pixel_image_pads() {
    let out = run(
        png(1, 1, RED),
        "a.png",
        serde_json::json!({
            "width": 3, "height": 3
        }),
    )
    .unwrap();
    let img = decoded(&out).to_rgba8();
    assert_eq!(*img.get_pixel(1, 1), RED);
    assert_eq!(*img.get_pixel(0, 0), CLEAR);
}

#[test]
fn test_an_image_larger_than_the_canvas_fails_naming_both_sizes() {
    let result = run(
        png(60, 40, RED),
        "big.png",
        serde_json::json!({
            "width": 50, "height": 50
        }),
    );
    let msg = result.err().expect("should fail").to_string();
    assert!(msg.contains("60x40"), "{msg}");
    assert!(msg.contains("50x50"), "{msg}");
}

// --- Background ---

#[test]
fn test_a_background_color_fills_the_padding() {
    let out = run(
        png(2, 2, RED),
        "a.png",
        serde_json::json!({
            "width": 4, "height": 4, "background": "#00ff00"
        }),
    )
    .unwrap();
    let img = decoded(&out).to_rgba8();
    assert_eq!(*img.get_pixel(0, 0), GREEN);
    assert_eq!(*img.get_pixel(1, 1), RED);
}

#[test]
fn test_the_background_shows_through_transparent_source_pixels() {
    let out = run(
        png(2, 2, CLEAR),
        "a.png",
        serde_json::json!({
            "width": 2, "height": 2, "background": "#0f0"
        }),
    )
    .unwrap();
    assert_eq!(*decoded(&out).to_rgba8().get_pixel(0, 0), GREEN);
}

#[test]
fn test_a_background_alpha_is_kept() {
    let out = run(
        png(1, 1, RED),
        "a.png",
        serde_json::json!({
            "width": 2, "background": "#00ff0080"
        }),
    )
    .unwrap();
    assert_eq!(
        *decoded(&out).to_rgba8().get_pixel(1, 0),
        Rgba([0, 255, 0, 128])
    );
}

// --- Formats ---

#[test]
fn test_jpeg_padding_comes_out_white_when_transparent() {
    let out = run(
        create_test_jpeg(20, 10),
        "photo.jpg",
        serde_json::json!({
            "width": 20, "height": 20
        }),
    )
    .unwrap();
    assert_eq!(&bytes(&out)[..2], &[0xFF, 0xD8], "still a JPEG");
    assert_eq!(out.files[0].mime_type, "image/jpeg");
    let corner = decoded(&out).to_rgb8().get_pixel(0, 0).0;
    assert!(
        corner.iter().all(|c| *c > 240),
        "white padding, got {corner:?}"
    );
}

#[test]
fn test_webp_keeps_transparent_padding() {
    let data = encode(&solid(4, 2, RED), image::ImageFormat::WebP);
    let out = run(data, "a.webp", serde_json::json!({"width": 4, "height": 4})).unwrap();
    assert_eq!(out.files[0].mime_type, "image/webp");
    assert_eq!(decoded(&out).to_rgba8().get_pixel(0, 0)[3], 0);
}

#[test]
fn test_exif_orientation_is_applied_before_padding() {
    // Stored 60x40 with orientation 6 → displayed 40x60, which fits 40 wide.
    let data = inject_exif_orientation(&create_test_jpeg(60, 40), 6);
    let out = run(
        data,
        "phone.jpg",
        serde_json::json!({"width": 40, "height": 60}),
    );
    assert_eq!(decoded(&out.unwrap()).dimensions(), (40, 60));
}

// --- Output file and metadata ---

#[test]
fn test_output_filename_gets_a_padded_suffix() {
    let out = run(png(1, 1, RED), "logo.png", serde_json::json!({"width": 2})).unwrap();
    assert_eq!(out.files[0].filename, "logo-padded.png");
    let out = run(png(1, 1, RED), "logo", serde_json::json!({"width": 2})).unwrap();
    assert_eq!(out.files[0].filename, "logo-padded.png");
}

#[test]
fn test_metadata_reports_sizes_and_format() {
    let data = png(4, 2, RED);
    let original_size = data.len() as u64;
    let out = run(data, "a.png", serde_json::json!({"width": 4, "height": 4})).unwrap();
    let m = &out.metadata;
    assert_eq!(m["originalWidth"], 4);
    assert_eq!(m["originalHeight"], 2);
    assert_eq!(m["newWidth"], 4);
    assert_eq!(m["newHeight"], 4);
    assert_eq!(m["originalSize"], original_size);
    assert_eq!(m["newSize"], bytes(&out).len() as u64);
    assert_eq!(m["format"], "Png");
}

// --- Errors ---

#[test]
fn test_corrupt_input_fails() {
    let result = run(
        b"not an image".to_vec(),
        "a.png",
        serde_json::json!({"width": 2}),
    );
    assert!(result.is_err());
}

#[test]
fn test_empty_input_fails() {
    assert!(run(Vec::new(), "a.png", serde_json::json!({"width": 2})).is_err());
}

#[test]
fn test_output_filename_keeps_a_dotted_folder() {
    // ISS-33: the dot in the folder `v1.2` used to split the name.
    let out = run(png(1, 1, RED), "v1.2/data", serde_json::json!({"width": 2})).unwrap();
    assert_eq!(out.files[0].filename, "v1.2/data-padded.png");
}
