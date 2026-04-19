//! Confirm widget — renders a boolean Yes/No toggle as `Vec<Line>`.
//!
//! Focused: `> Label   [ Yes ]  No` or `> Label   Yes  [ No ]`
//! Unfocused: `  Label   Yes` or `  Label   No`

use ratatui::text::{Line, Span};

use crate::field::{Field, FieldKind};
use crate::theme::FormTheme;

/// Render a confirm field as lines of styled spans.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };

    let (affirmative, negative) = match &field.kind {
        FieldKind::Confirm {
            affirmative,
            negative,
        } => (affirmative.clone(), negative.clone()),
        _ => ("Yes".to_string(), "No".to_string()),
    };

    let is_true = field.value == "true";

    let mut spans = vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
    ];

    let active_style = theme.selected();
    let inactive_style = theme.muted();

    if is_true {
        spans.push(Span::styled(format!("[ {affirmative} ]"), active_style));
        spans.push(Span::raw("  "));
        spans.push(Span::styled(negative, inactive_style));
    } else {
        spans.push(Span::styled(affirmative, inactive_style));
        spans.push(Span::raw("  "));
        spans.push(Span::styled(format!("[ {negative} ]"), active_style));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::confirm;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn test_confirm_render_true() {
        let field = confirm("ok").label("Overwrite?").value("true").build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("[ Yes ]"));
        assert!(text.contains("No"));
        assert!(!text.contains("[ No ]"));
    }

    #[test]
    fn test_confirm_render_false() {
        let field = confirm("ok").label("Overwrite?").value("false").build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("[ No ]"));
        assert!(text.contains("Yes"));
        assert!(!text.contains("[ Yes ]"));
    }

    #[test]
    fn test_confirm_render_custom_labels() {
        let mut field = confirm("ok").label("Continue?").value("true").build();
        if let FieldKind::Confirm {
            ref mut affirmative,
            ref mut negative,
        } = field.kind
        {
            *affirmative = "Proceed".to_string();
            *negative = "Abort".to_string();
        }
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("[ Proceed ]"));
        assert!(text.contains("Abort"));
    }

    #[test]
    fn test_confirm_render_focused_prefix() {
        let field = confirm("ok").label("Ok?").value("true").build();
        let lines = render(&field, true, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_confirm_render_unfocused_prefix() {
        let field = confirm("ok").label("Ok?").value("true").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }

    #[test]
    fn test_confirm_render_default_false() {
        let field = confirm("ok").label("Ok?").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("[ No ]"));
    }

    #[test]
    fn test_confirm_active_uses_theme_selected() {
        let field = confirm("ok").label("Ok?").value("true").build();
        let lines = render(&field, true, &theme());
        let yes_span = lines[0]
            .spans
            .iter()
            .find(|s| s.content.contains("[ Yes ]"));
        assert!(yes_span.is_some());
        // DefaultTheme selected() is BOLD — verify it matches
        let style = yes_span.unwrap().style;
        assert_eq!(style, theme().selected());
    }

    #[test]
    fn test_confirm_inactive_uses_theme_muted() {
        let field = confirm("ok").label("Ok?").value("true").build();
        let lines = render(&field, true, &theme());
        let no_span = lines[0].spans.iter().find(|s| s.content.as_ref() == "No");
        assert!(no_span.is_some());
        assert_eq!(no_span.unwrap().style, theme().muted());
    }
}
