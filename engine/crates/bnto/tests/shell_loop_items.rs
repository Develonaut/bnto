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

#[test]
fn a_value_cant_bring_back_into_a_group_a_token_the_step_used_outside_it() {
    // SEC-27: the executor fills `{{item.secret}}` outside the group, so a
    // count over the whole params stays even when a cell brings one into the
    // group, which shell-run would then fill. Each group is counted alone.
    let dir = temp_output_dir();
    let tool = quote(json!(["{{item.secret}}", { "each": ["{{loop.note}}"] }]));
    let csv = "note,secret\n{{item.secret}},S3CRET\n";
    let (output, text) = run(dir.path(), per_row(tool), csv, &[]);
    assert!(!output.status.success(), "{text}");
    assert!(
        stderr(&output).contains("contains '{{item.secret}}'"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn outside_a_group_a_key_a_step_changed_reads_the_new_value() {
    // The brief's second case: the rest of the command reads the step's item.
    let dir = temp_output_dir();
    let set = json!({ "id": "set", "type": "edit-fields", "parameters": { "values": { "team": "new" } } });
    let tool = quote(json!(["{{item.team}}", "{{loop.team}}", { "each": ["each-{{item.team}}"] }]));
    let nodes = json!([input(), { "id": "rows", "type": "spreadsheet-read" },
        { "id": "each", "type": "loop", "nodes": [set, tool] }, output()]);
    let (output, text) = run(dir.path(), nodes, "team\nold\n", &[]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(text.contains("'new' 'old' 'each-new'"), "{text}");
}

/// `bnto run` of `nodes` on a `rows.json` holding `rows`; returns the output.
fn run_json(dir: &Path, nodes: Value, rows: Value) -> Output {
    let recipe = dir.join("loop.bnto.json");
    std::fs::write(
        &recipe,
        json!({ "name": "Shell Loop", "nodes": nodes }).to_string(),
    )
    .unwrap();
    let table = dir.join("rows.json");
    std::fs::write(&table, rows.to_string()).unwrap();
    Sandbox::new()
        .command()
        .arg("run")
        .arg(&recipe)
        .arg(&table)
        .arg("-o")
        .arg(dir.join("out"))
        .arg("--yes")
        .output()
        .unwrap()
}

fn per_row_tool(params: Value) -> Value {
    per_row(json!({ "id": "tool", "type": "shell-run", "parameters": params }))
}

#[test]
fn a_value_that_fills_a_whole_args_list_cant_bring_in_a_group() {
    // SEC-27: a whole-value placeholder keeps its JSON type, so the fill can
    // make a group the raw params never had; it's counted alone too.
    let dir = temp_output_dir();
    let params =
        json!({ "command": "git", "args": "{{loop.g}}", "env": { "X": "{{item.secret}}" } });
    let rows = json!([{ "g": ["rev-parse", "--sq-quote", { "each": ["{{item.secret}}"] }],
        "secret": "S3CRET" }]);
    let output = run_json(dir.path(), per_row_tool(params), rows);
    assert!(!output.status.success(), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("contains '{{item.secret}}'"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn a_value_that_completes_a_group_cant_bring_in_a_token() {
    // `{"each": "{{loop.g}}"}` isn't a group until the fill makes it one.
    let dir = temp_output_dir();
    let args = json!(["rev-parse", "--sq-quote", "{{item.secret}}", { "each": "{{loop.g}}" }]);
    let params = json!({ "command": "git", "args": args });
    let rows = json!([{ "g": ["leak={{item.secret}}"], "secret": "S3CRET" }]);
    let output = run_json(dir.path(), per_row_tool(params), rows);
    assert!(!output.status.success(), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("contains '{{item.secret}}'"),
        "{}",
        stderr(&output)
    );
}
