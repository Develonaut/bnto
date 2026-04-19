//! Help bar for the demo binary.
//!
//! Renders a single styled `Line` with key hints and metadata.
//! Uses `FormTheme` for consistent styling with the form.

use ratatui::text::{Line, Span};

use crate::theme::FormTheme;

/// Render the bottom help bar showing key hints and field count.
///
/// Format: `Tab navigate  Enter edit  Esc cancel  Ctrl+R reset  10 fields  v0.1.0`
pub fn render_help_bar<'a>(field_count: usize, theme: &dyn FormTheme) -> Line<'a> {
    let hints = [
        ("Tab", "navigate"),
        ("Enter", "edit"),
        ("Esc", "cancel/quit"),
        ("Ctrl+R", "reset"),
    ];

    let mut spans: Vec<Span<'a>> = Vec::new();

    for (i, (key, desc)) in hints.iter().enumerate() {
        spans.push(Span::styled((*key).to_string(), theme.heading()));
        spans.push(Span::styled(format!(" {desc}"), theme.muted()));
        if i < hints.len() - 1 {
            spans.push(Span::raw("  "));
        }
    }

    // Field count
    spans.push(Span::raw("  "));
    spans.push(Span::styled(format!("{field_count} fields"), theme.muted()));

    // Version
    spans.push(Span::raw("  "));
    spans.push(Span::styled(
        format!("v{}", env!("CARGO_PKG_VERSION")),
        theme.muted(),
    ));

    Line::from(spans)
}
