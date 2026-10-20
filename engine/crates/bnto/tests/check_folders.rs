// `bnto check <folder>` through the built binary — a folder checks every
// `.bnto.json` under it, sorted by path, so a new recipe can't miss the check
// on a shell that expands no globs (Windows). A folder with no recipes is a
// finding, never a silent pass.

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Command;

use helpers::{bnto_bin, custom_recipe_path};

const BUNDLED_RECIPES: usize = 22;

/// Run `bnto check` on `paths`: (exit ok, stdout lines).
fn run_check(paths: &[String]) -> (bool, Vec<String>) {
    let output = Command::new(bnto_bin())
        .arg("check")
        .args(paths)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().map(str::to_string).collect();
    (output.status.success(), lines)
}

/// A folder under `tests/fixtures/check-folders`.
fn folder(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/check-folders")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn unknown_type(path: &str) -> String {
    format!(r#"{path}: node "zap" (image-zap): unknown node type "image-zap""#)
}

fn write_recipe(path: &Path, node_type: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let json = format!(r#"{{ "nodes": [{{ "id": "zap", "type": "{node_type}" }}] }}"#);
    std::fs::write(path, json).unwrap();
}

fn temp_path(dir: &tempfile::TempDir) -> String {
    dir.path().to_string_lossy().into_owned()
}

#[test]
fn test_check_passes_a_folder_of_nested_recipes() {
    let (ok, lines) = run_check(&[folder("good")]);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(lines, ["Check: PASS (2 recipes)"]);
}

#[test]
fn test_check_names_each_bad_file_under_a_folder_in_path_order() {
    let dir = folder("mixed");
    let (ok, lines) = run_check(std::slice::from_ref(&dir));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            unknown_type(&format!("{dir}/b-unknown.bnto.json")),
            unknown_type(&format!("{dir}/nested/deeper/c-unknown.bnto.json")),
            "Check: FAIL (2 findings)".to_string(),
        ]
    );
}

#[test]
fn test_check_counts_every_recipe_in_a_folder() {
    let (_, lines) = run_check(&[folder("good"), folder("good")]);
    assert_eq!(lines, ["Check: PASS (4 recipes)"]);
}

#[test]
fn test_check_takes_files_and_folders_together_in_argument_order() {
    let broken = custom_recipe_path("check/invalid-json");
    let dir = folder("mixed");
    let (ok, lines) = run_check(&[dir.clone(), broken.clone(), folder("good")]);
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(lines.len(), 4, "{lines:?}");
    assert_eq!(
        lines[0],
        unknown_type(&format!("{dir}/b-unknown.bnto.json"))
    );
    assert_eq!(
        lines[1],
        unknown_type(&format!("{dir}/nested/deeper/c-unknown.bnto.json"))
    );
    assert!(
        lines[2].starts_with(&format!("{broken}: invalid JSON:")),
        "{lines:?}"
    );
    assert_eq!(lines[3], "Check: FAIL (3 findings)");
}

#[test]
fn test_check_fails_a_folder_with_no_recipes_naming_the_folder() {
    let dir = folder("no-recipes");
    let (ok, lines) = run_check(std::slice::from_ref(&dir));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            format!("{dir}: no .bnto.json recipes in this folder"),
            "Check: FAIL (1 finding)".to_string(),
        ]
    );
}

#[test]
fn test_check_fails_an_empty_folder() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = temp_path(&tmp);
    let (ok, lines) = run_check(std::slice::from_ref(&dir));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            format!("{dir}: no .bnto.json recipes in this folder"),
            "Check: FAIL (1 finding)".to_string(),
        ]
    );
}

#[test]
fn test_an_empty_folder_does_not_hide_other_findings() {
    let tmp = tempfile::tempdir().unwrap();
    let empty = temp_path(&tmp);
    let dir = folder("mixed");
    let (ok, lines) = run_check(&[empty.clone(), dir]);
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines[0],
        format!("{empty}: no .bnto.json recipes in this folder")
    );
    assert_eq!(lines.last().unwrap(), "Check: FAIL (3 findings)");
}

#[test]
fn test_check_skips_node_modules_and_target_folders() {
    let tmp = tempfile::tempdir().unwrap();
    write_recipe(&tmp.path().join("ok.bnto.json"), "file-filter");
    write_recipe(&tmp.path().join("node_modules/pkg/bad.bnto.json"), "nope");
    write_recipe(&tmp.path().join("sub/target/debug/bad.bnto.json"), "nope");
    let (ok, lines) = run_check(&[temp_path(&tmp)]);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(lines, ["Check: PASS (1 recipe)"]);
}

#[test]
fn test_a_folder_named_on_the_command_line_is_checked_even_if_skipped_when_walking() {
    let tmp = tempfile::tempdir().unwrap();
    let target = tmp.path().join("target");
    write_recipe(&target.join("bad.bnto.json"), "nope");
    let dir = target.to_string_lossy().into_owned();
    let (ok, lines) = run_check(std::slice::from_ref(&dir));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines[0],
        format!(r#"{dir}/bad.bnto.json: node "zap" (nope): unknown node type "nope""#)
    );
}

#[test]
fn test_a_trailing_separator_on_a_folder_is_not_doubled() {
    let dir = folder("mixed");
    let with_slash = format!("{dir}{}", std::path::MAIN_SEPARATOR);
    let (_, lines) = run_check(&[with_slash]);
    assert_eq!(
        lines[0],
        unknown_type(&format!("{dir}/b-unknown.bnto.json"))
    );
}

#[test]
fn test_check_passes_the_bundled_recipes_folder() {
    let recipes: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../bnto-engine/recipes");
    let (ok, lines) = run_check(&[recipes.to_string_lossy().into_owned()]);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(lines, [format!("Check: PASS ({BUNDLED_RECIPES} recipes)")]);
}
