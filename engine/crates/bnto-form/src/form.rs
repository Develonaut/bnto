//! Form-level model and message types.
//!
//! `FormModel` holds a collection of fields and tracks which one is focused.
//! `FormMessage` is the union of all possible user actions across field types.
//! `update()` is a pure function: takes ownership of the model + message,
//! returns a new model with the transition applied.

use crate::controls;
use crate::field::{Field, FieldKind, FieldState};

/// Top-level form state — a list of fields with focus tracking.
#[derive(Debug, Clone)]
pub struct FormModel {
    pub fields: Vec<Field>,
    pub focused: usize,
    pub scroll_offset: usize,
    pub viewport_height: usize,
}

impl FormModel {
    /// Create a new form from a list of fields. Focus starts on the first visible field.
    pub fn new(fields: Vec<Field>) -> Self {
        let focused = fields.iter().position(|f| f.visible).unwrap_or(0);
        Self {
            fields,
            focused,
            scroll_offset: 0,
            viewport_height: 20,
        }
    }

    /// Set the viewport height. When non-zero, `render_form()` slices output
    /// to fit and auto-scrolls the focused field into view.
    pub fn with_viewport(mut self, height: usize) -> Self {
        self.viewport_height = height;
        self
    }

    /// Get the currently focused field, if any.
    pub fn focused_field(&self) -> Option<&Field> {
        self.fields.get(self.focused)
    }

    /// Get a field's current value by id.
    pub fn value(&self, id: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|f| f.id == id)
            .map(|f| f.value.as_str())
    }
}

/// All possible user actions across field types.
#[derive(Debug, Clone, PartialEq)]
pub enum FormMessage {
    // --- Navigation ---
    FocusNext,
    FocusPrev,

    // --- Edit lifecycle ---
    StartEdit,
    CommitEdit,
    CancelEdit,

    // --- Text input (TextEditing / NumberEditing) ---
    EditChar(char),
    EditBackspace,
    DeleteForward,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    CursorWordBack,
    CursorWordForward,
    DeleteWordBack,

    // --- Inline actions (no edit mode needed) ---
    ToggleConfirm,
    CycleNext,
    CyclePrev,
    ResetDefault,

    // --- Select list (SelectExpanded) ---
    SelectHighlightNext,
    SelectHighlightPrev,
    SelectConfirm,
    SelectFilterChar(char),
    SelectFilterBackspace,

    // --- Viewport ---
    Resize { height: usize },
}

/// Pure state transition — apply a message to the form model.
pub fn update(model: FormModel, msg: FormMessage) -> FormModel {
    match msg {
        FormMessage::FocusNext => focus_next(model),
        FormMessage::FocusPrev => focus_prev(model),
        FormMessage::Resize { height } => FormModel {
            viewport_height: height,
            ..model
        },
        FormMessage::ResetDefault => reset_default(model),
        // All other messages dispatch to the focused field
        _ => update_focused_field(model, msg),
    }
}

// --- Navigation ---

fn focus_next(model: FormModel) -> FormModel {
    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible)
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

fn focus_prev(model: FormModel) -> FormModel {
    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible)
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

