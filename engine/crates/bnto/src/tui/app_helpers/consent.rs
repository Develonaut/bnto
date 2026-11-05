// Consent before a TUI run (SEC-19). The run's bridge checks the run first,
// as `bnto run` does (ISS-77), then for a recipe that needs consent asks here
// and waits: the execution screen holds the run until the user allows its
// commands, and only then does the bridge go on.

use super::super::app::AppModel;
use super::super::bridge::ConsentCheck;
use super::super::screens::execution::{
    ConsentScroll, ExecutionMessage, ExecutionModel, ExecutionStatus,
};
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
/// reloaded copy). Anything else is held to the library's rule. The file is
/// named only when `definition_json` is that file's text.
fn source_kind(model: &AppModel, slug: &str, definition_json: &str) -> SourceKind {
    let catalog = &model.catalog;
    let shipped = catalog
        .resolve_bundled(slug)
        .is_some_and(|e| e.source == RecipeSource::Bundled && e.definition_json == definition_json);
    if shipped {
        return SourceKind::Bundled;
    }
    let Some(entry) = catalog.resolve(slug) else {
        return SourceKind::Library(None);
    };
    let same_text = entry.definition_json == definition_json;
    match entry.source.kind() {
        SourceKind::Project { root, file } => SourceKind::Project {
            root,
            file: file.filter(|_| same_text),
        },
        SourceKind::Library(file) => SourceKind::Library(file.filter(|_| same_text)),
        _ => SourceKind::Library(None),
    }
}

/// Whether `msg` answers or scrolls the consent prompt (`handle_message`).
pub(crate) fn is_consent_message(msg: &ExecutionMessage) -> bool {
    matches!(
        msg,
        ExecutionMessage::AllowCommands | ExecutionMessage::ScrollConsent(_)
    )
}

/// Allow the commands, or scroll the prompt a row.
pub(crate) fn handle_message(model: AppModel, msg: ExecutionMessage) -> AppModel {
    match msg {
        ExecutionMessage::ScrollConsent(way) => scroll(model, way),
        _ => handle_allow(model),
    }
}

/// Move the prompt a row; drawing keeps it within the prompt and notes
/// whether its end shows (`fit_consent`). One row at a time, so the end is
/// reached only past every row.
fn scroll(mut model: AppModel, way: ConsentScroll) -> AppModel {
    if let Some(exec) = model.execution.as_mut() {
        exec.consent_scroll = match way {
            ConsentScroll::Down => exec.consent_scroll.saturating_add(1),
            ConsentScroll::Up => exec.consent_scroll.saturating_sub(1),
        };
    }
    model
}

/// The user allowed the commands: remember the approval and let it run.
fn handle_allow(model: AppModel) -> AppModel {
    let Some(exec) = model.execution.as_ref() else {
        return model;
    };
    let (ExecutionStatus::AwaitingConsent, Some(request)) = (&exec.status, &exec.consent) else {
        return model;
    };
    // Only a prompt read to its end can be allowed, whatever sent this.
    if !exec.consent_at_end {
        return model;
    }
    trust_gate::record_approval(&model.paths.trusted_file(), request.hash.clone());
    let execution = model.execution.map(ExecutionModel::consent_given);
    AppModel { execution, ..model }
}

#[cfg(test)]
mod tests;
