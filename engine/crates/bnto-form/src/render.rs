//! Top-level form renderer.
//!
//! Composes individual field widgets into a complete form layout.
//! Returns `Vec<Line>` for the caller to render into their terminal frame.

use ratatui::text::Line;

use crate::form::FormModel;
use crate::widgets;

/// Render the entire form as a list of styled lines.
///
/// Each visible field contributes one or more lines. The focused field
/// gets a `>` prefix. Hidden fields are skipped entirely.
pub fn render_form(model: &FormModel) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for (i, field) in model.fields.iter().enumerate() {
        if !field.visible {
            continue;
        }
        let focused = i == model.focused;

        let field_lines = match &field.kind {
            crate::field::FieldKind::Text { .. } => widgets::text_input::render(field, focused),
            // Other field types will be added in Wave 2
            _ => {
                let prefix = if focused { "> " } else { "  " };
                vec![Line::raw(format!(
                    "{prefix}{}: {}",
                    field.label, field.value
                ))]
            }
        };

        lines.extend(field_lines);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, number, text};

    #[test]
    fn test_render_form_multiple_fields() {
        let model = FormModel::new(vec![
            text("a").label("First").value("hello").build(),
            text("b").label("Second").value("world").build(),
        ]);
        let lines = render_form(&model);
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_render_form_skips_hidden() {
        let mut model = FormModel::new(vec![
            text("a").label("First").value("hello").build(),
            text("b").label("Second").value("world").build(),
        ]);
        model.fields[0].visible = false;
        let lines = render_form(&model);
        // Only second field should render
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(!all_text.contains("First"));
        assert!(all_text.contains("Second"));
    }

    #[test]
    fn test_render_form_fallback_for_other_types() {
        let model = FormModel::new(vec![
            confirm("ok").label("Confirm?").value("true").build(),
            number("q").label("Quality").value("80").build(),
        ]);
        let lines = render_form(&model);
        assert!(!lines.is_empty());
    }
}
