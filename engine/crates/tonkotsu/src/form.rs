//! Form-level model and update logic.
//!
//! `FormModel` holds a collection of fields and tracks which one is focused.
//! `update()` is a pure function: takes ownership of the model + message,
//! returns a new model with the transition applied.

use crate::field::{Field, FieldState};
use crate::form_nav::{focus_next, focus_prev, reset_default, update_focused_field};
use crate::group::FieldGroup;

/// Controls how the form renders: all fields expanded (Inline),
/// compact one-liners with focused-field editing (DisplayEdit),
/// or full-screen single-field editing (FullScreenEdit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormMode {
    /// Legacy behavior: every field renders its full control at all times.
    Inline,
    /// Compact display: each field shows label + value on one line.
    /// Enter on focused field opens edit mode (full control for that field only).
    /// Enter/Esc returns to display with saved/cancelled value.
    DisplayEdit,
    /// Full-screen edit: display mode identical to DisplayEdit (compact one-liners).
    /// Edit mode hides all other fields and renders a dedicated panel with
    /// label header, full control widget, and helper footer.
    FullScreenEdit,
}

/// Top-level form state — a list of fields with focus tracking.
#[derive(Debug, Clone)]
pub struct FormModel {
    pub fields: Vec<Field>,
    pub focused: usize,
    pub scroll_offset: usize,
    pub viewport_height: usize,
    pub mode: FormMode,
    /// Named sections grouping fields. Always non-empty — flat fields
    /// get a single implicit group with an empty label.
    pub groups: Vec<FieldGroup>,
    /// Side effects pending caller fulfillment. Drain after each `update()`.
    pub pending_effects: Vec<FormEffect>,
}

impl FormModel {
    /// Create a new form from a list of fields. Focus starts on the first visible field.
    /// All fields are placed in a single implicit group (no visible header).
    /// Defaults to `FormMode::Inline` (legacy behavior).
    pub fn new(fields: Vec<Field>) -> Self {
        let indices: Vec<usize> = (0..fields.len()).collect();
        let groups = vec![FieldGroup::new("", indices)];
        let focused = fields
            .iter()
            .position(|f| f.visible && f.is_focusable())
            .unwrap_or(0);
        Self {
            fields,
            focused,
            scroll_offset: 0,
            viewport_height: 20,
            mode: FormMode::Inline,
            groups,
            pending_effects: Vec::new(),
        }
    }

    /// Create a form with explicit named groups.
    ///
    /// Each `(label, fields)` pair becomes a `FieldGroup`. Groups render
    /// section headers in DisplayEdit and FullScreenEdit modes.
    pub fn with_groups(groups: Vec<(&str, Vec<Field>)>) -> Self {
        let mut all_fields: Vec<Field> = Vec::new();
        let mut field_groups: Vec<FieldGroup> = Vec::new();

        for (label, fields) in groups {
            let start = all_fields.len();
            let count = fields.len();
            all_fields.extend(fields);
            let indices: Vec<usize> = (start..start + count).collect();
            field_groups.push(FieldGroup::new(label, indices));
        }

        let focused = all_fields
            .iter()
            .position(|f| f.visible && f.is_focusable())
            .unwrap_or(0);
        Self {
            fields: all_fields,
            focused,
            scroll_offset: 0,
            viewport_height: 20,
            mode: FormMode::Inline,
            groups: field_groups,
            pending_effects: Vec::new(),
        }
    }

    /// Set the form interaction mode.
    pub fn with_mode(mut self, mode: FormMode) -> Self {
        self.mode = mode;
        self
    }

    /// Whether the form is in display mode (DisplayEdit/FullScreenEdit with no field editing).
    pub fn is_display(&self) -> bool {
        matches!(self.mode, FormMode::DisplayEdit | FormMode::FullScreenEdit)
            && self
                .focused_field()
                .is_some_and(|f| matches!(f.state, FieldState::Idle))
    }

    /// Whether the form is in edit mode (DisplayEdit/FullScreenEdit with focused field editing).
    pub fn is_editing(&self) -> bool {
        matches!(self.mode, FormMode::DisplayEdit | FormMode::FullScreenEdit)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, multiselect, note, number, select, text};

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

