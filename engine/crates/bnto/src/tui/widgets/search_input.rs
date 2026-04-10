// Search input widget — renders the search query row in the browser.
//
// Pure function: takes query + active flag, returns `Option<Line>`.
// Returns None when there's nothing to display (inactive, empty query).

use ratatui::text::{Line, Span};

use super::super::theme::Theme;

/// Render the search input row for the recipe browser.
///
/// Returns `None` when inactive with an empty query (nothing to show).
/// When active, shows a trailing cursor (`_`).
pub fn render_search_input<'a>(query: &str, active: bool, theme: &Theme) -> Option<Line<'a>> {
    if !active && query.is_empty() {
        return None;
    }

    let text = if active {
        format!("  Search: {}_", query)
    } else {
        format!("  Search: {}", query)
    };

    Some(Line::from(Span::styled(text, theme.text())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::theme::{Theme, ThemeVariant};

    fn theme() -> Theme {
        Theme::from_variant(ThemeVariant::LosAngeles)
    }

    #[test]
    fn renders_none_when_inactive_and_no_query() {
        let result = render_search_input("", false, &theme());
        assert!(result.is_none());
    }

    #[test]
    fn renders_query_when_inactive_with_query() {
        let line = render_search_input("foo", false, &theme()).unwrap();
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Search: foo"));
        assert!(!text.ends_with('_'), "inactive should not show cursor");
    }

    #[test]
    fn renders_query_with_cursor_when_active() {
        let line = render_search_input("foo", true, &theme()).unwrap();
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Search: foo_"));
    }

    #[test]
    fn renders_cursor_only_when_active_empty() {
        let line = render_search_input("", true, &theme()).unwrap();
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Search: _"));
    }
}
