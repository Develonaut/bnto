//! "bnto form" ASCII wordmark for the demo binary.

use ratatui::style::Style;
use ratatui::text::{Line, Span};

/// Block-character logo height in terminal rows.
pub const LOGO_HEIGHT: u16 = 7;

const LOGO: [&str; 6] = [
    "▄▄                                          ",
    "██           ██       ▄▄                     ",
    "████▄ ████▄ ▀██▀▀ ▄███▄   ████  ▄███▄ ████▄▄",
    "██ ██ ██ ██  ██   ██ ██   ██   ██ ██  ██ ██ ",
    "████▀ ██ ██  ██   ▀███▀  ▀██   ▀███▀  ██    ",
    "                                             ",
];

/// Render the logo as styled lines.
pub fn render_logo(style: Style) -> Vec<Line<'static>> {
    LOGO.iter()
        .map(|row| Line::from(Span::styled((*row).to_string(), style)))
        .collect()
}
