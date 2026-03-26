// Shared helpers for CLI integration tests.
// Each test file compiles as a separate crate — not all helpers are used by every file.
#![allow(dead_code)]

use std::process::Command;

/// Path to the compiled CLI binary.
pub fn bnto_bin() -> String {
    let path = std::path::PathBuf::from(env!("CARGO_BIN_EXE_bnto"));
    assert!(path.exists(), "bnto binary not found at {}", path.display());
    path.to_string_lossy().to_string()
}

/// Path to the engine workspace root.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Path to the repo root (engine's parent).
fn repo_root() -> std::path::PathBuf {
    engine_root().parent().unwrap().to_path_buf()
}

pub fn recipe_path(slug: &str) -> String {
    repo_root()
        .join(format!(
            "packages/@bnto/registry/src/recipes/generated/{slug}.bnto.json"
        ))
        .to_string_lossy()
        .to_string()
}

/// Path to an explicit (loop-container) recipe fixture.
pub fn explicit_recipe_path(slug: &str) -> String {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/explicit/{slug}.bnto.json"))
        .to_string_lossy()
        .to_string()
}

/// Run an explicit recipe fixture and assert it succeeds, returning stderr output.
pub fn run_explicit_recipe_ok(slug: &str, fixture: &str) -> (tempfile::TempDir, String) {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &explicit_recipe_path(slug)])
        .arg(fixture)
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "[explicit/{slug}] stderr: {stderr}");
    (out, stderr)
}

pub fn fixture_image(name: &str) -> String {
    repo_root()
        .join(format!("test-fixtures/images/{name}"))
        .to_string_lossy()
        .to_string()
}

pub fn fixture_csv(name: &str) -> String {
    repo_root()
        .join(format!("test-fixtures/csv/{name}"))
        .to_string_lossy()
        .to_string()
}

pub fn temp_output_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

/// Run a recipe and assert it succeeds, returning stderr output.
pub fn run_recipe_ok(slug: &str, fixture: &str) -> (tempfile::TempDir, String) {
    let out = temp_output_dir();
    let output = Command::new(bnto_bin())
        .args(["run", &recipe_path(slug)])
        .arg(fixture)
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "stderr: {stderr}");
    (out, stderr)
}

/// List output files in a temp directory.
pub fn output_files(dir: &tempfile::TempDir) -> Vec<std::fs::DirEntry> {
    std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect()
}
