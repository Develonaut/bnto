//! The keys the focused field takes while it's being edited, for a footer.
//!
//! Kept beside `keys.rs`, which maps those keys, and tested against it: every
//! key named here does something in that state.

use crate::field::FieldState;
use crate::form::FormModel;

/// A key and what it does, as a footer shows them.
pub type KeyHint = (&'static str, &'static str);

/// The keys the focused field takes in its editing state, or `None` when no
/// field is being edited (the screen's own keys apply then).
pub fn key_hints(model: &FormModel) -> Option<&'static [KeyHint]> {
    let hints: &[KeyHint] = match &model.focused_field()?.state {
        FieldState::Idle => return None,
        FieldState::TextEditing { .. } | FieldState::NumberEditing { .. } => {
            &[("Enter", "save"), ("Esc", "cancel")]
        }
        FieldState::TextAreaEditing { .. } => {
            &[("Ctrl+D", "save"), ("Enter", "new line"), ("Esc", "cancel")]
        }
        FieldState::SelectExpanded { .. } => {
            &[("↑↓", "choose"), ("Enter", "pick"), ("Esc", "cancel")]
        }
        FieldState::MultiSelectEditing { .. } => {
            &[("Space", "toggle"), ("Enter", "done"), ("Esc", "cancel")]
        }
        FieldState::FilePathBrowsing { .. } => &[
            ("↑↓", "navigate"),
            ("l", "open"),
            ("h", "parent"),
            ("Space", "pick"),
            ("Esc", "cancel"),
        ],
    };
    Some(hints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldBuilder;
    use crate::field::{file_path, multiselect, select, text, textarea};
    use crate::form::FormMessage;
    use crate::keys::map_key_event;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    /// The key event a hint names (the first, for "↑↓").
    fn event(hint: &str) -> KeyEvent {
        let (code, mods) = match hint {
            "Enter" => (KeyCode::Enter, KeyModifiers::NONE),
            "Esc" => (KeyCode::Esc, KeyModifiers::NONE),
            "Space" => (KeyCode::Char(' '), KeyModifiers::NONE),
            "Ctrl+D" => (KeyCode::Char('d'), KeyModifiers::CONTROL),
            "↑↓" => (KeyCode::Up, KeyModifiers::NONE),
            one => (
                KeyCode::Char(one.chars().next().unwrap()),
                KeyModifiers::NONE,
            ),
        };
        KeyEvent::new(code, mods)
    }

    /// A one-field form, with that field put into editing (Enter).
    fn editing(field: FieldBuilder) -> FormModel {
        let mut model = FormModel::new(vec![field.build()]);
        let start = map_key_event(event("Enter"), &model).expect("Enter starts editing");
        model = crate::form::update(model, start);
        model
    }

    const OPTIONS: &[(&str, &str)] = &[("a", "A"), ("b", "B")];

    #[test]
    fn an_idle_field_has_no_hints() {
        let model = FormModel::new(vec![text("Name").build()]);
        assert_eq!(key_hints(&model), None);
    }

    #[test]
    fn every_hinted_key_does_something_in_its_state() {
        let fields = [
            text("Name"),
            textarea("Notes"),
            select("Kind", OPTIONS),
            multiselect("Tags", OPTIONS),
            file_path("Path"),
        ];
        for field in fields {
            let model = editing(field);
            let hints = key_hints(&model).expect("an editing field has hints");
            for (key, _) in hints {
                let message: Option<FormMessage> = map_key_event(event(key), &model);
                let state = model.focused_field().map(|f| &f.state);
                assert!(message.is_some(), "{key} does nothing in {state:?}");
            }
        }
    }
}
