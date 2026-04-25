// Filesystem entry types for the file path browser.
//
// Adapted from the bnto TUI picker — simplified for single-select form field use.
// No bnto dependency; only standard library types.

use std::path::PathBuf;

/// A single filesystem entry shown in the file path browser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub path: PathBuf,
    pub size: Option<u64>,
}

/// LIFO stack of cursor/offset positions for directory navigation.
///
/// When the user enters a child directory, the current position is pushed.
/// When they navigate to the parent, the position is restored.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NavHistory {
    stack: Vec<NavEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NavEntry {
    cursor: usize,
    viewport_offset: usize,
}

impl NavHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, cursor: usize, viewport_offset: usize) {
        self.stack.push(NavEntry {
            cursor,
            viewport_offset,
        });
    }

    pub fn pop(&mut self) -> Option<(usize, usize)> {
        self.stack.pop().map(|e| (e.cursor, e.viewport_offset))
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_entry_stores_name_and_path() {
        let entry = FileEntry {
            name: "photo.jpg".into(),
            is_dir: false,
            path: PathBuf::from("/home/user/photo.jpg"),
            size: Some(290_000),
        };
        assert_eq!(entry.name, "photo.jpg");
        assert!(!entry.is_dir);
        assert_eq!(entry.size, Some(290_000));
    }

    #[test]
    fn file_entry_dir_has_no_size() {
        let entry = FileEntry {
            name: "docs".into(),
            is_dir: true,
            path: PathBuf::from("/home/user/docs"),
            size: None,
        };
        assert!(entry.is_dir);
        assert!(entry.size.is_none());
    }

    #[test]
    fn nav_history_new_is_empty() {
        let h = NavHistory::new();
        assert!(h.is_empty());
    }

    #[test]
    fn nav_history_push_then_pop() {
        let mut h = NavHistory::new();
        h.push(5, 10);
        let (cursor, offset) = h.pop().expect("should have entry");
        assert_eq!(cursor, 5);
        assert_eq!(offset, 10);
        assert!(h.is_empty());
    }

    #[test]
    fn nav_history_pop_empty_returns_none() {
        let mut h = NavHistory::new();
        assert!(h.pop().is_none());
    }

    #[test]
    fn nav_history_lifo_order() {
        let mut h = NavHistory::new();
        h.push(1, 0);
        h.push(5, 10);
        h.push(3, 20);

        assert_eq!(h.pop().unwrap(), (3, 20));
        assert_eq!(h.pop().unwrap(), (5, 10));
        assert_eq!(h.pop().unwrap(), (1, 0));
        assert!(h.pop().is_none());
    }

    #[test]
    fn nav_history_is_empty_transitions() {
        let mut h = NavHistory::new();
        assert!(h.is_empty());
        h.push(0, 0);
        assert!(!h.is_empty());
        h.pop();
        assert!(h.is_empty());
    }

    #[test]
    fn file_entry_equality() {
        let a = FileEntry {
            name: "a.txt".into(),
            is_dir: false,
            path: PathBuf::from("/a.txt"),
            size: Some(100),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
