// Editor screen — TUI list editor for creating and modifying recipes.
//
// Pure TEA state machine: EditorScreenModel + EditorMessage + update().
// All mutations delegate to EditorModel (bnto-core) for undo/redo support.
// The picker overlay reuses the Browser's search-and-filter pattern.

use bnto_core::editor::EditorModel;
use bnto_core::metadata::{NodeTypeInfo, all_node_types};

/// TUI-specific messages for the editor screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMessage {
    // --- Node list navigation ---
    CursorDown,
    CursorUp,
    ExpandToggle,

    // --- Node operations ---
    MoveUp,
    MoveDown,
    DeleteRequest,
    DeleteConfirm,
    DeleteCancel,

    // --- Undo/redo ---
    Undo,
    Redo,

    // --- Picker overlay ---
    OpenPicker,
    PickerInput(char),
    PickerBackspace,
    PickerClear,
    PickerSelect,
    PickerCancel,
    PickerCursorDown,
    PickerCursorUp,
}

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

/// The editor screen's TUI state — wraps the core EditorModel with
/// screen-specific UI state (picker overlay, delete confirmation).
#[derive(Debug, Clone)]
pub struct EditorScreenModel {
    pub editor: EditorModel,
    pub picker: Option<PickerState>,
    pub confirming_delete: bool,
}

impl EditorScreenModel {
    /// Create an editor screen from an existing EditorModel.
    #[allow(dead_code)] // Used in tests; production entry point pending detail→editor wiring
    pub fn new(editor: EditorModel) -> Self {
        Self {
            editor,
            picker: None,
            confirming_delete: false,
        }
    }
}

