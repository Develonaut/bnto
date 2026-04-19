//! Select widget — renders a choice field as `Vec<Line>`.
//!
//! Two modes based on field state:
//! - **Idle**: compact display showing current value with cycle arrows if focused
//! - **Expanded**: vertical list with highlight, filter bar, and match count

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::field::{Field, FieldKind, FieldState};
use crate::theme::FormTheme;

/// Render a select field as lines of styled spans.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    match &field.state {
        FieldState::SelectExpanded {
            highlight,
            filter,
            filtered_indices,
        } => render_expanded(field, *highlight, filter, filtered_indices, theme),
        _ => render_idle(field, focused, theme),
    }
}

fn render_idle(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };

    let options = match &field.kind {
        FieldKind::Select { options, .. } => options,
        _ => {
            return vec![Line::raw(format!(
                "{prefix}{}   {}",
                field.label, field.value
            ))];
        }
    };

    // Show the display label for the current value, not the raw value
    let display = options
        .iter()
        .find(|o| o.value == field.value)
        .map(|o| o.label.as_str())
        .unwrap_or(field.value.as_str());

    let mut spans = vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
    ];

    if focused {
        spans.push(Span::styled("< ", theme.muted()));
        spans.push(Span::raw(display.to_string()));
        spans.push(Span::styled(" >", theme.muted()));
    } else {
        spans.push(Span::raw(display.to_string()));
    }

    let mut lines = vec![Line::from(spans)];

    if let Some(ref err) = field.error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), theme.error()),
        ]));
    }

    lines
}

fn render_expanded(
    field: &Field,
    highlight: usize,
    filter: &str,
    filtered_indices: &[usize],
    theme: &dyn FormTheme,
) -> Vec<Line<'static>> {
    let options = match &field.kind {
        FieldKind::Select { options, .. } => options,
        _ => return vec![],
    };

    let label_style = theme.heading();
    let mut lines = vec![];

    // Header line with label
    lines.push(Line::from(vec![
        Span::styled("> ", label_style),
        Span::styled(field.label.clone(), label_style),
    ]));

    // Filter bar (always shown in expanded mode)
    let cursor_style = Style::default().bg(Color::White).fg(Color::Black);
    let mut filter_spans = vec![Span::raw("    Filter: ".to_string())];
    if filter.is_empty() {
        filter_spans.push(Span::styled(" ", cursor_style));
    } else {
        filter_spans.push(Span::raw(filter.to_string()));
        filter_spans.push(Span::styled(" ", cursor_style));
    }
    lines.push(Line::from(filter_spans));

    // Option list
    if filtered_indices.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("No matches".to_string(), theme.muted()),
        ]));
    } else {
        let highlight_style = theme.selected();

        for (pos, &idx) in filtered_indices.iter().enumerate() {
            if let Some(opt) = options.get(idx) {
                let is_highlighted = pos == highlight;
                let marker = if is_highlighted { "  > " } else { "    " };
                let style = if is_highlighted {
                    highlight_style
                } else {
                    theme.text()
                };
                lines.push(Line::from(vec![
                    Span::raw(marker.to_string()),
                    Span::styled(opt.label.clone(), style),
                ]));
            }
        }

        // Match count
        let total = options.len();
        let showing = filtered_indices.len();
        if showing < total {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(format!("({showing} of {total})"), theme.muted()),
            ]));
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::select;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn make_select_field(value: &str) -> Field {
        select("fmt", &[("jpeg", "JPEG"), ("png", "PNG"), ("webp", "WebP")])
            .label("Format")
            .value(value)
            .build()
    }

    #[test]
    fn test_select_idle_focused_shows_arrows() {
        let field = make_select_field("jpeg");
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("< JPEG >"), "got: {text}");
    }

    #[test]
    fn test_select_idle_unfocused_no_arrows() {
        let field = make_select_field("jpeg");
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("JPEG"));
        assert!(!text.contains("<"), "got: {text}");
        assert!(!text.contains(">"), "got: {text}");
    }

    #[test]
    fn test_select_idle_shows_label_not_value() {
        let field = make_select_field("png");
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("PNG"), "should show label, got: {text}");
    }

    #[test]
    fn test_select_idle_focused_prefix() {
        let field = make_select_field("jpeg");
        let lines = render(&field, true, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_select_idle_unfocused_prefix() {
        let field = make_select_field("jpeg");
        let lines = render(&field, false, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }

    #[test]
    fn test_select_expanded_shows_header() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: String::new(),
            filtered_indices: vec![0, 1, 2],
        };
        let lines = render(&field, true, &theme());
        let header = line_text(&lines[0]);
        assert!(header.contains("Format"), "got: {header}");
    }

    #[test]
    fn test_select_expanded_shows_filter_bar() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: "pn".to_string(),
            filtered_indices: vec![1],
        };
        let lines = render(&field, true, &theme());
        let filter_line = line_text(&lines[1]);
        assert!(filter_line.contains("Filter:"), "got: {filter_line}");
        assert!(filter_line.contains("pn"), "got: {filter_line}");
    }

    #[test]
    fn test_select_expanded_highlight() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 1,
            filter: String::new(),
            filtered_indices: vec![0, 1, 2],
        };
        let lines = render(&field, true, &theme());
        let highlighted = line_text(&lines[3]);
        assert!(
            highlighted.contains("> "),
            "highlighted should have > marker: {highlighted}"
        );
        assert!(highlighted.contains("PNG"), "got: {highlighted}");

        let not_highlighted = line_text(&lines[2]);
        assert!(
            !not_highlighted.contains("> ") || not_highlighted.starts_with(">"),
            "non-highlighted: {not_highlighted}"
        );
    }

    #[test]
    fn test_select_expanded_empty_filter() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: "xyz".to_string(),
            filtered_indices: vec![],
        };
        let lines = render(&field, true, &theme());
        let all_text: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(all_text.contains("No matches"), "got: {all_text}");
    }

    #[test]
    fn test_select_expanded_match_count() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: "p".to_string(),
            filtered_indices: vec![0, 1, 2],
        };
        let lines = render(&field, true, &theme());
        let all_text: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !all_text.contains("of 3"),
            "no count when all match: {all_text}"
        );

        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: "pn".to_string(),
            filtered_indices: vec![1],
        };
        let lines = render(&field, true, &theme());
        let all_text: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(all_text.contains("(1 of 3)"), "got: {all_text}");
    }

    #[test]
    fn test_select_expanded_highlight_uses_theme() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: String::new(),
            filtered_indices: vec![0, 1, 2],
        };
        let lines = render(&field, true, &theme());
        let option_line = &lines[2];
        let label_span = option_line
            .spans
            .iter()
            .find(|s| s.content.as_ref() == "JPEG");
        assert!(label_span.is_some());
        assert_eq!(label_span.unwrap().style, theme().selected());
    }

    #[test]
    fn test_select_unknown_value_shows_raw() {
        let field = select("fmt", &[("a", "Alpha")]).value("unknown").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(
            text.contains("unknown"),
            "should fall back to raw value: {text}"
        );
    }
}
