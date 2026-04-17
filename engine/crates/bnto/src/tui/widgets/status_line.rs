// Status line widget — shows recipe count, version, and active theme.
//
// Pure function: takes counts + strings + theme, returns a `Line`.
// Displayed above or alongside the help bar.

use ratatui::text::{Line, Span};

use super::super::theme::Theme;

/// Render the status line, showing a status message if present.
///
/// When `status_message` is `Some`, it replaces the default recipe/version line.
/// When `None`, shows the standard recipe count + version + theme.
pub fn render_status_line_with_message<'a>(
    recipe_count: usize,
    version: &str,
    theme_name: &str,
    status_message: Option<&str>,
    theme: &Theme,
) -> Line<'a> {
    if let Some(msg) = status_message {
        return Line::from(Span::styled(msg.to_string(), theme.muted()));
    }
    render_status_line(recipe_count, version, theme_name, theme)
}

/// Render the status line with recipe count, version, and theme name.
pub fn render_status_line<'a>(
    recipe_count: usize,
    version: &str,
    theme_name: &str,
    theme: &Theme,
) -> Line<'a> {
    let recipe_label = if recipe_count == 1 {
        "1 recipe".to_string()
    } else {
        format!("{recipe_count} recipes")
    };

    Line::from(vec![
        Span::styled(recipe_label, theme.muted()),
        Span::styled("  ·  ", theme.muted()),
        Span::styled(format!("v{version}"), theme.muted()),
        Span::styled("  ·  ", theme.muted()),
        Span::styled(theme_name.to_string(), theme.muted()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::theme::{Theme, ThemeVariant};

    fn theme() -> Theme {
        Theme::from_variant(ThemeVariant::LosAngeles)
    }

    fn collect_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn includes_recipe_count() {
        let line = render_status_line(15, "0.1.0", "Los Angeles", &theme());
        let text = collect_text(&line);
        assert!(text.contains("15 recipes"));
    }

    #[test]
    fn includes_version() {
        let line = render_status_line(5, "0.2.3", "Tokyo", &theme());
        let text = collect_text(&line);
        assert!(text.contains("v0.2.3"));
    }

    #[test]
    fn includes_theme_name() {
        let line = render_status_line(3, "0.1.0", "Monaco", &theme());
        let text = collect_text(&line);
        assert!(text.contains("Monaco"));
    }

    #[test]
    fn singular_recipe_count() {
        let line = render_status_line(1, "0.1.0", "Tokyo", &theme());
        let text = collect_text(&line);
        assert!(text.contains("1 recipe"));
        assert!(!text.contains("1 recipes"));
    }

    #[test]
    fn status_message_overrides_default_line() {
        let line =
            render_status_line_with_message(3, "0.1.0", "Tokyo", Some("Settings saved"), &theme());
        let text = collect_text(&line);
        assert!(text.contains("Settings saved"));
    }

    #[test]
    fn no_status_message_shows_default() {
        let line = render_status_line_with_message(3, "0.1.0", "Tokyo", None, &theme());
        let text = collect_text(&line);
        assert!(text.contains("3 recipes"));
        assert!(text.contains("Tokyo"));
    }

    #[test]
    fn error_status_message_shown() {
        let line = render_status_line_with_message(
            3,
            "0.1.0",
            "Tokyo",
            Some("Failed to save config"),
            &theme(),
        );
        let text = collect_text(&line);
        assert!(text.contains("Failed to save config"));
    }
}
