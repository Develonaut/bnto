// Navigation history — remembers cursor + viewport position when entering directories.

/// A snapshot of the picker's position when navigating into a child directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavEntry {
    pub cursor: usize,
    pub viewport_offset: usize,
}

/// LIFO stack of navigation positions for the file picker.
#[derive(Debug)]
pub struct NavHistory {
    stack: Vec<NavEntry>,
}

impl NavHistory {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Save the current position before navigating into a directory.
    pub fn push(&mut self, cursor: usize, viewport_offset: usize) {
        self.stack.push(NavEntry {
            cursor,
            viewport_offset,
        });
    }

    /// Restore the previous position when navigating to the parent directory.
    pub fn pop(&mut self) -> Option<NavEntry> {
        self.stack.pop()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_history_is_empty() {
        let h = NavHistory::new();
        assert!(h.is_empty());
    }

    #[test]
    fn push_then_pop_returns_entry() {
        let mut h = NavHistory::new();
        h.push(5, 10);
        let entry = h.pop().expect("should have entry");
        assert_eq!(
            entry,
            NavEntry {
                cursor: 5,
                viewport_offset: 10
            }
        );
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut h = NavHistory::new();
        assert!(h.pop().is_none());
    }

    #[test]
    fn lifo_order() {
        let mut h = NavHistory::new();
        h.push(1, 0);
        h.push(5, 10);
        h.push(3, 20);

        assert_eq!(h.pop().unwrap().cursor, 3);
        assert_eq!(h.pop().unwrap().cursor, 5);
        assert_eq!(h.pop().unwrap().cursor, 1);
        assert!(h.pop().is_none());
    }

    #[test]
    fn is_empty_transitions() {
        let mut h = NavHistory::new();
        assert!(h.is_empty());
        h.push(0, 0);
        assert!(!h.is_empty());
        h.pop();
        assert!(h.is_empty());
    }
}
