// Golden tests — byte-exact output verification for all predefined recipes.
//
// Run:        cargo test -p bnto-cli -- golden
// Bless:      BLESS=1 cargo test -p bnto-cli -- golden
// Then:       git diff engine/crates/bnto-cli/tests/golden/

mod golden_helpers;
mod helpers;

use golden_helpers::assert_golden;
use helpers::{fixture_csv, fixture_image, run_recipe_ok};

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
