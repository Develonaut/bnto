// Render functions — draw each TUI screen to the terminal frame.
//
// Extracted from mod.rs to keep files under 250 lines.
// Each function takes a Frame, model/screen, Theme, and area.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::{AppModel, Screen};
use super::theme::{ALL_VARIANTS, ROUNDED_BORDERS, Theme};
use super::widgets::{help_bar, search_input, status_line};

/// Render the main content area based on the current screen.
pub fn draw_content(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    match &model.screen {
        Screen::Browser => draw_browser(frame, model, theme, area),
        Screen::Settings => draw_settings(frame, model, theme, area),
        screen => draw_placeholder(frame, screen, theme, area),
    }
}

/// Render the recipe browser — search bar + category-grouped recipe list.
fn draw_browser(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let browser = &model.browser;
    let mut lines: Vec<Line> = Vec::new();

    // Search input row (delegated to widget)
    if let Some(search_line) =
        search_input::render_search_input(&browser.search_query, browser.searching, theme)
    {
        lines.push(search_line);
        lines.push(Line::from(""));
    }

    // Group filtered recipes by category, preserving order
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

        let is_selected = list_idx == browser.cursor;
        let marker = if is_selected { "▸ " } else { "  " };
        let name_style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}", recipe.name), name_style),
            Span::styled(format!("  {}", recipe.description), theme.muted()),
        ]));
    }

    if browser.filtered.is_empty() {
        lines.push(Line::from(Span::styled("  No matches", theme.muted())));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Render a placeholder screen (used for screens not yet implemented).
fn draw_placeholder(frame: &mut ratatui::Frame, screen: &Screen, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let content = Paragraph::new(screen.placeholder_label())
        .style(theme.text())
        .block(block);
    frame.render_widget(content, area);
}

/// Render the settings screen with theme picker.
fn draw_settings(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let variant_labels = [
        ("Los Angeles", "Warm terracotta accent"),
        ("Tokyo", "Cool electric blue accent"),
        ("Monaco", "Golden-hour amber accent"),
    ];

    let lines: Vec<Line> = ALL_VARIANTS
        .iter()
        .zip(variant_labels.iter())
        .map(|(variant, (label, desc))| {
            let is_selected = *variant == model.theme_variant;
            let marker = if is_selected { "▸ " } else { "  " };
            let style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };
            Line::from(vec![
                Span::styled(format!("{marker}{label}"), style),
                Span::styled(format!("  {desc}"), theme.muted()),
            ])
        })
        .collect();

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Render the bottom bar: status line (left) + help hints (right).
pub fn draw_help_bar(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let hints = model.screen.help_hints();
    let line = help_bar::render_help_bar(&hints, theme);
    let bar = Paragraph::new(line);
    frame.render_widget(bar, area);
}

/// Render the status line with recipe count, version, and theme name.
pub fn draw_status_line(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let recipe_count = model.browser.recipes.len();
    let version = env!("CARGO_PKG_VERSION");
    let theme_name = model.theme_variant.display_name();
    let line = status_line::render_status_line(recipe_count, version, theme_name, theme);
    let bar = Paragraph::new(line);
    frame.render_widget(bar, area);
}
