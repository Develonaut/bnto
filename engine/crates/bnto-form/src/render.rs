//! Top-level form renderer.
//!
//! Composes individual field widgets into a complete form layout.
//! Returns `Vec<Line>` for the caller to render into their terminal frame.
//! Supports scroll/viewport — auto-scrolls the focused field into view.

use ratatui::text::{Line, Span};

use crate::form::{FormMode, FormModel};
use crate::theme::FormTheme;
use crate::widgets;

/// Render the entire form as a list of styled lines.
///
/// Each visible field contributes one or more lines. The focused field
/// gets a `>` prefix. Hidden fields are skipped entirely.
///
/// When `viewport_height > 0`, the output is sliced to fit the viewport,
/// auto-scrolling so the focused field is always visible.
pub fn render_form(model: &FormModel, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    match model.mode {
        FormMode::DisplayEdit => render_display_edit(model, theme),
        FormMode::Inline => render_inline(model, theme),
    }
}

/// DisplayEdit mode: all fields as compact one-liners, except the editing field
/// which gets its full control rendered.
fn render_display_edit(model: &FormModel, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let mut all_lines: Vec<Line<'static>> = Vec::new();
    let mut field_start_lines: Vec<(usize, usize)> = Vec::new();

    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible)
        .map(|(i, _)| i)
        .collect();
    let last_visible = visible_indices.last().copied();
    let is_editing = model.is_editing();

    for (i, field) in model.fields.iter().enumerate() {
        if !field.visible {
            continue;
        }
        let focused = i == model.focused;
        let start = all_lines.len();

        let field_lines = if focused && is_editing {
            // Focused field in edit mode: render its full control
            render_field_inline(field, true, theme)
        } else {
            // All other fields (or focused field in display mode): compact display
            widgets::display::render(field, focused, theme)
        };
        all_lines.extend(field_lines);

        // Description only shown for focused field
        if focused && let Some(ref desc) = field.description {
            all_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(desc.clone(), theme.muted()),
            ]));
        }

        field_start_lines.push((i, start));

        if last_visible != Some(i) {
            all_lines.push(Line::default());
        }
    }

    apply_viewport(model, all_lines, &field_start_lines)
}

/// Inline mode (legacy): every field renders its full control.
fn render_inline(model: &FormModel, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let mut all_lines: Vec<Line<'static>> = Vec::new();
    let mut field_start_lines: Vec<(usize, usize)> = Vec::new();

    let visible_indices: Vec<usize> = model
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.visible)
        .map(|(i, _)| i)
        .collect();
    let last_visible = visible_indices.last().copied();

    for (i, field) in model.fields.iter().enumerate() {
        if !field.visible {
            continue;
        }
        let focused = i == model.focused;
        let start = all_lines.len();

        let field_lines = render_field_inline(field, focused, theme);
        all_lines.extend(field_lines);

        // Description always shown; muted style for all fields
        if let Some(ref desc) = field.description {
            all_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(desc.clone(), theme.muted()),
            ]));
        }

        field_start_lines.push((i, start));

        // Blank line between fields (not after the last one)
        if last_visible != Some(i) {
            all_lines.push(Line::default());
        }
    }

    apply_viewport(model, all_lines, &field_start_lines)
}

/// Render a single field using its type-specific full control widget.
fn render_field_inline(
    field: &crate::field::Field,
    focused: bool,
    theme: &dyn FormTheme,
) -> Vec<Line<'static>> {
    match &field.kind {
        crate::field::FieldKind::Text { .. } => widgets::text_input::render(field, focused, theme),
        crate::field::FieldKind::Select { .. } => widgets::select::render(field, focused, theme),
        crate::field::FieldKind::Confirm { .. } => widgets::confirm::render(field, focused, theme),
        crate::field::FieldKind::Number { .. } => widgets::number::render(field, focused, theme),
        crate::field::FieldKind::FilePath { .. } => {
            widgets::file_path::render(field, focused, theme)
        }
        crate::field::FieldKind::TextArea { .. } => {
            widgets::text_area::render(field, focused, theme)
        }
    }
}

