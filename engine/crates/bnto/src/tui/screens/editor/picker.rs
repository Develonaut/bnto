// Editor "add node" picker overlay — search-and-filter over all node types.

use bnto_core::metadata::{NodeTypeInfo, all_node_types};

use super::{EditorAction, EditorMessage, EditorScreenModel};

/// A node type entry in the picker overlay.
#[derive(Debug, Clone)]
pub struct PickerEntry {
    pub name: String,
    pub label: String,
    pub category: String,
}

/// Overlay state for the "add node" picker.
#[derive(Debug, Clone)]
pub struct PickerState {
    pub entries: Vec<PickerEntry>,
    pub filtered: Vec<usize>,
    pub cursor: usize,
    pub query: String,
}

impl PickerState {
    /// Build the picker from the full node type registry.
    pub(super) fn from_node_types(types: &[NodeTypeInfo]) -> Self {
        let entries: Vec<PickerEntry> = types
            .iter()
            .map(|t| PickerEntry {
                name: t.name.clone(),
                label: t.label.clone(),
                category: format!("{:?}", t.category).to_lowercase(),
            })
            .collect();
        let filtered = (0..entries.len()).collect();
        Self {
            entries,
            filtered,
            cursor: 0,
            query: String::new(),
        }
    }

    /// Rebuild the filtered list after a query change.
    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        self.filtered = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                q.is_empty()
                    || e.label.to_lowercase().contains(&q)
                    || e.name.to_lowercase().contains(&q)
                    || e.category.contains(&q)
            })
            .map(|(i, _)| i)
            .collect();
        // Clamp cursor to new bounds.
        if !self.filtered.is_empty() {
            self.cursor = self.cursor.min(self.filtered.len() - 1);
        } else {
            self.cursor = 0;
        }
    }

    /// Return the selected entry's node type name, if any.
    fn selected_type(&self) -> Option<&str> {
        let &idx = self.filtered.get(self.cursor)?;
        Some(&self.entries[idx].name)
    }

    /// Apply a query edit, then refilter.
    fn edit_query(&mut self, edit: impl FnOnce(&mut String)) {
        edit(&mut self.query);
        self.refilter();
    }

    /// Move the cursor one step down, wrapping to the top.
    fn cursor_down(&mut self) {
        if !self.filtered.is_empty() {
            self.cursor = (self.cursor + 1) % self.filtered.len();
        }
    }

    /// Move the cursor one step up, wrapping to the bottom.
    fn cursor_up(&mut self) {
        if !self.filtered.is_empty() {
            self.cursor = if self.cursor == 0 {
                self.filtered.len() - 1
            } else {
                self.cursor - 1
            };
        }
    }
}

/// Handle messages while the picker overlay is active.
///
/// Selecting, cancelling, or any non-picker message closes the overlay.
pub(super) fn update_picker(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    let Some(picker) = model.picker.as_mut() else {
        return (model, EditorAction::None);
    };
    match msg {
        EditorMessage::PickerInput(ch) => picker.edit_query(|q| q.push(ch)),
        EditorMessage::PickerBackspace => picker.edit_query(|q| {
            q.pop();
        }),
        EditorMessage::PickerClear => picker.edit_query(String::clear),
        EditorMessage::PickerCursorDown => picker.cursor_down(),
        EditorMessage::PickerCursorUp => picker.cursor_up(),
        EditorMessage::PickerSelect => add_selected_node(&mut model),
        _ => model.picker = None,
    }
    (model, EditorAction::None)
}

/// Add the highlighted node type to the recipe and close the picker.
fn add_selected_node(model: &mut EditorScreenModel) {
    let selected = model
        .picker
        .as_ref()
        .and_then(|p| p.selected_type())
        .map(|s| s.to_string());
    if let Some(type_name) = selected {
        let info = all_node_types().into_iter().find(|t| t.name == type_name);
        if let Some(info) = info {
            model.editor.add_node(&type_name, &info);
        }
    }
    model.picker = None;
}
