// Dry-run inspection — shows exactly which commands a recipe will execute.
//
// `bnto dry-run <recipe>` performs static analysis on a recipe's pipeline
// to display resolved shell commands without executing anything. Useful for
// verifying what tools will run with which arguments before committing.

use std::collections::BTreeMap;

use bnto_core::context::NoopContext;
use bnto_core::executor::resolve::collect_field_values;
use bnto_core::executor::template::{TemplateContext, resolve_templates};
use bnto_core::{Dependency, PipelineDefinition, PipelineNode};
use bnto_engine::deps::collect_pipeline_dependencies;
use bnto_engine::recipes::builtin_recipe_by_slug;

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

/// Full dry-run result for a recipe.
#[derive(Debug)]
pub struct DryRunResult {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub shell_commands: Vec<ShellCommandInfo>,
    pub other_node_count: usize,
}

/// Perform dry-run analysis on a built-in recipe.
///
/// Returns `None` if the slug doesn't match any recipe.
/// Applies `--param` overrides to field defaults before resolving.
pub fn dry_run_recipe(slug: &str, param_overrides: &[String]) -> Option<DryRunResult> {
    let recipe = builtin_recipe_by_slug(slug)?;
    let mut def: PipelineDefinition = serde_json::from_str(recipe.definition_json).ok()?;

    // Apply field overrides before resolving templates.
    apply_field_overrides(&mut def, param_overrides);

    let registry = bnto_engine::create_registry();
    let dependencies = collect_pipeline_dependencies(&def, &registry);

    let mut shell_commands = Vec::new();
    let mut other_node_count = 0;
    collect_shell_commands(&def.nodes, &mut shell_commands, &mut other_node_count);

    Some(DryRunResult {
        name: recipe.name,
        description: recipe.description,
        dependencies,
        shell_commands,
        other_node_count,
    })
}

