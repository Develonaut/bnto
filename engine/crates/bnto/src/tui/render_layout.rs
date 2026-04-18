// Shared layout helpers — app frame, centering, spacing, and content panels.
//
// `app_frame()` is the top-level layout: centers all content with consistent
// margins. Splits into logo header, screen content, and bottom bar.
// Every screen renders inside the same frame for visual consistency.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Block;

use super::theme::{ROUNDED_BORDERS, Theme};

/// Logo height (6 lines + 1 spacing).
const LOGO_HEIGHT: u16 = 7;

/// Responsive horizontal margin based on terminal width.
fn margin_x(width: u16) -> u16 {
    if width >= 100 {
        12
    } else if width >= 80 {
        6
    } else {
        2
    }
}

/// Responsive vertical margin based on terminal height.
fn margin_y(height: u16) -> u16 {
    if height >= 30 { 2 } else { 1 }
}

/// Shared app frame — centers content with consistent margins.
///
/// Returns `(logo_area, content_area, bottom_bar_area)`.
/// All screens render into the same frame structure.
pub fn app_frame(area: Rect) -> (Rect, Rect, Rect) {
    let mx = margin_x(area.width);
    let my = margin_y(area.height);
    let content_w = area.width.saturating_sub(mx * 2);
    let content_h = area.height.saturating_sub(my * 2);
    let centered = center_rect(area, content_w, content_h);

    let [logo, content, bottom] = Layout::vertical([
        Constraint::Length(LOGO_HEIGHT),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(centered);

    (logo, content, bottom)
}

/// Compute viewport height for scrollable screens (picker, detail).
///
/// Subtracts the app frame chrome (margins, logo, bottom bar, panel borders)
/// and screen-specific overhead from the terminal height.
pub fn viewport_height(term_height: u16, screen_chrome: u16) -> usize {
    let my = margin_y(term_height);
    // margins (top + bottom) + logo + bottom bar + content_panel borders
    let frame_chrome = my * 2 + LOGO_HEIGHT + 1 + 2;
    let total = frame_chrome + screen_chrome;
    (term_height as usize).saturating_sub(total as usize)
}

/// Center a rect of given size within a larger area.
pub fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

/// Intersperse characters with spaces for spaced-out titles.
pub fn space_out(s: &str) -> String {
    s.chars()
        .map(|c| c.to_uppercase().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Wrap screen content in a bordered panel with spaced-out title.
///
/// No extra margins — `app_frame()` handles centering. This draws
/// a titled border filling the given area and returns the inner rect.
pub fn content_panel(frame: &mut ratatui::Frame, theme: &Theme, area: Rect, title: &str) -> Rect {
    let block = Block::bordered()
        .title(format!(" {} ", space_out(title)))
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}
