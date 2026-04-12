// File list widget — renders filesystem entries with selection markers.
//
// Pure function: takes entries + state + theme, returns Vec<Line>.
// The caller renders via Paragraph.

use std::collections::BTreeSet;

use ratatui::text::{Line, Span};

use super::super::screens::picker::FileEntry;
use super::super::theme::Theme;

/// Render the file list with cursor and selection markers.
///
/// Dirs show a folder icon, files show a document icon. Selected files
/// are marked with a checkbox. The cursor row is highlighted.
pub fn render_file_list<'a>(
    entries: &[FileEntry],
    cursor: usize,
    selected: &BTreeSet<usize>,
    theme: &Theme,
) -> Vec<Line<'a>> {
    entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_cursor = i == cursor;
            let is_selected = selected.contains(&i);

            let marker = if is_cursor { "▸ " } else { "  " };
            let icon = if entry.is_dir { "/ " } else { "  " };
            let check = if entry.is_dir {
                "   "
            } else if is_selected {
                " [x]"
            } else {
                " [ ]"
            };

            let name_style = if is_cursor {
                theme.selected()
            } else if is_selected {
                theme.heading()
            } else {
                theme.text()
            };

            Line::from(vec![
                Span::styled(format!("  {marker}{icon}"), name_style),
                Span::styled(entry.name.clone(), name_style),
                Span::styled(check.to_string(), theme.muted()),
            ])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::theme::{Theme, ThemeVariant};
    use std::path::PathBuf;

    fn theme() -> Theme {
        Theme::from_variant(ThemeVariant::LosAngeles)
    }

    fn sample_entries() -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "photos".into(),
                is_dir: true,
                path: PathBuf::from("/photos"),
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
            },
            FileEntry {
                name: "dog.png".into(),
                is_dir: false,
                path: PathBuf::from("/dog.png"),
            },
        ]
    }

    #[test]
    fn renders_all_entries() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme());
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn selected_file_shows_checkbox() {
        let mut selected = BTreeSet::new();
        selected.insert(1); // cat.jpg
        let lines = render_file_list(&sample_entries(), 0, &selected, &theme());
        let cat_line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            cat_line.contains("[x]"),
            "selected file should show [x], got: {cat_line}"
        );
    }

    #[test]
    fn unselected_file_shows_empty_checkbox() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme());
        let cat_line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            cat_line.contains("[ ]"),
            "unselected file should show [ ], got: {cat_line}"
        );
    }

    #[test]
    fn directory_has_no_checkbox() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme());
        let dir_line: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            !dir_line.contains("[x]") && !dir_line.contains("[ ]"),
            "directory should have no checkbox, got: {dir_line}"
        );
    }

    #[test]
    fn empty_entries_returns_empty_lines() {
        let lines = render_file_list(&[], 0, &BTreeSet::new(), &theme());
        assert!(lines.is_empty());
    }
}
