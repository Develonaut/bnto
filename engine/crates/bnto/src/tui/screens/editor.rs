// Editor screen — TUI list editor for creating and modifying recipes.
//
// Pure TEA state machine: EditorScreenModel + EditorMessage + update().
// All mutations delegate to EditorModel (bnto-core) for undo/redo support.
// The picker overlay reuses the Browser's search-and-filter pattern.

mod confirm;
mod form;
mod list;
mod picker;

use bnto_core::editor::EditorModel;
use bnto_core::registry::NodeRegistry;

use super::editor_bridge::ActiveNodeForm;

pub use picker::PickerState;

// Node type metadata the tests below build fixtures with.
#[cfg(test)]
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

    // --- Save + exit ---
    Save,
    Back,
    DirtySave,
    DirtyDiscard,
    DirtyCancel,

    // --- Inline form editing ---
    Form(tonkotsu::FormMessage),

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
///
/// Overlays capture input first (picker, delete confirm, dirty-exit
/// confirm), then form messages go to the inline form, and everything
/// else operates on the node list.
pub fn update(
    mut model: EditorScreenModel,
    msg: EditorMessage,
    registry: &NodeRegistry,
) -> (EditorScreenModel, EditorAction) {
    if model.picker.is_some() {
        return picker::update_picker(model, msg);
    }
    if model.confirming_delete {
        return confirm::update_delete_confirm(model, msg);
    }
    if model.confirming_dirty_exit {
        return confirm::update_dirty_confirm(model, msg);
    }
    if let EditorMessage::Form(form_msg) = msg {
        return form::update_form(model, form_msg, registry);
    }
    let action = list::update_list(&mut model, msg, registry);
    (model, action)
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
            EditorMessage::Form(tonkotsu::FormMessage::StartEdit),
            &r,
        );
        // Type new value.
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditChar('5')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditChar('0')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::CommitEdit),
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
            EditorMessage::Form(tonkotsu::FormMessage::StartEdit),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::CommitEdit),
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
            EditorMessage::Form(tonkotsu::FormMessage::StartEdit),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditBackspace),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::EditChar('1')),
            &r,
        );
        let (model, _) = update(
            model,
            EditorMessage::Form(tonkotsu::FormMessage::CommitEdit),
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
