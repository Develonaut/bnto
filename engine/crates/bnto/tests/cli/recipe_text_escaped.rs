// ISS-125: recipe text reaches the terminal escaped. A library recipe whose
// name, description and script hold ESC and `\r` can't erase or overwrite a
// line of the consent prompt, `bnto list`, `bnto info` or `bnto doctor`: each
// shows them as `\e` and `\r`.

use crate::helpers;

use std::io::Write;
use std::process::{Output, Stdio};

use helpers::Sandbox;

/// The repro: ESC `[2K` erases the line, `\r` returns to its start.
const RECIPE: &str = r#"{
    "id": "esc", "type": "group", "version": "1.0.0",
    "name": "Nice\u001b[2K\rEVIL",
    "metadata": { "description": "Fine\u001b[1A\rsneaky", "category": "evil\u001b[31m" },
    "nodes": [
        { "id": "input", "type": "input", "version": "1.0.0", "parameters": { "mode": "none" } },
        { "id": "s", "type": "shell-run", "version": "1.0.0",
          "parameters": { "shell": "sh", "script": "echo ok\u001b[1A\rrm -rf x" } }
    ]
}"#;

fn sandbox_with_recipe() -> Sandbox {
    let sandbox = Sandbox::new();
    let library = sandbox.bnto_home().join("recipes");
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(library.join("esc.bnto.json"), RECIPE).unwrap();
    sandbox
}

/// `bnto <args>` with colors off, so any ESC in its output is the recipe's.
fn bnto(sandbox: &Sandbox, args: &[&str], stdin: &str) -> Output {
    let mut child = sandbox
        .command()
        .env("NO_COLOR", "1")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn assert_no_control_chars(text: &str, what: &str) {
    assert!(
        !text.contains('\u{1b}') && !text.contains('\r'),
        "{what} holds a raw control character: {text:?}"
    );
}

#[test]
fn the_consent_prompt_shows_the_name_and_script_escaped() {
    let sandbox = sandbox_with_recipe();
    let output = bnto(&sandbox, &["run", "esc"], "n\n");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "declined: {stderr}");
    assert_no_control_chars(&stderr, "the prompt");
    assert!(stderr.contains("Recipe: Nice\\e[2K\\rEVIL"), "{stderr}");
    assert!(stderr.contains("echo ok\\e[1A\\rrm -rf x"), "{stderr}");
    assert!(stderr.contains("Library recipe `esc` from `"), "{stderr}");
}

#[test]
fn info_shows_recipe_text_escaped() {
    let sandbox = sandbox_with_recipe();
    let output = bnto(&sandbox, &["info", "esc"], "");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout}");
    assert_no_control_chars(&stdout, "bnto info");
    assert!(stdout.contains("Nice\\e[2K\\rEVIL"), "{stdout}");
    assert!(stdout.contains("Fine\\e[1A\\rsneaky"), "{stdout}");
}

#[test]
fn list_shows_recipe_text_escaped() {
    let sandbox = sandbox_with_recipe();
    let output = bnto(&sandbox, &["list"], "");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout}");
    assert_no_control_chars(&stdout, "bnto list");
    assert!(stdout.contains("Fine\\e[1A\\rsneaky"), "{stdout}");
    assert!(stdout.contains("evil\\e[31m"), "{stdout}");
}

#[test]
fn doctor_shows_recipe_text_escaped() {
    // A recipe that fails to parse is named by its slug, which comes from a
    // file name; a secret's description is the author's.
    let sandbox = sandbox_with_recipe();
    let library = sandbox.bnto_home().join("recipes");
    let secret = r#"{ "name": "S", "secrets": [{ "key": "BNTO_ISS125_KEY",
        "description": "key\u001b[2K\rgone", "required": true }], "nodes": [] }"#;
    std::fs::write(library.join("secret.bnto.json"), secret).unwrap();
    let output = bnto(&sandbox, &["doctor"], "");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_no_control_chars(&stdout, "bnto doctor");
    assert!(stdout.contains("key\\e[2K\\rgone"), "{stdout}");
}

/// `json` saved as the library recipe `slug`.
fn add_recipe(sandbox: &Sandbox, slug: &str, json: &str) {
    let library = sandbox.bnto_home().join("recipes");
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(library.join(format!("{slug}.bnto.json")), json).unwrap();
}

/// stdout and stderr together, for a check that neither holds raw control.
fn both(output: &Output) -> String {
    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);
    format!("{out}{err}")
}

#[test]
fn dry_run_shows_recipe_text_escaped() {
    let sandbox = Sandbox::new();
    add_recipe(
        &sandbox,
        "esc3",
        r#"{ "name": "n\u001b[2K", "nodes": [{ "id": "s\u001b[2K", "type": "shell-run",
            "parameters": { "shell": "sh", "script": "echo safe\u001b[2K\rrm -rf ~ #",
            "args": ["a\u001b]52;c;cm0K\u0007"], "env": { "MODE": "x\u001b[2K" } } }] }"#,
    );
    let output = bnto(&sandbox, &["dry-run", "esc3"], "");
    let text = both(&output);
    assert!(output.status.success(), "{text}");
    assert_no_control_chars(&text, "bnto dry-run");
    assert!(text.contains("n\\e[2K"), "{text}");
    assert!(text.contains("s\\e[2K"), "{text}");
    assert!(text.contains("echo safe\\e[2K\\rrm -rf ~ #"), "{text}");
    assert!(text.contains("MODE=x\\e[2K"), "{text}");
    assert!(!text.contains('\u{7}'), "{text}");
}

