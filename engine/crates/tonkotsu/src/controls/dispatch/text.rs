// Text-like dispatch — text, number and text-area editing and commits.

use unicode_segmentation::UnicodeSegmentation;

use super::number::{clamp_number, format_number};
use super::to_idle;
use crate::controls;
use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// StartEdit for a text field: buffer the value, cursor at the end.
pub(super) fn start_text_edit(field: Field) -> Field {
    let cursor = field.value.len();
    Field {
        state: FieldState::TextEditing {
            buffer: field.value.clone(),
            cursor,
        },
        error: None,
        ..field
    }
}

/// StartEdit for a number field: buffer the value, cursor at the end.
pub(super) fn start_number_edit(field: Field) -> Field {
    let cursor = field.value.len();
    Field {
        state: FieldState::NumberEditing {
            buffer: field.value.clone(),
            cursor,
        },
        error: None,
        ..field
    }
}

/// StartEdit for a text area: cursor at the end of the last line.
pub(super) fn start_text_area_edit(field: Field) -> Field {
    let lines: Vec<&str> = field.value.split('\n').collect();
    let last_line = lines.len().saturating_sub(1);
    let last_col = lines.last().map(|l| l.graphemes(true).count()).unwrap_or(0);
    Field {
        state: FieldState::TextAreaEditing {
            buffer: field.value.clone(),
            cursor: last_col,
            line: last_line,
            scroll_offset: 0,
        },
        error: None,
        ..field
    }
}

/// Handle a message while a text or number field is being edited.
pub(super) fn dispatch_editing(field: Field, msg: FormMessage) -> Field {
    match (&field.kind, &field.state, &msg) {
        (_, _, FormMessage::CancelEdit) => to_idle(field),
        (_, FieldState::TextEditing { buffer, .. }, FormMessage::CommitEdit) => {
            commit_text_edit(field.clone(), buffer.clone())
        }
        (_, FieldState::NumberEditing { buffer, .. }, FormMessage::CommitEdit) => {
            commit_number_edit(field.clone(), buffer.clone())
        }
        (FieldKind::Text { .. }, FieldState::TextEditing { .. }, _)
        | (FieldKind::Number { .. }, FieldState::NumberEditing { .. }, _) => {
            // Clear error on any keystroke during editing
            clear_error(controls::text_input::update(field, msg))
        }
        _ => field,
    }
}

/// Handle a message while a text area is being edited.
pub(super) fn dispatch_text_area(field: Field, msg: FormMessage) -> Field {
    match (&field.kind, &field.state, &msg) {
        (_, _, FormMessage::CancelEdit) => to_idle(field),
        (_, FieldState::TextAreaEditing { buffer, .. }, FormMessage::CommitEdit) => {
            commit_text_edit(field.clone(), buffer.clone())
        }
        (FieldKind::TextArea { .. }, _, _) => clear_error(controls::text_area::update(field, msg)),
        _ => field,
    }
}

fn clear_error(mut field: Field) -> Field {
    field.error = None;
    field
}

/// Validate and save a text buffer, or keep editing with the error shown.
fn commit_text_edit(field: Field, buffer: String) -> Field {
    if let Some(ref validator) = field.validator
        && let Some(err) = validator(&buffer)
    {
        return Field {
            error: Some(err),
            ..field
        };
    }
    Field {
        value: buffer,
        state: FieldState::Idle,
        error: None,
        ..field
    }
}

/// Parse, clamp and save a number buffer, or keep editing with an error.
fn commit_number_edit(field: Field, buffer: String) -> Field {
    let Ok(val) = buffer.parse::<f64>() else {
        return Field {
            error: Some("Must be a number".to_string()),
            ..field
        };
    };
    let FieldKind::Number { min, max, .. } = &field.kind else {
        return field;
    };
    let clamped = clamp_number(val, *min, *max);
    Field {
        value: format_number(clamped),
        state: FieldState::Idle,
        error: None,
        ..field
    }
}
