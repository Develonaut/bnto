//! The keys the focused field takes while it's being edited, for a footer.
//!
//! Kept beside `keys.rs`, which maps those keys, and tested against it: every
//! key named here does something in that state.

use crate::field::FieldState;
use crate::form::FormModel;

/// A key and what it does, as a footer shows them.
pub type KeyHint = (&'static str, &'static str);

// Each set lists its commit and cancel keys first: a narrow footer drops
// hints from the end, so those two outlast the rest.
const TEXT: &[KeyHint] = &[("Enter", "save"), ("Esc", "cancel")];
const TEXT_AREA: &[KeyHint] = &[("Ctrl+D", "save"), ("Esc", "cancel"), ("Enter", "new line")];
const SELECT: &[KeyHint] = &[("Enter", "pick"), ("Esc", "cancel"), ("↑↓", "choose")];
const MULTI_SELECT: &[KeyHint] = &[("Enter", "done"), ("Esc", "cancel"), ("Space", "toggle")];
const FILE_PATH: &[KeyHint] = &[
    ("Space", "pick"),
    ("Esc", "cancel"),
    ("l", "open"),
    ("h", "parent"),
    ("↑↓", "navigate"),
];

/// The keys the focused field takes in its editing state, or `None` when no
/// field is being edited (the screen's own keys apply then).
pub fn key_hints(model: &FormModel) -> Option<&'static [KeyHint]> {
    match &model.focused_field()?.state {
        FieldState::Idle => None,
        FieldState::TextEditing { .. } | FieldState::NumberEditing { .. } => Some(TEXT),
        FieldState::TextAreaEditing { .. } => Some(TEXT_AREA),
        FieldState::SelectExpanded { .. } => Some(SELECT),
        FieldState::MultiSelectEditing { .. } => Some(MULTI_SELECT),
        FieldState::FilePathBrowsing { .. } => Some(FILE_PATH),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldBuilder;
    use crate::field::{file_path, multiselect, number, select, text, textarea};
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
            number("Count"),
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
