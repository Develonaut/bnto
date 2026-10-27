// `bnto check` on recipe edges through the built binary — a cycle and edges
// that leave their level are findings, and every bundled recipe and fixture
// (all wired as chains) still passes.

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

/// Every `.bnto.json` file directly inside `dir`, sorted.
fn recipes_in(dir: &Path) -> Vec<String> {
    let mut paths: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.to_string_lossy().ends_with(".bnto.json"))
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    paths.sort();
    paths
}

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}

#[test]
fn test_check_reports_a_cycle_naming_its_path() {
    let path = custom_recipe_path("check/cycle");
    let (ok, lines) = run_check(std::slice::from_ref(&path));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            format!(
                "{path}: node \"keep-docs\" (file-filter): edges form a cycle: \
                 keep-docs → copy → keep-docs"
            ),
            "Check: FAIL (1 finding)".to_string(),
        ]
    );
}

#[test]
fn test_check_reports_every_edge_that_leaves_its_level() {
    let path = custom_recipe_path("check/dangling-edge");
    let (ok, lines) = run_check(std::slice::from_ref(&path));
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            format!(
                "{path}: node \"input\" (input): edge \"e2\" connects to \"copy\", \
                 which is inside container \"each\"; edges only connect nodes at the same level"
            ),
            format!(
                "{path}: node \"copy\" (file-copy): edge \"le2\" connects to \"archive\", \
                 which is not a node in this recipe"
            ),
            "Check: FAIL (2 findings)".to_string(),
        ]
    );
}

#[test]
fn test_check_passes_every_bundled_recipe() {
    let recipes = recipes_in(&crates_dir().join("bnto-engine/recipes"));
    assert_eq!(recipes.len(), BUNDLED_RECIPES, "{recipes:?}");
    let (ok, lines) = run_check(&recipes);
    assert!(ok, "stdout: {lines:?}");
    // Watermark Images leaves its overlay for the run: the web form or
    // `-p` gives it (ISS-25).
    let watermark = recipes
        .iter()
        .find(|path| path.ends_with("watermark-images.bnto.json"))
        .unwrap();
    assert_eq!(
        lines,
        [
            format!(
                "{watermark}: note: node \"overlay\" (image-overlay), param \"overlay\": \
                 left for run time; give it with bnto run -p overlay:overlay=<path>"
            ),
            format!("Check: PASS ({BUNDLED_RECIPES} recipes, 1 note)"),
        ]
    );
}

/// Fixtures that leave a required param for the test to pass with
/// `--params`, so `bnto check` notes it as left for run time.
const NEEDS_PARAMS: [&str; 1] = ["spreadsheet-lookup-rename-move.bnto.json"];

#[test]
fn test_check_passes_every_run_fixture() {
    let fixtures = crates_dir().join("bnto/tests/fixtures");
    let mut paths = recipes_in(&fixtures);
    paths.extend(recipes_in(&fixtures.join("explicit")));
    paths.retain(|p| !NEEDS_PARAMS.iter().any(|name| p.ends_with(name)));
    let (ok, lines) = run_check(&paths);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(lines, [format!("Check: PASS ({} recipes)", paths.len())]);
}

#[test]
fn test_a_fixture_that_needs_params_passes_with_a_note() {
    for name in NEEDS_PARAMS {
        let path = crates_dir().join("bnto/tests/fixtures").join(name);
        let (ok, lines) = run_check(&[path.to_string_lossy().into_owned()]);
        assert!(ok, "stdout: {lines:?}");
        assert_eq!(lines.len(), 2, "{lines:?}");
        assert!(lines[0].contains("note:"), "{lines:?}");
        assert_eq!(lines[1], "Check: PASS (1 recipe, 1 note)");
    }
}
