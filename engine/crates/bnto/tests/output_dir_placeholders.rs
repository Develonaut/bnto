// The output node's `directory` fills every placeholder a run has before the
// run starts, and refuses one left over before consent and before any folder
// is made; `bnto check` flags the ones no run can fill (ISS-22), and an
// unclosed `{{` (ISS-79).

mod helpers;

use std::path::Path;
use std::process::Output;

use helpers::Sandbox;

/// The env var the recipes below name their output folder by.
const KEY: &str = "BNTO_ISS22_OUT";

/// A file recipe that renames `note.txt` and writes it to `directory`.
fn rename_recipe(directory: &str) -> String {
    serde_json::json!({
        "id": "env-dir", "name": "Env Dir",
        "secrets": [{ "key": KEY }],
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
            { "id": "rename", "type": "file-rename", "parameters": { "prefix": "renamed-" } },
            { "id": "output", "type": "output",
              "parameters": { "mode": "write", "directory": directory } }
        ]
    })
    .to_string()
}

/// A shell recipe, which needs consent, writing to `directory`.
fn shell_recipe(directory: &str) -> String {
    serde_json::json!({
        "id": "shell-dir", "name": "Shell Dir",
        "secrets": [{ "key": KEY }],
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "say", "type": "shell-run",
              "parameters": { "command": "echo", "args": ["hi"] } },
            { "id": "output", "type": "output",
              "parameters": { "mode": "write", "directory": directory } }
        ]
    })
    .to_string()
}

/// `recipe` saved in the sandbox's work dir, beside a `note.txt` input.
fn save(sandbox: &Sandbox, recipe: &str) -> std::path::PathBuf {
    std::fs::write(sandbox.work_dir().join("note.txt"), b"hello").unwrap();
    let path = sandbox.work_dir().join("dir.bnto.json");
    std::fs::write(&path, recipe).unwrap();
    path
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Every entry of the work dir but the recipe and its input.
fn made_in(work: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(work)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n != "note.txt" && n != "dir.bnto.json")
        .collect();
    names.sort();
    names
}

#[test]
fn run_writes_into_the_folder_a_declared_secret_names() {
    let sandbox = Sandbox::new();
    let target = tempfile::tempdir().unwrap();
    let folder = target.path().join("index");
    let recipe = save(&sandbox, &rename_recipe(&format!("{{{{env.{KEY}}}}}")));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .env(KEY, &folder)
        .output()
        .unwrap();
    let err = stderr(&output);
    assert!(output.status.success(), "{err}");
    assert_eq!(
        std::fs::read(folder.join("renamed-note.txt")).unwrap(),
        b"hello"
    );
    assert!(
        made_in(sandbox.work_dir()).is_empty(),
        "made a literal folder"
    );
    assert!(!err.contains("{{env."), "{err}");
}

#[test]
fn run_refuses_an_unset_secret_before_consent_and_makes_no_folder() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &shell_recipe(&format!("{{{{env.{KEY}}}}}/out")));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .env_remove(KEY)
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(
        err.contains(&format!("{{{{env.{KEY}}}}} has no value: {KEY} is not set")),
        "{err}"
    );
    assert!(
        !err.contains("Allow execution?"),
        "asked for consent first: {err}"
    );
    assert!(made_in(sandbox.work_dir()).is_empty(), "made a folder");
}

#[test]
fn run_refuses_an_undeclared_env_value() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("{{env.BNTO_ISS22_OTHER}}"));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .env("BNTO_ISS22_OTHER", "elsewhere")
        .env(KEY, "x")
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("{{env.BNTO_ISS22_OTHER}}"), "{err}");
    assert!(err.contains("secrets"), "{err}");
    assert!(made_in(sandbox.work_dir()).is_empty(), "made a folder");
}

