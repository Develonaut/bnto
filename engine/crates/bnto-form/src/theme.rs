//! Theming for form rendering.
//!
//! `FormTheme` is a trait so consumers can bring their own color scheme.
//! `DefaultTheme` ships with terminal-safe colors that work on any background.

use ratatui::style::{Color, Modifier, Style};

/// Style contract for form rendering. Implement this to customize colors.
pub trait FormTheme {
    fn text(&self) -> Style;
    fn selected(&self) -> Style;
    fn muted(&self) -> Style;
    fn error(&self) -> Style;
    fn border(&self) -> Style;
    fn heading(&self) -> Style;
}

/// Terminal-safe default theme using reset colors and basic modifiers.
pub struct DefaultTheme;

impl FormTheme for DefaultTheme {
    fn text(&self) -> Style {
        Style::default()
    }

    fn selected(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    fn muted(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }

    fn error(&self) -> Style {
        Style::default().fg(Color::Red)
    }

    fn border(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }

    fn heading(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }
}
