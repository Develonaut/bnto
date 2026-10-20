// report's `{{file}}` through the built binary: a message that reads it
// prints a line per incoming file, naming that file.

mod helpers;

use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

/// Run the report-file-names fixture on empty files with these names.
fn run_report(names: &[&str]) -> Output {
    let work = temp_output_dir();
    let paths: Vec<String> = names
        .iter()
        .map(|name| {
            let path = work.path().join(name);
            std::fs::write(&path, b"").unwrap();
            path.to_string_lossy().into_owned()
        })
        .collect();
    Command::new(bnto_bin())
        .args(["run", &custom_recipe_path("report-file-names"), "--yes"])
        .args(&paths)
        .output()
        .unwrap()
}

/// The index of the line that holds `text`, panicking with `stderr` if none.
fn line_index(stderr: &str, text: &str) -> usize {
    stderr
        .lines()
        .position(|l| l.contains(text))
        .unwrap_or_else(|| panic!("no line with {text:?} in: {stderr}"))
}

#[test]
fn test_report_names_each_incoming_file_in_order() {
    let output = run_report(&["utils.ts", "hooks.ts"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let first = line_index(&stderr, "got utils.ts");
    let second = line_index(&stderr, "got hooks.ts");
    assert!(first < second, "out of order: {stderr}");
    assert!(!stderr.contains("{{file}}"), "stderr: {stderr}");
}

#[test]
fn test_check_passes_the_fixture() {
    let output = Command::new(bnto_bin())
        .args(["check", &custom_recipe_path("report-file-names")])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "stdout: {stdout}");
    assert!(stdout.contains("Check: PASS"), "{stdout}");
}
