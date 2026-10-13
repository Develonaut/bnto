// File picker screen keys — navigation, selection and search mode.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::picker::PickerMessage;

/// Handle key events on the Picker screen.
///
/// When search mode is active, character keys type into the query and
/// navigation is limited to cursor + Esc. Otherwise, full picker keys apply.
pub(super) fn handle_picker_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let picker = model.picker.as_ref()?;

    // Search mode captures text input (same pattern as browser/library).
    if picker.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Picker(PickerMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Picker(PickerMessage::SearchBackspace)),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Picker(PickerMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Picker(PickerMessage::SearchInput(ch))),
            _ => None,
        };
    }

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
        KeyCode::Char('/') => Some(AppMessage::Picker(PickerMessage::EnterSearch)),
        KeyCode::Char('p') => Some(AppMessage::Picker(PickerMessage::ToggleMetadata)),
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