impl PickerState {
    /// Build the picker from the full node type registry.
    fn from_node_types(types: &[NodeTypeInfo]) -> Self {
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
}

/// Result from the editor update — either stay on editor or navigate away.
#[derive(Debug, Clone, PartialEq)]
pub enum EditorAction {
    /// Stay on editor, no app-level side effect.
    None,
    /// User pressed Back with a clean state — navigate away.
    #[allow(dead_code)] // Used in tests; production entry point pending detail→editor wiring
    Back,
    /// User pressed Back with dirty state — show confirm prompt.
    #[allow(dead_code)] // Used in tests; production entry point pending detail→editor wiring
    ConfirmDirty,
}

/// Pure state transition for the editor screen.
pub fn update(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    // If a picker is open, route messages there first.
    if model.picker.is_some() {
        return update_picker(model, msg);
    }

    // Delete confirmation captures specific keys.
    if model.confirming_delete {
        return update_delete_confirm(model, msg);
    }

    let action = match msg {
        EditorMessage::CursorDown => {
            if !model.editor.nodes.is_empty() {
                let current = model.editor.selected_index.unwrap_or(0);
                let next = if current + 1 >= model.editor.nodes.len() {
                    0
                } else {
                    current + 1
                };
                model.editor.selected_index = Some(next);
            }
            EditorAction::None
        }
        EditorMessage::CursorUp => {
            if !model.editor.nodes.is_empty() {
                let current = model.editor.selected_index.unwrap_or(0);
                let prev = if current == 0 {
                    model.editor.nodes.len() - 1
                } else {
                    current - 1
                };
                model.editor.selected_index = Some(prev);
            }
            EditorAction::None
        }
        EditorMessage::ExpandToggle => {
            if let Some(idx) = model.editor.selected_index
                && idx < model.editor.nodes.len()
            {
                model.editor.nodes[idx].expanded = !model.editor.nodes[idx].expanded;
            }
            EditorAction::None
        }
        EditorMessage::MoveDown => {
            if let Some(idx) = model.editor.selected_index
                && idx + 1 < model.editor.nodes.len()
            {
                model.editor.reorder(idx, idx + 1);
            }
            EditorAction::None
        }
        EditorMessage::MoveUp => {
            if let Some(idx) = model.editor.selected_index
                && idx > 0
            {
                model.editor.reorder(idx, idx - 1);
            }
            EditorAction::None
        }
        EditorMessage::DeleteRequest => {
            if model.editor.selected_index.is_some() && !model.editor.nodes.is_empty() {
                model.confirming_delete = true;
            }
            EditorAction::None
        }
        EditorMessage::Undo => {
            model.editor.undo();
            EditorAction::None
        }
        EditorMessage::Redo => {
            model.editor.redo();
            EditorAction::None
        }
        EditorMessage::OpenPicker => {
            let types = all_node_types();
            model.picker = Some(PickerState::from_node_types(&types));
            EditorAction::None
        }
        // Picker/delete messages when no overlay is active — no-op.
        EditorMessage::PickerInput(_)
        | EditorMessage::PickerBackspace
        | EditorMessage::PickerClear
        | EditorMessage::PickerSelect
        | EditorMessage::PickerCancel
        | EditorMessage::PickerCursorDown
        | EditorMessage::PickerCursorUp
        | EditorMessage::DeleteConfirm
        | EditorMessage::DeleteCancel => EditorAction::None,
    };

    (model, action)
}

/// Handle messages while the picker overlay is active.
fn update_picker(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    match msg {
        EditorMessage::PickerInput(ch) => {
            if let Some(picker) = &mut model.picker {
                picker.query.push(ch);
                picker.refilter();
            }
        }
        EditorMessage::PickerBackspace => {
            if let Some(picker) = &mut model.picker {
                picker.query.pop();
                picker.refilter();
            }
        }
        EditorMessage::PickerClear => {
            if let Some(picker) = &mut model.picker {
                picker.query.clear();
                picker.refilter();
            }
        }
        EditorMessage::PickerCursorDown => {
            if let Some(picker) = &mut model.picker
                && !picker.filtered.is_empty()
            {
                picker.cursor = (picker.cursor + 1) % picker.filtered.len();
            }
        }
        EditorMessage::PickerCursorUp => {
            if let Some(picker) = &mut model.picker
                && !picker.filtered.is_empty()
            {
                picker.cursor = if picker.cursor == 0 {
                    picker.filtered.len() - 1
                } else {
                    picker.cursor - 1
                };
            }
        }
        EditorMessage::PickerSelect => {
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
        EditorMessage::PickerCancel => {
            model.picker = None;
        }
        // Non-picker messages close the picker and are ignored.
        _ => {
            model.picker = None;
        }
    }
    (model, EditorAction::None)
}

/// Handle messages during delete confirmation.
fn update_delete_confirm(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    match msg {
        EditorMessage::DeleteConfirm => {
            if let Some(idx) = model.editor.selected_index {
                model.editor.remove_node(idx);
            }
            model.confirming_delete = false;
        }
        EditorMessage::DeleteCancel => {
            model.confirming_delete = false;
        }
        // Any other message cancels the confirmation.
        _ => {
            model.confirming_delete = false;
        }
    }
    (model, EditorAction::None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::NodeCategory;

    fn test_info(name: &str) -> NodeTypeInfo {
        NodeTypeInfo {
            name: name.to_string(),
            label: name.to_string(),
            description: String::new(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "image".to_string(),
        }
    }

    fn model_with_nodes(count: usize) -> EditorScreenModel {
        let mut editor = EditorModel::new();
        for i in 0..count {
            let name = format!("node-{i}");
            editor.add_node(&name, &test_info(&name));
        }
        // Clear undo stack and dirty flag from setup.
        editor.undo_stack.clear();
        editor.redo_stack.clear();
        editor.dirty = false;
        editor.selected_index = Some(0);
        EditorScreenModel::new(editor)
    }

    // --- Navigation ---

    #[test]
    fn cursor_down_moves_to_next() {
        let model = model_with_nodes(3);
        let (model, _) = update(model, EditorMessage::CursorDown);
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn cursor_up_moves_to_prev() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(1);
        let (model, _) = update(model, EditorMessage::CursorUp);
        assert_eq!(model.editor.selected_index, Some(0));
    }

    #[test]
    fn cursor_wraps_at_bottom() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(2);
        let (model, _) = update(model, EditorMessage::CursorDown);
        assert_eq!(model.editor.selected_index, Some(0));
    }

    #[test]
    fn cursor_wraps_at_top() {
        let model = model_with_nodes(3);
        let (model, _) = update(model, EditorMessage::CursorUp);
        assert_eq!(model.editor.selected_index, Some(2));
    }

    #[test]
    fn cursor_noop_when_empty() {
        let model = EditorScreenModel::new(EditorModel::new());
        let (model, _) = update(model, EditorMessage::CursorDown);
        assert_eq!(model.editor.selected_index, None);
    }

    // --- Expand/collapse ---

    #[test]
    fn expand_toggle_opens_node() {
        let model = model_with_nodes(2);
        assert!(!model.editor.nodes[0].expanded);
        let (model, _) = update(model, EditorMessage::ExpandToggle);
        assert!(model.editor.nodes[0].expanded);
    }

    #[test]
    fn expand_toggle_closes_node() {
        let mut model = model_with_nodes(2);
        model.editor.nodes[0].expanded = true;
        let (model, _) = update(model, EditorMessage::ExpandToggle);
        assert!(!model.editor.nodes[0].expanded);
    }

    // --- Reorder ---

    #[test]
    fn move_down_swaps_nodes() {
        let model = model_with_nodes(3);
        let original_type = model.editor.nodes[0].node_type.clone();
        let (model, _) = update(model, EditorMessage::MoveDown);
        assert_eq!(model.editor.nodes[1].node_type, original_type);
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn move_up_swaps_nodes() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(1);
        let original_type = model.editor.nodes[1].node_type.clone();
        let (model, _) = update(model, EditorMessage::MoveUp);
        assert_eq!(model.editor.nodes[0].node_type, original_type);
        assert_eq!(model.editor.selected_index, Some(0));
    }

    #[test]
    fn move_at_top_is_noop() {
        let model = model_with_nodes(3);
        let types_before: Vec<_> = model
            .editor
            .nodes
            .iter()
            .map(|n| n.node_type.clone())
            .collect();
        let (model, _) = update(model, EditorMessage::MoveUp);
        let types_after: Vec<_> = model
            .editor
            .nodes
            .iter()
            .map(|n| n.node_type.clone())
            .collect();
        assert_eq!(types_before, types_after);
    }

    #[test]
    fn move_at_bottom_is_noop() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(2);
        let types_before: Vec<_> = model
            .editor
            .nodes
            .iter()
            .map(|n| n.node_type.clone())
            .collect();
        let (model, _) = update(model, EditorMessage::MoveDown);
        let types_after: Vec<_> = model
            .editor
            .nodes
            .iter()
            .map(|n| n.node_type.clone())
            .collect();
        assert_eq!(types_before, types_after);
    }

    #[test]
    fn reorder_cursor_follows_node() {
        let model = model_with_nodes(3);
        assert_eq!(model.editor.selected_index, Some(0));
        let (model, _) = update(model, EditorMessage::MoveDown);
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn reorder_triggers_undo_snapshot() {
        let model = model_with_nodes(3);
        assert!(model.editor.undo_stack.is_empty());
        let (model, _) = update(model, EditorMessage::MoveDown);
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    // --- Delete ---

    #[test]
    fn delete_request_shows_confirmation() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest);
        assert!(model.confirming_delete);
    }

    #[test]
    fn confirm_delete_removes_node() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest);
        let (model, _) = update(model, EditorMessage::DeleteConfirm);
        assert_eq!(model.editor.nodes.len(), 1);
        assert!(!model.confirming_delete);
    }

