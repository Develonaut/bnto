// Editor inline form — keeps the active node form and EditorModel in sync.

use bnto_core::registry::NodeRegistry;

use super::super::editor_bridge;
use super::{EditorAction, EditorScreenModel};

/// Handle a form message for the active inline form.
pub(super) fn update_form(
    mut model: EditorScreenModel,
    msg: tonkotsu::FormMessage,
    registry: &NodeRegistry,
) -> (EditorScreenModel, EditorAction) {
    if let Some(ref mut active) = model.active_form {
        let was_editing = editor_bridge::is_editing(active);
        editor_bridge::apply_form_message(active, msg);
        let is_editing_now = editor_bridge::is_editing(active);

        // On commit (was editing → now idle), sync values to EditorModel.
        if was_editing && !is_editing_now {
            commit_form(&mut model, registry);
        }
    }
    (model, EditorAction::None)
}

/// Push the active form's values into EditorModel, then rebuild the form
/// from the updated node to keep the two in sync.
fn commit_form(model: &mut EditorScreenModel, registry: &NodeRegistry) {
    let Some(active) = model.active_form.as_ref() else {
        return;
    };
    let idx = active.node_index;
    let updates = editor_bridge::collect_param_updates(active);
    for (key, value) in updates {
        model.editor.update_param(idx, &key, value);
    }
    if let Some(node) = model.editor.nodes.get(idx) {
        model.active_form = editor_bridge::build_form(node, idx, registry);
    }
}

/// Sync pending form edits to EditorModel and clear the active form.
pub(super) fn sync_and_clear_form(model: &mut EditorScreenModel) {
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
pub(super) fn rebuild_form_if_expanded(model: &mut EditorScreenModel, registry: &NodeRegistry) {
    if let Some(idx) = model.editor.selected_index
        && let Some(node) = model.editor.nodes.get(idx)
        && node.expanded
    {
        model.active_form = editor_bridge::build_form(node, idx, registry);
        return;
    }
    model.active_form = None;
}
