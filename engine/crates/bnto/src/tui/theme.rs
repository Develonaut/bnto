// Theme — accent-only theming for terminal compatibility.
//
// Body text uses terminal-native colors (Color::Reset, Color::DarkGray)
// so themes are readable on any terminal background. Themes control
// accent personality: the active color for selected items, borders,
// and key hints — plus per-theme status colors.

use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;

// Re-export generated palette for direct access when needed.
pub use super::palette;
pub use super::palette::{monaco, tokyo};

/// Rounded border set — warm and friendly, never sharp.
pub const ROUNDED_BORDERS: symbols::border::Set = symbols::border::ROUNDED;

// --- Theme variant ---

/// Which color theme the TUI is using.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Los Angeles — warm terracotta accent (default).
    LosAngeles,
    /// Tokyo — cool electric blue accent.
    Tokyo,
    /// Monaco — golden-hour amber accent.
    Monaco,
}

/// All available theme variants, used by settings screen and key handling.
pub const ALL_VARIANTS: [ThemeVariant; 3] = [
    ThemeVariant::LosAngeles,
    ThemeVariant::Tokyo,
    ThemeVariant::Monaco,
];

impl ThemeVariant {
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

// --- Per-theme active colors ---
// Mid-brightness saturated colors readable on both light and dark terminals.

/// Los Angeles — warm California terracotta.
const ACTIVE_LOS_ANGELES: Color = palette::PRIMARY; // RGB(240, 101, 66)

/// Tokyo — cool neon electric blue.
const ACTIVE_TOKYO: Color = tokyo::FOCUS_RING; // RGB(78, 134, 255)

/// Monaco — golden-hour sunset amber.
const ACTIVE_MONACO: Color = monaco::WARNING; // RGB(235, 136, 59)

/// Resolved theme — accent colors for a given variant.
///
/// Body text and muted text use terminal-native colors (Color::Reset,
/// Color::DarkGray) so the TUI is readable on any terminal background.
/// The theme only controls accent personality and status colors.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Per-theme hero color — selected items, borders, key hints.
    pub active: Color,
    /// Success status (green family).
    pub success: Color,
    /// Error/destructive status (red family).
    pub destructive: Color,
}

impl Theme {
    /// Build a resolved theme from a variant.
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::LosAngeles => Self {
                active: ACTIVE_LOS_ANGELES,
                success: palette::SUCCESS,
                destructive: palette::DESTRUCTIVE,
            },
            ThemeVariant::Tokyo => Self {
                active: ACTIVE_TOKYO,
                success: tokyo::SUCCESS,
                destructive: tokyo::DESTRUCTIVE,
            },
            ThemeVariant::Monaco => Self {
                active: ACTIVE_MONACO,
                success: monaco::SUCCESS,
                destructive: monaco::DESTRUCTIVE,
            },
        }
    }

    // --- Style helpers ---

    /// Normal body text — terminal native foreground, always readable.
    pub fn text(&self) -> Style {
        Style::default().fg(Color::Reset)
    }

    /// Muted/secondary text — terminal-safe on both light and dark.
    pub fn muted(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }

    /// Bold heading text — terminal native foreground.
    pub fn heading(&self) -> Style {
        Style::default()
            .fg(Color::Reset)
            .add_modifier(Modifier::BOLD)
    }

    /// Category header — bold, subtle.
    pub fn category(&self) -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    }

    /// Selected/highlighted item — per-theme active color.
    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.active)
            .add_modifier(Modifier::BOLD)
    }

    /// Border style — per-theme active color for persistent identity.
    pub fn border(&self) -> Style {
        Style::default().fg(self.active)
    }

    /// Success status text.
    pub fn success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Error status text.
    pub fn error(&self) -> Style {
        Style::default().fg(self.destructive)
    }

    /// Key hint in help bar — per-theme active color for attention.
    pub fn key_hint(&self) -> Style {
        Style::default()
            .fg(self.active)
            .add_modifier(Modifier::BOLD)
    }

    /// Description text next to key hint.
    pub fn key_desc(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_theme_has_distinct_active_color() {
        let la = Theme::from_variant(ThemeVariant::LosAngeles);
        let tk = Theme::from_variant(ThemeVariant::Tokyo);
        let mn = Theme::from_variant(ThemeVariant::Monaco);

        assert_ne!(la.active, tk.active);
        assert_ne!(la.active, mn.active);
        assert_ne!(tk.active, mn.active);
    }

    #[test]
    fn los_angeles_uses_terracotta() {
        let la = Theme::from_variant(ThemeVariant::LosAngeles);
        assert_eq!(la.active, palette::PRIMARY);
    }

    #[test]
    fn tokyo_uses_electric_blue() {
        let tk = Theme::from_variant(ThemeVariant::Tokyo);
        assert_eq!(tk.active, tokyo::FOCUS_RING);
    }

    #[test]
    fn monaco_uses_sunset_amber() {
        let mn = Theme::from_variant(ThemeVariant::Monaco);
        assert_eq!(mn.active, monaco::WARNING);
    }

    #[test]
    fn body_text_uses_terminal_native() {
        let t = Theme::from_variant(ThemeVariant::LosAngeles);
        assert_eq!(t.text().fg, Some(Color::Reset));
        assert_eq!(t.heading().fg, Some(Color::Reset));
    }

    #[test]
    fn muted_text_uses_dark_gray() {
        let t = Theme::from_variant(ThemeVariant::Tokyo);
        assert_eq!(t.muted().fg, Some(Color::DarkGray));
        assert_eq!(t.category().fg, Some(Color::DarkGray));
        assert_eq!(t.key_desc().fg, Some(Color::DarkGray));
    }

    #[test]
    fn selected_and_border_use_active_color() {
        let t = Theme::from_variant(ThemeVariant::Monaco);
        assert_eq!(t.selected().fg, Some(t.active));
        assert_eq!(t.border().fg, Some(t.active));
        assert_eq!(t.key_hint().fg, Some(t.active));
    }

    #[test]
    fn heading_selected_and_key_hint_are_bold() {
        let t = Theme::from_variant(ThemeVariant::Tokyo);
        assert!(t.heading().add_modifier.contains(Modifier::BOLD));
        assert!(t.selected().add_modifier.contains(Modifier::BOLD));
        assert!(t.key_hint().add_modifier.contains(Modifier::BOLD));
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
    fn from_str_lossy_error_includes_input() {
        let err = ThemeVariant::from_str_lossy("nope").unwrap_err();
        assert!(err.contains("nope"), "error should echo the bad input");
        assert!(
            err.contains("los-angeles"),
            "error should list valid options"
        );
    }

    #[test]
    fn status_colors_are_per_theme() {
        let la = Theme::from_variant(ThemeVariant::LosAngeles);
        let tk = Theme::from_variant(ThemeVariant::Tokyo);

        assert_eq!(la.success().fg, Some(palette::SUCCESS));
        assert_eq!(tk.success().fg, Some(tokyo::SUCCESS));
        assert_eq!(la.error().fg, Some(palette::DESTRUCTIVE));
        assert_eq!(tk.error().fg, Some(tokyo::DESTRUCTIVE));
    }
}
