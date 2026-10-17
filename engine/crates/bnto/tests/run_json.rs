// `bnto run --json` tests — one summary line on stdout for every outcome,
// with stderr's human summary line unchanged and still last.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

/// Run `file-copy-newer` on `src`, copying to `dest`, writing results to `out`.
fn run_copy(src: &Path, dest: &Path, out: &Path, extra: &[&str]) -> Output {
    let dest_param = format!("copy:destination={}", dest.to_str().unwrap());
    Command::new(bnto_bin())
        .args(["run", &custom_recipe_path("file-copy-newer")])
        .arg(src)
        .args(["--yes", "-p", &dest_param])
        .args(["-o", out.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap()
}

/// A source dir holding `report.txt`.
fn source_dir() -> tempfile::TempDir {
    let src = temp_output_dir();
    std::fs::write(src.path().join("report.txt"), "from source").unwrap();
    src
}

/// stdout as exactly one JSON line, parsed.
fn single_json_line(output: &Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "stdout: {stdout:?}");
    serde_json::from_str(lines[0]).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

fn last_stderr_line(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let last = stderr.lines().rev().find(|l| !l.trim().is_empty());
    last.unwrap_or_default().to_string()
}

/// A path that can't be created as a directory: it sits under a regular file.
fn uncreatable(tmp: &tempfile::TempDir) -> std::path::PathBuf {
    let file = tmp.path().join("not-a-dir");
    std::fs::write(&file, b"").unwrap();
    file.join("sub")
}

#[test]
fn test_run_json_prints_one_pass_line() {
    let (src, dest, out) = (source_dir(), temp_output_dir(), temp_output_dir());
    let output = run_copy(src.path(), dest.path(), out.path(), &["--json"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let summary = single_json_line(&output);
    assert_eq!(summary["status"], "pass");
    assert_eq!(summary["files_written"], 1);
    assert_eq!(summary["error"], serde_json::Value::Null);
    assert!(summary["duration_ms"].is_u64(), "summary: {summary}");
    let nodes = summary["nodes"].as_array().unwrap();
    let ids: Vec<_> = nodes.iter().map(|n| (&n["id"], &n["node_type"])).collect();
    assert_eq!(
        ids,
        [
            (&"collect".into(), &"file-collect".into()),
            (&"copy".into(), &"file-copy".into())
        ]
    );
    assert_eq!(nodes[1]["files_processed"], 1);
    assert!(
        last_stderr_line(&output).starts_with("Done."),
        "stderr: {stderr}"
    );
}

#[test]
fn test_run_json_pipeline_failure_prints_one_fail_line() {
    let (src, out, tmp) = (source_dir(), temp_output_dir(), temp_output_dir());
    let output = run_copy(src.path(), &uncreatable(&tmp), out.path(), &["--json"]);
    assert_eq!(output.status.code(), Some(1));
    let summary = single_json_line(&output);
    assert_eq!(summary["status"], "fail");
    let error = summary["error"].as_str().unwrap();
    let last = last_stderr_line(&output);
    assert_eq!(last, format!("Pipeline failed: {error}"));
}

#[test]
fn test_run_json_unresolved_var_prints_one_fail_line() {
    let (src, out) = (source_dir(), temp_output_dir());
    let output = Command::new(bnto_bin())
        .args(["run", &custom_recipe_path("file-copy-vars")])
        .arg(src.path())
        .args(["--yes", "--json", "-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let summary = single_json_line(&output);
    assert_eq!(summary["status"], "fail");
    let error = summary["error"].as_str().unwrap();
    assert!(error.contains("unresolved {{vars.Out}}"), "error: {error}");
    assert_eq!(
        last_stderr_line(&output),
        format!("Pipeline failed: {error}")
    );
}

#[test]
fn test_run_json_output_write_failure_prints_one_fail_line() {
    let (src, dest, tmp) = (source_dir(), temp_output_dir(), temp_output_dir());
    let output = run_copy(src.path(), dest.path(), &uncreatable(&tmp), &["--json"]);
    assert_eq!(output.status.code(), Some(1));
    let summary = single_json_line(&output);
    assert_eq!(summary["status"], "fail");
    let error = summary["error"].as_str().unwrap();
    assert_eq!(
        last_stderr_line(&output),
        format!("Error writing output: {error}")
    );
}

/// A source and destination where the destination's `report.txt` is newer.
fn newer_at_destination() -> (tempfile::TempDir, tempfile::TempDir) {
    let (src, dest) = (source_dir(), temp_output_dir());
    let kept = dest.path().join("report.txt");
    std::fs::write(&kept, "kept at destination").unwrap();
    let now = filetime::FileTime::now();
    let newer = filetime::FileTime::from_unix_time(now.unix_seconds() + 10, 0);
    filetime::set_file_mtime(&kept, newer).unwrap();
    (src, dest)
}

#[test]
fn test_run_with_a_warning_ends_stderr_with_done() {
    for extra in [&[][..], &["--json"][..]] {
        let (src, dest) = newer_at_destination();
        let out = temp_output_dir();
        let output = run_copy(src.path(), dest.path(), out.path(), extra);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "stderr: {stderr}");
        assert!(stderr.contains("Warning:"), "stderr: {stderr}");
        let last = last_stderr_line(&output);
        assert!(
            last.starts_with("Done. 1 file written to "),
            "{extra:?}: {last}"
        );
        if extra.is_empty() {
            assert!(output.stdout.is_empty(), "stdout without --json");
        } else {
            assert_eq!(single_json_line(&output)["status"], "pass");
        }
    }
}
