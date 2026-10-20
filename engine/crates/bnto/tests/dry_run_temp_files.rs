// Dry-run temp-file hygiene — a dry run executes shell-run nodes but must
// leave none of their temp output behind, while a real run still delivers it.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, fixture_image, temp_output_dir};

/// Run the shell-file-output fixture with the system temp dir pointed at `tmp`.
fn run_shell_file_output(tmp: &Path, out: &Path, extra: &[&str]) -> Output {
    run_fixture("shell-file-output", tmp, out, extra)
}

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
fn test_run_dry_run_leaves_no_shell_temp_output() {
    let tmp = temp_output_dir();
    let out_root = temp_output_dir();
    let out = out_root.path().join("not-yet");
    let output = run_shell_file_output(tmp.path(), &out, &["--dry-run"]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("would be written"), "stderr: {stderr}");
    assert!(!out.exists(), "output dir was created");
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_failed_dry_run_leaves_no_shell_temp_output() {
    // The shell node creates its temp output dir, writes nothing, and the
    // pipeline fails — the error exit must still clean up.
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let output = run_fixture(
        "shell-file-output-fails",
        tmp.path(),
        out.path(),
        &["--dry-run"],
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("produced no output files"),
        "stderr: {stderr}"
    );
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_run_without_dry_run_still_writes_shell_output() {
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let output = run_shell_file_output(tmp.path(), out.path(), &[]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let note = std::fs::read_to_string(out.path().join("note.cfg")).unwrap();
    assert!(note.contains("test = yes"), "note.cfg: {note}");
    assert!(
        !stderr.contains("dry run still executes"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_run_dry_run_notes_shell_commands_still_execute() {
    let tmp = temp_output_dir();
    let out = temp_output_dir();
    let output = run_shell_file_output(tmp.path(), out.path(), &["--dry-run"]);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("Note: dry run still executes 1 shell command"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("1 shell commands"), "stderr: {stderr}");
}

#[test]
fn test_run_dry_run_without_shell_commands_has_no_note() {
    let src = temp_output_dir();
    std::fs::write(src.path().join("a.txt"), "a").unwrap();
    let out = temp_output_dir();
    let dest_param = format!("copy:destination={}", out.path().display());
    let output = Command::new(bnto_bin())
        .args(["run", &custom_recipe_path("file-copy-newer")])
        .arg(src.path())
        .args(["--yes", "--dry-run", "-p", &dest_param])
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        !stderr.contains("dry run still executes"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_dry_run_subcommand_with_inputs_notes_shell_commands() {
    // `bnto dry-run` with inputs executes the pipeline too.
    let home = temp_output_dir();
    let library = home.path().join("recipes");
    std::fs::create_dir_all(&library).unwrap();
    std::fs::copy(
        custom_recipe_path("shell-file-input"),
        library.join("shell-file-input.bnto.json"),
    )
    .unwrap();
    let output = Command::new(bnto_bin())
        .env("BNTO_HOME", home.path())
        .args(["dry-run", "shell-file-input", "--yes"])
        .arg(fixture_image("small.jpg"))
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("dry run still executes 1 shell command"),
        "stderr: {stderr}"
    );
}