    // --- Note field tests ---

    #[test]
    fn test_note_skipped_by_focus_next() {
        let form = FormModel::new(vec![
            text("a").label("First").build(),
            note("info", "Read-only text").build(),
            text("b").label("Second").build(),
        ]);
        assert_eq!(form.focused, 0);
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 2, "should skip note at index 1");
    }

    #[test]
    fn test_note_skipped_by_focus_prev() {
        let form = FormModel::new(vec![
            text("a").label("First").build(),
            note("info", "Read-only text").build(),
            text("b").label("Second").build(),
        ]);
        let form = update(form, FormMessage::FocusNext); // move to index 2
        assert_eq!(form.focused, 2);
        let form = update(form, FormMessage::FocusPrev);
        assert_eq!(form.focused, 0, "should skip note at index 1");
    }

    #[test]
    fn test_note_start_edit_is_noop() {
        let form = FormModel::new(vec![note("info", "text").build()]);
        // Note won't get focus normally, but test the dispatch directly
        let form = update(form, FormMessage::StartEdit);
        assert_eq!(form.fields[0].state, FieldState::Idle);
    }

    #[test]
    fn test_note_initial_focus_skips_note() {
        let form = FormModel::new(vec![
            note("info", "Header text").build(),
            text("name").label("Name").build(),
        ]);
        assert_eq!(form.focused, 1, "initial focus should skip note at index 0");
    }

    // --- FullScreenEdit mode tests ---

    #[test]
    fn test_fullscreen_edit_is_display_initially() {
        let form = make_form().with_mode(FormMode::FullScreenEdit);
        assert!(form.is_display());
        assert!(!form.is_editing());
    }

    #[test]
    fn test_fullscreen_edit_enter_starts_editing() {
        let form = FormModel::new(vec![text("x").value("hello").build()])
            .with_mode(FormMode::FullScreenEdit);
        let form = update(form, FormMessage::StartEdit);
        assert!(form.is_editing());
        assert!(!form.is_display());
    }

    #[test]
    fn test_fullscreen_edit_commit_returns_to_display() {
        let form = FormModel::new(vec![text("x").value("old").build()])
            .with_mode(FormMode::FullScreenEdit);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::EditChar('!'));
        let form = update(form, FormMessage::CommitEdit);
        assert!(form.is_display());
        assert_eq!(form.fields[0].value, "old!");
    }

    #[test]
    fn test_fullscreen_edit_blocks_navigation_while_editing() {
        let form = FormModel::new(vec![
            text("a").label("First").build(),
            text("b").label("Second").build(),
        ])
        .with_mode(FormMode::FullScreenEdit);
        let form = update(form, FormMessage::StartEdit);
        assert!(form.is_editing());
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 0, "should not navigate while editing");
    }

    // --- MultiSelect tests ---

    #[test]
    fn test_multiselect_start_edit() {
        let form = FormModel::new(vec![
            multiselect("tags", &[("a", "A"), ("b", "B"), ("c", "C")])
                .value("a,c")
                .build(),
        ]);
        let form = update(form, FormMessage::StartEdit);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing {
                highlight,
                selected,
            } => {
                assert_eq!(*highlight, 0);
                assert_eq!(selected, &[0, 2]);
            }
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
    }

    #[test]
    fn test_multiselect_toggle() {
        let form = FormModel::new(vec![multiselect("tags", &[("a", "A"), ("b", "B")]).build()]);
        let form = update(form, FormMessage::StartEdit);
        // Toggle first option on
        let form = update(form, FormMessage::MultiSelectToggle);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing { selected, .. } => {
                assert_eq!(selected, &[0]);
            }
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
        // Toggle first option off
        let form = update(form, FormMessage::MultiSelectToggle);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing { selected, .. } => {
                assert!(selected.is_empty());
            }
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
    }

    #[test]
    fn test_multiselect_highlight_wraps() {
        let form = FormModel::new(vec![multiselect("tags", &[("a", "A"), ("b", "B")]).build()]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::MultiSelectHighlightNext);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing { highlight, .. } => assert_eq!(*highlight, 1),
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
        let form = update(form, FormMessage::MultiSelectHighlightNext);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing { highlight, .. } => assert_eq!(*highlight, 0),
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
    }

    #[test]
    fn test_multiselect_highlight_prev_wraps() {
        let form = FormModel::new(vec![multiselect("tags", &[("a", "A"), ("b", "B")]).build()]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::MultiSelectHighlightPrev);
        match &form.fields[0].state {
            FieldState::MultiSelectEditing { highlight, .. } => assert_eq!(*highlight, 1),
            other => panic!("expected MultiSelectEditing, got {other:?}"),
        }
    }

    #[test]
    fn test_multiselect_confirm_saves() {
        let form = FormModel::new(vec![
            multiselect("tags", &[("a", "A"), ("b", "B"), ("c", "C")]).build(),
        ]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::MultiSelectToggle); // toggle a
        let form = update(form, FormMessage::MultiSelectHighlightNext);
        let form = update(form, FormMessage::MultiSelectHighlightNext);
        let form = update(form, FormMessage::MultiSelectToggle); // toggle c
        let form = update(form, FormMessage::MultiSelectConfirm);
        assert_eq!(form.fields[0].state, FieldState::Idle);
        assert_eq!(form.fields[0].value, "a,c");
    }

    #[test]
    fn test_multiselect_cancel_discards() {
        let form = FormModel::new(vec![
            multiselect("tags", &[("a", "A"), ("b", "B")])
                .value("a")
                .build(),
        ]);
        let form = update(form, FormMessage::StartEdit);
        let form = update(form, FormMessage::MultiSelectToggle); // toggle a off
        let form = update(form, FormMessage::CancelEdit);
        assert_eq!(form.fields[0].state, FieldState::Idle);
        assert_eq!(form.fields[0].value, "a", "should revert to original");
    }

    // --- Field grouping tests ---

    #[test]
    fn test_new_creates_implicit_group() {
        let form = make_form();
        assert_eq!(form.groups.len(), 1);
        assert!(!form.groups[0].has_header(), "implicit group has no header");
        assert_eq!(form.groups[0].field_indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_with_groups_creates_named_groups() {
        let form = FormModel::with_groups(vec![
            ("General", vec![text("a").label("A").build()]),
            ("Advanced", vec![text("b").label("B").build()]),
        ]);
        assert_eq!(form.groups.len(), 2);
        assert_eq!(form.groups[0].label, "General");
        assert_eq!(form.groups[0].field_indices, vec![0]);
        assert_eq!(form.groups[1].label, "Advanced");
        assert_eq!(form.groups[1].field_indices, vec![1]);
        assert_eq!(form.fields.len(), 2);
    }

    #[test]
    fn test_with_groups_flattens_fields() {
        let form = FormModel::with_groups(vec![
            ("A", vec![text("x").build(), text("y").build()]),
            ("B", vec![text("z").build()]),
        ]);
        assert_eq!(form.fields.len(), 3);
        assert_eq!(form.fields[0].id, "x");
        assert_eq!(form.fields[1].id, "y");
        assert_eq!(form.fields[2].id, "z");
        assert_eq!(form.groups[0].field_indices, vec![0, 1]);
        assert_eq!(form.groups[1].field_indices, vec![2]);
    }

    #[test]
    fn test_with_groups_focus_crosses_groups() {
        let form = FormModel::with_groups(vec![
            ("A", vec![text("x").build()]),
            ("B", vec![text("y").build()]),
        ]);
        assert_eq!(form.focused, 0);
        let form = update(form, FormMessage::FocusNext);
        assert_eq!(form.focused, 1, "should cross group boundary");
    }

    #[test]
    fn test_with_groups_value_lookup() {
        let form = FormModel::with_groups(vec![
            ("A", vec![text("x").value("hello").build()]),
            ("B", vec![text("y").value("world").build()]),
        ]);
        assert_eq!(form.value("x"), Some("hello"));
        assert_eq!(form.value("y"), Some("world"));
    }
}
