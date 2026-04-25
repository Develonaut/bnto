// TextArea widget — renders a multi-line text editor as `Vec<Line>`.
//
// Editing: shows buffer with cursor, line numbers, and scroll viewport.
// Idle: shows label + value preview (or placeholder).

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use unicode_segmentation::UnicodeSegmentation;

use crate::field::{Field, FieldKind, FieldState};
use crate::theme::FormTheme;

/// Default visible lines in the editing viewport.
const VIEWPORT_LINES: usize = 6;

/// Render a TextArea field as styled lines.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };

    match &field.state {
        FieldState::TextAreaEditing {
            buffer,
            cursor,
            line,
            scroll_offset,
        } => render_editing(
            prefix,
            label_style,
            field,
            buffer,
            *cursor,
            *line,
            *scroll_offset,
        ),
        _ => render_idle(prefix, &field.label, label_style, field, theme),
    }
}

fn render_editing(
    prefix: &str,
    label_style: Style,
    field: &Field,
    buffer: &str,
    cursor: usize,
    current_line: usize,
    scroll_offset: usize,
) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
        Span::styled(
            "(Ctrl+D to save, Esc to cancel)",
            Style::default().fg(Color::DarkGray),
        ),
    ])];

    let buffer_lines: Vec<&str> = buffer.split('\n').collect();
    let end = (scroll_offset + VIEWPORT_LINES).min(buffer_lines.len());
    let visible = &buffer_lines[scroll_offset..end];

    let cursor_style = Style::default().bg(Color::White).fg(Color::Black);
    let line_num_style = Style::default().fg(Color::DarkGray);

    for (offset, text_line) in visible.iter().enumerate() {
        let absolute_line = scroll_offset + offset;
        let line_num = format!("  {: >3} ", absolute_line + 1);

        let mut spans = vec![Span::styled(line_num, line_num_style)];

        if absolute_line == current_line {
            // This is the cursor line — render with cursor highlight
            let graphemes: Vec<&str> = text_line.graphemes(true).collect();
            let before: String = graphemes[..cursor].concat();
            if !before.is_empty() {
                spans.push(Span::raw(before));
            }
            if cursor < graphemes.len() {
                spans.push(Span::styled(graphemes[cursor].to_string(), cursor_style));
                let after: String = graphemes[cursor + 1..].concat();
                if !after.is_empty() {
                    spans.push(Span::raw(after));
                }
            } else {
                spans.push(Span::styled(" ".to_string(), cursor_style));
            }
        } else {
            spans.push(Span::raw(text_line.to_string()));
        }

        lines.push(Line::from(spans));
    }

    if let Some(ref err) = field.error {
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
        FieldKind::TextArea { placeholder, .. } => placeholder.as_deref(),
        _ => None,
    };

    let value_display = if field.value.is_empty() {
        match placeholder {
            Some(ph) => Span::styled(ph.to_string(), theme.muted()),
            None => Span::raw(String::new()),
        }
    } else {
        // Show first line + line count for multi-line
        let preview = field.display_value();
        Span::styled(preview, theme.text())
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
    use crate::field::textarea;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn spans_text(lines: &[Line]) -> String {
        lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect()
    }

    #[test]
    fn test_render_idle_with_value() {
        let field = textarea("notes").label("Notes").value("hello").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 1);
        let text = spans_text(&lines);
        assert!(text.contains("Notes"));
        assert!(text.contains("hello"));
    }

    #[test]
    fn test_render_idle_with_placeholder() {
        let field = textarea("notes")
            .label("Notes")
            .placeholder("Enter notes")
            .build();
        let lines = render(&field, false, &theme());
        let text = spans_text(&lines);
        assert!(text.contains("Enter notes"));
    }

    #[test]
    fn test_render_idle_multiline_preview() {
        let field = textarea("notes")
            .label("Notes")
            .value("line1\nline2\nline3")
            .build();
        let lines = render(&field, false, &theme());
        let text = spans_text(&lines);
        assert!(text.contains("line1"), "should show first line");
        assert!(text.contains("3 lines"), "should show line count");
    }

    #[test]
    fn test_render_editing_shows_header() {
        let mut field = textarea("notes").label("Notes").build();
        field.state = FieldState::TextAreaEditing {
            buffer: "hello".to_string(),
            cursor: 5,
            line: 0,
            scroll_offset: 0,
        };
        let lines = render(&field, true, &theme());
        let text = spans_text(&lines);
        assert!(text.contains("Notes"));
        assert!(text.contains("Ctrl+D"));
    }

    #[test]
    fn test_render_editing_shows_line_numbers() {
        let mut field = textarea("notes").label("Notes").build();
        field.state = FieldState::TextAreaEditing {
            buffer: "line1\nline2".to_string(),
            cursor: 0,
            line: 0,
            scroll_offset: 0,
        };
        let lines = render(&field, true, &theme());
        let text = spans_text(&lines);
        assert!(text.contains(" 1 "), "should show line number 1");
        assert!(text.contains(" 2 "), "should show line number 2");
    }

    #[test]
    fn test_render_editing_cursor_visible() {
        let mut field = textarea("notes").label("Notes").build();
        field.state = FieldState::TextAreaEditing {
            buffer: "abc".to_string(),
            cursor: 1,
            line: 0,
            scroll_offset: 0,
        };
        let lines = render(&field, true, &theme());
        // Line 0 is the header, line 1 is the buffer line
        let buffer_line = &lines[1];
        let cursor_span = buffer_line.spans.iter().find(|s| s.content.as_ref() == "b");
        assert!(cursor_span.is_some(), "cursor character should be present");
        let style = cursor_span.unwrap().style;
        assert_eq!(style.bg, Some(Color::White));
        assert_eq!(style.fg, Some(Color::Black));
    }

    #[test]
    fn test_render_editing_error() {
        let mut field = textarea("notes").label("Notes").build();
        field.state = FieldState::TextAreaEditing {
            buffer: "".to_string(),
            cursor: 0,
            line: 0,
            scroll_offset: 0,
        };
        field.error = Some("Required".to_string());
        let lines = render(&field, true, &theme());
        let text = spans_text(&lines);
        assert!(text.contains("Required"));
    }
}
