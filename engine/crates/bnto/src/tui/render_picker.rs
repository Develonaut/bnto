// Render the file picker screen — directory path + file list with multi-select.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::theme::{ROUNDED_BORDERS, Theme};
use super::widgets::file_list;

/// Render the file picker screen.
pub fn draw_picker(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(picker) = &model.picker else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Current directory path
    lines.push(Line::from(Span::styled(
        format!("  {}", picker.current_dir.display()),
        theme.heading(),
    )));
    lines.push(Line::from(""));

    // Selection count
    let count = picker.selected.len();
    if count > 0 {
        lines.push(Line::from(Span::styled(
            format!(
                "  {count} file{} selected",
                if count == 1 { "" } else { "s" }
            ),
            theme.category(),
        )));
        lines.push(Line::from(""));
    }

    // File list
    if picker.entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No matching files",
            theme.muted(),
        )));
    } else {
        let file_lines =
            file_list::render_file_list(&picker.entries, picker.cursor, &picker.selected, theme);
        lines.extend(file_lines);
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
