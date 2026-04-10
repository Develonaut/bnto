// Theme — Motorway design language adapted for the terminal.
//
// Colors are generated from theme/palette.toml into `palette.rs`.
// This module provides the `Theme` struct (variant-aware color resolution)
// and style helpers used by all TUI rendering code.

use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;

// Re-export generated palette for direct access when needed.
pub use super::palette;
pub use super::palette::{monaco, tokyo};

// --- Backward-compatibility aliases (used in existing code) ---

/// Body text color (alias for FOREGROUND).
pub const TEXT: Color = palette::FOREGROUND;

/// Muted text color (alias for MUTED_FOREGROUND).
pub const TEXT_MUTED: Color = palette::MUTED_FOREGROUND;

/// Error color (alias for DESTRUCTIVE).
pub const ERROR: Color = palette::DESTRUCTIVE;

/// Rounded border set — warm and friendly, never sharp.
pub const ROUNDED_BORDERS: symbols::border::Set = symbols::border::ROUNDED;

// --- Theme variant ---

/// Which color theme the TUI is using.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Los Angeles — warm cream light theme (default).
    LosAngeles,
    /// Tokyo — cool dark slate theme.
    Tokyo,
    /// Monaco — golden-hour sunset theme.
    Monaco,
}

/// All available theme variants, used by settings screen and key handling.
pub const ALL_VARIANTS: [ThemeVariant; 3] = [
    ThemeVariant::LosAngeles,
    ThemeVariant::Tokyo,
    ThemeVariant::Monaco,
];

impl ThemeVariant {
    /// Human-readable display name for the variant.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::LosAngeles => "Los Angeles",
            Self::Tokyo => "Tokyo",
            Self::Monaco => "Monaco",
        }
    }

    /// Parse a variant from a CLI string.
    pub fn from_str_lossy(s: &str) -> Result<Self, String> {
        match s {
            "los-angeles" => Ok(Self::LosAngeles),
            "tokyo" => Ok(Self::Tokyo),
            "monaco" => Ok(Self::Monaco),
            other => Err(format!(
                "unknown theme: {other}. Valid: los-angeles, tokyo, monaco"
            )),
        }
    }
}

/// Resolved theme — all semantic colors for a given variant.
///
/// Constructed once from `ThemeVariant`, then threaded through
/// all rendering code. Style helpers are methods on this struct.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub accent: Color,
    pub secondary: Color,
    pub foreground: Color,
    pub muted_foreground: Color,
    pub background: Color,
    pub success: Color,
    pub destructive: Color,
    pub warning: Color,
    pub border: Color,
}

