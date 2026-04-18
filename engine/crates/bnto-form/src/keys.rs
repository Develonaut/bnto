//! Key event mapping — translates crossterm `KeyEvent` to `FormMessage`.
//!
//! This is the bridge between terminal input and the TEA message system.
//! Context-aware: the mapping changes based on the focused field's current state.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::field::{FieldKind, FieldState};
use crate::form::{FormMessage, FormModel};

/// Map a key event to a form message, considering the focused field's state.
/// Returns `None` if the key event isn't relevant to the form.
pub fn map_key_event(key: KeyEvent, model: &FormModel) -> Option<FormMessage> {
    let field = model.focused_field()?;

    match &field.state {
        FieldState::TextEditing { .. } | FieldState::NumberEditing { .. } => map_editing_key(key),
        FieldState::SelectExpanded { .. } => map_select_expanded_key(key),
        FieldState::Idle => map_idle_key(key, &field.kind),
    }
}

fn map_editing_key(key: KeyEvent) -> Option<FormMessage> {
    match (key.code, key.modifiers) {
        (KeyCode::Enter, _) => Some(FormMessage::CommitEdit),
        (KeyCode::Esc, _) => Some(FormMessage::CancelEdit),
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            Some(FormMessage::EditChar(c))
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => Some(FormMessage::EditBackspace),
        (KeyCode::Delete, KeyModifiers::NONE) => Some(FormMessage::DeleteForward),
        (KeyCode::Left, KeyModifiers::NONE) => Some(FormMessage::CursorLeft),
        (KeyCode::Right, KeyModifiers::NONE) => Some(FormMessage::CursorRight),
        (KeyCode::Home, _) => Some(FormMessage::CursorHome),
        (KeyCode::End, _) => Some(FormMessage::CursorEnd),
        (KeyCode::Left, KeyModifiers::CONTROL) => Some(FormMessage::CursorWordBack),
        (KeyCode::Right, KeyModifiers::CONTROL) => Some(FormMessage::CursorWordForward),
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => Some(FormMessage::DeleteWordBack),
        _ => None,
    }
}

fn map_select_expanded_key(key: KeyEvent) -> Option<FormMessage> {
    match (key.code, key.modifiers) {
        (KeyCode::Enter, _) => Some(FormMessage::SelectConfirm),
        (KeyCode::Esc, _) => Some(FormMessage::CancelEdit),
        (KeyCode::Up, _) => Some(FormMessage::SelectHighlightPrev),
        (KeyCode::Down, _) => Some(FormMessage::SelectHighlightNext),
        (KeyCode::Backspace, _) => Some(FormMessage::SelectFilterBackspace),
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            Some(FormMessage::SelectFilterChar(c))
        }
        _ => None,
    }
}

