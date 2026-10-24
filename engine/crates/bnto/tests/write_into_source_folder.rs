// A write-mode output pointed at the input's own folder (ISS-36): the file
// is already where the run would copy it, so it must be left as is, never
// truncated by a copy onto itself. Every run uses the sandboxed HOME.

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, custom_recipe_path, temp_output_dir};
use serde_json::json;

const CONTENT: &[u8] = b"important";

/// `steps` between an input and a write-mode output into `dir`.
fn recipe(dir: &Path, steps: &[serde_json::Value]) -> serde_json::Value {
    let mut nodes =
        vec![json!({ "id": "input", "type": "input", "parameters": { "mode": "files" } })];
    nodes.extend_from_slice(steps);
    nodes.push(json!({
        "id": "output", "type": "output",
        "parameters": { "mode": "write", "directory": dir.to_str().unwrap() }
    }));
    json!({ "name": "Same Folder", "nodes": nodes })
}

/// Run `recipe` on `files` inside a fresh sandbox.
fn run(recipe: &serde_json::Value, files: &[&Path], extra: &[&str]) -> Output {
    let sandbox = Sandbox::new();
    let dir = temp_output_dir();
    let path = dir.path().join("same.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    sandbox
        .command()
        .arg("run")
        .arg(&path)
        .args(files)
        .args(["--yes"])
        .args(extra)
        .output()
        .unwrap()
}

/// A folder holding `a.txt` with `CONTENT`.
fn folder() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = temp_output_dir();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, CONTENT).unwrap();
    (dir, file)
}

