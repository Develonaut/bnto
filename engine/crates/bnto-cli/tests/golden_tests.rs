// Golden tests — byte-exact output verification for all predefined recipes.
//
// Run:        cargo test -p bnto-cli -- golden
// Bless:      BLESS=1 cargo test -p bnto-cli -- golden
// Then:       git diff engine/crates/bnto-cli/tests/golden/

mod golden_helpers;
mod helpers;

use golden_helpers::assert_golden;
use helpers::{fixture_csv, fixture_image, run_explicit_recipe_ok, run_recipe_ok};

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
fn golden_strip_exif() {
    let (out, _) = run_recipe_ok("strip-exif", &fixture_image("small.jpg"));
    assert_golden("strip-exif", &out);
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
fn golden_csv_to_json_explicit() {
    let (out, _) = run_explicit_recipe_ok("csv-to-json", &fixture_csv("simple.csv"));
    assert_golden("csv-to-json", &out);
}
