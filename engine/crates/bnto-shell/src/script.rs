// Script mode: a literal script run as `<shell> -c <script> bnto <args...>`.
// Values reach the script only as "$1".."$N" (POSIX `sh -c` operands,
// ShellCheck SC2156), never as script text. The shell is a fixed absolute
// path, started with startup files off (`bash -p`, `zsh -f`) in a cleared
// environment, so nothing but the script runs (SEC-20..26).

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

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

/// What's wrong with a script step's params, for `validate`.
pub(crate) fn script_problems(params: &Map<String, Value>) -> Vec<String> {
    let mut problems = Vec::new();
    if params.contains_key("command") {
        problems.push("set 'command' or 'script', not both".to_string());
    }
    if script(params).is_some_and(|s| s.trim().is_empty()) {
        problems.push("'script' must not be empty".to_string());
    }
    let shell = shell_name(params);
    if !SHELLS.iter().any(|(name, _)| *name == shell) {
        let names: Vec<&str> = SHELLS.iter().map(|(name, _)| *name).collect();
        problems.push(format!(
            "'shell' must be one of {}, not '{shell}'",
            names.join(", ")
        ));
    }
    if params.contains_key("search_path") {
        problems.push("'search_path' doesn't apply to a script; the shell is fixed".to_string());
    }
    problems
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
/// <script> bnto`. The caller appends the step's args as `$1..$N`.
pub(crate) fn invocation(shell: &str, script: &str) -> Result<(PathBuf, Vec<String>), String> {
    if cfg!(windows) {
        return Err(
            "script mode runs a POSIX shell, which Windows doesn't have; \
                    use 'command' there"
                .to_string(),
        );
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
/// PATH or `search_path` lookup a planted `sh` could win.
fn shell_path(shell: &str, exists: impl Fn(&Path) -> bool) -> Result<PathBuf, String> {
    let (_, candidates) = SHELLS
        .iter()
        .find(|(name, _)| *name == shell)
        .ok_or_else(|| format!("unknown shell '{shell}'"))?;
    candidates
        .iter()
        .map(PathBuf::from)
        .find(|p| exists(p))
        .ok_or_else(|| {
            format!(
                "{shell} isn't installed (looked in {})",
                candidates.join(", ")
            )
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
