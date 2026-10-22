// What the consent prompt shows for each shell step (SEC-16): the full
// command line as the recipe wrote it — args, templates and groups — plus
// where the command is found and which env names it sets. Approving a
// recipe approves exactly these lines.

use bnto_core::PipelineNode;
use serde_json::{Map, Value};

/// One line per shell step, in recipe order, without repeats. A step
/// that runs only when `when` guards hold (its own or its containers')
/// says so: consent covers every branch, whichever this machine takes.
pub fn shell_command_lines(nodes: &[PipelineNode]) -> Vec<String> {
    let mut lines = Vec::new();
    collect(nodes, &[], &mut lines);
    lines
}

fn collect(nodes: &[PipelineNode], guards: &[String], lines: &mut Vec<String>) {
    for node in nodes {
        let mut guards = guards.to_vec();
        guards.extend(node.when.as_ref().map(|w| w.describe()));
        if node.node_type == bnto_shell::NODE_TYPE
            && let Some(line) = command_line(&node.params).map(|l| with_guards(l, &guards))
            && !lines.contains(&line)
        {
            lines.push(line);
        }
        if let Some(children) = &node.children {
            collect(children, &guards, lines);
        }
    }
}

/// `line` with `  (when a, b)` after its first row, above any script.
fn with_guards(line: String, guards: &[String]) -> String {
    if guards.is_empty() {
        return line;
    }
    let note = format!("  (when {})", guards.join(", "));
    match line.split_once('\n') {
        Some((first, rest)) => format!("{first}{note}\n{rest}"),
        None => format!("{line}{note}"),
    }
}

/// `command arg …`, then `(from <searchPath>)` and `[env: NAME, …]`. A
/// script step reads `<shell> -c <script> bnto arg …` (a PowerShell one
/// `<shell> <script> $env:BNTO_ARG_n: arg …`) with the whole script below
/// it, so what's approved is the script that runs.
fn command_line(params: &Map<String, Value>) -> Option<String> {
    let script = params.get("script").and_then(Value::as_str);
    let mut line = match (params.get("command").and_then(Value::as_str), script) {
        (Some(command), _) => command.to_string(),
        (None, Some(_)) => {
            let shell = params.get("shell").and_then(Value::as_str).unwrap_or("sh");
            if bnto_shell::is_powershell(shell) {
                format!("{shell} <script> $env:BNTO_ARG_n:")
            } else {
                format!("{shell} -c <script> bnto")
            }
        }
        (None, None) => return None,
    };
    if let Some(Value::Array(args)) = params.get("args") {
        for arg in args {
            line.push(' ');
            line.push_str(&shown_arg(arg));
        }
    }
    if let Ok(folders) = bnto_shell::search_path::read_search_path(params.get("searchPath"))
        && !folders.is_empty()
    {
        line.push_str(&format!(" (from {})", folders.join(", ")));
    }
    if let Some(Value::Object(env)) = params.get("env")
        && !env.is_empty()
    {
        let names: Vec<&str> = env.keys().map(String::as_str).collect();
        line.push_str(&format!("  [env: {}]", names.join(", ")));
    }
    if let Some(script) = script {
        line.push_str("\n      script:");
        for script_line in script.lines() {
            line.push_str("\n        ");
            line.push_str(script_line);
        }
    }
    Some(line)
}

/// An arg as written: text as-is, a conditional group as `[a b]`, an
/// `each` group as `each[a b]`.
fn shown_arg(arg: &Value) -> String {
    match arg {
        Value::String(s) => s.clone(),
        Value::Array(group) => format!("[{}]", joined(group)),
        Value::Object(map) => match map.get("each") {
            Some(Value::Array(group)) => format!("each[{}]", joined(group)),
            _ => arg.to_string(),
        },
        other => other.to_string(),
    }
}

