// Viewport scrolling — pure functions for keeping the cursor visible in a scrollable list.
//
// Adapted from the bnto TUI picker. Standalone, no bnto dependency.

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
