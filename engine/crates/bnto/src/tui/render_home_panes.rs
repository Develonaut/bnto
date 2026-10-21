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
    let mut selected_line = None;

    if home.library_names.is_empty() {
        lines.push(Line::from(Span::styled("  No recipes yet", theme.muted())));
    } else {
        for (i, name) in home.library_names.iter().enumerate() {
            let is_selected = focused && i == home.library_cursor;
            if is_selected {
                selected_line = Some(lines.len());
            }
            lines.push(item_line(name, is_selected, theme));
        }
    }

    let offset = scroll_to_show(selected_line, inner.height);
    frame.render_widget(Paragraph::new(lines).scroll((offset, 0)), inner);
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

    let (lines, selected_line) = recipe_lines(model, theme, focused);
    let offset = scroll_to_show(selected_line, inner.height);
    frame.render_widget(Paragraph::new(lines).scroll((offset, 0)), inner);
}

/// The catalog grouped under category headings, and the line of the selected
/// recipe (if the pane is focused).
fn recipe_lines<'a>(
    model: &'a AppModel,
    theme: &Theme,
    focused: bool,
) -> (Vec<Line<'a>>, Option<usize>) {
    let browser = &model.browser;
    let mut lines: Vec<Line> = Vec::new();
    let mut selected_line = None;
    let mut current_category = "";
    for (list_idx, &recipe_idx) in browser.filtered.iter().enumerate() {
        let recipe = &browser.recipes[recipe_idx];
        if recipe.category != current_category {
            if !current_category.is_empty() {
                lines.push(Line::from(""));
            }
            current_category = &recipe.category;
            let heading = format!("  {}", current_category.to_uppercase());
            lines.push(Line::from(Span::styled(heading, theme.category())));
        }
        let is_selected = focused && list_idx == model.home.recipes_cursor;
        if is_selected {
            selected_line = Some(lines.len());
        }
        lines.push(item_line(&recipe.name, is_selected, theme));
    }
    if browser.filtered.is_empty() {
        lines.push(Line::from(Span::styled("  No recipes", theme.muted())));
    }
    (lines, selected_line)
}

/// One list row, with the `▸` marker and selected style when it's selected.
fn item_line(name: &str, is_selected: bool, theme: &Theme) -> Line<'static> {
    let (marker, style) = if is_selected {
        (" ▸ ", theme.selected())
    } else {
        ("   ", theme.text())
    };
    Line::from(Span::styled(format!("{marker}{name}"), style))
}

/// Rows to scroll a pane `height` rows tall so line `selected` shows: none
/// until it would fall off the bottom, then just enough to keep it in view.
fn scroll_to_show(selected: Option<usize>, height: u16) -> u16 {
    let Some(line) = selected else { return 0 };
    let offset = (line + 1).saturating_sub(usize::from(height));
    u16::try_from(offset).unwrap_or(u16::MAX)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::paths::BntoPaths;
    use crate::tui::theme::ThemeVariant;

    fn model(dir: &tempfile::TempDir) -> AppModel {
        let paths = BntoPaths {
            home: dir.path().to_path_buf(),
        };
        AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths)
    }

    /// The text a pane draws into a `width`×`height` area, one string per row.
    fn rows(
        model: &AppModel,
        height: u16,
        draw: fn(&mut ratatui::Frame, &AppModel, &Theme, ratatui::layout::Rect, bool),
    ) -> Vec<String> {
        let backend = ratatui::backend::TestBackend::new(50, height);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal
            .draw(|f| draw(f, model, &model.theme, f.area(), true))
            .unwrap();
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| (0..50).map(|x| buffer[(x, y)].symbol()).collect())
            .collect()
    }

    fn selected_row(rows: &[String]) -> Option<&String> {
        rows.iter().find(|row| row.contains('▸'))
    }

    #[test]
    fn scrolling_starts_only_once_the_selection_would_fall_off() {
        assert_eq!(scroll_to_show(None, 10), 0);
        assert_eq!(scroll_to_show(Some(0), 10), 0);
        assert_eq!(scroll_to_show(Some(9), 10), 0);
        assert_eq!(scroll_to_show(Some(10), 10), 1);
        assert_eq!(scroll_to_show(Some(25), 10), 16);
        assert_eq!(scroll_to_show(Some(3), 0), 4);
    }

    #[test]
    fn the_last_recipe_stays_in_view_when_selected() {
        let dir = tempfile::tempdir().unwrap();
        let mut model = model(&dir);
        let last = model.browser.filtered.len() - 1;
        model.home.recipes_cursor = last;
        let name = model.browser.recipes[model.browser.filtered[last]]
            .name
            .clone();

        let rows = rows(&model, 12, draw_recipes_pane);
        let row = selected_row(&rows).expect("the selected recipe is off screen");
        assert!(row.contains(&name), "selected {row:?}, expected {name}");
    }

    #[test]
    fn the_first_recipe_shows_its_category_heading() {
        let dir = tempfile::tempdir().unwrap();
        let model = model(&dir);
        let rows = rows(&model, 12, draw_recipes_pane);
        assert!(selected_row(&rows).is_some());
        assert!(rows[1].contains("FILE"), "top row: {:?}", rows[1]);
    }

    #[test]
    fn a_selected_library_recipe_stays_in_view() {
        let dir = tempfile::tempdir().unwrap();
        let mut model = model(&dir);
        model.home.library_names = (0..30).map(|i| format!("saved-{i}")).collect();
        model.home.library_cursor = 29;

        let rows = rows(&model, 8, draw_library_pane);
        let row = selected_row(&rows).expect("the selected library recipe is off screen");
        assert!(row.contains("saved-29"), "{row:?}");
    }
}