    #[test]
    fn cancel_delete_preserves_node() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest);
        let (model, _) = update(model, EditorMessage::DeleteCancel);
        assert_eq!(model.editor.nodes.len(), 2);
        assert!(!model.confirming_delete);
    }

    #[test]
    fn delete_triggers_undo_snapshot() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest);
        let (model, _) = update(model, EditorMessage::DeleteConfirm);
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    #[test]
    fn delete_on_empty_is_noop() {
        let model = EditorScreenModel::new(EditorModel::new());
        let (model, _) = update(model, EditorMessage::DeleteRequest);
        assert!(!model.confirming_delete);
    }

    // --- Picker overlay ---

    #[test]
    fn open_picker_enters_picker_mode() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        assert!(model.picker.is_some());
        let picker = model.picker.unwrap();
        assert!(!picker.entries.is_empty());
        assert_eq!(picker.filtered.len(), picker.entries.len());
    }

    #[test]
    fn picker_shows_all_node_types() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        let picker = model.picker.unwrap();
        let expected = all_node_types().len();
        assert_eq!(picker.entries.len(), expected);
    }

    #[test]
    fn picker_search_filters_by_name() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        let total = model.picker.as_ref().unwrap().entries.len();
        // Type "image" to filter.
        let (model, _) = update(model, EditorMessage::PickerInput('i'));
        let (model, _) = update(model, EditorMessage::PickerInput('m'));
        let (model, _) = update(model, EditorMessage::PickerInput('a'));
        let (model, _) = update(model, EditorMessage::PickerInput('g'));
        let (model, _) = update(model, EditorMessage::PickerInput('e'));
        let filtered = model.picker.as_ref().unwrap().filtered.len();
        assert!(filtered < total, "filtering should reduce results");
        assert!(filtered > 0, "should have image matches");
    }

    #[test]
    fn picker_select_adds_node() {
        let model = model_with_nodes(1);
        assert_eq!(model.editor.nodes.len(), 1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        let (model, _) = update(model, EditorMessage::PickerSelect);
        assert_eq!(model.editor.nodes.len(), 2);
        assert!(model.picker.is_none());
    }

    #[test]
    fn picker_cancel_closes_overlay() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        assert!(model.picker.is_some());
        let (model, _) = update(model, EditorMessage::PickerCancel);
        assert!(model.picker.is_none());
        assert_eq!(model.editor.nodes.len(), 1);
    }

    #[test]
    fn add_triggers_undo_snapshot() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        let (model, _) = update(model, EditorMessage::PickerSelect);
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    #[test]
    fn picker_cursor_wraps() {
        let model = model_with_nodes(0);
        let (model, _) = update(model, EditorMessage::OpenPicker);
        let total = model.picker.as_ref().unwrap().filtered.len();
        // Go up from 0 should wrap to last.
        let (model, _) = update(model, EditorMessage::PickerCursorUp);
        assert_eq!(model.picker.as_ref().unwrap().cursor, total - 1);
        // Go down from last should wrap to 0.
        let (model, _) = update(model, EditorMessage::PickerCursorDown);
        assert_eq!(model.picker.as_ref().unwrap().cursor, 0);
    }

    // --- Undo/redo ---

    #[test]
    fn undo_restores_previous_state() {
        let model = model_with_nodes(2);
        // Move a node (creates undo snapshot).
        let (model, _) = update(model, EditorMessage::MoveDown);
        assert_eq!(model.editor.selected_index, Some(1));
        // Undo should restore original order.
        let (model, _) = update(model, EditorMessage::Undo);
        assert_eq!(model.editor.nodes[0].node_type, "node-0");
    }

    #[test]
    fn redo_reapplies_undone_state() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::MoveDown);
        let (model, _) = update(model, EditorMessage::Undo);
        assert_eq!(model.editor.nodes[0].node_type, "node-0");
        let (model, _) = update(model, EditorMessage::Redo);
        assert_eq!(model.editor.nodes[0].node_type, "node-1");
    }
}
