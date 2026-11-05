// `bnto run` verdicts — a run's last human line on stderr and, with `--json`,
// the same `RunSummary` as one JSON line on stdout.

use std::fmt::Display;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::display_text::display_lines;
use bnto_core::{NodeTally, PipelineReporter, PipelineResult, RunSummary};
use colored::Colorize;

/// Reports one run's outcome and ends a failed run. Clones share the run's
/// tally, so the run watchdog's thread can report it too.
#[derive(Clone)]
pub struct RunVerdict {
    json: bool,
    start: Instant,
    tally: Arc<Mutex<NodeTally>>,
}

impl RunVerdict {
    /// A verdict for a run starting now; `json` adds the stdout line.
    pub fn new(json: bool) -> Self {
        Self {
            json,
            start: Instant::now(),
            tally: Arc::default(),
        }
    }

    /// `inner`, with every event also folded into this run's node tally.
    pub fn tallying(&self, inner: PipelineReporter) -> PipelineReporter {
        let tally = Arc::clone(&self.tally);
        PipelineReporter::new(move |event| {
            tally.lock().unwrap().record(&event);
            inner.emit(event);
        })
    }

    /// The passing summary of `result`, with the nodes tallied so far.
    pub fn passed(&self, result: &PipelineResult) -> RunSummary {
        RunSummary::from_result(result, result.duration_ms)
            .with_nodes(self.tally.lock().unwrap().nodes())
            .with_node_metadata(&result.node_metadata)
    }

    /// Print `{label} {error}` as the last stderr line (and, with `--json`,
    /// the failed summary on stdout), delete the run's temp files (the exit
    /// skips destructors), then exit 1.
    pub fn fail(&self, label: &str, error: impl Display) -> ! {
        let elapsed_ms = self.start.elapsed().as_millis() as u64;
        let nodes = self.tally.lock().unwrap().nodes();
        let summary = RunSummary::from_error(error, elapsed_ms).with_nodes(nodes);
        let message = summary.error.as_deref().unwrap_or_default();
        eprintln!("{} {}", label.red(), shown_error(message));
        self.print_json(&summary);
        crate::scratch::remove_live_dirs();
        process::exit(1);
    }

    /// The `Ok` value, or `fail` with `label` and the error.
    pub fn or_fail<T, E: Display>(&self, result: Result<T, E>, label: &str) -> T {
        result.unwrap_or_else(|e| self.fail(label, e))
    }

    /// With `--json`, print `summary` as one line on stdout.
    pub fn print_json(&self, summary: &RunSummary) {
        if self.json {
            let line = serde_json::to_string(summary).expect("RunSummary serializes");
            println!("{line}");
        }
    }
}

/// `message` escaped (`display_lines`, which keeps its line breaks), every
/// line after the first starting at least two columns in. A dry run's
/// preview failure is shown this way too. bnto's own lines
/// start at column 0 ("✔ 1 file written", a hint to run a command), so recipe
/// text a builder failed to escape can't pass for one; a line bnto already
/// indents keeps its layout.
pub fn shown_error(message: &str) -> String {
    let shown = display_lines(message);
    let mut lines = shown.split('\n');
    let first = lines.next().unwrap_or_default().to_string();
    lines.fold(first, |mut out, line| {
        out.push('\n');
        if !line.starts_with(CONTINUATION) {
            out.push_str(CONTINUATION);
        }
        out.push_str(line);
        out
    })
}

/// The least indent of an error's later lines.
const CONTINUATION: &str = "  ";

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineEvent;
    use bnto_core::events::NodeInfo;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn an_errors_later_lines_never_start_at_column_zero() {
        let shown = shown_error("Refused: \"a\nFORGED 1 file written\"\n\nTo finish, run: x");
        assert_eq!(
            shown,
            "Refused: \"a\n  FORGED 1 file written\"\n  \n  To finish, run: x"
        );
    }

    #[test]
    fn an_already_indented_line_keeps_its_layout() {
        let message = "File copy refused: 2 names\n  \"a\" has no file name\n    deeper";
        assert_eq!(shown_error(message), message);
    }

    #[test]
    fn other_control_characters_are_still_shown() {
        assert_eq!(shown_error("a\u{1b}[2K\rb\nc"), "a\\e[2K\\rb\n  c");
    }

    #[test]
    fn test_tallying_reporter_forwards_events_and_records_nodes() {
        let verdict = RunVerdict::new(false);
        let seen = Rc::new(RefCell::new(0));
        let counter = Rc::clone(&seen);
        let inner = PipelineReporter::new(move |_| *counter.borrow_mut() += 1);
        let reporter = verdict.tallying(inner);
        reporter.emit(PipelineEvent::PipelineStarted {
            total_nodes: 1,
            total_files: 0,
            nodes: vec![NodeInfo {
                id: "copy".into(),
                node_type: "file-copy".into(),
            }],
        });
        let result = PipelineResult {
            files: vec![],
            duration_ms: 4,
            warnings: vec![],
            node_metadata: Default::default(),
        };
        let summary = verdict.passed(&result);
        assert_eq!(*seen.borrow(), 1);
        assert_eq!(summary.nodes[0].id, "copy");
        assert_eq!(summary.duration_ms, 4);
    }
}
