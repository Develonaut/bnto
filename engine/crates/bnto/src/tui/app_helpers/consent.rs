// Consent before a TUI run (SEC-19). A run of an untrusted shell recipe
// waits on the execution screen until the user allows its commands; the
// pipeline bridge only starts a run that is Idle, so it can't start one
// that is still waiting.

use super::super::app::AppModel;
use super::super::screens::execution::{ExecutionModel, ExecutionStatus};
use crate::catalog::RecipeSource;
use crate::trust::TrustStore;
use crate::trust_gate;

/// Hold `exec` for consent when its recipe needs it.
pub(crate) fn gate(exec: ExecutionModel, model: &AppModel) -> ExecutionModel {
    let store = TrustStore::load(&model.paths.trusted_file());
    let bundled = is_bundled_definition(model, &exec.slug, &exec.definition_json);
    match trust_gate::consent_needed(&exec.definition_json, &exec.slug, bundled, &store) {
        Some(request) => exec.awaiting_consent(request),
        None => exec,
    }
}

/// Bundled means the exact shipped definition, not just its slug: an
/// edited copy run from the editor, or a library recipe of the same slug,
/// keeps the slug but not the review.
fn is_bundled_definition(model: &AppModel, slug: &str, definition_json: &str) -> bool {
    model
        .catalog
        .resolve_bundled(slug)
        .is_some_and(|e| e.source == RecipeSource::Bundled && e.definition_json == definition_json)
}

/// The user allowed the commands: remember the approval and let it run.
pub(crate) fn handle_allow(model: AppModel) -> AppModel {
    let Some(exec) = model.execution.as_ref() else {
        return model;
    };
    let (ExecutionStatus::AwaitingConsent, Some(request)) = (&exec.status, &exec.consent) else {
        return model;
    };
    trust_gate::record_approval(&model.paths.trusted_file(), request.hash.clone());
    let execution = model.execution.map(ExecutionModel::consent_given);
    AppModel { execution, ..model }
}

#[cfg(test)]
mod tests;
