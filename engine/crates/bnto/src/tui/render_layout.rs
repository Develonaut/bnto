// Shared layout helpers — centering, spacing, and content panels.
//
// Used by the Home grid and all inner-screen renderers for
// visual consistency (centered content with margins).

use ratatui::layout::Rect;
use ratatui::widgets::Block;

use super::theme::{ROUNDED_BORDERS, Theme};

/// Horizontal margin on each side of inner-screen content panels.
const MARGIN_X: u16 = 6;
/// Vertical margin on top and bottom of inner-screen content panels.
const MARGIN_Y: u16 = 1;

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

/// Wrap screen content in a centered panel with margins and spaced-out title.
///
/// Returns the inner rect where content should be rendered.
pub fn content_panel(frame: &mut ratatui::Frame, theme: &Theme, area: Rect, title: &str) -> Rect {
    let content_w = area.width.saturating_sub(MARGIN_X * 2);
    let content_h = area.height.saturating_sub(MARGIN_Y * 2);
    let centered = center_rect(area, content_w, content_h);

    let block = Block::bordered()
        .title(format!(" {} ", space_out(title)))
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());
    let inner = block.inner(centered);
    frame.render_widget(block, centered);
    inner
}
