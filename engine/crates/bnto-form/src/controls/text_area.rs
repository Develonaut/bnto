// TextArea control — multi-line text editing with vertical cursor navigation.
//
// Reuses grapheme-aware cursor logic from text_input for horizontal movement.
// Adds line-based vertical navigation and scrolling.

use unicode_segmentation::UnicodeSegmentation;

use crate::field::{Field, FieldKind, FieldState};
use crate::form::FormMessage;

/// Default visible lines in the editor viewport.
const VIEWPORT_LINES: usize = 6;

/// Apply a text-area editing message to a field in TextAreaEditing state.
pub fn update(field: Field, msg: FormMessage) -> Field {
    let (buffer, cursor, line, scroll_offset) = match &field.state {
        FieldState::TextAreaEditing {
            buffer,
            cursor,
            line,
            scroll_offset,
        } => (buffer.clone(), *cursor, *line, *scroll_offset),
        _ => return field,
    };

    let lines: Vec<&str> = buffer.split('\n').collect();

    match msg {
        FormMessage::EditChar(ch) => insert_char(&field, &buffer, cursor, line, scroll_offset, ch),
        FormMessage::TextAreaNewline => {
            insert_newline(&field, &buffer, cursor, line, scroll_offset)
        }
        FormMessage::EditBackspace => backspace(&field, &buffer, cursor, line, scroll_offset),
        FormMessage::DeleteForward => delete_forward(&field, &buffer, cursor, line, scroll_offset),
        FormMessage::CursorLeft => cursor_left(&field, &buffer, cursor, line, scroll_offset),
        FormMessage::CursorRight => cursor_right(&field, &buffer, cursor, line, scroll_offset),
        FormMessage::CursorHome => set_state(field, buffer, 0, line, scroll_offset),
        FormMessage::CursorEnd => {
            let line_len = grapheme_count(lines[line]);
            set_state(field, buffer, line_len, line, scroll_offset)
        }
        FormMessage::TextAreaCursorUp => cursor_up(&field, &lines, cursor, line, scroll_offset),
        FormMessage::TextAreaCursorDown => cursor_down(&field, &lines, cursor, line, scroll_offset),
        _ => field,
    }
}

fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

fn insert_char(
    field: &Field,
    buffer: &str,
    cursor: usize,
    line: usize,
    scroll: usize,
    ch: char,
) -> Field {
    // Check char limit
    if let FieldKind::TextArea {
        char_limit: Some(limit),
        ..
    } = &field.kind
        && buffer.graphemes(true).count() >= *limit
    {
        return field.clone();
    }

    let lines: Vec<&str> = buffer.split('\n').collect();
    let graphemes: Vec<&str> = lines[line].graphemes(true).collect();

    let mut new_line_content = String::new();
    for g in graphemes.iter().take(cursor) {
        new_line_content.push_str(g);
    }
    new_line_content.push(ch);
    for g in graphemes.iter().skip(cursor) {
        new_line_content.push_str(g);
    }

    let new_buffer = rebuild_buffer(&lines, line, &new_line_content);
    set_state(field.clone(), new_buffer, cursor + 1, line, scroll)
}

fn insert_newline(field: &Field, buffer: &str, cursor: usize, line: usize, scroll: usize) -> Field {
    let lines: Vec<&str> = buffer.split('\n').collect();
    let graphemes: Vec<&str> = lines[line].graphemes(true).collect();

    let before: String = graphemes[..cursor].concat();
    let after: String = graphemes[cursor..].concat();

    let mut new_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    new_lines[line] = before;
    new_lines.insert(line + 1, after);

    let new_buffer = new_lines.join("\n");
    let new_line = line + 1;
    let new_scroll = adjust_scroll(new_line, scroll);
    set_state(field.clone(), new_buffer, 0, new_line, new_scroll)
}

fn backspace(field: &Field, buffer: &str, cursor: usize, line: usize, scroll: usize) -> Field {
    let lines: Vec<&str> = buffer.split('\n').collect();

    if cursor > 0 {
        // Delete character within line
        let graphemes: Vec<&str> = lines[line].graphemes(true).collect();
        let new_content: String = graphemes
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != cursor - 1)
            .map(|(_, g)| *g)
            .collect();
        let new_buffer = rebuild_buffer(&lines, line, &new_content);
        set_state(field.clone(), new_buffer, cursor - 1, line, scroll)
    } else if line > 0 {
        // Merge with previous line
        let prev_len = grapheme_count(lines[line - 1]);
        let merged = format!("{}{}", lines[line - 1], lines[line]);
        let mut new_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        new_lines[line - 1] = merged;
        new_lines.remove(line);
        let new_buffer = new_lines.join("\n");
        let new_line = line - 1;
        let new_scroll = if new_line < scroll { new_line } else { scroll };
        set_state(field.clone(), new_buffer, prev_len, new_line, new_scroll)
    } else {
        field.clone()
    }
}

