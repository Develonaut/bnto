// Key event routing — maps keys to AppMessages per screen.

mod browser;
mod detail;
mod editor;
mod home;
mod library;
mod picker;
mod run_screens;
mod settings;
mod wizard;

use crossterm::event::{KeyCode, KeyEvent};

use super::app::{AppMessage, AppModel, Screen};
use super::event;
use super::screens::detail_bridge;
use super::screens::editor_bridge;
use super::screens::execution::ExecutionMessage;

use browser::handle_browser_key;
use detail::handle_detail_key;
use editor::handle_editor_key;
use home::handle_home_key;
use library::handle_library_key;
use picker::handle_picker_key;
use run_screens::{handle_execution_key, handle_preview_key, handle_results_key};
use settings::handle_settings_key;
use wizard::handle_wizard_key;

/// A per-screen key handler.
type KeyHandler = fn(&AppModel, KeyEvent) -> Option<AppMessage>;

/// Map a key event to an AppMessage based on the current screen.
///
/// When the browser is in search mode, screen-specific keys take priority
/// so that Esc exits search and character keys type into the query.
pub fn handle_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    if let Some(handler) = capturing_handler(model) {
        return handler(model, key);
    }
    // Execution screen captures Esc for cancel (not global Back).
    if matches!(&model.screen, Screen::Execution { .. }) && key.code == KeyCode::Esc {
        return Some(AppMessage::Execution(ExecutionMessage::Cancel));
    }
    if let Some(msg) = event::map_global_key(key).or_else(|| upgrade_key(model, key)) {
        return Some(msg);
    }
    screen_handler(&model.screen)(model, key)
}

/// U while a newer release is out: quit and upgrade. Not mid-run, where a
/// stray U would cut the run short.
fn upgrade_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let running = matches!(model.screen, Screen::Execution { .. });
    (key.code == KeyCode::Char('U') && model.update_notice.is_some() && !running)
        .then_some(AppMessage::Upgrade)
}

/// Whether a search, question or field-editing mode takes every key, so
/// global keys (`q` to quit) don't apply. The footer asks too (`footer_hints`).
pub(super) fn captures_keys(model: &AppModel) -> bool {
    capturing_handler(model).is_some()
}

/// The screen's handler when a search, modal or field-editing mode is
/// active — those modes capture every key, ahead of global keys.
#[expect(clippy::too_many_lines)]
fn capturing_handler(model: &AppModel) -> Option<KeyHandler> {
    let capturing = match &model.screen {
        Screen::Browser => model.browser.searching,
        Screen::Picker { .. } => model.picker.as_ref().is_some_and(|p| p.searching),
        Screen::Library => model
            .library
            .as_ref()
            .is_some_and(|l| l.searching || l.renaming.is_some() || l.confirming_delete.is_some()),
        Screen::Editor { .. } => model.editor.as_ref().is_some_and(|e| {
            e.picker.is_some()
                || e.confirming_delete
                || e.confirming_dirty_exit
                || e.active_form
                    .as_ref()
                    .is_some_and(editor_bridge::is_editing)
        }),
        Screen::Wizard { .. } => model
            .wizard
            .as_ref()
            .is_some_and(|w| w.form.as_ref().is_some_and(detail_bridge::is_form_editing)),
        Screen::Detail { .. } => model
            .detail
            .as_ref()
            .is_some_and(|d| detail_bridge::is_form_editing(&d.form)),
        _ => false,
    };
    capturing.then(|| screen_handler(&model.screen))
}

/// The key handler for each screen.
fn screen_handler(screen: &Screen) -> KeyHandler {
    match screen {
        Screen::Home => handle_home_key,
        Screen::Library => handle_library_key,
        Screen::Browser => handle_browser_key,
        Screen::Settings => handle_settings_key,
        Screen::Detail { .. } => handle_detail_key,
        Screen::Picker { .. } => handle_picker_key,
        Screen::Execution { .. } => handle_execution_key,
        Screen::Results { .. } => handle_results_key,
        Screen::Preview { .. } => handle_preview_key,
        Screen::Editor { .. } => handle_editor_key,
        Screen::Wizard { .. } => handle_wizard_key,
    }
}
