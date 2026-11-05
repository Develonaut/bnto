// Render the execution screen — live pipeline progress with node and file status.
//
// Layout: fixed-height header (status, nodes, files, errors) on top,
// command output fills remaining space at the bottom.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::file_window::file_window;
use super::render_consent::draw_consent;
use super::render_layout::content_panel;
use super::screens::execution::{
    ExecutionModel, ExecutionStatus, FileStatus, NodeProgress, NodeStatus,
};
use super::screens::results::format_duration;
use super::theme::Theme;
use super::wrap_lines::wrap_lines;
use crate::display_text::display_text;

/// Render the execution screen.
pub fn draw_execution(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(exec) = &model.execution else {
        let fallback = Paragraph::new("Waiting to start...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    // Consent gets the whole panel, wrapped and scrolled, so a long command
    // line is read in full before it's approved.
    if let Some(request) = &exec.consent {
        draw_consent(frame, request, exec.consent_scroll, theme, inner);
        return;
    }

    let width = usize::from(inner.width);
    let header = header_lines(exec, theme, file_rows(exec, theme, inner), width);
    let header_height = header.len() as u16;

    let [top, bottom] =
        Layout::vertical([Constraint::Length(header_height), Constraint::Min(1)]).areas(inner);

    frame.render_widget(Paragraph::new(header), top);

    // Fill the bottom area with the most recent command output lines.
    let output = output_lines(exec, theme, bottom.height as usize);
    frame.render_widget(Paragraph::new(output), bottom);
}

/// Rows the OUTPUT pane keeps (heading and lines) whenever there's output,
/// so a long FILES list can't push it off a short terminal.
const MIN_OUTPUT_ROWS: usize = 4;

/// How many FILES rows fit in the `panel`, after the rest of the header and
/// the OUTPUT pane's rows.
fn file_rows(exec: &ExecutionModel, theme: &Theme, panel: Rect) -> usize {
    let height = panel.height;
    let full = header_lines(exec, theme, usize::MAX, usize::from(panel.width)).len();
    let file_lines = if show_files(exec) {
        exec.files.len()
    } else {
        0
    };
    let output = if exec.output_lines.is_empty() {
        0
    } else {
        MIN_OUTPUT_ROWS
    };
    usize::from(height).saturating_sub(full - file_lines + output)
}

/// Build header lines: status, steps, files (at most `file_rows`), errors
/// (wrapped to `width`).
fn header_lines<'a>(
    exec: &'a ExecutionModel,
    theme: &'a Theme,
    file_rows: usize,
    width: usize,
) -> Vec<Line<'a>> {
    let mut lines = status_lines(exec, theme);
    lines.extend(steps_lines(exec, theme));
    if show_files(exec) {
        lines.extend(files_lines(exec, theme, file_rows));
    }
    if let Some(error) = &exec.error {
        lines.extend(error_lines(error, theme, width));
    }
    lines
}

/// The error, one row per line it holds, each wrapped to `width`: a failed
/// write's later lines name where kept copies are, and must show in full.
/// Every row after the first, a wrapped row of the first line's included,
/// starts under the text after `Error: `, past the column the screen's own
/// rows start at, so recipe text a builder failed to escape can't pass for
/// one of them.
fn error_lines(error: &str, theme: &Theme, width: usize) -> Vec<Line<'static>> {
    let gutter = " ".repeat(ERROR_LABEL.len());
    let row = |part: &str| {
        let text = format!("{gutter}{}", display_text(part.trim_start()));
        Line::from(Span::styled(text, theme.heading()))
    };
    let mut rows = wrap_lines(error.lines().map(row).collect(), width);
    if let Some(first) = rows.first_mut() {
        let text = first.to_string();
        let rest = text.strip_prefix(&gutter).unwrap_or(text.trim_start());
        *first = Line::from(Span::styled(
            format!("{ERROR_LABEL}{rest}"),
            theme.heading(),
        ));
    }
    rows
}

/// What an error's first row starts with; its later rows indent to its end.
const ERROR_LABEL: &str = "  Error: ";