/// Walk the node tree and extract resolved shell-command info.
fn collect_shell_commands(
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
        .map(|arr| {
            arr.iter()
                .filter_map(serde_json::Value::as_str)
                .map(String::from)
                .collect()
        })
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

/// Apply `--param key=value` overrides to node field defaults.
///
/// Supports `key=value` (targets first shell-command node) and
/// `nodeId:key=value` (targets specific node).
fn apply_field_overrides(def: &mut PipelineDefinition, overrides: &[String]) {
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
    use bnto_core::FieldDef;
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

/// Print dry-run results to stdout with colored formatting.
pub fn print_dry_run(slug: &str, result: &DryRunResult) {
    use colored::Colorize;

    println!("\n{}", result.name.bold());
    println!("{}\n", result.description);

    if !result.dependencies.is_empty() {
        println!("  {}", "Dependencies:".dimmed());
        for dep in &result.dependencies {
            let version = if dep.version.is_empty() {
                String::new()
            } else {
                format!(" ({})", dep.version)
            };
            println!("    {}{version}", dep.binary.yellow());
        }
        println!();
    }

    if result.shell_commands.is_empty() {
        println!(
            "  {}",
            "No shell commands — this recipe runs entirely in-process.".dimmed()
        );
        if result.other_node_count > 0 {
            println!(
                "  {} processing node{}",
                result.other_node_count,
                if result.other_node_count == 1 {
                    ""
                } else {
                    "s"
                }
            );
        }
    } else {
        println!("  {}", "Commands:".dimmed());
        for cmd in &result.shell_commands {
            println!("\n    {} ({})", cmd.node_name.cyan(), cmd.node_id.dimmed());
            println!("    {} {}", "$".dimmed(), format_command_line(cmd).bold());
            println!("    {}  {}", "Output:".dimmed(), cmd.output_mode);

            if !cmd.unresolved.is_empty() {
                println!(
                    "    {}  {} (resolved at runtime)",
                    "Placeholders:".dimmed(),
                    cmd.unresolved.join(", ").yellow()
                );
            }
        }
    }

    println!(
        "\n{}",
        format!("Run with: bnto run {slug} <input>").dimmed()
    );
}

/// Format a command and its args as a shell-like command line.
fn format_command_line(cmd: &ShellCommandInfo) -> String {
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

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    /// Plain text formatter for test assertions (no ANSI colors).
    fn format_dry_run(result: &DryRunResult) -> String {
        let mut out = String::new();
        writeln!(out, "  {}", result.name).ok();
        writeln!(out, "  {}", result.description).ok();

        if !result.dependencies.is_empty() {
            writeln!(out).ok();
            writeln!(out, "  Dependencies:").ok();
            for dep in &result.dependencies {
                write!(out, "    {}", dep.binary).ok();
                if !dep.version.is_empty() {
                    write!(out, " ({})", dep.version).ok();
                }
                writeln!(out).ok();
            }
        }

        if result.shell_commands.is_empty() {
            writeln!(out).ok();
            writeln!(out, "  No shell commands").ok();
            if result.other_node_count > 0 {
                writeln!(out, "  {} processing node(s)", result.other_node_count).ok();
            }
        } else {
            writeln!(out).ok();
            writeln!(out, "  Commands:").ok();
            for cmd in &result.shell_commands {
                writeln!(out, "    {} ({})", cmd.node_name, cmd.node_id).ok();
                writeln!(out, "    $ {}", format_command_line(cmd)).ok();
                writeln!(out, "    Output: {}", cmd.output_mode).ok();
                if !cmd.unresolved.is_empty() {
                    writeln!(out, "    Placeholders: {}", cmd.unresolved.join(", ")).ok();
                }
            }
        }

        out
    }

    // --- dry_run_recipe ---

    #[test]
    fn test_dry_run_known_recipe_with_shell_commands() {
        let result = dry_run_recipe("download-video", &[]);
        assert!(result.is_some(), "download-video should exist");

        let result = result.unwrap();
        assert_eq!(result.name, "Download Video");
        assert!(
            !result.shell_commands.is_empty(),
            "download-video has shell-command nodes"
        );
    }

    #[test]
    fn test_dry_run_unknown_recipe() {
        let result = dry_run_recipe("nonexistent-recipe", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_dry_run_resolves_field_defaults() {
        let result = dry_run_recipe("download-video", &[]).unwrap();
        let cmd = &result.shell_commands[0];

        // Default format is "mp4" — should be resolved, not "{{fields.format}}"
        assert_eq!(cmd.command, "yt-dlp");
        assert!(
            cmd.args.iter().any(|a| a == "mp4"),
            "Default format 'mp4' should appear in resolved args: {:?}",
            cmd.args
        );
        assert!(
            !cmd.args.iter().any(|a| a.contains("{{fields.")),
            "No unresolved {{fields.*}} templates should remain: {:?}",
            cmd.args
        );
    }

    #[test]
    fn test_dry_run_shows_runtime_placeholders() {
        let result = dry_run_recipe("download-video", &[]).unwrap();
        let cmd = &result.shell_commands[0];

        // {{output_dir}} needs runtime resolution — should be flagged
        assert!(
            cmd.unresolved.contains(&"{{output_dir}}".to_string()),
            "Should flag {{output_dir}} as unresolved: {:?}",
            cmd.unresolved
        );
    }

    #[test]
    fn test_dry_run_no_shell_commands() {
        let result = dry_run_recipe("compress-images", &[]);
        assert!(result.is_some(), "compress-images should exist");

        let result = result.unwrap();
        assert!(
            result.shell_commands.is_empty(),
            "compress-images has no shell-command nodes"
        );
        assert!(
            result.other_node_count > 0,
            "compress-images has processing nodes"
        );
    }

    #[test]
    fn test_dry_run_with_param_override() {
        let result = dry_run_recipe("download-video", &["format=webm".to_string()]).unwrap();
        let cmd = &result.shell_commands[0];

        assert!(
            cmd.args.iter().any(|a| a == "webm"),
            "Override format 'webm' should appear in resolved args: {:?}",
            cmd.args
        );
        assert!(
            !cmd.args.iter().any(|a| a == "mp4"),
            "Default 'mp4' should not appear when overridden: {:?}",
            cmd.args
        );
    }

    #[test]
    fn test_dry_run_output_mode() {
        let result = dry_run_recipe("download-video", &[]).unwrap();
        let cmd = &result.shell_commands[0];
        assert_eq!(cmd.output_mode, "file");
    }

    #[test]
    fn test_dry_run_has_dependencies() {
        let result = dry_run_recipe("download-video", &[]).unwrap();
        let dep_names: Vec<&str> = result
            .dependencies
            .iter()
            .map(|d| d.binary.as_str())
            .collect();
        assert!(dep_names.contains(&"yt-dlp"));
        assert!(dep_names.contains(&"ffmpeg"));
    }

    #[test]
    fn test_dry_run_image_recipe_no_dependencies() {
        let result = dry_run_recipe("compress-images", &[]).unwrap();
        assert!(result.dependencies.is_empty());
    }

    // --- format_command_line ---

    #[test]
    fn test_format_command_line_simple() {
        let cmd = ShellCommandInfo {
            node_id: "n1".to_string(),
            node_name: "Test".to_string(),
            command: "echo".to_string(),
            args: vec!["hello".to_string(), "world".to_string()],
            output_mode: "stdout".to_string(),
            unresolved: vec![],
        };
        assert_eq!(format_command_line(&cmd), "echo hello world");
    }

    #[test]
    fn test_format_command_line_quotes_special() {
        let cmd = ShellCommandInfo {
            node_id: "n1".to_string(),
            node_name: "Test".to_string(),
            command: "yt-dlp".to_string(),
            args: vec!["--flag".to_string(), "{{output_dir}}/out.mp4".to_string()],
            output_mode: "file".to_string(),
            unresolved: vec![],
        };
        let formatted = format_command_line(&cmd);
        assert!(formatted.contains("'{{output_dir}}/out.mp4'"));
    }

    // --- format_dry_run ---

    #[test]
    fn test_format_dry_run_contains_command() {
        let result = dry_run_recipe("download-video", &[]).unwrap();
        let formatted = format_dry_run(&result);
        assert!(formatted.contains("yt-dlp"));
        assert!(formatted.contains("Download Video"));
    }

    #[test]
    fn test_format_dry_run_no_shell_shows_message() {
        let result = dry_run_recipe("compress-images", &[]).unwrap();
        let formatted = format_dry_run(&result);
        assert!(formatted.contains("No shell commands"));
    }
}
