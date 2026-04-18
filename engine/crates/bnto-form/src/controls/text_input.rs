//! TextInput control — grapheme-aware cursor operations.
//!
//! All functions are pure: take a Field + FormMessage, return a new Field.
//! Cursor positions are in grapheme cluster indices, not byte offsets,
//! so multi-byte characters (emoji, accented letters) are handled correctly.

use unicode_segmentation::UnicodeSegmentation;

use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// Apply a text-editing message to a field in TextEditing or NumberEditing state.
///
/// Both states share identical cursor logic — the only difference is which
/// `FieldState` variant wraps the `(buffer, cursor)` pair.
pub fn update(field: Field, msg: FormMessage) -> Field {
    let (buffer, cursor, is_number) = match &field.state {
        FieldState::TextEditing { buffer, cursor } => (buffer.clone(), *cursor, false),
        FieldState::NumberEditing { buffer, cursor } => (buffer.clone(), *cursor, true),
        _ => return field,
    };

    let graphemes: Vec<&str> = buffer.graphemes(true).collect();
    let len = graphemes.len();

    let result = match msg {
        FormMessage::EditChar(ch) => insert_char(field, &buffer, &graphemes, cursor, ch),
        FormMessage::EditBackspace => backspace(field, &graphemes, cursor),
        FormMessage::DeleteForward => delete_forward(field, &graphemes, cursor, len),
        FormMessage::CursorLeft => move_cursor(field, buffer, cursor.saturating_sub(1)),
        FormMessage::CursorRight => move_cursor(field, buffer, (cursor + 1).min(len)),
        FormMessage::CursorHome => move_cursor(field, buffer, 0),
        FormMessage::CursorEnd => move_cursor(field, buffer, len),
        FormMessage::CursorWordBack => {
            let pos = word_boundary_back(&graphemes, cursor);
            move_cursor(field, buffer, pos)
        }
        FormMessage::CursorWordForward => {
            let pos = word_boundary_forward(&graphemes, cursor);
            move_cursor(field, buffer, pos)
        }
        FormMessage::DeleteWordBack => delete_word_back(field, &graphemes, cursor),
        // CommitEdit and CancelEdit are handled at the form level
        _ => field,
    };

    // Re-wrap into the correct state variant if needed
    if is_number {
        rewrap_as_number(result)
    } else {
        result
    }
}

/// Convert a `TextEditing` state back to `NumberEditing`.
///
/// All internal helpers produce `TextEditing`. When we entered from
/// `NumberEditing`, we need to swap the variant so the rest of the
/// system (commit, cancel, key mapping) sees the right state.
fn rewrap_as_number(field: Field) -> Field {
    match field.state {
        FieldState::TextEditing { buffer, cursor } => Field {
            state: FieldState::NumberEditing { buffer, cursor },
            ..field
        },
        _ => field,
    }
}

fn insert_char(field: Field, buffer: &str, graphemes: &[&str], cursor: usize, ch: char) -> Field {
    // Check character limit
    if let FieldKind::Text {
        char_limit: Some(limit),
        ..
    } = &field.kind
        && graphemes.len() >= *limit
    {
        return field;
    }

    let mut new_buf = String::with_capacity(buffer.len() + ch.len_utf8());
    for g in graphemes.iter().take(cursor) {
        new_buf.push_str(g);
    }
    new_buf.push(ch);
    for g in graphemes.iter().skip(cursor) {
        new_buf.push_str(g);
    }

    Field {
        state: FieldState::TextEditing {
            buffer: new_buf,
            cursor: cursor + 1,
        },
        ..field
    }
}

fn backspace(field: Field, graphemes: &[&str], cursor: usize) -> Field {
    if cursor == 0 {
        return field;
    }
    let new_buf: String = graphemes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != cursor - 1)
        .map(|(_, g)| *g)
        .collect();
    Field {
        state: FieldState::TextEditing {
            buffer: new_buf,
            cursor: cursor - 1,
        },
        ..field
    }
}

fn delete_forward(field: Field, graphemes: &[&str], cursor: usize, len: usize) -> Field {
    if cursor >= len {
        return field;
    }
    let new_buf: String = graphemes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != cursor)
        .map(|(_, g)| *g)
        .collect();
    Field {
        state: FieldState::TextEditing {
            buffer: new_buf,
            cursor,
        },
        ..field
    }
}

fn move_cursor(field: Field, buffer: String, new_cursor: usize) -> Field {
    Field {
        state: FieldState::TextEditing {
            buffer,
            cursor: new_cursor,
        },
        ..field
    }
}

/// Find the previous word boundary by scanning backwards from cursor.
/// Skips whitespace, then skips non-whitespace, landing at the word start.
fn word_boundary_back(graphemes: &[&str], cursor: usize) -> usize {
    if cursor == 0 {
        return 0;
    }
    let mut pos = cursor;
    // Skip trailing whitespace
    while pos > 0 && graphemes[pos - 1].trim().is_empty() {
        pos -= 1;
    }
    // Skip word characters
    while pos > 0 && !graphemes[pos - 1].trim().is_empty() {
        pos -= 1;
    }
    pos
}

/// Find the next word boundary by scanning forward from cursor.
/// Skips non-whitespace, then skips whitespace, landing at the next word.
fn word_boundary_forward(graphemes: &[&str], cursor: usize) -> usize {
    let len = graphemes.len();
    if cursor >= len {
        return len;
    }
    let mut pos = cursor;
    // Skip word characters
    while pos < len && !graphemes[pos].trim().is_empty() {
        pos += 1;
    }
    // Skip whitespace
    while pos < len && graphemes[pos].trim().is_empty() {
        pos += 1;
    }
    pos
}

