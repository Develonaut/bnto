// Key event routing — maps keys to AppMessages per screen.

use bnto_core::metadata::ParameterType;
use crossterm::event::{KeyCode, KeyEvent};

use super::app::{AppMessage, AppModel, Screen};
use super::event;
use super::screens::browser::BrowserMessage;
use super::screens::detail::DetailMessage;
use super::screens::execution::{ExecutionMessage, ExecutionStatus};
use super::screens::home::HomeMessage;
use super::screens::picker::PickerMessage;
use super::screens::results::ResultsMessage;
use super::screens::settings::SettingsMessage;
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
        Screen::Home => handle_home_key(model, key),
        Screen::Library => None, // Placeholder — only global keys (q, Esc) apply.
        Screen::Browser => handle_browser_key(model, key),
        Screen::Settings => handle_settings_key(model, key),
        Screen::Detail { .. } => handle_detail_key(model, key),
        Screen::Picker { .. } => handle_picker_key(model, key),
        Screen::Execution { .. } => handle_execution_key(model, key),
        Screen::Results { .. } => handle_results_key(model, key),
    }
}

/// Handle key events on the Home screen — 2D pane navigation.
fn handle_home_key(_model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Tab | KeyCode::Char('l') | KeyCode::Right => {
            Some(AppMessage::Home(HomeMessage::NextPane))
        }
        KeyCode::BackTab | KeyCode::Char('h') | KeyCode::Left => {
            Some(AppMessage::Home(HomeMessage::PrevPane))
        }
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Home(HomeMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Home(HomeMessage::CursorUp)),
        KeyCode::Enter => Some(AppMessage::HomeConfirm),
        _ => None,
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

    // Peek at the focused param's type for smart key dispatch.
    let focused_param_type = model
        .detail
        .as_ref()
        .and_then(|d| d.params.get(d.focused))
        .map(|p| &p.param_type);
    let has_constraints = model
        .detail
        .as_ref()
        .and_then(|d| d.params.get(d.focused))
        .is_some_and(|p| p.constraints.is_some());

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Detail(DetailMessage::FocusNext)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Detail(DetailMessage::FocusPrev)),
        KeyCode::Char(' ') => Some(AppMessage::Detail(DetailMessage::ToggleBool)),
        KeyCode::Char('d') => Some(AppMessage::Detail(DetailMessage::ResetDefault)),
        KeyCode::Char('h') | KeyCode::Left => match focused_param_type {
            Some(ParameterType::Enum { .. }) => Some(AppMessage::Detail(DetailMessage::EnumPrev)),
            Some(ParameterType::Number) if has_constraints => {
                Some(AppMessage::Detail(DetailMessage::NumberDecrement))
            }
            _ => None,
        },
        KeyCode::Char('l') | KeyCode::Right => match focused_param_type {
            Some(ParameterType::Enum { .. }) => Some(AppMessage::Detail(DetailMessage::EnumNext)),
            Some(ParameterType::Number) if has_constraints => {
                Some(AppMessage::Detail(DetailMessage::NumberIncrement))
            }
            _ => None,
        },
        KeyCode::Enter => {
            let on_continue = model
                .detail
                .as_ref()
                .is_some_and(|d| d.is_continue_focused() || d.params.is_empty());
            if on_continue {
                Some(AppMessage::ConfigConfirmed { slug: slug() })
            } else {
                // Non-text params use inline controls; Enter starts text edit only for String.
                match focused_param_type {
                    Some(ParameterType::Boolean) => {
                        Some(AppMessage::Detail(DetailMessage::ToggleBool))
                    }
                    Some(ParameterType::Enum { .. }) => {
                        Some(AppMessage::Detail(DetailMessage::EnumNext))
                    }
                    _ => Some(AppMessage::Detail(DetailMessage::StartEdit)),
                }
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
        KeyCode::Tab => {
            if model.settings_picker_field.is_some() {
                Some(AppMessage::SettingsPathConfirmed)
            } else if !picker.selected.is_empty() {
                let slug = picker.slug.clone();
                Some(AppMessage::FilesSelected { slug })
            } else {
                None
            }
        }
        KeyCode::Enter => {
            if on_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else if model.settings_picker_field.is_none() && !picker.selected.is_empty() {
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
///
/// Enter transitions to Results when the pipeline has finished (Completed or Failed).
/// Esc cancels a running pipeline (handled above in handle_key as a special case).
fn handle_execution_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let finished = model.execution.as_ref().is_some_and(|e| {
        matches!(
            e.status,
            ExecutionStatus::Completed | ExecutionStatus::Failed
        )
    });

    match key.code {
        KeyCode::Enter if finished => {
            if let Screen::Execution { slug } = &model.screen {
                Some(AppMessage::ExecutionComplete { slug: slug.clone() })
            } else {
                None
            }
        }
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
///
/// Theme field: left/right arrows cycle through themes.
/// Telemetry field: left/right arrows toggle on/off.
/// Path fields: Enter opens file picker for directory browsing.
fn handle_settings_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let on_theme = model.settings.as_ref().is_some_and(|s| s.focused == 0);

    // Theme field: left/right cycle through variants.
    if on_theme {
        let current_idx = ALL_VARIANTS
            .iter()
            .position(|v| *v == model.theme_variant)
            .unwrap_or(0);
        match key.code {
            KeyCode::Left => {
                let prev = if current_idx == 0 {
                    ALL_VARIANTS.len() - 1
                } else {
                    current_idx - 1
                };
                return Some(AppMessage::ThemeChanged(ALL_VARIANTS[prev]));
            }
            KeyCode::Right => {
                let next = (current_idx + 1) % ALL_VARIANTS.len();
                return Some(AppMessage::ThemeChanged(ALL_VARIANTS[next]));
            }
            _ => {}
        }
    }

    // Telemetry field: left/right toggle on/off.
    let on_telemetry = model.settings.as_ref().is_some_and(|s| {
        s.fields
            .get(s.focused)
            .is_some_and(|f| f.key == "telemetry")
    });
    if on_telemetry && matches!(key.code, KeyCode::Left | KeyCode::Right) {
        let currently_on = model
            .settings
            .as_ref()
            .and_then(|s| s.fields.get(s.focused))
            .is_some_and(|f| f.value == "On");
        return Some(AppMessage::TelemetryToggled(!currently_on));
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            Some(AppMessage::Settings(SettingsMessage::FocusNext))
        }
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Settings(SettingsMessage::FocusPrev)),
        KeyCode::Enter => {
            if model
                .settings
                .as_ref()
                .is_some_and(|s| s.is_focused_editable())
            {
                let field_key = model
                    .settings
                    .as_ref()
                    .and_then(|s| s.fields.get(s.focused))
                    .map(|f| f.key.to_string())
                    .unwrap_or_default();
                Some(AppMessage::OpenSettingsPicker { field_key })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}
