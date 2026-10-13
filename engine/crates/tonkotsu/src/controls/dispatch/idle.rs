// Idle-state dispatch — starting an edit, toggling, and cycling in place.

use super::{multi_select, number, select, text};
use crate::field::{Field, FieldKind};
use crate::form::FormMessage;

/// Handle a message for a field that is not being edited.
pub(super) fn dispatch_idle(field: Field, msg: &FormMessage) -> Field {
    match (&field.kind, msg) {
        (FieldKind::MultiSelect { .. }, FormMessage::StartEdit) => multi_select::start_edit(field),
        (FieldKind::Text { .. }, FormMessage::StartEdit) => text::start_text_edit(field),
        (FieldKind::TextArea { .. }, FormMessage::StartEdit) => text::start_text_area_edit(field),
        (FieldKind::Number { .. }, FormMessage::StartEdit) => text::start_number_edit(field),
        (FieldKind::Select { .. }, FormMessage::StartEdit) => select::expand(field),
        (FieldKind::Confirm { .. }, FormMessage::ToggleConfirm) => toggle_confirm(field),
        (FieldKind::Select { .. }, FormMessage::CycleNext | FormMessage::CyclePrev) => {
            select::cycle(field, msg)
        }
        (FieldKind::Number { .. }, FormMessage::CycleNext | FormMessage::CyclePrev) => {
            number::step_number(field, msg)
        }
        _ => field,
    }
}

/// Flip a confirm field between "true" and "false".
fn toggle_confirm(field: Field) -> Field {
    let new_value = if field.value == "true" {
        "false"
    } else {
        "true"
    };
    Field {
        value: new_value.to_string(),
        ..field
    }
}
