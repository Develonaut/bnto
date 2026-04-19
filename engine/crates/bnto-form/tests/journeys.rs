//! Journey tests for bnto-form.
//!
//! Each test simulates a real user scenario end-to-end:
//! key press → message → state update → render to terminal buffer → verify output.
//! Uses ratatui's `TestBackend` so we assert on the exact same rendering path
//! the real TUI uses.

mod helpers;

use crossterm::event::KeyCode;
use helpers::*;

use bnto_form::{FieldState, FormModel, confirm, number, select, text};

const W: u16 = 60;
const H: u16 = 10;

// ═══════════════════════════════════════════════════════════
// Navigation
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_tab_navigation() {
    let model = FormModel::new(vec![
        text("a").label("First").value("one").build(),
        text("b").label("Second").value("two").build(),
        text("c").label("Third").value("three").build(),
    ]);

    let buf = render_to_buffer(&model, W, H);
    assert_line_contains(&buf, 0, "> ");
    assert_line_contains(&buf, 0, "First");

    // Tab to second field (row 2 — blank spacer line between fields)
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 1);
    let buf = render_to_buffer(&model, W, H);
    assert_line_contains(&buf, 2, "> ");
    assert_line_contains(&buf, 2, "Second");

    // Tab to third (row 4)
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 2);
    let buf = render_to_buffer(&model, W, H);
    assert_line_contains(&buf, 4, "> ");

    // Tab wraps to first
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 0);
}

#[test]
fn journey_shift_tab_backward() {
    let model = FormModel::new(vec![
        text("a").label("First").build(),
        text("b").label("Second").build(),
        text("c").label("Third").build(),
    ]);

    // Shift+Tab from first wraps to last
    let model = simulate_key(model, shift_tab());
    assert_eq!(model.focused, 2);

    // Shift+Tab goes to second
    let model = simulate_key(model, shift_tab());
    assert_eq!(model.focused, 1);
}

#[test]
fn journey_hidden_field_skipped() {
    let model = FormModel::new(vec![
        text("a").label("Visible1").value("yes").build(),
        text("b").label("Hidden").value("no").visible(false).build(),
        text("c").label("Visible2").value("yes").build(),
    ]);

    // Hidden field should not appear in render
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_not_contains(&buf, "Hidden");

    // Tab skips hidden field (0 → 2)
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 2);
}

// ═══════════════════════════════════════════════════════════
// Text Input
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_text_edit_commit() {
    let model = FormModel::new(vec![text("name").label("Name").value("old").build()]);

    // Enter edit mode
    let model = simulate_key(model, key(KeyCode::Enter));
    assert!(matches!(
        model.fields[0].state,
        FieldState::TextEditing { .. }
    ));

    // Type "!" at end
    let model = simulate_key(model, char_key('!'));
    // Verify cursor is visible in buffer (white-bg character)
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "old!");

    // Commit
    let model = simulate_key(model, key(KeyCode::Enter));
    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "old!");
}

#[test]
fn journey_text_edit_cancel() {
    let model = FormModel::new(vec![text("name").label("Name").value("original").build()]);

    let model = simulate_key(model, key(KeyCode::Enter));
    let model = simulate_key(model, char_key('X'));
    let model = simulate_key(model, key(KeyCode::Esc));

    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "original");
}

#[test]
fn journey_char_limit_rejects() {
    let model = FormModel::new(vec![
        text("code").label("Code").char_limit(3).value("ab").build(),
    ]);

    let model = simulate_key(model, key(KeyCode::Enter));
    let model = simulate_key(model, char_key('c'));
    // At limit (3 chars) — next char should be rejected
    let model = simulate_key(model, char_key('d'));
    let model = simulate_key(model, key(KeyCode::Enter));

    assert_eq!(model.fields[0].value, "abc");
}

