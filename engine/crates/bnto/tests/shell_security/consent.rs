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
    let recipe = lab.recipe(step(
        json!({ "command": "git", "args": ["log", "--format=%H", "{{fields.v}}"] }),
        &["v"],
    ));
    let out = lab.run_answering(&recipe, "n\n");
    assert!(
        out.text.contains("git log --format=%H {{fields.v}}"),
        "{}",
        out.text
    );
}
