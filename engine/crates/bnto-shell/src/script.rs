// Script mode: a literal script run as `<shell> -c <script> bnto <args...>`.
// Values reach the script only as "$1".."$N" (POSIX `sh -c` operands,
// ShellCheck SC2156), never as script text. The shell is a fixed absolute
// path, started with startup files off (`bash -p`, `zsh -f`) in a cleared
// environment, so nothing but the script runs (SEC-20..26). PowerShell
// scripts take another route (`powershell.rs`): values as environment
// variables, never args.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::powershell;

/// Shells a script may run in, with the absolute paths to try, in order.
const SHELLS: &[(&str, &[&str])] = &[
    ("sh", &["/bin/sh"]),
    (
        "bash",
        &[
            "/bin/bash",
            "/usr/bin/bash",
            "/usr/local/bin/bash",
            "/opt/homebrew/bin/bash",
        ],
    ),
    ("zsh", &["/bin/zsh", "/usr/bin/zsh"]),
    ("dash", &["/bin/dash", "/usr/bin/dash"]),
    ("ksh", &["/bin/ksh", "/usr/bin/ksh"]),
];

/// `$0` inside the script: fixed, so a value never shifts into `$0`.
pub(crate) const SCRIPT_NAME: &str = "bnto";

/// The `script` param, when the step runs a script.
pub(crate) fn script(params: &Map<String, Value>) -> Option<&str> {
    params.get("script").and_then(Value::as_str)
}

/// The `shell` param, `sh` when unset.
pub(crate) fn shell_name(params: &Map<String, Value>) -> &str {
    params.get("shell").and_then(Value::as_str).unwrap_or("sh")
}

/// Whether the step's values reach its script as `$env:BNTO_ARG_n` rather
/// than as args (a PowerShell script).
pub(crate) fn values_in_env(params: &Map<String, Value>) -> bool {
    script(params).is_some() && powershell::is_powershell(shell_name(params))
}

/// What's wrong with a script step's params, for `validate`.
pub(crate) fn script_problems(params: &Map<String, Value>) -> Vec<String> {
    let mut problems = Vec::new();
    if params.contains_key("command") {
        problems.push("set 'command' or 'script', not both".to_string());
    }
    if script(params).is_some_and(|s| s.trim().is_empty()) {
        problems.push("'script' must not be empty".to_string());
    }
    problems.extend(shell_problem(shell_name(params)));
    if params.contains_key("searchPath") {
        problems.push("'searchPath' doesn't apply to a script; the shell is fixed".to_string());
    }
    problems.extend(reserved_env_problem(params));
    if powershell::is_powershell(shell_name(params)) {
        problems.extend(script(params).and_then(powershell::length_problem));
    }
    problems
}

/// Every shell a script may name, POSIX first.
fn shell_names() -> Vec<&'static str> {
    let posix = SHELLS.iter().map(|(name, _)| *name);
    posix.chain(powershell::SHELLS.iter().copied()).collect()
}

fn shell_problem(shell: &str) -> Option<String> {
    let names = shell_names();
    (!names.contains(&shell))
        .then(|| format!("'shell' must be one of {}, not '{shell}'", names.join(", ")))
}

/// `BNTO_ARG_*` belongs to the values a PowerShell script reads, so a
/// script step's own `env` may not name it (SEC-37).
fn reserved_env_problem(params: &Map<String, Value>) -> Option<String> {
    let env = params.get("env")?.as_object()?;
    let prefix = powershell::ARG_PREFIX;
    env.keys()
        .any(|name| name.to_uppercase().starts_with(prefix))
        .then(|| format!("'env' may not set {prefix}* in a script step; those carry 'args'"))
}

/// Problems in a script step's unfilled params: a script or shell that data
/// could write. Run config (`{{vars.*}}`) is refused here too, so the
/// script approved at consent is the script that runs.
pub(crate) fn raw_script_problems(raw: &Map<String, Value>) -> Vec<String> {
    ["script", "shell"]
        .iter()
        .filter_map(|key| {
            let text = raw.get(*key)?.as_str()?;
            let templated =
                bnto_core::placeholders::mentions_data(text) || text.contains("{{vars.");
            templated.then(|| {
                format!(
                    "'{key}' must be written literally — templates would write the script. \
                     Pass values in 'args' and read them as \"$1\", \"$2\"…"
                )
            })
        })
        .collect()
}

/// The program and leading args for a script: `<abs shell> [-p|-f] -c
/// <script> bnto`, after which the caller appends the step's args as
/// `$1..$N`; or PowerShell's whole command line, which takes no more args.
pub(crate) fn invocation(shell: &str, script: &str) -> Result<(PathBuf, Vec<String>), String> {
    if powershell::is_powershell(shell) {
        return powershell::invocation(shell, script);
    }
    if cfg!(windows) {
        return Err(format!(
            "'{shell}' is a POSIX shell, which Windows doesn't have; \
             use 'powershell' there"
        ));
    }
    let path = shell_path(shell, Path::exists)?;
    let mut args: Vec<String> = startup_off_flag(shell)
        .iter()
        .map(|f| f.to_string())
        .collect();
    args.extend([
        "-c".to_string(),
        script.to_string(),
        SCRIPT_NAME.to_string(),
    ]);
    Ok((path, args))
}

/// The first installed copy of `shell`, by absolute path only — never a
/// PATH or `searchPath` lookup a planted `sh` could win.
fn shell_path(shell: &str, exists: impl Fn(&Path) -> bool) -> Result<PathBuf, String> {
    let (_, candidates) = SHELLS
        .iter()
        .find(|(name, _)| *name == shell)
        .ok_or_else(|| format!("unknown shell '{shell}'"))?;
    let candidates: Vec<PathBuf> = candidates.iter().map(PathBuf::from).collect();
    first_installed(shell, &candidates, exists)
}

/// The first of `candidates` that exists, else an error naming them all.
pub(crate) fn first_installed(
    shell: &str,
    candidates: &[PathBuf],
    exists: impl Fn(&Path) -> bool,
) -> Result<PathBuf, String> {
    candidates
        .iter()
        .find(|p| exists(p))
        .cloned()
        .ok_or_else(|| {
            let looked: Vec<String> = candidates.iter().map(|p| p.display().to_string()).collect();
            format!("{shell} isn't installed (looked in {})", looked.join(", "))
        })
}

/// The flag that stops a shell reading startup files and imported
/// functions: bash's privileged mode, zsh's no-rcs. The cleared env covers
/// the others.
fn startup_off_flag(shell: &str) -> &'static [&'static str] {
    match shell {
        "bash" => &["-p"],
        "zsh" => &["-f"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests;
