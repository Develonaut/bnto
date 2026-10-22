// `shell-run` static analysis — resolve templates and extract command info.
//
// Walks the recipe's node tree, resolves field defaults and template
// variables, and extracts the shell commands that would be executed.
// No pipeline execution needed — this is pure static analysis.

use std::collections::BTreeMap;

use bnto_core::context::NoopContext;
use bnto_core::executor::field_scope::{self, FieldValues};
use bnto_core::executor::template::{TemplateContext, resolve_templates};
use bnto_core::pipeline::{is_container_node, is_io_node};
use bnto_core::{FieldDef, PipelineDefinition, PipelineNode};

/// A resolved shell-run node ready for display.
#[derive(Debug)]
pub struct ShellCommandInfo {
    pub node_id: String,
    pub node_name: String,
    pub command: String,
    pub args: Vec<String>,
    pub output_mode: String,
    pub unresolved: Vec<String>,
    /// Why it won't run on this host: a `when` guard on it or a container
    /// above it that doesn't hold.
    pub skipped: Option<String>,
}

/// Walk the node tree and extract resolved shell-run info.
///
/// Counts non-shell processing nodes as `other_count` for display.
pub fn collect_shell_commands(
    nodes: &[PipelineNode],
    commands: &mut Vec<ShellCommandInfo>,
    other_count: &mut usize,
) {
    collect_under(nodes, &[], commands, other_count);
}

/// Walk `nodes`, which sit inside the containers in `above`.
fn collect_under<'a>(
    nodes: &'a [PipelineNode],
    above: &[&'a PipelineNode],
    commands: &mut Vec<ShellCommandInfo>,
    other_count: &mut usize,
) {
    for node in nodes {
        let mut path = above.to_vec();
        path.push(node);
        if node.node_type == bnto_shell::NODE_TYPE {
            let mut info = resolve_shell_command(node, &field_scope::along(&path));
            info.skipped = super::host_skip::host_skip(&path);
            commands.push(info);
        } else if !is_io_node(&node.node_type) && !is_container_node(&node.node_type) {
            *other_count += 1;
        }
        if let Some(children) = &node.children {
            collect_under(children, &path, commands, other_count);
        }
    }
}

/// Apply `--param key=value` overrides to node field defaults.
///
/// Supports `key=value` (targets the node that declares field `key`, else
/// the first shell-run node) and `nodeId:key=value` (targets specific node).
/// A value the field refuses (an unlisted enum option, a number out of
/// range), or a key two nodes declare, is not applied; the returned list
/// says why, one entry per refused override.
pub fn apply_field_overrides(def: &mut PipelineDefinition, overrides: &[String]) -> Vec<String> {
    let default_node_id = find_first_shell_command_id(&def.nodes);
    let mut refused = Vec::new();

    for entry in overrides {
        let Some((full_key, value)) = entry.split_once('=') else {
            continue;
        };

        let (node_id, key) = match override_target(def, full_key, default_node_id.as_deref()) {
            Ok(Some(target)) => target,
            Ok(None) => continue,
            Err(e) => {
                refused.push(e);
                continue;
            }
        };

        if let Err(e) = update_field_default(&mut def.nodes, &node_id, &key, value) {
            refused.push(format!("--param {key}: {e}"));
        }
    }
    refused
}

/// `(node id, key)` an override's `full_key` sets: the node it names, else
/// the node that declares the field, else `default_node_id`.
fn override_target(
    def: &PipelineDefinition,
    full_key: &str,
    default_node_id: Option<&str>,
) -> Result<Option<(String, String)>, String> {
    if let Some((nid, k)) = full_key.split_once(':') {
        return Ok(Some((nid.to_string(), k.to_string())));
    }
    let owner = crate::field_owner::field_owner(&def.nodes, full_key)?;
    let node_id = owner.or_else(|| default_node_id.map(str::to_string));
    Ok(node_id.map(|nid| (nid, full_key.to_string())))
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

/// Resolve a shell-run node's fields (`field_values`: its own and its
/// containers') and extract display info.
fn resolve_shell_command(node: &PipelineNode, field_values: &FieldValues) -> ShellCommandInfo {
    let noop = NoopContext;
    let empty_outputs = BTreeMap::new();
    let tpl_ctx = TemplateContext {
        field_values,
        process_ctx: &noop,
        node_outputs: &empty_outputs,
        loop_item: &None,
        run_started_ms: None,
    };
    let resolved_params = resolve_templates(&node.params, &tpl_ctx);

    let (command, mut args) = shown_program(&resolved_params);
    args.extend(
        resolved_params
            .get("args")
            .and_then(serde_json::Value::as_array)
            .map(|arr| bnto_shell::run::flatten_conditional_args(arr))
            .unwrap_or_default(),
    );

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
        skipped: None,
    }
}

