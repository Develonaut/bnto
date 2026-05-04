// Shell-command static analysis — resolve templates and extract command info.
//
// Walks the recipe's node tree, resolves field defaults and template
// variables, and extracts the shell commands that would be executed.
// No pipeline execution needed — this is pure static analysis.

use std::collections::BTreeMap;

use bnto_core::context::NoopContext;
use bnto_core::executor::resolve::collect_field_values;
use bnto_core::executor::template::{TemplateContext, resolve_templates};
use bnto_core::{FieldDef, PipelineDefinition, PipelineNode};

/// A resolved shell-command node ready for display.
#[derive(Debug)]
pub struct ShellCommandInfo {
    pub node_id: String,
    pub node_name: String,
    pub command: String,
    pub args: Vec<String>,
    pub output_mode: String,
    pub unresolved: Vec<String>,
}

/// Walk the node tree and extract resolved shell-command info.
///
/// Counts non-shell processing nodes as `other_count` for display.
pub fn collect_shell_commands(
    nodes: &[PipelineNode],
    commands: &mut Vec<ShellCommandInfo>,
    other_count: &mut usize,
) {
    let skip = ["input", "output", "group", "loop"];
    for node in nodes {
        if skip.contains(&node.node_type.as_str()) {
            if let Some(children) = &node.children {
                collect_shell_commands(children, commands, other_count);
            }
            continue;
        }

        if node.node_type == "shell-command" {
            commands.push(resolve_shell_command(node));
        } else {
            *other_count += 1;
        }

        if let Some(children) = &node.children {
            collect_shell_commands(children, commands, other_count);
        }
    }
}

/// Apply `--param key=value` overrides to node field defaults.
///
/// Supports `key=value` (targets first shell-command node) and
/// `nodeId:key=value` (targets specific node).
pub fn apply_field_overrides(def: &mut PipelineDefinition, overrides: &[String]) {
    let default_node_id = find_first_shell_command_id(&def.nodes);

    for entry in overrides {
        let Some((full_key, value)) = entry.split_once('=') else {
            continue;
        };

        let (node_id, key) = if let Some((nid, k)) = full_key.split_once(':') {
            (nid.to_string(), k.to_string())
        } else if let Some(ref nid) = default_node_id {
            (nid.clone(), full_key.to_string())
        } else {
            continue;
        };

        update_field_default(&mut def.nodes, &node_id, &key, value);
    }
}

/// Format a command and its args as a shell-like command line.
pub fn format_command_line(cmd: &ShellCommandInfo) -> String {
    let mut parts = vec![cmd.command.clone()];
    parts.extend(cmd.args.iter().map(|a| {
        if a.contains(' ') || a.contains('{') {
            format!("'{a}'")
        } else {
            a.clone()
        }
    }));
    parts.join(" ")
}

// --- Internal helpers ---

/// Resolve a shell-command node's fields and extract display info.
fn resolve_shell_command(node: &PipelineNode) -> ShellCommandInfo {
    let field_values = if node.fields.is_empty() {
        BTreeMap::new()
    } else {
        collect_field_values(&node.fields, &BTreeMap::new())
    };
    let noop = NoopContext;
    let empty_outputs = BTreeMap::new();
    let tpl_ctx = TemplateContext {
        field_values: &field_values,
        process_ctx: &noop,
        node_outputs: &empty_outputs,
        loop_item: &None,
    };
    let resolved_params = resolve_templates(&node.params, &tpl_ctx);

    let command = resolved_params
        .get("command")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("<unknown>")
        .to_string();

    let args: Vec<String> = resolved_params
        .get("args")
        .and_then(serde_json::Value::as_array)
        .map(|arr| bnto_shell::execute::flatten_conditional_args(arr))
        .unwrap_or_default();

    let output_mode = resolved_params
        .get("outputMode")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("stdout")
        .to_string();

    // Identify placeholders that need runtime input.
    let runtime_placeholders = ["{{url}}", "{{input}}", "{{output_dir}}"];
    let unresolved: Vec<String> = args
        .iter()
        .flat_map(|arg| {
            runtime_placeholders
                .iter()
                .filter(|p| arg.contains(**p))
                .map(|p| p.to_string())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let node_name = node.id.clone();

    ShellCommandInfo {
        node_id: node.id.clone(),
        node_name,
        command,
        args,
        output_mode,
        unresolved,
    }
}

/// Find the first shell-command node ID in the tree.
fn find_first_shell_command_id(nodes: &[PipelineNode]) -> Option<String> {
    for node in nodes {
        if node.node_type == "shell-command" {
            return Some(node.id.clone());
        }
        if let Some(children) = &node.children
            && let Some(id) = find_first_shell_command_id(children)
        {
            return Some(id);
        }
    }
    None
}

/// Update a node's field default value by name.
fn update_field_default(nodes: &mut [PipelineNode], node_id: &str, key: &str, value: &str) {
    for node in nodes.iter_mut() {
        if node.id == node_id {
            if let Some(field) = node.fields.get_mut(key) {
                match field {
                    FieldDef::String { default, .. } => *default = Some(value.to_string()),
                    FieldDef::Number { default, .. } => *default = value.parse::<f64>().ok(),
                    FieldDef::Boolean { default, .. } => *default = value.parse::<bool>().ok(),
                    FieldDef::Enum { default, .. } => *default = Some(value.to_string()),
                }
            }
            return;
        }
        if let Some(children) = &mut node.children {
            update_field_default(children, node_id, key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_command_line_simple() {
        let cmd = ShellCommandInfo {
            node_id: "n1".into(),
            node_name: "Test".into(),
            command: "echo".into(),
            args: vec!["hello".into(), "world".into()],
            output_mode: "stdout".into(),
            unresolved: vec![],
        };
        assert_eq!(format_command_line(&cmd), "echo hello world");
    }

    #[test]
    fn format_command_line_quotes_special() {
        let cmd = ShellCommandInfo {
            node_id: "n1".into(),
            node_name: "Test".into(),
            command: "yt-dlp".into(),
            args: vec!["--flag".into(), "{{output_dir}}/out.mp4".into()],
            output_mode: "file".into(),
            unresolved: vec![],
        };
        let formatted = format_command_line(&cmd);
        assert!(formatted.contains("'{{output_dir}}/out.mp4'"));
    }
}
