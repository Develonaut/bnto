// Recipe integration tests — verify each recipe type produces correct output.

mod helpers;

use helpers::{fixture_csv, fixture_image, output_files, run_recipe_ok};

// --- Image Recipes ---

#[test]
fn test_compress_images() {
    let (out, stderr) = run_recipe_ok("compress-images", &fixture_image("small.jpg"));
    assert!(stderr.contains("Done."));
    assert_eq!(output_files(&out).len(), 1, "Expected 1 output file");
}

#[test]
fn test_resize_images() {
    run_recipe_ok("resize-images", &fixture_image("small.jpg"));
}

#[test]
fn test_convert_image_format() {
    let (out, _) = run_recipe_ok("convert-image-format", &fixture_image("small.jpg"));
    let files = output_files(&out);
    assert_eq!(files.len(), 1);
    let name = files[0].file_name().to_string_lossy().to_string();
    assert!(
        name.ends_with(".webp"),
        "Expected .webp output, got: {name}"
    );
}

// --- File Recipes ---

#[test]
fn test_rename_files() {
    let (out, _) = run_recipe_ok("rename-files", &fixture_image("small.jpg"));
    let files = output_files(&out);
    assert_eq!(files.len(), 1);
    let name = files[0].file_name().to_string_lossy().to_string();
    assert!(
        name.starts_with("renamed-"),
        "Expected 'renamed-' prefix, got: {name}"
    );
}

// --- CSV Recipes ---

#[test]
fn test_clean_csv() {
    let (out, _) = run_recipe_ok("clean-csv", &fixture_csv("messy.csv"));
    assert_eq!(output_files(&out).len(), 1);
}

// --- Multi-Step Recipes ---

#[test]
fn test_generate_thumbnails() {
    run_recipe_ok("generate-thumbnails", &fixture_image("small.jpg"));
}

#[test]
fn test_optimize_images_for_web() {
    run_recipe_ok("optimize-images-for-web", &fixture_image("small.jpg"));
}