/// Find the first shell-run node ID in the tree.
fn find_first_shell_command_id(nodes: &[PipelineNode]) -> Option<String> {
    for node in nodes {
        if node.node_type == bnto_shell::NODE_TYPE {
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

/// The program a step runs and its leading args: `command`, or for a script
/// step `<shell> -c <script> bnto`.
fn shown_program(params: &serde_json::Map<String, serde_json::Value>) -> (String, Vec<String>) {
    let text = |key| params.get(key).and_then(serde_json::Value::as_str);
    match (text("command"), text("script")) {
        (Some(command), _) => (command.to_string(), Vec::new()),
        (None, Some(script)) => (
            text("shell").unwrap_or("sh").to_string(),
            vec!["-c".into(), script.to_string(), "bnto".into()],
        ),
        (None, None) => ("<unknown>".to_string(), Vec::new()),
    }
}

/// Update a node's field default value by name, if the field accepts it.
fn update_field_default(
    nodes: &mut [PipelineNode],
    node_id: &str,
    key: &str,
    value: &str,
) -> Result<(), String> {
    for node in nodes.iter_mut() {
        if node.id == node_id {
            if let Some(field) = node.fields.get_mut(key) {
                field.check_value(value)?;
                match field {
                    FieldDef::String { default, .. } => *default = Some(value.to_string()),
                    FieldDef::Number { default, .. } => *default = value.parse::<f64>().ok(),
                    FieldDef::Boolean { default, .. } => *default = value.parse::<bool>().ok(),
                    FieldDef::Enum { default, .. } => *default = Some(value.to_string()),
                }
            }
            return Ok(());
        }
        if let Some(children) = &mut node.children {
            update_field_default(children, node_id, key, value)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_step_in_another_oss_branch_is_marked_skipped() {
        let nodes: Vec<PipelineNode> = serde_json::from_value(serde_json::json!([
            { "id": "g", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "plan9" },
              "children": [{ "id": "s", "type": "shell-run", "parameters": { "command": "x" } }] },
            { "id": "t", "type": "shell-run", "parameters": { "command": "y" } }
        ]))
        .unwrap();
        let (mut commands, mut others) = (Vec::new(), 0);
        collect_shell_commands(&nodes, &mut commands, &mut others);
        assert!(
            commands[0]
                .skipped
                .as_deref()
                .is_some_and(|r| r.contains("plan9"))
        );
        assert_eq!(commands[1].skipped, None);
    }

    #[test]
    fn containers_are_not_counted_as_steps() {
        let nodes: Vec<PipelineNode> = serde_json::from_value(serde_json::json!([
            { "id": "p", "type": "parallel",
              "children": [{ "id": "a", "type": "file-rename" }] },
            { "id": "out", "type": "output" }
        ]))
        .unwrap();
        let (mut commands, mut others) = (Vec::new(), 0);
        collect_shell_commands(&nodes, &mut commands, &mut others);
        assert_eq!(others, 1, "only the rename");
    }

    #[test]
    fn format_command_line_simple() {
        let cmd = ShellCommandInfo {
            node_id: "n1".into(),
            node_name: "Test".into(),
            command: "echo".into(),
            args: vec!["hello".into(), "world".into()],
            output_mode: "stdout".into(),
            unresolved: vec![],
            skipped: None,
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
            skipped: None,
        };
        let formatted = format_command_line(&cmd);
        assert!(formatted.contains("'{{output_dir}}/out.mp4'"));
    }

    #[test]
    fn refused_override_is_reported_and_not_applied() {
        // SEC-13: the static preview must not show a command with a value
        // the run would refuse.
        let mut def: PipelineDefinition = serde_json::from_str(
            r#"{"nodes": [{
                "id": "run", "type": "shell-run", "parameters": {},
                "fields": {"mode": {"type": "enum", "label": "Mode", "default": "a",
                    "options": [{"value": "a", "label": "A"}, {"value": "b", "label": "B"}]}}
            }]}"#,
        )
        .unwrap();
        let refused = apply_field_overrides(&mut def, &["mode=-x".into(), "mode=b".into()]);
        assert_eq!(refused.len(), 1);
        assert!(refused[0].contains("--param mode"), "{}", refused[0]);
        let FieldDef::Enum { default, .. } = &def.nodes[0].fields["mode"] else {
            panic!("expected Enum");
        };
        assert_eq!(default.as_deref(), Some("b"));
    }

    /// A `session` group declaring `action`, holding one echo step.
    fn group_field_def() -> PipelineDefinition {
        serde_json::from_str(
            r#"{"nodes": [{ "id": "session", "type": "group",
                "fields": {"action": {"type": "enum", "label": "Action", "default": "start",
                    "options": [{"value": "start", "label": "Start"}, {"value": "stop", "label": "Stop"}]}},
                "nodes": [{ "id": "run", "type": "shell-run",
                    "parameters": {"command": "echo", "args": ["{{fields.action}}"]} }] }]}"#,
        )
        .unwrap()
    }

    #[test]
    fn a_step_shows_its_groups_field() {
        let mut def = group_field_def();
        let refused = apply_field_overrides(&mut def, &["action=stop".into()]);
        assert!(refused.is_empty(), "{refused:?}");
        let (mut commands, mut others) = (Vec::new(), 0);
        collect_shell_commands(&def.nodes, &mut commands, &mut others);
        assert_eq!(format_command_line(&commands[0]), "echo stop");
    }

    #[test]
    fn a_key_two_nodes_declare_is_refused() {
        let mut def = group_field_def();
        let step = &mut def.nodes[0].children.as_mut().unwrap()[0];
        step.fields = def_fields(r#"{"action": {"type": "string", "label": "Action"}}"#);
        let refused = apply_field_overrides(&mut def, &["action=stop".into()]);
        assert_eq!(refused.len(), 1);
        assert!(refused[0].contains("session:action="), "{}", refused[0]);
    }

    fn def_fields(json: &str) -> bnto_core::FieldDefs {
        serde_json::from_str(json).unwrap()
    }
}
