// Golden tests — byte-exact output verification for all predefined recipes.
//
// Run:        cargo test -p bnto -- golden
// Bless:      BLESS=1 cargo test -p bnto -- golden
// Then:       git diff engine/crates/bnto/tests/golden/

mod golden_helpers;
mod helpers;

use golden_helpers::assert_golden;
use helpers::{
    fixture_csv, fixture_image, fixture_overlay, run_custom_recipe_ok, run_explicit_recipe_ok,
    run_explicit_recipe_ok_multi, run_recipe_ok, run_recipe_ok_multi,
};

// --- Image Recipes ---

#[test]
fn golden_compress_images() {
    let (out, _) = run_recipe_ok("compress-images", &fixture_image("small.jpg"));
    assert_golden("compress-images", &out);
}

#[test]
fn golden_resize_images() {
    let (out, _) = run_recipe_ok("resize-images", &fixture_image("small.jpg"));
    assert_golden("resize-images", &out);
}

#[test]
fn golden_convert_image_format() {
    let (out, _) = run_recipe_ok("convert-image-format", &fixture_image("small.jpg"));
    assert_golden("convert-image-format", &out);
}

#[test]
fn golden_generate_thumbnails() {
    let (out, _) = run_recipe_ok("generate-thumbnails", &fixture_image("small.jpg"));
    assert_golden("generate-thumbnails", &out);
}

#[test]
fn golden_optimize_images_for_web() {
    let (out, _) = run_recipe_ok("optimize-images-for-web", &fixture_image("small.jpg"));
    assert_golden("optimize-images-for-web", &out);
}

#[test]
fn golden_compress_and_rename() {
    let (out, _) = run_recipe_ok("compress-and-rename", &fixture_image("small.jpg"));
    assert_golden("compress-and-rename", &out);
}

#[test]
fn golden_watermark_images() {
    let (out, _) = run_custom_recipe_ok("watermark-images", &fixture_image("small.jpg"));
    assert_golden("watermark-images", &out);
}

#[test]
fn golden_watermark_landscape_center() {
    let (out, _) = run_custom_recipe_ok(
        "watermark-landscape-center",
        &fixture_overlay("landscape-16x9.jpg"),
    );
    assert_golden("watermark-landscape-center", &out);
}

#[test]
fn golden_watermark_portrait_top_left() {
    let (out, _) = run_custom_recipe_ok(
        "watermark-portrait-top-left",
        &fixture_overlay("portrait-3x4.jpg"),
    );
    assert_golden("watermark-portrait-top-left", &out);
}

#[test]
fn golden_watermark_square_bottom_right() {
    let (out, _) = run_custom_recipe_ok(
        "watermark-square-bottom-right",
        &fixture_overlay("square-1x1.jpg"),
    );
    assert_golden("watermark-square-bottom-right", &out);
}

#[test]
fn golden_overlay_landscape() {
    let (out, _) =
        run_custom_recipe_ok("overlay-landscape", &fixture_overlay("landscape-16x9.jpg"));
    assert_golden("overlay-landscape", &out);
}

#[test]
fn golden_overlay_portrait() {
    let (out, _) = run_custom_recipe_ok("overlay-portrait", &fixture_overlay("portrait-3x4.jpg"));
    assert_golden("overlay-portrait", &out);
}

#[test]
fn golden_overlay_square() {
    let (out, _) = run_custom_recipe_ok("overlay-square", &fixture_overlay("square-1x1.jpg"));
    assert_golden("overlay-square", &out);
}

// --- File Recipes ---

#[test]
fn golden_rename_files() {
    let (out, _) = run_recipe_ok("rename-files", &fixture_image("small.jpg"));
    assert_golden("rename-files", &out);
}

// --- CSV Recipes ---

#[test]
fn golden_clean_csv() {
    let (out, _) = run_recipe_ok("clean-csv", &fixture_csv("messy.csv"));
    assert_golden("clean-csv", &out);
}

#[test]
fn golden_rename_csv_columns() {
    let (out, _) = run_recipe_ok("rename-csv-columns", &fixture_csv("simple.csv"));
    assert_golden("rename-csv-columns", &out);
}

#[test]
fn golden_standardize_csv() {
    let (out, _) = run_recipe_ok("standardize-csv", &fixture_csv("messy.csv"));
    assert_golden("standardize-csv", &out);
}

#[test]
fn golden_csv_to_json() {
    let (out, _) = run_recipe_ok("csv-to-json", &fixture_csv("simple.csv"));
    assert_golden("csv-to-json", &out);
}

#[test]
fn golden_merge_csv() {
    let (out, _) = run_recipe_ok_multi(
        "merge-csv",
        &[fixture_csv("simple.csv"), fixture_csv("numeric-values.csv")],
    );
    assert_golden("merge-csv", &out);
}

#[test]
fn golden_strip_exif() {
    let (out, _) = run_recipe_ok("strip-exif", &fixture_image("small.jpg"));
    assert_golden("strip-exif", &out);
}

#[test]
fn golden_watermark_images_medium() {
    let (out, _) = run_custom_recipe_ok("watermark-images", &fixture_image("medium.jpg"));
    assert_golden("watermark-images-medium", &out);
}

#[test]
fn golden_watermark_images_png() {
    let (out, _) = run_custom_recipe_ok("watermark-images", &fixture_image("small.png"));
    assert_golden("watermark-images-png", &out);
}

