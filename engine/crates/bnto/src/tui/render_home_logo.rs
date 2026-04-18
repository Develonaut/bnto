// ASCII wordmark — block-character "bnto" logo for the home screen.
//
// Chunky lowercase letterforms with ascender on b, rounded belly shapes.
// Styled with theme accent color.

use ratatui::text::{Line, Span};

use super::theme::Theme;

/// Raw logo lines — lowercase "bnto" (6 rows).
const LOGO: [&str; 6] = [
    "▄▄                      ",
    "██           ██         ",
    "████▄ ████▄ ▀██▀▀ ▄███▄ ",
    "██ ██ ██ ██  ██   ██ ██ ",
    "████▀ ██ ██  ██   ▀███▀ ",
    "                        ",
];

/// Render the "bnto" ASCII wordmark as styled lines.
pub fn logo_lines(theme: &Theme) -> Vec<Line<'static>> {
    let style = theme.border();
    LOGO.iter()
        .map(|row| Line::from(Span::styled(row.to_string(), style)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::theme::ThemeVariant;

    #[test]
    fn logo_returns_six_lines() {
        let theme = Theme::from_variant(ThemeVariant::LosAngeles);
        let lines = logo_lines(&theme);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn logo_lines_are_consistent_width() {
        let theme = Theme::from_variant(ThemeVariant::Tokyo);
        let lines = logo_lines(&theme);
        let widths: Vec<usize> = lines.iter().map(|l| l.width()).collect();
        assert!(
            widths.iter().all(|w| *w > 10),
            "lines should be non-trivial width"
        );
    }
}
