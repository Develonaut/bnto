// file-join through the built binary — file-list brings in notes.md then
// listing.md, file-join writes them as one README.md.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

const NOTES: &str = "# Sounds\n\nHand-written notes.\n";
const LISTING: &str = "- door.wav\n- rain.wav\n";

/// A root folder holding `notes.md` and `listing.md`.
fn root_fixture() -> tempfile::TempDir {
    let root = temp_output_dir();
    std::fs::write(root.path().join("notes.md"), NOTES).unwrap();
    std::fs::write(root.path().join("listing.md"), LISTING).unwrap();
    root
}

/// Run `file-list-join` with `Root` set to `root`, writing to `out`, the
/// system temp dir pointed at `tmp`, plus `extra` args.
fn run_join(root: &Path, out: &Path, tmp: &Path, extra: &[&str]) -> Output {
    let vars_dir = temp_output_dir();
    let vars_file = vars_dir.path().join("vars.json");
    let body = serde_json::json!({ "Root": root.to_str().unwrap() }).to_string();
    std::fs::write(&vars_file, body).unwrap();
    Command::new(bnto_bin())
        .env("TMPDIR", tmp)
        .env("TMP", tmp)
        .env("TEMP", tmp)
        .args(["run", &custom_recipe_path("file-list-join"), "--yes"])
        .args(["--vars", vars_file.to_str().unwrap()])
        .args(["-o", out.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap()
}

/// Every entry directly under `dir`, sorted.
fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn test_file_join_writes_the_listed_files_as_one_in_order() {
    let root = root_fixture();
    let (out, tmp) = (temp_output_dir(), temp_output_dir());
    let output = run_join(root.path(), out.path(), tmp.path(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        stderr.contains("Joined 2 files into README.md"),
        "stderr: {stderr}"
    );
    assert_eq!(entries(out.path()), ["README.md"]);
    let joined = std::fs::read_to_string(out.path().join("README.md")).unwrap();
    assert_eq!(joined, format!("{NOTES}\n{LISTING}"));
    assert_eq!(
        entries(root.path()),
        ["listing.md", "notes.md"],
        "sources stay"
    );
}

#[test]
fn test_file_join_real_run_leaves_no_temp_files() {
    // The joined file streams into a run temp file; the output step copies
    // it out, so the run must delete it afterwards.
    let root = root_fixture();
    let (out, tmp) = (temp_output_dir(), temp_output_dir());
    let output = run_join(root.path(), out.path(), tmp.path(), &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(entries(out.path()), ["README.md"]);
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
}

#[test]
fn test_file_join_dry_run_writes_nothing() {
    let root = root_fixture();
    let (out_root, tmp) = (temp_output_dir(), temp_output_dir());
    let out = out_root.path().join("out");
    let output = run_join(root.path(), &out, tmp.path(), &["--dry-run"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(!out.exists(), "output dir was created");
    let left = entries(tmp.path());
    assert!(left.is_empty(), "left in temp: {left:?}");
    assert_eq!(entries(root.path()), ["listing.md", "notes.md"]);
}
