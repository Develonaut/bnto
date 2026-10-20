// The indicatif bar that draws a step's live line on stderr. It ticks on its
// own, so the spinner and elapsed time keep moving between engine events.
// indicatif draws nothing when stderr isn't a terminal: piped runs stay quiet.

use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::progress_line::{LineView, StepLine};

/// How often the spinner and elapsed time redraw on their own.
const TICK: Duration = Duration::from_millis(100);

const BAR_TEMPLATE: &str =
    "        {spinner:.cyan} {bar:30.cyan/dim} {pos}/{len} {msg} {elapsed:.dim}";
const SPINNER_TEMPLATE: &str = "        {spinner:.cyan} {msg} {elapsed:.dim}";

/// Braille spinner frames; the last is shown once the bar finishes.
const TICK_CHARS: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";

/// The live line of the running step, if one is showing.
#[derive(Default)]
pub struct LiveBar {
    bar: Option<ProgressBar>,
    /// Whether the bar is styled as a spinner, so the style changes only
    /// when the view switches.
    spinner_style: Option<bool>,
    line: StepLine,
}

impl LiveBar {
    pub fn file_progress(&mut self, index: usize, total: usize, percent: u32, msg: &str) {
        let view = self.line.file_progress(index, total, percent, msg);
        self.show(view);
    }

    pub fn node_progress(&mut self, done: u64, total: Option<u64>, msg: &str) {
        let view = self.line.node_progress(done, total, msg);
        self.show(view);
    }

    /// Remove the line, ready for the next step.
    pub fn clear(&mut self) {
        if let Some(bar) = self.bar.take() {
            bar.finish_and_clear();
        }
        *self = Self::default();
    }

    /// Print above the line without tearing it.
    pub fn suspend(&self, print: impl FnOnce()) {
        match &self.bar {
            Some(bar) => bar.suspend(print),
            None => print(),
        }
    }

    fn show(&mut self, view: LineView) {
        let spinner = matches!(view, LineView::Spinner { .. });
        let bar = self.bar.get_or_insert_with(new_bar);
        if self.spinner_style != Some(spinner) {
            bar.set_style(style(spinner));
            self.spinner_style = Some(spinner);
        }
        match view {
            LineView::Bar { pos, len, msg } => {
                bar.set_length(len);
                bar.set_position(pos);
                bar.set_message(msg);
            }
            LineView::Spinner { msg } => bar.set_message(msg),
        }
    }
}

fn new_bar() -> ProgressBar {
    let bar = ProgressBar::new(0);
    bar.enable_steady_tick(TICK);
    bar
}

fn style(spinner: bool) -> ProgressStyle {
    let template = if spinner {
        SPINNER_TEMPLATE
    } else {
        BAR_TEMPLATE
    };
    ProgressStyle::with_template(template)
        .expect("valid template")
        .progress_chars("━╸─")
        .tick_chars(TICK_CHARS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_templates_parse() {
        style(true);
        style(false);
    }

    #[test]
    fn switching_between_bar_and_spinner_restyles_once() {
        let mut live = LiveBar::default();
        live.file_progress(0, 1, 0, "one folder");
        assert_eq!(live.spinner_style, Some(true));
        live.node_progress(40, Some(100), "Compressing");
        assert_eq!(live.spinner_style, Some(false));
        live.clear();
        assert!(live.bar.is_none());
        assert_eq!(live.spinner_style, None);
    }
}
