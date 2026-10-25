// `bnto dry-run --vars` fills `{{vars.KEY}}` as `bnto run --vars` does, so
// the preview shows the plan the run would follow (ISS-40).

mod helpers;

use std::process::{Command, Output, Stdio};

use helpers::Sandbox;

/// A shell step whose arg reads `{{vars.Drive}}`.
const DRIVE_RECIPE: &str = r#"{ "id": "drive", "name": "Drive",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "list", "type": "shell-run",
      "parameters": { "command": "echo", "args": ["{{vars.Drive}}/Art"] } }] }"#;

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// DRIVE_RECIPE and a vars file setting `Drive` to `D:` in the sandbox.
fn drive_sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("drive.bnto.json"), DRIVE_RECIPE).unwrap();
    std::fs::write(sandbox.work_dir().join("vars.json"), r#"{"Drive": "D:"}"#).unwrap();
    sandbox
}

/// `bnto dry-run drive.bnto.json <args>`, with no `BNTO_VAR_DRIVE` inherited.
fn dry_run(sandbox: &Sandbox, args: &[&str]) -> Command {
    let mut cmd = sandbox.command();
    cmd.args(["dry-run", "drive.bnto.json"])
        .args(args)
        .env_remove("BNTO_VAR_DRIVE")
        .stdin(Stdio::null());
    cmd
}

#[test]
fn dry_run_shows_the_value_a_vars_file_sets() {
    let sandbox = drive_sandbox();
    let output = dry_run(&sandbox, &["--vars", "vars.json"])
        .output()
        .unwrap();
    let out = stdout(&output);
    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(out.contains("$ echo D:/Art"), "{out}");
}

#[test]
fn dry_run_lets_an_env_var_beat_the_vars_file() {
    let sandbox = drive_sandbox();
    let mut cmd = dry_run(&sandbox, &["--vars", "vars.json"]);
    let output = cmd.env("BNTO_VAR_DRIVE", "E:").output().unwrap();
    let out = stdout(&output);
    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(out.contains("$ echo E:/Art"), "{out}");
}

#[test]
fn dry_run_refuses_an_unset_var_as_the_run_would() {
    let sandbox = drive_sandbox();
    let output = dry_run(&sandbox, &[]).output().unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "stdout: {}", stdout(&output));
    assert!(
        err.contains("unresolved {{vars.Drive}} (no --vars file or BNTO_VAR_DRIVE sets it)"),
        "{err}"
    );
}

/// The `bnto dry-run …` line a declined command (stdin closed) suggests.
fn declined_suggestion(output: &Output) -> String {
    let err = stderr(output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    let line = err
        .split("Run bnto dry-run ")
        .nth(1)
        .and_then(|rest| rest.split(" (without input files)").next())
        .unwrap_or_else(|| panic!("no dry-run suggestion: {err}"));
    format!("bnto dry-run {line}")
}

#[test]
fn a_declined_runs_suggestion_carries_its_vars_and_previews() {
    let sandbox = drive_sandbox();
    let mut run = sandbox.command();
    run.args(["run", "drive.bnto.json", "--vars", "vars.json"]);
    let declined = run.env_remove("BNTO_VAR_DRIVE").stdin(Stdio::null());
    let suggested = declined_suggestion(&declined.output().unwrap());
    assert_eq!(suggested, "bnto dry-run drive.bnto.json --vars vars.json");
    let words: Vec<&str> = suggested.split_whitespace().skip(3).collect();
    let output = dry_run(&sandbox, &words).output().unwrap();
    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(
        stdout(&output).contains("$ echo D:/Art"),
        "{}",
        stdout(&output)
    );
}

#[test]
fn a_dry_run_with_inputs_asks_consent_for_the_filled_command() {
    // With inputs the preview runs the pipeline, so it asks consent first;
    // the prompt and the suggestion it ends with both carry the vars.
    let sandbox = drive_sandbox();
    std::fs::write(sandbox.work_dir().join("in.txt"), "x").unwrap();
    let output = dry_run(&sandbox, &["--vars", "vars.json", "in.txt"])
        .output()
        .unwrap();
    assert!(
        stderr(&output).contains("echo D:/Art"),
        "{}",
        stderr(&output)
    );
    let suggested = declined_suggestion(&output);
    assert_eq!(suggested, "bnto dry-run drive.bnto.json --vars vars.json");
}
