// Render the preview screen — file transformation table (original → result).

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::results::format_duration;
use super::theme::Theme;

/// Render the preview screen.
pub fn draw_preview(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(preview) = &model.preview else {
        let fallback = Paragraph::new("No preview available.").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = preview_lines(preview, theme);
    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Build lines for the preview screen content.
fn preview_lines<'a>(
    preview: &'a super::screens::preview::PreviewModel,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Timing header
    let elapsed = format_duration(preview.duration_ms);
    lines.push(Line::from(vec![
        Span::styled("  Preview completed in ", theme.text()),
        Span::styled(elapsed, theme.heading()),
    ]));
    lines.push(Line::from(""));

    // Warnings
    for warning in &preview.warnings {
        lines.push(Line::from(Span::styled(
            format!("  ⚠ {warning}"),
            theme.category(),
        )));
    }
    if !preview.warnings.is_empty() {
        lines.push(Line::from(""));
    }

    // File transformation list with cursor
    if preview.entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No file transformations.",
            theme.muted(),
        )));
    } else {
        let count = preview.entries.len();
        lines.push(Line::from(Span::styled(
            format!(
                "  FILE TRANSFORMATIONS ({count} file{})",
                if count == 1 { "" } else { "s" }
            ),
            theme.category(),
        )));

        for (i, entry) in preview.entries.iter().enumerate() {
            let is_selected = i == preview.cursor;
            let marker = if is_selected { "▸ " } else { "  " };
            let name_style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };

            if entry.original == entry.result {
                // No change — show filename only (dimmed).
                lines.push(Line::from(Span::styled(
                    format!("  {marker}{}", entry.result),
                    theme.muted(),
                )));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {marker}{}", entry.original), name_style),
                    Span::styled("  →  ", theme.muted()),
                    Span::styled(entry.result.as_str(), theme.heading()),
                ]));
            }
        }
    }

    lines
}
