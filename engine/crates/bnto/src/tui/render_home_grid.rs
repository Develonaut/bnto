// Bento grid layout — home screen with paned compartments.
//
// Renders when terminal is large enough (≥60 cols, ≥10 rows).
// Four panes: Library (top-left), Recipes (right), NewRecipe + Settings (bottom-left).
// The app frame handles centering, margins, logo, and bottom bar.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};

use super::app::AppModel;
use super::render_home_panes::{draw_action_pane, draw_library_pane, draw_recipes_pane};
use super::render_layout::space_out;
use super::screens::home::HomePane;
use super::theme::{ROUNDED_BORDERS, Theme};

/// Render the bento grid home screen.
///
/// Receives the content area from `app_frame()` — already centered with
/// logo above and bottom bar below.
pub fn draw_home_grid(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    // Horizontal split: left column (35%) + right column.
    let left_width = (area.width * 35) / 100;
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Length(left_width), Constraint::Min(1)]).areas(area);

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
