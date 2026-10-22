// Render the results screen — output files, sizes, savings, timing.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::render_layout::space_out;
use super::screens::results::{OutputFile, ResultsModel, format_duration, format_size};
use super::screens::size_change::SizeChange;
use super::theme::Theme;
use crate::run_message::RunMessage;

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

/// The most recent command-output lines kept on the results screen.
const MAX_LOG_LINES: usize = 15;

/// Build lines for the results screen content. A message-mode run shows its
/// message in place of files, which it never writes.
fn results_lines<'a>(
    results: &'a super::screens::results::ResultsModel,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let mut lines = timing_lines(results, theme);
    lines.extend(savings_lines(results, theme));
    if let Some(message) = &results.message {
        lines.extend(message_lines(message, theme));
        return lines;
    }
    lines.extend(output_dir_lines(results, theme));
    lines.extend(file_lines(results, theme));
    lines.extend(log_lines(&results.log, theme));
    lines
}

fn timing_lines<'a>(results: &ResultsModel, theme: &Theme) -> Vec<Line<'a>> {
    vec![
        Line::from(vec![
            Span::styled("  Completed in ", theme.text()),
            Span::styled(format_duration(results.total_time_ms), theme.heading()),
        ]),
        Line::from(""),
    ]
}

fn savings_lines<'a>(results: &ResultsModel, theme: &Theme) -> Vec<Line<'a>> {
    let Some(savings) = &results.savings else {
        return Vec::new();
    };
    let before = format_size(savings.before);
    let after = format_size(savings.after);
    let summary = match savings.change() {
        Some(change) => format!("  {}: {before} → {after} ({change})", change.heading()),
        None => format!("  Size: {before} → {after}"),
    };
    vec![
        Line::from(Span::styled(summary, theme.category())),
        Line::from(""),
    ]
}

/// What the recipe reported, under its output's label (e.g. "Session Log").
fn message_lines<'a>(message: &'a RunMessage, theme: &Theme) -> Vec<Line<'a>> {
    let label = message.label.as_deref().unwrap_or("Output");
    let mut lines = vec![Line::from(Span::styled(
        format!("  {}", space_out(label)),
        theme.category(),
    ))];
    lines.extend(
        message
            .text
            .lines()
            .map(|line| Line::from(Span::styled(format!("  {line}"), theme.text()))),
    );
    lines
}

fn output_dir_lines<'a>(results: &ResultsModel, theme: &Theme) -> Vec<Line<'a>> {
    let Some(dir) = &results.output_dir else {
        return Vec::new();
    };
    vec![
        Line::from(Span::styled(format!("  Output: {dir}"), theme.muted())),
        Line::from(""),
    ]
}

fn file_lines<'a>(results: &'a ResultsModel, theme: &Theme) -> Vec<Line<'a>> {
    if results.outputs.is_empty() {
        return vec![Line::from(Span::styled(
            "  No output files.",
            theme.muted(),
        ))];
    }
    let mut lines = vec![Line::from(Span::styled("  FILES", theme.category()))];
    for (i, file) in results.outputs.iter().enumerate() {
        lines.push(file_line(file, i == results.cursor, theme));
    }
    lines
}

/// One output file: name, then how its size changed when the original's known.
fn file_line<'a>(file: &'a OutputFile, is_selected: bool, theme: &Theme) -> Line<'a> {
    let (marker, name_style) = if is_selected {
        ("▸ ", theme.selected())
    } else {
        ("  ", theme.text())
    };
    let size_info = match file.original_size {
        Some(orig) => {
            let (orig_str, new_str) = (format_size(orig), format_size(file.size_bytes));
            match SizeChange::between(orig, file.size_bytes) {
                Some(change) => format!("  {orig_str} → {new_str}  ({change})"),
                None => format!("  {orig_str} → {new_str}"),
            }
        }
        None => format!("  {}", format_size(file.size_bytes)),
    };
    Line::from(vec![
        Span::styled(format!("  {marker}{}", file.name), name_style),
        Span::styled(size_info, theme.muted()),
    ])
}

/// The tail of the run's command output, still readable once it's done.
fn log_lines<'a>(log: &'a [String], theme: &Theme) -> Vec<Line<'a>> {
    if log.is_empty() {
        return Vec::new();
    }
    let tail = &log[log.len().saturating_sub(MAX_LOG_LINES)..];
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled("  OUTPUT", theme.category())),
    ];
    lines.extend(
        tail.iter()
            .map(|line| Line::from(Span::styled(format!("  {line}"), theme.muted()))),
    );
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

    fn rendered_report(message: Option<RunMessage>, log: &[&str]) -> Vec<String> {
        let log = log.iter().map(|l| l.to_string()).collect();
        let results =
            ResultsModel::new("s", vec![], 0, Some("/tmp/out".into())).with_report(message, log);
        let theme = Theme::from_variant(ThemeVariant::LosAngeles);
        results_lines(&results, &theme)
            .iter()
            .map(|line| line.to_string())
            .collect()
    }

    #[test]
    fn a_recipe_message_shows_under_its_label() {
        let message = RunMessage {
            label: Some("Session Log".into()),
            text: "No active session.\nIdle for 3s right now.".into(),
        };
        let lines = rendered_report(Some(message), &[]);
        assert!(
            lines.contains(&"  S E S S I O N   L O G".to_string()),
            "{lines:?}"
        );
        assert!(
            lines.contains(&"  No active session.".to_string()),
            "{lines:?}"
        );
        assert!(
            lines.contains(&"  Idle for 3s right now.".to_string()),
            "{lines:?}"
        );
        // A message run writes no files, so "no files" would only confuse.
        assert!(
            !lines.iter().any(|l| l.contains("No output files")),
            "{lines:?}"
        );
        assert!(!lines.iter().any(|l| l.contains("Output: ")), "{lines:?}");
    }

    #[test]
    fn the_run_log_stays_readable_after_the_run() {
        let lines = rendered_report(None, &["line 1", "line 2"]);
        assert!(lines.contains(&"  OUTPUT".to_string()), "{lines:?}");
        assert!(lines.contains(&"  line 1".to_string()), "{lines:?}");
        assert!(lines.contains(&"  line 2".to_string()), "{lines:?}");
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
