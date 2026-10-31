// Inside a loop, a step decided or filled once for all its files (a `when`
// guard, a batch step the executor fills) reads the item a per-file step
// would: one file's item, or the value several files agree on (ISS-110).

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

const CSV: &str = "name,team,status\nAda,core,pending\nGrace,core,pending\n";

fn write_recipe(dir: &Path, nodes: Value) -> PathBuf {
    let recipe = json!({ "name": "Step Item", "nodes": nodes });
    let path = dir.join("loop.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "file-upload" } })
}

fn output(mode: &str) -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": mode } })
}

/// `bnto run recipe people.csv -o dir/out`, from a sandbox.
fn run(dir: &Path, recipe: &Path) -> Output {
    let table = dir.join("people.csv");
    std::fs::write(&table, CSV).unwrap();
    let out = dir.join("out");
    Sandbox::new()
        .command()
        .arg("run")
        .arg(recipe)
        .arg(&table)
        .arg("-o")
        .arg(&out)
        .output()
        .unwrap()
}

fn text(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

/// One iteration per CSV row, running `steps`.
fn per_row(dir: &Path, steps: Value, mode: &str) -> PathBuf {
    write_recipe(
        dir,
        json!([
            input(),
            { "id": "rows", "type": "spreadsheet-read" },
            { "id": "each", "type": "loop", "nodes": steps },
            output(mode)
        ]),
    )
}

fn mark_done() -> Value {
    json!({ "id": "set", "type": "edit-fields", "parameters": { "values": { "status": "done" } } })
}

fn guarded_report(key: &str, expected: &str) -> Value {
    json!({ "id": "say", "type": "report", "parameters": { "message": "ran" },
        "when": { "value": format!("{{{{item.{key}}}}}"), "op": "equals", "expected": expected } })
}

#[test]
fn a_guard_reads_a_key_a_step_changed() {
    let dir = temp_output_dir();
    let recipe = per_row(
        dir.path(),
        json!([mark_done(), guarded_report("status", "done")]),
        "none",
    );
    let output = run(dir.path(), &recipe);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert_eq!(text.matches("ran").count(), 2, "{text}");
    assert!(!text.contains("skipped"), "{text}");
}

#[test]
fn a_batch_step_the_executor_fills_reads_a_key_a_step_changed() {
    let dir = temp_output_dir();
    let join = json!({ "id": "join", "type": "file-join",
        "parameters": { "filename": "{{item.name}}-{{item.status}}.txt" } });
    let recipe = per_row(dir.path(), json!([mark_done(), join]), "write");
    let output = run(dir.path(), &recipe);
    assert!(output.status.success(), "{}", text(&output));
    assert!(
        dir.path().join("out/Ada-done.txt").exists(),
        "{}",
        text(&output)
    );
}

/// One iteration for the CSV file, which a step fans out into its rows.
fn fanned_out(dir: &Path, after: Value) -> PathBuf {
    write_recipe(
        dir,
        json!([
            input(),
            { "id": "each", "type": "loop", "nodes": [
                { "id": "rows", "type": "spreadsheet-read" }, after
            ] },
            output("none")
        ]),
    )
}

#[test]
fn a_guard_over_files_that_agree_reads_their_value() {
    let dir = temp_output_dir();
    let recipe = fanned_out(dir.path(), guarded_report("team", "core"));
    let output = run(dir.path(), &recipe);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("ran"), "{text}");
}

#[test]
fn a_guard_over_files_that_disagree_is_refused() {
    let dir = temp_output_dir();
    let recipe = fanned_out(dir.path(), guarded_report("name", "Ada"));
    let output = run(dir.path(), &recipe);
    let text = text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("{{item.name}} differ"), "{text}");
}

#[test]
fn a_guard_reads_a_key_file_inspect_added() {
    // ISS-110's own case: `extension` is added inside the iteration.
    let dir = temp_output_dir();
    let recipe = write_recipe(
        dir.path(),
        json!([
            input(),
            { "id": "each", "type": "loop", "nodes": [
                { "id": "inspect", "type": "file-inspect" },
                guarded_report("extension", "txt")
            ] },
            output("none")
        ]),
    );
    let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.csv"));
    std::fs::write(&a, "a").unwrap();
    std::fs::write(&b, "b").unwrap();
    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&recipe)
        .arg(&a)
        .arg(&b)
        .output()
        .unwrap();
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert_eq!(text.matches("ran").count(), 1, "{text}");
    assert_eq!(text.matches("skipped").count(), 1, "{text}");
}
