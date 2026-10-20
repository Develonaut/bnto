// assert's `{{file}}` through the built binary: a filter keeps the files a
// check forbids, and an assert on `{{file}}` fails the run naming each one.

mod helpers;

use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, temp_output_dir};

/// Run the assert-no-files fixture on empty files with these names.
fn run_no_files(names: &[&str]) -> Output {
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
        .args(["run", &custom_recipe_path("assert-no-files"), "--yes"])
        .args(&paths)
        .output()
        .unwrap()
}

fn last_line(stderr: &str) -> &str {
    stderr.lines().rev().find(|l| !l.trim().is_empty()).unwrap()
}

#[test]
fn test_no_file_reaching_the_assert_ends_done() {
    let output = run_no_files(&["format.ts", "Button.tsx"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(last_line(&stderr).contains("Done."), "stderr: {stderr}");
}

#[test]
fn test_each_file_reaching_the_assert_is_named_in_the_failure() {
    let output = run_no_files(&["utils.ts", "format.ts", "hooks.ts"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    let last = last_line(&stderr);
    assert!(last.contains("Grab-bag file names:"), "last line: {last}");
    assert!(
        last.contains("utils.ts") && last.contains("hooks.ts"),
        "last line: {last}"
    );
    assert!(!last.contains("format.ts"), "last line: {last}");
    assert!(!last.contains("{{file}}"), "last line: {last}");
}

#[test]
fn test_check_passes_the_fixture() {
    let output = Command::new(bnto_bin())
        .args(["check", &custom_recipe_path("assert-no-files")])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "stdout: {stdout}");
    assert!(stdout.contains("Check: PASS"), "{stdout}");
}
