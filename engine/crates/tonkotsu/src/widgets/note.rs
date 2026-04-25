//! Note widget — renders read-only informational text.
//!
//! Note fields are not focusable and not editable. They display styled
//! text between other fields for context or instructions.

use ratatui::text::{Line, Span};

use crate::field::{Field, FieldKind};
use crate::theme::FormTheme;

/// Render a Note field as a styled text block.
pub fn render(field: &Field, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let content = match &field.kind {
        FieldKind::Note { content } => content.clone(),
        _ => return vec![],
    };

    vec![Line::from(vec![
        Span::raw("  "),
        Span::styled(content, theme.muted()),
    ])]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::note;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn test_note_renders_content() {
        let field = note("info", "This is informational text").build();
        let lines = render(&field, &theme());
        assert_eq!(lines.len(), 1);
        let text = line_text(&lines[0]);
        assert!(text.contains("This is informational text"), "got: {text}");
    }

    #[test]
    fn test_note_has_indentation() {
        let field = note("info", "Some text").build();
        let lines = render(&field, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }
}
