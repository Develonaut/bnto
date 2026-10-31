// A loop that fails passes its step's error up with the iteration that
// failed, so the loop's line and the run's say which one (ISS-108).

mod helpers;

use helpers::{Sandbox, temp_output_dir};
use serde_json::json;

#[test]
fn a_failed_iteration_is_named_on_the_loops_line_and_the_runs() {
    let dir = temp_output_dir();
    let recipe = json!({ "name": "Loop Failure", "nodes": [
        { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
        { "id": "rows", "type": "spreadsheet-read" },
        { "id": "each", "type": "loop", "nodes": [
            { "id": "check", "type": "assert",
              "parameters": { "value": "{{item.name}}", "op": "not-equals", "expected": "Grace" } }
        ] },
        { "id": "output", "type": "output", "parameters": { "mode": "none" } }
    ] });
    let path = dir.path().join("loop.bnto.json");
    std::fs::write(&path, recipe.to_string()).unwrap();
    let table = dir.path().join("people.csv");
    std::fs::write(&table, "name\nAda\nGrace\n").unwrap();

    let output = Sandbox::new()
        .command()
        .arg("run")
        .arg(&path)
        .arg(&table)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    let step = "FAILED check: Processing failed: Assertion failed: people_row1.csv";
    assert!(
        stderr.contains(step),
        "the step's own line is unchanged: {stderr}"
    );
    let each = "FAILED each: Processing failed: iteration 2 of 2: Assertion failed";
    assert!(stderr.contains(each), "{stderr}");
    let run = "Pipeline failed: Processing failed: iteration 2 of 2: Assertion failed";
    assert!(stderr.contains(run), "{stderr}");
}