fn delete_forward(field: &Field, buffer: &str, cursor: usize, line: usize, scroll: usize) -> Field {
    let lines: Vec<&str> = buffer.split('\n').collect();
    let line_len = grapheme_count(lines[line]);

    if cursor < line_len {
        // Delete character at cursor
        let graphemes: Vec<&str> = lines[line].graphemes(true).collect();
        let new_content: String = graphemes
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != cursor)
            .map(|(_, g)| *g)
            .collect();
        let new_buffer = rebuild_buffer(&lines, line, &new_content);
        set_state(field.clone(), new_buffer, cursor, line, scroll)
    } else if line + 1 < lines.len() {
        // Merge with next line
        let merged = format!("{}{}", lines[line], lines[line + 1]);
        let mut new_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        new_lines[line] = merged;
        new_lines.remove(line + 1);
        let new_buffer = new_lines.join("\n");
        set_state(field.clone(), new_buffer, cursor, line, scroll)
    } else {
        field.clone()
    }
}

fn cursor_left(field: &Field, buffer: &str, cursor: usize, line: usize, scroll: usize) -> Field {
    if cursor > 0 {
        set_state(field.clone(), buffer.to_string(), cursor - 1, line, scroll)
    } else if line > 0 {
        // Wrap to end of previous line
        let lines: Vec<&str> = buffer.split('\n').collect();
        let prev_len = grapheme_count(lines[line - 1]);
        let new_scroll = if line - 1 < scroll { line - 1 } else { scroll };
        set_state(
            field.clone(),
            buffer.to_string(),
            prev_len,
            line - 1,
            new_scroll,
        )
    } else {
        field.clone()
    }
}

fn cursor_right(field: &Field, buffer: &str, cursor: usize, line: usize, scroll: usize) -> Field {
    let lines: Vec<&str> = buffer.split('\n').collect();
    let line_len = grapheme_count(lines[line]);

    if cursor < line_len {
        set_state(field.clone(), buffer.to_string(), cursor + 1, line, scroll)
    } else if line + 1 < lines.len() {
        // Wrap to start of next line
        let new_scroll = adjust_scroll(line + 1, scroll);
        set_state(field.clone(), buffer.to_string(), 0, line + 1, new_scroll)
    } else {
        field.clone()
    }
}

fn cursor_up(field: &Field, lines: &[&str], cursor: usize, line: usize, scroll: usize) -> Field {
    if line == 0 {
        return field.clone();
    }
    let new_line = line - 1;
    let new_line_len = grapheme_count(lines[new_line]);
    let new_cursor = cursor.min(new_line_len);
    let new_scroll = if new_line < scroll { new_line } else { scroll };
    let buffer: String = lines.join("\n");
    set_state(field.clone(), buffer, new_cursor, new_line, new_scroll)
}

fn cursor_down(field: &Field, lines: &[&str], cursor: usize, line: usize, scroll: usize) -> Field {
    if line + 1 >= lines.len() {
        return field.clone();
    }
    let new_line = line + 1;
    let new_line_len = grapheme_count(lines[new_line]);
    let new_cursor = cursor.min(new_line_len);
    let new_scroll = adjust_scroll(new_line, scroll);
    let buffer: String = lines.join("\n");
    set_state(field.clone(), buffer, new_cursor, new_line, new_scroll)
}

/// Adjust scroll offset to keep the target line visible in the viewport.
fn adjust_scroll(target_line: usize, current_scroll: usize) -> usize {
    if target_line >= current_scroll + VIEWPORT_LINES {
        target_line - VIEWPORT_LINES + 1
    } else if target_line < current_scroll {
        target_line
    } else {
        current_scroll
    }
}

/// Rebuild the full buffer after modifying one line.
fn rebuild_buffer(lines: &[&str], line_idx: usize, new_content: &str) -> String {
    let mut result: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    result[line_idx] = new_content.to_string();
    result.join("\n")
}

