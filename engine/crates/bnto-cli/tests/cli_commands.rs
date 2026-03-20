// CLI command tests — help, list, error handling, batch processing.

mod helpers;

use std::process::Command;

use helpers::{bnto_bin, fixture_image, output_files, recipe_path, temp_output_dir};

// --- Help & List ---

#[test]
fn test_help_flag() {
    let output = Command::new(bnto_bin()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Run .bnto.json recipes"));
}

#[test]
fn test_list_command() {
    let output = Command::new(bnto_bin()).arg("list").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("image-compress"));
    assert!(stdout.contains("file-rename"));
    assert!(stdout.contains("spreadsheet-clean"));
}

// --- Error Cases ---

#[test]
fn test_missing_recipe_file() {
    let output = Command::new(bnto_bin())
        .args(["run", "nonexistent.bnto.json", "file.jpg"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error reading recipe"));
}

#[test]
fn test_missing_input_file() {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("compress-images")])
        .arg("nonexistent.jpg")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no valid input files"));
}

// --- Multiple Files ---

#[test]
fn test_multiple_input_files() {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("compress-images")])
        .arg(fixture_image("small.jpg"))
        .arg(fixture_image("small.png"))
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("2 files"));
    assert_eq!(output_files(&out).len(), 2, "Expected 2 output files");
}
