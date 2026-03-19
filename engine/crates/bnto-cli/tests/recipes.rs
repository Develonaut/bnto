// Recipe execution tests — each predefined recipe runs end-to-end with real processors.
// This is the authoritative test for engine correctness.

mod common;

use common::{assert_csv, assert_jpeg, assert_webp, make_file, run_pipeline};

static SMALL_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
static SMALL_PNG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.png");
static LARGE_JPG: &[u8] = include_bytes!("../../../../test-fixtures/images/large.jpg");
static SIMPLE_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/simple.csv");
static MESSY_CSV: &[u8] = include_bytes!("../../../../test-fixtures/csv/messy.csv");

static CR1_DEF: &str =
    include_str!("../../../../test-fixtures/definitions/cr1-web-ready-image-pipeline.bnto.json");
static CR2_DEF: &str =
    include_str!("../../../../test-fixtures/definitions/cr2-compress-organize.bnto.json");
static CR3_DEF: &str =
    include_str!("../../../../test-fixtures/definitions/cr3-clean-standardize-csv.bnto.json");
static CR4_DEF: &str =
    include_str!("../../../../test-fixtures/definitions/cr4-thumbnail-generator.bnto.json");

// --- CR1: Web-ready image pipeline (resize → convert → compress) ---

#[test]
fn cr1_produces_webp_output() {
    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(CR1_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_webp(&result.files[0].data);
}

#[test]
fn cr1_output_is_smaller_than_input() {
    // Use large.jpg (176KB) — resize to 200px width dramatically reduces pixel count,
    // overcoming any format overhead from WebP lossless encoding.
    let files = vec![make_file("photo.jpg", LARGE_JPG, "image/jpeg")];
    let result = run_pipeline(CR1_DEF, files).unwrap();

    assert!(
        result.files[0].data.len() < LARGE_JPG.len(),
        "Output ({} bytes) should be smaller than input ({} bytes)",
        result.files[0].data.len(),
        LARGE_JPG.len()
    );
}

#[test]
fn cr1_works_with_png_input() {
    let files = vec![make_file("photo.png", SMALL_PNG, "image/png")];
    let result = run_pipeline(CR1_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_webp(&result.files[0].data);
}

#[test]
fn cr1_handles_multiple_files() {
    let files = vec![
        make_file("a.jpg", SMALL_JPG, "image/jpeg"),
        make_file("b.jpg", SMALL_JPG, "image/jpeg"),
        make_file("c.jpg", SMALL_JPG, "image/jpeg"),
    ];
    let result = run_pipeline(CR1_DEF, files).unwrap();
    assert_eq!(result.files.len(), 3);

    for file in &result.files {
        assert_webp(&file.data);
    }
}

// --- CR2: Compress + organize (compress → rename with suffix) ---

#[test]
fn cr2_produces_compressed_jpeg_with_suffix() {
    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(CR2_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_jpeg(&result.files[0].data);
    assert!(
        result.files[0].name.contains("-min"),
        "Output name should contain '-min' suffix, got: {}",
        result.files[0].name
    );
}

// --- CR3: Clean & standardize CSV (clean → rename columns) ---

#[test]
fn cr3_produces_valid_csv() {
    let files = vec![make_file("data.csv", SIMPLE_CSV, "text/csv")];
    let result = run_pipeline(CR3_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_csv(&result.files[0].data);
}

#[test]
fn cr3_cleans_messy_csv() {
    let files = vec![make_file("messy.csv", MESSY_CSV, "text/csv")];
    let result = run_pipeline(CR3_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_csv(&result.files[0].data);
}

// --- CR4: Thumbnail generator (resize → convert → rename) ---

#[test]
fn cr4_produces_small_webp_with_prefix() {
    let files = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let result = run_pipeline(CR4_DEF, files).unwrap();

    assert_eq!(result.files.len(), 1);
    assert_webp(&result.files[0].data);
    assert!(
        result.files[0].name.starts_with("thumb_"),
        "Output name should start with 'thumb_', got: {}",
        result.files[0].name
    );
}

#[test]
fn cr4_output_is_smaller_than_cr1() {
    // CR4 resizes to 100px (thumbnails), CR1 resizes to 200px.
    let files_cr1 = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];
    let files_cr4 = vec![make_file("photo.jpg", SMALL_JPG, "image/jpeg")];

    let result_cr1 = run_pipeline(CR1_DEF, files_cr1).unwrap();
    let result_cr4 = run_pipeline(CR4_DEF, files_cr4).unwrap();

    assert!(
        result_cr4.files[0].data.len() <= result_cr1.files[0].data.len(),
        "Thumbnail ({} bytes) should be <= web-ready ({} bytes)",
        result_cr4.files[0].data.len(),
        result_cr1.files[0].data.len()
    );
}
