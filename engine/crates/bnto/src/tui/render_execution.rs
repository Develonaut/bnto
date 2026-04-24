// Render the execution screen — live pipeline progress with node and file status.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::execution::{ExecutionStatus, FileStatus, NodeStatus};
use super::screens::results::format_duration;
use super::theme::Theme;

/// Render the execution screen.
pub fn draw_execution(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(exec) = &model.execution else {
        let fallback = Paragraph::new("Waiting to start...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = execution_lines(exec, theme);
    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Build lines for the execution screen content.
fn execution_lines<'a>(
    exec: &'a super::screens::execution::ExecutionModel,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Status + elapsed time header
    let status_label = match &exec.status {
        ExecutionStatus::Idle => "Idle",
        ExecutionStatus::Running => "Running",
        ExecutionStatus::Completed => "Completed",
        ExecutionStatus::Failed => "Failed",
        ExecutionStatus::Cancelled => "Cancelled",
    };
    let elapsed = format_duration(exec.elapsed_ms);
    lines.push(Line::from(vec![
        Span::styled(format!("  {status_label}"), theme.heading()),
        Span::styled(format!("  {elapsed}"), theme.muted()),
    ]));
    lines.push(Line::from(""));

    // Node progress section
    if !exec.nodes.is_empty() {
        lines.push(Line::from(Span::styled("  NODES", theme.category())));
        for node in &exec.nodes {
            let (marker, style) = node_marker(&node.status, theme);
            let label = if node.node_type.is_empty() {
                node.id.as_str()
            } else {
                node.node_type.as_str()
            };
            lines.push(Line::from(Span::styled(
                format!("  {marker} {label}"),
                style,
            )));
        }
        lines.push(Line::from(""));
    }

    // File progress section
    if !exec.files.is_empty() {
        lines.push(Line::from(Span::styled("  FILES", theme.category())));
        for file in &exec.files {
            let (marker, style) = file_marker(&file.status, theme);
            let pct = if file.percent > 0 && file.percent < 100 {
                format!(" {}%", file.percent)
            } else {
                String::new()
            };
            lines.push(Line::from(Span::styled(
                format!("  {marker} {}{pct}", file.name),
                style,
            )));
        }
        lines.push(Line::from(""));
    }

    // Command output section (streaming stderr from child processes)
    if !exec.output_lines.is_empty() {
        lines.push(Line::from(Span::styled("  OUTPUT", theme.category())));
        for line in &exec.output_lines {
            lines.push(Line::from(Span::styled(format!("  {line}"), theme.muted())));
        }
        lines.push(Line::from(""));
    }

    // Error display
    if let Some(error) = &exec.error {
        lines.push(Line::from(Span::styled(
            format!("  Error: {error}"),
            theme.heading(),
        )));
    }

    lines
}

/// Map node status to a marker character and style.
fn node_marker(status: &NodeStatus, theme: &Theme) -> (&'static str, ratatui::style::Style) {
    match status {
        NodeStatus::Pending => ("○", theme.muted()),
        NodeStatus::Active => ("◉", theme.selected()),
        NodeStatus::Completed { .. } => ("●", theme.text()),
        NodeStatus::Failed(_) => ("✗", theme.heading()),
    }
}

/// Map file status to a marker character and style.
fn file_marker(status: &FileStatus, theme: &Theme) -> (&'static str, ratatui::style::Style) {
    match status {
        FileStatus::Waiting => ("○", theme.muted()),
        FileStatus::Processing => ("◉", theme.selected()),
        FileStatus::Done => ("●", theme.text()),
        FileStatus::Failed(_) => ("✗", theme.heading()),
    }
}
