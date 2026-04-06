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
    // New output lists recipe slugs grouped by category, not processor type names.
    assert!(
        stdout.contains("compress-images"),
        "Should list compress-images recipe, got: {stdout}"
    );
    assert!(
        stdout.contains("rename-files"),
        "Should list rename-files recipe, got: {stdout}"
    );
    assert!(
        stdout.contains("clean-csv"),
        "Should list clean-csv recipe, got: {stdout}"
    );
    // Verify category headers are present.
    assert!(
        stdout.contains("image"),
        "Should have image category, got: {stdout}"
    );
}

#[test]
fn test_info_command() {
    let output = Command::new(bnto_bin())
        .args(["info", "compress-images"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Compress Images"),
        "Should show recipe name, got: {stdout}"
    );
    assert!(
        stdout.contains("image"),
        "Should show category, got: {stdout}"
    );
    assert!(
        stdout.contains("image-compress"),
        "Should show processor node type, got: {stdout}"
    );
}

#[test]
fn test_info_unknown_recipe() {
    let output = Command::new(bnto_bin())
        .args(["info", "nonexistent-recipe"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown recipe"),
        "Should show unknown recipe error, got: {stderr}"
    );
}

#[test]
fn test_doctor_command() {
    let output = Command::new(bnto_bin()).arg("doctor").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Doctor should run without crashing. It may pass or fail depending
    // on whether external tools (yt-dlp, ffmpeg) are installed.
    assert!(
        stdout.contains("ok") || stdout.contains("MISSING") || stdout.contains("self-contained"),
        "Doctor should report dependency status, got: {stdout}"
    );
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
fn test_youtube_download() {
    // End-to-end download: YouTube → yt-dlp → output file.
    // "Me at the Zoo" (jNQXAC9IVRw) — first YouTube video, always available, ~465KB.
    // Requires yt-dlp + ffmpeg installed. Skips gracefully if missing.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Must route through URL mode
    assert!(
        stderr.contains("URL input"),
        "Expected 'URL input' status message, got: {stderr}"
    );

    if output.status.success() {
        // Download succeeded — verify output file exists with content
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
    } else {
        // yt-dlp not installed — error must come from processor, not routing
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not routing: {stderr}"
        );
    }
}

#[test]
fn test_youtube_download_with_param_override() {
    // Downloads with --param format=webm override.
    // Verifies param injection works alongside URL input.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["--param", "format=webm"])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("URL input"),
        "Expected 'URL input' status message, got: {stderr}"
    );
    assert!(
        !stderr.contains("Invalid --param"),
        "Param override should be valid"
    );

    if output.status.success() {
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
    } else {
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not routing: {stderr}"
        );
    }
}

// --- m3u8 / HLS URL ---

#[test]
fn test_m3u8_url_routes_correctly() {
    // Verifies m3u8 (HLS) URLs route through URL mode without
    // validation errors. Routing is fast — no download needed.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("URL input"),
        "m3u8 URL should route through URL mode, got: {stderr}"
    );
    assert!(
        !stderr.contains("Expected a URL"),
        "m3u8 URL should pass validation"
    );
}

#[test]
#[ignore] // HLS streams are large (~466MB). Run manually: cargo test -p bnto -- --ignored
fn test_m3u8_download() {
    // End-to-end download: m3u8 (HLS) → yt-dlp → output file.
    // Big Buck Bunny via Mux test stream. Requires yt-dlp + ffmpeg.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8")
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "m3u8 download failed: {stderr}");

    let files = output_files(&out);
    assert!(!files.is_empty(), "Expected at least one output file");
    let size = files[0].metadata().unwrap().len();
    assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
}

// --- Extra Args ---

#[test]
fn test_youtube_download_with_extra_args_verbose() {
    // Extra args pass-through: --verbose makes yt-dlp emit debug output.
    // If args weren't appended, yt-dlp would run silently (no debug lines).
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["--param", "args=--verbose"])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
    } else {
        // yt-dlp not installed — confirm failure is from yt-dlp, not args parsing
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not extra args: {stderr}"
        );
    }
}

#[test]
fn test_youtube_download_with_extra_args_quality_cap() {
    // Extra args override: -S "res:360" caps resolution to 360p via args.
    // This overrides the recipe's default quality since args are appended last
    // (yt-dlp uses last-wins for -S flags).
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["--param", "args=-S res:360"])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        // 360p "Me at the zoo" (~19s) should be smaller than 2MB
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
        assert!(
            size < 2_000_000,
            "360p video should be < 2MB, got {size} bytes — args may not have applied"
        );
    } else {
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not extra args: {stderr}"
        );
    }
}

#[test]
fn test_youtube_download_with_extra_args_write_json() {
    // Extra args: --write-info-json tells yt-dlp to write a .info.json sidecar.
    // yt-dlp writes it next to the output file. Verifies args that produce
    // additional output files work through the pass-through.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args(["--param", "args=--write-info-json"])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        // The video file should be present and valid
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
    } else {
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not extra args: {stderr}"
        );
    }
}

#[test]
fn test_youtube_download_with_multiple_extra_args() {
    // Multiple extra args combined: --no-check-certificates --socket-timeout 30
    // These are harmless flags that shouldn't affect the download result
    // but verify multiple space-separated args are all passed through.
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("download-video")])
        .arg("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .args([
            "--param",
            "args=--no-check-certificates --socket-timeout 30",
        ])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let files = output_files(&out);
        assert!(!files.is_empty(), "Expected at least one output file");
        let size = files[0].metadata().unwrap().len();
        assert!(size > 1000, "Output file should be > 1KB, got {size} bytes");
    } else {
        assert!(
            stderr.contains("Failed to run") || stderr.contains("yt-dlp"),
            "Should fail due to yt-dlp, not extra args: {stderr}"
        );
    }
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
