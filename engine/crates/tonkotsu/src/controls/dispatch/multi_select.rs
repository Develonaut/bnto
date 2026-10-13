// Multi-select dispatch — expand, toggle, highlight and confirm selections.

use super::{to_idle, wrap_prev};
use crate::field::{Field, FieldKind, FieldState, parse_multiselect_value};
use crate::form::FormMessage;

/// StartEdit: expand with the current value's options pre-selected.
pub(super) fn start_edit(field: Field) -> Field {
    let FieldKind::MultiSelect { options } = &field.kind else {
        return field;
    };
    let current = parse_multiselect_value(&field.value);
    let selected: Vec<usize> = options
        .iter()
        .enumerate()
        .filter(|(_, o)| current.contains(&o.value))
        .map(|(i, _)| i)
        .collect();
    Field {
        state: FieldState::MultiSelectEditing {
            highlight: 0,
            selected,
        },
        error: None,
        ..field
    }
}

/// Handle a message while the option list is expanded.
pub(super) fn dispatch_editing(field: Field, msg: &FormMessage) -> Field {
    let Some((len, highlight, selected)) = editing_state(&field) else {
        return field;
    };
    match msg {
        FormMessage::MultiSelectToggle => editing(field, highlight, toggle(selected, highlight)),
        FormMessage::MultiSelectHighlightNext if len > 0 => {
            editing(field, (highlight + 1) % len, selected)
        }
        FormMessage::MultiSelectHighlightPrev if len > 0 => {
            editing(field, wrap_prev(highlight, len), selected)
        }
        FormMessage::MultiSelectConfirm => confirm(field, &selected),
        FormMessage::CancelEdit => to_idle(field),
        _ => field,
    }
}

/// Option count, highlight and selection of an expanded multi-select.
fn editing_state(field: &Field) -> Option<(usize, usize, Vec<usize>)> {
    let FieldKind::MultiSelect { options } = &field.kind else {
        return None;
    };
    let FieldState::MultiSelectEditing {
        highlight,
        selected,
    } = &field.state
    else {
        return None;
    };
    Some((options.len(), *highlight, selected.clone()))
}

/// Replace the expanded state's highlight and selection.
fn editing(field: Field, highlight: usize, selected: Vec<usize>) -> Field {
    Field {
        state: FieldState::MultiSelectEditing {
            highlight,
            selected,
        },
        ..field
    }
}

/// Toggle `highlight` in the (sorted) selection.
fn toggle(mut selected: Vec<usize>, highlight: usize) -> Vec<usize> {
    if selected.contains(&highlight) {
        selected.retain(|&i| i != highlight);
    } else {
        selected.push(highlight);
        selected.sort_unstable();
    }
    selected
}

/// Confirm saves the selection as a comma-separated value.
fn confirm(field: Field, selected: &[usize]) -> Field {
    let FieldKind::MultiSelect { options } = &field.kind else {
        return field;
    };
    let values: Vec<String> = selected
        .iter()
        .filter_map(|&i| options.get(i).map(|o| o.value.clone()))
        .collect();
    Field {
        value: crate::field::serialize_multiselect_value(&values),
        state: FieldState::Idle,
        ..field
    }
}