fn joined(group: &[Value]) -> String {
    group.iter().map(shown_arg).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes(json: Value) -> Vec<PipelineNode> {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn shows_the_full_command_line_as_written() {
        let ns = nodes(serde_json::json!([{ "id": "a", "type": "shell-run",
            "parameters": { "command": "git", "args": ["log", "--format=%H", "{{fields.v}}"] } }]));
        assert_eq!(
            shell_command_lines(&ns),
            ["git log --format=%H {{fields.v}}"]
        );
    }

    #[test]
    fn shows_groups_search_path_and_env_names() {
        let ns = nodes(
            serde_json::json!([{ "id": "a", "type": "shell-run", "parameters": {
            "command": "Build.bat",
            "args": [["--cookies", "{{fields.b}}"], { "each": ["-object={{path}}"] }],
            "searchPath": ["{{env.ROOT}}/bin", "tools"],
            "env": { "TOKEN": "{{env.T}}", "MODE": "x" }
        } }]),
        );
        assert_eq!(
            shell_command_lines(&ns),
            ["Build.bat [--cookies {{fields.b}}] each[-object={{path}}] \
              (from {{env.ROOT}}/bin, tools)  [env: MODE, TOKEN]"]
        );
    }

    #[test]
    fn nested_steps_in_order_without_repeats() {
        let ns = nodes(serde_json::json!([
            { "id": "a", "type": "shell-run", "parameters": { "command": "yt-dlp" } },
            { "id": "g", "type": "group", "children": [
                { "id": "b", "type": "shell-run", "parameters": { "command": "ffmpeg", "args": ["-y"] } },
                { "id": "c", "type": "shell-run", "parameters": { "command": "yt-dlp" } }
            ] }
        ]));
        assert_eq!(shell_command_lines(&ns), ["yt-dlp", "ffmpeg -y"]);
    }

    #[test]
    fn a_script_step_shows_its_shell_args_and_whole_script() {
        let ns = nodes(
            serde_json::json!([{ "id": "s", "type": "shell-run", "parameters": {
            "shell": "bash", "script": "set -eu\nprintf '%s' \"$1\"", "args": ["{{fields.v}}"]
        } }]),
        );
        assert_eq!(
            shell_command_lines(&ns),
            [
                "bash -c <script> bnto {{fields.v}}\n      script:\n        set -eu\n        printf '%s' \"$1\""
            ]
        );
    }

    #[test]
    fn a_powershell_step_shows_its_values_as_variables_not_args() {
        let ns = nodes(
            serde_json::json!([{ "id": "s", "type": "shell-run", "parameters": {
            "shell": "pwsh", "script": "Write-Output $env:BNTO_ARG_1", "args": ["{{fields.v}}", "x"]
        } }]),
        );
        assert_eq!(
            shell_command_lines(&ns),
            [
                "pwsh <script> $env:BNTO_ARG_n: {{fields.v}} x\n      script:\n        Write-Output $env:BNTO_ARG_1"
            ]
        );
    }

    #[test]
    fn guarded_steps_are_listed_whether_or_not_their_guard_holds() {
        // Consent covers every command the recipe could run (SEC-16).
        let ns = nodes(serde_json::json!([
            { "id": "a", "type": "shell-run", "parameters": { "command": "caffeinate" },
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "macos" } },
            { "id": "g", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "linux" },
              "children": [
                { "id": "b", "type": "shell-run", "parameters": { "command": "systemd-inhibit" } }
            ] }
        ]));
        assert_eq!(
            shell_command_lines(&ns),
            [
                "caffeinate  (when {{ctx.platform}} equals macos)",
                "systemd-inhibit  (when {{ctx.platform}} equals linux)",
            ]
        );
    }

    #[test]
    fn nested_guards_all_show_and_sit_above_a_script() {
        let ns = nodes(serde_json::json!([
            { "id": "g", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "macos" },
              "children": [
                { "id": "s", "type": "shell-run",
                  "when": { "value": "{{fields.action}}", "op": "equals", "expected": "start" },
                  "parameters": { "shell": "bash", "script": "echo hi" } }
            ] }
        ]));
        assert_eq!(
            shell_command_lines(&ns),
            [
                "bash -c <script> bnto  (when {{ctx.platform}} equals macos, {{fields.action}} equals start)\n      script:\n        echo hi"
            ]
        );
    }

    #[test]
    fn non_shell_steps_show_nothing() {
        let ns = nodes(serde_json::json!([{ "id": "r", "type": "file-rename", "parameters": {} }]));
        assert!(shell_command_lines(&ns).is_empty());
    }
}
