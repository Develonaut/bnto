// Field message dispatch — routes messages to the appropriate control handler.
//
// Pure function: takes a Field + FormMessage, returns a new Field with
// the transition applied. No I/O, no rendering.

use unicode_segmentation::UnicodeSegmentation;

use crate::controls;
use crate::controls::fuzzy::fuzzy_filter_options;
use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// Route a message to the appropriate control handler based on field kind + state.
pub(crate) fn dispatch_field_message(field: Field, msg: FormMessage) -> Field {
    match (&field.kind, &field.state, &msg) {
        // Note fields are read-only — ignore all messages
        (FieldKind::Note { .. }, _, _) => field,

        // StartEdit: transition from Idle to editing state
        (FieldKind::Text { .. }, FieldState::Idle, FormMessage::StartEdit) => {
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

        // Cancel: any editing state → Idle, discard buffer
        (_, FieldState::TextEditing { .. }, FormMessage::CancelEdit)
        | (_, FieldState::NumberEditing { .. }, FormMessage::CancelEdit)
        | (_, FieldState::SelectExpanded { .. }, FormMessage::CancelEdit) => Field {
            state: FieldState::Idle,
            ..field
        },

        // Commit: text editing → validate and save
        (_, FieldState::TextEditing { buffer, .. }, FormMessage::CommitEdit) => {
            commit_text_edit(field.clone(), buffer.clone())
        }

        // Commit: number editing → parse, validate, save
        (_, FieldState::NumberEditing { buffer, .. }, FormMessage::CommitEdit) => {
            commit_number_edit(field.clone(), buffer.clone())
        }

        // Commit: select expanded → confirm highlighted option
        (
            FieldKind::Select { options, .. },
            FieldState::SelectExpanded {
                highlight,
                filtered_indices,
                ..
            },
            FormMessage::SelectConfirm,
        ) => {
            let actual_idx = filtered_indices.get(*highlight).copied();
            let new_value = actual_idx
                .and_then(|i| options.get(i))
                .map(|o| o.value.clone())
                .unwrap_or_else(|| field.value.clone());
            Field {
                value: new_value,
                state: FieldState::Idle,
                ..field
            }
        }

        // Select highlight navigation
        (
            _,
            FieldState::SelectExpanded {
                highlight,
                filter,
                filtered_indices,
                ..
            },
            FormMessage::SelectHighlightNext,
        ) => {
            if filtered_indices.is_empty() {
                return field;
            }
            let next = (*highlight + 1) % filtered_indices.len();
            Field {
                state: FieldState::SelectExpanded {
                    highlight: next,
                    filter: filter.clone(),
                    filtered_indices: filtered_indices.clone(),
                },
                ..field
            }
        }

        (
            _,
            FieldState::SelectExpanded {
                highlight,
                filter,
                filtered_indices,
                ..
            },
            FormMessage::SelectHighlightPrev,
        ) => {
            if filtered_indices.is_empty() {
                return field;
            }
            let prev = if *highlight == 0 {
                filtered_indices.len() - 1
            } else {
                highlight - 1
            };
            Field {
                state: FieldState::SelectExpanded {
                    highlight: prev,
                    filter: filter.clone(),
                    filtered_indices: filtered_indices.clone(),
                },
                ..field
            }
        }

        // Select filter typing
        (
            FieldKind::Select { options, .. },
            FieldState::SelectExpanded { filter, .. },
            FormMessage::SelectFilterChar(ch),
        ) => {
            let new_filter = format!("{filter}{ch}");
            let filtered = fuzzy_filter_options(options, &new_filter);
            Field {
                state: FieldState::SelectExpanded {
                    highlight: 0,
                    filter: new_filter,
                    filtered_indices: filtered,
                },
                ..field
            }
        }

        (
            FieldKind::Select { options, .. },
            FieldState::SelectExpanded { filter, .. },
            FormMessage::SelectFilterBackspace,
        ) => {
            let new_filter = {
                let mut f = filter.clone();
                f.pop();
                f
            };
            let filtered = fuzzy_filter_options(options, &new_filter);
            Field {
                state: FieldState::SelectExpanded {
                    highlight: 0,
                    filter: new_filter,
                    filtered_indices: filtered,
                },
                ..field
            }
        }

        // TextArea StartEdit
        (FieldKind::TextArea { .. }, FieldState::Idle, FormMessage::StartEdit) => {
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

        // TextArea Cancel
        (_, FieldState::TextAreaEditing { .. }, FormMessage::CancelEdit) => Field {
            state: FieldState::Idle,
            ..field
        },

        // TextArea Commit
        (_, FieldState::TextAreaEditing { buffer, .. }, FormMessage::CommitEdit) => {
            if let Some(ref validator) = field.validator
                && let Some(err) = validator(buffer)
            {
                return Field {
                    error: Some(err),
                    ..field
                };
            }
            Field {
                value: buffer.clone(),
                state: FieldState::Idle,
                error: None,
                ..field
            }
        }

        // TextArea editing messages
        (FieldKind::TextArea { .. }, FieldState::TextAreaEditing { .. }, _) => {
            let mut updated = controls::text_area::update(field, msg);
            updated.error = None;
            updated
        }

        // Text/Number editing messages (after commit/cancel handled above)
        (FieldKind::Text { .. }, FieldState::TextEditing { .. }, _)
        | (FieldKind::Number { .. }, FieldState::NumberEditing { .. }, _) => {
            let mut updated = controls::text_input::update(field, msg);
            // Clear error on any keystroke during editing
            updated.error = None;
            updated
        }

        // Confirm toggle works in Idle
        (FieldKind::Confirm { .. }, FieldState::Idle, FormMessage::ToggleConfirm) => {
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

        // Select cycling in Idle
        (FieldKind::Select { .. }, FieldState::Idle, FormMessage::CycleNext)
        | (FieldKind::Select { .. }, FieldState::Idle, FormMessage::CyclePrev) => {
            cycle_select(field, &msg)
        }

        // Number stepping in Idle
        (FieldKind::Number { .. }, FieldState::Idle, FormMessage::CycleNext)
        | (FieldKind::Number { .. }, FieldState::Idle, FormMessage::CyclePrev) => {
            step_number(field, &msg)
        }

        // Number StartEdit
        (FieldKind::Number { .. }, FieldState::Idle, FormMessage::StartEdit) => {
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

        // Select StartEdit → expand
        (FieldKind::Select { options, .. }, FieldState::Idle, FormMessage::StartEdit) => {
            let current_idx = options
                .iter()
                .position(|o| o.value == field.value)
                .unwrap_or(0);
            let all_indices: Vec<usize> = (0..options.len()).collect();
            Field {
                state: FieldState::SelectExpanded {
                    highlight: current_idx,
                    filter: String::new(),
                    filtered_indices: all_indices,
                },
                ..field
            }
        }

        _ => field,
    }
}

fn cycle_select(field: Field, msg: &FormMessage) -> Field {
    if let FieldKind::Select { ref options, .. } = field.kind {
        if options.is_empty() {
            return field;
        }
        let current = options
            .iter()
            .position(|o| o.value == field.value)
            .unwrap_or(0);
        let next = match msg {
            FormMessage::CycleNext => (current + 1) % options.len(),
            FormMessage::CyclePrev => {
                if current == 0 {
                    options.len() - 1
                } else {
                    current - 1
                }
            }
            _ => current,
        };
        Field {
            value: options[next].value.clone(),
            ..field
        }
    } else {
        field
    }
}

fn step_number(field: Field, msg: &FormMessage) -> Field {
    if let FieldKind::Number { min, max, step, .. } = &field.kind {
        let current: f64 = field.value.parse().unwrap_or(0.0);
        let increment = step.unwrap_or(1.0);
        let new_val = match msg {
            FormMessage::CycleNext => current + increment,
            FormMessage::CyclePrev => current - increment,
            _ => current,
        };
        let clamped = clamp_number(new_val, *min, *max);
        Field {
            value: format_number(clamped),
            ..field
        }
    } else {
        field
    }
}

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

fn commit_number_edit(field: Field, buffer: String) -> Field {
    let parsed: Result<f64, _> = buffer.parse();
    match parsed {
        Ok(val) => {
            if let FieldKind::Number { min, max, .. } = &field.kind {
                let clamped = clamp_number(val, *min, *max);
                Field {
                    value: format_number(clamped),
                    state: FieldState::Idle,
                    error: None,
                    ..field
                }
            } else {
                field
            }
        }
        Err(_) => Field {
            error: Some("Must be a number".to_string()),
            ..field
        },
    }
}

fn clamp_number(val: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut v = val;
    if let Some(lo) = min {
        v = v.max(lo);
    }
    if let Some(hi) = max {
        v = v.min(hi);
    }
    v
}

/// Format a number, stripping trailing ".0" for clean display.
fn format_number(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{}", val as i64)
    } else {
        format!("{val}")
    }
}
