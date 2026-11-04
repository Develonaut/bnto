// A line break in recipe text can't forge a line of bnto's own output. A run
// error that holds recipe text shows its `\n` escaped where the error is
// built, and any line an error does break onto starts indented, so nothing a
// recipe writes starts a line at column 0, where bnto prints "✔ 1 file
// written" or a hint to run a command.

use crate::helpers;

use std::process::Output;

use helpers::Sandbox;
use serde_json::{Value, json};

/// What a recipe would like to print as a line of its own.
const FORGED: &str = "FORGED 1 file written";

/// A recipe of `steps` between a file-upload input and a write output.
fn run_steps(sandbox: &Sandbox, steps: Value) -> Output {
    let mut nodes = vec![json!({ "id": "input", "type": "input",
        "parameters": { "mode": "file-upload" } })];
    nodes.extend(steps.as_array().unwrap().iter().cloned());
    nodes.push(json!({ "id": "output", "type": "output", "parameters": { "mode": "write" } }));
    let def = json!({ "id": "forge", "name": "Forge", "nodes": nodes });
    let recipe = sandbox.work_dir().join("forge.bnto.json");
    std::fs::write(&recipe, def.to_string()).unwrap();
    std::fs::write(sandbox.work_dir().join("a.txt"), "a").unwrap();
    sandbox
        .command()
        .env("NO_COLOR", "1")
        .args(["run", recipe.to_str().unwrap(), "a.txt", "-o", "out"])
        .output()
        .unwrap()
}

/// The run failed, showing the recipe's `\n` escaped, and no line of stderr
/// starts with the recipe's text.
fn assert_not_forged(output: &Output) {
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "the run passed: {err}");
    assert!(err.contains(&format!("\\n{FORGED}")), "not escaped: {err}");
    for line in err.lines() {
        assert!(!line.starts_with("FORGED"), "a forged line: {err}");
    }
}

#[test]
fn a_line_break_in_a_copy_destination_forges_no_line() {
    let sandbox = Sandbox::new();
    let destination = format!("C:x\n{FORGED}");
    let output = run_steps(
        &sandbox,
        json!([{ "id": "copy", "type": "file-copy",
            "parameters": { "destination": destination } }]),
    );
    assert_not_forged(&output);
}

#[test]
fn a_line_break_in_a_rename_prefix_forges_no_line() {
    let sandbox = Sandbox::new();
    let prefix = format!("../\n{FORGED}");
    let output = run_steps(
        &sandbox,
        json!([
            { "id": "rename", "type": "file-rename", "parameters": { "prefix": prefix } },
            { "id": "copy", "type": "file-copy", "parameters": { "destination": "copied" } }
        ]),
    );
    assert_not_forged(&output);
    assert!(!sandbox.work_dir().join("copied").exists());
}

/// A dry run shows a multi-line refusal as a run does: its line breaks
/// kept, every later line indented, rather than one line with a literal `\n`.
#[test]
fn a_dry_runs_multi_line_refusal_keeps_its_line_breaks() {
    let sandbox = Sandbox::new();
    let dir = sandbox.work_dir();
    std::fs::write(dir.join("a.txt"), "a").unwrap();
    let def = json!({ "id": "forge", "name": "Forge", "nodes": [
        { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
        { "id": "rename", "type": "file-rename", "parameters": { "prefix": "../" } },
        { "id": "copy", "type": "file-copy", "parameters": { "destination": "copied" } },
        { "id": "output", "type": "output", "parameters": { "mode": "write" } }
    ] });
    let recipe = dir.join("forge.bnto.json");
    std::fs::write(&recipe, def.to_string()).unwrap();
    let output = sandbox
        .command()
        .env("NO_COLOR", "1")
        .args(["dry-run", recipe.to_str().unwrap(), "a.txt"])
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&output.stderr);
    let refusal = "pipeline preview failed: Invalid input: File copy refused, \
                   this step copied nothing: 1 name\n  \"../a.txt\" would leave";
    assert!(err.contains(refusal), "{err}");
    assert!(err.contains("\n  A file's name is its path"), "{err}");
    assert!(!err.contains("\\n"), "a literal line break: {err}");
}
