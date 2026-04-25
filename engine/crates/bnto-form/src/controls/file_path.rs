// FilePath control — pure state transitions for the file browser.
//
// All functions take the FilePathBrowsing state fields and return updated values.
// No I/O, no rendering — just cursor/navigation/selection logic.

use std::path::{Path, PathBuf};

use crate::field::{Field, FieldState};
use crate::file_entry::{FileEntry, NavHistory};
use crate::viewport;

/// Move cursor down by one, wrapping at the end.
pub fn cursor_down(
    entries: &[FileEntry],
    cursor: usize,
    offset: usize,
    height: usize,
) -> (usize, usize) {
    if entries.is_empty() {
        return (0, 0);
    }
    let new_cursor = if cursor + 1 >= entries.len() {
        0
    } else {
        cursor + 1
    };
    let new_offset = viewport::ensure_cursor_visible(new_cursor, offset, height);
    (new_cursor, new_offset)
}

/// Move cursor up by one, wrapping at the start.
pub fn cursor_up(
    entries: &[FileEntry],
    cursor: usize,
    offset: usize,
    height: usize,
) -> (usize, usize) {
    if entries.is_empty() {
        return (0, 0);
    }
    let new_cursor = if cursor == 0 {
        entries.len() - 1
    } else {
        cursor - 1
    };
    let new_offset = viewport::ensure_cursor_visible(new_cursor, offset, height);
    (new_cursor, new_offset)
}

/// Enter a directory: returns the new dir path if the cursor is on a dir,
/// or `None` if it's a file (caller should use `confirm` instead).
pub fn enter_dir(entries: &[FileEntry], cursor: usize) -> Option<PathBuf> {
    entries
        .get(cursor)
        .filter(|e| e.is_dir)
        .map(|e| e.path.clone())
}

/// Navigate to parent directory.
pub fn parent_dir(current_dir: &Path) -> Option<PathBuf> {
    current_dir.parent().map(|p| p.to_path_buf())
}

/// Confirm selection: returns the file path if cursor is on a file.
pub fn confirm_file(entries: &[FileEntry], cursor: usize) -> Option<String> {
    entries
        .get(cursor)
        .filter(|e| !e.is_dir)
        .map(|e| e.path.to_string_lossy().to_string())
}

/// Apply a DirLoaded response to a field, restoring nav_history cursor if available.
pub fn apply_dir_loaded(
    field: Field,
    dir: PathBuf,
    entries: Vec<FileEntry>,
    restored: Option<(usize, usize)>,
) -> Field {
    match field.state {
        FieldState::FilePathBrowsing {
            show_hidden,
            viewport_height,
            nav_history,
            ..
        } => {
            let (cursor, offset) = restored.unwrap_or((0, 0));
            // Clamp cursor to entry count
            let cursor = if entries.is_empty() {
                0
            } else {
                cursor.min(entries.len() - 1)
            };
            let offset = viewport::ensure_cursor_visible(cursor, offset, viewport_height);
            Field {
                state: FieldState::FilePathBrowsing {
                    current_dir: dir,
                    entries,
                    cursor,
                    show_hidden,
                    viewport_offset: offset,
                    viewport_height,
                    nav_history,
                },
                ..field
            }
        }
        _ => field,
    }
}

