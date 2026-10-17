// `bnto run` verdicts — a run's last human line on stderr and, with `--json`,
// the same `RunSummary` as one JSON line on stdout.

use std::cell::RefCell;
use std::fmt::Display;
use std::process;
use std::rc::Rc;
use std::time::Instant;

use bnto_core::{NodeTally, PipelineReporter, PipelineResult, RunSummary};
use colored::Colorize;

/// Reports one run's outcome and ends a failed run.
pub struct RunVerdict {
    json: bool,
    start: Instant,
    tally: Rc<RefCell<NodeTally>>,
}

impl RunVerdict {
    /// A verdict for a run starting now; `json` adds the stdout line.
    pub fn new(json: bool) -> Self {
        Self {
            json,
            start: Instant::now(),
            tally: Rc::default(),
        }
    }

    /// `inner`, with every event also folded into this run's node tally.
    pub fn tallying(&self, inner: PipelineReporter) -> PipelineReporter {
        let tally = Rc::clone(&self.tally);
        PipelineReporter::new(move |event| {
            tally.borrow_mut().record(&event);
            inner.emit(event);
        })
    }

    /// The passing summary of `result`, with the nodes tallied so far.
    pub fn passed(&self, result: &PipelineResult) -> RunSummary {
        RunSummary::from_result(result, result.duration_ms).with_nodes(self.tally.borrow().nodes())
    }

    /// Print `{label} {error}` as the last stderr line (and, with `--json`,
    /// the failed summary on stdout), then exit 1.
    pub fn fail(&self, label: &str, error: impl Display) -> ! {
        let elapsed_ms = self.start.elapsed().as_millis() as u64;
        let nodes = self.tally.borrow().nodes();
        let summary = RunSummary::from_error(error, elapsed_ms).with_nodes(nodes);
        let message = summary.error.as_deref().unwrap_or_default();
        eprintln!("{} {message}", label.red());
        self.print_json(&summary);
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

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineEvent;
    use bnto_core::events::NodeInfo;

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
        };
        let summary = verdict.passed(&result);
        assert_eq!(*seen.borrow(), 1);
        assert_eq!(summary.nodes[0].id, "copy");
        assert_eq!(summary.duration_ms, 4);
    }
}
