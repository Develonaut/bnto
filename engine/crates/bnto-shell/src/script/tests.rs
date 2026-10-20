use std::path::{Path, PathBuf};

use serde_json::json;

use super::*;

fn params(value: serde_json::Value) -> Map<String, Value> {
    value.as_object().unwrap().clone()
}

#[test]
fn a_script_step_with_a_known_shell_is_valid() {
    for shell in ["sh", "bash", "zsh", "dash", "ksh"] {
        let p = params(json!({ "script": "echo \"$1\"", "shell": shell }));
        assert!(script_problems(&p).is_empty(), "{shell}");
    }
    assert!(script_problems(&params(json!({ "script": "true" }))).is_empty());
}

#[test]
fn unknown_shells_are_refused() {
    for shell in ["python3", "fish", "cmd", "powershell", "/bin/sh", "bash -c"] {
        let found = script_problems(&params(json!({ "script": "true", "shell": shell })));
        assert_eq!(found.len(), 1, "{shell}");
        assert!(found[0].contains("'shell' must be one of"), "{}", found[0]);
    }
}

#[test]
fn command_and_script_together_are_refused() {
    let found = script_problems(&params(json!({ "script": "true", "command": "git" })));
    assert!(found[0].contains("not both"), "{found:?}");
}

#[test]
fn an_empty_script_and_a_search_path_are_refused() {
    assert_eq!(script_problems(&params(json!({ "script": "  " }))).len(), 1);
    let found = script_problems(&params(json!({ "script": "x", "search_path": ["bin"] })));
    assert!(found[0].contains("search_path"), "{found:?}");
}

#[test]
fn templates_in_the_script_or_shell_are_refused_before_filling() {
    for (key, text) in [
        ("script", "echo {{fields.v}}"),
        ("script", "cat {{path}}"),
        ("script", "echo {{vars.Name}}"),
        ("shell", "{{fields.s}}"),
    ] {
        let found = raw_script_problems(&params(json!({ key: text })));
        assert_eq!(found.len(), 1, "{key}={text}");
        assert!(found[0].contains("\"$1\""), "{}", found[0]);
    }
}

#[test]
fn literal_braces_in_a_script_are_fine() {
    let p = params(json!({ "script": "awk '{ print $1 }' | jq '{a: 1}'; echo \"${HOME}\"" }));
    assert!(raw_script_problems(&p).is_empty());
}

#[test]
fn shells_resolve_to_a_fixed_absolute_path() {
    let only = |wanted: &'static str| move |p: &Path| p == Path::new(wanted);
    assert_eq!(
        shell_path("sh", only("/bin/sh")),
        Ok(PathBuf::from("/bin/sh"))
    );
    assert_eq!(
        shell_path("zsh", only("/usr/bin/zsh")),
        Ok(PathBuf::from("/usr/bin/zsh"))
    );
    assert!(
        shell_path("bash", |_| false)
            .unwrap_err()
            .contains("isn't installed")
    );
    assert!(shell_path("fish", |_| true).is_err());
}

#[cfg(unix)]
#[test]
fn the_invocation_turns_startup_files_off_and_fixes_dollar_zero() {
    let (path, args) = invocation("sh", "echo \"$1\"").unwrap();
    assert_eq!(path, PathBuf::from("/bin/sh"));
    assert_eq!(args, ["-c", "echo \"$1\"", "bnto"]);
    if Path::new("/bin/bash").exists() {
        let (_, args) = invocation("bash", "true").unwrap();
        assert_eq!(args, ["-p", "-c", "true", "bnto"]);
    }
    if Path::new("/bin/zsh").exists() {
        let (_, args) = invocation("zsh", "true").unwrap();
        assert_eq!(args[0], "-f");
    }
}