fn map_idle_key(key: KeyEvent, kind: &FieldKind) -> Option<FormMessage> {
    match (key.code, key.modifiers) {
        (KeyCode::Tab, KeyModifiers::NONE) | (KeyCode::Down, KeyModifiers::NONE) => {
            Some(FormMessage::FocusNext)
        }
        (KeyCode::BackTab, _) | (KeyCode::Up, KeyModifiers::NONE) => Some(FormMessage::FocusPrev),
        (KeyCode::Enter, _) => Some(FormMessage::StartEdit),
        (KeyCode::Char('r'), KeyModifiers::CONTROL) => Some(FormMessage::ResetDefault),
        // Confirm-specific
        (KeyCode::Char(' '), _) if matches!(kind, FieldKind::Confirm { .. }) => {
            Some(FormMessage::ToggleConfirm)
        }
        (KeyCode::Char('y'), _) if matches!(kind, FieldKind::Confirm { .. }) => {
            Some(FormMessage::ToggleConfirm)
        }
        (KeyCode::Char('n'), _) if matches!(kind, FieldKind::Confirm { .. }) => {
            Some(FormMessage::ToggleConfirm)
        }
        // Select/Number cycling
        (KeyCode::Left, _)
            if matches!(kind, FieldKind::Select { .. } | FieldKind::Number { .. }) =>
        {
            Some(FormMessage::CyclePrev)
        }
        (KeyCode::Right, _)
            if matches!(kind, FieldKind::Select { .. } | FieldKind::Number { .. }) =>
        {
            Some(FormMessage::CycleNext)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, number, select, text};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn ctrl_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::CONTROL)
    }

    #[test]
    fn test_idle_tab_focus_next() {
        let model = FormModel::new(vec![text("a").build()]);
        assert_eq!(
            map_key_event(key(KeyCode::Tab), &model),
            Some(FormMessage::FocusNext)
        );
    }

    #[test]
    fn test_idle_backtab_focus_prev() {
        let model = FormModel::new(vec![text("a").build()]);
        let k = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        assert_eq!(map_key_event(k, &model), Some(FormMessage::FocusPrev));
    }

    #[test]
    fn test_idle_enter_starts_edit() {
        let model = FormModel::new(vec![text("a").build()]);
        assert_eq!(
            map_key_event(key(KeyCode::Enter), &model),
            Some(FormMessage::StartEdit)
        );
    }

    #[test]
    fn test_editing_char() {
        let mut model = FormModel::new(vec![text("a").build()]);
        model.fields[0].state = FieldState::TextEditing {
            buffer: String::new(),
            cursor: 0,
        };
        let k = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(map_key_event(k, &model), Some(FormMessage::EditChar('x')));
    }

    #[test]
    fn test_editing_esc_cancels() {
        let mut model = FormModel::new(vec![text("a").build()]);
        model.fields[0].state = FieldState::TextEditing {
            buffer: String::new(),
            cursor: 0,
        };
        assert_eq!(
            map_key_event(key(KeyCode::Esc), &model),
            Some(FormMessage::CancelEdit)
        );
    }

    #[test]
    fn test_editing_ctrl_w_deletes_word() {
        let mut model = FormModel::new(vec![text("a").build()]);
        model.fields[0].state = FieldState::TextEditing {
            buffer: "hello world".to_string(),
            cursor: 11,
        };
        assert_eq!(
            map_key_event(ctrl_key(KeyCode::Char('w')), &model),
            Some(FormMessage::DeleteWordBack)
        );
    }

    #[test]
    fn test_idle_confirm_space_toggles() {
        let model = FormModel::new(vec![confirm("ok").build()]);
        assert_eq!(
            map_key_event(key(KeyCode::Char(' ')), &model),
            Some(FormMessage::ToggleConfirm)
        );
    }

    #[test]
    fn test_idle_select_arrows_cycle() {
        let model = FormModel::new(vec![select("fmt", &[("a", "A"), ("b", "B")]).build()]);
        assert_eq!(
            map_key_event(key(KeyCode::Right), &model),
            Some(FormMessage::CycleNext)
        );
        assert_eq!(
            map_key_event(key(KeyCode::Left), &model),
            Some(FormMessage::CyclePrev)
        );
    }

    #[test]
    fn test_idle_number_arrows_step() {
        let model = FormModel::new(vec![number("q").range(0.0, 100.0).value("50").build()]);
        assert_eq!(
            map_key_event(key(KeyCode::Right), &model),
            Some(FormMessage::CycleNext)
        );
    }

    #[test]
    fn test_idle_ctrl_r_resets_default() {
        let model = FormModel::new(vec![text("a").build()]);
        assert_eq!(
            map_key_event(ctrl_key(KeyCode::Char('r')), &model),
            Some(FormMessage::ResetDefault)
        );
    }

    #[test]
    fn test_select_expanded_enter_confirms() {
        let mut model = FormModel::new(vec![select("fmt", &[("a", "A")]).build()]);
        model.fields[0].state = FieldState::SelectExpanded {
            highlight: 0,
            filter: String::new(),
            filtered_indices: vec![0],
        };
        assert_eq!(
            map_key_event(key(KeyCode::Enter), &model),
            Some(FormMessage::SelectConfirm)
        );
    }

    #[test]
    fn test_unrecognized_key_returns_none() {
        let model = FormModel::new(vec![text("a").build()]);
        assert_eq!(map_key_event(key(KeyCode::F(1)), &model), None);
    }
}
