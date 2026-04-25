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
        FieldState::TextAreaEditing { .. } => map_text_area_editing_key(key),
        FieldState::SelectExpanded { .. } => map_select_expanded_key(key),
        FieldState::FilePathBrowsing { .. } => map_file_path_browsing_key(key),
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

fn map_text_area_editing_key(key: KeyEvent) -> Option<FormMessage> {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, _) => Some(FormMessage::CancelEdit),
        // Ctrl+D commits (Enter inserts newline)
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(FormMessage::CommitEdit),
        (KeyCode::Enter, _) => Some(FormMessage::TextAreaNewline),
        (KeyCode::Up, _) => Some(FormMessage::TextAreaCursorUp),
        (KeyCode::Down, _) => Some(FormMessage::TextAreaCursorDown),
        (KeyCode::Left, KeyModifiers::NONE) => Some(FormMessage::CursorLeft),
        (KeyCode::Right, KeyModifiers::NONE) => Some(FormMessage::CursorRight),
        (KeyCode::Home, _) => Some(FormMessage::CursorHome),
        (KeyCode::End, _) => Some(FormMessage::CursorEnd),
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            Some(FormMessage::EditChar(c))
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => Some(FormMessage::EditBackspace),
        (KeyCode::Delete, KeyModifiers::NONE) => Some(FormMessage::DeleteForward),
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

fn map_file_path_browsing_key(key: KeyEvent) -> Option<FormMessage> {
    match (key.code, key.modifiers) {
        // Navigation
        (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
            Some(FormMessage::FilePathCursorDown)
        }
        (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
            Some(FormMessage::FilePathCursorUp)
        }
        // Enter directory / confirm file
        (KeyCode::Enter, _) | (KeyCode::Right, _) | (KeyCode::Char('l'), KeyModifiers::NONE) => {
            Some(FormMessage::FilePathEnterDir)
        }
        // Parent directory
        (KeyCode::Backspace, _) | (KeyCode::Left, _) | (KeyCode::Char('h'), KeyModifiers::NONE) => {
            Some(FormMessage::FilePathParentDir)
        }
        // Confirm (pick file)
        (KeyCode::Char(' '), _) => Some(FormMessage::FilePathConfirm),
        // Cancel
        (KeyCode::Esc, _) => Some(FormMessage::FilePathCancel),
        // Toggle hidden files
        (KeyCode::Char('.'), _) => Some(FormMessage::FilePathToggleHidden),
        // Page navigation
        (KeyCode::PageDown, _) | (KeyCode::Char('J'), KeyModifiers::SHIFT) => {
            Some(FormMessage::FilePathPageDown)
        }
        (KeyCode::PageUp, _) | (KeyCode::Char('K'), KeyModifiers::SHIFT) => {
            Some(FormMessage::FilePathPageUp)
        }
        (KeyCode::Home, _) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
            Some(FormMessage::FilePathGoToTop)
        }
        (KeyCode::End, _) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
            Some(FormMessage::FilePathGoToBottom)
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

    // --- FilePathBrowsing tests ---

    fn file_path_browsing_model() -> FormModel {
        use crate::field::file_path;
        use crate::file_entry::NavHistory;
        use std::path::PathBuf;
        let mut model = FormModel::new(vec![file_path("f").build()]);
        model.fields[0].state = FieldState::FilePathBrowsing {
            current_dir: PathBuf::from("/tmp"),
            entries: vec![],
            cursor: 0,
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
        };
        model
    }

    #[test]
    fn test_file_path_j_cursor_down() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Char('j')), &model),
            Some(FormMessage::FilePathCursorDown)
        );
    }

    #[test]
    fn test_file_path_k_cursor_up() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Char('k')), &model),
            Some(FormMessage::FilePathCursorUp)
        );
    }

    #[test]
    fn test_file_path_enter_enters_dir() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Enter), &model),
            Some(FormMessage::FilePathEnterDir)
        );
    }

    #[test]
    fn test_file_path_esc_cancels() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Esc), &model),
            Some(FormMessage::FilePathCancel)
        );
    }

    #[test]
    fn test_file_path_dot_toggles_hidden() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Char('.')), &model),
            Some(FormMessage::FilePathToggleHidden)
        );
    }

    #[test]
    fn test_file_path_h_parent_dir() {
        let model = file_path_browsing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Char('h')), &model),
            Some(FormMessage::FilePathParentDir)
        );
    }

    // --- TextAreaEditing tests ---

    fn text_area_editing_model() -> FormModel {
        use crate::field::textarea;
        let mut model = FormModel::new(vec![textarea("notes").build()]);
        model.fields[0].state = FieldState::TextAreaEditing {
            buffer: "hello\nworld".to_string(),
            cursor: 0,
            line: 0,
            scroll_offset: 0,
        };
        model
    }

    #[test]
    fn test_textarea_enter_inserts_newline() {
        let model = text_area_editing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Enter), &model),
            Some(FormMessage::TextAreaNewline)
        );
    }

    #[test]
    fn test_textarea_ctrl_d_commits() {
        let model = text_area_editing_model();
        assert_eq!(
            map_key_event(ctrl_key(KeyCode::Char('d')), &model),
            Some(FormMessage::CommitEdit)
        );
    }

    #[test]
    fn test_textarea_esc_cancels() {
        let model = text_area_editing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Esc), &model),
            Some(FormMessage::CancelEdit)
        );
    }

    #[test]
    fn test_textarea_up_down_cursor() {
        let model = text_area_editing_model();
        assert_eq!(
            map_key_event(key(KeyCode::Up), &model),
            Some(FormMessage::TextAreaCursorUp)
        );
        assert_eq!(
            map_key_event(key(KeyCode::Down), &model),
            Some(FormMessage::TextAreaCursorDown)
        );
    }

    #[test]
    fn test_textarea_char_input() {
        let model = text_area_editing_model();
        let k = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(map_key_event(k, &model), Some(FormMessage::EditChar('x')));
    }
}
