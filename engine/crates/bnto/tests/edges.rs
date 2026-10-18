// Recipe edges through the built binary: a step with no incoming edge
// starts from no files, two steps fan into file-join in edge order, and a
// cycle fails the run before any step starts.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

const NOTES: &str = "# Sounds\n\nHand-written notes.\n";
/// file-render ends its header with a newline of its own.
const LISTING: &str = "- door.wav\n- rain.wav\n";

/// A root folder holding `notes.md` and `listing.md`.
fn root_fixture() -> tempfile::TempDir {
    let root = temp_output_dir();
    std::fs::write(root.path().join("notes.md"), NOTES).unwrap();
    std::fs::write(root.path().join("listing.md"), "not this one\n").unwrap();
    root
}

/// Run `recipe` with `Root` set to `root`, writing to `out`, plus `extra` args.
fn run(recipe: &str, root: &Path, out: &Path, extra: &[&str]) -> Output {
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let body = serde_json::json!({ "Root": root.to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    Command::new(bnto_bin())
        .args(["run", recipe, "--yes"])
        .args(["--vars", vars_file.to_str().unwrap()])
        .args(["-o", out.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap()
}

/// Every entry directly under `dir`, sorted.
fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// stdout as exactly one JSON line, parsed.
fn single_json_line(output: &Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "stdout: {stdout:?}");
    serde_json::from_str(lines[0]).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

#[test]
fn test_a_file_list_with_no_incoming_edge_lists_only_its_own_paths() {
    // `emit` writes paths.txt naming listing.md; if `fresh` received it, it
    // would read it as a path list and list listing.md instead.
    let (root, out) = (root_fixture(), temp_output_dir());
    let recipe = custom_recipe_path("edges-fresh-list");
    let output = run(&recipe, root.path(), out.path(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(entries(out.path()), ["notes.md"]);
    let notes = std::fs::read_to_string(out.path().join("notes.md")).unwrap();
    assert_eq!(notes, NOTES);
}

#[test]
fn test_two_steps_fan_into_file_join_in_edge_order() {
    // `listing` comes first in `nodes`, but the notes edge comes first.
    let (root, out) = (root_fixture(), temp_output_dir());
    let recipe = custom_recipe_path("edges-notes-listing-join");
    let output = run(&recipe, root.path(), out.path(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(entries(out.path()), ["README.md"]);
    let joined = std::fs::read_to_string(out.path().join("README.md")).unwrap();
    assert_eq!(joined, format!("{NOTES}\n{LISTING}"));
}

#[test]
fn test_the_fan_in_dry_run_writes_nothing() {
    let (root, out_root) = (root_fixture(), temp_output_dir());
    let out = out_root.path().join("out");
    let recipe = custom_recipe_path("edges-notes-listing-join");
    let output = run(&recipe, root.path(), &out, &["--dry-run"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(!out.exists(), "output dir was created");
}

#[test]
fn test_run_json_reports_every_branch_in_execution_order() {
    let (root, out) = (root_fixture(), temp_output_dir());
    let recipe = custom_recipe_path("edges-notes-listing-join");
    let output = run(&recipe, root.path(), out.path(), &["--json"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let summary = single_json_line(&output);
    let nodes = summary["nodes"].as_array().unwrap();
    let ids: Vec<&str> = nodes.iter().map(|n| n["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["listing", "notes", "join"]);
    assert_eq!(nodes[2]["metadata"]["joined"], 2, "summary: {summary}");
}

#[test]
fn test_a_cycle_fails_the_run_before_any_step() {
    let (root, out) = (root_fixture(), temp_output_dir());
    let recipe_dir = temp_output_dir();
    let recipe = recipe_dir.path().join("cycle.bnto.json");
    let body = serde_json::json!({
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "a", "type": "file-list",
              "parameters": { "base": "{{vars.Root}}", "paths": ["notes.md"] } },
            { "id": "b", "type": "file-join", "parameters": { "filename": "x.md" } },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ],
        "edges": [
            { "id": "e1", "source": "a", "target": "b" },
            { "id": "e2", "source": "b", "target": "a" },
            { "id": "e3", "source": "b", "target": "output" }
        ]
    });
    std::fs::write(&recipe, body.to_string()).unwrap();
    let output = run(recipe.to_str().unwrap(), root.path(), out.path(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(
        stderr.contains("edges form a cycle: a → b → a"),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("[1/"), "no step started: {stderr}");
    assert!(entries(out.path()).is_empty());
}
