// File path browser widget — renders directory header + entry list with cursor marker.

use ratatui::text::{Line, Span};

use crate::field::{Field, FieldState};
use crate::format_size::format_size;
use crate::theme::FormTheme;

/// Render the file path browser: directory header + visible entries.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let (current_dir, entries, cursor, viewport_offset, viewport_height) = match &field.state {
        FieldState::FilePathBrowsing {
            current_dir,
            entries,
            cursor,
            viewport_offset,
            viewport_height,
            ..
        } => (
            current_dir,
            entries,
            *cursor,
            *viewport_offset,
            *viewport_height,
        ),
        _ => {
            // Idle: show label + current value
            let prefix = if focused { "> " } else { "  " };
            let value = if field.value.is_empty() {
                "(no file selected)".to_string()
            } else {
                field.display_value()
            };
            return vec![Line::from(vec![
                Span::styled(prefix.to_string(), theme.text()),
                Span::styled(format!("{}: ", field.label), theme.heading()),
                Span::styled(value, theme.muted()),
            ])];
        }
    };

    let mut lines = Vec::new();

    // Directory header
    let dir_display = crate::field::abbreviate_home(&current_dir.to_string_lossy());
    let prefix = if focused { "> " } else { "  " };
    lines.push(Line::from(vec![
        Span::styled(prefix.to_string(), theme.heading()),
        Span::styled(format!("{}: ", field.label), theme.heading()),
        Span::styled(dir_display, theme.text()),
    ]));

    if entries.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("(empty directory)", theme.muted()),
        ]));
        return lines;
    }

    // Visible entries within viewport
    let end = (viewport_offset + viewport_height).min(entries.len());
    for (i, entry) in entries
        .iter()
        .enumerate()
        .skip(viewport_offset)
        .take(end - viewport_offset)
    {
        let is_cursor = i == cursor;
        let marker = if is_cursor { "  > " } else { "    " };
        let marker_style = if is_cursor {
            theme.heading()
        } else {
            theme.text()
        };

        let icon = if entry.is_dir { "📁 " } else { "   " };
        let name = entry.name.clone();
        let name_style = if entry.is_dir {
            theme.heading()
        } else {
            theme.text()
        };

        let mut spans = vec![
            Span::styled(marker.to_string(), marker_style),
            Span::raw(icon.to_string()),
            Span::styled(name, name_style),
        ];

        // Show file size for files
        if let Some(size) = entry.size {
            spans.push(Span::styled(
                format!("  {}", format_size(size)),
                theme.muted(),
            ));
        }

        lines.push(Line::from(spans));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::file_path;
    use crate::file_entry::{FileEntry, NavHistory};
    use crate::theme::DefaultTheme;
    use std::path::PathBuf;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn browsing_field() -> Field {
        let mut f = file_path("input").label("Input File").build();
        f.state = FieldState::FilePathBrowsing {
            current_dir: PathBuf::from("/home/user"),
            entries: vec![
                FileEntry {
                    name: "docs".into(),
                    is_dir: true,
                    path: PathBuf::from("/home/user/docs"),
                    size: None,
                },
                FileEntry {
                    name: "photo.jpg".into(),
                    is_dir: false,
                    path: PathBuf::from("/home/user/photo.jpg"),
                    size: Some(290_000),
                },
            ],
            cursor: 0,
            show_hidden: false,
            viewport_offset: 0,
            viewport_height: 20,
            nav_history: NavHistory::new(),
        };
        f
    }

    #[test]
    fn renders_dir_header() {
        let field = browsing_field();
        let lines = render(&field, true, &theme());
        let header = line_text(&lines[0]);
        assert!(header.contains("Input File:"), "got: {header}");
    }

    #[test]
    fn renders_entry_names() {
        let field = browsing_field();
        let lines = render(&field, true, &theme());
        let all: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(all.contains("docs"), "got: {all}");
        assert!(all.contains("photo.jpg"), "got: {all}");
    }

    #[test]
    fn cursor_marker_on_focused_entry() {
        let field = browsing_field();
        let lines = render(&field, true, &theme());
        // Entry lines start after the header
        let entry_line = line_text(&lines[1]);
        assert!(
            entry_line.contains(">"),
            "cursor entry should have marker: {entry_line}"
        );
    }

    #[test]
    fn shows_file_size() {
        let field = browsing_field();
        let lines = render(&field, true, &theme());
        let all: String = lines
            .iter()
            .map(|l| line_text(l))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(all.contains("283 KB"), "should show file size: {all}");
    }

    #[test]
    fn idle_shows_no_file_selected() {
        let field = file_path("input").label("Input File").build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("(no file selected)"), "got: {text}");
    }
}