/// Push current position to nav history before entering a child dir.
pub fn push_nav_history(nav: &mut NavHistory, cursor: usize, offset: usize) {
    nav.push(cursor, offset);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldKind;

    fn sample_entries() -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "docs".into(),
                is_dir: true,
                path: PathBuf::from("/home/user/docs"),
                size: None,
            },
            FileEntry {
                name: "photos".into(),
                is_dir: true,
                path: PathBuf::from("/home/user/photos"),
                size: None,
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/home/user/cat.jpg"),
                size: Some(290_000),
            },
            FileEntry {
                name: "dog.png".into(),
                is_dir: false,
                path: PathBuf::from("/home/user/dog.png"),
                size: Some(150_000),
            },
        ]
    }

    #[test]
    fn cursor_down_advances() {
        let entries = sample_entries();
        let (c, _) = cursor_down(&entries, 0, 0, 20);
        assert_eq!(c, 1);
    }

    #[test]
    fn cursor_down_wraps() {
        let entries = sample_entries();
        let (c, _) = cursor_down(&entries, 3, 0, 20);
        assert_eq!(c, 0);
    }

    #[test]
    fn cursor_up_retreats() {
        let entries = sample_entries();
        let (c, _) = cursor_up(&entries, 2, 0, 20);
        assert_eq!(c, 1);
    }

    #[test]
    fn cursor_up_wraps() {
        let entries = sample_entries();
        let (c, _) = cursor_up(&entries, 0, 0, 20);
        assert_eq!(c, 3);
    }

    #[test]
    fn cursor_on_empty_entries() {
        let (c, o) = cursor_down(&[], 0, 0, 20);
        assert_eq!((c, o), (0, 0));
        let (c, o) = cursor_up(&[], 0, 0, 20);
        assert_eq!((c, o), (0, 0));
    }

    #[test]
    fn enter_dir_on_directory() {
        let entries = sample_entries();
        let result = enter_dir(&entries, 0);
        assert_eq!(result, Some(PathBuf::from("/home/user/docs")));
    }

    #[test]
    fn enter_dir_on_file_returns_none() {
        let entries = sample_entries();
        assert!(enter_dir(&entries, 2).is_none());
    }

    #[test]
    fn parent_dir_returns_parent() {
        let dir = PathBuf::from("/home/user/docs");
        assert_eq!(parent_dir(&dir), Some(PathBuf::from("/home/user")));
    }

    #[test]
    fn parent_dir_at_root() {
        let dir = PathBuf::from("/");
        assert!(parent_dir(&dir).is_none());
    }

    #[test]
    fn confirm_file_on_file() {
        let entries = sample_entries();
        let result = confirm_file(&entries, 2);
        assert_eq!(result, Some("/home/user/cat.jpg".to_string()));
    }

    #[test]
    fn confirm_file_on_dir_returns_none() {
        let entries = sample_entries();
        assert!(confirm_file(&entries, 0).is_none());
    }

    #[test]
    fn cursor_down_scrolls_viewport() {
        let mut entries = Vec::new();
        for i in 0..30 {
            entries.push(FileEntry {
                name: format!("file_{i}.txt"),
                is_dir: false,
                path: PathBuf::from(format!("/tmp/file_{i}.txt")),
                size: Some(100),
            });
        }
        let (c, o) = cursor_down(&entries, 9, 0, 10);
        assert_eq!(c, 10);
        assert!(o > 0, "viewport should scroll");
    }

    #[test]
    fn apply_dir_loaded_updates_state() {
        let field = Field {
            id: "f".into(),
            label: "File".into(),
            kind: FieldKind::FilePath {
                extensions: vec![],
                start_home: true,
            },
            state: FieldState::FilePathBrowsing {
                current_dir: PathBuf::from("/old"),
                entries: vec![],
                cursor: 0,
                show_hidden: false,
                viewport_offset: 0,
                viewport_height: 20,
                nav_history: NavHistory::new(),
            },
            value: String::new(),
            default: None,
            description: None,
            error: None,
            validator: None,
            visible: true,
        };
        let new_entries = sample_entries();
        let updated = apply_dir_loaded(field, PathBuf::from("/new"), new_entries.clone(), None);
        match &updated.state {
            FieldState::FilePathBrowsing {
                current_dir,
                entries,
                cursor,
                ..
            } => {
                assert_eq!(current_dir, &PathBuf::from("/new"));
                assert_eq!(entries.len(), 4);
                assert_eq!(*cursor, 0);
            }
            _ => panic!("expected FilePathBrowsing"),
        }
    }

    #[test]
    fn apply_dir_loaded_restores_cursor() {
        let field = Field {
            id: "f".into(),
            label: "File".into(),
            kind: FieldKind::FilePath {
                extensions: vec![],
                start_home: true,
            },
            state: FieldState::FilePathBrowsing {
                current_dir: PathBuf::from("/old"),
                entries: vec![],
                cursor: 0,
                show_hidden: false,
                viewport_offset: 0,
                viewport_height: 20,
                nav_history: NavHistory::new(),
            },
            value: String::new(),
            default: None,
            description: None,
            error: None,
            validator: None,
            visible: true,
        };
        let entries = sample_entries();
        let updated = apply_dir_loaded(field, PathBuf::from("/new"), entries, Some((2, 0)));
        match &updated.state {
            FieldState::FilePathBrowsing { cursor, .. } => {
                assert_eq!(*cursor, 2);
            }
            _ => panic!("expected FilePathBrowsing"),
        }
    }

    #[test]
    fn apply_dir_loaded_clamps_cursor() {
        let field = Field {
            id: "f".into(),
            label: "File".into(),
            kind: FieldKind::FilePath {
                extensions: vec![],
                start_home: true,
            },
            state: FieldState::FilePathBrowsing {
                current_dir: PathBuf::from("/old"),
                entries: vec![],
                cursor: 0,
                show_hidden: false,
                viewport_offset: 0,
                viewport_height: 20,
                nav_history: NavHistory::new(),
            },
            value: String::new(),
            default: None,
            description: None,
            error: None,
            validator: None,
            visible: true,
        };
        // Restore cursor=10 but only 4 entries → should clamp to 3
        let entries = sample_entries();
        let updated = apply_dir_loaded(field, PathBuf::from("/new"), entries, Some((10, 0)));
        match &updated.state {
            FieldState::FilePathBrowsing { cursor, .. } => {
                assert_eq!(*cursor, 3);
            }
            _ => panic!("expected FilePathBrowsing"),
        }
    }
}
