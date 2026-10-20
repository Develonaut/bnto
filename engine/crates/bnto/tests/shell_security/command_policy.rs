// SEC-04..08: which programs a shell-command step may name.
// Static checks through `bnto check` and `bnto run`; nothing here needs the
// program to exist, so these run on every OS.

use serde_json::json;

use crate::support::{Lab, step};

fn checks_ok(command: &str) -> bool {
    let lab = Lab::new();
    lab.check(&lab.recipe(step(json!({ "command": command, "args": [] }), &[])))
        .ok
}

/// Every command in `commands` that `bnto check` wrongly accepts.
fn accepted(commands: &[&str]) -> Vec<String> {
    // Control: a plain tool passes, so a refusal means the name was refused.
    assert!(checks_ok("git"), "check refuses even a plain tool");
    commands
        .iter()
        .filter(|c| checks_ok(c))
        .map(|c| c.to_string())
        .collect()
}

/// Harness self-test: the canary fires when a command really runs, so a
/// passing "canary never fired" assertion means something.
#[cfg(unix)]
#[test]
fn harness_canary_fires_when_a_command_runs() {
    let lab = Lab::new();
    let canary = lab.canary().display().to_string();
    let recipe = lab.recipe(step(json!({ "command": "touch", "args": [canary] }), &[]));
    assert!(lab.run(&recipe, &[]).ok);
    assert!(lab.canary_fired());
}

/// SEC-04: shells can't be the command, in any letter case.
#[test]
fn sec04_shells_are_refused_as_commands() {
    let shells = [
        "sh",
        "bash",
        "zsh",
        "fish",
        "dash",
        "csh",
        "tcsh",
        "ksh",
        "cmd",
        "cmd.exe",
        "powershell",
        "pwsh",
        "SH",
        "Bash",
        "PowerShell",
        "CMD.EXE",
    ];
    assert_eq!(accepted(&shells), Vec::<String>::new());
}

/// SEC-05: Windows resolves `bash.exe` / `pwsh.exe` to the same shells the
/// denylist names, so the `.exe` spellings must be refused too.
#[test]
fn sec05_windows_exe_spellings_of_shells_are_refused() {
    let shells = [
        "sh.exe",
        "bash.exe",
        "zsh.exe",
        "fish.exe",
        "dash.exe",
        "ksh.exe",
        "powershell.exe",
        "pwsh.exe",
        "PowerShell.EXE",
        "Bash.Exe",
    ];
    assert_eq!(accepted(&shells), Vec::<String>::new());
}

/// SEC-06: commands are bare names resolved on PATH, never paths.
#[test]
fn sec06_command_paths_are_refused() {
    let paths = [
        "/bin/sh",
        "/usr/bin/env",
        "./tool",
        "../tool",
        "bin/tool",
        "C:\\Windows\\System32\\cmd.exe",
        "..\\tool",
        ".hidden",
    ];
    assert_eq!(accepted(&paths), Vec::<String>::new());
}

/// SEC-07: the program name must be written literally (Home Assistant's
/// rule). A templated command lets data pick which program runs.
#[test]
fn sec07_command_must_be_literal() {
    let lab = Lab::new();
    let recipe = lab.recipe(step(
        json!({ "command": "{{fields.tool}}", "args": [] }),
        &["tool"],
    ));
    let check = lab.check(&recipe);
    assert!(
        !check.ok,
        "check accepted a templated command: {}",
        check.text
    );
    let run = lab.run(&recipe, &["tool=git"]);
    assert!(!run.ok, "run accepted a templated command: {}", run.text);
    assert!(run.text.contains("literal"), "{}", run.text);
}

/// SEC-08: wrappers that exec their first argument reach a shell around
/// the denylist (GTFOBins). Later hardening — the consent hash is the real
/// boundary for authors; this keeps the denylist honest about its job.
#[test]
#[ignore = "OPEN SEC-08 (later): `env sh -c ...` and other wrappers bypass the denylist"]
fn sec08_exec_wrappers_cannot_launch_shells() {
    let lab = Lab::new();
    let wrapped = [
        json!({ "command": "env", "args": ["sh", "-c", "true"] }),
        json!({ "command": "xargs", "args": ["sh", "-c", "true"] }),
        json!({ "command": "nohup", "args": ["bash", "-c", "true"] }),
        json!({ "command": "nice", "args": ["sh", "-c", "true"] }),
        json!({ "command": "timeout", "args": ["5", "sh", "-c", "true"] }),
    ];
    for params in wrapped {
        let out = lab.check(&lab.recipe(step(params.clone(), &[])));
        assert!(!out.ok, "check accepted {params}: {}", out.text);
    }
}