impl Theme {
    /// Build a resolved theme from a variant.
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::LosAngeles => Self {
                primary: palette::PRIMARY,
                accent: palette::ACCENT,
                secondary: palette::SECONDARY,
                foreground: palette::FOREGROUND,
                muted_foreground: palette::MUTED_FOREGROUND,
                background: palette::BACKGROUND,
                success: palette::SUCCESS,
                destructive: palette::DESTRUCTIVE,
                warning: palette::WARNING,
                border: palette::BORDER,
            },
            ThemeVariant::Tokyo => Self {
                primary: tokyo::PRIMARY,
                accent: tokyo::ACCENT,
                secondary: tokyo::SECONDARY,
                foreground: tokyo::FOREGROUND,
                muted_foreground: tokyo::MUTED_FOREGROUND,
                background: tokyo::BACKGROUND,
                success: tokyo::SUCCESS,
                destructive: tokyo::DESTRUCTIVE,
                warning: tokyo::WARNING,
                border: tokyo::BORDER,
            },
            ThemeVariant::Monaco => Self {
                primary: monaco::PRIMARY,
                accent: monaco::ACCENT,
                secondary: monaco::SECONDARY,
                foreground: monaco::FOREGROUND,
                muted_foreground: monaco::MUTED_FOREGROUND,
                background: monaco::BACKGROUND,
                success: monaco::SUCCESS,
                destructive: monaco::DESTRUCTIVE,
                warning: monaco::WARNING,
                border: monaco::BORDER,
            },
        }
    }

    // --- Style helpers ---

    /// Normal body text.
    pub fn text(&self) -> Style {
        Style::default().fg(self.foreground)
    }

    /// Muted/secondary text.
    pub fn muted(&self) -> Style {
        Style::default().fg(self.muted_foreground)
    }

    /// Primary-colored text (terracotta).
    pub fn primary(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Accent-colored text (salmon pink).
    pub fn accent(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Bold heading text.
    pub fn heading(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .add_modifier(Modifier::BOLD)
    }

    /// Category header — uppercase, bold, muted.
    pub fn category(&self) -> Style {
        Style::default()
            .fg(self.muted_foreground)
            .add_modifier(Modifier::BOLD)
    }

    /// Selected/highlighted item.
    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Success status text.
    pub fn success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Error status text.
    pub fn error(&self) -> Style {
        Style::default().fg(self.destructive)
    }

    /// Key hint in help bar — uses warning (golden) for attention.
    pub fn key_hint(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    /// Description text next to key hint.
    pub fn key_desc(&self) -> Style {
        Style::default().fg(self.muted_foreground)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_from_variant_returns_distinct_colors() {
        let la = Theme::from_variant(ThemeVariant::LosAngeles);
        let tk = Theme::from_variant(ThemeVariant::Tokyo);
        let mn = Theme::from_variant(ThemeVariant::Monaco);

        // Backgrounds must differ across all three themes.
        assert_ne!(la.background, tk.background);
        assert_ne!(la.background, mn.background);
        assert_ne!(tk.background, mn.background);

        // Primary is the same terracotta across all themes.
        assert_eq!(la.primary, tk.primary);
        assert_eq!(la.primary, mn.primary);
    }

    #[test]
    fn theme_variant_from_str_lossy() {
        assert_eq!(
            ThemeVariant::from_str_lossy("los-angeles"),
            Ok(ThemeVariant::LosAngeles)
        );
        assert_eq!(
            ThemeVariant::from_str_lossy("tokyo"),
            Ok(ThemeVariant::Tokyo)
        );
        assert_eq!(
            ThemeVariant::from_str_lossy("monaco"),
            Ok(ThemeVariant::Monaco)
        );
        assert!(ThemeVariant::from_str_lossy("invalid").is_err());
    }

    #[test]
    fn theme_style_helpers_use_correct_colors() {
        let theme = Theme::from_variant(ThemeVariant::LosAngeles);
        assert_eq!(theme.text().fg, Some(palette::FOREGROUND));
        assert_eq!(theme.muted().fg, Some(palette::MUTED_FOREGROUND));
        assert_eq!(theme.primary().fg, Some(palette::PRIMARY));
    }

    #[test]
    fn theme_changed_swaps_colors() {
        let la = Theme::from_variant(ThemeVariant::LosAngeles);
        let tk = Theme::from_variant(ThemeVariant::Tokyo);
        assert_ne!(la.text().fg, tk.text().fg);
    }

    #[test]
    fn backward_compat_aliases_match_palette() {
        assert_eq!(TEXT, palette::FOREGROUND);
        assert_eq!(TEXT_MUTED, palette::MUTED_FOREGROUND);
        assert_eq!(ERROR, palette::DESTRUCTIVE);
    }

    #[test]
    fn from_str_lossy_error_includes_input() {
        let err = ThemeVariant::from_str_lossy("nope").unwrap_err();
        assert!(err.contains("nope"), "error should echo the bad input");
        assert!(
            err.contains("los-angeles"),
            "error should list valid options"
        );
    }

    #[test]
    fn all_style_helpers_return_correct_colors() {
        let t = Theme::from_variant(ThemeVariant::LosAngeles);
        assert_eq!(t.accent().fg, Some(palette::ACCENT));
        assert_eq!(t.heading().fg, Some(palette::FOREGROUND));
        assert_eq!(t.category().fg, Some(palette::MUTED_FOREGROUND));
        assert_eq!(t.selected().fg, Some(palette::PRIMARY));
        assert_eq!(t.success().fg, Some(palette::SUCCESS));
        assert_eq!(t.error().fg, Some(palette::DESTRUCTIVE));
        assert_eq!(t.key_hint().fg, Some(palette::WARNING));
        assert_eq!(t.key_desc().fg, Some(palette::MUTED_FOREGROUND));
    }

    #[test]
    fn heading_and_selected_are_bold() {
        let t = Theme::from_variant(ThemeVariant::Tokyo);
        assert!(t.heading().add_modifier.contains(Modifier::BOLD));
        assert!(t.selected().add_modifier.contains(Modifier::BOLD));
        assert!(t.key_hint().add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn display_name_returns_human_readable_labels() {
        assert_eq!(ThemeVariant::LosAngeles.display_name(), "Los Angeles");
        assert_eq!(ThemeVariant::Tokyo.display_name(), "Tokyo");
        assert_eq!(ThemeVariant::Monaco.display_name(), "Monaco");
    }

    #[test]
    fn monaco_variant_has_distinct_background() {
        let mn = Theme::from_variant(ThemeVariant::Monaco);
        assert_eq!(mn.background, monaco::BACKGROUND);
        assert_eq!(mn.foreground, monaco::FOREGROUND);
        assert_ne!(mn.background, palette::BACKGROUND);
    }
}
