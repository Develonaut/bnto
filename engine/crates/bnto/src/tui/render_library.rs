// Render the Library screen — search bar + recipe list with actions.
//
// Supports search filtering, delete confirmation overlay, and inline rename mode.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::theme::Theme;
use super::widgets::search_input;

/// Draw the Library screen content area.
pub fn draw_library(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());
    let lib = match &model.library {
        Some(l) => l,
        None => return,
    };

    let mut lines: Vec<Line> = Vec::new();

    // Search input row.
    if let Some(search_line) =
        search_input::render_search_input(&lib.search_query, lib.searching, theme)
    {
        lines.push(search_line);
        lines.push(Line::from(""));
    }

    // Empty state.
    if lib.entries.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Your library is empty.",
            theme.muted(),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Browse Recipes and press 'a' to add one.",
            theme.muted(),
        )));
    } else if lib.filtered.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  No matching recipes.",
            theme.muted(),
        )));
    } else {
        // Delete confirmation overlay.
        if let Some(del_idx) = lib.confirming_delete {
            let name = lib
                .entries
                .get(del_idx)
                .map(|e| e.name.as_str())
                .unwrap_or("recipe");
            lines.push(Line::from(Span::styled(
                format!("  Delete '{name}'? (y/n)"),
                theme.heading(),
            )));
            lines.push(Line::from(""));
        }

        for (list_idx, &entry_idx) in lib.filtered.iter().enumerate() {
            let entry = &lib.entries[entry_idx];
            let is_selected = list_idx == lib.cursor;

            // Rename mode for this entry.
            if let Some((rename_idx, ref buf)) = lib.renaming
                && rename_idx == entry_idx
            {
                let prefix = if is_selected { "▸ " } else { "  " };
                let style = theme.heading();
                lines.push(Line::from(Span::styled(format!("{prefix}{buf}▏"), style)));
                continue;
            }

            let prefix = if is_selected { "▸ " } else { "  " };
            let name_style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };
            let desc_style = theme.muted();

            lines.push(Line::from(vec![
                Span::styled(prefix, name_style),
                Span::styled(&entry.name, name_style),
            ]));
            if !entry.description.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("    {}", entry.description),
                    desc_style,
                )));
            }
        }
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
