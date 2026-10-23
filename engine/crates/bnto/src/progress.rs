// Progress reporting — prints pipeline events to stderr with colors.
// Also logs command output lines via the session Logger for diagnostics.

use std::cell::RefCell;
use std::sync::Arc;

use bnto_core::logging::{LogEntry, LogLevel, Logger};
use bnto_core::{PipelineEvent, PipelineReporter};
use colored::Colorize;

use crate::live_bar::LiveBar;
use crate::output_names::OutputNames;
use crate::progress_indent::{self, StepDepths};

/// Create a reporter that prints colored progress events to stderr
/// and logs command output through the session logger.
///
/// When `output_dir` is provided, progressive output files from
/// `IterationCompleted` events are copied to disk immediately.
#[expect(clippy::too_many_lines)]
pub fn stderr_reporter(logger: Arc<dyn Logger>, output_dir: Option<String>) -> PipelineReporter {
    // RefCell for interior mutability — PipelineReporter takes Fn, not FnMut.
    // Safe because the reporter is only called from a single thread.
    let live: RefCell<LiveBar> = RefCell::new(LiveBar::default());
    let depths: RefCell<StepDepths> = RefCell::new(StepDepths::default());
    let output_names: RefCell<OutputNames> = RefCell::default();

    PipelineReporter::new(move |event: PipelineEvent| match &event {
        PipelineEvent::PipelineStarted { total_nodes, .. } => {
            eprintln!(
                "  {} {total_nodes} step{}",
                "Pipeline:".dimmed(),
                if *total_nodes == 1 { "" } else { "s" }
            );
        }
        PipelineEvent::IterationStarted { .. } | PipelineEvent::IterationFailed { .. } => {}
        PipelineEvent::NodeSkipped {
            node_id,
            node_type,
            node_index,
            total_nodes,
            parent_node_id,
            reason,
        } => {
            live.borrow_mut().clear();
            let depth = depths
                .borrow_mut()
                .start(node_id, parent_node_id.as_deref());
            eprintln!(
                "{}{} {} {}",
                progress_indent::step_indent(depth),
                progress_indent::step_counter(*node_index, *total_nodes).dimmed(),
                node_type.dimmed(),
                format!("skipped: {reason}").dimmed(),
            );
        }
        PipelineEvent::IterationCompleted { output_files, .. } => {
            if let Some(dir) = &output_dir {
                let names = &mut output_names.borrow_mut();
                crate::progressive_output::write_progressive_files(dir, output_files, names);
            }
        }
        PipelineEvent::NodeStarted {
            node_id,
            node_type,
            node_index,
            total_nodes,
            parent_node_id,
        } => {
            live.borrow_mut().clear();
            let depth = depths
                .borrow_mut()
                .start(node_id, parent_node_id.as_deref());
            eprintln!(
                "{}{} {}",
                progress_indent::step_indent(depth),
                progress_indent::step_counter(*node_index, *total_nodes).dimmed(),
                node_type.cyan(),
            );
        }
        PipelineEvent::FileProgress {
            file_index,
            total_files,
            percent,
            message,
            ..
        } => live
            .borrow_mut()
            .file_progress(*file_index, *total_files, *percent, message),
        PipelineEvent::NodeProgress {
            done,
            total,
            message,
            ..
        } => live.borrow_mut().node_progress(*done, *total, message),
        PipelineEvent::NodeCompleted {
            node_id,
            duration_ms,
            files_processed,
            ..
        } => {
            live.borrow_mut().clear();
            let duration = format_duration_short(*duration_ms);
            eprintln!(
                "{}{} {} file{} in {duration}",
                progress_indent::status_indent(depths.borrow().depth(node_id)),
                "done".green(),
                files_processed,
                if *files_processed == 1 { "" } else { "s" },
            );
        }
        PipelineEvent::NodeFailed { node_id, error, .. } => {
            live.borrow_mut().clear();
            eprintln!(
                "{}{} {node_id}: {error}",
                progress_indent::step_indent(depths.borrow().depth(node_id)),
                "FAILED".red().bold()
            );
        }
        // No line here: the run's verdict prints `Pipeline failed:` once, last.
        PipelineEvent::PipelineFailed { .. } => {
            live.borrow_mut().clear();
        }
        PipelineEvent::PipelineCompleted {
            duration_ms,
            total_files_processed,
        } => {
            live.borrow_mut().clear();
            eprintln!(
                "\n  {} {}",
                "✓".green().bold(),
                format_completion_summary(*total_files_processed, *duration_ms),
            );
        }
        PipelineEvent::CommandOutput { node_id, line } => {
            // Suspend the live line to print cleanly, then log for diagnostics.
            live.borrow()
                .suspend(|| eprintln!("{}", command_output_line(line).dimmed()));
            logger.log(LogEntry {
                level: LogLevel::Debug,
                target: "engine",
                message: format!("[{node_id}] {line}"),
                elapsed_us: None,
            });
        }
    })
}

