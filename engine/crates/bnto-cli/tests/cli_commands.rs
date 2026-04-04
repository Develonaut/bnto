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

// --- URL Mode ---

#[test]
fn test_url_mode_no_input_shows_error() {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("requires a URL"),
        "Expected URL-specific error, got: {stderr}"
    );
}

#[test]
fn test_url_mode_non_url_input_shows_error() {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("not-a-url.mp4")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Expected a URL"),
        "Expected URL validation error, got: {stderr}"
    );
}

#[test]
fn test_url_mode_valid_url_reaches_processor() {
    // Passes a valid URL. The processor will fail because yt-dlp
    // likely isn't installed in CI, but the error should come from
    // the processor — NOT from URL validation or input routing.
    // This proves the full path: CLI → input routing → engine.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    // The status message should say "URL input", proving mode detection worked
    assert!(
        stderr.contains("URL input"),
        "Expected 'URL input' status message, got: {stderr}"
    );

    // If yt-dlp is installed, the command succeeds (actual download).
    // If yt-dlp is NOT installed, it fails with a processor error.
    // Either way, we must NOT see input validation errors.
    assert!(
        !stderr.contains("Expected a URL"),
        "URL should have passed validation"
    );
    assert!(
        !stderr.contains("requires a URL"),
        "URL was provided but routing failed"
    );
}

#[test]
fn test_url_mode_param_override() {
    // Combines URL input with --param override.
    // Like the valid URL test, the processor may fail without yt-dlp,
    // but we verify the input routing doesn't reject the combination.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["--param", "format=webm"])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Must not fail on input routing or param injection
    assert!(
        !stderr.contains("Expected a URL"),
        "URL should have passed validation"
    );
    assert!(
        !stderr.contains("Invalid --param"),
        "Param override should be valid"
    );
    assert!(
        stderr.contains("URL input"),
        "Expected 'URL input' status message, got: {stderr}"
    );
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
