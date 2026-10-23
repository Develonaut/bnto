// SEC-15..16: consent is the real boundary for who wrote the commands.
// A non-bundled recipe with a shell step asks first, the approval is pinned
// to the definition's hash, and the prompt must show what will run
// (Coder CVE-2026-44454: a path that skipped confirmation became one-click).

use serde_json::json;

use crate::support::{Lab, step};

fn git_recipe(lab: &Lab, args: serde_json::Value) -> String {
    lab.recipe(step(json!({ "command": "git", "args": args }), &[]))
}

/// SEC-15: approval covers exactly the approved definition. Any edit —
/// here one extra argument — asks again, and declining runs nothing.
#[test]
fn sec15_editing_an_approved_recipe_asks_again() {
    let lab = Lab::new();
    let recipe = git_recipe(&lab, json!(["--version"]));
    assert!(
        !lab.run_answering(&recipe, "n\n").ok,
        "declined run went ahead"
    );
    assert!(lab.run_answering(&recipe, "y\n").ok, "approved run failed");
    assert!(
        lab.run_answering(&recipe, "").ok,
        "approval wasn't remembered"
    );

    git_recipe(&lab, json!(["--version", "--build-options"]));
    let edited = lab.run_answering(&recipe, "");
    assert!(
        !edited.ok,
        "edited recipe ran on the old approval: {}",
        edited.text
    );
    assert!(
        edited.text.contains("Execution cancelled"),
        "{}",
        edited.text
    );
}

/// SEC-16: the prompt shows the full command line, not just the program,
/// so the person approving sees the args (and, in script mode, the script).
#[test]
fn sec16_consent_shows_the_full_command_line() {
    let lab = Lab::new();
    let mut node = step(
        json!({ "command": "git", "args": ["log", "--format=%H", "{{fields.v}}"] }),
        &["v"],
    );
    // A value for `v`: an unset field is refused before the prompt (ISS-1).
    node["fields"]["v"]["default"] = json!("HEAD");
    let recipe = lab.recipe(node);
    let out = lab.run_answering(&recipe, "n\n");
    assert!(
        out.text.contains("git log --format=%H {{fields.v}}"),
        "{}",
        out.text
    );
}

/// SEC-29: `{{vars.*}}` are filled before consent, so approval covers the
/// values that run. A different value asks again, and the prompt shows the
/// value, not the template.
#[test]
fn sec29_consent_covers_the_values_vars_fill_in() {
    let lab = Lab::new();
    let recipe = git_recipe(&lab, json!(["{{vars.Flag}}"]));
    let version = [("BNTO_VAR_FLAG", "--version")];
    let first = lab.run_answering_with(&recipe, "y\n", &version);
    assert!(first.ok, "{}", first.text);
    assert!(
        first.text.contains("git --version"),
        "prompt must show the value: {}",
        first.text
    );
    assert!(
        lab.run_answering_with(&recipe, "", &version).ok,
        "same value runs"
    );

    let changed = lab.run_answering_with(&recipe, "", &[("BNTO_VAR_FLAG", "--help")]);
    assert!(
        !changed.ok,
        "a new var value ran on the old approval: {}",
        changed.text
    );
    assert!(
        changed.text.contains("Execution cancelled"),
        "{}",
        changed.text
    );
}