#[test]
fn golden_watermark_top_left() {
    let (out, _) = run_custom_recipe_ok("watermark-top-left", &fixture_image("small.jpg"));
    assert_golden("watermark-top-left", &out);
}

#[test]
fn golden_watermark_center_large() {
    let (out, _) = run_custom_recipe_ok("watermark-center-large", &fixture_image("small.jpg"));
    assert_golden("watermark-center-large", &out);
}

#[test]
fn golden_watermark_low_opacity() {
    let (out, _) = run_custom_recipe_ok("watermark-low-opacity", &fixture_image("small.jpg"));
    assert_golden("watermark-low-opacity", &out);
}

// --- Explicit (loop-container) equivalence tests ---
//
// These use preserved explicit recipe fixtures with loop containers.
// They assert against the SAME golden directories as the auto versions,
// proving byte-identical output between auto and explicit iteration modes.

#[test]
fn golden_compress_images_explicit() {
    let (out, _) = run_explicit_recipe_ok("compress-images", &fixture_image("small.jpg"));
    assert_golden("compress-images", &out);
}

#[test]
fn golden_resize_images_explicit() {
    let (out, _) = run_explicit_recipe_ok("resize-images", &fixture_image("small.jpg"));
    assert_golden("resize-images", &out);
}

#[test]
fn golden_convert_image_format_explicit() {
    let (out, _) = run_explicit_recipe_ok("convert-image-format", &fixture_image("small.jpg"));
    assert_golden("convert-image-format", &out);
}

#[test]
fn golden_generate_thumbnails_explicit() {
    let (out, _) = run_explicit_recipe_ok("generate-thumbnails", &fixture_image("small.jpg"));
    assert_golden("generate-thumbnails", &out);
}

#[test]
fn golden_optimize_images_for_web_explicit() {
    let (out, _) = run_explicit_recipe_ok("optimize-images-for-web", &fixture_image("small.jpg"));
    assert_golden("optimize-images-for-web", &out);
}

#[test]
fn golden_compress_and_rename_explicit() {
    let (out, _) = run_explicit_recipe_ok("compress-and-rename", &fixture_image("small.jpg"));
    assert_golden("compress-and-rename", &out);
}

#[test]
fn golden_rename_files_explicit() {
    let (out, _) = run_explicit_recipe_ok("rename-files", &fixture_image("small.jpg"));
    assert_golden("rename-files", &out);
}

#[test]
fn golden_clean_csv_explicit() {
    let (out, _) = run_explicit_recipe_ok("clean-csv", &fixture_csv("messy.csv"));
    assert_golden("clean-csv", &out);
}

#[test]
fn golden_rename_csv_columns_explicit() {
    let (out, _) = run_explicit_recipe_ok("rename-csv-columns", &fixture_csv("simple.csv"));
    assert_golden("rename-csv-columns", &out);
}

#[test]
fn golden_standardize_csv_explicit() {
    let (out, _) = run_explicit_recipe_ok("standardize-csv", &fixture_csv("messy.csv"));
    assert_golden("standardize-csv", &out);
}

#[test]
fn golden_strip_exif_explicit() {
    let (out, _) = run_explicit_recipe_ok("strip-exif", &fixture_image("small.jpg"));
    assert_golden("strip-exif", &out);
}

#[test]
fn golden_merge_csv_explicit() {
    let (out, _) = run_explicit_recipe_ok_multi(
        "merge-csv",
        &[fixture_csv("simple.csv"), fixture_csv("numeric-values.csv")],
    );
    assert_golden("merge-csv", &out);
}

#[test]
fn golden_csv_to_json_explicit() {
    let (out, _) = run_explicit_recipe_ok("csv-to-json", &fixture_csv("simple.csv"));
    assert_golden("csv-to-json", &out);
}

#[test]
fn golden_watermark_images_explicit() {
    let (out, _) = run_explicit_recipe_ok("watermark-images", &fixture_image("small.jpg"));
    assert_golden("watermark-images", &out);
}

#[test]
fn golden_watermark_landscape_center_explicit() {
    let (out, _) = run_explicit_recipe_ok(
        "watermark-landscape-center",
        &fixture_overlay("landscape-16x9.jpg"),
    );
    assert_golden("watermark-landscape-center", &out);
}

#[test]
fn golden_watermark_portrait_top_left_explicit() {
    let (out, _) = run_explicit_recipe_ok(
        "watermark-portrait-top-left",
        &fixture_overlay("portrait-3x4.jpg"),
    );
    assert_golden("watermark-portrait-top-left", &out);
}

#[test]
fn golden_watermark_square_bottom_right_explicit() {
    let (out, _) = run_explicit_recipe_ok(
        "watermark-square-bottom-right",
        &fixture_overlay("square-1x1.jpg"),
    );
    assert_golden("watermark-square-bottom-right", &out);
}

#[test]
fn golden_overlay_landscape_explicit() {
    let (out, _) =
        run_explicit_recipe_ok("overlay-landscape", &fixture_overlay("landscape-16x9.jpg"));
    assert_golden("overlay-landscape", &out);
}

#[test]
fn golden_overlay_portrait_explicit() {
    let (out, _) = run_explicit_recipe_ok("overlay-portrait", &fixture_overlay("portrait-3x4.jpg"));
    assert_golden("overlay-portrait", &out);
}

#[test]
fn golden_overlay_square_explicit() {
    let (out, _) = run_explicit_recipe_ok("overlay-square", &fixture_overlay("square-1x1.jpg"));
    assert_golden("overlay-square", &out);
}
