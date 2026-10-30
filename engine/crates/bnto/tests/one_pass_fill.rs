// A node that fills its own tokens (report, assert, edit-fields, file-render,
// file-copy) fills the run's namespaces in the same pass: a `{{fields.*}}`,
// `{{env.*}}` or `{{node.*}}` value that reads like a token comes out as
// written, at the top level and inside a loop (ISS-35).

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

fn write_recipe(dir: &Path, nodes: Value, edges: Option<Value>) -> PathBuf {
    let mut recipe = json!({ "name": "One Pass", "nodes": nodes });
    if let Some(edges) = edges {
        recipe["edges"] = edges;
    }
    let path = dir.join("one-pass.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

fn text(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "file-upload" } })
}

fn output_none() -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } })
}

/// `bnto run recipe a.txt` with `args` and `env`, from a sandbox.
fn run(dir: &Path, recipe: &Path, args: &[&str], env: &[(&str, &str)]) -> Output {
    run_on(dir, "A", recipe, args, env)
}

/// `run`, with `a.txt` holding `content`.
fn run_on(dir: &Path, content: &str, recipe: &Path, args: &[&str], env: &[(&str, &str)]) -> Output {
    let file = dir.join("a.txt");
    std::fs::write(&file, content).unwrap();
    let sandbox = Sandbox::new();
    let mut command = sandbox.command();
    command.arg("run").arg(recipe).arg(&file).args(args);
    for (key, value) in env {
        command.env(key, value);
    }
    command.output().unwrap()
}

#[test]
fn report_prints_a_field_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let say = json!({ "id": "say", "type": "report",
        "fields": { "note": { "type": "string", "label": "Note" } },
        "parameters": { "message": "note={{fields.note}} in {{file}}" } });
    let recipe = write_recipe(dir.path(), json!([input(), say, output_none()]), None);
    let output = run(dir.path(), &recipe, &["-p", "note={{file}}"], &[]);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("note={{file}} in a.txt"), "{text}");
}

#[test]
fn edit_fields_sets_an_env_value_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let set = json!({ "id": "set", "type": "edit-fields",
        "parameters": { "values": { "copied": "{{env.BNTO_ONE_PASS_NOTE}}" } } });
    let say = json!({ "id": "say", "type": "report",
        "parameters": { "message": "copied={{item.copied}}" } });
    let recipe = write_recipe(dir.path(), json!([input(), set, say, output_none()]), None);
    let env = [("BNTO_ONE_PASS_NOTE", "{{file}}")];
    let output = run(dir.path(), &recipe, &[], &env);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("copied={{file}}"), "{text}");
}

#[test]
fn assert_compares_a_field_holding_a_token_as_written() {
    let dir = temp_output_dir();
    let check = json!({ "id": "check", "type": "assert",
        "fields": { "want": { "type": "string", "label": "Want" } },
        "parameters": { "value": "{{fields.want}}", "op": "not-equals", "expected": "{{file}}" } });
    let recipe = write_recipe(dir.path(), json!([input(), check, output_none()]), None);
    let output = run(dir.path(), &recipe, &["-p", "want={{file}}"], &[]);
    assert!(output.status.success(), "{}", text(&output));
}

#[test]
fn in_a_loop_a_node_value_holding_an_item_token_is_not_filled_again() {
    // The input lists the path `{{item.first}}`; the loop's item has `first`.
    let dir = temp_output_dir();
    let base = dir.path().join("base");
    std::fs::create_dir(&base).unwrap();
    let nodes = json!([
        input(),
        { "id": "ls", "type": "file-list", "parameters": { "base": base } },
        { "id": "set", "type": "edit-fields", "parameters": { "values": { "first": "Ada" } } },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "say", "type": "report",
              "parameters": { "each": "{{node.ls.entries}}", "message": "path={{entry.path}}" } }
        ] },
        output_none()
    ]);
    let edges = json!([
        { "id": "e1", "source": "input", "target": "ls" },
        { "id": "e2", "source": "input", "target": "set" },
        { "id": "e3", "source": "set", "target": "each" },
        { "id": "e4", "source": "each", "target": "output" }
    ]);
    let recipe = write_recipe(dir.path(), nodes, Some(edges));
    let output = run_on(dir.path(), "{{item.first}}\n", &recipe, &[], &[]);
    let text = text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("path={{item.first}}"), "{text}");
}
