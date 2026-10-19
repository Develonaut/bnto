// shell-command's `env`, `timeout` and `cwd` through the built binary: a
// variable in `env` reaches the command, a command that outlives its
// `timeout` is stopped and fails the run naming the limit, and `cwd` runs
// the command in a folder under the working directory.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use helpers::{bnto_bin, temp_output_dir};

/// Far longer than a stopped command may take to return on a loaded machine.
const STOP_DEADLINE: Duration = Duration::from_secs(60);

/// Write a recipe that runs one shell-command with `params`, run it with
/// `--yes`, writing to `out`.
fn run_shell(params: serde_json::Value, out: &Path) -> Output {
    run_shell_in(None, params, out)
}

/// `run_shell`, run from `work` when given (else the test's own folder).
fn run_shell_in(work: Option<&Path>, params: serde_json::Value, out: &Path) -> Output {
    let dir = temp_output_dir();
    let recipe = dir.path().join("shell-options.bnto.json");
    let body = serde_json::json!({
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "tool", "type": "shell-command", "parameters": params },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]
    });
    std::fs::write(&recipe, body.to_string()).unwrap();
    let mut command = Command::new(bnto_bin());
    command
        .args(["run", recipe.to_str().unwrap(), "--yes"])
        .args(["-o", out.to_str().unwrap()]);
    if let Some(work) = work {
        command.current_dir(work);
    }
    command.output().unwrap()
}

/// The text of the one file the run wrote to `out`.
fn only_output(out: &Path) -> String {
    let files: Vec<_> = std::fs::read_dir(out)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(files.len(), 1, "files: {files:?}");
    std::fs::read_to_string(&files[0]).unwrap()
}

#[test]
fn test_an_env_var_reaches_the_command() {
    // `git var GIT_AUTHOR_IDENT` echoes GIT_AUTHOR_NAME and _EMAIL back.
    let out = temp_output_dir();
    let output = run_shell(
        serde_json::json!({
            "command": "git",
            "args": ["var", "GIT_AUTHOR_IDENT"],
            "env": { "GIT_AUTHOR_NAME": "Env Reached", "GIT_AUTHOR_EMAIL": "env@example.com" }
        }),
        out.path(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let ident = only_output(out.path());
    assert!(
        ident.starts_with("Env Reached <env@example.com>"),
        "ident: {ident}"
    );
}

#[test]
fn test_a_command_that_outlives_its_timeout_is_stopped() {
    let (command, args) = if cfg!(windows) {
        ("ping", serde_json::json!(["-n", "300", "127.0.0.1"]))
    } else {
        ("sleep", serde_json::json!(["300"]))
    };
    let out = temp_output_dir();
    let started = Instant::now();
    let output = run_shell(
        serde_json::json!({ "command": command, "args": args, "timeout": 1 }),
        out.path(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        started.elapsed() < STOP_DEADLINE,
        "it was stopped, not waited out"
    );
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(
        stderr.contains(&format!("Command '{command}' timed out after 1s")),
        "stderr: {stderr}"
    );
}

/// A git repo with an empty `Tools` folder, to run `git` from.
fn repo_with_tools() -> tempfile::TempDir {
    let work = temp_output_dir();
    let init = Command::new("git")
        .args(["init", "-q"])
        .current_dir(work.path())
        .status()
        .unwrap();
    assert!(init.success());
    std::fs::create_dir(work.path().join("Tools")).unwrap();
    work
}

#[test]
fn test_cwd_runs_the_command_in_a_folder_under_the_work_dir() {
    // `git rev-parse --show-prefix` prints where it runs, relative to the repo.
    let (work, out) = (repo_with_tools(), temp_output_dir());
    let params = serde_json::json!({
        "command": "git", "args": ["rev-parse", "--show-prefix"], "cwd": "Tools"
    });
    let output = run_shell_in(Some(work.path()), params, out.path());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(only_output(out.path()).trim(), "Tools/");
}

#[test]
fn test_a_cwd_outside_the_work_dir_refuses_the_run() {
    let (work, out) = (repo_with_tools(), temp_output_dir());
    let params = serde_json::json!({
        "command": "git", "args": ["rev-parse", "--show-prefix"], "cwd": "../elsewhere"
    });
    let output = run_shell_in(Some(work.path()), params, out.path());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(
        stderr.contains("'cwd' must be a folder under the working directory"),
        "stderr: {stderr}"
    );
}
