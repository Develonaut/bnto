// Theme — Motorway design language adapted for the terminal.
//
// Warm palette, rounded borders, generous padding. Matches the web
// app's visual identity within terminal constraints.

// Many palette entries and style helpers are defined upfront for use by
// screen modules added in later waves (browser, detail, execution, etc.).

use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;

/// Warm terracotta — primary actions, selected items.
pub const PRIMARY: Color = Color::Rgb(188, 92, 47);

/// Golden yellow — accents and highlights.
pub const ACCENT: Color = Color::Rgb(214, 178, 56);

/// Soft teal — secondary information.
pub const SECONDARY: Color = Color::Rgb(100, 180, 170);

/// Warm dark brown — body text.
pub const TEXT: Color = Color::Rgb(67, 52, 44);

/// Muted warm gray — secondary text, dimmed content.
pub const TEXT_MUTED: Color = Color::Rgb(128, 110, 100);

/// Warm cream — background tint (for terminals that support it).
pub const BG: Color = Color::Reset;

/// Green — success indicators.
pub const SUCCESS: Color = Color::Rgb(80, 160, 80);

/// Red — error/failure indicators.
pub const ERROR: Color = Color::Rgb(190, 60, 50);

/// Rounded border set — warm and friendly, never sharp.
pub const ROUNDED_BORDERS: symbols::border::Set = symbols::border::ROUNDED;

// --- Style helpers ---

/// Normal body text.
pub fn text() -> Style {
    Style::default().fg(TEXT)
}

/// Muted/secondary text.
pub fn muted() -> Style {
    Style::default().fg(TEXT_MUTED)
}

/// Primary-colored text (terracotta).
pub fn primary() -> Style {
    Style::default().fg(PRIMARY)
}

/// Accent-colored text (golden).
pub fn accent() -> Style {
    Style::default().fg(ACCENT)
}

/// Bold heading text.
pub fn heading() -> Style {
    Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
}

/// Category header — uppercase, bold, muted.
pub fn category() -> Style {
    Style::default()
        .fg(TEXT_MUTED)
        .add_modifier(Modifier::BOLD)
}

/// Selected/highlighted item.
pub fn selected() -> Style {
    Style::default()
        .fg(PRIMARY)
        .add_modifier(Modifier::BOLD)
}

/// Success status text.
pub fn success() -> Style {
    Style::default().fg(SUCCESS)
}

/// Error status text.
pub fn error() -> Style {
    Style::default().fg(ERROR)
}

/// Key hint in help bar (e.g., the "q" in "q quit").
pub fn key_hint() -> Style {
    Style::default()
        .fg(ACCENT)
        .add_modifier(Modifier::BOLD)
}

/// Description text next to key hint.
pub fn key_desc() -> Style {
    Style::default().fg(TEXT_MUTED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_helpers_return_non_default() {
        // Verify each helper produces a distinct style (not plain default).
        assert_ne!(primary(), Style::default());
        assert_ne!(heading(), Style::default());
        assert_ne!(selected(), Style::default());
        assert_ne!(muted(), Style::default());
    }
}
