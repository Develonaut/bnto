// SEC-14: URL-mode input is appended to the command's args, so it must be
// an http(s) URL — never another scheme, and never an option (`-o...`).
#![cfg(unix)]

use serde_json::json;

use crate::support::{Lab, step};

fn url_recipe(lab: &Lab) -> String {
    lab.recipe_with(
        step(json!({ "command": "printf", "args": ["[%s]"] }), &[]),
        json!({ "mode": "url", "accept": [], "extensions": [], "label": "URL", "multiple": true }),
    )
}

/// SEC-14: non-http(s) values are refused before any step runs.
#[test]
fn sec14_url_inputs_must_be_http() {
    let lab = Lab::new();
    let recipe = url_recipe(&lab);
    for input in [
        "-oProxyCommand=touch canary",
        "--exec=touch canary",
        "file:///etc/passwd",
        "ftp://example.com/x",
        "javascript:alert(1)",
    ] {
        let out = lab.run_with(&recipe, &[], &["--", input], &[]);
        assert!(!out.ok, "accepted {input:?}: {}", out.text);
        assert!(!lab.canary_fired(), "{input:?} executed");
    }
}

/// An https URL arrives as one literal argument.
#[test]
fn sec14_http_urls_reach_argv_literally() {
    let lab = Lab::new();
    let url = "https://example.com/watch?v=1&list=$(id);x";
    let out = lab.run_with(&url_recipe(&lab), &[], &[url], &[]);
    assert!(out.ok, "{}", out.text);
    assert!(out.text.contains(&format!("[{url}]")), "{}", out.text);
}
