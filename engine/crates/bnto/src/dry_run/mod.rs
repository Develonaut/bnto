// Unified dry-run module — analyzes recipes and previews file transformations.
//
// Routes between two strategies based on what's available:
// - Static analysis: resolve shell-command templates (always available)
// - Pipeline preview: run the pipeline without writes (requires input files)
//
// Both strategies produce a unified `DryRunResult` that the CLI and TUI
// can display consistently.

pub mod files;
mod format;
mod shell;

use bnto_core::{Dependency, PipelineDefinition};
use bnto_engine::deps::collect_pipeline_dependencies;

use crate::catalog::RecipeCatalog;
use files::FilePreview;
use shell::ShellCommandInfo;

pub use format::print_dry_run;

/// Unified dry-run result combining static analysis and pipeline preview.
#[derive(Debug)]
pub struct DryRunResult {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub shell_commands: Vec<ShellCommandInfo>,
    pub file_previews: Vec<FilePreview>,
    pub other_node_count: usize,
    pub output_dir: Option<String>,
    pub duration_ms: Option<u64>,
    pub warnings: Vec<String>,
}

/// Analyze a recipe and produce a unified dry-run result.
///
/// Without inputs: static analysis only (shell commands, dependencies).
/// With inputs: also runs the pipeline without writing, extracts file
/// transformation previews from the engine's metadata.
pub fn dry_run_recipe(
    slug: &str,
    inputs: &[String],
    param_overrides: &[String],
    catalog: &RecipeCatalog,
) -> Option<DryRunResult> {
    let entry = catalog.resolve(slug)?;
    let mut def: PipelineDefinition = serde_json::from_str(&entry.definition_json).ok()?;

    // Apply field overrides before resolving shell-command templates.
    shell::apply_field_overrides(&mut def, param_overrides);

    let registry = bnto_engine::create_registry();
    let dependencies = collect_pipeline_dependencies(&def, &registry);

    // Static analysis: extract shell commands and count other nodes.
    let mut shell_commands = Vec::new();
    let mut other_node_count = 0;
    shell::collect_shell_commands(&def.nodes, &mut shell_commands, &mut other_node_count);

    // Pipeline preview: run with stubbed data if inputs are provided.
    let (file_previews, output_dir, duration_ms, warnings) = if inputs.is_empty() {
        (vec![], None, None, vec![])
    } else {
        match files::run_preview(&entry.definition_json, inputs, param_overrides) {
            Ok(result) => {
                let previews = files::extract_previews(&result);
                (
                    previews,
                    None, // No output dir — we didn't write anything.
                    Some(result.duration_ms),
                    result.warnings,
                )
            }
            Err(e) => {
                eprintln!("Warning: pipeline preview failed: {e}");
                (vec![], None, None, vec![])
            }
        }
    };

    Some(DryRunResult {
        name: entry.name.clone(),
        description: entry.description.clone(),
        dependencies,
        shell_commands,
        file_previews,
        other_node_count,
        output_dir,
        duration_ms,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn catalog() -> RecipeCatalog {
        RecipeCatalog::load(Path::new("/nonexistent"))
    }

    // --- Static analysis tests (migrated from old dry_run.rs) ---

    #[test]
    fn dry_run_known_recipe_with_shell_commands() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c);
        assert!(result.is_some(), "download-video should exist");

        let result = result.unwrap();
        assert_eq!(result.name, "Download Video");
        assert!(!result.shell_commands.is_empty());
        // No inputs → no file previews.
        assert!(result.file_previews.is_empty());
    }

    #[test]
    fn dry_run_unknown_recipe() {
        let c = catalog();
        let result = dry_run_recipe("nonexistent-recipe", &[], &[], &c);
        assert!(result.is_none());
    }

    #[test]
    fn dry_run_resolves_field_defaults() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c).unwrap();
        let cmd = &result.shell_commands[0];

        assert_eq!(cmd.command, "yt-dlp");
        assert!(
            cmd.args.iter().any(|a| a == "mp4"),
            "Default format 'mp4' should appear in resolved args: {:?}",
            cmd.args
        );
        assert!(
            !cmd.args.iter().any(|a| a.contains("{{fields.")),
            "No unresolved {{{{fields.*}}}} templates should remain: {:?}",
            cmd.args
        );
    }

    #[test]
    fn dry_run_shows_runtime_placeholders() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c).unwrap();
        let cmd = &result.shell_commands[0];

        assert!(
            cmd.unresolved.contains(&"{{output_dir}}".to_string()),
            "Should flag {{{{output_dir}}}} as unresolved: {:?}",
            cmd.unresolved
        );
    }

    #[test]
    fn dry_run_no_shell_commands() {
        let c = catalog();
        let result = dry_run_recipe("compress-images", &[], &[], &c);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!(result.shell_commands.is_empty());
        assert!(result.other_node_count > 0);
    }

    #[test]
    fn dry_run_with_param_override() {
        let c = catalog();
        let result =
            dry_run_recipe("download-video", &[], &["format=webm".to_string()], &c).unwrap();
        let cmd = &result.shell_commands[0];

        assert!(
            cmd.args.iter().any(|a| a == "webm"),
            "Override format 'webm' should appear: {:?}",
            cmd.args
        );
        assert!(
            !cmd.args.iter().any(|a| a == "mp4"),
            "Default 'mp4' should not appear when overridden: {:?}",
            cmd.args
        );
    }

    #[test]
    fn dry_run_output_mode() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c).unwrap();
        let cmd = &result.shell_commands[0];
        assert_eq!(cmd.output_mode, "file");
    }

    #[test]
    fn dry_run_has_dependencies() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c).unwrap();
        let dep_names: Vec<&str> = result
            .dependencies
            .iter()
            .map(|d| d.binary.as_str())
            .collect();
        assert!(dep_names.contains(&"yt-dlp"));
        assert!(dep_names.contains(&"ffmpeg"));
    }

    #[test]
    fn dry_run_image_recipe_no_dependencies() {
        let c = catalog();
        let result = dry_run_recipe("compress-images", &[], &[], &c).unwrap();
        assert!(result.dependencies.is_empty());
    }

    // --- Format tests (migrated from old dry_run.rs) ---

    #[test]
    fn format_dry_run_contains_command() {
        let c = catalog();
        let result = dry_run_recipe("download-video", &[], &[], &c).unwrap();
        let formatted = format::format_dry_run_plain(&result);
        assert!(formatted.contains("yt-dlp"));
        assert!(formatted.contains("Download Video"));
    }

    #[test]
    fn format_dry_run_no_shell_shows_message() {
        let c = catalog();
        let result = dry_run_recipe("compress-images", &[], &[], &c).unwrap();
        let formatted = format::format_dry_run_plain(&result);
        assert!(formatted.contains("No shell commands"));
    }
}
