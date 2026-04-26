//! Navigation and dispatch helpers for `FormModel`.
//!
//! Pure functions extracted from `form.rs` to keep file sizes
//! under the Bento Box 250-line limit.

use crate::controls::dispatch::dispatch_field_message;
use crate::field::FieldState;
use crate::form::{FormMessage, FormModel};

/// Advance focus to the next visible, focusable field (wrapping).
pub(crate) fn focus_next(model: FormModel) -> FormModel {
    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible && f.is_focusable())
        .map(|(i, _)| i)
        .collect();
    if visible_indices.is_empty() {
        return model;
    }
    let current_pos = visible_indices
        .iter()
        .position(|&i| i == model.focused)
        .unwrap_or(0);
    let next_pos = (current_pos + 1) % visible_indices.len();
    FormModel {
        focused: visible_indices[next_pos],
        ..model
    }
}

/// Move focus to the previous visible, focusable field (wrapping).
pub(crate) fn focus_prev(model: FormModel) -> FormModel {
    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible && f.is_focusable())
        .map(|(i, _)| i)
        .collect();
    if visible_indices.is_empty() {
        return model;
    }
    let current_pos = visible_indices
        .iter()
        .position(|&i| i == model.focused)
        .unwrap_or(0);
    let next_pos = if current_pos == 0 {
        visible_indices.len() - 1
    } else {
        current_pos - 1
    };
    FormModel {
        focused: visible_indices[next_pos],
        ..model
    }
}

/// Reset the focused field to its default value, if one is set.
pub(crate) fn reset_default(model: FormModel) -> FormModel {
    let mut fields = model.fields;
    if let Some(field) = fields.get_mut(model.focused)
        && let Some(ref default) = field.default
    {
        field.value = default.clone();
        field.state = FieldState::Idle;
        field.error = None;
    }
    FormModel { fields, ..model }
}

/// Dispatch a message to the currently focused field.
pub(crate) fn update_focused_field(model: FormModel, msg: FormMessage) -> FormModel {
    let mut fields = model.fields;
    let focused = model.focused;
    if let Some(field) = fields.get_mut(focused) {
        *field = dispatch_field_message(field.clone(), msg);
    }
    FormModel {
        fields,
        focused,
        ..model
    }
}
