// Shell-command consent gate for `bnto run` and `bnto dry-run`.
//
// The decision itself lives in `trust_gate` (shared with the TUI); this
// module asks on the terminal and exits when the user declines.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::{storage, trust, trust_gate};

/// Check trust level for a recipe and prompt for consent if needed.
///
/// Exits the process when the user declines.
pub fn check_trust(recipe_arg: &str, raw_json: &str, skip_consent: bool, catalog: &RecipeCatalog) {
    if skip_consent {
        return;
    }
    let trust_path = storage::BntoPaths::resolve().map(|p| p.trusted_file());
    let store = trust_path
        .as_ref()
        .map(|p| trust::TrustStore::load(p))
        .unwrap_or_default();
    let is_bundled = catalog.is_bundled(recipe_arg);
    let Some(request) = trust_gate::consent_needed(raw_json, recipe_arg, is_bundled, &store) else {
        return;
    };
    if !trust::prompt_consent(&request.recipe_name, &request.commands) {
        exit_declined(recipe_arg);
    }
    if let Some(path) = trust_path {
        trust_gate::record_approval(&path, request.hash);
    }
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
