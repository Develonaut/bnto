// =============================================================================
// Progress Reporting — How Nodes Talk to the UI
// =============================================================================
//
// Target-agnostic progress reporting. Uses a plain Rust closure instead of
// `js_sys::Function` so bnto-core stays platform-independent. The WASM-specific
// wrapping lives in each node crate's `wasm_bridge.rs`.
//
// Every node gets the same two ways to say it's still working:
//   - `report(percent, msg)` — it knows how far through it is
//   - `report_count(count, msg)` — it doesn't know the total (files found
//     during a walk, rows read from a stream)
// The executor turns both into `PipelineEvent::NodeProgress`, throttled.

use crate::progress_throttle::Throttle;

/// One live progress update from a running node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveProgress {
    /// How far through a known amount of work, 0-100.
    Percent(u32),
    /// Units of work done so far, total unknown.
    Count(u64),
}

impl LiveProgress {
    /// The update as `(done, total)`: a percent is out of 100, a count has
    /// no total.
    pub fn done_and_total(self) -> (u64, Option<u64>) {
        match self {
            Self::Percent(percent) => (u64::from(percent), Some(100)),
            Self::Count(count) => (count, None),
        }
    }
}

// =============================================================================
// ProgressReporter
// =============================================================================

/// Reports processing progress from a node back to the caller (UI, CLI, etc.).
///
/// Every callback is optional: a reporter with none is a no-op (used in tests).
pub struct ProgressReporter {
    #[allow(clippy::type_complexity)]
    live: Option<Box<dyn Fn(LiveProgress, &str)>>,
    /// Streaming command output lines (stderr from child processes).
    #[allow(clippy::type_complexity)]
    output_callback: Option<Box<dyn Fn(&str)>>,
    /// When set, live updates past the first are let through once per interval.
    throttle: Option<Throttle>,
}

impl ProgressReporter {
    /// Create a reporter that receives percent updates.
    ///
    /// The callback receives `(percent 0-100, message)`. Count updates
    /// (`report_count`) don't reach it; use [`ProgressReporter::with_live`]
    /// to receive both.
    ///
    /// ```rust
    /// use bnto_core::ProgressReporter;
    ///
    /// let reporter = ProgressReporter::new(|percent, message| {
    ///     println!("{}% — {}", percent, message);
    /// });
    /// ```
    pub fn new(callback: impl Fn(u32, &str) + 'static) -> Self {
        let live = move |progress: LiveProgress, message: &str| {
            if let LiveProgress::Percent(percent) = progress {
                callback(percent, message);
            }
        };
        Self {
            live: Some(Box::new(live)),
            output_callback: None,
            throttle: None,
        }
    }

    /// Create a no-op reporter that discards all progress updates.
    pub fn new_noop() -> Self {
        Self {
            live: None,
            output_callback: None,
            throttle: None,
        }
    }

    /// Create a reporter that receives every live update and command output
    /// line. The executor uses this to relay both as pipeline events.
    pub fn with_live(
        live: impl Fn(LiveProgress, &str) + 'static,
        output_callback: impl Fn(&str) + 'static,
    ) -> Self {
        Self {
            live: Some(Box::new(live)),
            output_callback: Some(Box::new(output_callback)),
            throttle: None,
        }
    }

    /// Let at most one live update through per `interval_ms` (the first one
    /// always gets through). Command output lines are never throttled.
    pub fn throttled(mut self, clock: impl Fn() -> u64 + 'static, interval_ms: u64) -> Self {
        self.throttle = Some(Throttle::new(clock, interval_ms));
        self
    }

    /// Report how far along the node is (0 to 100), with what it's doing
    /// ("Compressing image 3/10...").
    pub fn report(&self, percent: u32, message: &str) {
        self.send(LiveProgress::Percent(percent), message);
    }

    /// Report work done so far when the node can't know the total, e.g.
    /// files found while walking a folder. Call it as often as you like;
    /// a throttled reporter drops the excess.
    pub fn report_count(&self, count: u64, message: &str) {
        self.send(LiveProgress::Count(count), message);
    }

    /// Report a line of streaming output from a child process.
    ///
    /// Called by processors that use `run_command_streaming()` to relay
    /// stderr lines (e.g. yt-dlp progress) to the pipeline reporter.
    pub fn report_output(&self, line: &str) {
        if let Some(cb) = &self.output_callback {
            cb(line);
        }
    }