#[test]
fn journey_word_navigation() {
    let model = FormModel::new(vec![
        text("txt").label("Text").value("hello world foo").build(),
    ]);

    // Enter edit mode (cursor at end = 15)
    let model = simulate_key(model, key(KeyCode::Enter));
    // Ctrl+Left to jump back one word
    let model = simulate_key(model, ctrl(KeyCode::Left));
    match &model.fields[0].state {
        FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 12), // start of "foo"
        _ => panic!("expected TextEditing"),
    }

    // Ctrl+Left again to "world"
    let model = simulate_key(model, ctrl(KeyCode::Left));
    match &model.fields[0].state {
        FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 6),
        _ => panic!("expected TextEditing"),
    }

    // Ctrl+Right forward to "foo"
    let model = simulate_key(model, ctrl(KeyCode::Right));
    match &model.fields[0].state {
        FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 12),
        _ => panic!("expected TextEditing"),
    }
}

#[test]
fn journey_type_full_value() {
    let model = FormModel::new(vec![text("name").label("Name").build()]);

    // Enter edit, type "hi", backspace, type "ello", commit
    let model = simulate_keys(
        model,
        &[
            key(KeyCode::Enter),
            char_key('h'),
            char_key('i'),
            key(KeyCode::Backspace),
            char_key('e'),
            char_key('l'),
            char_key('l'),
            char_key('o'),
            key(KeyCode::Enter),
        ],
    );

    assert_eq!(model.fields[0].value, "hello");
}

// ═══════════════════════════════════════════════════════════
// Confirm
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_confirm_toggle_space() {
    let model = FormModel::new(vec![confirm("ok").label("Proceed?").value("false").build()]);

    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "[ No ]");

    let model = simulate_key(model, char_key(' '));
    assert_eq!(model.fields[0].value, "true");

    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "[ Yes ]");

    let model = simulate_key(model, char_key(' '));
    assert_eq!(model.fields[0].value, "false");
}

#[test]
fn journey_confirm_y_n_shortcuts() {
    let model = FormModel::new(vec![confirm("ok").label("OK?").value("false").build()]);

    let model = simulate_key(model, char_key('y'));
    assert_eq!(model.fields[0].value, "true");

    let model = simulate_key(model, char_key('n'));
    assert_eq!(model.fields[0].value, "false");
}

// ═══════════════════════════════════════════════════════════
// Select
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_select_cycle() {
    let model = FormModel::new(vec![
        select("fmt", &[("a", "Alpha"), ("b", "Beta"), ("c", "Gamma")])
            .value("a")
            .build(),
    ]);

    // Right cycles forward
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.fields[0].value, "b");

    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.fields[0].value, "c");

    // Wraps around
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.fields[0].value, "a");

    // Left wraps backward
    let model = simulate_key(model, key(KeyCode::Left));
    assert_eq!(model.fields[0].value, "c");
}

#[test]
fn journey_select_expand_confirm() {
    let model = FormModel::new(vec![
        select("fmt", &[("a", "Alpha"), ("b", "Beta"), ("c", "Gamma")])
            .value("a")
            .build(),
    ]);

    // Enter expands list
    let model = simulate_key(model, key(KeyCode::Enter));
    assert!(matches!(
        model.fields[0].state,
        FieldState::SelectExpanded { .. }
    ));

    // Down to highlight "Beta" (index 1)
    let model = simulate_key(model, key(KeyCode::Down));
    if let FieldState::SelectExpanded { highlight, .. } = &model.fields[0].state {
        assert_eq!(*highlight, 1);
    } else {
        panic!("expected SelectExpanded");
    }

    // Enter confirms selection
    let model = simulate_key(model, key(KeyCode::Enter));
    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "b");
}

#[test]
fn journey_select_expand_cancel() {
    let model = FormModel::new(vec![
        select("fmt", &[("a", "Alpha"), ("b", "Beta")])
            .value("a")
            .build(),
    ]);

    let model = simulate_key(model, key(KeyCode::Enter));
    let model = simulate_key(model, key(KeyCode::Down));
    // Esc cancels — keeps original value
    let model = simulate_key(model, key(KeyCode::Esc));
    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "a");
}

// ═══════════════════════════════════════════════════════════
// Number
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_number_step_clamp() {
    let model = FormModel::new(vec![
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .step(10.0)
            .value("90")
            .build(),
    ]);

    // Step up — should clamp at 100
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.fields[0].value, "100");

    // Another step up stays at 100
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.fields[0].value, "100");

    // Step down
    let model = simulate_key(model, key(KeyCode::Left));
    assert_eq!(model.fields[0].value, "90");
}

