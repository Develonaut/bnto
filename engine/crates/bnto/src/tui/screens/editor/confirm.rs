// Editor confirmation overlays — delete-node and dirty-exit prompts.

use super::{EditorAction, EditorMessage, EditorScreenModel};

/// Handle messages during delete confirmation.
///
/// Any message closes the prompt; only `DeleteConfirm` removes the node.
pub(super) fn update_delete_confirm(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    if msg == EditorMessage::DeleteConfirm
        && let Some(idx) = model.editor.selected_index
    {
        model.editor.remove_node(idx);
    }
    model.confirming_delete = false;
    (model, EditorAction::None)
}

/// Handle messages during dirty-exit confirmation ("Save? y/n/Esc").
///
/// Any message closes the prompt; anything other than save or discard
/// cancels and stays on the editor.
pub(super) fn update_dirty_confirm(
    mut model: EditorScreenModel,
    msg: EditorMessage,
) -> (EditorScreenModel, EditorAction) {
    model.confirming_dirty_exit = false;
    let action = match msg {
        EditorMessage::DirtySave => EditorAction::SaveAndBack,
        EditorMessage::DirtyDiscard => EditorAction::Back,
        _ => EditorAction::None,
    };
    (model, action)
}
