// Parameter sensitivity tests — verify different param values produce different outputs.

mod common;

use common::{assert_jpeg, assert_png, assert_webp, make_file, run_pipeline};

static SMALL_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
// medium.jpg has enough pixel data for compression quality to produce size differences.
static MEDIUM_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.jpg");

#[test]
fn compress_level_affects_output_size() {
    // CompressImages uses `compression` param (inverted: 100=max compression, 1=minimal).
    let high_compression = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress", "compression": 90 } },
            { "id": "output", "type": "output" }
        ]
    }"#;
    let low_compression = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "compress", "type": "image", "parameters": { "operation": "compress", "compression": 10 } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files_high = vec![make_file("photo.jpg", MEDIUM_JPG, "image/jpeg")];
    let files_low = vec![make_file("photo.jpg", MEDIUM_JPG, "image/jpeg")];

    let result_high = run_pipeline(high_compression, files_high).unwrap();
    let result_low = run_pipeline(low_compression, files_low).unwrap();

    assert_jpeg(&result_high.files[0].data);
    assert_jpeg(&result_low.files[0].data);

    assert!(
        result_high.files[0].data.len() < result_low.files[0].data.len(),
        "High compression ({} bytes) should be smaller than low compression ({} bytes)",
        result_high.files[0].data.len(),
        result_low.files[0].data.len()
    );
}

#[test]
fn resize_width_affects_output_size() {
    let wide = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image", "parameters": { "operation": "resize", "width": 200 } },
            { "id": "output", "type": "output" }
        ]
    }"#;
    let narrow = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "resize", "type": "image", "parameters": { "operation": "resize", "width": 50 } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files_wide = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let files_narrow = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];

    let result_wide = run_pipeline(wide, files_wide).unwrap();
    let result_narrow = run_pipeline(narrow, files_narrow).unwrap();

    assert_jpeg(&result_wide.files[0].data);
    assert_jpeg(&result_narrow.files[0].data);

    assert!(
        result_narrow.files[0].data.len() < result_wide.files[0].data.len(),
        "Narrow ({} bytes) should be smaller than wide ({} bytes)",
        result_narrow.files[0].data.len(),
        result_wide.files[0].data.len()
    );
}

#[test]
fn convert_format_produces_correct_output_type() {
    let to_webp = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "convert", "type": "image", "parameters": { "operation": "convert", "format": "webp" } },
            { "id": "output", "type": "output" }
        ]
    }"#;
    let to_png = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "convert", "type": "image", "parameters": { "operation": "convert", "format": "png" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files_webp = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let files_png = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];

    let result_webp = run_pipeline(to_webp, files_webp).unwrap();
    let result_png = run_pipeline(to_png, files_png).unwrap();

    assert_webp(&result_webp.files[0].data);
    assert_png(&result_png.files[0].data);
}

#[test]
fn rename_prefix_applied_correctly() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "rename", "type": "file-system", "parameters": { "operation": "rename", "prefix": "v2_" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("document.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert!(
        result.files[0].name.starts_with("v2_"),
        "Expected prefix 'v2_', got: {}",
        result.files[0].name
    );
}

#[test]
fn rename_suffix_applied_correctly() {
    let def = r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            { "id": "rename", "type": "file-system", "parameters": { "operation": "rename", "suffix": "-final" } },
            { "id": "output", "type": "output" }
        ]
    }"#;

    let files = vec![make_file("document.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(def, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert!(
        result.files[0].name.contains("-final"),
        "Expected suffix '-final', got: {}",
        result.files[0].name
    );
}
