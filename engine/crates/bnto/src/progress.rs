// Progress reporting — prints pipeline events to stderr with colors.

use std::cell::RefCell;

use bnto_core::{PipelineEvent, PipelineReporter};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Create a reporter that prints colored progress events to stderr.
pub fn stderr_reporter() -> PipelineReporter {
    // RefCell for interior mutability — PipelineReporter takes Fn, not FnMut.
    // Safe because the reporter is only called from a single thread.
    let bar: RefCell<Option<ProgressBar>> = RefCell::new(None);

    PipelineReporter::new(move |event: PipelineEvent| match &event {
        PipelineEvent::PipelineStarted { total_nodes, .. } => {
            eprintln!(
                "  {} {total_nodes} step{}",
                "Pipeline:".dimmed(),
                if *total_nodes == 1 { "" } else { "s" }
            );
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
        PipelineEvent::PipelineCompleted { .. } => {
            if let Some(b) = bar.borrow_mut().take() {
                b.finish_and_clear();
            }
        }
    })
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

    #[test]
    fn test_stderr_reporter_handles_full_lifecycle() {
        // Verify the reporter doesn't panic on a full event sequence.
        let reporter = stderr_reporter();
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 2,
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "image-compress".to_string(),
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
        });
        reporter.emit(PipelineEvent::PipelineCompleted {
            duration_ms: 300,
            total_files_processed: 2,
        });
    }

    #[test]
    fn test_stderr_reporter_handles_failure() {
        let reporter = stderr_reporter();
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 1,
        });
        reporter.emit(PipelineEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_index: 0,
            total_nodes: 1,
            node_type: "image-compress".to_string(),
        });
        reporter.emit(PipelineEvent::NodeFailed {
            node_id: "n1".to_string(),
            error: "Unsupported format".to_string(),
        });
        reporter.emit(PipelineEvent::PipelineFailed {
            node_id: "n1".to_string(),
            error: "Node n1 failed".to_string(),
        });
    }
}
