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

    // Node progress (labeled "STEPS" — user-facing terminology)
    if !exec.nodes.is_empty() {
        lines.push(Line::from(Span::styled("  STEPS", theme.category())));
        for node in &exec.nodes {
            let (marker, style) = node_marker(&node.status, theme);
            let label = node_label(node);
            let annotation = node_annotation(node);
            lines.push(Line::from(Span::styled(
                format!("  {marker} {label}{annotation}"),
                style,
            )));
            // Render child nodes indented under their parent container.
            for child in &node.children {
                let (cm, cs) = node_marker(&child.status, theme);
                let child_label = node_label(child);
                let child_ann = node_annotation(child);
                lines.push(Line::from(Span::styled(
                    format!("    {cm} {child_label}{child_ann}"),
                    cs,
                )));
            }
        }
        lines.push(Line::from(""));
    }

    // File progress — hidden when all files are done but pipeline is still
    // running, since it just shows "Complete" for input files while containers
    // are doing the real work (e.g., loop iterating over dynamic items).
    let show_files = !exec.files.is_empty() && !should_hide_files(exec);
    if show_files {
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

/// Display label for a node — prefers node_type, falls back to id.
fn node_label(node: &super::screens::execution::NodeProgress) -> &str {
    if node.node_type.is_empty() {
        node.id.as_str()
    } else {
        node.node_type.as_str()
    }
}

/// Build a status annotation for a node.
///
/// - Active loops: iteration count `(3/15)`
/// - Active non-loops with files: file count `(2/4)`
/// - Completed: elapsed time `(2.4s)`
fn node_annotation(node: &super::screens::execution::NodeProgress) -> String {
    match &node.status {
        NodeStatus::Active if node.total_iterations.is_some() => {
            let iter = node.iteration.unwrap_or(0) + 1; // 1-based for display
            let total = node.total_iterations.unwrap();
            format!(" ({iter}/{total})")
        }
        NodeStatus::Active if node.total_files > 0 => {
            format!(" ({}/{})", node.files_processed, node.total_files)
        }
        NodeStatus::Completed { duration_ms } => {
            format!(" ({})", format_duration(*duration_ms))
        }
        _ => String::new(),
    }
}

/// Hide the FILES section when it's misleading: all input files are done
/// but the pipeline is still running (containers generating dynamic work).
fn should_hide_files(exec: &super::screens::execution::ExecutionModel) -> bool {
    if exec.status != ExecutionStatus::Running {
        return false;
    }
    let has_containers = exec.nodes.iter().any(|n| !n.children.is_empty());
    let all_done = exec.files.iter().all(|f| f.status == FileStatus::Done);
    has_containers && all_done
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::screens::execution::{NodeProgress, NodeStatus};

    fn make_node(status: NodeStatus, files_processed: usize, total_files: usize) -> NodeProgress {
        NodeProgress {
            id: "n1".into(),
            node_type: "image-compress".into(),
            status,
            files_processed,
            total_files,
            iteration: None,
            total_iterations: None,
            children: Vec::new(),
        }
    }

    #[test]
    fn annotation_active_with_files_shows_count() {
        let node = make_node(NodeStatus::Active, 3, 10);
        assert_eq!(node_annotation(&node), " (3/10)");
    }

    #[test]
    fn annotation_active_no_files_is_empty() {
        let node = make_node(NodeStatus::Active, 0, 0);
        assert_eq!(node_annotation(&node), "");
    }

    #[test]
    fn annotation_completed_shows_elapsed() {
        let node = make_node(NodeStatus::Completed { duration_ms: 2400 }, 0, 0);
        assert_eq!(node_annotation(&node), " (2.4s)");
    }

    #[test]
    fn annotation_completed_sub_second() {
        let node = make_node(NodeStatus::Completed { duration_ms: 350 }, 0, 0);
        assert_eq!(node_annotation(&node), " (350ms)");
    }

    #[test]
    fn annotation_pending_is_empty() {
        let node = make_node(NodeStatus::Pending, 0, 0);
        assert_eq!(node_annotation(&node), "");
    }

    #[test]
    fn annotation_shows_iteration_for_loops() {
        let mut node = make_node(NodeStatus::Active, 0, 0);
        node.node_type = "loop".into();
        node.iteration = Some(2); // 0-based
        node.total_iterations = Some(15);
        // 1-based display: iteration 2 -> "3/15"
        assert_eq!(node_annotation(&node), " (3/15)");
    }

    #[test]
    fn annotation_iteration_takes_priority_over_file_count() {
        let mut node = make_node(NodeStatus::Active, 5, 10);
        node.iteration = Some(0);
        node.total_iterations = Some(3);
        // Iteration count shown instead of file count.
        assert_eq!(node_annotation(&node), " (1/3)");
    }
}
