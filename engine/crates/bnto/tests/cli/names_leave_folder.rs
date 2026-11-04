// A file's name is its path under the folder it's written into, and a recipe
// builds names (`file-render`'s `filename`, a `file-rename` prefix or
// find/replace), so a name that would leave that folder (absolute, a drive,
// or "..") refuses the run before any file is written: the output write and
// `file-copy` both check each file's final name (ISS-128).

use crate::helpers;

use std::path::Path;
use std::process::Output;

use helpers::Sandbox;
use serde_json::{Value, json};

/// A recipe of `steps` between a file-upload input and a write output.
fn recipe(sandbox: &Sandbox, steps: Value) -> std::path::PathBuf {
    let mut nodes = vec![json!({ "id": "input", "type": "input",
        "parameters": { "mode": "file-upload" } })];
    nodes.extend(steps.as_array().unwrap().iter().cloned());
    nodes.push(json!({ "id": "output", "type": "output", "parameters": { "mode": "write" } }));
    let def = json!({ "id": "names", "name": "Names", "nodes": nodes });
    let path = sandbox.work_dir().join("names.bnto.json");
    std::fs::write(&path, def.to_string()).unwrap();
    path
}

/// `bnto run <recipe> <input> -o out/deep` in the sandbox.
fn run(sandbox: &Sandbox, recipe: &Path, input: &str) -> Output {
    sandbox
        .command()
        .args(["run", recipe.to_str().unwrap(), input, "-o", "out/deep"])
        .output()
        .unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Every file under `dir`, relative, `/`-separated, sorted.
fn files_under(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(folder) = stack.pop() {
        for entry in std::fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).unwrap();
                found.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    found.sort();
    found
}

/// The run failed naming `name`, and the sandbox holds only what the test
/// put there: nothing written, under the output folder or anywhere else.
fn assert_refused(sandbox: &Sandbox, output: &Output, name: &str, inputs: &[&str]) {
    let err = stderr(output);
    assert!(!output.status.success(), "the run passed: {err}");
    assert!(err.contains(name), "{name} not named: {err}");
    let mut expected: Vec<String> = inputs.iter().map(|s| s.to_string()).collect();
    expected.push("names.bnto.json".to_string());
    expected.sort();
    assert_eq!(files_under(sandbox.work_dir()), expected, "{err}");
}

fn render(filename: &str) -> Value {
    json!([{ "id": "render", "type": "file-render",
        "parameters": { "row": "x", "filename": filename } }])
}

#[test]
fn a_rendered_filename_that_climbs_is_refused() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("note.txt"), "n").unwrap();
    let recipe = recipe(&sandbox, render("../evil.txt"));
    let output = run(&sandbox, &recipe, "note.txt");
    assert_refused(&sandbox, &output, "../evil.txt", &["note.txt"]);
}

#[test]
fn a_rendered_filename_that_is_absolute_is_refused() {
    let sandbox = Sandbox::new();
    let elsewhere = tempfile::tempdir().unwrap();
    let target = elsewhere.path().join("evil.txt");
    std::fs::write(sandbox.work_dir().join("note.txt"), "n").unwrap();
    let recipe = recipe(&sandbox, render(target.to_str().unwrap()));
    let output = run(&sandbox, &recipe, "note.txt");
    assert_refused(&sandbox, &output, "evil.txt", &["note.txt"]);
    assert!(!target.exists(), "wrote the absolute path");
}

#[test]
fn a_rendered_filename_at_a_root_is_refused_on_every_os() {
    // `\evil.txt` is the drive's root on Windows; refused everywhere so a
    // recipe means the same on every OS.
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("note.txt"), "n").unwrap();
    let recipe = recipe(&sandbox, render("\\evil.txt"));
    let output = run(&sandbox, &recipe, "note.txt");
    assert_refused(&sandbox, &output, "evil.txt", &["note.txt"]);
}

#[test]
fn a_rename_prefix_that_climbs_is_refused() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("note.txt"), "n").unwrap();
    let steps = json!([{ "id": "rename", "type": "file-rename",
        "parameters": { "prefix": "../x_" } }]);
    let recipe = recipe(&sandbox, steps);
    let output = run(&sandbox, &recipe, "note.txt");
    assert_refused(&sandbox, &output, "../x_note.txt", &["note.txt"]);
}

