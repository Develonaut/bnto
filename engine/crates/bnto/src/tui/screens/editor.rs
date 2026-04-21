// Editor screen — TUI list editor for creating and modifying recipes.
//
// Pure TEA state machine: EditorScreenModel + EditorMessage + update().
// All mutations delegate to EditorModel (bnto-core) for undo/redo support.
// The picker overlay reuses the Browser's search-and-filter pattern.

use bnto_core::editor::EditorModel;
use bnto_core::metadata::{NodeTypeInfo, all_node_types};
use bnto_core::registry::NodeRegistry;

use super::editor_bridge::{self, ActiveNodeForm};

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

    // --- Save + exit ---
    Save,
    Back,
    DirtySave,
    DirtyDiscard,
    DirtyCancel,

    // --- Inline form editing ---
    Form(bnto_form::FormMessage),

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
/// screen-specific UI state (picker overlay, delete confirmation, inline form).
#[derive(Debug, Clone)]
pub struct EditorScreenModel {
    pub editor: EditorModel,
    pub picker: Option<PickerState>,
    pub confirming_delete: bool,
    /// Whether the dirty-exit confirmation overlay is showing.
    pub confirming_dirty_exit: bool,
    /// Active inline form for the currently expanded+selected node.
    pub active_form: Option<ActiveNodeForm>,
}

