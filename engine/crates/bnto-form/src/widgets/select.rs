//! Select widget — renders a choice field as `Vec<Line>`.
//!
//! Two modes based on field state:
//! - **Idle**: compact display showing current value with cycle arrows if focused
//! - **Expanded**: vertical list with highlight, filter bar, and match count

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::field::{Field, FieldKind, FieldState};

/// Render a select field as lines of styled spans.
pub fn render(field: &Field, focused: bool) -> Vec<Line<'static>> {
    match &field.state {
        FieldState::SelectExpanded {
            highlight,
            filter,
            filtered_indices,
        } => render_expanded(field, *highlight, filter, filtered_indices),
        _ => render_idle(field, focused),
    }
}

fn render_idle(field: &Field, focused: bool) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
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
        spans.push(Span::styled("< ", Style::default().fg(Color::DarkGray)));
        spans.push(Span::raw(display.to_string()));
        spans.push(Span::styled(" >", Style::default().fg(Color::DarkGray)));
    } else {
        spans.push(Span::raw(display.to_string()));
    }

    let mut lines = vec![Line::from(spans)];

    if let Some(ref err) = field.error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), Style::default().fg(Color::Red)),
        ]));
    }

    lines
}

fn render_expanded(
    field: &Field,
    highlight: usize,
    filter: &str,
    filtered_indices: &[usize],
) -> Vec<Line<'static>> {
    let options = match &field.kind {
        FieldKind::Select { options, .. } => options,
        _ => return vec![],
    };

    let label_style = Style::default().add_modifier(Modifier::BOLD);
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
            Span::styled(
                "No matches".to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    } else {
        let highlight_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White);

        for (pos, &idx) in filtered_indices.iter().enumerate() {
            if let Some(opt) = options.get(idx) {
                let is_highlighted = pos == highlight;
                let marker = if is_highlighted { "  > " } else { "    " };
                let style = if is_highlighted {
                    highlight_style
                } else {
                    Style::default()
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
                Span::styled(
                    format!("({showing} of {total})"),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::select;

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
        let lines = render(&field, true);
        let text = line_text(&lines[0]);
        assert!(text.contains("< JPEG >"), "got: {text}");
    }

    #[test]
    fn test_select_idle_unfocused_no_arrows() {
        let field = make_select_field("jpeg");
        let lines = render(&field, false);
        let text = line_text(&lines[0]);
        assert!(text.contains("JPEG"));
        assert!(!text.contains("<"), "got: {text}");
        assert!(!text.contains(">"), "got: {text}");
    }

    #[test]
    fn test_select_idle_shows_label_not_value() {
        let field = make_select_field("png");
        let lines = render(&field, false);
        let text = line_text(&lines[0]);
        assert!(text.contains("PNG"), "should show label, got: {text}");
        // "png" is the value, "PNG" is the label — only label should show
    }

    #[test]
    fn test_select_idle_focused_prefix() {
        let field = make_select_field("jpeg");
        let lines = render(&field, true);
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_select_idle_unfocused_prefix() {
        let field = make_select_field("jpeg");
        let lines = render(&field, false);
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
        let lines = render(&field, true);
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
        let lines = render(&field, true);
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
        let lines = render(&field, true);
        // Line indices: 0=header, 1=filter, 2=JPEG, 3=PNG (highlighted), 4=WebP
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
        let lines = render(&field, true);
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
            filtered_indices: vec![0, 1, 2], // all match
        };
        let lines = render(&field, true);
        let all_text: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        // All 3 match out of 3, so no count line
        assert!(
            !all_text.contains("of 3"),
            "no count when all match: {all_text}"
        );

        // Now filter to 1 match
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: "pn".to_string(),
            filtered_indices: vec![1], // only PNG
        };
        let lines = render(&field, true);
        let all_text: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(all_text.contains("(1 of 3)"), "got: {all_text}");
    }

    #[test]
    fn test_select_expanded_highlight_style() {
        let mut field = make_select_field("jpeg");
        field.state = FieldState::SelectExpanded {
            highlight: 0,
            filter: String::new(),
            filtered_indices: vec![0, 1, 2],
        };
        let lines = render(&field, true);
        // Line 2 is first option (highlighted)
        let option_line = &lines[2];
        let label_span = option_line
            .spans
            .iter()
            .find(|s| s.content.as_ref() == "JPEG");
        assert!(label_span.is_some());
        let style = label_span.unwrap().style;
        assert!(style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(style.fg, Some(Color::White));
    }

    #[test]
    fn test_select_unknown_value_shows_raw() {
        let field = select("fmt", &[("a", "Alpha")]).value("unknown").build();
        let lines = render(&field, false);
        let text = line_text(&lines[0]);
        assert!(
            text.contains("unknown"),
            "should fall back to raw value: {text}"
        );
    }
}
