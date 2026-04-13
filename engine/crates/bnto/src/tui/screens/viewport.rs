// Viewport scrolling — pure functions for keeping the cursor visible in a scrollable list.

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
        // Viewport shows items 0..5, cursor moved to 6
        assert_eq!(ensure_cursor_visible(6, 0, 5), 2);
    }

    #[test]
    fn cursor_above_viewport_scrolls_up() {
        // Viewport shows items 5..10, cursor moved to 3
        assert_eq!(ensure_cursor_visible(3, 5, 5), 3);
    }

    #[test]
    fn cursor_within_viewport_no_change() {
        assert_eq!(ensure_cursor_visible(3, 0, 5), 0);
    }

    #[test]
    fn page_down_advances_and_clamps() {
        // 20 items, viewport height 5, cursor at 3, offset 0
        let (c, o) = page_down(3, 0, 5, 20);
        assert_eq!(c, 8);
        assert_eq!(o, 4); // 8 - 5 + 1

        // At the end, clamp to last
        let (c, _) = page_down(18, 14, 5, 20);
        assert_eq!(c, 19);
    }

    #[test]
    fn page_up_retreats_and_clamps() {
        let (c, o) = page_up(8, 4, 5);
        assert_eq!(c, 3);
        assert_eq!(o, 3);

        // At the top, clamp to 0
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
        // 20 items, viewport height 5 → offset = 15 so items 15..20 visible
        let (c, o) = go_to_bottom(5, 20);
        assert_eq!(c, 19);
        assert_eq!(o, 15);
    }

    #[test]
    fn page_down_at_bottom_clamps() {
        let (c, o) = page_down(19, 15, 5, 20);
        assert_eq!(c, 19);
        assert_eq!(o, 15);
    }
}
