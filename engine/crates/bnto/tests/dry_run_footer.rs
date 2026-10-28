// A dry run's `Run with:` footer ends by the recipe's input mode, as `bnto
// info` does: nothing for a recipe that takes no input, `<files...>` for
// files (ISS-96).

mod helpers;

use helpers::Sandbox;

/// A recipe that takes no input: one shell step.
const NO_INPUT: &str = r#"{ "id": "say", "name": "Say",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "say", "type": "shell-run", "parameters": { "command": "echo", "args": ["hi"] } }] }"#;

fn footer(output: &std::process::Output) -> String {
    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {err}");
    out.trim_end()
        .lines()
        .last()
        .unwrap_or_default()
        .to_string()
}

#[test]
fn a_recipe_that_takes_no_input_gets_no_placeholder() {
    let sandbox = Sandbox::new();
    std::fs::write(sandbox.work_dir().join("say.bnto.json"), NO_INPUT).unwrap();
    let output = sandbox
        .command()
        .args(["dry-run", "say.bnto.json"])
        .output()
        .unwrap();
    assert_eq!(footer(&output), "Run with: bnto run say.bnto.json");
}

#[test]
fn a_file_recipe_ends_with_files() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .command()
        .args(["dry-run", "compress-images"])
        .output()
        .unwrap();
    assert_eq!(
        footer(&output),
        "Run with: bnto run compress-images <files...>"
    );
}

#[test]
fn a_url_recipe_ends_with_url() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .command()
        .args(["dry-run", "download-video"])
        .output()
        .unwrap();
    assert_eq!(footer(&output), "Run with: bnto run download-video <url>");
}
