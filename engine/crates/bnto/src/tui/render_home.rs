// Home screen renderer — dispatches to bento grid or simple fallback.
//
// Large terminals (≥60 cols, ≥20 rows) get the full bento grid layout.
// Small terminals get a simple linear menu.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::render_home_grid::draw_home_grid;
use super::screens::home::HomePane;
use super::theme::{ROUNDED_BORDERS, Theme};

/// Minimum terminal size for bento grid layout.
const MIN_GRID_WIDTH: u16 = 60;
const MIN_GRID_HEIGHT: u16 = 20;

/// Render the home screen — grid for large terminals, simple list for small.
pub fn draw_home(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    if area.width >= MIN_GRID_WIDTH && area.height >= MIN_GRID_HEIGHT {
        draw_home_grid(frame, model, theme, area);
    } else {
        draw_home_simple(frame, model, theme, area);
    }
}

/// Simple linear menu fallback for small terminals.
fn draw_home_simple(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let home = &model.home;
    let items: [(HomePane, &str, &str); 4] = [
        (HomePane::Library, "My Library", "Your recipes"),
        (HomePane::Recipes, "Recipes", "Browse & discover"),
        (HomePane::NewRecipe, "New Recipe", "Create from scratch"),
        (HomePane::Settings, "Settings", "Preferences"),
    ];

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (pane, label, desc) in &items {
        let is_selected = home.focused == *pane;
        let marker = if is_selected { "▸ " } else { "  " };
        let name_style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };

        let badge = if matches!(pane, HomePane::Library) && !home.library_names.is_empty() {
            format!(" ({})", home.library_names.len())
        } else {
            String::new()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{label}{badge}"), name_style),
            Span::styled(format!("  {desc}"), theme.muted()),
        ]));
        lines.push(Line::from(""));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
