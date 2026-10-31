// Inside a loop, `{{item.*}}` reads the file's current metadata over the
// loop's item, so a step's change is seen, and `{{loop.*}}` reads the loop's
// item as the iteration started (ISS-109).

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

const CSV: &str = "name,status\nAda,pending\n";

fn write_recipe(dir: &Path, nodes: Value) -> PathBuf {
    let recipe = json!({ "name": "Loop Item", "nodes": nodes });
    let path = dir.join("loop.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

/// Reads the CSV into one item per row and runs `steps` in a loop over them.
fn in_a_loop(dir: &Path, steps: Value) -> PathBuf {
    write_recipe(
        dir,
        json!([
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rows", "type": "spreadsheet-read" },
            { "id": "each", "type": "loop", "nodes": steps },
            { "id": "output", "type": "output", "parameters": { "mode": "none" } }
        ]),
    )
}

/// `bnto <command> recipe people.csv`, from a sandbox.
fn bnto(dir: &Path, command: &str, recipe: &Path) -> Output {
    let table = dir.join("people.csv");
    std::fs::write(&table, CSV).unwrap();
    Sandbox::new()
        .command()
        .arg(command)
        .arg(recipe)
        .arg(&table)
        .output()
        .unwrap()
}

fn text(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn mark_done() -> Value {
    json!({ "id": "set", "type": "edit-fields", "parameters": { "values": { "status": "done" } } })
}

fn report(message: &str) -> Value {
    json!({ "id": "say", "type": "report", "parameters": { "message": message } })
}

#[test]
fn item_reads_a_value_a_step_changed() {
    let dir = temp_output_dir();
    let recipe = in_a_loop(
        dir.path(),
        json!([mark_done(), report("now={{item.status}}")]),
    );
    let output = bnto(dir.path(), "run", &recipe);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("now=done"), "{text}");
}

#[test]
fn loop_reads_the_row_as_the_iteration_started() {
    let dir = temp_output_dir();
    let say = report("was={{loop.status}} now={{item.status}} {{loop.name}}");
    let recipe = in_a_loop(dir.path(), json!([mark_done(), say]));
    let output = bnto(dir.path(), "run", &recipe);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("was=pending now=done Ada"), "{text}");
}

#[test]
fn a_loop_key_the_row_lacks_is_refused() {
    let dir = temp_output_dir();
    let recipe = in_a_loop(dir.path(), json!([report("{{loop.gone}}")]));
    let output = bnto(dir.path(), "run", &recipe);
    let text = text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("{{loop.gone}} has no value"), "{text}");
}

#[test]
fn loop_outside_a_loop_is_refused() {
    let dir = temp_output_dir();
    let recipe = write_recipe(
        dir.path(),
        json!([
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            report("{{loop.name}}"),
            { "id": "output", "type": "output", "parameters": { "mode": "none" } }
        ]),
    );
    let output = bnto(dir.path(), "run", &recipe);
    let text = text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("not inside a loop"), "{text}");
}

#[test]
fn check_accepts_the_loop_namespace() {
    let dir = temp_output_dir();
    let recipe = in_a_loop(dir.path(), json!([report("{{loop.name}}")]));
    let output = Sandbox::new()
        .command()
        .arg("check")
        .arg(&recipe)
        .output()
        .unwrap();
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(!text.contains("namespace"), "{text}");
}

#[test]
fn a_template_written_once_reads_loop_too() {
    // file-render's header and filename are written once per call, from the
    // loop's item; its row reads the file's metadata over it.
    let dir = temp_output_dir();
    let render = json!({ "id": "render", "type": "file-render", "parameters": {
        "header": "# {{loop.status}}", "row": "- {{item.status}}", "filename": "{{loop.name}}.md"
    } });
    let recipe = write_recipe(
        dir.path(),
        json!([
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rows", "type": "spreadsheet-read" },
            { "id": "each", "type": "loop", "nodes": [mark_done(), render] },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]),
    );
    let table = dir.path().join("people.csv");
    std::fs::write(&table, CSV).unwrap();
    let out = dir.path().join("out");
    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&recipe)
        .arg(&table)
        .arg("-o")
        .arg(&out)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", text(&output));
    let ada = std::fs::read_to_string(out.join("Ada.md")).unwrap();
    assert_eq!(ada, "# pending\n- done\n");
}