/// Apply viewport scrolling, auto-scrolling the focused field into view.
fn apply_viewport(
    model: &FormModel,
    all_lines: Vec<Line<'static>>,
    field_start_lines: &[(usize, usize)],
) -> Vec<Line<'static>> {
    if model.viewport_height == 0 || all_lines.len() <= model.viewport_height {
        return all_lines;
    }

    let focused_start = field_start_lines
        .iter()
        .find(|(idx, _)| *idx == model.focused)
        .map(|(_, start)| *start)
        .unwrap_or(0);

    let mut offset = model.scroll_offset;

    if focused_start < offset {
        offset = focused_start;
    }
    if focused_start >= offset + model.viewport_height {
        offset = focused_start.saturating_sub(model.viewport_height - 1);
    }

    let max_offset = all_lines.len().saturating_sub(model.viewport_height);
    offset = offset.min(max_offset);

    let end = (offset + model.viewport_height).min(all_lines.len());
    all_lines[offset..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{FieldState, confirm, number, select, text};
    use crate::form::FormMode;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    #[test]
    fn test_render_form_multiple_fields() {
        let model = FormModel::new(vec![
            text("a").label("First").value("hello").build(),
            text("b").label("Second").value("world").build(),
        ]);
        let lines = render_form(&model, &theme());
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_render_form_skips_hidden() {
        let mut model = FormModel::new(vec![
            text("a").label("First").value("hello").build(),
            text("b").label("Second").value("world").build(),
        ]);
        model.fields[0].visible = false;
        let lines = render_form(&model, &theme());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(!all_text.contains("First"));
        assert!(all_text.contains("Second"));
    }

    #[test]
    fn test_render_form_all_field_types() {
        let model = FormModel::new(vec![
            confirm("ok").label("Confirm?").value("true").build(),
            number("q").label("Quality").value("80").build(),
        ]);
        let lines = render_form(&model, &theme());
        assert!(!lines.is_empty());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(all_text.contains("Confirm?"));
        assert!(all_text.contains("Quality"));
    }

    #[test]
    fn test_description_shown_on_focus() {
        let model = FormModel::new(vec![
            text("a")
                .label("Name")
                .value("x")
                .description(Some("Enter a name"))
                .build(),
            text("b").label("Other").value("y").build(),
        ]);
        let lines = render_form(&model, &theme());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(
            all_text.contains("Enter a name"),
            "focused field description should show"
        );
    }

    #[test]
    fn test_description_shown_when_unfocused() {
        let mut model = FormModel::new(vec![
            text("a")
                .label("Name")
                .value("x")
                .description(Some("Enter a name"))
                .build(),
            text("b").label("Other").value("y").build(),
        ]);
        model.focused = 1; // focus second field
        let lines = render_form(&model, &theme());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(
            all_text.contains("Enter a name"),
            "unfocused field description should still show"
        );
    }

    #[test]
    fn test_description_not_shown_when_none() {
        let model = FormModel::new(vec![text("a").label("Name").value("x").build()]);
        let lines = render_form(&model, &theme());
        // Only 1 line (the field itself), no description line
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_viewport_limits_output() {
        let model = FormModel::new(vec![
            text("a").label("First").value("1").build(),
            text("b").label("Second").value("2").build(),
            text("c").label("Third").value("3").build(),
            text("d").label("Fourth").value("4").build(),
            text("e").label("Fifth").value("5").build(),
        ])
        .with_viewport(3);
        let lines = render_form(&model, &theme());
        assert!(lines.len() <= 3, "viewport should limit to 3 lines");
    }

    #[test]
    fn test_auto_scroll_focused_into_view() {
        let mut model = FormModel::new(vec![
            text("a").label("First").value("1").build(),
            text("b").label("Second").value("2").build(),
            text("c").label("Third").value("3").build(),
            text("d").label("Fourth").value("4").build(),
            text("e").label("Fifth").value("5").build(),
        ])
        .with_viewport(2);
        // Focus the last field
        model.focused = 4;
        let lines = render_form(&model, &theme());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(
            all_text.contains("Fifth"),
            "focused field should be visible: {all_text}"
        );
    }

    #[test]
    fn test_with_viewport_builder() {
        let model = FormModel::new(vec![text("a").label("A").build()]).with_viewport(10);
        assert_eq!(model.viewport_height, 10);
    }

    // --- DisplayEdit mode rendering tests ---

    fn all_text(lines: &[Line]) -> String {
        lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect()
    }

    #[test]
    fn test_display_edit_renders_all_fields_compact() {
        let model = FormModel::new(vec![
            text("name").label("Name").value("My Recipe").build(),
            number("q").label("Quality").suffix("%").value("80").build(),
            select("fmt", &[("jpeg", "JPEG")])
                .label("Format")
                .value("jpeg")
                .build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        let lines = render_form(&model, &theme());
        let text = all_text(&lines);
        // All fields should render as compact display lines with "label: value"
        assert!(text.contains("Name:"), "got: {text}");
        assert!(text.contains("My Recipe"), "got: {text}");
        assert!(text.contains("Quality:"), "got: {text}");
        assert!(text.contains("80%"), "got: {text}");
        assert!(text.contains("Format:"), "got: {text}");
        assert!(text.contains("JPEG"), "got: {text}");
    }

    #[test]
    fn test_display_edit_editing_field_shows_full_control() {
        let mut model = FormModel::new(vec![
            text("name").label("Name").value("My Recipe").build(),
            text("other").label("Other").value("val").build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        // Put first field into editing state
        model.fields[0].state = FieldState::TextEditing {
            buffer: "My Recipe".to_string(),
            cursor: 9,
        };
        let lines = render_form(&model, &theme());
        let text = all_text(&lines);
        // Other field should still be compact display (label: value)
        assert!(
            text.contains("Other:"),
            "non-editing field should be compact: {text}"
        );
    }

    #[test]
    fn test_display_edit_one_line_per_field_in_display() {
        let model = FormModel::new(vec![
            text("a").label("First").value("1").build(),
            text("b").label("Second").value("2").build(),
            text("c").label("Third").value("3").build(),
        ])
        .with_mode(FormMode::DisplayEdit);
        let lines = render_form(&model, &theme());
        // 3 fields, each 1 line + 2 blank separators = 5 lines
        assert_eq!(
            lines.len(),
            5,
            "3 display fields + 2 spacers = 5 lines, got {}",
            lines.len()
        );
    }

    #[test]
    fn test_inline_mode_unchanged() {
        // Verify existing Inline mode still works exactly as before
        let model = FormModel::new(vec![
            text("a").label("First").value("hello").build(),
            number("q").label("Quality").value("80").build(),
        ]);
        assert_eq!(model.mode, FormMode::Inline);
        let lines = render_form(&model, &theme());
        let text = all_text(&lines);
        // Inline mode shows full controls (not "label: value" format)
        assert!(text.contains("First"), "got: {text}");
        assert!(text.contains("hello"), "got: {text}");
    }
}
