// A line break in a recipe-built name (a file name on disk, a name a node
// made, a param) is escaped where the run error naming it is built, so the
// raw error, before the CLI indents its later lines, holds no line break of
// the recipe's. Unix only: the names live on disk, and Windows refuses a
// `\n` in a file name.
#![cfg(unix)]

use crate::helpers;

use std::path::Path;

use helpers::Sandbox;
use serde_json::{Value, json};

/// A name with a line break before what it would like to forge.
const FORGING_NAME: &str = "x\nFORGED 1 file written";

/// One path from a recipe-built name to a run error: the files made first in
/// the work folder, the recipe's nodes and edges, and the inputs.
struct Case {
    what: &'static str,
    setup: fn(&Path),
    nodes: Value,
    edges: Value,
    inputs: &'static [&'static str],
}

/// Every case's run fails with the name's `\n` escaped in the raw error (the
/// `--json` summary's).
#[test]
fn a_line_break_in_a_recipe_built_name_is_escaped_where_the_error_is_built() {
    let mut failed = Vec::new();
    for case in cases() {
        let sandbox = Sandbox::new();
        (case.setup)(sandbox.work_dir());
        let error = raw_error(&sandbox, &case);
        if !error.contains("x\\nFORGED") || error.contains("x\nFORGED") {
            failed.push(format!("{}: {error}", case.what));
        }
    }
    assert!(failed.is_empty(), "not escaped:\n{}", failed.join("\n\n"));
}

fn cases() -> Vec<Case> {
    let write = || json!({ "id": "output", "type": "output", "parameters": { "mode": "write" } });
    let input =
        || json!({ "id": "input", "type": "input", "parameters": { "mode": "file-upload" } });
    vec![
        Case {
            what: "copy conflict",
            setup: |dir| {
                std::fs::write(dir.join("a.txt"), "a").unwrap();
                std::fs::create_dir(dir.join(FORGING_NAME)).unwrap();
                std::fs::write(dir.join(FORGING_NAME).join("a.txt"), "b").unwrap();
            },
            nodes: json!([input(), { "id": "copy", "type": "file-copy",
                "parameters": { "destination": FORGING_NAME, "conflict": "overwrite" } }, write()]),
            edges: json!([]),
            inputs: &["a.txt", "x\nFORGED 1 file written/a.txt"],
        },
        // A name under `blocker`, a file: its folder can't be made.
        copy_under_blocker("copy folder failure", "blocker/x\nFORGED/"),
        // A name directly in `blocker`: the copy can't be written.
        copy_under_blocker("copy write failure", "blocker/x\nFORGED-"),
        Case {
            what: "ordered_write conflict",
            setup: |dir| {
                std::fs::write(dir.join("a.txt"), "a").unwrap();
                std::fs::create_dir(dir.join("out")).unwrap();
                std::os::unix::fs::symlink(".", dir.join("out/sub")).unwrap();
            },
            nodes: json!([input(),
                { "id": "here", "type": "file-rename", "parameters": { "prefix": FORGING_NAME } },
                { "id": "sub", "type": "file-rename",
                    "parameters": { "prefix": format!("sub/{FORGING_NAME}") } },
                write()]),
            edges: json!([
                { "id": "e1", "source": "input", "target": "here" },
                { "id": "e2", "source": "input", "target": "sub" },
                { "id": "e3", "source": "here", "target": "output" },
                { "id": "e4", "source": "sub", "target": "output" }
            ]),
            inputs: &["a.txt"],
        },
        Case {
            what: "move plan",
            setup: |dir| std::fs::write(dir.join(FORGING_NAME), "a").unwrap(),
            nodes: json!([input(), { "id": "move", "type": "file-move",
                "parameters": { "destination": "moved" } }, write()]),
            edges: json!([]),
            inputs: &["x\nFORGED 1 file written", "./x\nFORGED 1 file written"],
        },
    ]
}

/// file-copy of `a.txt`, renamed with `prefix`, into the work folder, which
/// holds a file `blocker`.
fn copy_under_blocker(what: &'static str, prefix: &str) -> Case {
    Case {
        what,
        setup: |dir| {
            std::fs::write(dir.join("a.txt"), "a").unwrap();
            std::fs::write(dir.join("blocker"), "a file, not a folder").unwrap();
        },
        nodes: json!([
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rename", "type": "file-rename", "parameters": { "prefix": prefix } },
            { "id": "copy", "type": "file-copy", "parameters": {} },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ]),
        edges: json!([]),
        inputs: &["a.txt"],
    }
}

/// The run's error, from its `--json` summary.
fn raw_error(sandbox: &Sandbox, case: &Case) -> String {
    let mut def = json!({ "id": "forge", "name": "Forge", "nodes": case.nodes });
    if case.edges.as_array().is_some_and(|e| !e.is_empty()) {
        def["edges"] = case.edges.clone();
    }
    let recipe = sandbox.work_dir().join("forge.bnto.json");
    std::fs::write(&recipe, def.to_string()).unwrap();
    let output = sandbox
        .command()
        .args(["run", recipe.to_str().unwrap(), "--json", "-o", "out"])
        .args(case.inputs)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "{}: the run passed: {stderr}",
        case.what
    );
    let summary: Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("{}: {e}: {stdout}\n{stderr}", case.what));
    summary["error"].as_str().unwrap_or_default().to_string()
}
