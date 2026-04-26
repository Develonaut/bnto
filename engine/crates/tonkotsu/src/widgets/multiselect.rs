//! MultiSelect widget — renders a multi-choice field as `Vec<Line>`.
//!
//! Two modes based on field state:
//! - **Idle**: compact display showing selected items
//! - **Editing**: vertical list with checkboxes, highlight, and selection count

use ratatui::text::{Line, Span};

use crate::field::{Field, FieldKind, FieldState};
use crate::theme::FormTheme;

/// Render a multiselect field as lines of styled spans.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    match &field.state {
        FieldState::MultiSelectEditing {
            highlight,
            selected,
        } => render_expanded(field, *highlight, selected, theme),
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

    let display = field.display_value();
    let value_part = if display == "None selected" {
        Span::styled(display, theme.muted())
    } else {
        Span::raw(display)
    };

    vec![Line::from(vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
        value_part,
    ])]
}

fn render_expanded(
    field: &Field,
    highlight: usize,
    selected: &[usize],
    theme: &dyn FormTheme,
) -> Vec<Line<'static>> {
    let options = match &field.kind {
        FieldKind::MultiSelect { options } => options,
        _ => return vec![],
    };

    let label_style = theme.heading();
    let mut lines = vec![];

    // Header
    lines.push(Line::from(vec![
        Span::styled("> ", label_style),
        Span::styled(field.label.clone(), label_style),
    ]));

    // Option list with checkboxes
    let highlight_style = theme.selected();
    for (i, opt) in options.iter().enumerate() {
        let is_highlighted = i == highlight;
        let is_selected = selected.contains(&i);
        let checkbox = if is_selected { "[x] " } else { "[ ] " };
        let marker = if is_highlighted { "  > " } else { "    " };
        let style = if is_highlighted {
            highlight_style
        } else {
            theme.text()
        };
        lines.push(Line::from(vec![
            Span::raw(marker.to_string()),
            Span::styled(checkbox.to_string(), style),
            Span::styled(opt.label.clone(), style),
        ]));
    }

    // Selection count
    let count = selected.len();
    let total = options.len();
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(format!("({count} of {total} selected)"), theme.muted()),
    ]));

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::multiselect;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn make_field(value: &str) -> Field {
        multiselect(
            "tags",
            &[("img", "Image"), ("vec", "Vector"), ("csv", "CSV")],
        )
        .label("Tags")
        .value(value)
        .build()
    }

    #[test]
    fn test_idle_focused_prefix() {
        let field = make_field("");
        let lines = render(&field, true, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_idle_unfocused_prefix() {
        let field = make_field("");
        let lines = render(&field, false, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }

    #[test]
    fn test_idle_empty_shows_none_selected() {
        let field = make_field("");
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("None selected"), "got: {text}");
    }

    #[test]
    fn test_idle_shows_selected_labels() {
        let field = make_field("img,csv");
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("Image"), "got: {text}");
        assert!(text.contains("CSV"), "got: {text}");
    }

    #[test]
    fn test_expanded_shows_header() {
        let mut field = make_field("");
        field.state = FieldState::MultiSelectEditing {
            highlight: 0,
            selected: vec![],
        };
        let lines = render(&field, true, &theme());
        let header = line_text(&lines[0]);
        assert!(header.contains("Tags"), "got: {header}");
    }

    #[test]
    fn test_expanded_shows_checkboxes() {
        let mut field = make_field("");
        field.state = FieldState::MultiSelectEditing {
            highlight: 0,
            selected: vec![0, 2],
        };
        let lines = render(&field, true, &theme());
        // Lines: header, 3 options, count = 5
        assert_eq!(lines.len(), 5, "got {} lines", lines.len());
        let opt0 = line_text(&lines[1]);
        assert!(opt0.contains("[x]"), "selected should have [x]: {opt0}");
        assert!(opt0.contains("Image"), "got: {opt0}");
        let opt1 = line_text(&lines[2]);
        assert!(opt1.contains("[ ]"), "unselected should have [ ]: {opt1}");
        let opt2 = line_text(&lines[3]);
        assert!(opt2.contains("[x]"), "selected should have [x]: {opt2}");
    }

    #[test]
    fn test_expanded_highlight_marker() {
        let mut field = make_field("");
        field.state = FieldState::MultiSelectEditing {
            highlight: 1,
            selected: vec![],
        };
        let lines = render(&field, true, &theme());
        let opt0 = line_text(&lines[1]);
        assert!(!opt0.starts_with("  > "), "non-highlighted: {opt0}");
        let opt1 = line_text(&lines[2]);
        assert!(opt1.contains("> "), "highlighted should have >: {opt1}");
    }

    #[test]
    fn test_expanded_selection_count() {
        let mut field = make_field("");
        field.state = FieldState::MultiSelectEditing {
            highlight: 0,
            selected: vec![0, 2],
        };
        let lines = render(&field, true, &theme());
        let count_line = line_text(lines.last().unwrap());
        assert!(
            count_line.contains("(2 of 3 selected)"),
            "got: {count_line}"
        );
    }

    #[test]
    fn test_expanded_uses_theme_for_highlight() {
        let mut field = make_field("");
        field.state = FieldState::MultiSelectEditing {
            highlight: 0,
            selected: vec![],
        };
        let lines = render(&field, true, &theme());
        // The checkbox span of highlighted option should use selected style
        let label_span = lines[1]
            .spans
            .iter()
            .find(|s| s.content.as_ref() == "Image");
        assert!(label_span.is_some());
        assert_eq!(label_span.unwrap().style, theme().selected());
    }
}
