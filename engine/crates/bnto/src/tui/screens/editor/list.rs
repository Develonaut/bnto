// Editor node list — navigation, reorder, expand, undo/redo, save/exit.
//
// Runs when no overlay is capturing input. Every operation that leaves
// the current node first syncs and clears the inline form.

use bnto_core::editor::EditorModel;
use bnto_core::metadata::all_node_types;
use bnto_core::registry::NodeRegistry;

use super::super::editor_bridge;
use super::form::{rebuild_form_if_expanded, sync_and_clear_form};
use super::{EditorAction, EditorMessage, EditorScreenModel, PickerState};

/// Handle a node-list message. Overlay and form messages arriving here
/// (no overlay active) are no-ops.
pub(super) fn update_list(
    model: &mut EditorScreenModel,
    msg: EditorMessage,
    registry: &NodeRegistry,
) -> EditorAction {
    match msg {
        EditorMessage::CursorDown => step_cursor(model, registry, wrap_next),
        EditorMessage::CursorUp => step_cursor(model, registry, wrap_prev),
        EditorMessage::ExpandToggle => toggle_expand(model, registry),
        EditorMessage::MoveDown => {
            move_selected(model, |idx, len| Some(idx + 1).filter(|&t| t < len))
        }
        EditorMessage::MoveUp => move_selected(model, |idx, _| idx.checked_sub(1)),
        EditorMessage::DeleteRequest => request_delete(model),
        EditorMessage::Undo => apply_history(model, registry, EditorModel::undo),
        EditorMessage::Redo => apply_history(model, registry, EditorModel::redo),
        EditorMessage::OpenPicker => open_picker(model),
        EditorMessage::Save => {
            sync_and_clear_form(model);
            EditorAction::Save
        }
        EditorMessage::Back => back(model),
        _ => EditorAction::None,
    }
}

/// Index after `current`, wrapping to the first node.
fn wrap_next(current: usize, len: usize) -> usize {
    if current + 1 >= len { 0 } else { current + 1 }
}

/// Index before `current`, wrapping to the last node.
fn wrap_prev(current: usize, len: usize) -> usize {
    if current == 0 { len - 1 } else { current - 1 }
}

/// Move the selection with `step`, then rebuild the form for the new node.
fn step_cursor(
    model: &mut EditorScreenModel,
    registry: &NodeRegistry,
    step: fn(usize, usize) -> usize,
) -> EditorAction {
    sync_and_clear_form(model);
    let len = model.editor.nodes.len();
    if len > 0 {
        let current = model.editor.selected_index.unwrap_or(0);
        model.editor.selected_index = Some(step(current, len));
    }
    rebuild_form_if_expanded(model, registry);
    EditorAction::None
}

/// Expand or collapse the selected node, building or clearing its form.
fn toggle_expand(model: &mut EditorScreenModel, registry: &NodeRegistry) -> EditorAction {
    if let Some(idx) = model.editor.selected_index
        && idx < model.editor.nodes.len()
    {
        let was_expanded = model.editor.nodes[idx].expanded;
        model.editor.nodes[idx].expanded = !was_expanded;
        if was_expanded {
            // Collapsing: sync any pending edits and clear form.
            sync_and_clear_form(model);
        } else {
            model.active_form = editor_bridge::build_form(&model.editor.nodes[idx], idx, registry);
        }
    }
    EditorAction::None
}

/// Reorder the selected node to the index `target` picks (if any).
fn move_selected(
    model: &mut EditorScreenModel,
    target: fn(usize, usize) -> Option<usize>,
) -> EditorAction {
    sync_and_clear_form(model);
    if let Some(idx) = model.editor.selected_index
        && let Some(to) = target(idx, model.editor.nodes.len())
    {
        model.editor.reorder(idx, to);
    }
    EditorAction::None
}

/// Show the delete confirmation when a node is selected.
fn request_delete(model: &mut EditorScreenModel) -> EditorAction {
    sync_and_clear_form(model);
    if model.editor.selected_index.is_some() && !model.editor.nodes.is_empty() {
        model.confirming_delete = true;
    }
    EditorAction::None
}

/// Undo or redo. Pending form edits are dropped, not synced.
fn apply_history(
    model: &mut EditorScreenModel,
    registry: &NodeRegistry,
    op: fn(&mut EditorModel),
) -> EditorAction {
    model.active_form = None;
    op(&mut model.editor);
    rebuild_form_if_expanded(model, registry);
    EditorAction::None
}

/// Open the "add node" picker over the full node type list.
fn open_picker(model: &mut EditorScreenModel) -> EditorAction {
    sync_and_clear_form(model);
    let types = all_node_types();
    model.picker = Some(PickerState::from_node_types(&types));
    EditorAction::None
}

/// Leave the editor, or ask to save first when there are unsaved changes.
fn back(model: &mut EditorScreenModel) -> EditorAction {
    sync_and_clear_form(model);
    if model.editor.dirty {
        model.confirming_dirty_exit = true;
        EditorAction::None
    } else {
        EditorAction::Back
    }
}
