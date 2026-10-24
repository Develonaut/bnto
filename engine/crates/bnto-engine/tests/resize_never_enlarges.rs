// The bundled recipes that resize to a target size for the web never make an
// image bigger: a 400×400 image stays 400×400, a 1200×800 one still shrinks.

use bnto_core::processor::FileData;
use bnto_core::{NoopContext, PipelineFile, PipelineReporter};
use bnto_engine::run_pipeline;

const MEDIUM_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/medium.jpg");
const LARGE_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/large.jpg");
const OPTIMIZE_FOR_WEB: &str = include_str!("../recipes/optimize-images-for-web.bnto.json");
const THUMBNAILS: &str = include_str!("../recipes/generate-thumbnails.bnto.json");

/// Run a recipe on one JPEG and return the output image's (width, height).
fn output_dimensions(recipe: &str, jpeg: &[u8]) -> (u32, u32) {
    let files = vec![PipelineFile {
        name: "photo.jpg".to_string(),
        data: FileData::Bytes(jpeg.to_vec()),
        mime_type: "image/jpeg".to_string(),
        metadata: serde_json::Map::new(),
    }];
    let reporter = PipelineReporter::new_noop();
    let result = run_pipeline(recipe, files, &reporter, &NoopContext).expect("recipe runs");
    assert_eq!(result.files.len(), 1);
    let bytes = result.files[0].data.clone().into_bytes().unwrap();
    let img = bnto_image::orientation::decode_with_orientation(&bytes).unwrap();
    (img.width(), img.height())
}

#[test]
fn optimize_images_for_web_keeps_a_400px_image_at_400px() {
    assert_eq!(output_dimensions(OPTIMIZE_FOR_WEB, MEDIUM_JPEG), (400, 400));
}

#[test]
fn optimize_images_for_web_still_shrinks_a_wide_image_to_800px() {
    assert_eq!(output_dimensions(OPTIMIZE_FOR_WEB, LARGE_JPEG), (800, 533));
}

#[test]
fn generate_thumbnails_keeps_an_image_smaller_than_the_thumbnail() {
    // A 400×400 image is bigger than the 150 px thumbnail, so it shrinks...
    assert_eq!(output_dimensions(THUMBNAILS, MEDIUM_JPEG), (150, 150));
    // ...but the 100×100 fixture is already smaller, so it stays.
    let small = include_bytes!("../../../../test-fixtures/images/small.jpg");
    assert_eq!(output_dimensions(THUMBNAILS, small), (100, 100));
}
