// What the live line under a running step shows: a bar when the step knows
// how far through it is, a spinner and a running message when it doesn't
// (one long item, or a node counting work with no known total).

/// The live line for the current event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineView {
    /// `pos` of `len` done.
    Bar { pos: u64, len: u64, msg: String },
    /// Still working, how far unknown.
    Spinner { msg: String },
}

/// One step's line state. Remembers the file position so a node's own
/// updates don't replace a bar across several files.
#[derive(Debug, Default)]
pub struct StepLine {
    /// `(files done, files in all)` when the step spans more than one file.
    files: Option<(u64, u64)>,
}

impl StepLine {
    /// A `FileProgress` event: the step started (`percent` 0) or finished
    /// (`percent` 100) file `file_index` of `total_files`.
    pub fn file_progress(
        &mut self,
        file_index: usize,
        total_files: usize,
        percent: u32,
        message: &str,
    ) -> LineView {
        let done = file_index as u64 + u64::from(percent >= 100);
        if total_files <= 1 {
            self.files = None;
            return spinner(message);
        }
        self.files = Some((done, total_files as u64));
        bar(done, total_files as u64, message)
    }

    /// A `NodeProgress` event: the node's own report from inside the step.
    pub fn node_progress(&mut self, done: u64, total: Option<u64>, message: &str) -> LineView {
        match (self.files, total) {
            (Some((files_done, files)), _) => bar(files_done, files, message),
            (None, Some(total)) => bar(done.min(total), total, message),
            (None, None) => spinner(message),
        }
    }
}

fn bar(pos: u64, len: u64, msg: &str) -> LineView {
    LineView::Bar {
        pos,
        len,
        msg: msg.to_string(),
    }
}

fn spinner(msg: &str) -> LineView {
    LineView::Spinner {
        msg: msg.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_input_shows_a_spinner_not_a_full_bar() {
        // The reported bug: one folder in, the bar sat full at 1/1.
        let view = StepLine::default().file_progress(0, 1, 0, "Processing D:\\_ToDelete...");
        assert_eq!(view, spinner("Processing D:\\_ToDelete..."));
    }

    #[test]
    fn bar_counts_files_finished_not_files_started() {
        let mut line = StepLine::default();
        assert_eq!(line.file_progress(0, 3, 0, "a"), bar(0, 3, "a"));
        assert_eq!(line.file_progress(0, 3, 100, "a done"), bar(1, 3, "a done"));
        assert_eq!(line.file_progress(2, 3, 100, "c done"), bar(3, 3, "c done"));
    }

    #[test]
    fn open_ended_node_count_shows_a_spinner_with_the_running_message() {
        let mut line = StepLine::default();
        line.file_progress(0, 1, 0, "Processing D:\\_ToDelete...");
        let view = line.node_progress(1204, None, "Found 1204 files · photos");
        assert_eq!(view, spinner("Found 1204 files · photos"));
    }

    #[test]
    fn node_percent_on_one_long_item_shows_a_bar() {
        let mut line = StepLine::default();
        line.file_progress(0, 1, 0, "Processing big.png...");
        assert_eq!(
            line.node_progress(40, Some(100), "Compressing"),
            bar(40, 100, "Compressing")
        );
    }

    #[test]
    fn node_updates_keep_the_file_bar_of_a_multi_file_step() {
        let mut line = StepLine::default();
        line.file_progress(4, 10, 0, "Processing e.png...");
        assert_eq!(
            line.node_progress(40, Some(100), "Compressing e.png"),
            bar(4, 10, "Compressing e.png")
        );
    }

    #[test]
    fn node_percent_past_its_total_is_capped() {
        let view = StepLine::default().node_progress(140, Some(100), "over");
        assert_eq!(view, bar(100, 100, "over"));
    }
}