impl EditorScreenModel {
    /// Create an editor screen from an existing EditorModel.
    pub fn new(editor: EditorModel) -> Self {
        Self {
            editor,
            picker: None,
            confirming_delete: false,
            confirming_dirty_exit: false,
            active_form: None,
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
    /// Navigate away from editor (clean exit or discard).
    Back,
    /// Save the recipe, then navigate away.
    SaveAndBack,
    /// Save the recipe, stay on editor.
    Save,
}

/// Pure state transition for the editor screen.
pub fn update(
    mut model: EditorScreenModel,
    msg: EditorMessage,
    registry: &NodeRegistry,
) -> (EditorScreenModel, EditorAction) {
    // If a picker is open, route messages there first.
    if model.picker.is_some() {
        return update_picker(model, msg);
    }

    // Delete confirmation captures specific keys.
    if model.confirming_delete {
        return update_delete_confirm(model, msg);
    }

    // Dirty-exit confirmation captures specific keys.
    if model.confirming_dirty_exit {
        return update_dirty_confirm(model, msg);
    }

    // Form messages delegate to the active form.
    if let EditorMessage::Form(form_msg) = msg {
        return update_form(model, form_msg, registry);
    }

    let action = match msg {
        EditorMessage::CursorDown => {
            sync_and_clear_form(&mut model);
            if !model.editor.nodes.is_empty() {
                let current = model.editor.selected_index.unwrap_or(0);
                let next = if current + 1 >= model.editor.nodes.len() {
                    0
                } else {
                    current + 1
                };
                model.editor.selected_index = Some(next);
            }
            rebuild_form_if_expanded(&mut model, registry);
            EditorAction::None
        }
        EditorMessage::CursorUp => {
            sync_and_clear_form(&mut model);
            if !model.editor.nodes.is_empty() {
                let current = model.editor.selected_index.unwrap_or(0);
                let prev = if current == 0 {
                    model.editor.nodes.len() - 1
                } else {
                    current - 1
                };
                model.editor.selected_index = Some(prev);
            }
            rebuild_form_if_expanded(&mut model, registry);
            EditorAction::None
        }
        EditorMessage::ExpandToggle => {
            if let Some(idx) = model.editor.selected_index
                && idx < model.editor.nodes.len()
            {
                let was_expanded = model.editor.nodes[idx].expanded;
                model.editor.nodes[idx].expanded = !was_expanded;
                if was_expanded {
                    // Collapsing: sync any pending edits and clear form.
                    sync_and_clear_form(&mut model);
                } else {
                    // Expanding: build form for the node.
                    model.active_form =
                        editor_bridge::build_form(&model.editor.nodes[idx], idx, registry);
                }
            }
            EditorAction::None
        }
        EditorMessage::MoveDown => {
            sync_and_clear_form(&mut model);
            if let Some(idx) = model.editor.selected_index
                && idx + 1 < model.editor.nodes.len()
            {
                model.editor.reorder(idx, idx + 1);
            }
            EditorAction::None
        }
        EditorMessage::MoveUp => {
            sync_and_clear_form(&mut model);
            if let Some(idx) = model.editor.selected_index
                && idx > 0
            {
                model.editor.reorder(idx, idx - 1);
            }
            EditorAction::None
        }
        EditorMessage::DeleteRequest => {
            sync_and_clear_form(&mut model);
            if model.editor.selected_index.is_some() && !model.editor.nodes.is_empty() {
                model.confirming_delete = true;
            }
            EditorAction::None
        }
        EditorMessage::Undo => {
            model.active_form = None;
            model.editor.undo();
            rebuild_form_if_expanded(&mut model, registry);
            EditorAction::None
        }
        EditorMessage::Redo => {
            model.active_form = None;
            model.editor.redo();
            rebuild_form_if_expanded(&mut model, registry);
            EditorAction::None
        }
        EditorMessage::OpenPicker => {
            sync_and_clear_form(&mut model);
            let types = all_node_types();
            model.picker = Some(PickerState::from_node_types(&types));
            EditorAction::None
        }
        EditorMessage::Save => {
            sync_and_clear_form(&mut model);
            EditorAction::Save
        }
        EditorMessage::Back => {
            sync_and_clear_form(&mut model);
            if model.editor.dirty {
                model.confirming_dirty_exit = true;
                EditorAction::None
            } else {
                EditorAction::Back
            }
        }
        // Dirty-confirm messages when no overlay is active — no-op.
        EditorMessage::DirtySave | EditorMessage::DirtyDiscard | EditorMessage::DirtyCancel => {
            EditorAction::None
        }
        // Form/picker/delete messages when no overlay is active — no-op.
        EditorMessage::Form(_)
        | EditorMessage::PickerInput(_)
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

/// Handle a form message for the active inline form.
fn update_form(
    mut model: EditorScreenModel,
    msg: bnto_form::FormMessage,
    registry: &NodeRegistry,
) -> (EditorScreenModel, EditorAction) {
    if let Some(ref mut active) = model.active_form {
        let was_editing = editor_bridge::is_editing(active);
        editor_bridge::apply_form_message(active, msg);
        let is_editing_now = editor_bridge::is_editing(active);

        // On commit (was editing → now idle), sync values to EditorModel.
        if was_editing && !is_editing_now {
            let idx = active.node_index;
            let updates = editor_bridge::collect_param_updates(active);
            for (key, value) in updates {
                model.editor.update_param(idx, &key, value);
            }
            // Rebuild form from updated EditorModel to keep in sync.
            if let Some(node) = model.editor.nodes.get(idx) {
                model.active_form = editor_bridge::build_form(node, idx, registry);
            }
        }
    }
    (model, EditorAction::None)
}

/// Sync pending form edits to EditorModel and clear the active form.
fn sync_and_clear_form(model: &mut EditorScreenModel) {
    if let Some(active) = model.active_form.take() {
        let idx = active.node_index;
        let updates = editor_bridge::collect_param_updates(&active);
        // Only push undo if values actually changed.
        let node = &model.editor.nodes[idx];
        let changed = updates.iter().any(|(k, v)| node.params.get(k) != Some(v));
        if changed {
            for (key, value) in updates {
                model.editor.update_param(idx, &key, value);
            }
        }
    }
}

/// Rebuild the active form if the selected node is expanded.
fn rebuild_form_if_expanded(model: &mut EditorScreenModel, registry: &NodeRegistry) {
    if let Some(idx) = model.editor.selected_index
        && let Some(node) = model.editor.nodes.get(idx)
        && node.expanded
    {
        model.active_form = editor_bridge::build_form(node, idx, registry);
        return;
    }
    model.active_form = None;
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

/// Handle messages during dirty-exit confirmation ("Save? y/n/Esc").
fn update_dirty_confirm(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    match msg {
        EditorMessage::DirtySave => {
            model.confirming_dirty_exit = false;
            (model, EditorAction::SaveAndBack)
        }
        EditorMessage::DirtyDiscard => {
            model.confirming_dirty_exit = false;
            (model, EditorAction::Back)
        }
        EditorMessage::DirtyCancel => {
            model.confirming_dirty_exit = false;
            (model, EditorAction::None)
        }
        // Any other message cancels the confirmation.
        _ => {
            model.confirming_dirty_exit = false;
            (model, EditorAction::None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::NodeCategory;
    use bnto_engine::create_registry;

    fn reg() -> NodeRegistry {
        create_registry()
    }

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

    fn model_with_real_node() -> EditorScreenModel {
        let mut editor = EditorModel::new();
        let info = NodeTypeInfo {
            name: "image-compress".into(),
            label: "Compress Image".into(),
            description: String::new(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".into()],
            icon: "image".into(),
        };
        editor.add_node_with_defaults(
            "image-compress",
            &info,
            &[("quality".into(), serde_json::json!(80))],
        );
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
        let (model, _) = update(model, EditorMessage::CursorDown, &reg());
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn cursor_up_moves_to_prev() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(1);
        let (model, _) = update(model, EditorMessage::CursorUp, &reg());
        assert_eq!(model.editor.selected_index, Some(0));
    }

    #[test]
    fn cursor_wraps_at_bottom() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(2);
        let (model, _) = update(model, EditorMessage::CursorDown, &reg());
        assert_eq!(model.editor.selected_index, Some(0));
    }

    #[test]
    fn cursor_wraps_at_top() {
        let model = model_with_nodes(3);
        let (model, _) = update(model, EditorMessage::CursorUp, &reg());
        assert_eq!(model.editor.selected_index, Some(2));
    }

    #[test]
    fn cursor_noop_when_empty() {
        let model = EditorScreenModel::new(EditorModel::new());
        let (model, _) = update(model, EditorMessage::CursorDown, &reg());
        assert_eq!(model.editor.selected_index, None);
    }

    // --- Expand/collapse ---

    #[test]
    fn expand_toggle_opens_node() {
        let model = model_with_nodes(2);
        assert!(!model.editor.nodes[0].expanded);
        let (model, _) = update(model, EditorMessage::ExpandToggle, &reg());
        assert!(model.editor.nodes[0].expanded);
    }

    #[test]
    fn expand_toggle_closes_node() {
        let mut model = model_with_nodes(2);
        model.editor.nodes[0].expanded = true;
        let (model, _) = update(model, EditorMessage::ExpandToggle, &reg());
        assert!(!model.editor.nodes[0].expanded);
    }

    // --- Reorder ---

    #[test]
    fn move_down_swaps_nodes() {
        let model = model_with_nodes(3);
        let original_type = model.editor.nodes[0].node_type.clone();
        let (model, _) = update(model, EditorMessage::MoveDown, &reg());
        assert_eq!(model.editor.nodes[1].node_type, original_type);
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn move_up_swaps_nodes() {
        let mut model = model_with_nodes(3);
        model.editor.selected_index = Some(1);
        let original_type = model.editor.nodes[1].node_type.clone();
        let (model, _) = update(model, EditorMessage::MoveUp, &reg());
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
        let (model, _) = update(model, EditorMessage::MoveUp, &reg());
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
        let (model, _) = update(model, EditorMessage::MoveDown, &reg());
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
        let (model, _) = update(model, EditorMessage::MoveDown, &reg());
        assert_eq!(model.editor.selected_index, Some(1));
    }

    #[test]
    fn reorder_triggers_undo_snapshot() {
        let model = model_with_nodes(3);
        assert!(model.editor.undo_stack.is_empty());
        let (model, _) = update(model, EditorMessage::MoveDown, &reg());
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    // --- Delete ---

    #[test]
    fn delete_request_shows_confirmation() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest, &reg());
        assert!(model.confirming_delete);
    }

    #[test]
    fn confirm_delete_removes_node() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest, &reg());
        let (model, _) = update(model, EditorMessage::DeleteConfirm, &reg());
        assert_eq!(model.editor.nodes.len(), 1);
        assert!(!model.confirming_delete);
    }

    #[test]
    fn cancel_delete_preserves_node() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest, &reg());
        let (model, _) = update(model, EditorMessage::DeleteCancel, &reg());
        assert_eq!(model.editor.nodes.len(), 2);
        assert!(!model.confirming_delete);
    }

    #[test]
    fn delete_triggers_undo_snapshot() {
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::DeleteRequest, &reg());
        let (model, _) = update(model, EditorMessage::DeleteConfirm, &reg());
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    #[test]
    fn delete_on_empty_is_noop() {
        let model = EditorScreenModel::new(EditorModel::new());
        let (model, _) = update(model, EditorMessage::DeleteRequest, &reg());
        assert!(!model.confirming_delete);
    }

    // --- Picker overlay ---

    #[test]
    fn open_picker_enters_picker_mode() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &reg());
        assert!(model.picker.is_some());
        let picker = model.picker.unwrap();
        assert!(!picker.entries.is_empty());
        assert_eq!(picker.filtered.len(), picker.entries.len());
    }

    #[test]
    fn picker_shows_all_node_types() {
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &reg());
        let picker = model.picker.unwrap();
        let expected = all_node_types().len();
        assert_eq!(picker.entries.len(), expected);
    }

    #[test]
    fn picker_search_filters_by_name() {
        let r = reg();
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &r);
        let total = model.picker.as_ref().unwrap().entries.len();
        let (model, _) = update(model, EditorMessage::PickerInput('i'), &r);
        let (model, _) = update(model, EditorMessage::PickerInput('m'), &r);
        let (model, _) = update(model, EditorMessage::PickerInput('a'), &r);
        let (model, _) = update(model, EditorMessage::PickerInput('g'), &r);
        let (model, _) = update(model, EditorMessage::PickerInput('e'), &r);
        let filtered = model.picker.as_ref().unwrap().filtered.len();
        assert!(filtered < total, "filtering should reduce results");
        assert!(filtered > 0, "should have image matches");
    }

