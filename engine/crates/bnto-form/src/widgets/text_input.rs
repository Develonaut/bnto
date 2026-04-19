//! TextInput widget — renders a text field as `Vec<Line>`.
//!
//! Focused + editing: shows cursor-tracked buffer with block cursor.
//! Focused + idle: shows value with focus indicator.
//! Unfocused: shows label + value (or placeholder in muted style).

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use unicode_segmentation::UnicodeSegmentation;

use crate::field::{Field, FieldKind, FieldState};
use crate::theme::FormTheme;

/// Render a text field as lines of styled spans.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };

    match &field.state {
        FieldState::TextEditing { buffer, cursor } => render_editing(
            prefix,
            &field.label,
            label_style,
            buffer,
            *cursor,
            &field.error,
        ),
        _ => render_idle(prefix, &field.label, label_style, field, theme),
    }
}

fn render_editing(
    prefix: &str,
    label: &str,
    label_style: Style,
    buffer: &str,
    cursor: usize,
    error: &Option<String>,
) -> Vec<Line<'static>> {
    let cursor_style = Style::default().bg(Color::White).fg(Color::Black);

    let mut spans = vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{label}   "), label_style),
    ];

    let graphemes: Vec<&str> = buffer.graphemes(true).collect();

    // Text before cursor
    let before: String = graphemes[..cursor].concat();
    if !before.is_empty() {
        spans.push(Span::raw(before));
    }

    // Cursor character (or block at end)
    if cursor < graphemes.len() {
        spans.push(Span::styled(graphemes[cursor].to_string(), cursor_style));
        let after: String = graphemes[cursor + 1..].concat();
        if !after.is_empty() {
            spans.push(Span::raw(after));
        }
    } else {
        spans.push(Span::styled(" ".to_string(), cursor_style));
    }

    let mut lines = vec![Line::from(spans)];

    if let Some(err) = error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), Style::default().fg(Color::Red)),
        ]));
    }

    lines
}

fn render_idle(
    prefix: &str,
    label: &str,
    label_style: Style,
    field: &Field,
    theme: &dyn FormTheme,
) -> Vec<Line<'static>> {
    let placeholder = match &field.kind {
        FieldKind::Text { placeholder, .. } => placeholder.as_deref(),
        _ => None,
    };

    let value_display = if field.value.is_empty() {
        match placeholder {
            Some(ph) => Span::styled(ph.to_string(), theme.muted()),
            None => Span::raw(String::new()),
        }
    } else {
        Span::styled(field.value.clone(), theme.text())
    };

    let mut lines = vec![Line::from(vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{label}   "), label_style),
        value_display,
    ])];

    if let Some(ref err) = field.error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), theme.error()),
        ]));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::text;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    #[test]
    fn test_render_idle_with_value() {
        let field = text("name").label("Name").value("hello").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 1);
        let text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Name"));
        assert!(text.contains("hello"));
    }

    #[test]
    fn test_render_idle_with_placeholder() {
        let field = text("name").label("Name").placeholder("Enter name").build();
        let lines = render(&field, false, &theme());
        let text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Enter name"));
    }

    #[test]
    fn test_render_focused_prefix() {
        let field = text("name").label("Name").value("x").build();
        let lines = render(&field, true, &theme());
        let first_span = &lines[0].spans[0];
        assert_eq!(first_span.content.as_ref(), "> ");
    }

    #[test]
    fn test_render_unfocused_prefix() {
        let field = text("name").label("Name").value("x").build();
        let lines = render(&field, false, &theme());
        let first_span = &lines[0].spans[0];
        assert_eq!(first_span.content.as_ref(), "  ");
    }

    #[test]
    fn test_render_editing_shows_cursor() {
        let mut field = text("name").label("Name").build();
        field.state = FieldState::TextEditing {
            buffer: "abc".to_string(),
            cursor: 1,
        };
        let lines = render(&field, true, &theme());
        let spans = &lines[0].spans;
        let cursor_span = spans.iter().find(|s| s.content.as_ref() == "b");
        assert!(cursor_span.is_some());
        let style = cursor_span.unwrap().style;
        assert_eq!(style.bg, Some(Color::White));
        assert_eq!(style.fg, Some(Color::Black));
    }

    #[test]
    fn test_render_editing_cursor_at_end() {
        let mut field = text("name").label("Name").build();
        field.state = FieldState::TextEditing {
            buffer: "abc".to_string(),
            cursor: 3,
        };
        let lines = render(&field, true, &theme());
        let spans = &lines[0].spans;
        let last_styled = spans.last().unwrap();
        assert_eq!(last_styled.content.as_ref(), " ");
        assert_eq!(last_styled.style.bg, Some(Color::White));
    }

    #[test]
    fn test_render_error_line() {
        let mut field = text("name").label("Name").build();
        field.error = Some("Required".to_string());
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 2);
        let err_text: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(err_text.contains("Required"));
    }

    #[test]
    fn test_render_empty_no_placeholder() {
        let field = text("name").label("Name").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 1);
        let text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Name"));
    }
}
