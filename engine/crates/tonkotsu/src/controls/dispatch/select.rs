// Select dispatch — expand, filter, highlight, confirm and idle cycling.

use super::{to_idle, wrap_prev};
use crate::controls::fuzzy::fuzzy_filter_options;
use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// StartEdit: expand with every option visible and the current one highlighted.
pub(super) fn expand(field: Field) -> Field {
    let FieldKind::Select { options, .. } = &field.kind else {
        return field;
    };
    let current_idx = options
        .iter()
        .position(|o| o.value == field.value)
        .unwrap_or(0);
    let all_indices: Vec<usize> = (0..options.len()).collect();
    expanded(field, current_idx, String::new(), all_indices)
}

/// Handle a message while the option list is expanded.
pub(super) fn dispatch_expanded(field: Field, msg: &FormMessage) -> Field {
    let Some((highlight, filter, filtered_indices)) = expanded_state(&field) else {
        return field;
    };
    let is_select = matches!(field.kind, FieldKind::Select { .. });
    let len = filtered_indices.len();
    match msg {
        FormMessage::CancelEdit => to_idle(field),
        FormMessage::SelectConfirm if is_select => confirm(field, highlight, &filtered_indices),
        FormMessage::SelectHighlightNext if len > 0 => {
            expanded(field, (highlight + 1) % len, filter, filtered_indices)
        }
        FormMessage::SelectHighlightPrev if len > 0 => {
            expanded(field, wrap_prev(highlight, len), filter, filtered_indices)
        }
        FormMessage::SelectFilterChar(ch) if is_select => refilter(field, format!("{filter}{ch}")),
        FormMessage::SelectFilterBackspace if is_select => refilter(field, without_last(filter)),
        _ => field,
    }
}

/// Highlight, filter and visible option indices of an expanded select.
fn expanded_state(field: &Field) -> Option<(usize, String, Vec<usize>)> {
    match &field.state {
        FieldState::SelectExpanded {
            highlight,
            filter,
            filtered_indices,
        } => Some((*highlight, filter.clone(), filtered_indices.clone())),
        _ => None,
    }
}

/// Replace the expanded state.
fn expanded(field: Field, highlight: usize, filter: String, filtered_indices: Vec<usize>) -> Field {
    Field {
        state: FieldState::SelectExpanded {
            highlight,
            filter,
            filtered_indices,
        },
        ..field
    }
}

/// Confirm the highlighted option (keeping the old value if none).
fn confirm(field: Field, highlight: usize, filtered_indices: &[usize]) -> Field {
    let FieldKind::Select { options, .. } = &field.kind else {
        return field;
    };
    let new_value = filtered_indices
        .get(highlight)
        .and_then(|&i| options.get(i))
        .map(|o| o.value.clone())
        .unwrap_or_else(|| field.value.clone());
    Field {
        value: new_value,
        state: FieldState::Idle,
        ..field
    }
}

/// Apply a new filter string and reset the highlight to the top match.
fn refilter(field: Field, filter: String) -> Field {
    let FieldKind::Select { options, .. } = &field.kind else {
        return field;
    };
    let filtered = fuzzy_filter_options(options, &filter);
    expanded(field, 0, filter, filtered)
}

fn without_last(mut filter: String) -> String {
    filter.pop();
    filter
}

/// Cycle the value to the next or previous option without expanding.
pub(super) fn cycle(field: Field, msg: &FormMessage) -> Field {
    let FieldKind::Select { ref options, .. } = field.kind else {
        return field;
    };
    if options.is_empty() {
        return field;
    }
    let current = options
        .iter()
        .position(|o| o.value == field.value)
        .unwrap_or(0);
    let next = match msg {
        FormMessage::CycleNext => (current + 1) % options.len(),
        FormMessage::CyclePrev => wrap_prev(current, options.len()),
        _ => current,
    };
    Field {
        value: options[next].value.clone(),
        ..field
    }
}
