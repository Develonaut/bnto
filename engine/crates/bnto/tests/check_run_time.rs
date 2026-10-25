// `bnto check` on recipes that leave a required param for `bnto run -p`:
// passes with a note when a run can give it, fails when none can, and the
// run itself still fails naming the param when it isn't given.

mod helpers;

use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, fixture_csv, temp_output_dir};

/// Run `bnto check` on `path`: (exit ok, stdout lines).
fn run_check(path: &str) -> (bool, Vec<String>) {
    let output = Command::new(bnto_bin())
        .args(["check", path])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().map(str::to_string).collect();
    (output.status.success(), lines)
}

fn run(args: &[&str]) -> Output {
    Command::new(bnto_bin())
        .arg("run")
        .args(args)
        .output()
        .unwrap()
}

// --- check ---

#[test]
fn test_check_passes_file_delete_with_base_left_for_run_time() {
    let path = custom_recipe_path("check/file-delete-run-time");
    let (ok, lines) = run_check(&path);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(
        lines,
        [
            format!(
                "{path}: note: node \"clean\" (file-delete), param \"base\": left for run time; \
                 give it with bnto run -p clean:base=<value>"
            ),
            "Check: PASS (1 recipe, 1 note)".to_string(),
        ]
    );
}

#[test]
fn test_check_passes_spreadsheet_lookup_in_a_loop_with_table_left_for_run_time() {
    let path = custom_recipe_path("check/lookup-run-time");
    let (ok, lines) = run_check(&path);
    assert!(ok, "stdout: {lines:?}");
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(
        lines[0].contains("note: node \"lookup\" (spreadsheet-lookup), param \"table\""),
        "{lines:?}"
    );
    assert_eq!(lines[1], "Check: PASS (1 recipe, 1 note)");
}

#[test]
fn test_check_fails_params_no_run_can_give() {
    let path = custom_recipe_path("check/no-run-can-give");
    let (ok, lines) = run_check(&path);
    assert!(!ok, "stdout: {lines:?}");
    assert_eq!(lines.len(), 5, "{lines:?}");
    assert!(lines[0].contains("\"set\"") && lines[0].contains("'values' is required"));
    assert!(
        lines[1].contains("param \"values\"") && lines[1].contains("no run can give it"),
        "{lines:?}"
    );
    assert!(lines[2].contains("\"clean:up\"") && lines[2].contains("'base' is required"));
    assert!(
        lines[3].contains("param \"base\"") && lines[3].contains("no run can give it"),
        "{lines:?}"
    );
    assert_eq!(lines[4], "Check: FAIL (4 findings)");
}

// --- run ---

#[test]
fn test_run_without_the_param_fails_naming_it() {
    let output = run(&[&custom_recipe_path("check/file-delete-run-time"), "--yes"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    // Pre-flight refuses it before any step runs.
    assert!(
        stderr.contains("Node 'clean': param \"base\" (Base Folder) is required"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_run_with_the_param_given_succeeds() {
    let base = temp_output_dir();
    let param = format!("clean:base={}", base.path().to_str().unwrap());
    let recipe = custom_recipe_path("check/file-delete-run-time");
    let output = run(&[&recipe, "--yes", "-p", &param]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
}

#[test]
fn test_run_without_the_table_fails_naming_it() {
    let out = temp_output_dir();
    let recipe = custom_recipe_path("check/lookup-run-time");
    let csv = fixture_csv("manifest.csv");
    let output = run(&[&recipe, &csv, "-o", out.path().to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("Node 'lookup': param \"table\" (Table) is required"),
        "stderr: {stderr}"
    );
}
