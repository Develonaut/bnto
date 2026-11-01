// shell-run inside a loop (ISS-111): an `each` group fills `{{item.*}}` per
// file, over the loop's item, while the rest of the command reads the step's
// item. Values the executor fills into a group stay refused when they carry
// a token shell-run fills itself (SEC-27).

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

/// `git rev-parse --sq-quote` prints its args back quoted: a portable echo.
fn quote(args: Value) -> Value {
    json!({ "id": "tool", "type": "shell-run", "parameters": {
        "command": "git", "args": json!(["rev-parse", "--sq-quote"]).as_array().unwrap()
            .iter().cloned().chain(args.as_array().unwrap().iter().cloned()).collect::<Vec<_>>()
    } })
}

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "file-upload" } })
}

fn output() -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": "write" } })
}

/// Runs `nodes` on a CSV holding `csv`, with `args`; returns the output and
/// every file written, joined.
fn run(dir: &Path, nodes: Value, csv: &str, args: &[&str]) -> (Output, String) {
    let recipe = dir.join("loop.bnto.json");
    std::fs::write(
        &recipe,
        json!({ "name": "Shell Loop", "nodes": nodes }).to_string(),
    )
    .unwrap();
    let table = dir.join("people.csv");
    std::fs::write(&table, csv).unwrap();
    let out = dir.join("out");
    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&recipe)
        .arg(&table)
        .arg("-o")
        .arg(&out)
        .arg("--yes")
        .args(args)
        .output()
        .unwrap();
    let mut text = String::new();
    if let Ok(entries) = std::fs::read_dir(&out) {
        let mut paths: Vec<_> = entries.map(|e| e.unwrap().path()).collect();
        paths.sort();
        for path in paths {
            text.push_str(&std::fs::read_to_string(path).unwrap_or_default());
        }
    }
    (output, text)
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// One iteration for the CSV file, fanned out into its rows, then `tool`.
fn fanned_out(tool: Value) -> Value {
    json!([input(),
        { "id": "tag", "type": "edit-fields", "parameters": { "values": { "team": "core" } } },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "rows", "type": "spreadsheet-read" }, tool
        ] },
        output()])
}

#[test]
fn a_group_reads_each_fanned_out_row_and_the_loops_item() {
    // `name` differs per row and is read only inside the group, so the step
    // rule doesn't refuse it; `team` is only on the loop's item.
    let dir = temp_output_dir();
    let tool = quote(json!([{ "each": ["{{item.name}}-{{item.team}}"] }]));
    let (output, text) = run(dir.path(), fanned_out(tool), "name\nAda\nGrace\n", &[]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(text.contains("'Ada-core' 'Grace-core'"), "{text}");
}

#[test]
fn outside_a_group_files_that_disagree_are_still_refused() {
    let dir = temp_output_dir();
    let tool = quote(json!(["{{item.name}}", { "each": ["{{item.name}}"] }]));
    let (output, _) = run(dir.path(), fanned_out(tool), "name\nAda\nGrace\n", &[]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("{{item.name}} differ"),
        "{}",
        stderr(&output)
    );
}

/// A loop over the CSV's rows, then `tool`.
fn per_row(tool: Value) -> Value {
    json!([input(), { "id": "rows", "type": "spreadsheet-read" },
        { "id": "each", "type": "loop", "nodes": [tool] }, output()])
}

#[test]
fn a_loop_value_carrying_a_path_token_into_a_group_is_refused() {
    // SEC-27: shell-run would fill the `{{path}}` the cell brought in.
    let dir = temp_output_dir();
    let tool = quote(json!([{ "each": ["{{loop.note}}"] }]));
    let (output, _) = run(dir.path(), per_row(tool), "note\n{{path}}\n", &[]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("contains '{{path}}'"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn a_field_carrying_an_item_token_into_a_group_is_refused() {
    let dir = temp_output_dir();
    let mut tool = quote(json!([{ "each": ["{{fields.x}}"] }]));
    tool["fields"] = json!({ "x": { "type": "string", "label": "X" } });
    let (output, _) = run(
        dir.path(),
        per_row(tool),
        "note\nA\n",
        &["-p", "x={{item.note}}"],
    );
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("contains '{{item.note}}'"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn a_cell_carrying_an_output_dir_token_read_in_a_group_is_refused() {
    let dir = temp_output_dir();
    let tool = quote(json!([{ "each": ["{{item.note}}"] }]));
    let (output, _) = run(dir.path(), per_row(tool), "note\nx{{output_dir}}\n", &[]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("'{{output_dir}}'"),
        "{}",
        stderr(&output)
    );
}