#[test]
fn journey_number_edit_commit() {
    let model = FormModel::new(vec![
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("50")
            .build(),
    ]);

    // Enter text-entry mode
    let model = simulate_key(model, key(KeyCode::Enter));
    assert!(matches!(
        model.fields[0].state,
        FieldState::NumberEditing { .. }
    ));

    // Clear the buffer and type "75"
    // First select all (Home then shift-delete isn't mapped, so let's backspace twice then type)
    let model = simulate_keys(
        model,
        &[
            key(KeyCode::Backspace),
            key(KeyCode::Backspace),
            char_key('7'),
            char_key('5'),
            key(KeyCode::Enter),
        ],
    );

    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "75");
}

#[test]
fn journey_number_edit_reject_invalid() {
    let model = FormModel::new(vec![number("q").label("Quality").value("50").build()]);

    // Enter edit, clear buffer, type "abc", commit — should show error
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

    assert!(model.fields[0].error.is_some());
    assert_eq!(model.fields[0].error.as_deref(), Some("Must be a number"));
}

#[test]
fn journey_number_edit_clamps() {
    let model = FormModel::new(vec![
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("50")
            .build(),
    ]);

    // Enter edit, clear, type "200", commit — should clamp to 100
    let model = simulate_keys(
        model,
        &[
            key(KeyCode::Enter),
            key(KeyCode::Backspace),
            key(KeyCode::Backspace),
            char_key('2'),
            char_key('0'),
            char_key('0'),
            key(KeyCode::Enter),
        ],
    );

    assert_eq!(model.fields[0].state, FieldState::Idle);
    assert_eq!(model.fields[0].value, "100");
}

// ═══════════════════════════════════════════════════════════
// Select — Rendering
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_select_render_compact_idle() {
    let model = FormModel::new(vec![
        select("fmt", &[("jpeg", "JPEG"), ("png", "PNG")])
            .label("Format")
            .value("jpeg")
            .build(),
    ]);

    // Focused: shows arrows around current label
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "< JPEG >");
}

#[test]
fn journey_select_render_expanded_list() {
    let model = FormModel::new(vec![
        select("fmt", &[("jpeg", "JPEG"), ("png", "PNG"), ("webp", "WebP")])
            .label("Format")
            .value("jpeg")
            .build(),
    ]);

    // Expand the select
    let model = simulate_key(model, key(KeyCode::Enter));
    let buf = render_to_buffer(&model, W, H);

    // Should show the filter bar and all options
    assert_buffer_contains(&buf, "Filter:");
    assert_buffer_contains(&buf, "JPEG");
    assert_buffer_contains(&buf, "PNG");
    assert_buffer_contains(&buf, "WebP");
}

#[test]
fn journey_select_filter_renders() {
    let model = FormModel::new(vec![
        select("fmt", &[("jpeg", "JPEG"), ("png", "PNG"), ("webp", "WebP")])
            .label("Format")
            .value("jpeg")
            .build(),
    ]);

    // Expand and type filter "pn"
    let model = simulate_keys(model, &[key(KeyCode::Enter), char_key('p'), char_key('n')]);
    let buf = render_to_buffer(&model, W, H);

    // Only PNG should match
    assert_buffer_contains(&buf, "PNG");
    assert_buffer_contains(&buf, "(1 of 3)");
}

// ═══════════════════════════════════════════════════════════
// Number — Rendering
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_number_render_slider() {
    let model = FormModel::new(vec![
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .suffix("%")
            .value("50")
            .build(),
    ]);

    let buf = render_to_buffer(&model, W, H);
    let text = buffer_text(&buf);
    // Should show slider bar characters
    assert!(text.contains('█'), "should have filled blocks: {text}");
    assert!(text.contains('░'), "should have empty blocks: {text}");
    assert!(
        text.contains("50%"),
        "should show value with suffix: {text}"
    );
}

#[test]
fn journey_number_edit_renders_cursor() {
    let model = FormModel::new(vec![
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("80")
            .build(),
    ]);

    // Enter text editing mode
    let model = simulate_key(model, key(KeyCode::Enter));
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Quality");
    assert_buffer_contains(&buf, "80");
}