fn status_lines<'a>(exec: &ExecutionModel, theme: &Theme) -> Vec<Line<'a>> {
    let status_label = match &exec.status {
        ExecutionStatus::AwaitingConsent => "Waiting for your OK",
        ExecutionStatus::Idle => "Idle",
        ExecutionStatus::Running => "Running",
        ExecutionStatus::Completed => "Completed",
        ExecutionStatus::Failed => "Failed",
        ExecutionStatus::Cancelled => "Cancelled",
    };
    vec![
        Line::from(vec![
            Span::styled(format!("  {status_label}"), theme.heading()),
            Span::styled(
                format!("  {}", format_duration(exec.elapsed_ms)),
                theme.muted(),
            ),
        ]),
        Line::from(""),
    ]
}

/// STEPS: each node, with a container's steps indented under it.
fn steps_lines<'a>(exec: &'a ExecutionModel, theme: &Theme) -> Vec<Line<'a>> {
    if exec.nodes.is_empty() {
        return Vec::new();
    }
    let mut lines = vec![Line::from(Span::styled("  STEPS", theme.category()))];
    for node in &exec.nodes {
        lines.push(step_line(node, "  ", theme));
        lines.extend(
            node.children
                .iter()
                .map(|child| step_line(child, "    ", theme)),
        );
    }
    lines.push(Line::from(""));
    lines
}

fn step_line<'a>(node: &NodeProgress, indent: &str, theme: &Theme) -> Line<'a> {
    let (marker, style) = node_marker(&node.status, theme);
    let text = format!(
        "{indent}{marker} {}{}",
        node_label(node),
        node_annotation(node)
    );
    Line::from(Span::styled(text, style))
}

/// FILES: a window around the file being worked on, the rest counted. With
/// room for fewer than two rows, one summary line instead: "FILES 2/3 · ◉ name".
#[expect(clippy::too_many_lines)]
fn files_lines<'a>(exec: &'a ExecutionModel, theme: &Theme, rows: usize) -> Vec<Line<'a>> {
    let active = exec
        .files
        .iter()
        .position(|f| f.status == FileStatus::Processing)
        .or_else(|| {
            exec.files
                .iter()
                .position(|f| f.status == FileStatus::Waiting)
        });
    if rows < 2 && exec.files.len() > rows {
        return vec![files_summary_line(exec, active, theme)];
    }
    let window = file_window(exec.files.len(), active, rows);
    let mut lines = vec![Line::from(Span::styled("  FILES", theme.category()))];
    for file in &exec.files[window.shown] {
        let (marker, style) = file_marker(&file.status, theme);
        lines.push(Line::from(Span::styled(
            format!("  {marker} {}", file_text(file)),
            style,
        )));
    }
    if window.hidden > 0 {
        let more = format!("  … {} more", window.hidden);
        lines.push(Line::from(Span::styled(more, theme.muted())));
    }
    lines.push(Line::from(""));
    lines
}

/// FILES in one line: which file of how many, and its status.
fn files_summary_line<'a>(exec: &ExecutionModel, active: Option<usize>, theme: &Theme) -> Line<'a> {
    let index = active.unwrap_or(exec.files.len() - 1);
    let file = &exec.files[index];
    let (marker, style) = file_marker(&file.status, theme);
    Line::from(vec![
        Span::styled(
            format!("  FILES {}/{} · ", index + 1, exec.files.len()),
            theme.category(),
        ),
        Span::styled(format!("{marker} {}", file_text(file)), style),
    ])
}

/// A file row's text: its name or progress message, and the percent mid-way.
fn file_text(file: &super::screens::execution::FileProgress) -> String {
    if file.percent > 0 && file.percent < 100 {
        format!("{} {}%", file.name, file.percent)
    } else {
        file.name.clone()
    }
}

/// Whether FILES shows; see [`should_hide_files`].
fn show_files(exec: &ExecutionModel) -> bool {
    !exec.files.is_empty() && !should_hide_files(exec)
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
        NodeStatus::Completed { .. } => ("✓", theme.text()),
        NodeStatus::Failed(_) => ("✗", theme.heading()),
        NodeStatus::Skipped => ("–", theme.muted()),
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
        NodeStatus::Skipped => " (skipped)".to_string(),
        _ => String::new(),
    }
}