#[test]
fn run_refuses_placeholders_nothing_fills_instead_of_dropping_them() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("out-{{ctx.nope}}-{{item.x}}"));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .env(KEY, "x")
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("{{ctx.nope}}"), "{err}");
    assert!(err.contains("{{item.x}}"), "{err}");
    assert!(made_in(sandbox.work_dir()).is_empty(), "made a folder");
}

#[test]
fn run_refuses_an_unclosed_placeholder_and_makes_no_folder() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("out-{{ctx.date"));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .env(KEY, "x")
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("unclosed placeholder \"{{ctx.date\""), "{err}");
    assert!(made_in(sandbox.work_dir()).is_empty(), "made a folder");
}

#[test]
fn a_dry_run_refuses_a_folder_nothing_fills_too() {
    // `--dry-run` writes nothing, but would name the folder it writes to.
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("out-{{ctx.nope}}"));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .arg("--dry-run")
        .env(KEY, "x")
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains("nothing fills {{ctx.nope}}"), "{err}");
    assert!(made_in(sandbox.work_dir()).is_empty(), "made a folder");
}

#[test]
fn an_output_flag_wins_over_a_folder_nothing_fills() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("out-{{ctx.nope}}"));
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .args(["-o", "chosen"])
        .env(KEY, "x")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", stderr(&output));
    let written = sandbox.work_dir().join("chosen/renamed-note.txt");
    assert_eq!(std::fs::read(written).unwrap(), b"hello");
    assert_eq!(made_in(sandbox.work_dir()), ["chosen"]);
}

#[test]
fn a_p_value_for_an_output_field_names_the_folder() {
    let sandbox = Sandbox::new();
    let mut recipe: serde_json::Value =
        serde_json::from_str(&rename_recipe("out/{{fields.sub}}")).unwrap();
    recipe["nodes"][2]["fields"] =
        serde_json::json!({ "sub": { "type": "string", "label": "Sub", "default": "draft" } });
    let recipe = save(&sandbox, &recipe.to_string());
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg("note.txt")
        .args(["-p", "output:sub=final"])
        .env(KEY, "x")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", stderr(&output));
    let written = sandbox.work_dir().join("out/final/renamed-note.txt");
    assert_eq!(std::fs::read(written).unwrap(), b"hello");
    assert_eq!(made_in(sandbox.work_dir()), ["out"]);
    assert!(!sandbox.work_dir().join("out/draft").exists());
}

#[test]
fn check_flags_output_placeholders_no_run_can_fill() {
    let sandbox = Sandbox::new();
    let recipe = save(
        &sandbox,
        &rename_recipe("{{env.UNDECLARED}}/{{item.x}}/{{node.rename.count}}"),
    );
    let output = sandbox
        .command()
        .arg("check")
        .arg(&recipe)
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(output.status.code(), Some(1), "{out}");
    for placeholder in ["{{env.UNDECLARED}}", "{{item.x}}", "{{node.rename.count}}"] {
        assert!(
            out.lines()
                .any(|l| l.contains("param \"directory\"") && l.contains(placeholder)),
            "{placeholder} not flagged: {out}"
        );
    }
}

#[test]
fn check_passes_output_placeholders_a_run_fills() {
    let sandbox = Sandbox::new();
    let directory = format!("{{{{env.{KEY}}}}}/{{{{ctx.date}}}}-{{{{node.input.filename_stem}}}}");
    let recipe = save(&sandbox, &rename_recipe(&directory));
    let output = sandbox
        .command()
        .arg("check")
        .arg(&recipe)
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(output.status.success(), "{out}");
}

#[test]
fn check_flags_an_unclosed_output_placeholder() {
    let sandbox = Sandbox::new();
    let recipe = save(&sandbox, &rename_recipe("out-{{ctx.date"));
    let output = sandbox
        .command()
        .arg("check")
        .arg(&recipe)
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(output.status.code(), Some(1), "{out}");
    assert!(
        out.lines().any(|l| l.contains("param \"directory\"")
            && l.contains("unclosed placeholder \"{{ctx.date\"")),
        "{out}"
    );
}
