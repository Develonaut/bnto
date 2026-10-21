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

/// Columns between two hints in [`render_help_bar`].
const SEPARATOR_WIDTH: usize = 2;

/// The leading `hints` that fit in `width` columns as [`render_help_bar`]
/// draws them. Hints that don't fit are dropped whole, from the end, so the
/// bar never shows a half-cut "Enter s".
pub fn fit_hints<'h>(hints: &[(&'h str, &'h str)], width: usize) -> Vec<(&'h str, &'h str)> {
    let mut used = 0;
    let mut fitted = Vec::new();
    for &(key, desc) in hints {
        let separator = if fitted.is_empty() {
            0
        } else {
            SEPARATOR_WIDTH
        };
        let needed = separator + Line::from(format!("{key} {desc}")).width();
        if used + needed > width {
            break;
        }
        used += needed;
        fitted.push((key, desc));
    }
    fitted
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

    const HINTS: [(&str, &str); 3] = [("Tab", "pane"), ("↑↓", "navigate"), ("q", "quit")];

    #[test]
    fn every_hint_fits_a_wide_bar() {
        assert_eq!(fit_hints(&HINTS, 80), HINTS.to_vec());
    }

    #[test]
    fn hints_that_dont_fit_are_dropped_whole_from_the_end() {
        // "Tab pane" (8) + "  " + "↑↓ navigate" (11) = 21; "q quit" needs 8 more.
        assert_eq!(fit_hints(&HINTS, 21), HINTS[..2].to_vec());
        assert_eq!(fit_hints(&HINTS, 28), HINTS[..2].to_vec());
        assert_eq!(fit_hints(&HINTS, 29), HINTS.to_vec());
    }

    #[test]
    fn no_room_means_no_hints() {
        assert!(fit_hints(&HINTS, 7).is_empty());
        assert!(fit_hints(&HINTS, 0).is_empty());
    }

    #[test]
    fn renders_empty_for_no_hints() {
        let line = render_help_bar(&[], &theme());
        assert!(line.spans.is_empty());
    }
}
