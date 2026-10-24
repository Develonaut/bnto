// `bnto dry-run` takes a recipe slug or a `.bnto.json` path, as `bnto run`
// does, so the `bnto dry-run <path>` a declined consent suggests works (ISS-11).

mod helpers;

use std::process::{Output, Stdio};

use helpers::{Sandbox, custom_recipe_path, fixture_image};

const SHELL_RECIPE: &str = "shell-file-input.bnto.json";

/// A group declaring `version` with no default, around a shell step using it.
const UNSET_VERSION: &str = r#"{ "id": "unset-version", "name": "Unset Version",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "checks", "type": "group",
    "fields": { "version": { "type": "string", "label": "Version" } },
    "nodes": [{ "id": "crates", "type": "shell-run",
      "parameters": { "command": "echo", "args": ["{{fields.version}}"] } }] }] }"#;

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Copy the shell-run fixture into the sandbox's working directory.
fn shell_recipe_in_work_dir(sandbox: &Sandbox) {
    let to = sandbox.work_dir().join(SHELL_RECIPE);
    std::fs::copy(custom_recipe_path("shell-file-input"), to).unwrap();
}

fn dry_run(sandbox: &Sandbox, args: &[&str]) -> Output {
    sandbox
        .command()
        .arg("dry-run")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

/// Assert a static preview of the shell-run fixture succeeded.
fn assert_shell_preview(output: &Output) {
    let (out, err) = (stdout(output), stderr(output));
    assert!(output.status.success(), "stderr: {err}");
    assert!(out.contains("Shell File Input"), "stdout: {out}");
    assert!(out.contains("git"), "stdout: {out}");
}

#[test]
fn dry_run_previews_a_recipe_by_absolute_path() {
    let sandbox = Sandbox::new();
    let output = dry_run(&sandbox, &[&custom_recipe_path("shell-file-input")]);
    assert_shell_preview(&output);
}

#[test]
fn dry_run_previews_a_recipe_by_relative_path() {
    let sandbox = Sandbox::new();
    shell_recipe_in_work_dir(&sandbox);
    let output = dry_run(&sandbox, &[SHELL_RECIPE]);
    assert_shell_preview(&output);
}

#[test]
fn the_dry_run_a_declined_run_suggests_works() {
    let sandbox = Sandbox::new();
    shell_recipe_in_work_dir(&sandbox);
    let declined = sandbox
        .command()
        .args(["run", SHELL_RECIPE, &fixture_image("small.jpg")])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let err = stderr(&declined);
    assert_eq!(declined.status.code(), Some(1), "{err}");
    let suggested = err
        .split("Run bnto dry-run ")
        .nth(1)
        .and_then(|rest| rest.split(" (without input files)").next())
        .unwrap_or_else(|| panic!("no dry-run suggestion: {err}"));
    assert_eq!(suggested, SHELL_RECIPE);
    assert_shell_preview(&dry_run(&sandbox, &[suggested]));
}

#[test]
fn a_slug_wins_over_a_file_of_the_same_name() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("compress-images"), "not json").unwrap();
    let output = dry_run(&sandbox, &["compress-images"]);
    let out = stdout(&output);
    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(out.contains("Compress Images"), "stdout: {out}");
}

#[test]
fn dry_run_with_inputs_asks_consent_for_a_path_recipe() {
    let sandbox = Sandbox::new();
    shell_recipe_in_work_dir(&sandbox);
    let output = dry_run(&sandbox, &[SHELL_RECIPE, &fixture_image("small.jpg")]);
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("Execution cancelled"), "{err}");
}

#[test]
fn dry_run_with_inputs_and_yes_previews_a_path_recipe() {
    let sandbox = Sandbox::new();
    shell_recipe_in_work_dir(&sandbox);
    let image = fixture_image("small.jpg");
    let output = dry_run(&sandbox, &[SHELL_RECIPE, &image, "--yes"]);
    let (out, err) = (stdout(&output), stderr(&output));
    assert!(output.status.success(), "stderr: {err}");
    assert!(!err.contains("preview failed"), "stderr: {err}");
    // The shell step's stdout replaces the file, then the rename prefixes it.
    assert!(out.contains("checked-git-output"), "stdout: {out}");
}

#[test]
fn dry_run_refuses_an_unset_field_in_a_path_recipe() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("unset.bnto.json"), UNSET_VERSION).unwrap();
    let output = dry_run(&sandbox, &["unset.bnto.json"]);
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(
        err.contains("field \"version\" (node \"checks\") is unset: pass -p version=..."),
        "{err}"
    );
}

#[test]
fn dry_run_of_a_missing_file_exits_1() {
    let sandbox = Sandbox::new();
    let output = dry_run(&sandbox, &["missing.bnto.json"]);
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("Unknown recipe: missing.bnto.json"), "{err}");
}

#[test]
fn dry_run_of_invalid_json_exits_1() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("broken.bnto.json"), "{ not json").unwrap();
    let output = dry_run(&sandbox, &["broken.bnto.json"]);
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("Invalid recipe JSON"), "{err}");
}
