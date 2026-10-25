// file-copy over a file the run still reads outside its batch (ISS-59).
//
// Inside a loop, each iteration hands file-copy one file, and a graph level
// holds files for branches that run later. A copy that would replace one of
// those files is refused, and a refusal after earlier copies names them. An
// in-place overwrite of new bytes stays allowed. Every run uses the
// sandboxed HOME and temp folders.

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Output;

use helpers::{Sandbox, fixture_image, temp_output_dir};
use serde_json::{Value, json};

/// `a.txt` = `AAAA` and `b.txt` = `BBBB` in a fresh folder.
fn two_inputs() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let dir = temp_output_dir();
    let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
    std::fs::write(&a, "AAAA").unwrap();
    std::fs::write(&b, "BBBB").unwrap();
    (dir, a, b)
}

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "files" } })
}

fn no_output() -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } })
}

fn rename(find: &str, replace: &str) -> Value {
    json!({ "id": "ren", "type": "file-rename", "parameters": { "find": find, "replace": replace } })
}

fn copy_into(dir: &Path, conflict: &str) -> Value {
    json!({
        "id": "cp", "type": "file-copy",
        "parameters": { "destination": dir.to_str().unwrap(), "conflict": conflict }
    })
}

fn in_loop(children: &[Value]) -> Value {
    json!({
        "id": "each", "type": "loop", "parameters": { "mode": "forEach" },
        "nodes": children, "edges": []
    })
}

fn recipe(nodes: &[Value]) -> Value {
    json!({ "name": "Held Files", "nodes": nodes })
}

