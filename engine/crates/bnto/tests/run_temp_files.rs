// Real-run temp-file hygiene — the output step copies results out, so a run
// must delete the temp files its nodes made, and never touch the user's files.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

/// Run fixture `slug` with the system temp dir pointed at `tmp`.
fn run_fixture(slug: &str, tmp: &Path, out: &Path, extra: &[&str]) -> Output {
    Command::new(bnto_bin())
        .env("TMPDIR", tmp)
        .env("TMP", tmp)
        .env("TEMP", tmp)
        .args(["run", &custom_recipe_path(slug), "--yes"])
        .args(extra)
        .args(["-o", out.to_str().unwrap()])
        .output()
        .unwrap()
}

/// Every entry left directly under `dir`, for a readable assertion message.
fn entries(dir: &Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn test_run_writes_shell_output_and_leaves_no_temp_files() {
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let output = run_fixture("shell-file-output", tmp.path(), out.path(), &[]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let note = std::fs::read_to_string(out.path().join("note.cfg")).unwrap();
    assert!(note.contains("test = yes"), "note.cfg: {note}");
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_failed_run_leaves_no_temp_files() {
    // The shell node creates its temp output dir, writes nothing, and the
    // pipeline fails — the error exit must still clean up.
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let output = run_fixture("shell-file-output-fails", tmp.path(), out.path(), &[]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_run_whose_write_step_fails_leaves_no_temp_files() {
    // The pipeline succeeds but the output dir can't be created (its parent
    // is a regular file), so the write step exits the run with an error.
    let tmp = temp_output_dir();
    let out_root = temp_output_dir();
    let file = out_root.path().join("not-a-dir");
    std::fs::write(&file, b"").unwrap();
    let output = run_fixture("shell-file-output", tmp.path(), &file.join("out"), &[]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("Error writing output"), "stderr: {stderr}");
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_run_copies_listed_user_files_and_keeps_the_originals() {
    // file-list hands the output step path references to the user's own
    // files: they must be copied, never moved or deleted with the temp files.
    let root = temp_output_dir();
    std::fs::write(root.path().join("a.txt"), "alpha").unwrap();
    std::fs::write(root.path().join("b.txt"), "beta").unwrap();
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let body = serde_json::json!({ "Root": root.path().to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let vars_arg = ["--vars", vars_file.to_str().unwrap()];
    let output = run_fixture("file-list-write", tmp.path(), out.path(), &vars_arg);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    for (name, body) in [("a.txt", "alpha"), ("b.txt", "beta")] {
        assert_eq!(
            std::fs::read_to_string(root.path().join(name)).unwrap(),
            body
        );
        assert_eq!(
            std::fs::read_to_string(out.path().join(name)).unwrap(),
            body
        );
    }
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}
