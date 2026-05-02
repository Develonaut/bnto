// Progress reporting — prints pipeline events to stderr with colors.
// Also logs command output lines via the session Logger for diagnostics.

use std::cell::RefCell;
use std::sync::Arc;

use bnto_core::logging::{LogEntry, LogLevel, Logger};
use bnto_core::{PipelineEvent, PipelineReporter};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Create a reporter that prints colored progress events to stderr
/// and logs command output through the session logger.
///
/// When `output_dir` is provided, progressive output files from
/// `IterationCompleted` events are written to disk immediately.
pub fn stderr_reporter(logger: Arc<dyn Logger>, output_dir: Option<String>) -> PipelineReporter {
    // RefCell for interior mutability — PipelineReporter takes Fn, not FnMut.
    // Safe because the reporter is only called from a single thread.
    let bar: RefCell<Option<ProgressBar>> = RefCell::new(None);
    let spinner_tick: RefCell<usize> = RefCell::new(0);

    PipelineReporter::new(move |event: PipelineEvent| match &event {
        PipelineEvent::PipelineStarted { total_nodes, .. } => {
            eprintln!(
                "  {} {total_nodes} step{}",
                "Pipeline:".dimmed(),
                if *total_nodes == 1 { "" } else { "s" }
            );
        }
        PipelineEvent::IterationStarted { .. } | PipelineEvent::IterationFailed { .. } => {}
        PipelineEvent::IterationCompleted { output_files, .. } => {
            if let Some(dir) = &output_dir {
                for file in output_files {
                    let path = std::path::Path::new(dir).join(&file.name);
                    let _ = std::fs::write(&path, &file.data);
                }
            }
        }
        PipelineEvent::NodeStarted {
            node_type,
            node_index,
            total_nodes,
            ..
        } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
            eprintln!(
                "  {} {}",
                format!("[{}/{}]", node_index + 1, total_nodes).dimmed(),
                node_type.cyan(),
            );
        }
        PipelineEvent::FileProgress {
            file_index,
            total_files,
            message,
            ..
        } => {
            let mut bar_ref = bar.borrow_mut();
            let pb = bar_ref.get_or_insert_with(|| {
                let pb = ProgressBar::new(*total_files as u64);
                pb.set_style(
                    ProgressStyle::with_template("        {bar:30.cyan/dim} {pos}/{len} {msg}")
                        .expect("valid template")
                        .progress_chars("━╸─"),
                );
                pb
            });
            pb.set_position(*file_index as u64 + 1);
            pb.set_message(message.clone());
        }
        PipelineEvent::NodeCompleted {
            duration_ms,
            files_processed,
            ..
        } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
            let duration = format_duration_short(*duration_ms);
            eprintln!(
                "        {} {} file{} in {duration}",
                "done".green(),
                files_processed,
                if *files_processed == 1 { "" } else { "s" },
            );
        }
        PipelineEvent::NodeFailed { node_id, error, .. } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
            eprintln!("  {} {node_id}: {error}", "FAILED".red().bold());
        }
        PipelineEvent::PipelineFailed { error, .. } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
            eprintln!("  {} {error}", "Pipeline failed:".red().bold());
        }
        PipelineEvent::PipelineCompleted {
            duration_ms,
            total_files_processed,
        } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
            eprintln!(
                "\n  {} {}",
                "✓".green().bold(),
                format_completion_summary(*total_files_processed, *duration_ms),
            );
        }
        PipelineEvent::CommandOutput { node_id, line } => {
            let tick = {
                let mut t = spinner_tick.borrow_mut();
                let val = *t;
                *t += 1;
                val
            };
            let spinner = braille_frame(tick);
            // Suspend indicatif bar to print cleanly, then log for diagnostics.
            if let Some(b) = bar.borrow().as_ref() {
                b.suspend(|| eprintln!("        {} {}", spinner.to_string().cyan(), line.dimmed()));
            } else {
                eprintln!("        {} {}", spinner.to_string().cyan(), line.dimmed());
            }
            logger.log(LogEntry {
                level: LogLevel::Debug,
                target: "engine",
                message: format!("[{node_id}] {line}"),
                elapsed_us: None,
            });
        }
    })
}

/// Braille spinner frames for indeterminate progress (shell-command nodes).
const BRAILLE_FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// Get the braille spinner character for a given frame index.
pub fn braille_frame(tick: usize) -> char {
    BRAILLE_FRAMES[tick % BRAILLE_FRAMES.len()]
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

    // --- Braille spinner tests ---

    #[test]
    fn test_braille_frame_cycles() {
        assert_eq!(braille_frame(0), '⠋');
        assert_eq!(braille_frame(1), '⠙');
        assert_eq!(braille_frame(9), '⠏');
    }

    #[test]
    fn test_braille_frame_wraps() {
        assert_eq!(braille_frame(10), '⠋');
        assert_eq!(braille_frame(11), '⠙');
        assert_eq!(braille_frame(20), '⠋');
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
            node_type: "shell-command".to_string(),
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
