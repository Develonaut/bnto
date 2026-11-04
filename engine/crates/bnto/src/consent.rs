// Consent gate for `bnto run` and `bnto dry-run`.
//
// The decision itself lives in `trust_gate` (shared with the TUI); this
// module asks on the terminal and exits when the user declines.

use std::process;

use colored::Colorize;

use crate::catalog::SourceKind;
use crate::dry_run_hint::DryRunHint;
use crate::{storage, trust, trust_gate};

/// Check trust level for a recipe and prompt for consent if needed. `kind`
/// is where the recipe came from (`recipe_source::read_recipe`), and
/// approvals live in the configured bnto home's trust file, as the TUI's do.
///
/// Exits the process when the user declines, suggesting the `bnto dry-run`
/// that inspects the same recipe with the same `-p` and `--vars` (`given`).
///
/// `shown_json` is the recipe as the run will execute it (`-p` values
/// filled in), which the prompt shows; the approval is pinned to `raw_json`.
pub fn check_trust(
    (raw_json, shown_json): (&str, &str),
    given: &DryRunHint,
    skip_consent: bool,
    kind: &SourceKind,
) {
    if skip_consent {
        return;
    }
    let recipe_arg = given.recipe_arg;
    let trust_path = storage::BntoPaths::resolve_configured().map(|p| p.trusted_file());
    let store = trust_path
        .as_ref()
        .map(|p| trust::TrustStore::load(p))
        .unwrap_or_default();
    let request = trust_gate::consent_needed_shown(raw_json, shown_json, recipe_arg, kind, &store);
    let Some(request) = request else {
        return;
    };
    if !crate::consent_prompt::prompt_consent(&request) {
        exit_declined(given);
    }
    if let Some(path) = trust_path {
        trust_gate::record_approval(&path, request.hash);
    }
}

/// Explain the refusal and how to inspect the commands safely, then exit.
fn exit_declined(given: &DryRunHint) -> ! {
    eprintln!("{} Execution cancelled.", "Aborted:".red().bold());
    eprintln!(
        "Run {} (without input files) to inspect the commands without running them.",
        given.command().cyan()
    );
    process::exit(1);
}