#[test]
fn test_writing_a_file_into_its_own_folder_leaves_it_as_is() {
    let (dir, file) = folder();
    let output = run(&recipe(dir.path(), &[]), &[&file], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(std::fs::read(&file).unwrap(), CONTENT, "stderr: {stderr}");
    assert!(
        stderr.contains("0 files written") && stderr.contains("1 already there, left as is"),
        "the summary says the file was left, not written: {stderr}"
    );
}

#[test]
fn test_a_dry_run_into_its_own_folder_says_the_file_would_be_left() {
    let (dir, file) = folder();
    let output = run(&recipe(dir.path(), &[]), &[&file], &["--dry-run"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("0 files would be written")
            && stderr.contains("1 already there, would be left as is"),
        "{stderr}"
    );
}

#[test]
fn test_a_case_only_rename_into_its_own_folder_keeps_the_content() {
    // On a case-insensitive filesystem `A.txt` is `a.txt`; elsewhere the
    // run writes a second file. Either way the source keeps its bytes.
    let (dir, file) = folder();
    let upper = json!({ "id": "ren", "type": "file-rename", "parameters": { "case": "upper" } });
    let output = run(&recipe(dir.path(), &[upper]), &[&file], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(std::fs::read(&file).unwrap(), CONTENT, "stderr: {stderr}");
    assert_eq!(std::fs::read(dir.path().join("A.txt")).unwrap(), CONTENT);
}

#[test]
fn test_file_copy_overwriting_into_the_source_folder_keeps_the_content() {
    let (dir, file) = folder();
    let copy = json!({
        "id": "cp", "type": "file-copy",
        "parameters": { "destination": dir.path().to_str().unwrap(), "conflict": "overwrite" }
    });
    let mut recipe = recipe(dir.path(), &[copy]);
    recipe["nodes"][2]["parameters"] = json!({ "mode": "none" });
    let output = run(&recipe, &[&file], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(std::fs::read(&file).unwrap(), CONTENT, "stderr: {stderr}");
}

#[test]
fn test_progressive_loop_output_into_the_listed_folder_leaves_the_files() {
    // Each iteration's files are copied out as the loop runs, before the
    // final write: the same copy, so the same guard.
    let (dir, file) = folder();
    let vars = temp_output_dir();
    let vars_file = vars.path().join("vars.json");
    let body = json!({ "Root": dir.path().to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    let output = Sandbox::new()
        .command()
        .args([
            "run",
            &custom_recipe_path("file-list-loop-progressive"),
            "--yes",
        ])
        .arg("--vars")
        .arg(&vars_file)
        .arg("-o")
        .arg(dir.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(std::fs::read(&file).unwrap(), CONTENT, "stderr: {stderr}");
}

#[test]
fn test_writing_a_file_elsewhere_still_says_written() {
    // Guard: only a file already at its destination is left as is.
    let (_dir, file) = folder();
    let out = temp_output_dir();
    let output = run(&recipe(out.path(), &[]), &[&file], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("1 file written"), "{stderr}");
    assert!(!stderr.contains("left as is"), "{stderr}");
    assert_eq!(std::fs::read(out.path().join("a.txt")).unwrap(), CONTENT);
}

// --- a write onto another input that hasn't been read yet (ISS-48) ---

/// A folder holding `a.txt` = `AAAA` and `b.txt` = `BBBB`.
fn two_inputs() -> (tempfile::TempDir, [std::path::PathBuf; 2]) {
    let dir = temp_output_dir();
    let (a, b) = (dir.path().join("a.txt"), dir.path().join("b.txt"));
    std::fs::write(&a, "AAAA").unwrap();
    std::fs::write(&b, "BBBB").unwrap();
    (dir, [a, b])
}

/// `file-rename` turning `a` into `b`: `a.txt` comes out named `b.txt`.
fn a_to_b() -> serde_json::Value {
    json!({ "id": "ren", "type": "file-rename", "parameters": { "find": "a", "replace": "b" } })
}

/// Each file in `dir` with its content, sorted by name.
fn contents(dir: &Path) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .map(|p| {
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(&p).unwrap())
        })
        .collect();
    files.sort();
    files
}

/// What the a → b runs must leave: `a.txt` untouched, `a`'s copy at
/// `b.txt`, and `b`'s content kept at its new name `b_1.txt`.
fn kept_both() -> Vec<(String, String)> {
    [("a.txt", "AAAA"), ("b.txt", "AAAA"), ("b_1.txt", "BBBB")]
        .iter()
        .map(|(n, c)| (n.to_string(), c.to_string()))
        .collect()
}

#[test]
fn test_a_rename_onto_another_inputs_name_keeps_that_inputs_content() {
    // RED (ISS-48): `a`'s output is planned as `b.txt` and `b`'s as
    // `b_1.txt`; copying `a` first replaced `b.txt` before `b` was read, so
    // all three files held AAAA and BBBB was gone.
    let (dir, [a, b]) = two_inputs();
    let output = run(&recipe(dir.path(), &[a_to_b()]), &[&a, &b], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(contents(dir.path()), kept_both(), "stderr: {stderr}");
    assert!(stderr.contains("2 files written"), "{stderr}");
}

/// `file-list` of `a.txt` and `b.txt` in `dir`, renamed `a` → `b` inside a
/// progressive loop, written back into `dir`.
fn progressive_rename_recipe(dir: &Path) -> serde_json::Value {
    let dir = dir.to_str().unwrap();
    json!({
        "name": "Progressive Rename", "type": "group",
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "list", "type": "file-list",
              "parameters": { "base": dir, "paths": ["a.txt", "b.txt"] } },
            { "id": "each", "type": "loop",
              "parameters": { "mode": "forEach", "outputPersistence": "progressive" },
              "nodes": [a_to_b()], "edges": [] },
            { "id": "output", "type": "output",
              "parameters": { "mode": "write", "directory": dir } }
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "list" },
            { "id": "e2", "source": "list", "target": "each" },
            { "id": "e3", "source": "each", "target": "output" }
        ]
    })
}

#[test]
fn test_a_progressive_rename_onto_a_later_inputs_name_keeps_its_content() {
    // RED (ISS-48): the first iteration's progressive copy replaced `b.txt`
    // before the second iteration read it.
    let (dir, _) = two_inputs();
    let output = run(&progressive_rename_recipe(dir.path()), &[], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(contents(dir.path()), kept_both(), "stderr: {stderr}");
}
