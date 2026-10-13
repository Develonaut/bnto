// Editor screen keys — overlays, inline form editing and navigation.

use crossterm::event::{KeyCode, KeyEvent};

use super::super::app::{AppMessage, AppModel};
use super::super::screens::editor::EditorMessage;
use super::super::screens::editor_bridge;

/// Handle key events on the Editor screen.
///
/// When a picker overlay or delete confirmation is active, keys are routed
/// to the overlay. Otherwise, normal editor navigation applies.
pub(super) fn handle_editor_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let editor = model.editor.as_ref()?;

    // Picker overlay captures text input.
    if editor.picker.is_some() {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::PickerCancel)),
            KeyCode::Enter => Some(AppMessage::Editor(EditorMessage::PickerSelect)),
            KeyCode::Backspace => Some(AppMessage::Editor(EditorMessage::PickerBackspace)),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Editor(EditorMessage::PickerClear))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                Some(AppMessage::Editor(EditorMessage::PickerCursorDown))
            }
            KeyCode::Up | KeyCode::Char('k') => {
                Some(AppMessage::Editor(EditorMessage::PickerCursorUp))
            }
            KeyCode::Char(ch) => Some(AppMessage::Editor(EditorMessage::PickerInput(ch))),
            _ => None,
        };
    }

    // Delete confirmation.
    if editor.confirming_delete {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                Some(AppMessage::Editor(EditorMessage::DeleteConfirm))
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                Some(AppMessage::Editor(EditorMessage::DeleteCancel))
            }
            _ => None,
        };
    }

    // Dirty-exit confirmation.
    if editor.confirming_dirty_exit {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                Some(AppMessage::Editor(EditorMessage::DirtySave))
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                Some(AppMessage::Editor(EditorMessage::DirtyDiscard))
            }
            KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::DirtyCancel)),
            _ => None,
        };
    }

    // If an inline form is active, route keys through the form.
    if let Some(active) = &editor.active_form {
        let is_editing = editor_bridge::is_editing(active);

        // When a field is in editing mode, delegate everything to tonkotsu.
        if is_editing {
            return tonkotsu::map_key_event(key, &active.form).map(AppMessage::EditorForm);
        }

        // Selected node is expanded with a form: form navigation + vim shortcuts.
        let vim_msg = match key.code {
            KeyCode::Char('j') => Some(tonkotsu::FormMessage::FocusNext),
            KeyCode::Char('k') => Some(tonkotsu::FormMessage::FocusPrev),
            KeyCode::Char('h') => Some(tonkotsu::FormMessage::CyclePrev),
            KeyCode::Char('l') => Some(tonkotsu::FormMessage::CycleNext),
            KeyCode::Char(' ') => Some(tonkotsu::FormMessage::ToggleConfirm),
            _ => None,
        };
        if let Some(msg) = vim_msg {
            return Some(AppMessage::EditorForm(msg));
        }

        // Esc collapses the node (exits form).
        if key.code == KeyCode::Esc {
            return Some(AppMessage::Editor(EditorMessage::ExpandToggle));
        }

        // Fall through to tonkotsu's idle key mapping.
        if let Some(msg) = tonkotsu::map_key_event(key, &active.form) {
            return Some(AppMessage::EditorForm(msg));
        }
    }

    // Normal editor keys (no active form).
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Editor(EditorMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Editor(EditorMessage::CursorUp)),
        KeyCode::Enter => Some(AppMessage::Editor(EditorMessage::ExpandToggle)),
        KeyCode::Char('a') => Some(AppMessage::Editor(EditorMessage::OpenPicker)),
        KeyCode::Char('d') => Some(AppMessage::Editor(EditorMessage::DeleteRequest)),
        KeyCode::Char('J') => Some(AppMessage::Editor(EditorMessage::MoveDown)),
        KeyCode::Char('K') => Some(AppMessage::Editor(EditorMessage::MoveUp)),
        KeyCode::Char('u') => Some(AppMessage::Editor(EditorMessage::Undo)),
        KeyCode::Char('s')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            Some(AppMessage::Editor(EditorMessage::Save))
        }
        KeyCode::Char('r')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            Some(AppMessage::Editor(EditorMessage::Redo))
        }
        KeyCode::Esc => Some(AppMessage::Editor(EditorMessage::Back)),
        _ => None,
    }
}
