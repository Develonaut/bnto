// File list widget — renders filesystem entries with selection markers.
//
// Pure function: takes entries + state + theme, returns Vec<Line>.
// The caller renders via Paragraph.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ratatui::text::{Line, Span};

use super::super::format::format_size;
use super::super::screens::picker::FileEntry;
use super::super::theme::Theme;

/// Render the file list with cursor and selection markers.
///
/// Dirs show a folder icon, files show a document icon. Selected files
/// are marked with a checkbox. The cursor row is highlighted.
/// File sizes are shown right of the name for files with known size.
/// When `show_metadata` is true, permissions and symlink targets are shown.
pub fn render_file_list<'a>(
    entries: &[FileEntry],
    cursor: usize,
    selected: &BTreeSet<PathBuf>,
    theme: &Theme,
    show_metadata: bool,
) -> Vec<Line<'a>> {
    entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_cursor = i == cursor;
            let is_selected = selected.contains(&entry.path);

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

            let size_text = if !entry.is_dir {
                entry
                    .size
                    .map(|s| format!("  {}", format_size(s)))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            let mut spans = vec![
                Span::styled(format!("  {marker}{icon}"), name_style),
                Span::styled(entry.name.clone(), name_style),
            ];

            // Symlink target indicator
            if let Some(target) = &entry.symlink_target {
                spans.push(Span::styled(format!(" -> {target}"), theme.muted()));
            }

            spans.push(Span::styled(size_text, theme.muted()));

            // Permissions column (when metadata display is on)
            if show_metadata && let Some(perms) = &entry.permissions {
                spans.push(Span::styled(format!("  {perms}"), theme.muted()));
            }

            spans.push(Span::styled(check.to_string(), theme.muted()));

            Line::from(spans)
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
                size: None,
                permissions: None,
                symlink_target: None,
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
                size: Some(290_000),
                permissions: None,
                symlink_target: None,
            },
            FileEntry {
                name: "dog.png".into(),
                is_dir: false,
                path: PathBuf::from("/dog.png"),
                size: Some(150_000),
                permissions: None,
                symlink_target: None,
            },
        ]
    }

    #[test]
    fn renders_all_entries() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme(), false);
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn selected_file_shows_checkbox() {
        let mut selected = BTreeSet::new();
        selected.insert(PathBuf::from("/cat.jpg"));
        let lines = render_file_list(&sample_entries(), 0, &selected, &theme(), false);
        let cat_line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            cat_line.contains("[x]"),
            "selected file should show [x], got: {cat_line}"
        );
    }

    #[test]
    fn unselected_file_shows_empty_checkbox() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme(), false);
        let cat_line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            cat_line.contains("[ ]"),
            "unselected file should show [ ], got: {cat_line}"
        );
    }

    #[test]
    fn directory_has_no_checkbox() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme(), false);
        let dir_line: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            !dir_line.contains("[x]") && !dir_line.contains("[ ]"),
            "directory should have no checkbox, got: {dir_line}"
        );
    }

    #[test]
    fn empty_entries_returns_empty_lines() {
        let lines = render_file_list(&[], 0, &BTreeSet::new(), &theme(), false);
        assert!(lines.is_empty());
    }

    #[test]
    fn file_with_size_renders_size_text() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme(), false);
        let cat_line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            cat_line.contains("283 KB"),
            "file should show size, got: {cat_line}"
        );
    }

    #[test]
    fn directory_has_no_size_text() {
        let lines = render_file_list(&sample_entries(), 0, &BTreeSet::new(), &theme(), false);
        let dir_line: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            !dir_line.contains("KB") && !dir_line.contains("MB") && !dir_line.contains(" B"),
            "directory should have no size, got: {dir_line}"
        );
    }

    // --- Metadata display ---

    fn entries_with_metadata() -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "photos".into(),
                is_dir: true,
                path: PathBuf::from("/photos"),
                size: None,
                permissions: Some("rwxr-xr-x".into()),
                symlink_target: None,
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
                size: Some(290_000),
                permissions: Some("rw-r--r--".into()),
                symlink_target: None,
            },
            FileEntry {
                name: "link.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/link.jpg"),
                size: Some(100),
                permissions: Some("rwxrwxrwx".into()),
                symlink_target: Some("/real/cat.jpg".into()),
            },
        ]
    }

    #[test]
    fn metadata_off_hides_permissions() {
        let lines = render_file_list(
            &entries_with_metadata(),
            0,
            &BTreeSet::new(),
            &theme(),
            false,
        );
        let line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            !line.contains("rw-r--r--"),
            "permissions should be hidden when metadata off, got: {line}"
        );
    }

    #[test]
    fn metadata_on_shows_permissions() {
        let lines = render_file_list(
            &entries_with_metadata(),
            0,
            &BTreeSet::new(),
            &theme(),
            true,
        );
        let line: String = lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            line.contains("rw-r--r--"),
            "permissions should show when metadata on, got: {line}"
        );
    }

    #[test]
    fn symlink_shows_arrow_target() {
        let lines = render_file_list(
            &entries_with_metadata(),
            0,
            &BTreeSet::new(),
            &theme(),
            false,
        );
        let line: String = lines[2].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            line.contains("-> /real/cat.jpg"),
            "symlink should show arrow + target, got: {line}"
        );
    }
}
