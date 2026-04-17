// Home screen renderer — main menu with navigation items.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::screens::home::{HOME_ITEMS, HomeItem};
use super::theme::{ROUNDED_BORDERS, Theme};

/// Render the home menu with navigation items and library count badge.
pub fn draw_home(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let home = &model.home;
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, item) in HOME_ITEMS.iter().enumerate() {
        let is_selected = i == home.selected;
        let marker = if is_selected { "▸ " } else { "  " };
        let name_style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };

        let badge = if matches!(item, HomeItem::Library) && home.library_count > 0 {
            format!(" ({})", home.library_count)
        } else {
            String::new()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}{badge}", item.label()), name_style),
            Span::styled(format!("  {}", item.description()), theme.muted()),
        ]));
        lines.push(Line::from(""));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