/// Run `recipe` on `files` inside a fresh sandbox.
fn run(recipe: &Value, files: &[&Path]) -> Output {
    let sandbox = Sandbox::new();
    let dir = temp_output_dir();
    let path = dir.path().join("held.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    sandbox
        .command()
        .arg("run")
        .arg(&path)
        .args(files)
        .arg("--yes")
        .output()
        .unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The run failed with file-copy's refusal and `b.txt` still holds `BBBB`.
fn assert_refused(output: &Output, b: &Path) -> String {
    let stderr = stderr(output);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("File copy refused"), "{stderr}");
    assert!(stderr.contains(&b.display().to_string()), "{stderr}");
    assert_eq!(std::fs::read_to_string(b).unwrap(), "BBBB", "{stderr}");
    stderr
}

/// `a` renamed to `b.txt`, copied into the inputs' folder, one file per iteration.
fn loop_rename_then_copy(dir: &Path, conflict: &str) -> Value {
    let each = in_loop(&[rename("a", "b"), copy_into(dir, conflict)]);
    recipe(&[input(), each, no_output()])
}

#[test]
fn test_a_loop_copy_over_a_later_iterations_file_is_refused() {
    // RED (ISS-59): iteration `a` copied over `b.txt` before iteration `b`
    // read it: BBBB was lost.
    let (dir, a, b) = two_inputs();
    let output = run(&loop_rename_then_copy(dir.path(), "overwrite"), &[&a, &b]);
    let stderr = assert_refused(&output, &b);
    assert!(stderr.contains("nothing copied"), "{stderr}");
    assert_eq!(std::fs::read_to_string(&a).unwrap(), "AAAA");
}

#[test]
fn test_a_loop_copy_over_a_finished_iterations_file_is_refused() {
    // RED (ISS-59): iteration `b` ran first, but its result still reads
    // `b.txt`, so replacing it after loses `b`'s bytes all the same.
    let (dir, a, b) = two_inputs();
    let output = run(&loop_rename_then_copy(dir.path(), "overwrite"), &[&b, &a]);
    assert_refused(&output, &b);
}

#[test]
fn test_a_loop_newer_copy_over_another_iterations_file_is_refused() {
    let (dir, a, b) = two_inputs();
    let hour_ago = std::time::SystemTime::now() - std::time::Duration::from_secs(3600);
    let file = std::fs::File::options().write(true).open(&b).unwrap();
    file.set_modified(hour_ago).unwrap();
    let output = run(&loop_rename_then_copy(dir.path(), "newer"), &[&a, &b]);
    assert_refused(&output, &b);
}

#[test]
fn test_a_branch_copy_over_a_file_another_branch_reads_is_refused() {
    // RED (ISS-59): one branch keeps only `a`, renames it to `b.txt` and
    // copies it over `b.txt`; the output's branch still writes both inputs.
    let (dir, a, b) = two_inputs();
    let out = temp_output_dir();
    let nodes = [
        input(),
        json!({ "id": "only-a", "type": "file-filter", "parameters": { "namePattern": "a.txt" } }),
        rename("a", "b"),
        copy_into(dir.path(), "overwrite"),
        json!({ "id": "output", "type": "output",
            "parameters": { "mode": "write", "directory": out.path().to_str().unwrap() } }),
    ];
    let edges = json!([
        { "id": "e1", "source": "input", "target": "only-a" },
        { "id": "e2", "source": "only-a", "target": "ren" },
        { "id": "e3", "source": "ren", "target": "cp" },
        { "id": "e4", "source": "input", "target": "output" }
    ]);
    let mut graph = recipe(&nodes);
    graph["edges"] = edges;
    let output = run(&graph, &[&a, &b]);
    assert_refused(&output, &b);
}

#[test]
fn test_a_mid_loop_refusal_names_the_copies_earlier_iterations_made() {
    // RED: iteration `c` copied `c.txt` into the folder before iteration `a`
    // was refused; the refusal says so instead of "nothing copied".
    let (dir, a, b) = two_inputs();
    let elsewhere = temp_output_dir();
    let c = elsewhere.path().join("c.txt");
    std::fs::write(&c, "CCCC").unwrap();
    let output = run(
        &loop_rename_then_copy(dir.path(), "overwrite"),
        &[&c, &a, &b],
    );
    let stderr = assert_refused(&output, &b);
    let copied = dir.path().join("c.txt");
    assert!(stderr.contains("nothing more copied"), "{stderr}");
    assert!(stderr.contains("Copied earlier in this run"), "{stderr}");
    assert!(stderr.contains(&copied.display().to_string()), "{stderr}");
    assert_eq!(std::fs::read_to_string(&copied).unwrap(), "CCCC");
}

// --- Guard: an in-place overwrite of new bytes still runs ---

/// Two JPEGs in a fresh folder, and their sizes.
fn two_images() -> (tempfile::TempDir, [PathBuf; 2], [u64; 2]) {
    let dir = temp_output_dir();
    let x = dir.path().join("x.jpg");
    let y = dir.path().join("y.jpg");
    std::fs::copy(fixture_image("large.jpg"), &x).unwrap();
    std::fs::copy(fixture_image("gradient.jpg"), &y).unwrap();
    let sizes = [size(&x), size(&y)];
    (dir, [x, y], sizes)
}

fn size(path: &Path) -> u64 {
    std::fs::metadata(path).unwrap().len()
}

#[test]
fn test_compressing_images_in_place_still_runs_flat_and_in_a_loop() {
    // Guard: each copy replaces its own file's original, whose bytes the
    // run already turned into the compressed ones. Nothing reads it later.
    let compress =
        json!({ "id": "cmp", "type": "image-compress", "parameters": { "quality": 40 } });
    for looped in [false, true] {
        let (dir, [x, y], before) = two_images();
        let steps = [
            compress.clone(),
            rename("-compressed", ""),
            copy_into(dir.path(), "overwrite"),
        ];
        let nodes = if looped {
            vec![input(), in_loop(&steps), no_output()]
        } else {
            [vec![input()], steps.to_vec(), vec![no_output()]].concat()
        };
        let output = run(&recipe(&nodes), &[&x, &y]);
        let stderr = stderr(&output);
        assert!(output.status.success(), "loop {looped}: {stderr}");
        assert!(size(&x) < before[0], "loop {looped}: {stderr}");
        assert!(size(&y) < before[1], "loop {looped}: {stderr}");
    }
}
