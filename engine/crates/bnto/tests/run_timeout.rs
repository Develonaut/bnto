// The run's time limit through the built binary: a recipe's
// `settings.timeout` or `bnto run --timeout` stops a run whose step hangs
// (here a shell-command with no timeout of its own), kills the command it
// started, and fails with one line naming the limit.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use helpers::{bnto_bin, temp_output_dir};

/// Far longer than a stopped run may take to return on a loaded machine,
/// and far shorter than the hung command would run on its own.
const STOP_DEADLINE: Duration = Duration::from_secs(60);

/// How long to wait for a killed command to leave the process table.
const GONE_DEADLINE: Duration = Duration::from_secs(10);

/// A command that runs for `secs` seconds and can be found again by its
/// arguments: `ping -n <secs>` on Windows, `sleep <secs>` elsewhere.
fn hang(secs: u32) -> (&'static str, Vec<String>) {
    if cfg!(windows) {
        let args = ["-n", &secs.to_string(), "127.0.0.1"];
        ("ping", args.map(String::from).to_vec())
    } else {
        ("sleep", vec![secs.to_string()])
    }
}

/// A run time no other test's hang uses, so each test finds its own command.
fn unique_secs(test: u32) -> u32 {
    3000 + (std::process::id() % 1000) * 10 + test
}

/// Write a recipe whose one step hangs for `secs`, with `settings` at its root.
fn hung_recipe(dir: &Path, secs: u32, settings: serde_json::Value) -> String {
    let (command, args) = hang(secs);
    let body = serde_json::json!({
        "settings": settings,
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "hang", "type": "shell-command",
              "parameters": { "command": command, "args": args } },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]
    });
    let recipe = dir.join("hang.bnto.json");
    std::fs::write(&recipe, body.to_string()).unwrap();
    recipe.to_string_lossy().into_owned()
}

/// `bnto run <recipe> --yes -o <tmp> <extra...>`.
fn run(recipe: &str, extra: &[&str]) -> Output {
    let out = temp_output_dir();
    Command::new(bnto_bin())
        .args(["run", recipe, "--yes", "-o", out.path().to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap()
}

/// The last non-empty line `output` printed on stderr.
fn last_stderr_line(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stderr.lines().rev().find(|l| !l.trim().is_empty());
    line.unwrap_or_default().trim().to_string()
}

/// Whether the hang command running for `secs` is still alive.
fn hang_alive(secs: u32) -> bool {
    if cfg!(windows) {
        let filter = "Get-CimInstance Win32_Process -Filter \"Name='PING.EXE'\" \
                      | ForEach-Object { $_.CommandLine }";
        let out = Command::new("powershell")
            .args(["-NoProfile", "-Command", filter])
            .output()
            .unwrap();
        let lines = String::from_utf8_lossy(&out.stdout).into_owned();
        lines.contains(&format!("-n {secs} 127.0.0.1"))
    } else {
        let status = Command::new("pgrep")
            .args(["-f", &format!("sleep {secs}$")])
            .status()
            .unwrap();
        status.success()
    }
}

/// Assert the hang command for `secs` leaves the process table.
fn assert_hang_gone(secs: u32) {
    let started = Instant::now();
    while started.elapsed() < GONE_DEADLINE {
        if !hang_alive(secs) {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("the hung command (running {secs}s) is still alive");
}

#[test]
fn test_a_run_past_its_recipe_time_limit_is_stopped() {
    let dir = temp_output_dir();
    let secs = unique_secs(1);
    let recipe = hung_recipe(dir.path(), secs, serde_json::json!({ "timeout": 2 }));
    let started = Instant::now();
    let output = run(&recipe, &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(started.elapsed() < STOP_DEADLINE, "it was stopped");
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert_eq!(
        last_stderr_line(&output),
        "Pipeline failed: Run timed out after 2s",
        "stderr: {stderr}"
    );
    assert_hang_gone(secs);
}

#[test]
fn test_timeout_flag_overrides_the_recipes_limit() {
    let dir = temp_output_dir();
    let secs = unique_secs(2);
    let recipe = hung_recipe(dir.path(), secs, serde_json::json!({ "timeout": 600 }));
    let started = Instant::now();
    let output = run(&recipe, &["--timeout", "2"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(started.elapsed() < STOP_DEADLINE, "it was stopped");
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert_eq!(
        last_stderr_line(&output),
        "Pipeline failed: Run timed out after 2s",
        "stderr: {stderr}"
    );
    assert_hang_gone(secs);
}

#[test]
fn test_timeout_flag_limits_a_recipe_with_no_limit() {
    let dir = temp_output_dir();
    let secs = unique_secs(3);
    let recipe = hung_recipe(dir.path(), secs, serde_json::json!({}));
    let output = run(&recipe, &["--timeout", "1"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert_eq!(
        last_stderr_line(&output),
        "Pipeline failed: Run timed out after 1s",
        "stderr: {stderr}"
    );
    assert_hang_gone(secs);
}

#[test]
fn test_json_reports_a_timed_out_run() {
    let dir = temp_output_dir();
    let secs = unique_secs(4);
    let recipe = hung_recipe(dir.path(), secs, serde_json::json!({ "timeout": 1 }));
    let output = run(&recipe, &["--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "stdout: {stdout}");
    let summary: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(summary["status"], "fail");
    assert_eq!(summary["error"], "Run timed out after 1s");
    assert_eq!(summary["nodes"][0]["id"], "hang");
    assert_hang_gone(secs);
}

#[test]
fn test_a_run_within_its_limit_passes() {
    let dir = temp_output_dir();
    let recipe = dir.path().join("quick.bnto.json");
    let body = serde_json::json!({
        "settings": { "timeout": 60 },
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "quick", "type": "shell-command",
              "parameters": { "command": "git", "args": ["--version"] } },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]
    });
    std::fs::write(&recipe, body.to_string()).unwrap();
    let output = run(recipe.to_str().unwrap(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
}

#[test]
fn test_timeout_flag_must_be_seconds_above_zero() {
    let dir = temp_output_dir();
    let recipe = hung_recipe(dir.path(), unique_secs(5), serde_json::json!({}));
    let output = run(&recipe, &["--timeout", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "stderr: {stderr}");
    assert!(stderr.contains("--timeout"), "stderr: {stderr}");
}

#[test]
fn test_check_refuses_a_time_limit_of_zero() {
    let dir = temp_output_dir();
    let recipe = hung_recipe(
        dir.path(),
        unique_secs(6),
        serde_json::json!({ "timeout": 0 }),
    );
    let output = Command::new(bnto_bin())
        .args(["check", &recipe])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "stdout: {stdout}");
    assert!(
        stdout.contains("settings.timeout must be a whole number of seconds above 0"),
        "stdout: {stdout}"
    );
}
