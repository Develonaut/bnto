// Progressive loop output through the built binary — each iteration's files
// are written to `-o` as the loop runs, and path references to the user's own
// files must be copied there, never moved.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

const NAMES: [&str; 2] = ["a.txt", "b.txt"];

/// A root folder holding `a.txt` and `b.txt`, each containing its own name.
fn root_fixture() -> tempfile::TempDir {
    let root = temp_output_dir();
    for name in NAMES {
        std::fs::write(root.path().join(name), name).unwrap();
    }
    root
}

/// Run `file-list-loop-progressive` with `Root` set to `root`, writing to `out`.
fn run_progressive(root: &Path, out: &Path) -> Output {
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let body = serde_json::json!({ "Root": root.to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    Command::new(bnto_bin())
        .args([
            "run",
            &custom_recipe_path("file-list-loop-progressive"),
            "--yes",
        ])
        .args(["--vars", vars_file.to_str().unwrap()])
        .args(["-o", out.to_str().unwrap()])
        .output()
        .unwrap()
}

#[test]
fn test_progressive_write_copies_user_files_and_leaves_the_originals() {
    let root = root_fixture();
    let out = temp_output_dir();
    let output = run_progressive(root.path(), out.path());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    for name in NAMES {
        let original = root.path().join(name);
        assert!(original.exists(), "{name} was moved out of the root");
        assert_eq!(std::fs::read(&original).unwrap(), name.as_bytes());
        assert_eq!(
            std::fs::read(out.path().join(name)).unwrap(),
            name.as_bytes()
        );
    }
}

#[test]
fn test_progressive_write_leaves_only_the_outputs_in_the_output_dir() {
    // The final write after the loop rewrites the same names; no extra files.
    let root = root_fixture();
    let out = temp_output_dir();
    let output = run_progressive(root.path(), out.path());
    assert!(output.status.success());
    let mut written: Vec<String> = std::fs::read_dir(out.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    written.sort();
    assert_eq!(written, NAMES);
}