fn delete_word_back(field: Field, graphemes: &[&str], cursor: usize) -> Field {
    if cursor == 0 {
        return field;
    }
    let target = word_boundary_back(graphemes, cursor);
    let new_buf: String = graphemes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < target || *i >= cursor)
        .map(|(_, g)| *g)
        .collect();
    Field {
        state: FieldState::TextEditing {
            buffer: new_buf,
            cursor: target,
        },
        ..field
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::text;
    use crate::form::FormMessage;

    fn editing_field(value: &str, cursor: usize) -> Field {
        let mut f = text("test").value(value).build();
        f.state = FieldState::TextEditing {
            buffer: value.to_string(),
            cursor,
        };
        f
    }

    #[test]
    fn test_insert_char() {
        let field = editing_field("helo", 3);
        let field = update(field, FormMessage::EditChar('l'));
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "hello");
                assert_eq!(*cursor, 4);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_insert_at_end() {
        let field = editing_field("abc", 3);
        let field = update(field, FormMessage::EditChar('d'));
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abcd");
                assert_eq!(*cursor, 4);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_insert_at_middle() {
        let field = editing_field("ac", 1);
        let field = update(field, FormMessage::EditChar('b'));
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abc");
                assert_eq!(*cursor, 2);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_insert_at_start() {
        let field = editing_field("bc", 0);
        let field = update(field, FormMessage::EditChar('a'));
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abc");
                assert_eq!(*cursor, 1);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_backspace() {
        let field = editing_field("abc", 2);
        let field = update(field, FormMessage::EditBackspace);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "ac");
                assert_eq!(*cursor, 1);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_backspace_at_start_is_noop() {
        let field = editing_field("abc", 0);
        let field = update(field, FormMessage::EditBackspace);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abc");
                assert_eq!(*cursor, 0);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_delete_forward() {
        let field = editing_field("abc", 1);
        let field = update(field, FormMessage::DeleteForward);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "ac");
                assert_eq!(*cursor, 1);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_delete_forward_at_end_is_noop() {
        let field = editing_field("abc", 3);
        let field = update(field, FormMessage::DeleteForward);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abc");
                assert_eq!(*cursor, 3);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_left() {
        let field = editing_field("abc", 2);
        let field = update(field, FormMessage::CursorLeft);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 1),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_left_stops_at_zero() {
        let field = editing_field("abc", 0);
        let field = update(field, FormMessage::CursorLeft);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 0),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_right() {
        let field = editing_field("abc", 1);
        let field = update(field, FormMessage::CursorRight);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 2),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_right_stops_at_len() {
        let field = editing_field("abc", 3);
        let field = update(field, FormMessage::CursorRight);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 3),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_home() {
        let field = editing_field("hello world", 7);
        let field = update(field, FormMessage::CursorHome);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 0),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_cursor_end() {
        let field = editing_field("hello", 2);
        let field = update(field, FormMessage::CursorEnd);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 5),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_word_boundary_back() {
        // "hello world" with cursor at 8 (on 'r') → should jump to 6 (start of "world")
        let field = editing_field("hello world", 8);
        let field = update(field, FormMessage::CursorWordBack);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 6),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_word_boundary_back_from_start_of_word() {
        // "hello world" cursor at 6 (start of "world") → should jump to 0 (start of "hello")
        let field = editing_field("hello world", 6);
        let field = update(field, FormMessage::CursorWordBack);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 0),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_word_boundary_forward() {
        // "hello world" cursor at 0 → should jump to 6 (past "hello ")
        let field = editing_field("hello world", 0);
        let field = update(field, FormMessage::CursorWordForward);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 6),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_word_boundary_forward_from_middle() {
        // "hello world" cursor at 6 → should jump to 11 (end)
        let field = editing_field("hello world", 6);
        let field = update(field, FormMessage::CursorWordForward);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 11),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_char_limit_enforced() {
        let mut f = text("test").char_limit(3).value("abc").build();
        f.state = FieldState::TextEditing {
            buffer: "abc".to_string(),
            cursor: 3,
        };
        let f = update(f, FormMessage::EditChar('d'));
        match &f.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "abc"); // rejected
                assert_eq!(*cursor, 3);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_char_limit_allows_under_limit() {
        let mut f = text("test").char_limit(5).value("ab").build();
        f.state = FieldState::TextEditing {
            buffer: "ab".to_string(),
            cursor: 2,
        };
        let f = update(f, FormMessage::EditChar('c'));
        match &f.state {
            FieldState::TextEditing { buffer, .. } => assert_eq!(buffer, "abc"),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_grapheme_aware_cursor() {
        // Multi-byte grapheme cluster: "café" has 4 grapheme clusters
        let field = editing_field("café", 4);
        let field = update(field, FormMessage::CursorLeft);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => assert_eq!(*cursor, 3),
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_grapheme_emoji() {
        // Emoji: each emoji is one grapheme cluster
        let field = editing_field("a😀b", 1);
        let field = update(field, FormMessage::CursorRight);
        match &field.state {
            FieldState::TextEditing { cursor, .. } => {
                assert_eq!(*cursor, 2); // past the emoji
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_delete_word_back() {
        let field = editing_field("hello world", 11);
        let field = update(field, FormMessage::DeleteWordBack);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "hello ");
                assert_eq!(*cursor, 6);
            }
            _ => panic!("expected TextEditing"),
        }
    }

    #[test]
    fn test_delete_word_back_at_start() {
        let field = editing_field("hello", 0);
        let field = update(field, FormMessage::DeleteWordBack);
        match &field.state {
            FieldState::TextEditing { buffer, cursor } => {
                assert_eq!(buffer, "hello");
                assert_eq!(*cursor, 0);
            }
            _ => panic!("expected TextEditing"),
        }
    }
}
