// Unified dry-run module — analyzes recipes and previews file transformations.
//
// Routes between two strategies based on what's available:
// - Static analysis: resolve shell-run templates (always available)
// - Pipeline preview: run the pipeline without writes (requires input files)
//
// Both strategies produce a unified `DryRunResult` that the CLI and TUI
// can display consistently.

pub mod files;
mod format;
mod host_skip;
mod shell;
mod shell_note;

use bnto_core::{Dependency, HostPlatform, InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::deps::collect_pipeline_dependencies;

use files::FilePreview;
use shell::ShellCommandInfo;

pub use format::print_dry_run;
pub use shell_note::print_shell_note;

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
    /// The recipe's time limit in seconds (`settings.timeout`), if any.
    pub time_limit: Option<u64>,
    /// What the run takes as input, which the `Run with:` footer ends by.
    pub input_mode: InputMode,
}

/// Analyze a recipe and produce a unified dry-run result.
///
/// Without inputs: static analysis only (shell commands, dependencies).
/// With inputs: also runs the pipeline without writing, extracts file
/// transformation previews from the engine's metadata.
///
/// `definition_json` is the recipe as `recipe_source::read_recipe` read it;
/// `recipe_arg` (the slug or path the user typed) names a recipe with no
/// `name`. Fails only on JSON that isn't a recipe.
pub fn dry_run_recipe(
    recipe_arg: &str,
    definition_json: &str,
    inputs: &[String],
    param_overrides: &[String],
) -> Result<DryRunResult, String> {
    let mut def: PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| format!("Invalid recipe JSON: {e}"))?;
    let (name, description) = heading(definition_json, recipe_arg);

    // Apply field overrides before resolving shell-run templates.
    let refused = shell::apply_field_overrides(&mut def, param_overrides);
    let registry = bnto_engine::create_registry();
    let (shell_commands, other_node_count) = shell_steps(&def);
    let preview = preview_files(definition_json, inputs, param_overrides);
    Ok(DryRunResult {
        name,
        description,
        dependencies: collect_pipeline_dependencies(&def, &registry, HostPlatform::current()),
        shell_commands,
        file_previews: preview.file_previews,
        other_node_count,
        output_dir: None, // Nothing is written, so there's no output dir.
        duration_ms: preview.duration_ms,
        warnings: [refused, preview.warnings].concat(),
        time_limit: def.time_limit_secs(),
        input_mode: resolve_input_mode(&def),
    })
}

/// Static analysis: the shell commands, and how many other nodes there are.
fn shell_steps(def: &PipelineDefinition) -> (Vec<ShellCommandInfo>, usize) {
    let mut shell_commands = Vec::new();
    let mut other_node_count = 0;
    shell::collect_shell_commands(&def.nodes, &mut shell_commands, &mut other_node_count);
    (shell_commands, other_node_count)
}

/// The recipe's `name` (else `fallback`) and `metadata.description`, as the
/// catalog reads them.
fn heading(definition_json: &str, fallback: &str) -> (String, String) {
    let val: serde_json::Value = serde_json::from_str(definition_json).unwrap_or_default();
    let name = val["name"].as_str().unwrap_or(fallback).to_string();
    let description = val["metadata"]["description"].as_str().unwrap_or_default();
    (name, description.to_string())
}

/// What the pipeline preview found; empty when it didn't run.
#[derive(Default)]
struct FilesPreview {
    file_previews: Vec<FilePreview>,
    duration_ms: Option<u64>,
    warnings: Vec<String>,
}

/// Run the pipeline without writes when there are inputs to preview.
fn preview_files(definition_json: &str, inputs: &[String], params: &[String]) -> FilesPreview {
    if inputs.is_empty() {
        return FilesPreview::default();
    }
    match files::run_preview(definition_json, inputs, params) {
        Ok(result) => FilesPreview {
            file_previews: files::extract_previews(&result),
            duration_ms: Some(result.duration_ms),
            warnings: result.warnings,
        },
        Err(e) => {
            eprintln!("Warning: pipeline preview failed: {e}");
            FilesPreview::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dry-run a bundled recipe by slug, as `bnto dry-run <slug>` reads it.
    fn bundled(slug: &str, params: &[String]) -> Result<DryRunResult, String> {
        let library = tempfile::tempdir().unwrap();
        let catalog = crate::catalog::RecipeCatalog::load(library.path());
        let json = &catalog.resolve(slug).expect("bundled slug").definition_json;
        dry_run_recipe(slug, json, &[], params)
    }

    // --- Static analysis tests (migrated from old dry_run.rs) ---

    #[test]
    fn dry_run_known_recipe_with_shell_commands() {
        let result = bundled("download-video", &[]).unwrap();
        assert_eq!(result.name, "Download Video");
        assert!(!result.shell_commands.is_empty());
        // No inputs → no file previews.
        assert!(result.file_previews.is_empty());
    }

    #[test]
    fn dry_run_refuses_json_that_is_not_a_recipe() {
        let err = dry_run_recipe("broken.bnto.json", "{ not json", &[], &[]).unwrap_err();
        assert!(err.starts_with("Invalid recipe JSON"), "{err}");
    }

    #[test]
    fn the_preview_heading_comes_from_the_json() {
        let json = r#"{ "name": "Mine", "metadata": { "description": "Does a thing" },
          "nodes": [] }"#;
        let result = dry_run_recipe("mine.bnto.json", json, &[], &[]).unwrap();
        assert_eq!(result.name, "Mine");
        assert_eq!(result.description, "Does a thing");
    }

    #[test]
    fn a_recipe_with_no_name_is_named_by_its_argument() {
        let result = dry_run_recipe("recipes/mine.bnto.json", r#"{"nodes":[]}"#, &[], &[]);
        let result = result.unwrap();
        assert_eq!(result.name, "recipes/mine.bnto.json");
        assert_eq!(result.description, "");
    }

    #[test]
    fn dry_run_resolves_field_defaults() {
        let result = bundled("download-video", &[]).unwrap();
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
        let result = bundled("download-video", &[]).unwrap();
        let cmd = &result.shell_commands[0];

        assert!(
            cmd.unresolved.contains(&"{{output_dir}}".to_string()),
            "Should flag {{{{output_dir}}}} as unresolved: {:?}",
            cmd.unresolved
        );
    }

    #[test]
    fn dry_run_no_shell_commands() {
        let result = bundled("compress-images", &[]).unwrap();
        assert!(result.shell_commands.is_empty());
        assert!(result.other_node_count > 0);
    }

    #[test]
    fn dry_run_with_param_override() {
        let result = bundled("download-video", &["format=webm".to_string()]).unwrap();
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
        let result = bundled("download-video", &[]).unwrap();
        let cmd = &result.shell_commands[0];
        assert_eq!(cmd.output_mode, "file");
    }

    #[test]
    fn dry_run_has_dependencies() {
        let result = bundled("download-video", &[]).unwrap();
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
        let result = bundled("compress-images", &[]).unwrap();
        assert!(result.dependencies.is_empty());
    }

    // --- Format tests (migrated from old dry_run.rs) ---

    #[test]
    fn format_dry_run_contains_command() {
        let result = bundled("download-video", &[]).unwrap();
        let formatted = format::format_dry_run_plain(&result);
        assert!(formatted.contains("yt-dlp"));
        assert!(formatted.contains("Download Video"));
    }

    #[test]
    fn format_dry_run_no_shell_shows_message() {
        let result = bundled("compress-images", &[]).unwrap();
        let formatted = format::format_dry_run_plain(&result);
        assert!(formatted.contains("No shell commands"));
    }
}