// ═══════════════════════════════════════════════════════════
// Confirm — Rendering
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_confirm_render_buttons() {
    let model = FormModel::new(vec![
        confirm("ok").label("Overwrite?").value("true").build(),
    ]);

    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "[ Yes ]");
    assert_buffer_contains(&buf, "No");

    // Toggle and check reverse
    let model = simulate_key(model, char_key(' '));
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Yes");
    assert_buffer_contains(&buf, "[ No ]");
}

// ═══════════════════════════════════════════════════════════
// Cross-cutting
// ═══════════════════════════════════════════════════════════

#[test]
fn journey_reset_default() {
    let model = FormModel::new(vec![
        text("name")
            .label("Name")
            .value("changed")
            .default(Some("original"))
            .build(),
    ]);

    let model = simulate_key(model, ctrl(KeyCode::Char('r')));
    assert_eq!(model.fields[0].value, "original");

    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "original");
}

#[test]
fn journey_reset_no_default_noop() {
    let model = FormModel::new(vec![text("name").label("Name").value("hello").build()]);

    let model = simulate_key(model, ctrl(KeyCode::Char('r')));
    // No default — value unchanged
    assert_eq!(model.fields[0].value, "hello");
}

#[test]
fn journey_validation_error_renders() {
    let model = FormModel::new(vec![text("name").label("Name").required().build()]);

    // Enter edit (empty buffer), commit — validation fails, stays in editing
    let model = simulate_keys(model, &[key(KeyCode::Enter), key(KeyCode::Enter)]);
    assert_eq!(model.fields[0].error.as_deref(), Some("Cannot be empty"));
    assert!(matches!(
        model.fields[0].state,
        FieldState::TextEditing { .. }
    ));

    // Cancel edit — error renders in idle mode
    let model = simulate_key(model, key(KeyCode::Esc));
    assert_eq!(model.fields[0].state, FieldState::Idle);
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Cannot be empty");
}

#[test]
fn journey_error_clears_on_success() {
    let model = FormModel::new(vec![text("name").label("Name").required().build()]);

    // Trigger validation error
    let model = simulate_keys(model, &[key(KeyCode::Enter), key(KeyCode::Enter)]);
    assert!(model.fields[0].error.is_some());

    // Type something and commit — error should clear
    let model = simulate_keys(model, &[char_key('x'), key(KeyCode::Enter)]);
    assert!(model.fields[0].error.is_none());
    assert_eq!(model.fields[0].value, "x");

    let buf = render_to_buffer(&model, W, H);
    assert_buffer_not_contains(&buf, "Cannot be empty");
}

#[test]
fn journey_mixed_form() {
    let model = FormModel::new(vec![
        text("name").label("Name").value("Recipe").build(),
        number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .step(10.0)
            .value("80")
            .build(),
        select("fmt", &[("jpeg", "JPEG"), ("png", "PNG")])
            .label("Format")
            .value("jpeg")
            .build(),
        confirm("overwrite")
            .label("Overwrite?")
            .value("false")
            .build(),
    ]);

    // Text: edit and commit
    let model = simulate_keys(
        model,
        &[key(KeyCode::Enter), char_key('!'), key(KeyCode::Enter)],
    );
    assert_eq!(model.value("name"), Some("Recipe!"));

    // Tab to number, step up
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 1);
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.value("q"), Some("90"));

    // Tab to select, cycle
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 2);
    let model = simulate_key(model, key(KeyCode::Right));
    assert_eq!(model.value("fmt"), Some("png"));

    // Tab to confirm, toggle
    let model = simulate_key(model, key(KeyCode::Tab));
    assert_eq!(model.focused, 3);
    let model = simulate_key(model, char_key(' '));
    assert_eq!(model.value("overwrite"), Some("true"));

    // Render final state — all values should appear (widgets show labels, not raw values)
    let buf = render_to_buffer(&model, W, H);
    assert_buffer_contains(&buf, "Recipe!");
    assert_buffer_contains(&buf, "90");
    assert_buffer_contains(&buf, "PNG"); // select shows label, not value
    assert_buffer_contains(&buf, "[ Yes ]"); // confirm shows button, not "true"
}
