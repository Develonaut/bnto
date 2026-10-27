// file-move and file-delete on a file the run still reads outside their
// batch (ISS-82).
//
// Inside a loop, each iteration hands the node one file, so a file given
// twice reaches two iterations. Moving or deleting it in the first used to
// fail the second partway ("doesn't exist"). Now the first refuses, before
// anything moves or is deleted, naming the file. Every run uses the
// sandboxed HOME and temp folders.

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::{Value, json};

fn input() -> Value {
    json!({ "id": "input", "type": "input", "parameters": { "mode": "files" } })
}

fn no_output() -> Value {
    json!({ "id": "output", "type": "output", "parameters": { "mode": "none" } })
}

fn in_loop(children: &[Value]) -> Value {
    json!({
        "id": "each", "type": "loop", "parameters": { "mode": "forEach" },
        "nodes": children, "edges": []
    })
}

/// Run a recipe of `input`, `children` in a loop and no output on `files`.
fn run_loop(children: &[Value], files: &[&Path]) -> Output {
    let recipe = json!({
        "name": "Held Sources",
        "nodes": [input(), in_loop(children), no_output()]
    });
    let dir = temp_output_dir();
    let path = dir.path().join("held.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    Sandbox::new()
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

#[test]
fn a_move_of_a_file_another_iteration_reads_is_refused() {
    let inputs = temp_output_dir();
    let out = temp_output_dir();
    let a = inputs.path().join("a.txt");
    std::fs::write(&a, "AAAA").unwrap();
    let mv = json!({
        "id": "mv", "type": "file-move",
        "parameters": { "destination": out.path().to_str().unwrap() }
    });

    let output = run_loop(&[mv], &[&a, &a]);

    let stderr = stderr(&output);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("File move refused"), "{stderr}");
    assert!(
        stderr.contains("a later step still reads it as a.txt"),
        "{stderr}"
    );
    assert!(!stderr.contains("doesn't exist"), "{stderr}");
    assert_eq!(std::fs::read_to_string(&a).unwrap(), "AAAA", "never moved");
    assert!(!out.path().join("a.txt").exists(), "{stderr}");
}

#[test]
fn a_delete_of_a_file_another_iteration_reads_is_refused() {
    let inputs = temp_output_dir();
    let out = temp_output_dir();
    let a = inputs.path().join("a.txt");
    std::fs::write(&a, "AAAA").unwrap();
    let cp = json!({
        "id": "cp", "type": "file-copy",
        "parameters": { "destination": out.path().to_str().unwrap(), "conflict": "rename" }
    });
    let rm = json!({
        "id": "rm", "type": "file-delete",
        "parameters": { "base": inputs.path().to_str().unwrap() }
    });

    let output = run_loop(&[cp, rm], &[&a, &a]);

    let stderr = stderr(&output);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("File delete refused"), "{stderr}");
    assert!(
        stderr.contains("a later step still reads it as a.txt"),
        "{stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&a).unwrap(),
        "AAAA",
        "never deleted"
    );
}

#[test]
fn moving_each_files_own_source_in_a_loop_is_allowed() {
    let inputs = temp_output_dir();
    let out = temp_output_dir();
    let (a, b) = (inputs.path().join("a.txt"), inputs.path().join("b.txt"));
    std::fs::write(&a, "AAAA").unwrap();
    std::fs::write(&b, "BBBB").unwrap();
    let mv = json!({
        "id": "mv", "type": "file-move",
        "parameters": { "destination": out.path().to_str().unwrap() }
    });

    let output = run_loop(&[mv], &[&a, &b]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(out.path().join("a.txt")).unwrap(),
        "AAAA"
    );
    assert_eq!(
        std::fs::read_to_string(out.path().join("b.txt")).unwrap(),
        "BBBB"
    );
}

#[test]
fn deleting_each_files_own_source_in_a_loop_is_allowed() {
    let inputs = temp_output_dir();
    let (a, b) = (inputs.path().join("a.txt"), inputs.path().join("b.txt"));
    std::fs::write(&a, "AAAA").unwrap();
    std::fs::write(&b, "BBBB").unwrap();
    let rm = json!({
        "id": "rm", "type": "file-delete",
        "parameters": { "base": inputs.path().to_str().unwrap() }
    });

    let output = run_loop(&[rm], &[&a, &b]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(!a.exists() && !b.exists());
}

#[test]
fn a_branch_delete_of_a_file_the_output_branch_reads_is_refused() {
    // Without a loop: the delete's branch and the output's both get `a.txt`
    // from the input, and the output writes after the delete would run.
    let inputs = temp_output_dir();
    let out = temp_output_dir();
    let a = inputs.path().join("a.txt");
    std::fs::write(&a, "AAAA").unwrap();
    let recipe = json!({
        "name": "Held Sources",
        "nodes": [
            input(),
            json!({ "id": "rm", "type": "file-delete",
                "parameters": { "base": inputs.path().to_str().unwrap() } }),
            json!({ "id": "output", "type": "output",
                "parameters": { "mode": "write", "directory": out.path().to_str().unwrap() } }),
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "rm" },
            { "id": "e2", "source": "input", "target": "output" }
        ]
    });
    let dir = temp_output_dir();
    let path = dir.path().join("held.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();

    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&path)
        .arg(&a)
        .arg("--yes")
        .output()
        .unwrap();

    let stderr = stderr(&output);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("File delete refused"), "{stderr}");
    assert_eq!(
        std::fs::read_to_string(&a).unwrap(),
        "AAAA",
        "never deleted"
    );
}

#[test]
fn a_refusal_after_earlier_iterations_moved_files_claims_only_its_own_step() {
    // `b` moves in iteration 1; iteration 2 holds `a`, which iteration 3
    // still reads, so it is refused without claiming nothing moved at all.
    let inputs = temp_output_dir();
    let out = temp_output_dir();
    let (a, b) = (inputs.path().join("a.txt"), inputs.path().join("b.txt"));
    std::fs::write(&a, "AAAA").unwrap();
    std::fs::write(&b, "BBBB").unwrap();
    let mv = json!({
        "id": "mv", "type": "file-move",
        "parameters": { "destination": out.path().to_str().unwrap() }
    });

    let output = run_loop(&[mv], &[&b, &a, &a]);

    let stderr = stderr(&output);
    assert!(!output.status.success(), "{stderr}");
    assert!(
        stderr.contains("File move refused, this step moved nothing"),
        "{stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(out.path().join("b.txt")).unwrap(),
        "BBBB"
    );
    assert_eq!(std::fs::read_to_string(&a).unwrap(), "AAAA", "never moved");
}

#[test]
fn a_loop_delete_of_one_file_given_twice_is_refused() {
    // Flat, the batch deletes it once and skips the second as missing. In a
    // loop the second iteration still holds it, so the first is refused.
    let inputs = temp_output_dir();
    let a = inputs.path().join("a.txt");
    std::fs::write(&a, "AAAA").unwrap();
    let rm = json!({
        "id": "rm", "type": "file-delete",
        "parameters": { "base": inputs.path().to_str().unwrap() }
    });

    let output = run_loop(&[rm], &[&a, &a]);

    let stderr = stderr(&output);
    assert!(!output.status.success(), "{stderr}");
    assert!(
        stderr.contains("File delete refused, this step deleted nothing"),
        "{stderr}"
    );
    assert!(a.exists(), "never deleted");
}
