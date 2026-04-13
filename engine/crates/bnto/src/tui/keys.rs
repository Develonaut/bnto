// Key event routing — maps keys to AppMessages per screen.

use crossterm::event::{KeyCode, KeyEvent};

use super::app::{AppMessage, AppModel, Screen};
use super::event;
use super::screens::browser::BrowserMessage;
use super::screens::detail::DetailMessage;
use super::screens::execution::ExecutionMessage;
use super::screens::picker::PickerMessage;
use super::screens::results::ResultsMessage;
use super::theme::ALL_VARIANTS;

/// Map a key event to an AppMessage based on the current screen.
///
/// When the browser is in search mode, screen-specific keys take priority
/// so that Esc exits search and character keys type into the query.
pub fn handle_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let searching = matches!(&model.screen, Screen::Browser if model.browser.searching);
    if searching {
        return handle_browser_key(model, key);
    }

    // Detail editing mode captures all keys (like browser search mode).
    let detail_editing = matches!(&model.screen, Screen::Detail { .. }
        if model.detail.as_ref().is_some_and(|d| d.editing));
    if detail_editing {
        return handle_detail_key(model, key);
    }

    // Execution screen captures Esc for cancel (not global Back).
    if matches!(&model.screen, Screen::Execution { .. }) && key.code == KeyCode::Esc {
        return Some(AppMessage::Execution(ExecutionMessage::Cancel));
    }

    if let Some(msg) = event::map_global_key(key) {
        return Some(msg);
    }

    match &model.screen {
        Screen::Browser => handle_browser_key(model, key),
        Screen::Settings => handle_settings_key(model, key),
        Screen::Detail { .. } => handle_detail_key(model, key),
        Screen::Picker { .. } => handle_picker_key(model, key),
        Screen::Execution { .. } => handle_execution_key(model, key),
        Screen::Results { .. } => handle_results_key(model, key),
    }
}

/// Handle key events on the Browser screen.
fn handle_browser_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    if model.browser.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Browser(BrowserMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Browser(BrowserMessage::SearchBackspace)),
            KeyCode::Enter => model
                .browser
                .confirm()
                .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Browser(BrowserMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Browser(BrowserMessage::SearchInput(ch))),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Browser(BrowserMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Browser(BrowserMessage::CursorUp)),
        KeyCode::Char('/') => Some(AppMessage::Browser(BrowserMessage::EnterSearch)),
        KeyCode::Char('s') => Some(AppMessage::OpenSettings),
        KeyCode::Enter => model
            .browser
            .confirm()
            .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
        _ => None,
    }
}

/// Handle key events on the Detail screen.
///
/// When editing a parameter, char keys feed the edit buffer and Enter/Esc
/// commit or cancel. When not editing, j/k navigate params and Enter starts
/// editing or confirms when no params exist.
fn handle_detail_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let editing = model.detail.as_ref().is_some_and(|d| d.editing);

    if editing {
        return match key.code {
            KeyCode::Enter => Some(AppMessage::Detail(DetailMessage::CommitEdit)),
            KeyCode::Esc => Some(AppMessage::Detail(DetailMessage::CancelEdit)),
            KeyCode::Backspace => Some(AppMessage::Detail(DetailMessage::EditBackspace)),
            KeyCode::Char(ch) => Some(AppMessage::Detail(DetailMessage::EditChar(ch))),
            _ => None,
        };
    }

    let slug = || {
        model
            .detail
            .as_ref()
            .map(|d| d.slug.clone())
            .unwrap_or_default()
    };

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Detail(DetailMessage::FocusNext)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Detail(DetailMessage::FocusPrev)),
        KeyCode::Enter => {
            let on_continue = model
                .detail
                .as_ref()
                .is_some_and(|d| d.is_continue_focused() || d.params.is_empty());
            if on_continue {
                Some(AppMessage::ConfigConfirmed { slug: slug() })
            } else {
                Some(AppMessage::Detail(DetailMessage::StartEdit))
            }
        }
        KeyCode::Tab => Some(AppMessage::ConfigConfirmed { slug: slug() }),
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

/// Handle key events on the Picker screen.
fn handle_picker_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let picker = model.picker.as_ref()?;
    let on_dir = picker.cursor < picker.entries.len() && picker.entries[picker.cursor].is_dir;

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Picker(PickerMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Picker(PickerMessage::CursorUp)),
        KeyCode::Char(' ') => Some(AppMessage::Picker(PickerMessage::ToggleSelect)),
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => {
            Some(AppMessage::Picker(PickerMessage::ParentDir))
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if on_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else {
                None
            }
        }
        KeyCode::Char('g') => Some(AppMessage::Picker(PickerMessage::GoToTop)),
        KeyCode::Char('G') => Some(AppMessage::Picker(PickerMessage::GoToBottom)),
        KeyCode::Char('J') | KeyCode::PageDown => Some(AppMessage::Picker(PickerMessage::PageDown)),
        KeyCode::Char('K') | KeyCode::PageUp => Some(AppMessage::Picker(PickerMessage::PageUp)),
        KeyCode::Char('.') => Some(AppMessage::Picker(PickerMessage::ToggleHidden)),
        KeyCode::Char('a') => Some(AppMessage::Picker(PickerMessage::SelectAll)),
        KeyCode::Enter => {
            if on_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else if !picker.selected.is_empty() {
                let slug = picker.slug.clone();
                Some(AppMessage::FilesSelected { slug })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

/// Handle key events on the Execution screen.
fn handle_execution_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Esc => Some(AppMessage::Execution(ExecutionMessage::Cancel)),
        _ => None,
    }
}

/// Handle key events on the Results screen.
fn handle_results_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Results(ResultsMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Results(ResultsMessage::CursorUp)),
        KeyCode::Char('r') => Some(AppMessage::RunAnother),
        _ => None,
    }
}

/// Handle key events on the Settings screen.
fn handle_settings_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let current_idx = ALL_VARIANTS
        .iter()
        .position(|v| *v == model.theme_variant)
        .unwrap_or(0);

    match key.code {
        KeyCode::Up => {
            let prev = if current_idx == 0 {
                ALL_VARIANTS.len() - 1
            } else {
                current_idx - 1
            };
            Some(AppMessage::ThemeChanged(ALL_VARIANTS[prev]))
        }
        KeyCode::Down => {
            let next = (current_idx + 1) % ALL_VARIANTS.len();
            Some(AppMessage::ThemeChanged(ALL_VARIANTS[next]))
        }
        KeyCode::Enter => Some(AppMessage::Back),
        _ => None,
    }
}
