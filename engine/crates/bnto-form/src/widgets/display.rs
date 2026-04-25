//! Display-mode widget — renders any field as a compact one-liner.
//!
//! Used when `FormMode::DisplayEdit` is active and the field is in `Idle` state.
//! Format: `> Label: value` (focused) or `  Label: value` (unfocused).

use ratatui::text::{Line, Span};

use crate::field::Field;
use crate::theme::FormTheme;

/// Render a field as a compact display line: label + formatted value.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };
    let value_style = if focused { theme.text() } else { theme.muted() };

    let display = field.display_value();
    let value_part = if display.is_empty() {
        Span::styled("(empty)", theme.muted())
    } else {
        Span::styled(display, value_style)
    };

    vec![Line::from(vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}: ", field.label), label_style),
        value_part,
    ])]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{confirm, number, select, text};
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn test_display_text_field() {
        let field = text("name").label("Name").value("My Recipe").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 1);
        let text = line_text(&lines[0]);
        assert!(text.contains("Name:"), "got: {text}");
        assert!(text.contains("My Recipe"), "got: {text}");
    }

    #[test]
    fn test_display_number_with_suffix() {
        let field = number("q").label("Quality").suffix("%").value("80").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("Quality:"), "got: {text}");
        assert!(text.contains("80%"), "got: {text}");
    }

    #[test]
    fn test_display_select_shows_label() {
        let field = select("fmt", &[("jpeg", "JPEG"), ("png", "PNG")])
            .label("Format")
            .value("jpeg")
            .build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("JPEG"), "should show label, got: {text}");
    }

    #[test]
    fn test_display_confirm_shows_affirmative() {
        let field = confirm("strip").label("Strip EXIF").value("true").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("Yes"), "got: {text}");
    }

    #[test]
    fn test_display_focused_prefix() {
        let field = text("name").label("Name").value("x").build();
        let lines = render(&field, true, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_display_unfocused_prefix() {
        let field = text("name").label("Name").value("x").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }

    #[test]
    fn test_display_empty_value() {
        let field = text("name").label("Name").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("(empty)"), "got: {text}");
    }
}