fn reset_default(model: FormModel) -> FormModel {
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

// --- Dispatch to focused field ---

fn update_focused_field(model: FormModel, msg: FormMessage) -> FormModel {
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

/// Route a message to the appropriate control handler based on field kind + state.
fn dispatch_field_message(field: Field, msg: FormMessage) -> Field {
    match (&field.kind, &field.state, &msg) {
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
            let filtered = filter_options(options, &new_filter);
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
            let filtered = filter_options(options, &new_filter);
            Field {
                state: FieldState::SelectExpanded {
                    highlight: 0,
                    filter: new_filter,
                    filtered_indices: filtered,
                },
                ..field
            }
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

/// Return indices of options whose labels contain the filter string (case-insensitive).
fn filter_options(options: &[crate::field::SelectOption], filter: &str) -> Vec<usize> {
    if filter.is_empty() {
        return (0..options.len()).collect();
    }
    let lower = filter.to_lowercase();
    options
        .iter()
        .enumerate()
        .filter(|(_, o)| o.label.to_lowercase().contains(&lower))
        .map(|(i, _)| i)
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, number, select, text};

    fn make_form() -> FormModel {
        FormModel::new(vec![
            text("a").label("First").build(),
            text("b").label("Second").build(),
            text("c").label("Third").build(),
        ])
    }

    #[test]
    fn test_form_model_focus_starts_at_first_visible() {
        let form = make_form();
        assert_eq!(form.focused, 0);
    }

    #[test]
    fn test_form_model_focus_next_wraps() {
        let form = make_form();
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 1);
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 2);
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 0); // wraps
    }

    #[test]
    fn test_form_model_focus_prev_wraps() {
        let form = make_form();
        let form = update(form, FormMessage::FocusPrev);
        assert_eq!(form.focused, 2); // wraps to last
        let form = update(form, FormMessage::FocusPrev);
        assert_eq!(form.focused, 1);
    }

    #[test]
    fn test_form_model_focus_skips_hidden() {
        let mut form = make_form();
        form.fields[1].visible = false;
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 2); // skips index 1
    }

    #[test]
    fn test_start_edit_text_field() {
        let form = FormModel::new(vec![text("x").value("hello").build()]);
        let form = update(form, FormMessage::StartEdit);
        match &form.fields[0].state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "hello");
                assert_eq!(*cursor, 5);
            }
            other => panic!("expected TextEditing, got {other:?}"),
        }
    }

    #[test]
    fn test_cancel_edit_reverts() {
        let form = FormModel::new(vec![text("x").value("original").build()]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::EditChar('Z'));
        let form = update(form, FormMessage::CancelEdit);
        assert_eq!(form.fields[0].state, FieldState::Idle);
        assert_eq!(form.fields[0].value, "original");
    }

    #[test]
    fn test_commit_edit_saves_value() {
        let form = FormModel::new(vec![text("x").value("old").build()]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::EditChar('!'));
        let form = update(form, FormMessage::CommitEdit);
        assert_eq!(form.fields[0].state, FieldState::Idle);
        assert_eq!(form.fields[0].value, "old!");
    }

    #[test]
    fn test_toggle_confirm() {
        let form = FormModel::new(vec![confirm("ok").value("false").build()]);
        let form = update(form, FormMessage::ToggleConfirm);
        assert_eq!(form.fields[0].value, "true");
        let form = update(form, FormMessage::ToggleConfirm);
        assert_eq!(form.fields[0].value, "false");
    }

    #[test]
    fn test_cycle_select() {
        let form = FormModel::new(vec![
            select("fmt", &[("a", "A"), ("b", "B"), ("c", "C")])
                .value("a")
                .build(),
        ]);
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "b");
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "c");
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "a"); // wraps
    }

    #[test]
    fn test_cycle_select_prev_wraps() {
        let form = FormModel::new(vec![
            select("fmt", &[("a", "A"), ("b", "B")]).value("a").build(),
        ]);
        let form = update(form, FormMessage::CyclePrev);
        assert_eq!(form.fields[0].value, "b"); // wraps to last
    }

    #[test]
    fn test_step_number() {
        let form = FormModel::new(vec![
            number("q").range(0.0, 100.0).step(10.0).value("50").build(),
        ]);
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "60");
        let form = update(form, FormMessage::CyclePrev);
        assert_eq!(form.fields[0].value, "50");
    }

    #[test]
    fn test_step_number_clamps() {
        let form = FormModel::new(vec![
            number("q").range(0.0, 100.0).step(10.0).value("95").build(),
        ]);
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "100"); // clamped to max
    }

    #[test]
    fn test_reset_default() {
        let form = FormModel::new(vec![
            text("x").value("changed").default(Some("original")).build(),
        ]);
        let form = update(form, FormMessage::ResetDefault);
        assert_eq!(form.fields[0].value, "original");
    }

    #[test]
    fn test_resize_viewport() {
        let form = make_form();
        let form = update(form, FormMessage::Resize { height: 40 });
        assert_eq!(form.viewport_height, 40);
    }

    #[test]
    fn test_value_lookup() {
        let form = FormModel::new(vec![text("name").value("hello").build()]);
        assert_eq!(form.value("name"), Some("hello"));
        assert_eq!(form.value("missing"), None);
    }

    #[test]
    fn test_focused_field() {
        let form = make_form();
        assert_eq!(form.focused_field().unwrap().id, "a");
    }

    #[test]
    fn test_commit_number_edit_valid() {
        let form = FormModel::new(vec![number("q").range(0.0, 100.0).value("50").build()]);
        let form = update(form, FormMessage::StartEdit);
        // Type "75" by clearing and entering
        let mut form = form;
        form.fields[0].state = FieldState::NumberEditing {
            buffer: "75".to_string(),
            cursor: 2,
        };
        let form = update(form, FormMessage::CommitEdit);
        assert_eq!(form.fields[0].value, "75");
        assert_eq!(form.fields[0].state, FieldState::Idle);
    }

    #[test]
    fn test_commit_number_edit_invalid() {
        let form = FormModel::new(vec![number("q").value("50").build()]);
        let mut form = update(form, FormMessage::StartEdit);
        form.fields[0].state = FieldState::NumberEditing {
            buffer: "abc".to_string(),
            cursor: 3,
        };
        let form = update(form, FormMessage::CommitEdit);
        assert!(form.fields[0].error.is_some());
    }

    #[test]
    fn test_commit_text_with_validator() {
        let form = FormModel::new(vec![
            text("x")
                .validator(std::sync::Arc::new(crate::validators::not_empty))
                .build(),
        ]);
        let mut form = update(form, FormMessage::StartEdit);
        form.fields[0].state = FieldState::TextEditing {
            buffer: String::new(),
            cursor: 0,
        };
        let form = update(form, FormMessage::CommitEdit);
        assert_eq!(form.fields[0].error.as_deref(), Some("Cannot be empty"));
        assert!(matches!(
            form.fields[0].state,
            FieldState::TextEditing { .. }
        ));
    }

    #[test]
    fn test_error_clears_on_keystroke() {
        let form = FormModel::new(vec![text("x").required().build()]);
        // Start editing, set empty buffer, commit to get error
        let mut form = update(form, FormMessage::StartEdit);
        form.fields[0].state = FieldState::TextEditing {
            buffer: String::new(),
            cursor: 0,
        };
        let form = update(form, FormMessage::CommitEdit);
        assert!(form.fields[0].error.is_some());
        // Type a character — error should clear
        let form = update(form, FormMessage::EditChar('a'));
        assert!(form.fields[0].error.is_none());
    }

    #[test]
    fn test_required_builder_shorthand() {
        let field = text("x").required().build();
        assert!(field.validator.is_some());
    }

    #[test]
    fn test_commit_with_validator_clears_error_on_valid() {
        let form = FormModel::new(vec![text("x").required().value("initial").build()]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::CommitEdit);
        assert!(form.fields[0].error.is_none());
        assert_eq!(form.fields[0].value, "initial");
    }
}
