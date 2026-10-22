// SEC-20..26: acceptance spec for the planned `script` mode — a literal
// script run as `<shell> -c <script> <name> <args...>`, values only as
// "$1".."$N" (POSIX sh -c operands, ShellCheck SC2156). Every test is open
// until the mode ships; together they are its definition of done.
//
// Shape under test: params `{ "shell": "sh", "script": "...", "args": [...] }`.
#![cfg(unix)]

use std::path::Path;

use serde_json::{Value, json};

use crate::support::{Lab, payloads, step};

fn script_step(shell: &str, script: &str, args: Value, fields: &[&str]) -> Value {
    step(
        json!({ "shell": shell, "script": script, "args": args }),
        fields,
    )
}

fn write_executable(path: &Path, body: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

/// SEC-20: hostile values arrive as $1 and stay data; `$0` is not a value.
#[test]
fn sec20_values_arrive_as_positional_data() {
    let lab = Lab::new();
    let recipe = lab.recipe(script_step(
        "sh",
        "printf '[%s|%s]' \"$1\" \"$2\"",
        json!(["{{fields.v}}", "second"]),
        &["v"],
    ));
    for payload in payloads(&lab.canary()) {
        let out = lab.run(&recipe, &[&format!("v={payload}")]);
        assert!(out.ok, "{payload:?}: {}", out.text);
        assert!(!lab.canary_fired(), "{payload:?} executed");
    }
    let out = lab.run(&recipe, &["v=$(id)"]);
    assert!(out.text.contains("[$(id)|second]"), "{}", out.text);
}

/// SEC-21: a template in the script is refused on the raw definition, by
/// `check` and `run` — a check after resolution would see the data.
#[test]
fn sec21_templates_in_the_script_are_refused() {
    let lab = Lab::new();
    // Control: the same value passed the safe way runs, so the refusal
    // below is about the template, not script mode being missing.
    let safe = lab.recipe(script_step(
        "sh",
        "echo \"$1\"",
        json!(["{{fields.v}}"]),
        &["v"],
    ));
    assert!(
        lab.run(&safe, &["v=hi"]).text.contains("hi"),
        "safe script didn't run"
    );

    let recipe = lab.recipe(script_step("sh", "echo {{fields.v}}", json!([]), &["v"]));
    assert!(!lab.check(&recipe).ok, "check accepted a templated script");
    let payload = format!("v=$(touch {})", lab.canary().display());
    let out = lab.run(&recipe, &[&payload]);
    assert!(!out.ok, "{}", out.text);
    assert!(!lab.canary_fired(), "templated script executed");
}

/// SEC-22: startup hooks in bnto's own environment never run: SHELLOPTS +
/// PS4 and BASH_FUNC_* fire under bash and macOS /bin/sh, BASH_ENV under bash.
#[test]
fn sec22_inherited_shell_startup_hooks_are_ignored() {
    let lab = Lab::new();
    let hook = lab.path().join("hook.sh");
    std::fs::write(&hook, "touch canary\n").unwrap();
    let hostile = [
        ("SHELLOPTS", "xtrace"),
        ("PS4", "$(touch canary)"),
        ("BASH_ENV", hook.to_str().unwrap()),
        ("BASH_FUNC_printf%%", "() {  touch canary\n}"),
    ];
    for shell in ["sh", "bash"] {
        let recipe = lab.recipe(script_step(shell, "printf ok", json!([]), &[]));
        let out = lab.run_with(&recipe, &[], &[], &hostile);
        assert!(out.ok, "{shell}: {}", out.text);
        assert!(!lab.canary_fired(), "{shell} ran a startup hook");
    }
}

/// SEC-23: zsh reads `$ZDOTDIR/.zshenv` even for `-c`; script mode must not.
#[test]
fn sec23_zsh_startup_files_are_ignored() {
    if !Path::new("/bin/zsh").exists() {
        return;
    }
    let lab = Lab::new();
    let zdot = lab.path().join("zdot");
    std::fs::create_dir_all(&zdot).unwrap();
    std::fs::write(zdot.join(".zshenv"), "touch canary\n").unwrap();
    let recipe = lab.recipe(script_step("zsh", "print ok", json!([]), &[]));
    let out = lab.run_with(&recipe, &[], &[], &[("ZDOTDIR", zdot.to_str().unwrap())]);
    assert!(out.ok, "{}", out.text);
    assert!(!lab.canary_fired(), "zsh sourced $ZDOTDIR/.zshenv");
}

/// SEC-24: the shell is a fixed absolute path, never a PATH or
/// `searchPath` lookup that a planted `sh` could win.
#[test]
fn sec24_shell_is_not_looked_up_on_path() {
    let lab = Lab::new();
    let fake_bin = lab.path().join("fakebin");
    for name in ["sh", "bash"] {
        write_executable(&fake_bin.join(name), "#!/bin/sh\ntouch canary\n");
    }
    let path = format!("{}:{}", fake_bin.display(), std::env::var("PATH").unwrap());
    let mut planted = script_step("sh", "printf ok", json!([]), &[]);
    let recipe = lab.recipe(planted.clone());
    let out = lab.run_with(&recipe, &[], &[], &[("PATH", &path)]);
    assert!(
        out.ok && out.text.contains("ok"),
        "real sh didn't run: {}",
        out.text
    );
    assert!(
        !lab.canary_fired(),
        "ran the planted sh from PATH: {}",
        out.text
    );

    planted["parameters"]["searchPath"] = json!([fake_bin.to_str().unwrap()]);
    let out = lab.run(&lab.recipe(planted), &[]);
    assert!(
        !lab.canary_fired(),
        "ran the planted sh from searchPath: {}",
        out.text
    );
    assert!(!out.ok || out.text.contains("ok"), "{}", out.text);
}

/// SEC-25: only the named POSIX shells are accepted for `shell`.
#[test]
fn sec25_only_known_shells_are_accepted() {
    let lab = Lab::new();
    for shell in ["sh", "bash", "zsh", "dash", "ksh"] {
        let out = lab.check(&lab.recipe(script_step(shell, "true", json!([]), &[])));
        assert!(out.ok, "{shell} refused: {}", out.text);
    }
    for shell in [
        "python3",
        "fish",
        "cmd",
        "PowerShell.exe",
        "/bin/sh",
        "bash -c",
        "{{fields.s}}",
    ] {
        let out = lab.check(&lab.recipe(script_step(shell, "true", json!([]), &["s"])));
        assert!(!out.ok, "{shell} accepted");
    }
}

/// SEC-26: script mode starts from a cleared environment plus an allowlist,
/// so the user's own exported secrets don't reach a script that didn't ask.
#[test]
fn sec26_script_mode_does_not_inherit_the_full_environment() {
    let lab = Lab::new();
    let script = "printf '[%s]' \"${BNTO_PROBE_SECRET-unset}\"; command -v printf >/dev/null && printf ' path-ok'";
    let recipe = lab.recipe(script_step("sh", script, json!([]), &[]));
    let out = lab.run_with(&recipe, &[], &[], &[("BNTO_PROBE_SECRET", "leaked")]);
    assert!(out.ok, "{}", out.text);
    assert!(
        out.text.contains("[unset]"),
        "secret reached the script: {}",
        out.text
    );
    assert!(
        out.text.contains("path-ok"),
        "PATH should stay available: {}",
        out.text
    );
}
