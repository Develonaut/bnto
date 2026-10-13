// Field message dispatch — routes messages to the appropriate control handler.
//
// Pure function: takes a Field + FormMessage, returns a new Field with
// the transition applied. No I/O, no rendering.
//
// Routing is by editing state first (Idle vs each editing mode), then by
// field kind and message inside each state's handler.

mod idle;
mod multi_select;
mod number;
mod select;
mod text;

use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// Route a message to the appropriate control handler based on field kind + state.
pub(crate) fn dispatch_field_message(field: Field, msg: FormMessage) -> Field {
    // Note fields are read-only — ignore all messages
    if matches!(field.kind, FieldKind::Note { .. }) {
        return field;
    }
    match field.state {
        FieldState::Idle => idle::dispatch_idle(field, &msg),
        FieldState::MultiSelectEditing { .. } => multi_select::dispatch_editing(field, &msg),
        FieldState::SelectExpanded { .. } => select::dispatch_expanded(field, &msg),
        FieldState::TextEditing { .. } | FieldState::NumberEditing { .. } => {
            text::dispatch_editing(field, msg)
        }
        FieldState::TextAreaEditing { .. } => text::dispatch_text_area(field, msg),
        FieldState::FilePathBrowsing { .. } => field,
    }
}

/// Leave editing, discarding any in-progress state.
fn to_idle(field: Field) -> Field {
    Field {
        state: FieldState::Idle,
        ..field
    }
}

/// Index before `i` in a list of `len` items, wrapping to the end.
fn wrap_prev(i: usize, len: usize) -> usize {
    if i == 0 { len - 1 } else { i - 1 }
}
