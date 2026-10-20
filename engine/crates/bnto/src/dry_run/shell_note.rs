// The dry-run note that shell commands still execute.
//
// A dry run writes nothing, but shell-run nodes run for real (a dry run
// can't know their output otherwise), so the user is told up front.

use bnto_core::{PipelineDefinition, PipelineNode};
use colored::Colorize;

/// `shell-run` nodes in the tree, container children included.
pub fn count_shell_commands(nodes: &[PipelineNode]) -> usize {
    nodes
        .iter()
        .map(|node| {
            let own = usize::from(node.node_type == bnto_shell::NODE_TYPE);
            own + node.children.as_deref().map_or(0, count_shell_commands)
        })
        .sum()
}

/// `dry run still executes N shell command(s)`, or `None` when there are none.
pub fn shell_note_text(definition_json: &str) -> Option<String> {
    let def: PipelineDefinition = serde_json::from_str(definition_json).ok()?;
    let count = count_shell_commands(&def.nodes);
    let plural = if count == 1 { "" } else { "s" };
    (count > 0).then(|| format!("dry run still executes {count} shell command{plural}"))
}

/// Print the note to stderr when the recipe has shell-run nodes.
pub fn print_shell_note(definition_json: &str) {
    if let Some(text) = shell_note_text(definition_json) {
        eprintln!("  {} {text}", "Note:".yellow().bold());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recipe(nodes: &str) -> String {
        format!(r#"{{"id":"r","type":"group","version":"1.0.0","name":"R","nodes":{nodes}}}"#)
    }

    #[test]
    fn test_no_note_without_shell_commands() {
        let json = recipe(r#"[{"id":"a","type":"file-rename","parameters":{}}]"#);
        assert_eq!(shell_note_text(&json), None);
    }

    #[test]
    fn test_note_singular() {
        let json = recipe(r#"[{"id":"a","type":"shell-run","parameters":{}}]"#);
        assert_eq!(
            shell_note_text(&json).as_deref(),
            Some("dry run still executes 1 shell command")
        );
    }

    #[test]
    fn test_note_counts_nested_shell_commands() {
        let json = recipe(
            r#"[{"id":"a","type":"shell-run","parameters":{}},
                {"id":"l","type":"loop","parameters":{},"children":[
                  {"id":"b","type":"shell-run","parameters":{}}]}]"#,
        );
        assert_eq!(
            shell_note_text(&json).as_deref(),
            Some("dry run still executes 2 shell commands")
        );
    }

    #[test]
    fn test_no_note_for_unparseable_recipe() {
        assert_eq!(shell_note_text("not json"), None);
    }
}
