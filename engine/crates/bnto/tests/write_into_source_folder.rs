// Write-mode output into a folder that holds the run's inputs or links.
//
// A file already at its destination is left as is, never truncated by a
// copy onto itself (ISS-36). A write never replaces an input a later write
// still reads (ISS-48), in a final or a progressive write. Destinations that
// are one file on disk (a link, a folder link, a filesystem that treats two
// names as one) are refused before anything is written. Every run uses the
// sandboxed HOME.

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

// --- two destinations that are one file through a link ---

/// `file-rename` turning `a` into `l`: `a.txt` comes out named `l.txt`.
fn a_to_l() -> serde_json::Value {
    json!({ "id": "ren", "type": "file-rename", "parameters": { "find": "a", "replace": "l" } })
}

/// Run a → l into `dir`, where `l.txt` is a link to `b.txt`, and check the
/// run refused before writing: `b.txt`, reached through the link, keeps BBBB.
fn assert_refused_through_link(dir: &Path, a: &Path, b: &Path, extra: &[&str]) {
    let output = run(&recipe(dir, &[a_to_l()]), &[a, b], extra);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("one file on disk"), "{stderr}");
    assert_eq!(std::fs::read_to_string(b).unwrap(), "BBBB", "{stderr}");
    assert_eq!(std::fs::read_to_string(a).unwrap(), "AAAA", "{stderr}");
}

#[cfg(unix)]
#[test]
fn test_a_write_through_a_symlink_onto_an_input_left_as_is_is_refused() {
    // RED (cold review): `l.txt` → `b.txt`. `b`'s write leaves b.txt as is,
    // so `a` → l.txt waited for nothing and replaced b.txt through the link,
    // while the summary said b.txt was "already there, left as is".
    let (dir, [a, b]) = two_inputs();
    std::os::unix::fs::symlink("b.txt", dir.path().join("l.txt")).unwrap();
    assert_refused_through_link(dir.path(), &a, &b, &[]);
}

#[test]
fn test_a_write_through_a_hard_link_onto_an_input_left_as_is_is_refused() {
    // RED (cold review): the same loss through a hard link.
    let (dir, [a, b]) = two_inputs();
    std::fs::hard_link(&b, dir.path().join("l.txt")).unwrap();
    assert_refused_through_link(dir.path(), &a, &b, &[]);
}

#[test]
fn test_a_dry_run_through_a_link_is_refused_too() {
    let (dir, [a, b]) = two_inputs();
    std::fs::hard_link(&b, dir.path().join("l.txt")).unwrap();
    assert_refused_through_link(dir.path(), &a, &b, &["--dry-run"]);
}

#[test]
fn test_two_inputs_that_are_one_file_left_as_is_still_run() {
    // Guard: both writes leave the file as is, so nothing can be lost.
    let (dir, [_, b]) = two_inputs();
    let link = dir.path().join("l.txt");
    std::fs::hard_link(&b, &link).unwrap();
    let output = run(&recipe(dir.path(), &[]), &[&b, &link], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("2 already there, left as is"), "{stderr}");
    assert_eq!(std::fs::read_to_string(&b).unwrap(), "BBBB");
}

#[cfg(unix)]
#[test]
fn test_two_new_files_that_are_one_path_through_a_folder_link_are_refused() {
    // RED (ISS-48 review 2): `out/sub` → `.`, so `sub/b.txt` (a renamed) and
    // `b.txt` are one new file. Neither existed, so the identity check saw
    // nothing; the run said `2 files written` and `out` held only BBBB.
    let (_inputs, [a, b]) = two_inputs();
    let out = temp_output_dir();
    std::os::unix::fs::symlink(".", out.path().join("sub")).unwrap();
    let to_sub = json!({ "id": "ren", "type": "file-rename",
        "parameters": { "find": "a", "replace": "sub/b" } });
    let output = run(&recipe(out.path(), &[to_sub]), &[&a, &b], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("one file on disk"), "{stderr}");
    assert!(!out.path().join("b.txt").exists(), "nothing is written");
}

/// `sub/b` replacing `a`: `a.txt` comes out named `sub/b.txt`.
#[cfg(unix)]
fn a_to_sub_b() -> serde_json::Value {
    json!({ "id": "ren", "type": "file-rename", "parameters": { "find": "a", "replace": "sub/b" } })
}

#[cfg(unix)]
#[test]
fn test_a_progressive_refusal_names_what_iterations_already_wrote() {
    // RED (ISS-48 review 3): with `out/sub` → `.`, iteration 1 wrote
    // `sub/b.txt` (= `b.txt`) before the final write saw the conflict, yet
    // the refusal said "nothing was written".
    let (inputs, _) = two_inputs();
    let out = temp_output_dir();
    std::os::unix::fs::symlink(".", out.path().join("sub")).unwrap();
    let mut recipe = progressive_rename_recipe(inputs.path());
    recipe["nodes"][2]["nodes"] = json!([a_to_sub_b()]);
    recipe["nodes"][3]["parameters"]["directory"] = json!(out.path().to_str().unwrap());
    let output = run(&recipe, &[], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("one file on disk"), "{stderr}");
    assert!(!stderr.contains("nothing was written"), "{stderr}");
    assert!(stderr.contains("Loop iterations already wrote"), "{stderr}");
    assert!(stderr.contains("sub/b.txt"), "{stderr}");
}

#[cfg(target_os = "macos")]
#[test]
fn test_two_new_names_one_file_by_unicode_normalization_are_refused() {
    // RED (ISS-48 review 3): APFS treats `café` composed (NFC) and
    // decomposed (NFD) as one name. Both were new, so no check matched
    // them; the run said `2 files written` and one file held TWO.
    let (one, two) = (temp_output_dir(), temp_output_dir());
    let nfc = one.path().join("caf\u{e9}");
    let nfd = two.path().join("cafe\u{301}");
    std::fs::write(&nfc, "ONE").unwrap();
    std::fs::write(&nfd, "TWO").unwrap();
    let out = temp_output_dir();
    let output = run(&recipe(out.path(), &[]), &[&nfc, &nfd], &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("one file on disk"), "{stderr}");
    assert!(stderr.contains("nothing was written"), "{stderr}");
    let left = std::fs::read_dir(out.path()).unwrap().count();
    assert_eq!(left, 0, "nothing is written: {stderr}");
}
