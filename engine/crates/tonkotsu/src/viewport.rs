// Viewport scrolling — pure functions and a stateful struct for scrollable content.
//
// The free functions (`ensure_cursor_visible`, `page_down`, etc.) handle cursor-based
// list scrolling. The `Viewport` struct wraps offset-only scrolling for composite
// content (e.g. the detail screen's header + input + params + run button).

// --- Viewport struct (offset-only scrolling for composite content) ---

/// A reusable scroll container for content taller than the visible area.
///
/// Unlike the free functions below (which track a cursor position), `Viewport`
/// manages only a scroll offset. The consumer decides what "line" to scroll to
/// via `ensure_visible()`. Modeled after Charm's `bubbles/viewport`.
#[derive(Debug, Clone)]
pub struct Viewport {
    offset: usize,
    height: usize,
    content_height: usize,
}

impl Viewport {
    /// Create a viewport with zero dimensions (must call `set_height` before use).
    pub fn new() -> Self {
        Self {
            offset: 0,
            height: 0,
            content_height: 0,
        }
    }

    /// Set the visible height (terminal lines available for content).
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.clamp_offset();
    }

    /// Set the total content height (number of lines of content).
    pub fn set_content_height(&mut self, content_height: usize) {
        self.content_height = content_height;
        self.clamp_offset();
    }

    /// Scroll down by one line.
    pub fn scroll_down(&mut self) {
        if self.offset < self.max_offset() {
            self.offset += 1;
        }
    }

    /// Scroll up by one line.
    pub fn scroll_up(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    /// Scroll down by one page.
    pub fn page_down(&mut self) {
        if self.height == 0 {
            return;
        }
        self.offset = (self.offset + self.height).min(self.max_offset());
    }

    /// Scroll up by one page.
    pub fn page_up(&mut self) {
        if self.height == 0 {
            return;
        }
        self.offset = self.offset.saturating_sub(self.height);
    }

    /// Jump to the top of the content.
    pub fn go_to_top(&mut self) {
        self.offset = 0;
    }

    /// Jump to the bottom of the content.
    pub fn go_to_bottom(&mut self) {
        self.offset = self.max_offset();
    }

    /// Ensure a specific line is visible. Scrolls minimally to bring it into view.
    pub fn ensure_visible(&mut self, line: usize) {
        if self.height == 0 {
            return;
        }
        if line < self.offset {
            self.offset = line;
        } else if line >= self.offset + self.height {
            self.offset = line.saturating_sub(self.height - 1);
        }
        self.clamp_offset();
    }

    /// Current scroll offset (number of lines scrolled past the top).
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Visible height of the viewport.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Total content height.
    pub fn content_height(&self) -> usize {
        self.content_height
    }

    /// Whether content overflows the viewport (scrolling is possible).
    pub fn overflows(&self) -> bool {
        self.content_height > self.height
    }

    /// Whether the viewport is scrolled to the very top.
    pub fn at_top(&self) -> bool {
        self.offset == 0
    }

    /// Whether the viewport is scrolled to the very bottom.
    pub fn at_bottom(&self) -> bool {
        self.offset >= self.max_offset()
    }

    fn max_offset(&self) -> usize {
        if self.height == 0 {
            return 0;
        }
        self.content_height.saturating_sub(self.height)
    }

    fn clamp_offset(&mut self) {
        let max = self.max_offset();
        if self.offset > max {
            self.offset = max;
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

// --- Free functions (cursor-based list scrolling) ---

/// Adjust viewport offset so the cursor stays visible.
///
/// Returns the new offset. If the cursor is already within the visible
/// window, the offset is unchanged.
pub fn ensure_cursor_visible(cursor: usize, offset: usize, height: usize) -> usize {
    if height == 0 {
        return 0;
    }
    if cursor < offset {
        cursor
    } else if cursor >= offset + height {
        cursor - height + 1
    } else {
        offset
    }
}

/// Move one page down. Returns `(cursor, offset)`.
pub fn page_down(cursor: usize, offset: usize, height: usize, total: usize) -> (usize, usize) {
    if total == 0 || height == 0 {
        return (0, 0);
    }
    let last = total - 1;
    let new_cursor = (cursor + height).min(last);
    let new_offset = ensure_cursor_visible(new_cursor, offset, height);
    (new_cursor, new_offset)
}

/// Move one page up. Returns `(cursor, offset)`.
pub fn page_up(cursor: usize, offset: usize, height: usize) -> (usize, usize) {
    if height == 0 {
        return (0, 0);
    }
    let new_cursor = cursor.saturating_sub(height);
    let new_offset = ensure_cursor_visible(new_cursor, offset, height);
    (new_cursor, new_offset)
}

/// Jump to the first entry. Returns `(cursor, offset)`.
pub fn go_to_top() -> (usize, usize) {
    (0, 0)
}

/// Jump to the last entry. Returns `(cursor, offset)`.
pub fn go_to_bottom(height: usize, total: usize) -> (usize, usize) {
    if total == 0 {
        return (0, 0);
    }
    let last = total - 1;
    let offset = total.saturating_sub(height);
    (last, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Viewport struct tests ---

    #[test]
    fn viewport_new_starts_at_zero() {
        let vp = Viewport::new();
        assert_eq!(vp.offset(), 0);
        assert_eq!(vp.height(), 0);
        assert_eq!(vp.content_height(), 0);
    }

    #[test]
    fn viewport_scroll_down_increments() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(20);
        vp.scroll_down();
        assert_eq!(vp.offset(), 1);
        vp.scroll_down();
        assert_eq!(vp.offset(), 2);
    }

    #[test]
    fn viewport_scroll_down_clamps_at_bottom() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(12);
        // Max offset = 12 - 10 = 2
        vp.scroll_down();
        vp.scroll_down();
        vp.scroll_down();
        assert_eq!(vp.offset(), 2);
    }

    #[test]
    fn viewport_scroll_up_decrements() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(20);
        vp.scroll_down();
        vp.scroll_down();
        vp.scroll_up();
        assert_eq!(vp.offset(), 1);
    }

    #[test]
    fn viewport_scroll_up_clamps_at_zero() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(20);
        vp.scroll_up();
        assert_eq!(vp.offset(), 0);
    }

    #[test]
    fn viewport_page_down() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.page_down();
        assert_eq!(vp.offset(), 5);
        vp.page_down();
        assert_eq!(vp.offset(), 10);
    }

    #[test]
    fn viewport_page_down_clamps() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(8);
        // Max offset = 3
        vp.page_down();
        assert_eq!(vp.offset(), 3);
    }

    #[test]
    fn viewport_page_up() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.page_down();
        vp.page_down();
        assert_eq!(vp.offset(), 10);
        vp.page_up();
        assert_eq!(vp.offset(), 5);
    }

    #[test]
    fn viewport_page_up_clamps_at_zero() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.scroll_down();
        vp.scroll_down();
        vp.page_up();
        assert_eq!(vp.offset(), 0);
    }

    #[test]
    fn viewport_go_to_top() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.page_down();
        vp.go_to_top();
        assert_eq!(vp.offset(), 0);
    }

    #[test]
    fn viewport_go_to_bottom() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.go_to_bottom();
        assert_eq!(vp.offset(), 15);
    }

    #[test]
    fn viewport_ensure_visible_scrolls_down() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        // Line 7 is below visible range (0..5)
        vp.ensure_visible(7);
        assert_eq!(vp.offset(), 3); // 7 - 5 + 1 = 3
    }

    #[test]
    fn viewport_ensure_visible_scrolls_up() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.go_to_bottom(); // offset = 15
        vp.ensure_visible(3);
        assert_eq!(vp.offset(), 3);
    }

    #[test]
    fn viewport_ensure_visible_no_change_when_in_view() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.ensure_visible(3); // 3 is within 0..5
        assert_eq!(vp.offset(), 0);
    }

    #[test]
    fn viewport_overflows() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(10);
        assert!(!vp.overflows());
        vp.set_content_height(11);
        assert!(vp.overflows());
    }

    #[test]
    fn viewport_at_top_and_bottom() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(10);
        assert!(vp.at_top());
        assert!(!vp.at_bottom());
        vp.go_to_bottom();
        assert!(!vp.at_top());
        assert!(vp.at_bottom());
    }

    #[test]
    fn viewport_no_overflow_always_at_both() {
        let mut vp = Viewport::new();
        vp.set_height(10);
        vp.set_content_height(5);
        assert!(vp.at_top());
        assert!(vp.at_bottom());
    }

    #[test]
    fn viewport_set_content_height_clamps_offset() {
        let mut vp = Viewport::new();
        vp.set_height(5);
        vp.set_content_height(20);
        vp.go_to_bottom(); // offset = 15
        vp.set_content_height(8); // max offset now = 3
        assert_eq!(vp.offset(), 3);
    }

    #[test]
    fn viewport_zero_height_is_safe() {
        let mut vp = Viewport::new();
        vp.set_content_height(20);
        vp.scroll_down();
        vp.page_down();
        vp.ensure_visible(10);
        assert_eq!(vp.offset(), 0);
    }

    // --- Free function tests ---

    #[test]
    fn cursor_below_viewport_scrolls_down() {
        assert_eq!(ensure_cursor_visible(6, 0, 5), 2);
    }

    #[test]
    fn cursor_above_viewport_scrolls_up() {
        assert_eq!(ensure_cursor_visible(3, 5, 5), 3);
    }

    #[test]
    fn cursor_within_viewport_no_change() {
        assert_eq!(ensure_cursor_visible(3, 0, 5), 0);
    }

    #[test]
    fn zero_height_returns_zero() {
        assert_eq!(ensure_cursor_visible(5, 3, 0), 0);
    }

    #[test]
    fn page_down_advances_and_clamps() {
        let (c, o) = page_down(3, 0, 5, 20);
        assert_eq!(c, 8);
        assert_eq!(o, 4);

        let (c, _) = page_down(18, 14, 5, 20);
        assert_eq!(c, 19);
    }

    #[test]
    fn page_up_retreats_and_clamps() {
        let (c, o) = page_up(8, 4, 5);
        assert_eq!(c, 3);
        assert_eq!(o, 3);

        let (c, o) = page_up(2, 0, 5);
        assert_eq!(c, 0);
        assert_eq!(o, 0);
    }

    #[test]
    fn go_to_top_returns_zero() {
        assert_eq!(go_to_top(), (0, 0));
    }

    #[test]
    fn go_to_bottom_positions_last_visible() {
        let (c, o) = go_to_bottom(5, 20);
        assert_eq!(c, 19);
        assert_eq!(o, 15);
    }

    #[test]
    fn page_down_empty_list() {
        assert_eq!(page_down(0, 0, 5, 0), (0, 0));
    }

    #[test]
    fn go_to_bottom_empty_list() {
        assert_eq!(go_to_bottom(5, 0), (0, 0));
    }
}
