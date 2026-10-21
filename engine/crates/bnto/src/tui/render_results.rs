// Render the results screen — output files, sizes, savings, timing.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::results::{format_duration, format_size};
use super::screens::size_change::SizeChange;
use super::theme::Theme;

/// Render the results screen.
pub fn draw_results(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

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
        let summary = match savings.change() {
            Some(change) => format!("  {}: {before} → {after} ({change})", change.heading()),
            None => format!("  Size: {before} → {after}"),
        };
        lines.push(Line::from(Span::styled(summary, theme.category())));
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
            // Show how each file's size changed when original_size is available.
            let size_info = if let Some(orig) = file.original_size {
                let orig_str = format_size(orig);
                let new_str = format_size(file.size_bytes);
                match SizeChange::between(orig, file.size_bytes) {
                    Some(change) => format!("  {orig_str} → {new_str}  ({change})"),
                    None => format!("  {orig_str} → {new_str}"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::screens::results::{OutputFile, ResultsModel};
    use crate::tui::theme::ThemeVariant;

    fn rendered(outputs: Vec<OutputFile>) -> Vec<String> {
        let results = ResultsModel::new("s", outputs, 0, None);
        let theme = Theme::from_variant(ThemeVariant::LosAngeles);
        results_lines(&results, &theme)
            .iter()
            .map(|line| line.to_string())
            .collect()
    }

    #[test]
    fn grown_output_reads_as_larger_not_zero_percent_smaller() {
        let lines = rendered(vec![OutputFile {
            name: "gradient.webp".into(),
            size_bytes: 10_240,
            original_size: Some(5_120),
        }]);
        assert!(lines.contains(&"  Size: 5 KB → 10 KB (100% larger)".to_string()));
        assert!(
            lines
                .iter()
                .any(|l| l.ends_with("5 KB → 10 KB  (100% larger)"))
        );
        assert!(!lines.iter().any(|l| l.contains("smaller")));
    }

    #[test]
    fn shrunk_output_keeps_savings_wording() {
        let lines = rendered(vec![OutputFile {
            name: "photo.jpg".into(),
            size_bytes: 5_120,
            original_size: Some(10_240),
        }]);
        assert!(lines.contains(&"  Savings: 10 KB → 5 KB (50% smaller)".to_string()));
        assert!(
            lines
                .iter()
                .any(|l| l.ends_with("10 KB → 5 KB  (50% smaller)"))
        );
    }
}
