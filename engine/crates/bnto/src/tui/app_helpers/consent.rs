// Consent before a TUI run (SEC-19). The run's bridge checks the run first,
// as `bnto run` does (ISS-77), then for an untrusted shell recipe asks here
// and waits: the execution screen holds the run until the user allows its
// commands, and only then does the bridge go on.

use super::super::app::AppModel;
use super::super::bridge::ConsentCheck;
use super::super::screens::execution::{ExecutionModel, ExecutionStatus};
use crate::catalog::RecipeSource;
use crate::trust_gate::{self, ConsentRequest};

/// What the run's bridge needs to decide consent: whether `definition_json`
/// is the shipped recipe, and where approvals are kept.
pub(crate) fn consent_check(model: &AppModel, slug: &str, definition_json: &str) -> ConsentCheck {
    ConsentCheck {
        bundled: is_bundled_definition(model, slug, definition_json),
        trusted_file: model.paths.trusted_file(),
    }
}

/// The bridge's checks passed and the run needs consent: hold it on the
/// execution screen until the user answers.
pub(crate) fn hold(model: AppModel, request: ConsentRequest) -> AppModel {
    let execution = model.execution.map(|e| e.awaiting_consent(request));
    AppModel { execution, ..model }
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