fn set_state(field: Field, buffer: String, cursor: usize, line: usize, scroll: usize) -> Field {
    Field {
        state: FieldState::TextAreaEditing {
            buffer,
            cursor,
            line,
            scroll_offset: scroll,
        },
        ..field
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::textarea;

    fn editing_field(value: &str, cursor: usize, line: usize) -> Field {
        let mut f = textarea("test").value(value).build();
        f.state = FieldState::TextAreaEditing {
            buffer: value.to_string(),
            cursor,
            line,
            scroll_offset: 0,
        };
        f
    }

    #[test]
    fn test_textarea_insert_char() {
        let field = editing_field("hello", 5, 0);
        let field = update(field, FormMessage::EditChar('!'));
        match &field.state {
            FieldState::TextAreaEditing { buffer, cursor, .. } => {
                assert_eq!(buffer, "hello!");
                assert_eq!(*cursor, 6);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_newline_splits_line() {
        let field = editing_field("hello world", 5, 0);
        let field = update(field, FormMessage::TextAreaNewline);
        match &field.state {
            FieldState::TextAreaEditing {
                buffer,
                cursor,
                line,
                ..
            } => {
                assert_eq!(buffer, "hello\n world");
                assert_eq!(*cursor, 0);
                assert_eq!(*line, 1);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_cursor_up_down() {
        let field = editing_field("line1\nline2\nline3", 3, 1);
        // Move up
        let field = update(field, FormMessage::TextAreaCursorUp);
        match &field.state {
            FieldState::TextAreaEditing { cursor, line, .. } => {
                assert_eq!(*line, 0);
                assert_eq!(*cursor, 3);
            }
            _ => panic!("expected TextAreaEditing"),
        }
        // Move down twice
        let field = update(field, FormMessage::TextAreaCursorDown);
        let field = update(field, FormMessage::TextAreaCursorDown);
        match &field.state {
            FieldState::TextAreaEditing { line, .. } => {
                assert_eq!(*line, 2);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_cursor_up_at_top_is_noop() {
        let field = editing_field("only line", 3, 0);
        let field = update(field, FormMessage::TextAreaCursorUp);
        match &field.state {
            FieldState::TextAreaEditing { cursor, line, .. } => {
                assert_eq!(*line, 0);
                assert_eq!(*cursor, 3);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_cursor_clamps_to_shorter_line() {
        // line1 is 10 chars, line2 is 3 chars. Cursor at col 8, move down.
        let field = editing_field("0123456789\nabc", 8, 0);
        let field = update(field, FormMessage::TextAreaCursorDown);
        match &field.state {
            FieldState::TextAreaEditing { cursor, line, .. } => {
                assert_eq!(*line, 1);
                assert_eq!(*cursor, 3, "cursor should clamp to shorter line length");
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_backspace_merges_lines() {
        let field = editing_field("hello\nworld", 0, 1);
        let field = update(field, FormMessage::EditBackspace);
        match &field.state {
            FieldState::TextAreaEditing {
                buffer,
                cursor,
                line,
                ..
            } => {
                assert_eq!(buffer, "helloworld");
                assert_eq!(*line, 0);
                assert_eq!(*cursor, 5);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_delete_forward_merges_lines() {
        let field = editing_field("hello\nworld", 5, 0);
        let field = update(field, FormMessage::DeleteForward);
        match &field.state {
            FieldState::TextAreaEditing { buffer, cursor, .. } => {
                assert_eq!(buffer, "helloworld");
                assert_eq!(*cursor, 5);
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_scroll_when_exceeding_viewport() {
        // Create content with 8 lines, cursor at line 7
        let content = (0..8)
            .map(|i| format!("line{i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let field = editing_field(&content, 0, 0);
        // Move down to line 7 (past viewport of 6)
        let mut f = field;
        for _ in 0..7 {
            f = update(f, FormMessage::TextAreaCursorDown);
        }
        match &f.state {
            FieldState::TextAreaEditing {
                line,
                scroll_offset,
                ..
            } => {
                assert_eq!(*line, 7);
                assert!(
                    *scroll_offset > 0,
                    "should have scrolled: offset={scroll_offset}"
                );
            }
            _ => panic!("expected TextAreaEditing"),
        }
    }

    #[test]
    fn test_textarea_cursor_home_end() {
        let field = editing_field("hello world", 5, 0);
        let field = update(field, FormMessage::CursorHome);
        match &field.state {
            FieldState::TextAreaEditing { cursor, .. } => assert_eq!(*cursor, 0),
            _ => panic!("expected TextAreaEditing"),
        }
        let field = update(field, FormMessage::CursorEnd);
        match &field.state {
            FieldState::TextAreaEditing { cursor, .. } => assert_eq!(*cursor, 11),
            _ => panic!("expected TextAreaEditing"),
        }
    }
}
