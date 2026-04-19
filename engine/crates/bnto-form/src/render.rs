//! Top-level form renderer.
//!
//! Composes individual field widgets into a complete form layout.
//! Returns `Vec<Line>` for the caller to render into their terminal frame.
//! Supports scroll/viewport — auto-scrolls the focused field into view.

use ratatui::text::{Line, Span};

use crate::form::FormModel;
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
    // First pass: collect all field lines with their field indices
    let mut all_lines: Vec<Line<'static>> = Vec::new();
    let mut field_start_lines: Vec<(usize, usize)> = Vec::new(); // (field_idx, start_line)

    for (i, field) in model.fields.iter().enumerate() {
        if !field.visible {
            continue;
        }
        let focused = i == model.focused;
        let start = all_lines.len();

        let field_lines = match &field.kind {
            crate::field::FieldKind::Text { .. } => {
                widgets::text_input::render(field, focused, theme)
            }
            crate::field::FieldKind::Select { .. } => {
                widgets::select::render(field, focused, theme)
            }
            crate::field::FieldKind::Confirm { .. } => {
                widgets::confirm::render(field, focused, theme)
            }
            crate::field::FieldKind::Number { .. } => {
                widgets::number::render(field, focused, theme)
            }
        };
        all_lines.extend(field_lines);

        // Description shown only for focused field
        if focused && let Some(ref desc) = field.description {
            all_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(desc.clone(), theme.muted()),
            ]));
        }

        field_start_lines.push((i, start));
    }

    // Apply viewport scrolling
    if model.viewport_height == 0 || all_lines.len() <= model.viewport_height {
        return all_lines;
    }

    // Find the focused field's start line
    let focused_start = field_start_lines
        .iter()
        .find(|(idx, _)| *idx == model.focused)
        .map(|(_, start)| *start)
        .unwrap_or(0);

    // Auto-scroll: ensure focused field is within viewport
    let mut offset = model.scroll_offset;

    // If focused field is above viewport, scroll up
    if focused_start < offset {
        offset = focused_start;
    }

    // If focused field is below viewport, scroll down
    if focused_start >= offset + model.viewport_height {
        offset = focused_start.saturating_sub(model.viewport_height - 1);
    }

    // Clamp offset to valid range
    let max_offset = all_lines.len().saturating_sub(model.viewport_height);
    offset = offset.min(max_offset);

    let end = (offset + model.viewport_height).min(all_lines.len());
    all_lines[offset..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, number, text};
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
    fn test_description_not_shown_when_unfocused() {
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
            !all_text.contains("Enter a name"),
            "unfocused field description should not show"
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
}
