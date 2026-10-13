// Shell-command consent gate shared by `bnto run` and `bnto dry-run`.
//
// Bundled recipes are always trusted. Non-bundled recipes with shell-command
// nodes need first-run consent, cached per definition hash.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::{storage, trust};

/// Check trust level for a recipe and prompt for consent if needed.
///
/// Exits the process when the user declines.
pub fn check_trust(recipe_arg: &str, raw_json: &str, skip_consent: bool, catalog: &RecipeCatalog) {
    let def: bnto_core::PipelineDefinition = match serde_json::from_str(raw_json) {
        Ok(d) => d,
        Err(_) => return, // Let later validation handle parse errors.
    };
    let hash = trust::definition_hash(raw_json);
    let trust_path = storage::BntoPaths::resolve().map(|p| p.trusted_file());
    if skip_consent || is_trusted(recipe_arg, &def, &hash, &trust_path, catalog) {
        return;
    }
    // FirstRun — prompt for consent.
    let recipe_name = extract_recipe_name(raw_json, recipe_arg);
    let cmd_names = trust::collect_shell_command_names(&def.nodes);
    if !trust::prompt_consent(&recipe_name, &cmd_names) {
        exit_declined(recipe_arg);
    }
    if let Some(path) = trust_path {
        cache_approval(&path, hash);
    }
}

/// Whether the recipe can run without a consent prompt.
fn is_trusted(
    recipe_arg: &str,
    def: &bnto_core::PipelineDefinition,
    hash: &str,
    trust_path: &Option<std::path::PathBuf>,
    catalog: &RecipeCatalog,
) -> bool {
    let is_bundled = catalog.is_bundled(recipe_arg);
    let has_shell = trust::has_shell_commands(&def.nodes);
    let store = trust_path
        .as_ref()
        .map(|p| trust::TrustStore::load(p))
        .unwrap_or_default();
    trust::trust_level(is_bundled, has_shell, &store, hash) == trust::TrustLevel::Trusted
}

/// Explain the refusal and how to inspect the commands safely, then exit.
fn exit_declined(recipe_arg: &str) -> ! {
    eprintln!("{} Execution cancelled.", "Aborted:".red().bold());
    eprintln!(
        "Run {} (without input files) to inspect the commands without running them.",
        format!("bnto dry-run {recipe_arg}").cyan()
    );
    process::exit(1);
}

/// Record the approval so the same definition isn't prompted again.
fn cache_approval(path: &std::path::Path, hash: String) {
    let mut store = trust::TrustStore::load(path);
    store.approve(hash);
    let _ = store.save(path);
}

/// Extract recipe name from raw JSON, falling back to the CLI argument.
fn extract_recipe_name(raw_json: &str, fallback: &str) -> String {
    serde_json::from_str::<serde_json::Value>(raw_json)
        .ok()
        .and_then(|v| v["name"].as_str().map(String::from))
        .unwrap_or_else(|| fallback.to_string())
}