/// Hide the FILES section when it's misleading: all input files are done
/// but the pipeline is still running (containers generating dynamic work).
fn should_hide_files(exec: &ExecutionModel) -> bool {
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
        FileStatus::Done => ("✓", theme.text()),
        FileStatus::Failed(_) => ("✗", theme.heading()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::screens::execution::{NodeProgress, NodeStatus};

    /// The execution screen's text at `width` x `height`, rows joined.
    fn screen_text(model: &AppModel, width: u16, height: u16) -> String {
        use ratatui::{Terminal, backend::TestBackend};
        let theme = Theme::from_variant(crate::tui::theme::ThemeVariant::LosAngeles);
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal
            .draw(|f| draw_execution(f, model, &theme, f.area()))
            .unwrap();
        let buffer = terminal.backend().buffer();
        let row = |y| {
            (0..width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
        };
        (0..height).map(row).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn a_failed_writes_kept_copies_folder_is_shown_in_full() {
        // RED (ISS-48 review): the write error's second line names the folder
        // holding the only copy of a file; as one unwrapped line it was cut off.
        use crate::tui::screens::execution::{self, ExecutionMessage};
        let folder = "/Users/someone/Pictures/2026/holiday/.bnto-staged-Xy12Ab";
        let error = format!(
            "Cannot write /Users/someone/Pictures/2026/holiday/a.txt: Permission denied \
             (os error 13)\n  Copies set aside for a swap are kept in {folder}"
        );
        let failed = ExecutionMessage::PipelineFailed {
            node_id: String::new(),
            error,
        };
        let home = tempfile::tempdir().unwrap();
        let paths = crate::storage::BntoPaths {
            home: home.path().to_path_buf(),
        };
        let variant = crate::tui::theme::ThemeVariant::LosAngeles;
        let mut model = AppModel::with_paths(variant, None, false, paths);
        model.execution = Some(execution::update(ExecutionModel::new("s"), failed));

        let screen = screen_text(&model, 80, 24);
        assert!(screen.contains(folder), "cut:\n{screen}");
        assert!(
            screen.contains("Copies set aside for a swap are kept in"),
            "{screen}"
        );
    }

    /// What a recipe would like to show as a row of its own.
    const FORGED: &str = "FORGED 1 file written";

    /// The execution screen at 100x30 after a run failed with `error`.
    fn failed_screen(error: String) -> String {
        use crate::tui::screens::execution::{self, ExecutionMessage};
        let failed = ExecutionMessage::PipelineFailed {
            node_id: String::new(),
            error,
        };
        let home = tempfile::tempdir().unwrap();
        let paths = crate::storage::BntoPaths {
            home: home.path().to_path_buf(),
        };
        let variant = crate::tui::theme::ThemeVariant::LosAngeles;
        let mut model = AppModel::with_paths(variant, None, false, paths);
        model.execution = Some(execution::update(ExecutionModel::new("s"), failed));
        screen_text(&model, 100, 30)
    }

    /// No row of `screen` starts with the recipe's text at or before the
    /// column the screen's own rows use, past the panel's border.
    fn assert_no_forged_row(screen: &str) {
        for row in screen.lines() {
            let inner = row.split_once('│').map_or(row, |(_, rest)| rest);
            let indent = inner.chars().take_while(|c| *c == ' ').count();
            let forged = inner.trim_start().starts_with("FORGED");
            assert!(
                !forged || indent >= ERROR_LABEL.len(),
                "a forged row:\n{screen}"
            );
        }
    }

    #[test]
    fn a_line_break_in_a_copy_destination_forges_no_row() {
        let value = format!("C:x\n{FORGED}");
        let err = bnto_core::in_work_dir("destination", &value, &bnto_core::NoopContext);
        let screen = failed_screen(err.unwrap_err().to_string());
        assert!(screen.contains(&format!("\\n{FORGED}")), "{screen}");
        assert_no_forged_row(&screen);
    }

    #[test]
    fn a_line_break_in_a_rename_prefix_forges_no_row() {
        // The name a `file-rename` prefix of `../\nFORGED…` builds, refused
        // by `file-copy` as the engine refuses it.
        use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeProcessor};
        let out = tempfile::tempdir().unwrap();
        let file = BatchFile {
            data: FileData::Bytes(b"a".to_vec()),
            filename: format!("../\n{FORGED}a.txt"),
            mime_type: None,
            metadata: serde_json::Map::new(),
        };
        let params = serde_json::json!({ "destination": out.path().to_str().unwrap() });
        let input = BatchInput {
            files: vec![file],
            params: params.as_object().unwrap().clone(),
        };
        let progress = bnto_core::ProgressReporter::new_noop();
        let copy = bnto_file::FileCopy::new();
        let err = copy.process_batch(input, &progress, &bnto_core::NoopContext);
        let screen = failed_screen(err.err().unwrap().to_string());
        assert!(screen.contains(&format!("\\n{FORGED}")), "{screen}");
        assert_no_forged_row(&screen);
    }

    #[test]
    fn a_raw_line_break_in_an_error_starts_its_row_past_the_label() {
        // A builder that missed escaping: its line still can't pass for one
        // of the screen's own rows.
        let screen = failed_screen(format!("boom\n{FORGED}"));
        assert!(screen.contains("  Error: boom"), "{screen}");
        assert!(screen.contains(&format!("         {FORGED}")), "{screen}");
        assert_no_forged_row(&screen);
    }

    #[test]
    fn a_long_first_line_wraps_under_the_label() {
        // Enough words that some wrapped row starts with one.
        let screen = failed_screen("FORGED ".repeat(40));
        assert_no_forged_row(&screen);
    }

    #[test]
    fn a_skipped_node_is_marked_and_says_so() {
        let node = make_node(NodeStatus::Skipped, 0, 0);
        assert_eq!(
            node_marker(
                &node.status,
                &Theme::from_variant(crate::tui::theme::ThemeVariant::LosAngeles)
            )
            .0,
            "–"
        );
        assert_eq!(node_annotation(&node), " (skipped)");
    }

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
    fn done_reads_as_a_check_not_a_dot() {
        // ◉ (running) and ● (done) were easy to confuse; the CLI uses ✓.
        let theme = Theme::from_variant(crate::tui::theme::ThemeVariant::LosAngeles);
        let done = NodeStatus::Completed { duration_ms: 1 };
        assert_eq!(node_marker(&done, &theme).0, "✓");
        assert_eq!(file_marker(&FileStatus::Done, &theme).0, "✓");
        assert_ne!(node_marker(&NodeStatus::Active, &theme).0, "✓");
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

    #[test]
    fn consent_prompt_wraps_long_commands_so_they_read_in_full() {
        use crate::storage::paths::BntoPaths;
        use crate::trust_gate::ConsentRequest;
        use crate::tui::app::{AppModel, DetailOrigin, Screen};
        use crate::tui::screens::execution::ExecutionModel;
        use crate::tui::theme::ThemeVariant;

        let dir = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: dir.path().to_path_buf(),
        };
        let tail = "--sub-langs {{fields.subtitles}} -o {{output_dir}}/%(title)s.%(ext)s";
        let command = format!("yt-dlp --no-playlist {tail}");
        let request = ConsentRequest::for_commands("Probe", vec![command]);
        let model = AppModel {
            screen: Screen::Execution {
                slug: "probe".into(),
                from: DetailOrigin::Library,
            },
            execution: Some(ExecutionModel::new("probe").awaiting_consent(request)),
            ..AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths)
        };
        let backend = ratatui::backend::TestBackend::new(40, 30);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal
            .draw(|f| draw_execution(f, &model, &model.theme, f.area()))
            .unwrap();
        let text: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>()
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '│' && *c != '─')
            .collect();
        let wanted: String = tail.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(text.contains(&wanted), "the end of the command was cut off");
        assert!(text.contains("yallowandrun"), "the answer keys must show");
    }
}
