// Render functions — draw each TUI screen to the terminal frame.
//
// Per-screen renderers live in sibling modules (render_detail, render_picker)
// to keep each file under 250 lines.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::{AppModel, Screen};
use super::render_detail::draw_detail;
use super::render_execution::draw_execution;
use super::render_picker::draw_picker;
use super::render_results::draw_results;
use super::theme::{ROUNDED_BORDERS, Theme};
use super::widgets::{help_bar, search_input, status_line};

/// Render the main content area based on the current screen.
pub fn draw_content(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    match &model.screen {
        Screen::Browser => draw_browser(frame, model, theme, area),
        Screen::Detail { .. } => draw_detail(frame, model, theme, area),
        Screen::Picker { .. } => draw_picker(frame, model, theme, area),
        Screen::Execution { .. } => draw_execution(frame, model, theme, area),
        Screen::Results { .. } => draw_results(frame, model, theme, area),
        Screen::Settings => draw_settings(frame, model, theme, area),
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

/// Render the settings screen with multi-field form.
///
/// Theme field shows the current variant with left/right arrows.
/// Path fields show their value (Enter opens file picker to browse).
fn draw_settings(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let settings = match &model.settings {
        Some(s) => s,
        None => return,
    };

    let mut lines: Vec<Line> = Vec::new();

    for (i, field) in settings.fields.iter().enumerate() {
        let is_focused = i == settings.focused;
        let marker = if is_focused { "▸ " } else { "  " };
        let label_style = if is_focused {
            theme.selected()
        } else {
            theme.text()
        };

        // Label row.
        lines.push(Line::from(Span::styled(
            format!("{marker}{}", field.label),
            label_style,
        )));

        // Value row.
        let value_display = if field.key == "theme" {
            format!("    ◂ {} ▸", model.theme_variant.display_name())
        } else if field.key == "telemetry" {
            format!("    ◂ {} ▸", field.value)
        } else if field.value.is_empty() {
            "    (not set)".to_string()
        } else {
            format!("    {}", field.value)
        };

        let value_style = theme.muted();
        lines.push(Line::from(Span::styled(value_display, value_style)));

        // Description row.
        lines.push(Line::from(Span::styled(
            format!("    {}", field.description),
            theme.muted(),
        )));

        // Spacer between fields.
        lines.push(Line::from(""));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Render the bottom bar: status line (left) + help hints (right).
pub fn draw_help_bar(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    // Settings picker shows directory-selection hints instead of file-selection hints.
    let hints = if model.settings_picker_field.is_some()
        && matches!(&model.screen, Screen::Picker { .. })
    {
        vec![
            ("↑↓", "navigate"),
            ("h/l", "parent/enter"),
            (".", "hidden"),
            ("Tab", "select dir"),
            ("Esc", "back"),
        ]
    } else {
        model.screen.help_hints()
    };
    let line = help_bar::render_help_bar(&hints, theme);
    let bar = Paragraph::new(line);
    frame.render_widget(bar, area);
}

/// Render the status line with recipe count, version, theme, and optional status message.
pub fn draw_status_line(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let recipe_count = model.browser.recipes.len();
    let version = env!("CARGO_PKG_VERSION");
    let theme_name = model.theme_variant.display_name();
    let line = status_line::render_status_line_with_message(
        recipe_count,
        version,
        theme_name,
        model.status_message.as_deref(),
        theme,
    );
    let bar = Paragraph::new(line);
    frame.render_widget(bar, area);
}
