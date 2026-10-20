// SEC-13: enum fields are allowlists. Bundled recipes put enum values
// straight into option slots (download-video's `--embed-thumbnail`,
// keep-awake's caffeinate flag), so a value outside `options` must never
// reach a command — OWASP's layer-2 allowlist validation.

use serde_json::json;

use crate::support::{Lab, step};

fn enum_step() -> serde_json::Value {
    let mut step = step(
        json!({ "command": "git", "args": ["{{fields.mode}}"] }),
        &[],
    );
    step["fields"] = json!({
        "mode": {
            "type": "enum", "label": "Mode", "default": "--version",
            "options": [
                { "value": "--version", "label": "Version" },
                { "value": "--help", "label": "Help" }
            ]
        }
    });
    step
}

/// SEC-13: `--param` with a value outside the options is refused, naming
/// the allowed values, before the step runs.
#[test]
fn sec13_enum_values_must_be_listed_options() {
    let lab = Lab::new();
    let recipe = lab.recipe(enum_step());
    for value in ["--exec=touch canary", "-c", "version"] {
        let out = lab.run(&recipe, &[&format!("mode={value}")]);
        assert!(!out.ok, "accepted mode={value:?}: {}", out.text);
        assert!(
            out.text.contains("--help"),
            "error should list options: {}",
            out.text
        );
    }
}

/// A listed value still runs.
#[test]
fn sec13_listed_enum_values_run() {
    let lab = Lab::new();
    let out = lab.run(&lab.recipe(enum_step()), &["mode=--version"]);
    assert!(out.ok, "{}", out.text);
    assert!(out.text.contains("git version"), "{}", out.text);
}
