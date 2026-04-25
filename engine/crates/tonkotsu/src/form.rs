//! Form-level model and update logic.
//!
//! `FormModel` holds a collection of fields and tracks which one is focused.
//! `update()` is a pure function: takes ownership of the model + message,
//! returns a new model with the transition applied.

use crate::controls::dispatch::dispatch_field_message;
use crate::field::{Field, FieldState};

/// Controls how the form renders: all fields expanded (Inline) or
/// compact one-liners with focused-field editing (DisplayEdit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormMode {
    /// Legacy behavior: every field renders its full control at all times.
    Inline,
    /// Compact display: each field shows label + value on one line.
    /// Enter on focused field opens edit mode (full control for that field only).
    /// Enter/Esc returns to display with saved/cancelled value.
    DisplayEdit,
}

/// Top-level form state — a list of fields with focus tracking.
#[derive(Debug, Clone)]
pub struct FormModel {
    pub fields: Vec<Field>,
    pub focused: usize,
    pub scroll_offset: usize,
    pub viewport_height: usize,
    pub mode: FormMode,
    /// Side effects pending caller fulfillment. Drain after each `update()`.
    pub pending_effects: Vec<FormEffect>,
}

impl FormModel {
    /// Create a new form from a list of fields. Focus starts on the first visible field.
    /// Defaults to `FormMode::Inline` (legacy behavior).
    pub fn new(fields: Vec<Field>) -> Self {
        let focused = fields.iter().position(|f| f.visible).unwrap_or(0);
        Self {
            fields,
            focused,
            scroll_offset: 0,
            viewport_height: 20,
            mode: FormMode::Inline,
            pending_effects: Vec::new(),
        }
    }

    /// Set the form interaction mode.
    pub fn with_mode(mut self, mode: FormMode) -> Self {
        self.mode = mode;
        self
    }

    /// Whether the form is in display mode (DisplayEdit mode with no field editing).
    pub fn is_display(&self) -> bool {
        self.mode == FormMode::DisplayEdit
            && self
                .focused_field()
                .is_some_and(|f| matches!(f.state, FieldState::Idle))
    }

    /// Whether the form is in edit mode (DisplayEdit mode with focused field editing).
    pub fn is_editing(&self) -> bool {
        self.mode == FormMode::DisplayEdit
            && self
                .focused_field()
                .is_some_and(|f| !matches!(f.state, FieldState::Idle))
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

// Re-export messages types for downstream use.
pub use crate::messages::{FormEffect, FormMessage};

/// Pure state transition — apply a message to the form model.
pub fn update(model: FormModel, msg: FormMessage) -> FormModel {
    match msg {
        FormMessage::FocusNext => {
            // In DisplayEdit mode, block navigation while editing
            if model.is_editing() {
                return model;
            }
            focus_next(model)
        }
        FormMessage::FocusPrev => {
            if model.is_editing() {
                return model;
            }
            focus_prev(model)
        }
        FormMessage::Resize { height } => FormModel {
            viewport_height: height,
            ..model
        },
        FormMessage::ResetDefault => reset_default(model),

        // StartEdit for FilePath fields needs the form-level handler
        // (emits LoadDirectory effect, enters FilePathBrowsing state).
        FormMessage::StartEdit
            if model
                .focused_field()
                .is_some_and(|f| matches!(f.kind, crate::field::FieldKind::FilePath { .. })) =>
        {
            crate::form_file_path::handle_file_path_message(model, msg)
        }

        // FilePath messages are handled at the form level (not dispatch)
        // because they need access to pending_effects and field id lookups.
        FormMessage::FilePathCursorDown
        | FormMessage::FilePathCursorUp
        | FormMessage::FilePathEnterDir
        | FormMessage::FilePathParentDir
        | FormMessage::FilePathConfirm
        | FormMessage::FilePathToggleHidden
        | FormMessage::FilePathPageDown
        | FormMessage::FilePathPageUp
        | FormMessage::FilePathGoToTop
        | FormMessage::FilePathGoToBottom
        | FormMessage::FilePathCancel
        | FormMessage::FilePathDirLoaded { .. } => {
            crate::form_file_path::handle_file_path_message(model, msg)
        }

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

    // --- DisplayEdit mode tests ---

    #[test]
    fn test_form_defaults_to_inline_mode() {
        let form = make_form();
        assert_eq!(form.mode, FormMode::Inline);
    }

    #[test]
    fn test_form_with_display_edit_mode() {
        let form = make_form().with_mode(FormMode::DisplayEdit);
        assert_eq!(form.mode, FormMode::DisplayEdit);
    }

    #[test]
    fn test_display_edit_is_display_initially() {
        let form = make_form().with_mode(FormMode::DisplayEdit);
        assert!(form.is_display());
        assert!(!form.is_editing());
    }

    #[test]
    fn test_display_edit_enter_starts_editing() {
        let form =
            FormModel::new(vec![text("x").value("hello").build()]).with_mode(FormMode::DisplayEdit);
        let form = update(form, FormMessage::StartEdit);
        assert!(form.is_editing());
        assert!(!form.is_display());
        assert!(matches!(
            form.fields[0].state,
            FieldState::TextEditing { .. }
        ));
    }

    #[test]
    fn test_display_edit_commit_returns_to_display() {
        let form =
            FormModel::new(vec![text("x").value("old").build()]).with_mode(FormMode::DisplayEdit);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::EditChar('!'));
        let form = update(form, FormMessage::CommitEdit);
        assert!(form.is_display());
        assert_eq!(form.fields[0].value, "old!");
    }

    #[test]
    fn test_display_edit_cancel_returns_to_display() {
        let form = FormModel::new(vec![text("x").value("original").build()])
            .with_mode(FormMode::DisplayEdit);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::EditChar('Z'));
        let form = update(form, FormMessage::CancelEdit);
        assert!(form.is_display());
        assert_eq!(form.fields[0].value, "original");
    }

    #[test]
    fn test_display_edit_blocks_navigation_while_editing() {
        let form = FormModel::new(vec![
            text("a").label("First").build(),
            text("b").label("Second").build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        let form = update(form, FormMessage::StartEdit);
        assert!(form.is_editing());
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 0, "should not navigate while editing");
        let form = update(form, FormMessage::FocusPrev);
        assert_eq!(form.focused, 0, "should not navigate while editing");
    }

    #[test]
    fn test_display_edit_navigation_works_in_display_mode() {
        let form = FormModel::new(vec![
            text("a").label("First").build(),
            text("b").label("Second").build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        assert!(form.is_display());
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 1);
    }

    #[test]
    fn test_display_edit_confirm_toggle_works() {
        let form = FormModel::new(vec![confirm("ok").value("false").build()])
            .with_mode(FormMode::DisplayEdit);
        // Confirm toggle works without entering edit mode
        let form = update(form, FormMessage::ToggleConfirm);
        assert_eq!(form.fields[0].value, "true");
        assert!(form.is_display()); // stays in display
    }

    #[test]
    fn test_display_edit_select_cycle_works() {
        let form = FormModel::new(vec![
            select("fmt", &[("a", "A"), ("b", "B")]).value("a").build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        let form = update(form, FormMessage::CycleNext);
        assert_eq!(form.fields[0].value, "b");
        assert!(form.is_display());
    }
}
