// What the consent prompt shows for each shell step (SEC-16): the full
// command line as the recipe wrote it — args, templates and groups — plus
// where the command is found and which env names it sets. Approving a
// recipe approves exactly these lines.

use bnto_core::PipelineNode;
use serde_json::{Map, Value};

use bnto_core::executor::field_scope::{FieldValues, declare};

use crate::display_text::display_text;
use crate::fill_fields::fill_fields;

/// One line per shell step, in recipe order, without repeats. A step
/// that runs only when `when` guards hold (its own or its containers')
/// says so: consent covers every branch, whichever this machine takes. A
/// step reading `{{fields.*}}` gets a `runs:` row below with the values
/// filled (the nearest declaration's, as a run reads them), since a
/// field's default is the author's to choose.
pub fn shell_command_lines(nodes: &[PipelineNode]) -> Vec<String> {
    let mut lines = Vec::new();
    collect(nodes, &[], &FieldValues::new(), &mut lines);
    lines
}

fn collect(
    nodes: &[PipelineNode],
    guards: &[String],
    values: &FieldValues,
    lines: &mut Vec<String>,
) {
    for node in nodes {
        let mut guards = guards.to_vec();
        guards.extend(node.when.as_ref().map(|w| w.describe()));
        let values = declare(values, &node.fields);
        if node.node_type == bnto_shell::NODE_TYPE
            && let Some(line) = command_line(&node.params)
            && let line = with_guards(with_filled(line, &values), &guards)
            && !lines.contains(&line)
        {
            lines.push(line);
        }
        if let Some(children) = &node.children {
            collect(children, &guards, &values, lines);
        }
    }
}

/// `line` with a `runs:` row after its first, its fields filled, when it
/// reads any.
fn with_filled(line: String, values: &FieldValues) -> String {
    let (first, rest) = line
        .split_once('\n')
        .map_or((line.as_str(), None), |(f, r)| (f, Some(r)));
    let Some(filled) = fill_fields(first, values) else {
        return line;
    };
    let rest = rest.map(|r| format!("\n{r}")).unwrap_or_default();
    format!("{first}\n      runs: {filled}{rest}")
}

/// `line` with `  (when a, b)` after its first row, above any script.
fn with_guards(line: String, guards: &[String]) -> String {
    if guards.is_empty() {
        return line;
    }
    let note = display_text(&format!("  (when {})", guards.join(", "))).into_owned();
    match line.split_once('\n') {
        Some((first, rest)) => format!("{first}{note}\n{rest}"),
        None => format!("{line}{note}"),
    }
}

/// `command arg …`, then `(from <searchPath>)` and `[env: NAME=value, …]`. A
/// script step reads `<shell> -c <script> bnto arg …` (a PowerShell one
/// `<shell> <script> $env:BNTO_ARG_n: arg …`) with the whole script below
/// it, so what's approved is the script that runs. Recipe text is escaped
/// (`display_text`) piece by piece, so a newline in it can't pose as a
/// script line.
fn command_line(params: &Map<String, Value>) -> Option<String> {
    let script = params.get("script").and_then(Value::as_str);
    let mut line = program(params, script)?;
    if let Some(Value::Array(args)) = params.get("args") {
        for arg in args {
            line.push(' ');
            line.push_str(&shown_arg(arg));
        }
    }
    line.push_str(&where_and_env(params));
    let mut line = display_text(&line).into_owned();
    if script.is_some() {
        line.push_str("\n      script:");
    }
    for script_line in script.map(str::lines).into_iter().flatten() {
        line.push_str("\n        ");
        line.push_str(&display_text(script_line));
    }
    Some(line)
}

/// The command, or how its script is run; `None` for a step with neither.
fn program(params: &Map<String, Value>, script: Option<&str>) -> Option<String> {
    if let Some(command) = params.get("command").and_then(Value::as_str) {
        return Some(command.to_string());
    }
    script?;
    let shell = params.get("shell").and_then(Value::as_str).unwrap_or("sh");
    Some(if bnto_shell::is_powershell(shell) {
        format!("{shell} <script> $env:BNTO_ARG_n:")
    } else {
        format!("{shell} -c <script> bnto")
    })
}

/// ` (from a, b)` for a `searchPath`, and `  [env: NAME=value, …]` for an
/// `env`: a value can be a path or a flag the command reads.
fn where_and_env(params: &Map<String, Value>) -> String {
    let mut tail = String::new();
    if let Ok(folders) = bnto_shell::search_path::read_search_path(params.get("searchPath"))
        && !folders.is_empty()
    {
        tail.push_str(&format!(" (from {})", folders.join(", ")));
    }
    if let Some(Value::Object(env)) = params.get("env")
        && !env.is_empty()
    {
        let shown = |v: &Value| v.as_str().map_or_else(|| v.to_string(), str::to_string);
        let pairs: Vec<String> = env
            .iter()
            .map(|(k, v)| format!("{k}={}", shown(v)))
            .collect();
        tail.push_str(&format!("  [env: {}]", pairs.join(", ")));
    }
    tail
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
              (from {{env.ROOT}}/bin, tools)  [env: MODE=x, TOKEN={{env.T}}]"]
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
    fn recipe_text_is_escaped_so_it_cant_pose_as_a_script_line() {
        // ISS-125: an arg's newline and ESC show as text, on the command's row.
        let ns = nodes(
            serde_json::json!([{ "id": "a", "type": "shell-run", "parameters": {
            "command": "git", "args": ["ok\n      script:\n        fine\u{1b}[2K"] } }]),
        );
        assert_eq!(
            shell_command_lines(&ns),
            ["git ok\\n      script:\\n        fine\\e[2K"]
        );
    }

    #[test]
    fn non_shell_steps_show_nothing() {
        let ns = nodes(serde_json::json!([{ "id": "r", "type": "file-rename", "parameters": {} }]));
        assert!(shell_command_lines(&ns).is_empty());
    }
}
