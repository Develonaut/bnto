// `bnto run` and `bnto dry-run` refuse a `{{fields.*}}` with nothing to fill
// it before the consent prompt, naming the `-p` that would (ISS-1).

mod helpers;

use helpers::Sandbox;

/// A group `checks` declaring `version` with no default, around a shell
/// step that uses it (release-verify's shape). A shell step needs consent.
const UNSET_VERSION: &str = r#"{ "id": "unset-version", "name": "Unset Version",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "checks", "type": "group",
    "fields": { "version": { "type": "string", "label": "Version" } },
    "nodes": [{ "id": "crates", "type": "shell-run",
      "parameters": { "command": "echo", "args": ["{{fields.version}}"] } }] }] }"#;

const HINT: &str = "field \"version\" (node \"checks\") is unset: pass -p version=...";

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn run_refuses_an_unset_field_before_the_consent_prompt() {
    let sandbox = Sandbox::new();
    let recipe = sandbox
        .bnto_home()
        .with_file_name("unset-version.bnto.json");
    std::fs::write(&recipe, UNSET_VERSION).unwrap();
    let output = sandbox.command().arg("run").arg(&recipe).output().unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains(HINT), "{err}");
    assert!(
        !err.contains("Allow execution?"),
        "asked for consent first: {err}"
    );
}

#[test]
fn run_with_the_field_given_gets_past_the_check() {
    let sandbox = Sandbox::new();
    let recipe = sandbox
        .bnto_home()
        .with_file_name("unset-version.bnto.json");
    std::fs::write(&recipe, UNSET_VERSION).unwrap();
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .args(["--yes", "-p", "version=1.2.3"])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert!(output.status.success(), "{err}");
    assert!(!err.contains("is unset"), "{err}");
}

#[test]
fn dry_run_refuses_an_unset_field_in_a_library_recipe() {
    let sandbox = Sandbox::new();
    let library = sandbox.bnto_home().join("recipes");
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(library.join("unset-version.bnto.json"), UNSET_VERSION).unwrap();
    let output = sandbox
        .command()
        .args(["dry-run", "unset-version"])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains(HINT), "{err}");
}
