// Which of a run's files fit on the execution screen. A long run would
// otherwise push the OUTPUT pane off a short terminal, so the FILES list
// shows a window around the file being worked on and counts the rest.

use std::ops::Range;

/// The files to draw, and how many are left out around them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileWindow {
    /// Indexes of the files drawn.
    pub shown: Range<usize>,
    /// Files left out (above and below the window), shown as one line.
    pub hidden: usize,
}

/// The window over `total` files that fits in `rows` rows, keeping `active`
/// (the file being worked on, if any) in view. When not everything fits, one
/// row goes to the "… N more" line, and at least one file always shows.
pub fn file_window(total: usize, active: Option<usize>, rows: usize) -> FileWindow {
    if total <= rows {
        return FileWindow {
            shown: 0..total,
            hidden: 0,
        };
    }
    let visible = rows.saturating_sub(1).clamp(1, total);
    let focus = active.unwrap_or(0).min(total - 1);
    let start = focus.saturating_sub(visible / 2).min(total - visible);
    FileWindow {
        shown: start..start + visible,
        hidden: total - visible,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything_shows_when_it_fits() {
        assert_eq!(
            file_window(5, Some(2), 5),
            FileWindow {
                shown: 0..5,
                hidden: 0
            }
        );
    }

    #[test]
    fn a_long_list_keeps_the_active_file_in_view() {
        // 4 rows: 3 files plus one "… N more" line.
        let window = file_window(20, Some(10), 4);
        assert!(window.shown.contains(&10), "{window:?}");
        assert_eq!(window.shown.len(), 3);
        assert_eq!(window.hidden, 17);
    }

    #[test]
    fn the_window_stops_at_the_ends() {
        assert_eq!(file_window(20, Some(0), 4).shown, 0..3);
        assert_eq!(file_window(20, Some(19), 4).shown, 17..20);
    }

    #[test]
    fn no_active_file_shows_the_start() {
        assert_eq!(file_window(20, None, 4).shown, 0..3);
    }

    #[test]
    fn at_least_one_file_shows() {
        let window = file_window(20, Some(7), 0);
        assert_eq!(window.shown, 7..8);
        assert_eq!(window.hidden, 19);
    }
}
