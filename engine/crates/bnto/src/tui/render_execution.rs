// Render the execution screen — live pipeline progress with node and file status.
//
// Layout: fixed-height header (status, nodes, files, errors) on top,
// command output fills remaining space at the bottom.

use ratatui::layout::{Constraint, Layout, Rect};
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

    let header = header_lines(exec, theme);
    let header_height = header.len() as u16;

    let [top, bottom] =
        Layout::vertical([Constraint::Length(header_height), Constraint::Min(1)]).areas(inner);

    frame.render_widget(Paragraph::new(header), top);

    // Fill the bottom area with the most recent command output lines.
    let output = output_lines(exec, theme, bottom.height as usize);
    frame.render_widget(Paragraph::new(output), bottom);
}

/// Build header lines: status, nodes, files, errors.
fn header_lines<'a>(
    exec: &'a super::screens::execution::ExecutionModel,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Status + elapsed time
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

    // Node progress
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

    // File progress
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

    // Error display
    if let Some(error) = &exec.error {
        lines.push(Line::from(Span::styled(
            format!("  Error: {error}"),
            theme.heading(),
        )));
    }

    lines
}

/// Build output lines sized to fill available height.
fn output_lines<'a>(
    exec: &'a super::screens::execution::ExecutionModel,
    theme: &'a Theme,
    available_height: usize,
) -> Vec<Line<'a>> {
    if exec.output_lines.is_empty() {
        return Vec::new();
    }

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled("  OUTPUT", theme.category())));

    // Reserve 1 line for the "OUTPUT" header.
    let max_visible = available_height.saturating_sub(1);
    let total = exec.output_lines.len();
    let skip = total.saturating_sub(max_visible);

    for line in exec.output_lines.iter().skip(skip) {
        lines.push(Line::from(Span::styled(format!("  {line}"), theme.muted())));
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
