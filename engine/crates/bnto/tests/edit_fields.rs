// edit-fields through the built binary — CSV rows read into items, a field
// set from each row's other columns, and a later node reading it.

mod helpers;

use std::process::{Command, Output};

use helpers::{bnto_bin, custom_recipe_path, output_files, temp_output_dir};

/// Run the edit-fields-rename fixture on a CSV holding `csv`, writing to `out`.
fn run_on_csv(csv: &str, out: &tempfile::TempDir) -> Output {
    let src = tempfile::tempdir().unwrap();
    let table = src.path().join("people.csv");
    std::fs::write(&table, csv).unwrap();
    Command::new(bnto_bin())
        .args(["run", &custom_recipe_path("edit-fields-rename"), "--yes"])
        .arg(&table)
        .args(["-o", out.path().to_str().unwrap()])
        .output()
        .unwrap()
}

fn names(out: &tempfile::TempDir) -> Vec<String> {
    let mut names: Vec<String> = output_files(out)
        .iter()
        .map(|f| f.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn test_rows_are_named_after_a_field_edit_fields_set() {
    let out = temp_output_dir();
    let csv = "first,last,meta.id\nAda,Lovelace,1\nGrace,Hopper,2\n";
    let output = run_on_csv(csv, &out);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(names(&out), ["Ada-Lovelace.txt", "Grace-Hopper.txt"]);
}

#[test]
fn test_an_unknown_field_fails_the_run_naming_it_and_writes_nothing() {
    let out = temp_output_dir();
    let output = run_on_csv("first,meta.id\nAda,1\n", &out);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("\"last\""), "names the field: {stderr}");
    assert!(
        stderr.contains("people_row0.csv"),
        "names the item: {stderr}"
    );
    assert!(names(&out).is_empty(), "wrote {:?}", names(&out));
}

#[test]
fn test_check_reports_edit_fields_without_a_values_map() {
    let recipe = custom_recipe_path("check/edit-fields-bad");
    let output = Command::new(bnto_bin())
        .args(["check", &recipe])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "stdout: {stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.last(), Some(&"Check: FAIL (3 findings)"), "{stdout}");
    assert!(
        lines[0].contains("\"no-values\"") && lines[0].contains("'values' is required"),
        "{stdout}"
    );
    // A map is no value `-p` can give, so leaving it out is no run-time param.
    assert!(
        lines[1].contains("\"no-values\"") && lines[1].contains("no run can give it"),
        "{stdout}"
    );
    assert!(
        lines[2].contains("\"not-a-map\"") && lines[2].contains("map of field name"),
        "{stdout}"
    );
}
