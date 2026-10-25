// file-delete through the built binary — file-list picks paths under a
// folder, file-filter keeps the .log files, file-delete removes them.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

/// A root folder holding `a.log`, `keep.txt` and `logs/b.log`.
fn root_fixture() -> tempfile::TempDir {
    let root = temp_output_dir();
    std::fs::create_dir_all(root.path().join("logs")).unwrap();
    for name in ["a.log", "keep.txt", "logs/b.log"] {
        std::fs::write(root.path().join(name), name).unwrap();
    }
    root
}

/// Run `file-list-filter-delete` with `Root` set to `root`, plus `extra` args.
fn run_delete(root: &Path, extra: &[&str]) -> Output {
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let body = serde_json::json!({ "Root": root.to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    Command::new(bnto_bin())
        .args([
            "run",
            &custom_recipe_path("file-list-filter-delete"),
            "--yes",
        ])
        .args(["--vars", vars_file.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap()
}

fn untouched(root: &Path) -> bool {
    ["a.log", "keep.txt", "logs/b.log"]
        .iter()
        .all(|n| root.join(n).exists())
}

#[test]
fn test_the_filtered_files_are_deleted_and_counted() {
    let root = root_fixture();
    let output = run_delete(root.path(), &["--json"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(!root.path().join("a.log").exists());
    assert!(!root.path().join("logs/b.log").exists());
    assert!(root.path().join("keep.txt").exists());
    assert!(stderr.contains("Deleted 2 files under"), "stderr: {stderr}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let summary: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let nodes = summary["nodes"].as_array().unwrap();
    let delete = nodes.iter().find(|n| n["id"] == "delete").unwrap();
    assert_eq!(
        delete["metadata"],
        serde_json::json!({ "deleted": 2, "skipped": 0 })
    );
}

#[test]
fn test_a_dry_run_deletes_nothing() {
    let root = root_fixture();
    let output = run_delete(root.path(), &["--dry-run"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(untouched(root.path()));
    assert!(stderr.contains("Would delete 2 files"), "stderr: {stderr}");
}

#[test]
fn test_a_file_outside_base_refuses_the_run_and_deletes_nothing() {
    // `base` narrowed to `logs`, so `a.log` at the root is outside it.
    let root = root_fixture();
    let base = root.path().join("logs");
    let param = format!("delete:base={}", base.to_str().unwrap());
    let output = run_delete(root.path(), &["-p", &param]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("a.log"), "names the item: {stderr}");
    assert!(
        stderr.contains("outside the base folder"),
        "stderr: {stderr}"
    );
    assert!(untouched(root.path()));
}

#[test]
fn test_check_leaves_an_empty_file_delete_base_for_run_time() {
    // A required param given "" is left out, as a run's pre-flight treats it.
    let recipe = custom_recipe_path("check/file-delete-empty-base");
    let output = Command::new(bnto_bin())
        .args(["check", &recipe])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "stdout: {stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines.last(),
        Some(&"Check: PASS (1 recipe, 1 note)"),
        "{stdout}"
    );
    assert!(
        lines[0].contains("\"empty-base\"")
            && lines[0].contains("param \"base\": left for run time"),
        "{stdout}"
    );
}
