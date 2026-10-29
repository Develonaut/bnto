// file-rename's pattern wraps the other name params: `{{name}}` is the
// transformed stem. A param the pattern leaves no token for would do
// nothing, so `bnto check` flags it and `bnto run` refuses it before any
// step runs (ISS-107).

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::json;

/// A file recipe whose `rename` step has `params`, written into `dir`.
fn recipe(dir: &Path, params: serde_json::Value) -> std::path::PathBuf {
    let recipe = json!({
        "name": "Pattern Rename",
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rename", "type": "file-rename", "parameters": params },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]
    });
    let path = dir.join("pattern.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    path
}

fn text(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

#[test]
fn run_puts_the_prefix_inside_the_pattern() {
    let dir = temp_output_dir();
    let out = dir.path().join("out");
    let input = dir.path().join("a.txt");
    std::fs::write(&input, "A").unwrap();
    let params = json!({ "pattern": "{{name}}-v2.{{ext}}", "prefix": "x-" });
    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(recipe(dir.path(), params))
        .arg(&input)
        .arg("-o")
        .arg(&out)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", text(&output));
    assert_eq!(
        std::fs::read_to_string(out.join("x-a-v2.txt")).unwrap(),
        "A"
    );
}

#[test]
fn check_flags_a_prefix_the_pattern_drops() {
    let dir = temp_output_dir();
    let params = json!({ "pattern": "{{item.new_name}}", "prefix": "x-" });
    let output = Sandbox::new()
        .command()
        .arg("check")
        .arg(recipe(dir.path(), params))
        .output()
        .unwrap();
    let text = text(&output);
    assert_eq!(output.status.code(), Some(1), "{text}");
    assert!(
        text.lines()
            .any(|l| l.contains("node \"rename\"") && l.contains("'prefix'")),
        "{text}"
    );
}

#[test]
fn run_refuses_a_prefix_the_pattern_drops_before_any_write() {
    let dir = temp_output_dir();
    let out = dir.path().join("out");
    let input = dir.path().join("a.txt");
    std::fs::write(&input, "A").unwrap();
    let params = json!({ "pattern": "final.{{ext}}", "prefix": "x-" });
    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(recipe(dir.path(), params))
        .arg(&input)
        .arg("-o")
        .arg(&out)
        .output()
        .unwrap();
    let text = text(&output);
    assert_eq!(output.status.code(), Some(1), "{text}");
    assert!(text.contains("has no {{name}}"), "{text}");
    let written = std::fs::read_dir(&out).map_or(0, |d| d.count());
    assert_eq!(written, 0, "{text}");
}
