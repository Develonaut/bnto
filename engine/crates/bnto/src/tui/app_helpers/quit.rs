// Quitting the TUI, with or without an upgrade after (ISS-98). From the
// recipe editor with unsaved changes, a quit first asks the editor's "save?"
// question, and is held until the answer: save or discard quits, Esc (or a
// save that fails) stays in the editor and drops the quit.

use crate::tui::app::{AppModel, PendingExit, Screen};
use crate::tui::screens::editor::EditorMessage;

use super::handle_editor;

/// Quit (and upgrade after, with `upgrade`), asking about unsaved editor
/// changes first.
pub(crate) fn request_quit(model: AppModel, upgrade: bool) -> AppModel {
    let exit = PendingExit { upgrade };
    if !matches!(model.screen, Screen::Editor { .. }) {
        return quit(model, exit);
    }
    // The editor's own Back: it syncs a half-edited field and asks when dirty.
    let held = AppModel {
        pending_exit: Some(exit),
        ..model
    };
    settle_pending_exit(handle_editor(held, EditorMessage::Back))
}

/// After an editor message: finish a held quit once the editor has closed,
/// or drop it when the user stayed.
pub(crate) fn settle_pending_exit(model: AppModel) -> AppModel {
    let Some(exit) = model.pending_exit else {
        return model;
    };
    if model
        .editor
        .as_ref()
        .is_some_and(|e| e.confirming_dirty_exit)
    {
        return model;
    }
    let left = !matches!(model.screen, Screen::Editor { .. });
    let model = AppModel {
        pending_exit: None,
        ..model
    };
    if left { quit(model, exit) } else { model }
}

fn quit(model: AppModel, exit: PendingExit) -> AppModel {
    AppModel {
        should_quit: true,
        upgrade_on_exit: model.upgrade_on_exit || exit.upgrade,
        ..model
    }
}