    #[test]
    fn picker_select_adds_node() {
        let r = reg();
        let model = model_with_nodes(1);
        assert_eq!(model.editor.nodes.len(), 1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &r);
        let (model, _) = update(model, EditorMessage::PickerSelect, &r);
        assert_eq!(model.editor.nodes.len(), 2);
        assert!(model.picker.is_none());
    }

    #[test]
    fn picker_cancel_closes_overlay() {
        let r = reg();
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &r);
        assert!(model.picker.is_some());
        let (model, _) = update(model, EditorMessage::PickerCancel, &r);
        assert!(model.picker.is_none());
        assert_eq!(model.editor.nodes.len(), 1);
    }

    #[test]
    fn add_triggers_undo_snapshot() {
        let r = reg();
        let model = model_with_nodes(1);
        let (model, _) = update(model, EditorMessage::OpenPicker, &r);
        let (model, _) = update(model, EditorMessage::PickerSelect, &r);
        assert_eq!(model.editor.undo_stack.len(), 1);
    }

    #[test]
    fn picker_cursor_wraps() {
        let r = reg();
        let model = model_with_nodes(0);
        let (model, _) = update(model, EditorMessage::OpenPicker, &r);
        let total = model.picker.as_ref().unwrap().filtered.len();
        let (model, _) = update(model, EditorMessage::PickerCursorUp, &r);
        assert_eq!(model.picker.as_ref().unwrap().cursor, total - 1);
        let (model, _) = update(model, EditorMessage::PickerCursorDown, &r);
        assert_eq!(model.picker.as_ref().unwrap().cursor, 0);
    }

    // --- Undo/redo ---

    #[test]
    fn undo_restores_previous_state() {
        let r = reg();
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::MoveDown, &r);
        assert_eq!(model.editor.selected_index, Some(1));
        let (model, _) = update(model, EditorMessage::Undo, &r);
        assert_eq!(model.editor.nodes[0].node_type, "node-0");
    }

    #[test]
    fn redo_reapplies_undone_state() {
        let r = reg();
        let model = model_with_nodes(2);
        let (model, _) = update(model, EditorMessage::MoveDown, &r);
        let (model, _) = update(model, EditorMessage::Undo, &r);
        assert_eq!(model.editor.nodes[0].node_type, "node-0");
        let (model, _) = update(model, EditorMessage::Redo, &r);
        assert_eq!(model.editor.nodes[0].node_type, "node-1");
    }

    // --- Inline form editing ---

    #[test]
    fn expand_builds_form_for_processor_node() {
        let r = reg();
        let model = model_with_real_node();
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.editor.nodes[0].expanded);
        assert!(
            model.active_form.is_some(),
            "expanding a processor node should build a form"
        );
        let active = model.active_form.unwrap();
        assert_eq!(active.node_index, 0);
        assert!(!active.params.is_empty());
    }

    #[test]
    fn expand_no_form_for_unknown_node() {
        let r = reg();
        let model = model_with_nodes(1); // "node-0" is unknown type
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.editor.nodes[0].expanded);
        assert!(
            model.active_form.is_none(),
            "unknown node type should have no form"
        );
    }

    #[test]
    fn collapse_clears_form() {
        let r = reg();
        let model = model_with_real_node();
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.active_form.is_some());
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(!model.editor.nodes[0].expanded);
        assert!(model.active_form.is_none());
    }

    #[test]
    fn cursor_move_clears_and_rebuilds_form() {
        let r = reg();
        let mut model = model_with_real_node();
        // Add a second real node.
        let info = NodeTypeInfo {
            name: "image-compress".into(),
            label: "Compress Image 2".into(),
            description: String::new(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".into()],
            icon: "image".into(),
        };
        model.editor.add_node_with_defaults(
            "image-compress",
            &info,
            &[("quality".into(), serde_json::json!(90))],
        );
        model.editor.nodes[1].expanded = true;
        model.editor.undo_stack.clear();
        model.editor.dirty = false;
        // Reset selection to first node.
        model.editor.selected_index = Some(0);

        // Expand first node and verify form.
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.active_form.is_some());
        assert_eq!(model.active_form.as_ref().unwrap().node_index, 0);

        // Move to second node — form should switch.
        let (model, _) = update(model, EditorMessage::CursorDown, &r);
        assert!(model.active_form.is_some());
        assert_eq!(model.active_form.as_ref().unwrap().node_index, 1);
    }

    #[test]
    fn form_edit_commit_updates_editor_model() {
        let r = reg();
        let model = model_with_real_node();
        // Expand node to get form.
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.active_form.is_some());

        // Start editing, change value, commit.
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::StartEdit),
            &r,
        );
        // Type new value.
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditChar('5')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditChar('0')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::CommitEdit),
            &r,
        );

        // EditorModel should have the updated value.
        let quality = model.editor.nodes[0].params.get("quality");
        assert!(quality.is_some(), "quality param should exist");
        assert!(model.editor.dirty, "should be dirty after edit");
    }

    #[test]
    fn form_edit_triggers_undo_snapshot() {
        let r = reg();
        let model = model_with_real_node();
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        assert!(model.editor.undo_stack.is_empty());

        // Start edit, commit.
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::StartEdit),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::CommitEdit),
            &r,
        );

        assert!(
            !model.editor.undo_stack.is_empty(),
            "commit should create undo snapshot"
        );
    }

    #[test]
    fn undo_after_param_edit_restores_value() {
        let r = reg();
        let model = model_with_real_node();
        let (model, _) = update(model, EditorMessage::ExpandToggle, &r);
        let original_quality = model.editor.nodes[0].params.get("quality").cloned();

        // Edit quality.
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::StartEdit),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::EditChar('1')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(bnto_form::FormMessage::CommitEdit),
            &r,
        );

        // Undo should restore.
        let (model, _) = update(model, EditorMessage::Undo, &r);
        let restored = model.editor.nodes[0].params.get("quality").cloned();
        assert_eq!(
            restored, original_quality,
            "undo should restore original value"
        );
    }

    // --- Save workflow ---

    #[test]
    fn save_emits_save_action() {
        let r = reg();
        let model = model_with_nodes(1);
        let (_, action) = update(model, EditorMessage::Save, &r);
        assert_eq!(action, EditorAction::Save);
    }

    #[test]
    fn back_on_clean_emits_back_action() {
        let r = reg();
        let model = model_with_nodes(1);
        assert!(!model.editor.dirty);
        let (_, action) = update(model, EditorMessage::Back, &r);
        assert_eq!(action, EditorAction::Back);
    }

    #[test]
    fn back_on_dirty_shows_confirmation() {
        let r = reg();
        let mut model = model_with_nodes(1);
        model.editor.dirty = true;
        let (model, action) = update(model, EditorMessage::Back, &r);
        assert_eq!(action, EditorAction::None);
        assert!(model.confirming_dirty_exit);
    }

    #[test]
    fn dirty_save_emits_save_and_back() {
        let r = reg();
        let mut model = model_with_nodes(1);
        model.confirming_dirty_exit = true;
        let (model, action) = update(model, EditorMessage::DirtySave, &r);
        assert_eq!(action, EditorAction::SaveAndBack);
        assert!(!model.confirming_dirty_exit);
    }

    #[test]
    fn dirty_discard_emits_back() {
        let r = reg();
        let mut model = model_with_nodes(1);
        model.confirming_dirty_exit = true;
        let (model, action) = update(model, EditorMessage::DirtyDiscard, &r);
        assert_eq!(action, EditorAction::Back);
        assert!(!model.confirming_dirty_exit);
    }

    #[test]
    fn dirty_cancel_stays_on_editor() {
        let r = reg();
        let mut model = model_with_nodes(1);
        model.confirming_dirty_exit = true;
        let (model, action) = update(model, EditorMessage::DirtyCancel, &r);
        assert_eq!(action, EditorAction::None);
        assert!(!model.confirming_dirty_exit);
    }

    #[test]
    fn save_on_clean_editor_still_saves() {
        let r = reg();
        let model = model_with_nodes(1);
        assert!(!model.editor.dirty);
        let (_, action) = update(model, EditorMessage::Save, &r);
        assert_eq!(action, EditorAction::Save, "Ctrl+S always saves");
    }
}
