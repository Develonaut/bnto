// Home pane content renderers — list building for bento grid panes.
//
// Separated from the grid layout orchestration to keep each file focused.

use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_home_grid::pane_block;
use super::render_layout::space_out;
use super::theme::Theme;

/// Draw the Library pane — user's saved recipe files.
pub fn draw_library_pane(
    frame: &mut ratatui::Frame,
    model: &AppModel,
    theme: &Theme,
    area: ratatui::layout::Rect,
    focused: bool,
) {
    let block = pane_block(&space_out("Library"), theme, focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let home = &model.home;
    let mut lines: Vec<Line> = Vec::new();

    if home.library_names.is_empty() {
        lines.push(Line::from(Span::styled("  No recipes yet", theme.muted())));
    } else {
        for (i, name) in home.library_names.iter().enumerate() {
            let is_selected = focused && i == home.library_cursor;
            let marker = if is_selected { " ▸ " } else { "   " };
            let style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };
            lines.push(Line::from(Span::styled(format!("{marker}{name}"), style)));
        }
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draw the Recipes pane — category-grouped recipe catalog.
pub fn draw_recipes_pane(
    frame: &mut ratatui::Frame,
    model: &AppModel,
    theme: &Theme,
    area: ratatui::layout::Rect,
    focused: bool,
) {
    let block = pane_block(&space_out("Recipes"), theme, focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let browser = &model.browser;
    let cursor = model.home.recipes_cursor;
    let mut lines: Vec<Line> = Vec::new();
    let mut current_category = String::new();
    for (list_idx, &recipe_idx) in browser.filtered.iter().enumerate() {
        let recipe = &browser.recipes[recipe_idx];

        if recipe.category != current_category {
            if !current_category.is_empty() {
                lines.push(Line::from(""));
            }
            current_category = recipe.category.clone();
            lines.push(Line::from(Span::styled(
                format!("  {}", current_category.to_uppercase()),
                theme.category(),
            )));
        }

        let is_selected = focused && list_idx == cursor;
        let marker = if is_selected { " ▸ " } else { "   " };
        let style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };
        lines.push(Line::from(Span::styled(
            format!("{marker}{}", recipe.name),
            style,
        )));
    }

    if browser.filtered.is_empty() {
        lines.push(Line::from(Span::styled("  No recipes", theme.muted())));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Draw a single-line action pane (NewRecipe or Settings).
pub fn draw_action_pane(
    frame: &mut ratatui::Frame,
    theme: &Theme,
    area: ratatui::layout::Rect,
    title: &str,
    description: &str,
    focused: bool,
) {
    let block = pane_block(title, theme, focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let style = if focused { theme.text() } else { theme.muted() };
    let line = Line::from(Span::styled(format!("  {description}"), style));
    let content = Paragraph::new(vec![line]);
    frame.render_widget(content, inner);
}
