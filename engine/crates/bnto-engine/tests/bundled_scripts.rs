// ShellCheck over every bundled recipe's script-mode steps. Bundled scripts
// ship to every user and are shown whole at consent, so they're held to
// ShellCheck's default rules (shell-command.md, "safe-script contract").
// Without ShellCheck the checks skip, unless BNTO_REQUIRE_SHELLCHECK=1 (set in
// CI), where a missing ShellCheck fails so the gate can't pass by not running.

use std::process::Command;

use bnto_engine::recipes::builtin_recipes;
use serde_json::Value;

/// Shells ShellCheck can read. Script mode also accepts zsh, which ShellCheck
/// can't check, so a bundled script may not use it.
const CHECKABLE_SHELLS: &[&str] = &["sh", "bash", "dash", "ksh"];

/// A script-mode step in a bundled recipe.
struct ScriptStep {
    at: String,
    shell: String,
    script: String,
}

enum Outcome {
    Clean,
    Findings(String),
    Missing,
}

/// Every script-mode step in the bundled recipes, container children included.
fn bundled_script_steps() -> Vec<ScriptStep> {
    let mut steps = Vec::new();
    for recipe in builtin_recipes() {
        let def: Value = serde_json::from_str(recipe.definition_json).unwrap();
        collect_steps(&recipe.slug, &def["nodes"], &mut steps);
    }
    steps
}

fn collect_steps(recipe: &str, nodes: &Value, out: &mut Vec<ScriptStep>) {
    for node in nodes.as_array().into_iter().flatten() {
        let params = &node["parameters"];
        if let Some(script) = params["script"].as_str() {
            out.push(ScriptStep {
                at: format!("{recipe}/{}", node["id"].as_str().unwrap_or("?")),
                shell: params["shell"].as_str().unwrap_or("sh").to_string(),
                script: script.to_string(),
            });
        }
        collect_steps(recipe, &node["nodes"], out);
    }
}

/// Run `program` (ShellCheck) over `script` as written for `shell`.
fn shellcheck(program: &str, shell: &str, script: &str) -> Outcome {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("script.sh");
    std::fs::write(&path, script).unwrap();
    let output = Command::new(program)
        .args(["--shell", shell, "--format", "gcc"])
        .arg(&path)
        .output();
    match output {
        Err(_) => Outcome::Missing,
        Ok(out) if out.status.success() => Outcome::Clean,
        Ok(out) => Outcome::Findings(format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )),
    }
}

fn shellcheck_required() -> bool {
    std::env::var("BNTO_REQUIRE_SHELLCHECK").is_ok_and(|v| v == "1")
}

/// Skip quietly, or fail when CI requires ShellCheck.
fn on_missing(required: bool) {
    assert!(
        !required,
        "ShellCheck isn't installed, and BNTO_REQUIRE_SHELLCHECK=1 requires it"
    );
    eprintln!("skipping: ShellCheck isn't installed");
}

#[test]
fn every_bundled_script_passes_shellcheck() {
    let steps = bundled_script_steps();
    assert!(
        steps.iter().any(|s| s.at.starts_with("keep-awake/")),
        "no script steps found; the walk is broken"
    );
    let mut failures = Vec::new();
    for step in &steps {
        match shellcheck("shellcheck", &step.shell, &step.script) {
            Outcome::Clean => {}
            Outcome::Findings(text) => failures.push(format!("{}:\n{text}", step.at)),
            Outcome::Missing => return on_missing(shellcheck_required()),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn an_unquoted_value_is_a_finding() {
    // Control: the gate can fail, so a pass above means something.
    match shellcheck("shellcheck", "sh", "echo $1\n") {
        Outcome::Findings(text) => assert!(text.contains("SC2086"), "{text}"),
        Outcome::Clean => panic!("ShellCheck accepted an unquoted $1"),
        Outcome::Missing => on_missing(shellcheck_required()),
    }
}

#[test]
fn bundled_scripts_use_a_shell_shellcheck_reads() {
    for step in bundled_script_steps() {
        assert!(
            CHECKABLE_SHELLS.contains(&step.shell.as_str()),
            "{} uses {}, which ShellCheck can't check",
            step.at,
            step.shell
        );
    }
}

#[test]
fn a_missing_shellcheck_is_seen_as_missing() {
    let outcome = shellcheck("bnto-no-such-shellcheck", "sh", "true\n");
    assert!(matches!(outcome, Outcome::Missing));
}

#[test]
#[should_panic(expected = "BNTO_REQUIRE_SHELLCHECK=1 requires it")]
fn a_missing_shellcheck_fails_when_required() {
    on_missing(true);
}
