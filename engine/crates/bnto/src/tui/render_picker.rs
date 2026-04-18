// Render the file picker screen — directory path + file list with multi-select.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::theme::Theme;
use super::widgets::file_list;

/// Render the file picker screen.
pub fn draw_picker(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(picker) = &model.picker else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Current directory path + hidden indicator
    let hidden_label = if picker.show_hidden {
        "  Hidden: on"
    } else {
        ""
    };
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {}", picker.current_dir.display()),
            theme.heading(),
        ),
        Span::styled(hidden_label.to_string(), theme.muted()),
    ]));
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

    // File list — viewport-aware slice
    if picker.entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No matching files",
            theme.muted(),
        )));
    } else {
        let offset = picker.viewport_offset;
        let end = (offset + picker.viewport_height).min(picker.entries.len());
        let visible = &picker.entries[offset..end];
        let display_cursor = picker.cursor.saturating_sub(offset);
        let file_lines =
            file_list::render_file_list(visible, display_cursor, &picker.selected, theme);
        lines.extend(file_lines);
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
