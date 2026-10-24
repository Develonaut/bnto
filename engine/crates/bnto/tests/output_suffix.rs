// A write-mode output keeps two results with one name apart by writing the
// later as `stem_N.ext`. That made-up name must never replace a file already
// in the output folder: only a file named like an output is replaced. Every
// run uses the sandboxed HOME.

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::{Sandbox, temp_output_dir};
use serde_json::json;

/// `in/a.txt` (`AAAA`) and `in/b.txt` (`BBBB`), and an `out/` already
/// holding the user's own `b_1.txt` (`CCCC`).
fn fixture() -> tempfile::TempDir {
    let root = temp_output_dir();
    for dir in ["in", "out"] {
        std::fs::create_dir(root.path().join(dir)).unwrap();
    }
    std::fs::write(root.path().join("in/a.txt"), "AAAA").unwrap();
    std::fs::write(root.path().join("in/b.txt"), "BBBB").unwrap();
    std::fs::write(root.path().join("out/b_1.txt"), "CCCC").unwrap();
    root
}

/// Rename `a` to `b` (so both results are `b.txt`), then write into `out`;
/// in a progressive loop when `progressive`, so files go out as it runs.
fn recipe(out: &Path, progressive: bool) -> serde_json::Value {
    let rename = json!({
        "id": "ren", "type": "file-rename", "parameters": { "find": "a", "replace": "b" }
    });
    let step = if progressive {
        json!({
            "id": "each", "type": "loop",
            "parameters": { "mode": "forEach", "outputPersistence": "progressive" },
            "nodes": [rename], "edges": []
        })
    } else {
        rename
    };
    json!({ "name": "Suffix", "nodes": [
        { "id": "input", "type": "input", "parameters": { "mode": "files" } },
        step,
        { "id": "output", "type": "output",
          "parameters": { "mode": "write", "directory": out.to_str().unwrap() } }
    ]})
}

/// Run `recipe` on the fixture's two inputs.
fn run(sandbox: &Sandbox, root: &Path, recipe: &serde_json::Value, extra: &[&str]) -> Output {
    let path = root.join("suffix.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    sandbox
        .command()
        .arg("run")
        .arg(&path)
        .arg(root.join("in/a.txt"))
        .arg(root.join("in/b.txt"))
        .arg("--yes")
        .args(extra)
        .output()
        .unwrap()
}

/// Each file in `dir` with its content, sorted by name.
fn contents(dir: &Path) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(&path).unwrap())
        })
        .collect();
    files.sort();
    files
}

/// `out/` once the run kept the user's file: `b`'s result went to `b_2.txt`.
fn kept() -> Vec<(String, String)> {
    [("b.txt", "AAAA"), ("b_1.txt", "CCCC"), ("b_2.txt", "BBBB")]
        .map(|(n, c)| (n.to_string(), c.to_string()))
        .to_vec()
}

/// Run the fixture with `recipe` and check it kept the user's file.
fn assert_run_keeps_the_users_file(progressive: bool) {
    let root = fixture();
    let out = root.path().join("out");
    let output = run(
        &Sandbox::new(),
        root.path(),
        &recipe(&out, progressive),
        &[],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(contents(&out), kept(), "stderr: {stderr}");
    assert!(stderr.contains("b.txt → b_2.txt"), "{stderr}");
    // A progressive loop copied `b_2.txt` out already, and the final write
    // can't tell that copy from an earlier run's identical file, so it's
    // left as is and reported so. `b.txt` is a result's own name: written.
    let done = if progressive {
        "1 file written to"
    } else {
        "2 files written to"
    };
    assert!(stderr.contains(done), "{stderr}");
    assert_eq!(
        stderr.contains("1 already there, left as is"),
        progressive,
        "{stderr}"
    );
}

#[test]
fn test_a_made_up_name_never_replaces_a_file_already_in_the_folder() {
    // RED (data loss): `b`'s result was written as `b_1.txt` over the user's
    // own `b_1.txt`, and the run said `Done. 2 files written`.
    assert_run_keeps_the_users_file(false);
}

#[test]
fn test_a_progressive_loop_never_replaces_a_file_at_a_made_up_name() {
    // RED: the loop's copy went to `b_1.txt` first; the final write must then
    // pick the same `b_2.txt`, leaving no second copy behind.
    assert_run_keeps_the_users_file(true);
}

#[test]
fn test_a_dry_run_names_the_made_up_name_a_run_would_use() {
    // RED: the dry run said `b.txt → b_1.txt`, the file it would replace.
    let root = fixture();
    let out = root.path().join("out");
    let before = contents(&out);
    let output = run(
        &Sandbox::new(),
        root.path(),
        &recipe(&out, false),
        &["--dry-run"],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("would be written under a new name") && stderr.contains("b.txt → b_2.txt"),
        "{stderr}"
    );
    assert_eq!(contents(&out), before, "a dry run wrote: {stderr}");
}

#[test]
fn test_a_rerun_leaves_its_own_identical_result_as_is() {
    // RED: the rerun found its own `b_2.txt` holding the same bytes, took
    // the name (not `b_3`), but copied over it, so the file got the source's
    // mode and a new mtime. It's this run's result already: left as is and
    // reported so, while `b.txt`, a result's own name, is still written.
    let root = fixture();
    let out = root.path().join("out");
    let sandbox = Sandbox::new();
    let first = run(&sandbox, root.path(), &recipe(&out, false), &[]);
    assert!(first.status.success(), "{first:?}");
    let b2 = out.join("b_2.txt");
    let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_577_836_800);
    let file = std::fs::File::options().write(true).open(&b2).unwrap();
    file.set_modified(old).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&b2, std::fs::Permissions::from_mode(0o600)).unwrap();
    }

    let second = run(&sandbox, root.path(), &recipe(&out, false), &[]);
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(second.status.success(), "stderr: {stderr}");
    assert_eq!(contents(&out), kept(), "stderr: {stderr}");
    let meta = std::fs::metadata(&b2).unwrap();
    assert_eq!(meta.modified().unwrap(), old, "b_2.txt was rewritten");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    }
    assert!(
        stderr.contains("1 file written") && stderr.contains("1 already there, left as is"),
        "{stderr}"
    );
}
