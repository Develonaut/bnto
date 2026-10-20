// SEC-01..03: template values land in argv as data, one value per slot.
// This is GitHub's "preferred mitigation #1" — the value never generates
// script text — and bnto's core guarantee. All guards, all passing.
#![cfg(unix)]

use serde_json::json;

use crate::support::{Lab, payloads, step};

/// SEC-01: a field value full of shell syntax is printed, never run.
#[test]
fn sec01_field_values_reach_argv_literally() {
    let lab = Lab::new();
    let recipe = lab.recipe(step(
        json!({ "command": "printf", "args": ["[%s]", "{{fields.v}}"] }),
        &["v"],
    ));
    for payload in payloads(&lab.canary()) {
        let param = format!("v={payload}");
        let out = lab.run(&recipe, &[&param]);
        assert!(out.ok, "payload {payload:?}: {}", out.text);
        assert!(!lab.canary_fired(), "payload {payload:?} executed");
    }
    let out = lab.run(&recipe, &["v=$(id)"]);
    assert!(out.text.contains("[$(id)]"), "{}", out.text);
}

/// SEC-02: spaces and quotes don't split a value into several arguments
/// (Home Assistant's shlex re-split does; bnto must not).
#[test]
fn sec02_one_value_is_one_argument() {
    let lab = Lab::new();
    let recipe = lab.recipe(step(
        json!({ "command": "printf", "args": ["<%s>", "{{fields.v}}"] }),
        &["v"],
    ));
    let out = lab.run(&recipe, &["v=a b 'c d' \"e\" -f"]);
    assert!(out.ok, "{}", out.text);
    assert!(out.text.contains("<a b 'c d' \"e\" -f>"), "{}", out.text);
}

/// SEC-03: hostile file names fed through an `each` group stay data.
/// File names are attacker-controlled whenever the files were downloaded.
#[test]
fn sec03_file_names_reach_argv_literally() {
    // Steps run in bnto's working directory (the lab), so a relative
    // `touch canary` in the name would hit the lab's canary.
    let lab = Lab::new();
    let file = lab.path().join("$(touch canary)`touch canary`.txt");
    std::fs::write(&file, b"x").unwrap();
    let recipe = lab.file_recipe(step(
        json!({ "command": "printf", "args": ["[%s]", { "each": ["{{file}}"] }] }),
        &[],
    ));
    let out = lab.run_with(&recipe, &[], &[file.to_str().unwrap()], &[]);
    assert!(out.ok, "{}", out.text);
    assert!(!lab.canary_fired(), "file name executed: {}", out.text);
    assert!(out.text.contains("$(touch"), "{}", out.text);
}
