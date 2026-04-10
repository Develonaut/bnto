// Help bar widget — renders contextual key hints at the bottom of the screen.
//
// Pure function: takes hint tuples + theme, returns a `Line`.
// The caller renders the Line into the help area via Paragraph.

use ratatui::text::{Line, Span};

use super::super::theme::Theme;

/// Render a help bar from key-description hint pairs.
///
/// Each hint is rendered as `key desc` with separator spacing between pairs.
/// Returns an empty `Line` when hints is empty.
pub fn render_help_bar<'a>(hints: &[(&str, &str)], theme: &Theme) -> Line<'a> {
    let spans: Vec<Span> = hints
        .iter()
        .enumerate()
        .flat_map(|(i, (key, desc))| {
            let mut parts = vec![
                Span::styled(key.to_string(), theme.key_hint()),
                Span::styled(format!(" {desc}"), theme.key_desc()),
            ];
            if i < hints.len() - 1 {
                parts.push(Span::raw("  "));
            }
            parts
        })
        .collect();

    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::theme::{Theme, ThemeVariant};

    fn theme() -> Theme {
        Theme::from_variant(ThemeVariant::LosAngeles)
    }

    #[test]
    fn renders_all_hints_with_separators() {
        let hints = vec![("↑↓", "navigate"), ("/", "search"), ("q", "quit")];
        let line = render_help_bar(&hints, &theme());
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("navigate"));
        assert!(text.contains("search"));
        assert!(text.contains("quit"));
        // 3 hints = 2 separators (between pairs), each separator is "  "
        let separator_count = line
            .spans
            .iter()
            .filter(|s| s.content.as_ref() == "  ")
            .count();
        assert_eq!(separator_count, 2);
    }

    #[test]
    fn renders_single_hint_no_separator() {
        let hints = vec![("Esc", "cancel")];
        let line = render_help_bar(&hints, &theme());
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("Esc"));
        assert!(text.contains("cancel"));
        let separator_count = line
            .spans
            .iter()
            .filter(|s| s.content.as_ref() == "  ")
            .count();
        assert_eq!(separator_count, 0);
    }

    #[test]
    fn renders_empty_for_no_hints() {
        let line = render_help_bar(&[], &theme());
        assert!(line.spans.is_empty());
    }
}
