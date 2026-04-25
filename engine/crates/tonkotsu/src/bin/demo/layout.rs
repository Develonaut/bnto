//! App frame layout for the demo binary.
//!
//! Splits the terminal into three areas: logo, bordered content panel,
//! and bottom help bar. Mirrors the bnto TUI layout with responsive margins.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::symbols;
use ratatui::widgets::Block;

use super::logo::LOGO_HEIGHT;

/// Responsive horizontal margin based on terminal width.
fn margin_x(width: u16) -> u16 {
    match width {
        w if w >= 100 => 12,
        w if w >= 80 => 6,
        _ => 2,
    }
}

/// Responsive vertical margin based on terminal height.
fn margin_y(height: u16) -> u16 {
    if height >= 30 { 2 } else { 1 }
}

/// Center a rect of `w x h` inside `outer`.
fn center_rect(outer: Rect, w: u16, h: u16) -> Rect {
    let x = outer.x + (outer.width.saturating_sub(w)) / 2;
    let y = outer.y + (outer.height.saturating_sub(h)) / 2;
    Rect::new(x, y, w.min(outer.width), h.min(outer.height))
}

/// Split the terminal into (logo, content, bottom) areas with responsive margins.
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

/// Render a bordered content panel and return the inner area.
pub fn content_panel(
    frame: &mut ratatui::Frame,
    border_style: ratatui::style::Style,
    heading_style: ratatui::style::Style,
    area: Rect,
    title: &str,
) -> Rect {
    let spaced: String = title
        .chars()
        .map(|c| c.to_uppercase().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let block = Block::bordered()
        .title(format!(" {spaced} "))
        .title_style(heading_style)
        .border_set(symbols::border::ROUNDED)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

/// Calculate the usable viewport height for the form inside the content panel.
/// Subtracts border (2) from the content area height.
pub fn viewport_height(term_height: u16) -> usize {
    let my = margin_y(term_height);
    let available = term_height.saturating_sub(my * 2);
    // Total frame: LOGO_HEIGHT + border(2) + bottom(1)
    let overhead = LOGO_HEIGHT + 2 + 1;
    available.saturating_sub(overhead) as usize
}
