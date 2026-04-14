// Render the results screen — output files, sizes, savings, timing.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::screens::results::{file_savings_percent, format_duration, format_size};
use super::theme::{ROUNDED_BORDERS, Theme};

/// Render the results screen.
pub fn draw_results(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(results) = &model.results else {
        let fallback = Paragraph::new("No results available.").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = results_lines(results, theme);
    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Build lines for the results screen content.
fn results_lines<'a>(
    results: &'a super::screens::results::ResultsModel,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Timing header
    let elapsed = format_duration(results.total_time_ms);
    lines.push(Line::from(vec![
        Span::styled("  Completed in ", theme.text()),
        Span::styled(elapsed, theme.heading()),
    ]));
    lines.push(Line::from(""));

    // Savings summary
    if let Some(savings) = &results.savings {
        let before = format_size(savings.before);
        let after = format_size(savings.after);
        let pct = savings.percent();
        lines.push(Line::from(Span::styled(
            format!("  Savings: {before} → {after} ({pct}% smaller)"),
            theme.category(),
        )));
        lines.push(Line::from(""));
    }

    // Output directory
    if let Some(dir) = &results.output_dir {
        lines.push(Line::from(Span::styled(
            format!("  Output: {dir}"),
            theme.muted(),
        )));
        lines.push(Line::from(""));
    }

    // Output file list with cursor
    if results.outputs.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No output files.",
            theme.muted(),
        )));
    } else {
        lines.push(Line::from(Span::styled("  FILES", theme.category())));
        for (i, file) in results.outputs.iter().enumerate() {
            let is_selected = i == results.cursor;
            let marker = if is_selected { "▸ " } else { "  " };
            let name_style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };
            // Show per-file savings when original_size is available.
            let size_info = if let Some(orig) = file.original_size {
                let orig_str = format_size(orig);
                let new_str = format_size(file.size_bytes);
                if let Some(pct) = file_savings_percent(orig, file.size_bytes) {
                    format!("  {orig_str} → {new_str}  ({pct}% smaller)")
                } else {
                    format!("  {orig_str} → {new_str}")
                }
            } else {
                format!("  {}", format_size(file.size_bytes))
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {marker}{}", file.name), name_style),
                Span::styled(size_info, theme.muted()),
            ]));
        }
    }

    lines
}
