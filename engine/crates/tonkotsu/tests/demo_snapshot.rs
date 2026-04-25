//! Snapshot tests for the tonkotsu kitchen-sink demo.
//!
//! Tests field construction, form rendering, validation behavior,
//! help bar output, and demo model key handling.
//! Uses ratatui TestBackend for rendered output assertions.

mod helpers;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use helpers::*;

use tonkotsu::demo::fields::build_fields;
use tonkotsu::demo::help_bar::render_help_bar;
use tonkotsu::field::{FieldKind, FieldState};
use tonkotsu::{DefaultTheme, FormModel};

const W: u16 = 80;
const H: u16 = 40;

// ═══════════════════════════════════════════════════════════
// Field Construction
// ═══════════════════════════════════════════════════════════

#[test]
fn kitchen_sink_has_eleven_fields() {
    let fields = build_fields();
    assert_eq!(fields.len(), 11);
}

#[test]
fn kitchen_sink_first_is_required_text() {
    let fields = build_fields();
    let first = &fields[0];
    assert_eq!(first.id, "recipe_name");
    assert!(matches!(first.kind, FieldKind::Text { .. }));
    assert!(
        first.validator.is_some(),
        "first field should have required validator"
    );
}

#[test]
fn kitchen_sink_has_filterable_select() {
    let fields = build_fields();
    let filterable = fields.iter().find(|f| {
        matches!(
            &f.kind,
            FieldKind::Select {
                filterable: true,
                ..
            }
        )
    });
    assert!(
        filterable.is_some(),
        "should have at least one filterable select field"
    );
    assert_eq!(filterable.unwrap().id, "category");
}

#[test]
fn kitchen_sink_has_number_with_suffix() {
    let fields = build_fields();
    let quality = fields.iter().find(|f| f.id == "quality").unwrap();
    match &quality.kind {
        FieldKind::Number { suffix, .. } => {
            assert_eq!(suffix.as_deref(), Some("%"));
        }
        _ => panic!("quality should be a Number field"),
    }
}

#[test]
fn kitchen_sink_last_is_hidden() {
    let fields = build_fields();
    let last = &fields[10];
    assert_eq!(last.id, "hidden_field");
    assert!(!last.visible);
}

// ═══════════════════════════════════════════════════════════
// Form Rendering
// ═══════════════════════════════════════════════════════════

#[test]
fn initial_render_shows_focused_marker() {
    let model = FormModel::new(build_fields());
    let buf = render_to_buffer(&model, W, H);
    // First field should have the ">" focus prefix
    assert_line_contains(&buf, 0, "> ");
}

#[test]
fn initial_render_shows_all_visible_labels() {
    let model = FormModel::new(build_fields()).with_viewport(H as usize);
    let buf = render_to_buffer(&model, W, H);
    let text = buffer_text(&buf);

    let visible_labels = [
        "Recipe Name",
        "Description",
        "Output Format",
        "Category",
        "Overwrite Existing?",
        "Quality",
        "Max Width",
        "Notification Email",
        "Min File Size",
        "Input File",
    ];

    for label in &visible_labels {
        assert!(
            text.contains(label),
            "buffer should contain label {label:?}. buffer:\n{text}"
        );
    }
}

#[test]
fn initial_render_hides_invisible() {
    let model = FormModel::new(build_fields());
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_not_contains(&buf, "Hidden Secret");
}

#[test]
fn initial_render_shows_description() {
    let model = FormModel::new(build_fields());
    let buf = render_to_buffer(&model, W, H);
    // Focused field (recipe_name) description should be visible
    assert_buffer_contains(&buf, "A short name for your recipe");
}

#[test]
fn scroll_to_last_field_shows_it() {
    let mut model = FormModel::new(build_fields()).with_viewport(6);
    // Tab to the last visible field (index 9 = input_file, skipping hidden at 10)
    for _ in 0..9 {
        model = tonkotsu::update(model, tonkotsu::FormMessage::FocusNext);
    }
    let buf = render_to_buffer(&model, W, 6);
    assert_buffer_contains(&buf, "Input File");
}

// ═══════════════════════════════════════════════════════════
// Validation in Context
// ═══════════════════════════════════════════════════════════

#[test]
fn required_field_error_on_empty_commit() {
    let model = FormModel::new(build_fields());
    // Start editing recipe_name (already focused), commit empty → error
    let model = simulate_keys(model, &[key(KeyCode::Enter), key(KeyCode::Enter)]);
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Cannot be empty");
}

