// Bento grid layout — centered home screen with ASCII logo and paned compartments.
//
// Renders when terminal is large enough (≥60 cols, ≥20 rows).
// Four panes: Library (top-left), Recipes (right), NewRecipe + Settings (bottom-left).

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::app::AppModel;
use super::render_home_logo::logo_lines;
use super::render_home_panes::{draw_action_pane, draw_library_pane, draw_recipes_pane};
use super::render_layout::{center_rect, space_out};
use super::screens::home::HomePane;
use super::theme::{ROUNDED_BORDERS, Theme};
use super::widgets::{help_bar, status_line};

/// Horizontal margin on each side of the bento grid.
const MARGIN_X: u16 = 12;
/// Vertical margin on top and bottom of the bento grid.
const MARGIN_Y: u16 = 2;

/// Help hints rendered below the bento grid (Home owns its own hints).
const HOME_HINTS: [(&str, &str); 4] = [
    ("Tab", "pane"),
    ("↑↓", "navigate"),
    ("Enter", "select"),
    ("q", "quit"),
];

/// Render the bento grid home screen.
pub fn draw_home_grid(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    // Fill the terminal minus small margins on each side.
    let content_w = area.width.saturating_sub(MARGIN_X * 2);
    let content_h = area.height.saturating_sub(MARGIN_Y * 2);
    let centered = center_rect(area, content_w, content_h);

    // Vertical: logo (7 rows) + grid (flex) + bottom bar (1 row).
    let [logo_area, grid_area, hints_area] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(centered);

    draw_logo(frame, theme, logo_area);
    draw_bottom_bar(frame, model, theme, hints_area);

    // Horizontal split: left column (35%) + right column.
    let left_width = (grid_area.width * 35) / 100;
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Length(left_width), Constraint::Min(1)]).areas(grid_area);

    // Left column: Library (flex) + NewRecipe (3 rows) + Settings (3 rows).
    let [library_area, new_recipe_area, settings_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(left_area);

    let focused = model.home.focused;
    draw_library_pane(
        frame,
        model,
        theme,
        library_area,
        focused == HomePane::Library,
    );
    draw_action_pane(
        frame,
        theme,
        new_recipe_area,
        &space_out("New"),
        "Create from scratch",
        focused == HomePane::NewRecipe,
    );
    draw_action_pane(
        frame,
        theme,
        settings_area,
        &space_out("Settings"),
        "Preferences",
        focused == HomePane::Settings,
    );

    // Right column: Recipes pane fills the full height.
    draw_recipes_pane(
        frame,
        model,
        theme,
        right_area,
        focused == HomePane::Recipes,
    );
}

/// Draw the ASCII logo left-aligned in the logo area.
fn draw_logo(frame: &mut ratatui::Frame, theme: &Theme, area: Rect) {
    let lines = logo_lines(theme);
    let logo = Paragraph::new(lines);
    frame.render_widget(logo, area);
}

/// Draw the bottom bar: help hints left-aligned, status info right-aligned.
fn draw_bottom_bar(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let hints = help_bar::render_help_bar(&HOME_HINTS, theme);
    frame.render_widget(Paragraph::new(hints), area);

    let version = env!("CARGO_PKG_VERSION");
    let status = status_line::render_status_line_with_message(
        model.browser.recipes.len(),
        version,
        model.theme_variant.display_name(),
        model.status_message.as_deref(),
        theme,
    );
    frame.render_widget(Paragraph::new(status).alignment(Alignment::Right), area);
}

/// Build a pane block with focused/unfocused border styling.
pub fn pane_block(title: &str, theme: &Theme, focused: bool) -> Block<'static> {
    let border_style = if focused {
        theme.border()
    } else {
        theme.border_muted()
    };
    let title_style = if focused {
        theme.selected()
    } else {
        theme.muted()
    };
    Block::new()
        .borders(Borders::ALL)
        .border_set(ROUNDED_BORDERS)
        .border_style(border_style)
        .title(format!(" {title} "))
        .title_style(title_style)
}