#[test]
fn check_shows_recipe_text_escaped() {
    let sandbox = Sandbox::new();
    let file = sandbox.work_dir().join("esc2.bnto.json");
    let json = r#"{ "name": "n", "nodes": [{ "id": "a", "type": "bogus\u001b[2K\rOK", "parameters": {} }] }"#;
    std::fs::write(&file, json).unwrap();
    let output = bnto(&sandbox, &["check", file.to_str().unwrap()], "");
    let text = both(&output);
    assert_no_control_chars(&text, "bnto check");
    assert!(text.contains("bogus\\e[2K\\rOK"), "{text}");
}

#[test]
fn installs_plan_shows_the_recipes_command_escaped() {
    let sandbox = Sandbox::new();
    let hint = "echo hi\\u001b[2K\\rgone";
    let json = format!(
        r#"{{ "name": "Needs", "requires": [{{ "binary": "bnto-iss125-missing",
            "installHints": {{ "macos": "{hint}", "linux": "{hint}", "windows": "{hint}" }} }}],
            "nodes": [] }}"#
    );
    add_recipe(&sandbox, "needs", &json);
    let output = bnto(&sandbox, &["install", "needs"], "n\n");
    let text = both(&output);
    assert_no_control_chars(&text, "bnto install");
    assert!(text.contains("echo hi\\e[2K\\rgone"), "{text}");
}

#[test]
fn a_report_steps_message_shows_escaped() {
    let sandbox = Sandbox::new();
    add_recipe(
        &sandbox,
        "say",
        r#"{ "name": "Say", "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "r", "type": "report", "parameters": { "message": "MSG\u001b[2K\rFAKE" } }
        ] }"#,
    );
    let output = bnto(&sandbox, &["run", "say", "--yes"], "");
    let text = both(&output);
    assert_no_control_chars(&text, "a report step's run");
    assert!(text.contains("MSG\\e[2K\\rFAKE"), "{text}");
}

#[test]
fn a_failing_steps_error_shows_escaped() {
    let sandbox = Sandbox::new();
    add_recipe(
        &sandbox,
        "fails",
        r#"{ "name": "Fails", "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "s\u001b[2K", "type": "shell-run",
              "parameters": { "command": "bnto-iss125-missing\u001b[2K" } }
        ] }"#,
    );
    let output = bnto(&sandbox, &["run", "fails", "--yes"], "");
    let text = both(&output);
    assert!(!output.status.success(), "{text}");
    assert_no_control_chars(&text, "a failing step's run");
    assert!(text.contains("bnto-iss125-missing\\e[2K"), "{text}");
}

#[test]
fn a_refused_params_error_shows_escaped() {
    // Refused before the run starts, by the top-level error line.
    let sandbox = Sandbox::new();
    add_recipe(
        &sandbox,
        "refused",
        r#"{ "name": "Refused", "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "x", "type": "spreadsheet-export",
              "parameters": { "path": "nofolder\u001b[31m/x.csv" } }
        ] }"#,
    );
    let output = bnto(&sandbox, &["run", "refused", "--yes"], "");
    let text = both(&output);
    assert!(!output.status.success(), "{text}");
    assert_no_control_chars(&text, "a refused run");
    assert!(text.contains("nofolder\\e[31m/x.csv"), "{text}");
}

/// The `fdef` repro: a field's default is what runs, so the prompt shows it.
const FIELD_DEFAULT: &str = r#"{ "name": "Fdef", "nodes": [
    { "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "s", "type": "shell-run",
      "fields": { "fmt": { "type": "string", "label": "Format",
                           "default": "--exec=touch HIDDEN\u001b[2K" } },
      "parameters": { "command": "echo", "args": ["{{fields.fmt}}"] } }
] }"#;

#[test]
fn the_prompt_shows_a_fields_default_escaped() {
    let sandbox = Sandbox::new();
    add_recipe(&sandbox, "fdef", FIELD_DEFAULT);
    let output = bnto(&sandbox, &["run", "fdef"], "n\n");
    let text = both(&output);
    assert_no_control_chars(&text, "the prompt");
    assert!(text.contains("- echo {{fields.fmt}}"), "{text}");
    assert!(
        text.contains("runs: echo --exec=touch HIDDEN\\e[2K"),
        "{text}"
    );
}

#[test]
fn the_prompt_shows_a_value_given_with_p() {
    let sandbox = Sandbox::new();
    add_recipe(&sandbox, "fdef", FIELD_DEFAULT);
    let output = bnto(&sandbox, &["run", "fdef", "-p", "fmt=--version"], "n\n");
    let text = both(&output);
    assert!(text.contains("runs: echo --version"), "{text}");
    assert!(!text.contains("HIDDEN"), "{text}");
}