#[test]
fn pattern_validator_rejects_no_at() {
    let mut model = FormModel::new(build_fields()).with_viewport(H as usize);
    // Focus email field (index 7)
    for _ in 0..7 {
        model = tonkotsu::update(model, tonkotsu::FormMessage::FocusNext);
    }
    assert_eq!(model.fields[model.focused].id, "email");

    // Enter edit, type "nope", commit → pattern error
    let model = simulate_keys(
        model,
        &[
            key(KeyCode::Enter),
            char_key('n'),
            char_key('o'),
            char_key('p'),
            char_key('e'),
            key(KeyCode::Enter),
        ],
    );
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Must match pattern");
}

#[test]
fn number_error_on_non_numeric() {
    let mut model = FormModel::new(build_fields());
    // Focus quality field (index 5)
    for _ in 0..5 {
        model = tonkotsu::update(model, tonkotsu::FormMessage::FocusNext);
    }
    assert_eq!(model.fields[model.focused].id, "quality");

    // Enter edit, clear buffer, type "abc", commit → error
    let model = simulate_keys(
        model,
        &[
            key(KeyCode::Enter),
            key(KeyCode::Backspace),
            key(KeyCode::Backspace),
            char_key('a'),
            char_key('b'),
            char_key('c'),
            key(KeyCode::Enter),
        ],
    );
    assert_eq!(
        model.fields[model.focused].error.as_deref(),
        Some("Must be a number")
    );
}

// ═══════════════════════════════════════════════════════════
// Help Bar
// ═══════════════════════════════════════════════════════════

#[test]
fn help_bar_shows_version() {
    let theme = DefaultTheme;
    let line = render_help_bar(10, &theme);
    let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
    assert!(
        text.contains(&format!("v{}", env!("CARGO_PKG_VERSION"))),
        "help bar should contain version. got: {text}"
    );
}

#[test]
fn help_bar_shows_field_count() {
    let theme = DefaultTheme;
    let line = render_help_bar(11, &theme);
    let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
    assert!(
        text.contains("11 fields"),
        "help bar should show '11 fields'. got: {text}"
    );
}

#[test]
fn help_bar_shows_key_hints() {
    let theme = DefaultTheme;
    let line = render_help_bar(10, &theme);
    let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
    assert!(text.contains("Tab"), "should contain Tab hint. got: {text}");
    assert!(text.contains("Esc"), "should contain Esc hint. got: {text}");
    assert!(
        text.contains("Ctrl+R"),
        "should contain Ctrl+R hint. got: {text}"
    );
}

// ═══════════════════════════════════════════════════════════
// Demo Model
// ═══════════════════════════════════════════════════════════

// These tests use the binary's model module directly. Since the binary
// code isn't accessible from tests/, we test the underlying behavior
// through the form's public API instead.

#[test]
fn esc_in_idle_does_not_edit() {
    // When the focused field is idle, Esc should NOT map to a form message
    // (the demo binary catches it as Quit before it reaches the form)
    let model = FormModel::new(build_fields());
    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    let msg = tonkotsu::map_key_event(esc, &model);
    // In idle, Esc is not a form-level message (form doesn't handle quit)
    assert!(
        msg.is_none(),
        "Esc in idle should not produce a FormMessage"
    );
}

#[test]
fn esc_in_editing_cancels_not_quits() {
    let model = FormModel::new(build_fields());
    // Enter edit mode
    let model = simulate_key(model, key(KeyCode::Enter));
    assert!(matches!(
        model.fields[0].state,
        FieldState::TextEditing { .. }
    ));

    // Esc during editing should produce CancelEdit
    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    let msg = tonkotsu::map_key_event(esc, &model);
    assert_eq!(msg, Some(tonkotsu::FormMessage::CancelEdit));
}

#[test]
fn ctrl_r_in_idle_resets_default() {
    // Ctrl+R in idle maps to ResetDefault at the form level
    let model = FormModel::new(build_fields());
    let ctrl_r = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL);
    let msg = tonkotsu::map_key_event(ctrl_r, &model);
    assert_eq!(msg, Some(tonkotsu::FormMessage::ResetDefault));
}

#[test]
fn char_key_in_idle_does_nothing() {
    // Regular character keys shouldn't produce messages in idle (text fields)
    let model = FormModel::new(build_fields());
    let msg = tonkotsu::map_key_event(char_key('a'), &model);
    assert!(
        msg.is_none(),
        "char 'a' in idle text field should not produce a message"
    );
}
