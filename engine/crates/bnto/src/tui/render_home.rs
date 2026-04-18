// Home screen renderer — dispatches to bento grid or simple fallback.
//
// Large content areas (≥60 cols, ≥10 rows) get the full bento grid layout.
// Small content areas get a simple linear menu.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_home_grid::draw_home_grid;
use super::render_layout::content_panel;
use super::screens::home::HomePane;
use super::theme::Theme;

/// Minimum content area size for bento grid layout.
/// Height is lower than before — logo is now rendered by the app frame.
const MIN_GRID_WIDTH: u16 = 60;
const MIN_GRID_HEIGHT: u16 = 10;

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
    let inner = content_panel(frame, theme, area, model.screen.title());

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
