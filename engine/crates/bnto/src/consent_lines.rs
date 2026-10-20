// What the consent prompt shows for each shell step (SEC-16): the full
// command line as the recipe wrote it — args, templates and groups — plus
// where the command is found and which env names it sets. Approving a
// recipe approves exactly these lines.

use bnto_core::PipelineNode;
use serde_json::{Map, Value};

/// One line per shell step, in recipe order, without repeats.
pub fn shell_command_lines(nodes: &[PipelineNode]) -> Vec<String> {
    let mut lines = Vec::new();
    collect(nodes, &mut lines);
    lines
}

fn collect(nodes: &[PipelineNode], lines: &mut Vec<String>) {
    for node in nodes {
        if node.node_type == "shell-command"
            && let Some(line) = command_line(&node.params)
            && !lines.contains(&line)
        {
            lines.push(line);
        }
        if let Some(children) = &node.children {
            collect(children, lines);
        }
    }
}

/// `command arg …`, then `(from <search_path>)` and `[env: NAME, …]`.
fn command_line(params: &Map<String, Value>) -> Option<String> {
    let command = params.get("command")?.as_str()?;
    let mut line = command.to_string();
    if let Some(Value::Array(args)) = params.get("args") {
        for arg in args {
            line.push(' ');
            line.push_str(&shown_arg(arg));
        }
    }
    if let Ok(folders) = bnto_shell::search_path::read_search_path(params.get("search_path"))
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
        let ns = nodes(serde_json::json!([{ "id": "a", "type": "shell-command",
            "parameters": { "command": "git", "args": ["log", "--format=%H", "{{fields.v}}"] } }]));
        assert_eq!(
            shell_command_lines(&ns),
            ["git log --format=%H {{fields.v}}"]
        );
    }

    #[test]
    fn shows_groups_search_path_and_env_names() {
        let ns = nodes(
            serde_json::json!([{ "id": "a", "type": "shell-command", "parameters": {
            "command": "Build.bat",
            "args": [["--cookies", "{{fields.b}}"], { "each": ["-object={{path}}"] }],
            "search_path": ["{{env.ROOT}}/bin", "tools"],
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
            { "id": "a", "type": "shell-command", "parameters": { "command": "yt-dlp" } },
            { "id": "g", "type": "group", "children": [
                { "id": "b", "type": "shell-command", "parameters": { "command": "ffmpeg", "args": ["-y"] } },
                { "id": "c", "type": "shell-command", "parameters": { "command": "yt-dlp" } }
            ] }
        ]));
        assert_eq!(shell_command_lines(&ns), ["yt-dlp", "ffmpeg -y"]);
    }

    #[test]
    fn non_shell_steps_show_nothing() {
        let ns = nodes(serde_json::json!([{ "id": "r", "type": "file-rename", "parameters": {} }]));
        assert!(shell_command_lines(&ns).is_empty());
    }
}
