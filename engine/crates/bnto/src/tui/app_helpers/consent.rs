// Consent before a TUI run (SEC-19). The run's bridge checks the run first,
// as `bnto run` does (ISS-77), then for a recipe that needs consent asks here
// and waits: the execution screen holds the run until the user allows its
// commands, and only then does the bridge go on.

use super::super::app::AppModel;
use super::super::bridge::ConsentCheck;
use super::super::screens::execution::{ExecutionModel, ExecutionStatus};
use crate::catalog::{RecipeSource, SourceKind};
use crate::trust_gate::{self, ConsentRequest};

/// What the run's bridge needs to decide consent: where `definition_json`
/// came from, and the session's home, where approvals are kept and the run's
/// context is built.
pub(crate) fn consent_check(model: &AppModel, slug: &str, definition_json: &str) -> ConsentCheck {
    ConsentCheck {
        kind: source_kind(model, slug, definition_json),
        paths: model.paths.clone(),
    }
}

/// The bridge's checks passed and the run needs consent: hold it on the
/// execution screen until the user answers.
pub(crate) fn hold(model: AppModel, request: ConsentRequest) -> AppModel {
    let execution = model.execution.map(|e| e.awaiting_consent(request));
    AppModel { execution, ..model }
}

/// Where `definition_json` came from, failing closed. Bundled only for the
/// exact shipped definition: an edited copy run from the editor keeps the
/// slug but not the review. Never weaker than what the slug runs: a project
/// slug keeps the project rule even when its text differs (an edited or
/// reloaded copy). Anything else is held to the library's rule.
fn source_kind(model: &AppModel, slug: &str, definition_json: &str) -> SourceKind {
    let catalog = &model.catalog;
    let shipped = catalog
        .resolve_bundled(slug)
        .is_some_and(|e| e.source == RecipeSource::Bundled && e.definition_json == definition_json);
    if shipped {
        return SourceKind::Bundled;
    }
    match catalog.resolve(slug).map(|e| e.source.kind()) {
        Some(project @ SourceKind::Project(_)) => project,
        _ => SourceKind::Library,
    }
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
