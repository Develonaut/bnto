// SEC-17..18: a substituted value is data, never re-read as a template.
// Templates fill in stages (fields, then env, ctx, item, node), each stage
// rescanning the text the earlier ones produced — so a value can smuggle a
// placeholder in (CodeQL's `${{ env.X }}` re-templating lesson).
#![cfg(unix)]

use serde_json::json;

use crate::support::{Lab, step};

/// SEC-17: a field value of `{{env.SECRET}}` stays literal; it must not pull
/// the secret into the command line, where it could be sent anywhere.
#[test]
fn sec17_values_are_not_rescanned_for_templates() {
    let lab = Lab::new();
    let recipe = lab.recipe(step(
        json!({ "command": "printf", "args": ["[%s]", "x={{fields.v}}"] }),
        &["v"],
    ));
    let secret = [("BNTO_PROBE_SECRET", "hunter2")];
    let out = lab.run_with(&recipe, &["v={{env.BNTO_PROBE_SECRET}}"], &[], &secret);
    assert!(out.ok, "{}", out.text);
    assert!(!out.text.contains("hunter2"), "secret leaked: {}", out.text);
    assert!(
        out.text.contains("[x={{env.BNTO_PROBE_SECRET}}]"),
        "{}",
        out.text
    );
}

/// SEC-18: a value that contains its own placeholder must not loop forever.
#[test]
fn sec18_self_referencing_values_terminate() {
    let lab = Lab::new();
    let recipe = lab.recipe(step(
        json!({ "command": "printf", "args": ["[%s]", "x={{env.BNTO_PROBE_LOOP}}"] }),
        &[],
    ));
    let looping = [("BNTO_PROBE_LOOP", "{{env.BNTO_PROBE_LOOP}}")];
    let out = lab
        .run_within(10, &recipe, &[], &looping)
        .expect("bnto hung filling a self-referencing template");
    assert!(out.ok, "{}", out.text);
}

/// SEC-27: shell-command fills {{output_dir}}, {{url}}, {{input}} and each
/// groups' tokens itself, after values are in. A value carrying one can't
/// be told from the recipe's own, so it's refused, whichever way it came.
#[test]
fn sec27_values_carrying_shell_command_tokens_are_refused() {
    let lab = Lab::new();
    let field = lab.recipe(step(
        json!({ "command": "printf", "args": ["[%s]", "--title={{fields.v}}"] }),
        &["v"],
    ));
    let out = lab.run(&field, &["v=x{{output_dir}}"]);
    assert!(
        !out.ok && out.text.contains("fills itself"),
        "field: {}",
        out.text
    );
    assert!(lab.run(&field, &["v=plain"]).ok, "an ordinary value runs");

    let named = lab.path().join("clip{{url}}.txt");
    std::fs::write(&named, b"x").unwrap();
    let each = lab.file_recipe(step(
        json!({ "command": "printf", "args": ["[%s]", { "each": ["{{path}}"] }] }),
        &[],
    ));
    let out = lab.run_with(&each, &[], &[named.to_str().unwrap()], &[]);
    assert!(
        !out.ok && out.text.contains("fills itself"),
        "file name: {}",
        out.text
    );

    let url = lab.recipe_with(
        step(json!({ "command": "printf", "args": ["[%s]"] }), &[]),
        json!({ "mode": "url", "accept": [], "extensions": [], "label": "URL", "multiple": true }),
    );
    let out = lab.run_with(&url, &[], &["https://example.com/{{output_dir}}"], &[]);
    assert!(
        !out.ok && out.text.contains("fills itself"),
        "url: {}",
        out.text
    );
}
