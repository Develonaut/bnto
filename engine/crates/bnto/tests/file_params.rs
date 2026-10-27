// `-p` gives a file param (Watermark Images' overlay) as a path: the CLI
// reads the file and hands the node its base64 text (ISS-75).

mod helpers;

use std::process::Command;

use helpers::{bnto_bin, fixture_image, output_files, recipe_path};

/// Run Watermark Images on `small.jpg` into `out` with `-p` entries.
fn watermark(out: &tempfile::TempDir, params: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(bnto_bin());
    cmd.args(["run", &recipe_path("watermark-images"), "--yes"])
        .arg(fixture_image("small.jpg"))
        .arg("-o")
        .arg(out.path());
    for param in params {
        cmd.args(["-p", param]);
    }
    cmd.output().unwrap()
}

#[test]
fn test_overlay_param_takes_a_path() {
    let out = tempfile::tempdir().unwrap();
    let overlay = format!("overlay:overlay={}", fixture_image("small.png"));

    let output = watermark(&out, &[&overlay]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let files = output_files(&out);
    assert_eq!(files.len(), 1, "one watermarked image");
    let written = std::fs::read(files[0].path()).unwrap();
    assert_eq!(&written[..3], &[0xFF, 0xD8, 0xFF], "JPEG magic bytes");
}

#[test]
fn test_overlay_path_that_is_missing_is_refused_naming_it() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing.png");
    let out_parent = tempfile::tempdir().unwrap();
    let out = out_parent.path().join("out");
    let overlay = format!("overlay:overlay={}", missing.display());

    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path("watermark-images"), "--yes"])
        .arg(fixture_image("small.jpg"))
        .arg("-o")
        .arg(&out)
        .args(["-p", &overlay])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "a missing overlay fails the run");
    assert!(
        stderr.contains("--param overlay:overlay") && stderr.contains("missing.png"),
        "names the param and the path: {stderr}"
    );
    assert!(!out.exists(), "refused before the output folder is made");
}
