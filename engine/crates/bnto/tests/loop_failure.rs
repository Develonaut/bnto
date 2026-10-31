// A loop that fails passes its step's error up with the iteration that
// failed, so the loop's line and the run's say which one (ISS-108).

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

/// The check that fails on Grace's row.
fn not_grace() -> Value {
    json!({ "id": "check", "type": "assert",
        "parameters": { "value": "{{item.name}}", "op": "not-equals", "expected": "Grace" } })
}

/// `bnto run` of `nodes` on `files`, from a sandbox.
fn run(dir: &Path, nodes: Value, files: &[&Path]) -> Output {
    let path = dir.join("loop.bnto.json");
    let recipe = json!({ "name": "Loop Failure", "nodes": nodes });
    std::fs::write(&path, recipe.to_string()).unwrap();
    Sandbox::new()
        .command()
        .arg("run")
        .arg(&path)
        .args(files)
        .output()
        .unwrap()
}

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "file-upload" } })
}

fn no_output() -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } })
}

#[test]
fn a_failed_iteration_is_named_on_the_loops_line_and_the_runs() {
    let dir = temp_output_dir();
    let recipe = json!({ "name": "Loop Failure", "nodes": [
        { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
        { "id": "rows", "type": "spreadsheet-read" },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "check", "type": "assert",
              "parameters": { "value": "{{item.name}}", "op": "not-equals", "expected": "Grace" } }
        ] },
        { "id": "output", "type": "output", "parameters": { "mode": "none" } }
    ] });
    let path = dir.path().join("loop.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    let table = dir.path().join("people.csv");
    std::fs::write(&table, "name\nAda\nGrace\n").unwrap();

    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&path)
        .arg(&table)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    let step = "FAILED check: Processing failed: Assertion failed: people_row1.csv";
    assert!(
        stderr.contains(step),
        "the step's own line is unchanged: {stderr}"
    );
    let each = "FAILED each: Processing failed: iteration 2 of 2: Assertion failed";
    assert!(stderr.contains(each), "{stderr}");
    let run = "Pipeline failed: Processing failed: iteration 2 of 2: Assertion failed";
    assert!(stderr.contains(run), "{stderr}");
}

#[test]
fn a_continued_loop_warns_with_the_same_iteration_wording() {
    let dir = temp_output_dir();
    let table = dir.path().join("people.csv");
    std::fs::write(&table, "name\nAda\nGrace\n").unwrap();
    let nodes = json!([input(), { "id": "rows", "type": "spreadsheet-read" },
        { "id": "each", "type": "loop", "parameters": { "onError": "continue" },
          "nodes": [not_grace()] }, no_output()]);
    let output = run(dir.path(), nodes, &[&table]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stderr}");
    let warning = "Warning: Processing failed: iteration 2 of 2: Assertion failed";
    assert!(
        stdout.contains(warning) || stderr.contains(warning),
        "{stdout}{stderr}"
    );
}

#[test]
fn nested_loops_name_each_loops_iteration_outer_first() {
    let dir = temp_output_dir();
    let (a, b) = (dir.path().join("a.csv"), dir.path().join("b.csv"));
    std::fs::write(&a, "name\nAda\n").unwrap();
    std::fs::write(&b, "name\nGrace\n").unwrap();
    let nodes = json!([input(),
        { "id": "outer", "type": "loop", "nodes": [
            { "id": "rows", "type": "spreadsheet-read" },
            { "id": "inner", "type": "loop", "nodes": [not_grace()] }
        ] }, no_output()]);
    let output = run(dir.path(), nodes, &[&a, &b]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    let outer = "FAILED outer: Processing failed: iteration 2 of 2: iteration 1 of 1: Assertion";
    assert!(stderr.contains(outer), "{stderr}");
}