/// One line a step's command printed, lined up with the live line's text.
/// It stays on screen after the step ends, so it carries no spinner frame:
/// the live bar below it is what spins.
fn command_output_line(line: &str) -> String {
    format!("          {line}")
}

/// Format a pipeline completion summary line.
///
/// Example: `"Completed 10 files in 2.4s"` or `"Completed 1 file in 350ms"`
pub fn format_completion_summary(total_files: usize, duration_ms: u64) -> String {
    let duration = format_duration_short(duration_ms);
    let plural = if total_files == 1 { "file" } else { "files" };
    format!("Completed {total_files} {plural} in {duration}")
}

/// Format milliseconds into a compact human-readable string.
fn format_duration_short(ms: u64) -> String {
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_millis() {
        assert_eq!(format_duration_short(0), "0ms");
        assert_eq!(format_duration_short(42), "42ms");
        assert_eq!(format_duration_short(999), "999ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration_short(1000), "1.0s");
        assert_eq!(format_duration_short(2500), "2.5s");
    }

    // --- Completion summary tests ---

    #[test]
    fn test_completion_summary_singular() {
        assert_eq!(
            format_completion_summary(1, 350),
            "Completed 1 file in 350ms"
        );
    }

    #[test]
    fn test_completion_summary_plural() {
        assert_eq!(
            format_completion_summary(10, 2400),
            "Completed 10 files in 2.4s"
        );
    }

    #[test]
    fn test_completion_summary_zero_files() {
        assert_eq!(
            format_completion_summary(0, 100),
            "Completed 0 files in 100ms"
        );
    }

    // --- Command output lines ---

    #[test]
    fn test_command_output_line_has_no_spinner_glyph() {
        // The line stays on screen after the step ends, so a spinner frame
        // on it would freeze mid-spin.
        let line = command_output_line("No active session.");
        assert_eq!(line, "          No active session.");
    }

    #[test]
    fn test_progressive_output_creates_subdirectories() {
        use bnto_core::events::ProgressOutputFile;

        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let reporter = stderr_reporter(noop_logger(), Some(dir.to_str().unwrap().to_string()));

        reporter.emit(PipelineEvent::IterationCompleted {
            node_id: "loop-1".to_string(),
            iteration: 0,
            total_iterations: 1,
            duration_ms: 100,
            files_produced: 2,
            output_files: vec![
                ProgressOutputFile {
                    name: "Alpha Legion/part1.mp4".to_string(),
                    data: bnto_core::processor::FileData::Bytes(b"video1".to_vec()),
                    mime_type: "video/mp4".to_string(),
                },
                ProgressOutputFile {
                    name: "top.txt".to_string(),
                    data: bnto_core::processor::FileData::Bytes(b"flat".to_vec()),
                    mime_type: "text/plain".to_string(),
                },
            ],
        });

        assert!(dir.join("Alpha Legion").join("part1.mp4").exists());
        assert_eq!(
            std::fs::read(dir.join("Alpha Legion/part1.mp4")).unwrap(),
            b"video1"
        );
        assert!(dir.join("top.txt").exists());
    }

    #[test]
    fn test_progressive_output_copies_path_files_and_keeps_the_source() {
        use bnto_core::events::ProgressOutputFile;

        let src_dir = tempfile::tempdir().unwrap();
        let src = src_dir.path().join("mine.txt");
        std::fs::write(&src, b"user file").unwrap();
        let out = tempfile::tempdir().unwrap();

        let reporter = stderr_reporter(noop_logger(), Some(out.path().to_str().unwrap().into()));
        reporter.emit(PipelineEvent::IterationCompleted {
            node_id: "loop-1".to_string(),
            iteration: 0,
            total_iterations: 1,
            duration_ms: 1,
            files_produced: 1,
            output_files: vec![ProgressOutputFile {
                name: "mine.txt".to_string(),
                data: bnto_core::processor::FileData::Path(src.clone()),
                mime_type: "text/plain".to_string(),
            }],
        });

        assert_eq!(std::fs::read(&src).unwrap(), b"user file");
        assert_eq!(
            std::fs::read(out.path().join("mine.txt")).unwrap(),
            b"user file"
        );
    }

    fn noop_logger() -> Arc<dyn Logger> {
        Arc::new(bnto_core::logging::NoopLogger)
    }

    #[test]
    fn test_stderr_reporter_handles_full_lifecycle() {
        // Verify the reporter doesn't panic on a full event sequence.
        let reporter = stderr_reporter(noop_logger(), None);
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 2,
            nodes: vec![],
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "image-compress".to_string(),
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::FileProgress {
            node_id: "n1".to_string(),
            file_index: 0,
            total_files: 2,
            percent: 50,
            message: "Processing file1.jpg".to_string(),
        });
        reporter.emit(PipelineEvent::FileProgress {
            node_id: "n1".to_string(),
            file_index: 1,
            total_files: 2,
            percent: 100,
            message: "Processing file2.jpg".to_string(),
        });
        reporter.emit(PipelineEvent::NodeCompleted {
            node_id: "n1".to_string(),
            duration_ms: 250,
            files_processed: 2,
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 300,
            total_files_processed: 2,
        });
    }

    #[test]
    fn test_stderr_reporter_handles_command_output() {
        let reporter = stderr_reporter(noop_logger(), None);
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
            nodes: vec![],
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "shell-run".to_string(),
            parent_node_id: None,
        });
        // CommandOutput events should not panic.
        reporter.emit(PipelineEvent::CommandOutput {
            node_id: "n1".to_string(),
            line: "[download]  34.2% of ~150MiB".to_string(),
        });
        reporter.emit(PipelineEvent::NodeCompleted {
            node_id: "n1".to_string(),
            duration_ms: 5000,
            files_processed: 1,
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 5100,
            total_files_processed: 1,
        });
    }

    #[test]
    fn test_stderr_reporter_handles_live_node_progress() {
        let reporter = stderr_reporter(noop_logger(), None);
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "walk".to_string(),
            node_index: 0,
            total_nodes: 2,
            node_type: "file-collect".to_string(),
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::FileProgress {
            node_id: "walk".to_string(),
            file_index: 0,
            total_files: 1,
            percent: 0,
            message: "Processing big-folder...".to_string(),
        });
        for (done, total) in [(10, None), (5000, None), (40, Some(100))] {
            reporter.emit(PipelineEvent::NodeProgress {
                node_id: "walk".to_string(),
                done,
                total,
                message: format!("Found {done} files"),
            });
        }
        reporter.emit(PipelineEvent::NodeCompleted {
            node_id: "walk".to_string(),
            duration_ms: 9000,
            files_processed: 1,
            parent_node_id: None,
        });
    }

    #[test]
    fn test_stderr_reporter_handles_failure() {
        let reporter = stderr_reporter(noop_logger(), None);
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
            nodes: vec![],
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "image-compress".to_string(),
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::NodeFailed {
            node_id: "n1".to_string(),
            error: "Unsupported format".to_string(),
            parent_node_id: None,
        });
        reporter.emit(PipelineEvent::PipelineFailed {
            node_id: "n1".to_string(),
            error: "Node n1 failed".to_string(),
        });
    }
}