    fn send(&self, progress: LiveProgress, message: &str) {
        let Some(cb) = &self.live else { return };
        if self.throttle.as_ref().is_none_or(Throttle::allow) {
            cb(progress, message);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    type Recorded = Arc<Mutex<Vec<(LiveProgress, String)>>>;

    /// A reporter that records every live update it receives.
    fn recording() -> (Recorded, ProgressReporter) {
        let calls: Recorded = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&calls);
        let reporter = ProgressReporter::with_live(
            move |p, m| sink.lock().unwrap().push((p, m.to_string())),
            |_| {},
        );
        (calls, reporter)
    }

    #[test]
    fn test_noop_reporter_doesnt_panic() {
        let reporter = ProgressReporter::new_noop();
        reporter.report(0, "Starting...");
        reporter.report_count(3, "Found 3 files");
        reporter.report(100, "Done!");
    }

    #[test]
    fn test_reporter_calls_callback() {
        let calls: Arc<Mutex<Vec<(u32, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let calls_clone = Arc::clone(&calls);
        let reporter = ProgressReporter::new(move |percent, message| {
            calls_clone
                .lock()
                .unwrap()
                .push((percent, message.to_string()));
        });

        reporter.report(0, "Starting...");
        reporter.report(50, "Halfway there...");
        reporter.report(100, "Done!");

        let recorded = calls.lock().unwrap();
        assert_eq!(recorded.len(), 3, "Should have recorded 3 calls");
        assert_eq!(recorded[0], (0, "Starting...".to_string()));
        assert_eq!(recorded[1], (50, "Halfway there...".to_string()));
        assert_eq!(recorded[2], (100, "Done!".to_string()));
    }

    #[test]
    fn percent_only_callback_ignores_counts() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&calls);
        let reporter = ProgressReporter::new(move |p, _| sink.lock().unwrap().push(p));

        reporter.report_count(7, "Found 7 files");
        reporter.report(40, "Working");

        assert_eq!(*calls.lock().unwrap(), vec![40]);
    }

    #[test]
    fn live_reporter_receives_percents_and_counts() {
        let (calls, reporter) = recording();

        reporter.report(25, "Quarter");
        reporter.report_count(12, "Found 12 files");

        let calls = calls.lock().unwrap();
        assert_eq!(calls[0], (LiveProgress::Percent(25), "Quarter".to_string()));
        assert_eq!(
            calls[1],
            (LiveProgress::Count(12), "Found 12 files".to_string())
        );
    }

    #[test]
    fn throttled_reporter_lets_one_update_through_per_interval() {
        let (calls, reporter) = recording();
        let now = Rc::new(Cell::new(0u64));
        let clock = Rc::clone(&now);
        let reporter = reporter.throttled(move || clock.get(), 100);

        for (t, count) in [(0, 1), (10, 2), (60, 3), (100, 4), (150, 5), (210, 6)] {
            now.set(t);
            reporter.report_count(count, "walking");
        }

        let counts: Vec<LiveProgress> = calls.lock().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            counts,
            vec![
                LiveProgress::Count(1),
                LiveProgress::Count(4),
                LiveProgress::Count(6)
            ]
        );
    }

    #[test]
    fn throttle_never_drops_command_output() {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&lines);
        let reporter = ProgressReporter::with_live(
            |_, _| {},
            move |l| sink.lock().unwrap().push(l.to_string()),
        )
        .throttled(|| 0, 100);

        reporter.report_output("one");
        reporter.report_output("two");

        assert_eq!(lines.lock().unwrap().len(), 2);
    }

    #[test]
    fn done_and_total_maps_percent_to_out_of_100() {
        assert_eq!(LiveProgress::Percent(40).done_and_total(), (40, Some(100)));
        assert_eq!(LiveProgress::Count(9).done_and_total(), (9, None));
    }

    #[test]
    fn report_output_calls_callback() {
        let received: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let received_clone = Arc::clone(&received);

        let reporter = ProgressReporter::with_live(
            |_, _| {},
            move |line| received_clone.lock().unwrap().push(line.to_string()),
        );

        reporter.report_output("downloading 50%");
        reporter.report_output("downloading 100%");

        let lines = received.lock().unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "downloading 50%");
        assert_eq!(lines[1], "downloading 100%");
    }

    #[test]
    fn report_output_noop_doesnt_panic() {
        let reporter = ProgressReporter::new_noop();
        reporter.report_output("ignored");
    }
}
