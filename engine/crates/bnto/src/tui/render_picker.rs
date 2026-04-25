// Render the file picker screen — breadcrumb path + search + file list with multi-select.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::theme::Theme;
use super::widgets::{file_list, search_input};

/// Render the file picker screen.
pub fn draw_picker(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(picker) = &model.picker else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Breadcrumb path with status indicators
    let breadcrumb = render_breadcrumb(&picker.current_dir, inner.width as usize);
    let mut path_spans = vec![Span::styled(format!("  {breadcrumb}"), theme.heading())];

    if picker.show_hidden {
        path_spans.push(Span::styled("  Hidden: on", theme.muted()));
    }
    if picker.show_metadata {
        path_spans.push(Span::styled("  Meta: on", theme.muted()));
    }
    lines.push(Line::from(path_spans));

    // Search input row
    if let Some(search_line) =
        search_input::render_search_input(&picker.query, picker.searching, theme)
    {
        lines.push(search_line);
    }

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

    // File list — use filtered entries when search is active
    let filtered = picker.filtered_entries();
    if filtered.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No matching files",
            theme.muted(),
        )));
    } else {
        let offset = picker.viewport_offset;
        let end = (offset + picker.viewport_height).min(filtered.len());
        let visible: Vec<_> = filtered[offset..end].to_vec();
        let display_cursor = picker.cursor.saturating_sub(offset);

        // Convert &FileEntry refs to owned slice for render_file_list
        let owned: Vec<_> = visible.iter().map(|e| (*e).clone()).collect();
        let file_lines = file_list::render_file_list(
            &owned,
            display_cursor,
            &picker.selected,
            theme,
            picker.show_metadata,
        );
        lines.extend(file_lines);
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Render a directory path as a breadcrumb with `/` separators.
///
/// Truncates from the left when the path is too wide, showing `…/` prefix.
/// Home directory (`~`) abbreviation is applied when possible.
fn render_breadcrumb(path: &std::path::Path, max_width: usize) -> String {
    let display = abbreviate_home(path);

    // Leave room for the "  " prefix in the caller
    let available = max_width.saturating_sub(4);
    if display.len() <= available {
        return display;
    }

    // Truncate from left: show "…/" + as many trailing segments as fit
    let parts: Vec<&str> = display.split('/').collect();
    let mut result = String::new();
    for part in parts.iter().rev() {
        let candidate = if result.is_empty() {
            part.to_string()
        } else {
            format!("{part}/{result}")
        };
        if candidate.len() + 2 > available {
            break;
        }
        result = candidate;
    }

    format!("…/{result}")
}

/// Replace the user's home directory with `~` for shorter display.
fn abbreviate_home(path: &std::path::Path) -> String {
    if let Some(home) = dirs::home_dir()
        && let Ok(relative) = path.strip_prefix(&home)
    {
        return format!("~/{}", relative.display());
    }
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn breadcrumb_short_path_unchanged() {
        let path = PathBuf::from("/usr/local/bin");
        let result = render_breadcrumb(&path, 80);
        assert!(
            result.contains("local") && result.contains("bin"),
            "short path should render fully: {result}"
        );
    }

    #[test]
    fn breadcrumb_truncates_long_path() {
        let path = PathBuf::from("/very/long/deeply/nested/directory/structure/here");
        let result = render_breadcrumb(&path, 30);
        assert!(
            result.starts_with('…'),
            "long path should start with …: {result}"
        );
        assert!(
            result.len() <= 30,
            "truncated breadcrumb should fit: {result}"
        );
    }

    #[test]
    fn breadcrumb_preserves_trailing_segments() {
        let path = PathBuf::from("/a/b/c/d/e/final");
        let result = render_breadcrumb(&path, 20);
        assert!(
            result.contains("final"),
            "should keep the last segment: {result}"
        );
    }
}