/// `xx/xx/g.txt` collected with its folders, then `x` replaced by `.`,
/// which builds `../../g.txt` from the path's parts (the review's `dotrep`).
fn dotrep(then: Value) -> Value {
    let mut steps = vec![
        json!({ "id": "collect", "type": "file-collect", "parameters": { "flatten": false } }),
        json!({ "id": "rename", "type": "file-rename",
            "parameters": { "find": "x", "replace": "." } }),
    ];
    steps.extend(then.as_array().unwrap().iter().cloned());
    Value::Array(steps)
}

fn with_tree(sandbox: &Sandbox) {
    let tree = sandbox.work_dir().join("tree").join("xx").join("xx");
    std::fs::create_dir_all(&tree).unwrap();
    std::fs::write(tree.join("g.txt"), "g").unwrap();
}

#[test]
fn a_find_and_replace_that_builds_a_climb_is_refused_by_the_output() {
    let sandbox = Sandbox::new();
    with_tree(&sandbox);
    let recipe = recipe(&sandbox, dotrep(json!([])));
    let output = run(&sandbox, &recipe, "tree");
    assert_refused(&sandbox, &output, "../../g.txt", &["tree/xx/xx/g.txt"]);
}

#[test]
fn a_file_copy_of_a_name_that_climbs_is_refused() {
    let sandbox = Sandbox::new();
    with_tree(&sandbox);
    let copy = json!([{ "id": "copy", "type": "file-copy",
        "parameters": { "destination": "a/b" } }]);
    let recipe = recipe(&sandbox, dotrep(copy));
    let output = run(&sandbox, &recipe, "tree");
    assert_refused(&sandbox, &output, "../../g.txt", &["tree/xx/xx/g.txt"]);
}

#[test]
fn a_dotted_name_that_stays_inside_is_written() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("note.txt"), "n").unwrap();
    let recipe = recipe(&sandbox, render("./sub/..hidden.txt"));
    let output = run(&sandbox, &recipe, "note.txt");
    assert!(output.status.success(), "{}", stderr(&output));
    let written = sandbox.work_dir().join("out/deep/sub/..hidden.txt");
    assert!(written.exists(), "{:?}", files_under(sandbox.work_dir()));
}

#[test]
fn a_refusal_after_a_progressive_loop_names_what_the_loop_wrote() {
    // The loop copies `a.txt` as its iteration ends and skips `../b.txt`;
    // the final write's refusal can't say nothing was written, so it names
    // the loop's copy, which stays.
    let sandbox = Sandbox::new();
    for name in ["a.txt", "b.txt"] {
        std::fs::write(sandbox.work_dir().join(name), name).unwrap();
    }
    let steps = json!([{ "id": "each", "type": "loop",
        "parameters": { "mode": "forEach", "outputPersistence": "progressive" },
        "nodes": [{ "id": "rename", "type": "file-rename",
            "parameters": { "find": "^b", "replace": "../b" } }],
        "edges": [] }]);
    let recipe = recipe(&sandbox, steps);
    let output = sandbox
        .command()
        .args([
            "run",
            recipe.to_str().unwrap(),
            "a.txt",
            "b.txt",
            "-o",
            "out/deep",
        ])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert!(!output.status.success(), "the run passed: {err}");
    assert!(err.contains("../b.txt"), "{err}");
    assert!(err.contains("the final write wrote nothing"), "{err}");
    assert!(err.contains("Loop iterations already wrote"), "{err}");
    // A multi-line error keeps its line breaks rather than showing `\n`.
    assert!(!err.contains("\\n"), "{err}");
    let wrote = err.split("Loop iterations already wrote").nth(1).unwrap();
    assert!(wrote.contains("deep") && wrote.contains("a.txt"), "{err}");
    let mut expected = vec!["a.txt", "b.txt", "names.bnto.json", "out/deep/a.txt"];
    expected.sort();
    assert_eq!(files_under(sandbox.work_dir()), expected, "{err}");
}
