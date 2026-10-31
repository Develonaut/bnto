// Inside a loop, a batch node that fills `{{item.*}}` itself (report,
// assert, edit-fields, file-render, file-copy) fills it once, from each
// file's metadata over the loop's item: a key a step inside the
// iteration adds is there (ISS-58), and a value that reads like a token
// comes out as written (ISS-35). A template written once per call (a
// header, a filename, report's entry lines) still reads the loop's item.

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

/// A recipe that reads a CSV into one item per row and runs `steps` in a
/// loop over them, written into `dir`.
fn recipe(dir: &Path, steps: Value) -> PathBuf {
    let recipe = json!({
        "name": "Loop Items",
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rows", "type": "spreadsheet-read" },
            { "id": "each", "type": "loop", "nodes": steps },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]
    });
    let path = dir.join("loop.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

/// Run `steps` in a loop over the rows of `csv`, writing to `dir/out`.
fn run(dir: &Path, csv: &str, steps: Value, extra: &[&str]) -> Output {
    let table = dir.join("people.csv");
    std::fs::write(&table, csv).unwrap();
    Sandbox::new()
        .command()
        .arg("run")
        .arg(recipe(dir, steps))
        .arg(&table)
        .arg("-o")
        .arg(dir.join("out"))
        .args(extra)
        .output()
        .unwrap()
}

fn text(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn report(id: &str, message: &str) -> Value {
    json!({ "id": id, "type": "report", "parameters": { "message": message } })
}

fn set_fields(values: Value) -> Value {
    json!({ "id": "set", "type": "edit-fields", "parameters": { "values": values } })
}

// --- ISS-58: a key a step inside the iteration adds ---

#[test]
fn report_reads_a_key_edit_fields_set_inside_the_loop() {
    let dir = temp_output_dir();
    let steps = json!([
        set_fields(json!({ "greeting": "hi {{item.first}}" })),
        report("say", "said {{item.greeting}}")
    ]);
    let output = run(dir.path(), "first\nAda\nGrace\n", steps, &[]);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("said hi Ada"), "{text}");
    assert!(text.contains("said hi Grace"), "{text}");
}

#[test]
fn file_copy_reads_a_key_set_inside_the_loop() {
    let dir = temp_output_dir();
    let copies = dir.path().join("copies");
    let destination = format!("{}/{{{{item.folder}}}}", copies.display());
    let steps = json!([
        set_fields(json!({ "folder": "by-{{item.first}}" })),
        { "id": "copy", "type": "file-copy", "parameters": { "destination": destination } }
    ]);
    let output = run(dir.path(), "first\nAda\nGrace\n", steps, &[]);
    assert!(output.status.success(), "{}", text(&output));
    assert!(copies.join("by-Ada").is_dir(), "{}", text(&output));
    assert!(copies.join("by-Grace").is_dir(), "{}", text(&output));
}

// --- ISS-35: a value that reads like a token ---

#[test]
fn report_prints_a_cell_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let csv = "note\n{{file}}\n{{item.note}}\n{{env.HOME}}\n";
    let output = run(
        dir.path(),
        csv,
        json!([report("say", "note={{item.note}}")]),
        &[],
    );
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("note={{file}}"), "{text}");
    assert!(text.contains("note={{item.note}}"), "{text}");
    assert!(text.contains("note={{env.HOME}}"), "{text}");
}

#[test]
fn assert_checks_a_cell_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let check = json!({ "id": "check", "type": "assert", "parameters": {
        "value": "{{item.note}}", "op": "not-equals", "expected": "{{item.named}}"
    } });
    let output = run(
        dir.path(),
        "note,named\n{{file}},people_row0.csv\n",
        json!([check]),
        &[],
    );
    assert!(output.status.success(), "{}", text(&output));
}

#[test]
fn edit_fields_sets_a_cell_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let steps = json!([
        set_fields(json!({ "copied": "{{item.note}}" })),
        report("say", "copied={{item.copied}}")
    ]);
    let output = run(dir.path(), "note\n{{file}}\n", steps, &[]);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("copied={{file}}"), "{text}");
}

// --- Templates written once per call still read the loop's item ---

#[test]
fn file_render_names_its_file_from_the_loop_item() {
    let dir = temp_output_dir();
    let render = json!({ "id": "render", "type": "file-render", "parameters": {
        "header": "# {{item.first}}", "row": "- {{file}}", "filename": "{{item.first}}.md"
    } });
    let output = run(dir.path(), "first\nAda\nGrace\n", json!([render]), &[]);
    assert!(output.status.success(), "{}", text(&output));
    let ada = std::fs::read_to_string(dir.path().join("out/Ada.md")).unwrap();
    assert!(ada.starts_with("# Ada\n"), "{ada}");
    assert!(dir.path().join("out/Grace.md").is_file());
}

#[test]
fn report_entry_lines_read_the_loop_item() {
    let dir = temp_output_dir();
    let say = json!({ "id": "say", "type": "report", "parameters": {
        "message": "{{item.first}}: {{entry}}", "each": ["x", "y"]
    } });
    let output = run(dir.path(), "first\nAda\n", json!([say]), &[]);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("Ada: x") && text.contains("Ada: y"), "{text}");
}

// --- A missing key ---

#[test]
fn a_key_neither_the_file_nor_the_loop_item_has_fails_naming_both() {
    let dir = temp_output_dir();
    let output = run(
        dir.path(),
        "first\nAda\n",
        json!([report("say", "{{item.gone}}")]),
        &[],
    );
    let text = text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("gone") && text.contains("first"), "{text}");
}

#[test]
fn a_dry_run_refuses_a_missing_key_too() {
    let dir = temp_output_dir();
    let output = run(
        dir.path(),
        "first\nAda\n",
        json!([report("say", "{{item.gone}}")]),
        &["--dry-run"],
    );
    let text = text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("gone"), "{text}");
}
