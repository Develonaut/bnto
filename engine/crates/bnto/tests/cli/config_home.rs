// ISS-127: `config.toml`'s `paths.home` moves the bnto home for every
// surface. `bnto run` keeps its approvals in the moved home's trust file,
// where the TUI reads and writes them, and a run reads the moved home's
// `.env` and fills `{{ctx.paths.output_dir}}` from it.

use crate::helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::Sandbox;

/// The env var the output recipe below names its folder by.
const KEY: &str = "BNTO_ISS127_DIR";

/// A sandbox whose default home's `config.toml` moves the home to `moved`.
fn sandbox_with_moved_home() -> (Sandbox, tempfile::TempDir) {
    let sandbox = Sandbox::new();
    let moved = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(sandbox.bnto_home()).unwrap();
    // A TOML literal string, so a Windows path's `\` stays as written.
    let config = format!(
        "version = 3\n\n[paths]\nhome = '{}'\n",
        moved.path().display()
    );
    std::fs::write(sandbox.bnto_home().join("config.toml"), config).unwrap();
    (sandbox, moved)
}

/// `recipe` saved in the sandbox's work dir, beside a `note.txt` input.
fn save(sandbox: &Sandbox, recipe: &serde_json::Value) -> PathBuf {
    std::fs::write(sandbox.work_dir().join("note.txt"), b"hello").unwrap();
    let path = sandbox.work_dir().join("home.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

/// A shell recipe, which asks for consent until approved.
fn shell_recipe() -> serde_json::Value {
    serde_json::json!({
        "id": "probe", "name": "Probe",
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "run", "type": "shell-run",
              "parameters": { "command": "git", "args": ["--version"] } }
        ]
    })
}

/// `bnto run <recipe>` answering the consent prompt with `answer`.
fn run_answering(sandbox: &Sandbox, recipe: &Path, answer: &str) -> Output {
    use std::io::Write;
    let mut child = sandbox
        .command()
        .arg("run")
        .arg(recipe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(answer.as_bytes()).unwrap();
    drop(stdin);
    child.wait_with_output().unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn an_approval_is_kept_in_the_moved_homes_trust_file() {
    let (sandbox, moved) = sandbox_with_moved_home();
    let recipe = save(&sandbox, &shell_recipe());
    let approved = run_answering(&sandbox, &recipe, "y\n");
    assert!(approved.status.success(), "{}", stderr(&approved));
    // The TUI reads approvals from the moved home's trust file.
    let trusted = moved.path().join("state").join("trusted.json");
    assert!(trusted.is_file(), "no approval at {}", trusted.display());
    let default_trusted = sandbox.bnto_home().join("state").join("trusted.json");
    assert!(
        !default_trusted.exists(),
        "approval kept in the default home"
    );
    let again = run_answering(&sandbox, &recipe, "");
    assert!(
        again.status.success(),
        "approval not remembered: {}",
        stderr(&again)
    );
}

#[test]
fn an_approval_in_the_moved_home_counts_for_bnto_run() {
    // Approve in a sandbox with no override, then put its trust file where
    // the TUI keeps approvals once the home is moved.
    let plain = Sandbox::new();
    let plain_recipe = save(&plain, &shell_recipe());
    assert!(run_answering(&plain, &plain_recipe, "y\n").status.success());
    let (sandbox, moved) = sandbox_with_moved_home();
    let state = moved.path().join("state");
    std::fs::create_dir_all(&state).unwrap();
    let approvals = plain.bnto_home().join("state").join("trusted.json");
    std::fs::copy(approvals, state.join("trusted.json")).unwrap();
    let recipe = save(&sandbox, &shell_recipe());
    let run = run_answering(&sandbox, &recipe, "");
    assert!(
        run.status.success(),
        "approval in the moved home ignored: {}",
        stderr(&run)
    );
}

#[test]
fn a_run_reads_the_moved_homes_env_and_output_folder() {
    let (sandbox, moved) = sandbox_with_moved_home();
    std::fs::write(
        moved.path().join(".env"),
        format!("{KEY}=from-moved-home\n"),
    )
    .unwrap();
    let directory = format!("{{{{ctx.paths.output_dir}}}}/{{{{env.{KEY}}}}}");
    let recipe = save(
        &sandbox,
        &serde_json::json!({
            "id": "home-dir", "name": "Home Dir",
            "secrets": [{ "key": KEY }],
            "nodes": [
                { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
                { "id": "rename", "type": "file-rename", "parameters": { "prefix": "renamed-" } },
                { "id": "output", "type": "output",
                  "parameters": { "mode": "write", "directory": directory } }
            ]
        }),
    );
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .env_remove(KEY)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", stderr(&output));
    let written = moved
        .path()
        .join("output")
        .join("from-moved-home")
        .join("renamed-note.txt");
    assert_eq!(
        std::fs::read(&written).unwrap(),
        b"hello",
        "not at {}",
        written.display()
    );
}
